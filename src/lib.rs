//! [![github]](https://github.com/j4ger/ts2rs)&ensp;[![crates-io]](https://crates.io/crates/ts2rs)&ensp;[![docs-rs]](https://docs.rs/ts2rs)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! This crate provides the [`import!`] macro for importing Typescript interfaces into Rust.
//!
//! # Type Mappings
//! For now, only primitive types are supported,
//! with the following mapping:
//!
//! | Typescript Type | Rust Type |
//! |-----------------|-----------|
//! | `string`        | `String`  |
//! | `number`        | `f64`     |
//! | `boolean`       | `bool`    |
//! | `T[]`           | `Vec<T>`  |
//! | `T?`            | `Option<T>`|
//! | any user-defined type/interface | a struct definition(first letter capitalized) |
//!
//! # Usage
//! See the [`import!`] macro.
//!
//! # Ignored Sections
//!
//! Comments are ignored by default.
//! A comment-based mapping attribution will be added in the future.
//!
//! Only interface definitions will be extracted, so any expressions and other sorts of code blocks will be ignored.
//!
//! Interfaces with generics are not supported yet and will be ignored.
//!
//!
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

/// Imports Typescript interfaces from raw text.
/// # Examples
/// ```
/// # use ts2rs::raw_import;
/// raw_import!{
///     interface File {
///         name: string;
///         size: number;
///         isDirectory: boolean;
///         children: File[];
///     }
///}
///```
///
/// This will result in the following struct definition:
/// ```
/// pub struct File {
///     pub name: String,
///     pub size: f64,
///     pub isDirectory: bool,
///     pub children: Vec<File>,
///}
///
#[proc_macro_error]
#[proc_macro]
pub fn raw_import(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = item.to_string();
    parse_input(&input).into()
}

/// Imports Typescript interfaces from a file.
/// # Examples
/// ```
/// # use ts2rs::import;
/// import!("path/to/file.ts");
/// ```
///
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
