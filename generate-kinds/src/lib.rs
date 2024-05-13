use proc_macro::TokenStream;
use quote::quote;
use syn::Item;

/// This macro is not meant to be used directly. It should be used with the `get-kinds` crate
#[proc_macro_derive(GetKind)]
pub fn kinds(stream: TokenStream) -> TokenStream {
    let ast: Item = syn::parse(stream).expect("code is invalid");
    if let Item::Enum(ast) = ast {
        let ident = &ast.ident;
        let vis = &ast.vis;
        let variants = ast.variants.iter().cloned().map(|s| s.ident);
        quote! {
            impl ::get_kinds::GetKind for #ident {
                #vis fn kind<'a>(&self) -> &'a str {
                    match self {
                        #(Self::#variants => stringify!(#ident::#variants)),
                        *
                    }
                }
            }
        }
        .into()
    } else {
        panic!("Macro only accepts enums")
    }
}
