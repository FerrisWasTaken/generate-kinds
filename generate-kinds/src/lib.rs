use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

/// This macro is not meant to be used directly. It should be used with the `get-kinds` crate
#[proc_macro_derive(Kind)]
pub fn kinds(stream: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(stream).expect("code is invalid");
    let ident = ast.ident;
    let vis = ast.vis;
    match ast.data {
        Data::Struct(_) => {
            quote! {
                impl ::get_kinds::Kind for #ident {
                    #vis fn kind<'a>(&self) -> &'a str {
                        stringify!(#ident)
                    }
                }
            }.into()
        },
        Data::Enum(e) => {
            let variants = e.variants.iter();
            quote! {
                impl ::get_kinds::Kind for #ident {
                    #vis fn kind<'a>(&self) -> &'a str {
                        match self {
                            #(#ident::#variants => stringify!(#ident::#variants),)
                            *
                        }
                    }
                }
            }.into()
        },
        Data::Union(_) => todo!(),
    }
}
