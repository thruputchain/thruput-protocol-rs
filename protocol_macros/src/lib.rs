use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Data, Fields};

#[proc_macro_derive(ReadXdr)]
pub fn derive_read_xdr(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_read_xdr(&ast)
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
                        #field_name : #ty ::read_xdr(r)?,
                    }
                });

                quote! {
                    impl ReadXdr for #name {
                        fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
                            r.with_limited_depth(|r| {
                                Ok(#name {
                                    #(#read_xdr_fields)*
                                })
                            })
                        }
                    }
                }
            }
            _ => unimplemented!("Only named fields are supported"),
        },
        _ => unimplemented!("Only structs are supported"),
    };
    trait_output.into()
}

#[proc_macro_derive(WriteXdr)]
pub fn derive_write_xdr(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_write_xdr(&ast)
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
        _ => unimplemented!("Only structs are supported"),
    };
    trait_output.into()
}
