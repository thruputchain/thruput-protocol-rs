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
    /// Rust newtype wrappers (the transparent wrapper structs in the
    /// generated code) pass `.into_inner()` when passed by value.
    #[serde(default = "default_false")]
    newtype: bool,
    /// Enum types and other tuple structs with a public `.0` (e.g.
    /// `TransactionStatistic(i32)`) pass `.0 as SyscallWord` when passed by
    /// value.
    #[serde(default = "default_false")]
    dot_zero_type: bool,
}

#[derive(Debug)]
struct TypeInfo {
    rust_name: String,
    rust_uses_as_ptr: bool,
    primitive_type: bool,
    newtype: bool,
    dot_zero_type: bool,
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
                    newtype: elt.newtype,
                    dot_zero_type: elt.dot_zero_type,
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
                    quote! { &mut ::core::mem::MaybeUninit<#ty> }
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
                        "syscall arg `{}`: couldn't parse `typename` field as a type expression: \"{}\"",
                        self.name, typename
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

    /// Returns true if this arg's type is a generated newtype wrapper that
    /// needs `.into_inner() as SyscallWord` instead of `as SyscallWord`.
    fn is_newtype(&self, type_map: &TypeMap) -> bool {
        if let Some(type_details) = self.get_type_details(type_map) {
            type_details.newtype
        } else {
            false
        }
    }

    fn is_dot_zero_type(&self, type_map: &TypeMap) -> bool {
        if let Some(type_details) = self.get_type_details(type_map) {
            type_details.dot_zero_type
        } else {
            false
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
            panic!("{}", self.unknown_type_message());
        }
    }

    /// Diagnostic for an arg that is neither a span argument nor has a
    /// resolvable `typename`. Kept in one place so every rejection site
    /// names the offending arg the same way.
    fn unknown_type_message(&self) -> String {
        format!(
            "syscall arg `{}` has unknown type: `span_argument` is false and \
             `typename` is {}; add a `typename` (or a type_mappings entry \
             for it) in syscalls.yaml",
            self.name,
            match self.typename.as_deref() {
                Some(t) => format!("\"{t}\""),
                None => "missing".to_string(),
            }
        )
    }

    /// Lowers this argument into one or more syscall slots, each a
    /// `SyscallWord` (pointer-width: `u32` on riscv32, `u64` on native). The
    /// `SyscallWord` alias is resolved in the consuming crate, so each cast is
    /// correct for the target the bindings are compiled for.
    pub fn to_words(
        &self,
        type_map: &TypeMap,
        is_input: bool,
    ) -> Vec<TokenStream2> {
        let name = self.formatted_name();

        if self.is_span_argument() {
            let span_len = quote! { #name.len() as SyscallWord };
            let span_ptr = if is_input {
                quote! {
                    #name.as_ptr() as SyscallWord
                }
            } else {
                quote! {
                    #name.as_mut_ptr() as SyscallWord
                }
            };
            vec![span_ptr, span_len]
        } else if let Some(arg_ty) = self.fixed_size_type(type_map) {
            let arg = if self.optional {
                assert!(is_input);
                if self.uses_as_ptr(type_map) {
                    quote! {
                        match #name {
                            Some(x) => x.as_ptr() as SyscallWord,
                            None => SyscallWord::MAX,
                        }
                    }
                } else {
                    quote! {
                        match #name {
                            Some(x) => x as *const _ as SyscallWord,
                            None => SyscallWord::MAX,
                        }
                    }
                }
            } else if self.is_primitive(type_map) {
                if is_input {
                    // A primitive input is passed by value in a single slot, so
                    // it MUST fit in a `SyscallWord`. On riscv32 that slot is
                    // 32 bits, so this assertion (resolved per-target in the
                    // consuming crate) turns an oversized by-value argument into
                    // a compile error instead of a silent truncation. Wider
                    // values must be passed by reference.
                    let value = if self.is_newtype(type_map) {
                        quote! { #name.into_inner() }
                    } else if self.is_dot_zero_type(type_map) {
                        quote! { #name.0 }
                    } else {
                        quote! { #name }
                    };
                    quote! {
                        {
                            const _: () = assert!(
                                ::core::mem::size_of::<#arg_ty>()
                                    <= ::core::mem::size_of::<SyscallWord>(),
                                "syscall by-value primitive argument is wider than a syscall word; pass it by reference instead"
                            );
                            #value as SyscallWord
                        }
                    }
                } else {
                    quote! { #name as *mut _ as SyscallWord }
                }
            } else if self.uses_as_ptr(type_map) {
                if is_input {
                    quote! { #name.as_ptr() as SyscallWord }
                } else {
                    quote! { #name.as_mut_ptr() as SyscallWord }
                }
            } else {
                // else it's, at this point in time, a 128 bit int
                if is_input {
                    quote! { #name as *const _ as SyscallWord }
                } else {
                    quote! { #name as *mut _ as SyscallWord }
                }
            };

            vec![arg]
        } else {
            panic!("{}", self.unknown_type_message());
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

    pub fn rust_type(&self, type_map: &TypeMap) -> TokenStream2 {
        if self.is_span_argument() {
            quote! { () } // Spans don't have a simple return type
        } else if let Some(ty) = self.fixed_size_type(type_map) {
            ty
        } else {
            quote! { () }
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

        if is_input {
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

        panic!("{}", self.unknown_type_message());
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

fn generate_word_args(
    syscall: &Syscall,
    type_map: &TypeMap,
    syscall_input_count: usize,
) -> Vec<TokenStream2> {
    // inputs
    let mut word_args: Vec<TokenStream2> = syscall
        .inputs
        .iter()
        .flat_map(|inp| inp.to_words(type_map, true))
        .collect();

    // outputs
    let output_args: Vec<TokenStream2> = syscall
        .outputs
        .iter()
        .flat_map(|outp| outp.to_words(type_map, false))
        .collect();

    word_args.extend(output_args);

    if word_args.len() > syscall_input_count {
        return vec![quote! {
            compile_error!("Too many syscall args")
        }];
    }

    word_args.resize(syscall_input_count, quote! { 0 });

    word_args
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
    //          SysErr(syscall_wrapper(SyscallNo::kSHA256.0 as SyscallWord,
    //              buffer.as_ptr() as SyscallWord,
    //              buffer.len() as SyscallWord,
    //              hash.as_mut_ptr() as SyscallWord,
    //              0,
    //              0,
    //              0));
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

    let word_args = generate_word_args(syscall, type_map, syscall_input_count);
    let uninit_syscall_inputs =
        generate_uninit_syscall_inputs(syscall, type_map);

    let uninit_ident = format_ident!("uninit_{}", &syscall.name);
    let syscall_no = format_ident!("k{}", &syscall.name);

    // Generate documentation
    let mut doc_lines = Vec::new();

    if let Some(doc) = doc {
        add_description_section(&mut doc_lines, doc);
        add_uninit_arguments_section(
            &mut doc_lines,
            &syscall.inputs,
            &syscall.outputs,
        );
        add_error_section(&mut doc_lines, &doc.errors);
    } else {
        panic!(
            "No documentation found for syscall `{}` in syscalls_docs.json; \
             regenerate it with scripts/generate.sh (protocol/generate_syscalls_doc.py)",
            &syscall.name
        );
    }

    let doc_comment = generate_doc_comment(&doc_lines);

    quote! {
        #doc_comment
        #[inline(always)]
        pub fn #uninit_ident(#(#uninit_syscall_inputs,)*) -> #return_type {

            // syscall will initialize outputs, if error is kNone
            let err: SysErr =
                SysErr(syscall_wrapper(SyscallNo::#syscall_no.0 as SyscallWord,
                    #(#word_args),*));

            #post_syscall
        }
    }
}

fn generate_syscall_raw_decl(
    syscall: &Syscall,
    type_map: &TypeMap,
    doc: Option<&SyscallDoc>,
) -> TokenStream2 {
    let raw_syscall_inputs = generate_raw_syscall_args(syscall, type_map);
    let forward_args = generate_raw_through_calls(syscall, type_map);

    let uninit_ident = format_ident!("uninit_{}", &syscall.name);
    let raw_ident = format_ident!("raw_{}", &syscall.name);

    let return_type = syscall.return_type();

    // Generate documentation
    let mut doc_lines = Vec::new();
    let has_outputs = !syscall.outputs.is_empty();

    if let Some(doc) = doc {
        add_description_section(&mut doc_lines, doc);
        add_raw_arguments_section(
            &mut doc_lines,
            &syscall.inputs,
            &syscall.outputs,
        );

        // Add note about raw variant
        if has_outputs {
            doc_lines.push(format!(
                "This is the `raw_{}` variant that wraps `uninit_{}`.",
                &syscall.name, &syscall.name
            ));
            doc_lines.push(format!("It converts regular output types to `MaybeUninit` and calls [`uninit_{}`](uninit_{}).", &syscall.name, &syscall.name));
        } else {
            doc_lines.push(format!(
                "This is a thin wrapper around [`uninit_{}`](uninit_{}).",
                &syscall.name, &syscall.name
            ));
        }
        doc_lines.push(String::new());

        add_error_section(&mut doc_lines, &doc.errors);
    } else {
        panic!(
            "No documentation found for syscall `{}` in syscalls_docs.json; \
             regenerate it with scripts/generate.sh (protocol/generate_syscalls_doc.py)",
            &syscall.name
        );
    }

    let doc_comment = generate_doc_comment(&doc_lines);

    quote! {
        #doc_comment
        #[inline(always)]
        pub fn #raw_ident(#(#raw_syscall_inputs,)*) -> #return_type {
            #uninit_ident(#(#forward_args,)*)
        }
    }
}

fn generate_syscall_result_decl(
    syscall: &Syscall,
    type_map: &TypeMap,
    doc: Option<&SyscallDoc>,
) -> TokenStream2 {
    // Skip generation for syscalls with span outputs - these need pre-allocated buffers
    if syscall.outputs.iter().any(|out| out.span_argument) {
        return quote! {};
    }

    // Generate input parameters (no outputs in signature)
    let input_params: Vec<_> = syscall
        .inputs
        .iter()
        .map(|inp| inp.to_raw_param(true, type_map))
        .collect();

    // Collect output information
    let output_types: Vec<_> = syscall
        .outputs
        .iter()
        .map(|out| out.rust_type(type_map))
        .collect();
    let output_names: Vec<_> = syscall
        .outputs
        .iter()
        .map(|out| out.formatted_name())
        .collect();

    let uninit_ident = format_ident!("uninit_{}", &syscall.name);
    let result_ident = format_ident!("{}", &syscall.name);

    // Build uninit call arguments
    let uninit_call_args: Vec<_> = syscall
        .inputs
        .iter()
        .map(|inp| {
            let name = inp.formatted_name();
            quote! { #name }
        })
        .chain(output_names.iter().map(|name| {
            quote! { &mut #name }
        }))
        .collect();

    // Handle non-returning syscalls separately
    if !syscall.returns {
        let function_body = quote! {
            #uninit_ident(#(#uninit_call_args,)*);
        };

        let mut doc_lines = Vec::new();
        if let Some(doc) = doc {
            add_description_section(&mut doc_lines, doc);
            add_arguments_section(&mut doc_lines, &syscall.inputs);
            doc_lines.push(format!(
                "This is a convenience wrapper around [`uninit_{}`](uninit_{}).",
                &syscall.name, &syscall.name
            ));
        } else {
            panic!(
                "No documentation found for syscall `{}` in syscalls_docs.json; \
             regenerate it with scripts/generate.sh (protocol/generate_syscalls_doc.py)",
                &syscall.name
            );
        }
        let doc_comment = generate_doc_comment(&doc_lines);

        return quote! {
            #doc_comment
            pub fn #result_ident(#(#input_params,)*) -> ! {
                #function_body
            }
        };
    }

    // Build return type
    let return_type = if output_types.is_empty() {
        quote! { SyscallResult<()> }
    } else if output_types.len() == 1 {
        let ty = &output_types[0];
        quote! { SyscallResult<#ty> }
    } else {
        quote! { SyscallResult<(#(#output_types,)*)> }
    };

    // Build output declarations
    let output_decls: Vec<_> = output_names
        .iter()
        .zip(output_types.iter())
        .map(|(name, ty)| {
            quote! { let mut #name = ::core::mem::MaybeUninit::<#ty>::uninit(); }
        })
        .collect();

    // Build success value
    let success_value = if output_names.is_empty() {
        quote! { Ok(()) }
    } else if output_names.len() == 1 {
        let name = &output_names[0];
        quote! { Ok(unsafe { #name.assume_init() }) }
    } else {
        quote! { Ok((#(unsafe { #output_names.assume_init() }),*)) }
    };

    let function_body = quote! {
        #(#output_decls)*
        #uninit_ident(#(#uninit_call_args,)*)?;
        #success_value
    };

    // Generate documentation
    let mut doc_lines = Vec::new();

    if let Some(doc) = doc {
        add_description_section(&mut doc_lines, doc);
        add_arguments_section(&mut doc_lines, &syscall.inputs);
        add_outputs_section(&mut doc_lines, &syscall.outputs);

        // Add note about this being the convenient wrapper
        doc_lines.push(format!(
            "This is a convenience wrapper around [`uninit_{}`](uninit_{}) that returns the result as `SyscallResult<T>`.",
            &syscall.name, &syscall.name
        ));
        doc_lines.push(String::new());

        add_error_section(&mut doc_lines, &doc.errors);
    } else {
        panic!(
            "No documentation found for syscall `{}` in syscalls_docs.json; \
             regenerate it with scripts/generate.sh (protocol/generate_syscalls_doc.py)",
            &syscall.name
        );
    }

    let doc_comment = generate_doc_comment(&doc_lines);

    quote! {
        #doc_comment
        pub fn #result_ident(#(#input_params,)*) -> #return_type {
            #function_body
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

    let content = std::fs::read_to_string(&yaml_path).unwrap_or_else(|e| {
        panic!("Failed to read {}: {e}", yaml_path.display())
    });

    serde_saphyr::from_str(&content).unwrap_or_else(|e| {
        panic!("Failed to parse syscall spec {}: {e}", yaml_path.display())
    })
}

fn load_syscall_docs(json_path: String) -> BTreeMap<String, SyscallDoc> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let docs_path = std::path::PathBuf::from(manifest_dir).join(&json_path);

    // Every syscall needs its documentation, so a missing or unreadable file
    // is an error here rather than at the first undocumented syscall.
    let content = std::fs::read_to_string(&docs_path).unwrap_or_else(|e| {
        panic!(
            "Failed to read {}: {e}; regenerate it with scripts/generate.sh",
            docs_path.display()
        )
    });

    serde_json::from_str(&content).unwrap_or_else(|e| {
        panic!(
            "Failed to parse syscall docs {}: {e}; regenerate it with \
             scripts/generate.sh",
            docs_path.display()
        )
    })
}

fn generate_doc_comment(lines: &[String]) -> TokenStream2 {
    if lines.is_empty() {
        return quote! {};
    }

    let doc_lines: Vec<TokenStream2> = lines
        .iter()
        .map(|line| {
            quote! {
                #[doc = #line]
            }
        })
        .collect();

    quote! {
        #(#doc_lines)*
    }
}

fn add_description_section(doc_lines: &mut Vec<String>, doc: &SyscallDoc) {
    if !doc.description.is_empty() {
        for (para_idx, paragraph) in doc.description.iter().enumerate() {
            // Split each paragraph on newlines to get individual lines
            for line in paragraph.split('\n') {
                let trimmed = line.trim_end();
                if !trimmed.is_empty() {
                    doc_lines.push(trimmed.to_string());
                }
            }

            // Add blank line between paragraphs (but not after the last one)
            if para_idx < doc.description.len() - 1 {
                doc_lines.push(String::new());
            }
        }
        doc_lines.push(String::new());
    }
}

fn add_arguments_section(doc_lines: &mut Vec<String>, inputs: &[Arg]) {
    if !inputs.is_empty() {
        doc_lines.push("## Arguments".to_string());
        doc_lines.push(String::new());
        for input in inputs {
            doc_lines.push(format!("- `{}`", input.name));
        }
        doc_lines.push(String::new());
    }
}

fn add_outputs_section(doc_lines: &mut Vec<String>, outputs: &[Arg]) {
    if !outputs.is_empty() {
        doc_lines.push("## Outputs".to_string());
        doc_lines.push(String::new());
        for output in outputs {
            doc_lines.push(format!("- `{}`", output.name));
        }
        doc_lines.push(String::new());
    }
}

fn add_error_section(doc_lines: &mut Vec<String>, errors: &[ErrorDoc]) {
    if !errors.is_empty() {
        doc_lines.push("## Errors".to_string());
        doc_lines.push(String::new());
        for error in errors {
            doc_lines.push(format!("- **{}**:", error.code));
            doc_lines.extend(error.description.iter().cloned());
            doc_lines.push(String::new());
        }
    }
}

fn add_uninit_arguments_section(
    doc_lines: &mut Vec<String>,
    inputs: &[Arg],
    outputs: &[Arg],
) {
    doc_lines.push("## Arguments".to_string());
    doc_lines.push(String::new());

    // Inputs subsection
    doc_lines.push("### Inputs".to_string());
    if !inputs.is_empty() {
        for input in inputs {
            doc_lines.push(format!("- `{}`", input.name));
        }
    } else {
        doc_lines.push("- None".to_string());
    }
    doc_lines.push(String::new());

    // Outputs subsection
    doc_lines.push("### Outputs".to_string());
    if !outputs.is_empty() {
        doc_lines.push("The following parameters use `MaybeUninit` types and are written by the syscall only if it returns `kSyscallErrorCodeNone`. A fixed-size output is then fully initialized; a span output has only its first `min(len, written_len)` bytes initialized:".to_string());
        for output in outputs {
            doc_lines.push(format!("- `{}`", output.name));
        }
    } else {
        doc_lines.push("- None".to_string());
    }
    doc_lines.push(String::new());
}

fn add_raw_arguments_section(
    doc_lines: &mut Vec<String>,
    inputs: &[Arg],
    outputs: &[Arg],
) {
    doc_lines.push("## Arguments".to_string());
    doc_lines.push(String::new());

    // Inputs subsection
    doc_lines.push("### Inputs".to_string());
    if !inputs.is_empty() {
        for input in inputs {
            doc_lines.push(format!("- `{}`", input.name));
        }
    } else {
        doc_lines.push("- None".to_string());
    }
    doc_lines.push(String::new());

    // Outputs subsection
    doc_lines.push("### Outputs".to_string());
    if !outputs.is_empty() {
        doc_lines.push("The following parameters are written by the syscall only if it returns `kSyscallErrorCodeNone`. A span output then has only its first `min(len, written_len)` bytes written:".to_string());
        for output in outputs {
            doc_lines.push(format!("- `{}`", output.name));
        }
    } else {
        doc_lines.push("- None".to_string());
    }
    doc_lines.push(String::new());
}

/// Fails fast, naming the syscall and arg, when an arg can never be
/// lowered. The later per-arg panics cannot see the enclosing syscall,
/// so this pass turns spec mistakes into a diagnostic that names both.
fn validate_spec_args(spec: &SyscallSpec, file_path: &str) {
    for syscall in &spec.syscalls {
        let args = syscall
            .inputs
            .iter()
            .map(|arg| ("input", arg))
            .chain(syscall.outputs.iter().map(|arg| ("output", arg)));
        for (kind, arg) in args {
            if !arg.is_span_argument() && arg.typename.is_none() {
                panic!(
                    "in {file_path}: syscall `{}` {kind}: {}",
                    syscall.name,
                    arg.unknown_type_message()
                );
            }
        }
    }
}

pub fn generate_syscall_decls(input: TokenStream) -> TokenStream {
    let file_path = syn::parse_macro_input!(input as syn::LitStr).value();
    let spec = load_syscall_yaml(file_path.clone());
    validate_spec_args(&spec, &file_path);
    let type_map = spec.get_type_map();

    // Load the documentation JSON from the crate directory. scripts/generate.sh
    // writes it into thruput-xdr from protocol/syscalls.yaml.
    let docs_path = "syscalls_docs.json";
    let docs = load_syscall_docs(docs_path.to_string());

    let decls = spec.syscalls.iter().map(|func| {
        let doc = docs.get(&func.name);
        let uninit = generate_syscall_uninit_decl(
            func,
            &type_map,
            spec.syscall_input_count,
            doc,
        );
        let raw = generate_syscall_raw_decl(func, &type_map, doc);
        let result = generate_syscall_result_decl(func, &type_map, doc);
        quote! {
            #uninit
            #raw
            #result
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
        pub const #name: Self = Self(#num as u32);
    }
}

fn generate_syscall_no_display_line(syscall: &Syscall) -> TokenStream2 {
    let name = syscall.name.as_str();
    let num: u32 = syscall.number.try_into().unwrap_or_else(|_| {
        panic!(
            "syscall `{}` has negative number {}; syscall numbers must fit in u32",
            syscall.name, syscall.number
        )
    });
    quote! {
        #num => Some(#name),
    }
}

pub fn generate_syscall_numbers(input: TokenStream) -> TokenStream {
    let file_path = syn::parse_macro_input!(input as syn::LitStr).value();
    let spec = load_syscall_yaml(file_path);

    let lines = spec.syscalls.iter().map(generate_syscall_no_line);
    let display_lines =
        spec.syscalls.iter().map(generate_syscall_no_display_line);

    quote! {
        /// Syscall numbers identifying each system call.
        /// Generated from the syscalls.yaml specification.
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        #[cfg_attr(feature = "xdrs", derive(xdrs::ReadXDR, xdrs::WriteXDR, xdrs::ConstWriteXDR, xdrs::ConstXDRSize))]
        #[derive(Eq,PartialEq,Copy,Debug,Clone,Hash)]
        #[repr(transparent)]
        pub struct SyscallNo(pub u32);

        impl SyscallNo {
            #(#lines)*
        }

        #[cfg(feature = "std")]
        const _: () = {
            use core::fmt::{self, Display};

            impl Display for SyscallNo {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let name = match self.0 {
                        #(#display_lines)*
                        _ => None,
                    };

                    if let Some(name) = name {
                        write!(f, "{}", name)
                    } else {
                        write!(f, "#{}", self.0)
                    }
                }
            }
        };
    }
    .into()
}
