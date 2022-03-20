extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Lifetime;

#[proc_macro_derive(CompareByAddress)]
pub fn compare_by_address_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    let lifetimes = &ast
        .generics
        .lifetimes()
        .into_iter()
        .map(|x| &x.lifetime)
        .collect::<Vec<&Lifetime>>();
    let expanded = quote! {
        impl<#(#lifetimes),*> PartialEq for #name<#(#lifetimes),*> {
            fn eq(&self, other: &Self) -> bool {
                by_address::ByAddress(self) == by_address::ByAddress(other)
            }
        }

        impl<#(#lifetimes),*> Eq for #name<#(#lifetimes),*> {}

        impl<#(#lifetimes),*> std::hash::Hash for #name<#(#lifetimes),*> {
            fn hash<H>(&self, state: &mut H)
                where H: std::hash::Hasher {
                    by_address::ByAddress(self).hash(state)
                }

            fn hash_slice<H>(data: &[Self], state: &mut H)
                where H: std::hash::Hasher {
                    by_address::ByAddress(data).hash(state)
                }

        }
    };

    TokenStream::from(expanded)
}
