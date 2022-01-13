extern crate pest;
#[macro_use]
extern crate pest_derive;

mod interface;

use crate::interface::*;
use proc_macro2::TokenStream;

#[proc_macro]
pub fn raw_import(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = item.to_string();
    let input = parse_interface(&input).unwrap();
    let mut output = TokenStream::new();
    for interface in input {
        output.extend(derive_struct_def(interface));
    }
    output.into()
}
