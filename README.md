# ts2rs

A proc-macro library that imports Typescript interface definitions into Rust.

## Usage

See [docs.rs](https://docs.rs/ts2rs)

## Todo list

- Better error reporting;
- Derive serde::Serialize & serde::Deserialize if 'serde' feature is enabled;
- Specify import details using comments (custom type names, skip import, etc.);
- Import from class definitions;
- Advanced types like date and time;
