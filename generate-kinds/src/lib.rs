use proc_macro::TokenStream;
use quote::quote;
use syn::Item;
use get_kinds::Kind;

/// Does not take any arguments
/// Valid forms are `#[kinds]`
/// Apply on an enum
/// ### Examples
/// ```
/// use generate_kinds::kinds;
/// 
/// #[kinds]
/// enum Test {
///     T1,
///     T2
/// }
/// assert_eq!(Test::T1.kind(), "Test :: T1");
/// ```
#[proc_macro_derive(Kind)]
pub fn kinds(stream: TokenStream) -> TokenStream {
    let ast: Item = syn::parse(stream).expect("code is invalid");
    if let Item::Enum(ast) = ast {
        let ident = &ast.ident;
        let vis = &ast.vis;
        let variants = ast.variants.iter().cloned().map(|s| s.ident);
        quote! {
            impl ::get_kinds::Kind for #ident {
                #vis fn kind<'a>(&self) -> &'a str {
                    match self {
                        #(Self::#variants => stringify!(#ident::#variants)),
                        *
                    }
                }
            }
        }.into()
    } else {
        panic!("Macro only accepts enums")
    }
}