extern crate pest;
#[macro_use]
extern crate pest_derive;

mod interface;

use std::{fs::File, io::prelude::*, io::BufReader, path::Path};

use crate::interface::*;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};

fn parse_input(input: &str) -> TokenStream {
    match parse_interface(input) {
        Ok(interfaces) => {
            let mut output = TokenStream::new();
            for interface in interfaces {
                output.extend(derive_struct_def(interface));
            }
            output.into()
        }
        Err(_) => {
            abort!(proc_macro2::Span::call_site(), "Failed to parse interface")
        }
    }
}

#[proc_macro]
pub fn raw_import(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = item.to_string();
    parse_input(&input).into()
}

#[proc_macro_error]
#[proc_macro]
pub fn import(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let span = proc_macro2::Span::call_site();
    let raw_input = item.to_string().trim_matches('"').to_string();
    match raw_input.len() {
        0 => {
            abort!(span, "No source file provided")
        }
        _ => {
            let path = Path::new(&raw_input);
            let root = env!("CARGO_MANIFEST_DIR");
            let full_path = Path::new(&root).join(&path);
            match File::open(&full_path) {
                Ok(file) => {
                    let mut contents = String::new();
                    let mut buffer = BufReader::new(file);
                    buffer.read_to_string(&mut contents).unwrap();
                    parse_input(&contents).into()
                }
                Err(_) => {
                    abort!(span, "Failed to read {}", full_path.display());
                }
            }
        }
    }
}
