use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, Fields};

// XDR serialization generation for Structs and Unions
#[proc_macro_derive(ReadXdr)]
pub fn derive_read_xdr(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_read_xdr(&ast)
}

#[proc_macro_derive(WriteXdr)]
pub fn derive_write_xdr(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_write_xdr(&ast)
}

// XDR serialization generation for XDR Enums
#[proc_macro_derive(ReadXdrEnum)]
pub fn derive_read_xdr_enum(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_read_xdr_enum(&ast)
}

#[proc_macro_derive(WriteXdrEnum)]
pub fn derive_write_xdr_enum(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_write_xdr_enum(&ast)
}

fn impl_derive_read_union(name: &syn::Ident, data_enum: &syn::DataEnum) -> Option<TokenStream2> {
    let match_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        // Extract discriminant (e.g. `= CryptographicScheme::Ed25519 as i32`)
        let discriminant = match &variant.discriminant {
            Some((_, expr)) => expr,
            None => panic!("Each variant must have an explicit discriminant"),
        };

        match &variant.fields {
            // Expect struct-like field: `{ field_name: field_type }`
            Fields::Named(fields_named) => {
                if fields_named.named.len() == 1 {
                    let field = fields_named.named.first().unwrap();
                    let field_name = &field.ident;
                    let field_type = &field.ty;

                    return Some(quote! {
                        x if x == #discriminant => {
                            Ok(Self::#variant_name {
                                #field_name: <#field_type as ReadXdr>::read_xdr(r)?,
                            })
                        }
                    });
                } else {
                    return None;
                }
            }
            Fields::Unit => {
                return Some(quote! {
                    x if x == #discriminant => {
                        Ok(Self::#variant_name)
                    }
                });
            }
            _ => {
                return None;
            }
        };
    });

    for x in match_arms.clone() {
        match x {
            None => {
                return None;
            }
            _ => {}
        }
    }

    Some(quote! {
        impl ReadXdr for #name {
            fn read_xdr<R: std::io::Read>(r: &mut Limited<R>) -> Result<Self, Error> {
                r.with_limited_depth(|r| {
                    let e = i32::read_xdr(r)?;
                    match e {
                        #(#match_arms,)*

                        #[allow(unreachable_patterns)]
                        _ => Err(Error::Invalid),
                    }
                })
            }
        }
    })
}

fn impl_derive_write_union(name: &syn::Ident, data_enum: &syn::DataEnum) -> Option<TokenStream2> {
    let match_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            // Expect struct-like field: `{ field_name: field_type }`
            Fields::Named(fields_named) => {
                if fields_named.named.len() == 1 {
                    let field = fields_named.named.first().unwrap();
                    let field_name = &field.ident;
                    return Some(quote! {
                        Self::#variant_name { #field_name } => { #field_name.write_xdr(w)?; }
                    });
                }
                return None;
            }
            // This handles the case of a void union arm
            Fields::Unit => {
                return Some(quote! {
                    Self::#variant_name => {} // nothing to do in void case, after discriminant is written
                });
            }
            _ => {
                return None;
            }
        };
    });

    for x in match_arms.clone() {
        match x {
            None => {
                return None;
            }
            _ => {}
        }
    }

    Some(quote! {

        impl WriteXdr for #name {
            fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
                w.with_limited_depth(|w| {
                    let discriminant: i32 = unsafe { *(self as *const Self as *const i32) };
                    discriminant.write_xdr(w)?;
                    match self {
                        #(#match_arms,)*
                    };
                    Ok(())
                })
            }
        }
    })
}

fn impl_derive_read_enum(name: &syn::Ident, data_enum: &syn::DataEnum) -> Option<TokenStream2> {
    let match_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        // Extract discriminant (e.g. `= CryptographicScheme::Ed25519 as i32`)
        let discriminant = match &variant.discriminant {
            Some((_, expr)) => expr,
            None => panic!("Each variant must have an explicit discriminant"),
        };

        match &variant.fields {
            Fields::Unit => (),
            _ => {
                return None;
            }
        };

        Some(quote! {
            x if x == #discriminant => Self::#variant_name
        })
    });

    for x in match_arms.clone() {
        match x {
            None => {
                return None;
            }
            _ => {}
        }
    }

    Some(quote! {
        impl TryFrom<i32> for #name {
            type Error = Error;

            fn try_from(i: i32) -> Result<Self, Error> {
                let e = match i {
                    #(#match_arms,)*

                    #[allow(unreachable_patterns)]
                    _ => return Err(Error::Invalid),
                };
                Ok(e)
            }
        }

        impl ReadXdr for #name {
            fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
                r.with_limited_depth(|r| {
                    let e = i32::read_xdr(r)?;
                    let v: Self = e.try_into()?;
                    Ok(v)
                })
            }
        }
    })
}

fn impl_derive_write_enum(name: &syn::Ident, data_enum: &syn::DataEnum) -> Option<TokenStream2> {
    for x in data_enum.variants.iter() {
        match x.fields {
            Fields::Unit => (),
            _ => {
                return None;
            }
        }
    }

    Some(quote! {
        impl WriteXdr for #name {
            fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
                w.with_limited_depth(|w| {
                    let i: i32 = self.clone() as i32;
                    i.write_xdr(w)
                })
            }
        }
    })
}

fn impl_derive_read_xdr(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Generates code of the form
    // impl ReadXdr for ContractDeploymentHashPreimage {
    // fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
    //    r.with_limited_depth(|r| {
    //        Ok(ContractDeploymentHashPreimage {
    //            deployer_address : ContractAddress::read_xdr(r)?,
    //            nonce : SHA256Hash::read_xdr(r)?,
    //            contract_hash : SHA256Hash::read_xdr(r)?,
    //        })
    //    })
    //  }
    // }

    let trait_output = match ast.data {
        Data::Struct(ref data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let read_xdr_fields = fields_named.named.iter().map(|f| {
                    let field_name = &f.ident;

                    let ty = &f.ty;

                    quote! {
                        #field_name :  <#ty as ReadXdr>::read_xdr(r)?
                    }
                });

                quote! {
                    impl ReadXdr for #name {
                        fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
                            r.with_limited_depth(|r| {
                                Ok(Self {
                                    #(#read_xdr_fields,)*
                                })
                            })
                        }
                    }
                }
            }
            _ => unimplemented!("Only named fields are supported"),
        },
        Data::Enum(ref data_enum) => {
            // Two types of rust enums used here, corresponding to Unions and Enums in XDR.
            if let Some(union_tokens) = impl_derive_read_union(name, data_enum) {
                union_tokens
            } else {
                unimplemented!("Invalid usage of rust enum for XDR types");
            }
        }
        _ => unimplemented!("Only structs and unions are supported"),
    };
    trait_output.into()
}

fn impl_derive_write_xdr(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Generates code of the form
    // impl WriteXdr for ContractDeploymentHashPreimage {
    // fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
    //    w.with_limited_depth(|w| {
    //            self.deployer_address.write_xdr(w)?;
    //            self.nonce.write_xdr(w)?;
    //            self.contract_hash.write_xdr(w)?;
    //        Ok(())
    //    })
    // }
    //}

    let trait_output = match ast.data {
        Data::Struct(ref data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let write_xdr_fields = fields_named.named.iter().map(|f| {
                    let field_name = &f.ident;

                    quote! {
                        self.#field_name.write_xdr(w)?;
                    }
                });

                quote! {
                    impl WriteXdr for #name {
                        fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
                            w.with_limited_depth(|w| {
                                #(#write_xdr_fields)*
                                Ok(())
                            })
                        }
                    }
                }
            }
            _ => unimplemented!("Only named fields are supported"),
        },
        Data::Enum(ref data_enum) => {
            // Two types of rust enums used here, corresponding to Unions and Enums in XDR.
            if let Some(union_tokens) = impl_derive_write_union(name, data_enum) {
                union_tokens
            } else {
                unimplemented!("Invalid usage of rust enum for XDR types");
            }
        }
        _ => unimplemented!("Only structs are supported"),
    };
    trait_output.into()
}

fn impl_derive_read_xdr_enum(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let trait_output = match ast.data {
        Data::Enum(ref data_enum) => {
            // Two types of rust enums used here, corresponding to Unions and Enums in XDR.
            if let Some(enum_tokens) = impl_derive_read_enum(name, data_enum) {
                enum_tokens
            } else {
                unimplemented!("Invalid usage of rust enum for XDR types");
            }
        }
        _ => unimplemented!("Only structs and unions are supported"),
    };
    trait_output.into()
}

fn impl_derive_write_xdr_enum(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let trait_output = match ast.data {
        Data::Enum(ref data_enum) => {
            if let Some(enum_tokens) = impl_derive_write_enum(name, data_enum) {
                enum_tokens
            } else {
                unimplemented!("Invalid usage of rust enum for XDR types");
            }
        }
        _ => unimplemented!("Only structs are supported"),
    };
    trait_output.into()
}
