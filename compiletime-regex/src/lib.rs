use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let lit_str = parse_macro_input!(input as LitStr);
    let pat = lit_str.value();

    if let Err(err) = Regex::new(&pat) {
        return syn::Error::new(lit_str.span(), err)
            .to_compile_error()
            .into();
    }

    quote! { ::regex::Regex::new(#pat).unwrap() }.into()
}
