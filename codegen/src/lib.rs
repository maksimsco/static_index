#![feature(proc_macro_hygiene)]
extern crate proc_macro;
use codegen_const::GLOBAL_INDEX;
use quote::quote;
use std::sync::atomic::Ordering;
use syn;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::punctuated::Punctuated;

struct Input {
    idents: Punctuated<syn::Ident, syn::Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Input> {
        let result = Input {
            idents: input.parse_terminated(syn::Ident::parse)?,
        };

        Ok(result)
    }
}

fn quote_ident(ident: syn::Ident) -> proc_macro2::TokenStream {
    let index = unsafe { GLOBAL_INDEX.fetch_add(1, Ordering::Relaxed) };
    quote! {
        impl Indexable for #ident {
            const INDEX: usize = #index;
        }
    }
}

/// Static index proc macro
///
/// ## Example
/// ```
/// use static_index::{Indexable, static_index};
///
/// struct A;
/// struct B;
///
/// static_index!(A);
/// static_index!(B);
///
/// assert_eq!(0, <A as Indexable>::INDEX);
/// assert_eq!(1, <B as Indexable>::INDEX);
/// ```
#[proc_macro]
pub fn static_index(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as Input);

    let mut output = proc_macro2::TokenStream::new();
    for ident in input.idents {
        output.extend(quote_ident(ident));
    }

    proc_macro::TokenStream::from(output)
}

/// Static index proc macro derive
///
/// ## Example
/// ```
/// use static_index::{Indexable, StaticIndex};
///
/// #[derive(StaticIndex)]
/// struct A;
///
/// #[derive(StaticIndex)]
/// struct B;
///
/// assert_eq!(0, <A as Indexable>::INDEX);
/// assert_eq!(1, <B as Indexable>::INDEX);
/// ```
#[proc_macro_derive(StaticIndex)]
pub fn static_index_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

    proc_macro::TokenStream::from(quote_ident(ast.ident))
}
