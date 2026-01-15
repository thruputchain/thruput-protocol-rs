use ::serde;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
struct SyscallSpec {
    syscall_input_count: usize,
    type_mappings: Vec<TypeMapping>,
    syscalls: Vec<Syscall>,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[derive(Debug, serde::Deserialize)]
struct Syscall {
    name: String,
    number: i32,
    #[serde(default)]
    inputs: Vec<Arg>,
    #[serde(default)]
    outputs: Vec<Arg>,
    #[serde(default = "default_true")]
    returns: bool, // default to true
}

#[derive(Debug, serde::Deserialize)]
struct TypeMapping {
    base_name: String,
    rust_name: String,
    #[serde(default = "default_true")]
    rust_uses_as_ptr: bool,
    #[serde(default = "default_false")]
    primitive_type: bool,
}

#[derive(Debug)]
struct TypeInfo {
    rust_name: String,
    rust_uses_as_ptr: bool,
    primitive_type: bool,
}

type TypeMap = HashMap<String, TypeInfo>;

impl SyscallSpec {
    pub fn get_type_map(&self) -> TypeMap {
        let mut out: HashMap<String, TypeInfo> = HashMap::new();
        for elt in &self.type_mappings {
            out.insert(
                elt.base_name.clone(),
                TypeInfo {
                    rust_name: elt.rust_name.clone(),
                    rust_uses_as_ptr: elt.rust_uses_as_ptr,
                    primitive_type: elt.primitive_type,
                },
            );
        }
        out
    }
}

impl Syscall {
    pub fn return_type(&self) -> TokenStream2 {
        if self.returns {
            return quote! {
                SyscallResult<()>
            };
        }
        quote! {
            !
        }
    }
}

enum ArgMapped {
    Span {
        is_input: bool,
    },
    FixedSize {
        ty: TokenStream2,
        is_input: bool,
        is_primitive: bool,
        is_optional: bool,
    },
}

impl ArgMapped {
    fn is_input(&self) -> bool {
        match *self {
            ArgMapped::Span { is_input } => is_input,
            ArgMapped::FixedSize { is_input, .. } => is_input,
        }
    }

    fn is_primitive(&self) -> bool {
        match *self {
            ArgMapped::FixedSize { is_primitive, .. } => is_primitive,
            _ => false,
        }
    }

    fn is_optional(&self) -> bool {
        match *self {
            ArgMapped::FixedSize { is_optional, .. } => is_optional,
            _ => false,
        }
    }

    fn to_uninit_param(&self) -> TokenStream2 {
        match *self {
            ArgMapped::Span { is_input } => {
                if is_input {
                    quote! { &[u8] }
                } else {
                    quote! { &mut [::core::mem::MaybeUninit<u8>] }
                }
            }
            ArgMapped::FixedSize {
                ref ty,
                is_input,
                is_primitive,
                is_optional,
            } => {
                let mut ty = if is_input {
                    if is_primitive && !is_optional {
                        quote! { #ty }
                    } else {
                        quote! { &#ty }
                    }
                } else {
                    if is_primitive {
                        quote! { &mut #ty }
                    } else {
                        quote! { &mut ::core::mem::MaybeUninit<#ty> }
                    }
                };

                if is_optional {
                    ty = quote! { Option<#ty> };
                }

                ty
            }
        }
    }

    fn to_raw_param(&self) -> TokenStream2 {
        let mut ty = match self {
            ArgMapped::Span { .. } => quote! { [u8] },
            ArgMapped::FixedSize { ty, .. } => {
                quote! { #ty }
            }
        };

        if self.is_input() {
            if !self.is_primitive() || self.is_optional() {
                ty = quote! { &#ty };
            }
        } else {
            ty = quote! { &mut #ty };
        }

        if self.is_optional() {
            ty = quote! { Option<#ty> };
        }

        ty
    }
}

#[derive(Debug, serde::Deserialize)]
struct Arg {
    name: String,

    #[serde(default)]
    span_argument: bool,

    typename: Option<String>,

    #[serde(default)]
    primitive_type: bool, // default to false

    #[serde(default)]
    optional: bool, // default to false
}

impl Arg {
    fn is_span_argument(&self) -> bool {
        self.span_argument
    }

    fn formatted_name(&self) -> proc_macro2::Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    fn get_type_details<'a>(
        &self,
        type_map: &'a TypeMap,
    ) -> Option<&'a TypeInfo> {
        // Return `None` if we have no typename
        let typename: &str = self.typename.as_ref()?.as_ref();

        type_map.get(typename)
    }

    fn fixed_size_type(&self, type_map: &TypeMap) -> Option<TokenStream2> {
        // Return `None` if we have no typename
        let mut typename: &str = self.typename.as_ref()?.as_ref();

        // Possibly translate via the type map
        if let Some(type_details) = type_map.get(typename) {
            typename = &type_details.rust_name;
        }

        // It's an unmapped identifier or some other kind of type expression
        match syn::parse_str::<syn::Type>(typename) {
            Ok(ty) => Some(ty.into_token_stream()),
            Err(e) => {
                let err = syn::Error::new(
                    e.span(),
                    format!(
                        "Couldn't parse `typename` field as a type expression: \"{}\"",
                        typename
                    ),
                );
                Some(err.into_compile_error())
            }
        }
    }

    fn is_primitive(&self, type_map: &TypeMap) -> bool {
        if let Some(type_details) = self.get_type_details(type_map) {
            type_details.primitive_type
        } else {
            self.primitive_type
        }
    }

    fn uses_as_ptr(&self, type_map: &TypeMap) -> bool {
        if let Some(type_details) = self.get_type_details(type_map) {
            type_details.rust_uses_as_ptr
        } else {
            true
        }
    }

    fn mapped(&self, is_input: bool, type_map: &TypeMap) -> ArgMapped {
        if self.is_span_argument() {
            ArgMapped::Span { is_input }
        } else if let Some(ty) = self.fixed_size_type(type_map) {
            ArgMapped::FixedSize {
                ty,
                is_input,
                is_primitive: self.is_primitive(type_map),
                is_optional: self.optional,
            }
        } else {
            unimplemented!("Unknown arg type");
        }
    }

    pub fn to_u64s(
        &self,
        type_map: &TypeMap,
        is_input: bool,
    ) -> Vec<TokenStream2> {
        let name = self.formatted_name();

        if self.is_span_argument() {
            let span_len = quote! { #name.len() as u64 };
            let span_ptr = if is_input {
                quote! {
                    #name.as_ptr() as u64
                }
            } else {
                quote! {
                    #name.as_mut_ptr() as u64
                }
            };
            vec![span_ptr, span_len]
        } else if self.fixed_size_type(type_map).is_some() {
            let arg = if self.optional {
                assert!(is_input);
                if self.uses_as_ptr(type_map) {
                    quote! {
                        match #name {
                            Some(x) => x.as_ptr() as u64,
                            None => u64::MAX,
                        }
                    }
                } else {
                    quote! {
                        match #name {
                            Some(x) => x as *const _ as u64,
                            None => u64::MAX,
                        }
                    }
                }
            } else if self.is_primitive(type_map) {
                if is_input {
                    quote! { #name as u64 }
                } else {
                    quote! { #name as *mut _ as u64 }
                }
            } else if self.uses_as_ptr(type_map) {
                if is_input {
                    quote! { #name.as_ptr() as u64 }
                } else {
                    quote! { #name.as_mut_ptr() as u64 }
                }
            } else {
                // else it's, at this point in time, a 128 bit int
                if is_input {
                    quote! { #name as *const _ as u64 }
                } else {
                    quote! { #name as *mut _ as u64 }
                }
            };

            vec![arg]
        } else {
            unimplemented!("Unknown arg type");
        }
    }

    pub fn to_uninit_param(
        &self,
        is_input: bool,
        type_map: &TypeMap,
    ) -> TokenStream2 {
        let name = self.formatted_name();
        let uninit_ty = self.mapped(is_input, type_map).to_uninit_param();

        quote! {
            #name: #uninit_ty
        }
    }

    pub fn to_raw_param(
        &self,
        is_input: bool,
        type_map: &TypeMap,
    ) -> TokenStream2 {
        let name = self.formatted_name();
        let raw_ty = self.mapped(is_input, type_map).to_raw_param();

        quote! {
            #name: #raw_ty
        }
    }

    // raw_SYSCALL wraps uninit_SYSCALL, getting rid of the MaybeUninit usage.
    // These return the arguments forwarded to the uninit_SYSCALL call.
    pub fn to_raw_through_call(
        &self,
        is_input: bool,
        type_map: &TypeMap,
    ) -> TokenStream2 {
        let name = self.formatted_name();

        if is_input || self.is_primitive(type_map) {
            return quote! { #name };
        }
        if self.is_span_argument() {
            return quote! {

                unsafe {core::mem::transmute::<&mut [u8], &mut [::core::mem::MaybeUninit<u8>]>(#name)}
            };
        }
        if let Some(ty) = self.fixed_size_type(type_map) {
            return quote! {
                unsafe {core::mem::transmute::<&mut #ty, &mut ::core::mem::MaybeUninit<#ty>>(#name)}
            };
        }

        unimplemented!("Unknown arg type");
    }
}

fn generate_uninit_syscall_inputs(
    syscall: &Syscall,
    type_map: &TypeMap,
) -> Vec<TokenStream2> {
    // inputs
    let mut args: Vec<TokenStream2> = syscall
        .inputs
        .iter()
        .map(|inp| inp.to_uninit_param(true, type_map))
        .collect();

    // outputs
    let output_args: Vec<TokenStream2> = syscall
        .outputs
        .iter()
        .map(|outp| outp.to_uninit_param(false, type_map))
        .collect();

    args.extend(output_args);

    args
}

fn generate_raw_through_calls(
    syscall: &Syscall,
    type_map: &TypeMap,
) -> Vec<TokenStream2> {
    // inputs
    let mut args: Vec<TokenStream2> = syscall
        .inputs
        .iter()
        .map(|inp| inp.to_raw_through_call(true, type_map))
        .collect();

    // outputs
    let output_args: Vec<TokenStream2> = syscall
        .outputs
        .iter()
        .map(|outp| outp.to_raw_through_call(false, type_map))
        .collect();

    args.extend(output_args);

    args
}

fn generate_raw_syscall_args(
    syscall: &Syscall,
    type_map: &TypeMap,
) -> Vec<TokenStream2> {
    // inputs
    let mut args: Vec<TokenStream2> = syscall
        .inputs
        .iter()
        .map(|inp| inp.to_raw_param(true, type_map))
        .collect();

    // outputs
    let output_args: Vec<TokenStream2> = syscall
        .outputs
        .iter()
        .map(|outp| outp.to_raw_param(false, type_map))
        .collect();

    args.extend(output_args);

    args
}

fn generate_u64_syscall_args(
    syscall: &Syscall,
    type_map: &TypeMap,
    syscall_input_count: usize,
) -> Vec<TokenStream2> {
    // inputs
    let mut u64_args: Vec<TokenStream2> = syscall
        .inputs
        .iter()
        .flat_map(|inp| inp.to_u64s(type_map, true))
        .collect();

    // outputs
    let output_args: Vec<TokenStream2> = syscall
        .outputs
        .iter()
        .flat_map(|outp| outp.to_u64s(type_map, false))
        .collect();

    u64_args.extend(output_args);

    if u64_args.len() > syscall_input_count {
        return vec![quote! {
            compile_error!("Too many syscall args")
        }];
    }

    u64_args.resize(syscall_input_count, quote! { 0 });

    u64_args
}

fn generate_syscall_uninit_decl(
    syscall: &Syscall,
    type_map: &TypeMap,
    syscall_input_count: usize,
    doc: Option<&SyscallDoc>,
) -> TokenStream2 {
    //  pub fn uninit_SHA256(buffer : &[u8], hash : &mut MaybeUninit<SHA256Hash>) -> SyscallResult<()> {
    //
    //      // syscall will initialize outputs, if error is kNone
    //      let err: SysErr =
    //          syscall_wrapper(SyscallNo::kSHA256,
    //              buffer.as_ptr() as u64,
    //              buffer.len() as u64,
    //              hash.as_mut_ptr() as u64,
    //              0,
    //              0,
    //              0);
    //
    //      if !err.is_none() {
    //          return Err(err);
    //      }
    //
    //      Ok(())
    //  }

    let return_type = syscall.return_type();
    let post_syscall = if syscall.returns {
        quote! {
            if !err.is_none() {
                return Err(err);
            }

            Ok(())
        }
    } else {
        quote! {
            unsafe {::core::hint::unreachable_unchecked()}
        }
    };

    let u64_args =
        generate_u64_syscall_args(&syscall, type_map, syscall_input_count);
    let uninit_syscall_inputs =
        generate_uninit_syscall_inputs(&syscall, type_map);

    let uninit_ident = format_ident!("uninit_{}", &syscall.name);
    let syscall_no = format_ident!("k{}", &syscall.name);

    // Generate documentation
    let mut doc_lines = Vec::new();
    let has_outputs = !syscall.outputs.is_empty();
    let has_inputs = !syscall.inputs.is_empty();

    if let Some(doc) = doc {
        // Add main description
        if !doc.description.is_empty() {
            doc_lines.extend(doc.description.iter().cloned());
            doc_lines.push(String::new());
        }

        // Add inputs/outputs lists
        doc_lines.push("## Arguments".to_string());
        doc_lines.push(String::new());

        // Inputs
        doc_lines.push("### Inputs".to_string());
        if has_inputs {
            for input in &syscall.inputs {
                doc_lines.push(format!("- `{}`", input.name));
            }
        } else {
            doc_lines.push("- None".to_string());
        }
        doc_lines.push(String::new());

        // Outputs
        doc_lines.push("### Outputs".to_string());
        if has_outputs {
            doc_lines.push("The following parameters use `MaybeUninit` types and will be initialized by the syscall if the error is `kNone`:".to_string());
            for output in &syscall.outputs {
                doc_lines.push(format!("- `{}`", output.name));
            }
        } else {
            doc_lines.push("- None".to_string());
        }
        doc_lines.push(String::new());

        // Add note about uninit variant only if there are outputs
        if has_outputs {
            doc_lines.push(format!("This is the `uninit_{}` variant that takes `MaybeUninit` parameters for outputs.", &syscall.name));
            doc_lines.push(
                "The syscall will initialize outputs if the error is `kNone`."
                    .to_string(),
            );
            doc_lines.push(String::new());
        }

        // Add error documentation
        if !doc.errors.is_empty() {
            doc_lines.push("## Errors".to_string());
            doc_lines.push(String::new());
            for error in &doc.errors {
                doc_lines.push(format!("- **{}**:", error.code));
                doc_lines.extend(error.description.iter().cloned());
                doc_lines.push(String::new());
            }
        }
    } else {
        // Fallback documentation if no doc available
        // Add inputs/outputs lists
        doc_lines.push("## Arguments".to_string());
        doc_lines.push(String::new());

        // Inputs
        doc_lines.push("### Inputs".to_string());
        if has_inputs {
            for input in &syscall.inputs {
                doc_lines.push(format!("- `{}`", input.name));
            }
        } else {
            doc_lines.push("- None".to_string());
        }
        doc_lines.push(String::new());

        // Outputs
        doc_lines.push("### Outputs".to_string());
        if has_outputs {
            doc_lines.push("The following parameters use `MaybeUninit` types and will be initialized by the syscall if the error is `kNone`:".to_string());
            for output in &syscall.outputs {
                doc_lines.push(format!("- `{}`", output.name));
            }
            doc_lines.push(String::new());
            doc_lines.push(format!("This is the `uninit_{}` variant that takes `MaybeUninit` parameters for outputs.", &syscall.name));
        } else {
            doc_lines.push("- None".to_string());
        }
    }

    let doc_comment = generate_doc_comment(&doc_lines);

    quote! {
        #doc_comment
        pub fn #uninit_ident(#(#uninit_syscall_inputs,)*) -> #return_type {

            // syscall will initialize outputs, if error is kNone
            let err: SysErr =
                syscall_wrapper(SyscallNo::#syscall_no,
                    #(#u64_args),*);

            #post_syscall
        }
    }
}

fn generate_syscall_raw_decl(
    syscall: &Syscall,
    type_map: &TypeMap,
    doc: Option<&SyscallDoc>,
) -> TokenStream2 {
    let raw_syscall_inputs = generate_raw_syscall_args(&syscall, type_map);
    let forward_args = generate_raw_through_calls(&syscall, type_map);

    let uninit_ident = format_ident!("uninit_{}", &syscall.name);
    let raw_ident = format_ident!("raw_{}", &syscall.name);

    let return_type = syscall.return_type();

    // Generate documentation
    let mut doc_lines = Vec::new();
    let has_outputs = !syscall.outputs.is_empty();
    let has_inputs = !syscall.inputs.is_empty();

    if let Some(doc) = doc {
        // Add main description
        if !doc.description.is_empty() {
            doc_lines.extend(doc.description.iter().cloned());
            doc_lines.push(String::new());
        }

        // Add inputs/outputs lists
        doc_lines.push("## Arguments".to_string());
        doc_lines.push(String::new());

        // Inputs
        doc_lines.push("### Inputs".to_string());
        if has_inputs {
            for input in &syscall.inputs {
                doc_lines.push(format!("- `{}`", input.name));
            }
        } else {
            doc_lines.push("- None".to_string());
        }
        doc_lines.push(String::new());

        // Outputs
        doc_lines.push("### Outputs".to_string());
        if has_outputs {
            for output in &syscall.outputs {
                doc_lines.push(format!("- `{}`", output.name));
            }
        } else {
            doc_lines.push("- None".to_string());
        }
        doc_lines.push(String::new());

        // Add note about raw variant
        if has_outputs {
            doc_lines.push(format!(
                "This is the `raw_{}` variant that wraps `uninit_{}`.",
                &syscall.name, &syscall.name
            ));
            doc_lines.push(format!("It converts regular output types to `MaybeUninit` and calls [`uninit_{}`](uninit_{}).", &syscall.name, &syscall.name));
        } else {
            doc_lines.push(format!(
                "This is the a thin wrapper around [`uninit_{}`](uninit_{}).",
                &syscall.name, &syscall.name
            ));
        }
        doc_lines.push(String::new());

        // Add error documentation
        if !doc.errors.is_empty() {
            doc_lines.push("## Errors".to_string());
            doc_lines.push(String::new());
            for error in &doc.errors {
                doc_lines.push(format!("- **{}**:", error.code));
                doc_lines.extend(error.description.iter().cloned());
                doc_lines.push(String::new());
            }
        }
    } else {
        // Fallback documentation if no doc available
        // Add inputs/outputs lists
        doc_lines.push("## Arguments".to_string());
        doc_lines.push(String::new());

        // Inputs
        doc_lines.push("### Inputs".to_string());
        if has_inputs {
            for input in &syscall.inputs {
                doc_lines.push(format!("- `{}`", input.name));
            }
        } else {
            doc_lines.push("- None".to_string());
        }
        doc_lines.push(String::new());

        // Outputs
        doc_lines.push("### Outputs".to_string());
        if has_outputs {
            for output in &syscall.outputs {
                doc_lines.push(format!("- `{}`", output.name));
            }
            doc_lines.push(String::new());
            doc_lines.push(format!(
                "This is the `raw_{}` variant that wraps `uninit_{}`.",
                &syscall.name, &syscall.name
            ));
            doc_lines.push(format!("It converts regular output types to `MaybeUninit` and calls `uninit_{}`.", &syscall.name));
        } else {
            doc_lines.push("- None".to_string());
            doc_lines.push(String::new());
            doc_lines.push(format!(
                "This is the `raw_{}` variant that wraps `uninit_{}`.",
                &syscall.name, &syscall.name
            ));
        }
    }

    let doc_comment = generate_doc_comment(&doc_lines);

    quote! {
        #doc_comment
        pub fn #raw_ident(#(#raw_syscall_inputs,)*) -> #return_type {
            #uninit_ident(#(#forward_args,)*)
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct SyscallDoc {
    description: Vec<String>,
    errors: Vec<ErrorDoc>,
}

#[derive(Debug, serde::Deserialize)]
struct ErrorDoc {
    code: String,
    description: Vec<String>,
}

fn load_syscall_yaml(file_path: String) -> SyscallSpec {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let yaml_path = std::path::PathBuf::from(manifest_dir).join(&file_path);

    let content = std::fs::read_to_string(&yaml_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", yaml_path.display()));

    serde_yaml::from_str(&content).unwrap()
}

fn load_syscall_docs(json_path: String) -> BTreeMap<String, SyscallDoc> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let docs_path = std::path::PathBuf::from(manifest_dir).join(&json_path);

    if !docs_path.exists() {
        // Return empty map if docs don't exist (for backwards compatibility)
        return BTreeMap::new();
    }

    let content = std::fs::read_to_string(&docs_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", docs_path.display()));

    serde_json::from_str(&content).unwrap_or_else(|_| {
        eprintln!("Warning: Failed to parse syscall docs JSON, continuing without docs");
        BTreeMap::new()
    })
}

fn generate_doc_comment(lines: &[String]) -> TokenStream2 {
    if lines.is_empty() {
        return quote! {};
    }

    let doc_lines: Vec<TokenStream2> = lines
        .iter()
        .map(|line| {
            let line_str = line.as_str();
            // Format as /// doc comment - use #[doc = "..."] for proper escaping
            quote! {
                #[doc = #line_str]
            }
        })
        .collect();

    quote! {
        #(#doc_lines)*
    }
}

pub fn generate_syscall_decls(input: TokenStream) -> TokenStream {
    let file_path = syn::parse_macro_input!(input as syn::LitStr).value();
    let spec = load_syscall_yaml(file_path.clone());
    let type_map = spec.get_type_map();

    // Try to load documentation JSON (relative to the YAML file)
    let yaml_path_buf = std::path::PathBuf::from(&file_path);
    let yaml_dir = yaml_path_buf.parent().unwrap();
    let docs_path = yaml_dir.join("syscalls_docs.json");
    let docs = load_syscall_docs(docs_path.to_string_lossy().to_string());

    let decls = spec.syscalls.iter().map(|func| {
        let doc = docs.get(&func.name);
        let uninit = generate_syscall_uninit_decl(
            &func,
            &type_map,
            spec.syscall_input_count,
            doc,
        );
        let raw = generate_syscall_raw_decl(&func, &type_map, doc);
        quote! {
            #uninit
            #raw
        }
    });

    quote! {
        #(#decls)*
    }
    .into()
}

fn generate_syscall_no_line(syscall: &Syscall) -> TokenStream2 {
    let name = format_ident!("k{}", &syscall.name);
    let num = syscall.number;
    quote! {
        #name = #num as i32
    }
}

pub fn generate_syscall_numbers(input: TokenStream) -> TokenStream {
    let file_path = syn::parse_macro_input!(input as syn::LitStr).value();
    let spec = load_syscall_yaml(file_path);

    let lines = spec
        .syscalls
        .iter()
        .map(|func| generate_syscall_no_line(&func));

    quote! {
        #[repr(i32)]
        #[derive(Eq,PartialEq,Copy,Debug,Clone)]
        pub enum SyscallNo {
            #(#lines,)*
        }
    }
    .into()
}
