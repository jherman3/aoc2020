extern crate regex;

use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::parse_macro_input;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn regex_parsed(attr: TokenStream, item: TokenStream) -> TokenStream {
    use regex::Regex;
    let input = parse_macro_input!(item as ItemStruct);
    let attr = parse_macro_input!(attr as syn::LitStr);
    // Check regex is good
    let _ = Regex::new(&attr.value()).expect("Invalid regex");

    let mut parsers = Vec::new();
    let mut fields = Vec::new();
    for (i, f) in input.fields.iter().enumerate() {
        let v = format_ident!("__field_{}", i);
        let ty = &f.ty;
        let id = &f.ident;
        parsers.push(quote! {
            let #v: #ty = caps.get(#i+1).ok_or("Missing capture")?.as_str().parse()?;
        });
        fields.push(quote! {
            #id: #v,
        });
    }

    let id = &input.ident;
    let reg = format_ident!("__REGEX_{}", id.to_string().to_uppercase());
    let fs = quote! {
        lazy_static! {
            static ref #reg: ::regex::Regex = ::regex::Regex::new(#attr).unwrap();
        }
        #input
        impl ::std::str::FromStr for #id {
            type Err = Box<dyn ::std::error::Error>;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let caps = #reg.captures(s).ok_or("No match")?;
                #(#parsers)*
                Ok(Self {
                    #(#fields)*
                })
            }
        }
    };
    let ts = TokenStream::from(fs);
    // println!("{}", ts.to_string());
    ts
}
