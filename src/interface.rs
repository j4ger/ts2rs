#[derive(Parser)]
#[grammar = "ts_interface.pest"]
pub struct TsInterfaceParser;

use pest::{error::Error, iterators::Pair, Parser};
use proc_macro2::TokenStream;

#[derive(Debug, PartialEq, Eq)]
pub struct TsAttribute {
    pub name: String,
    pub ts_type: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TsInterface {
    pub name: String,
    pub attributes: Vec<TsAttribute>,
}

trait Capitalize {
    fn capitalize(&self) -> String;
}

impl Capitalize for String {
    fn capitalize(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

//TODO: use snafu for better error handling
pub fn parse_interface(input: &str) -> Result<Vec<TsInterface>, Error<Rule>> {
    let mut output: Vec<TsInterface> = Vec::new();
    let mut interfaces = TsInterfaceParser::parse(Rule::declaration_file, input)?;
    let interfaces = interfaces.next().unwrap().into_inner();
    let interfaces = interfaces
        .filter(|pair| pair.as_rule() == Rule::interface)
        .into_iter();
    for interface in interfaces {
        let mut interface_inner = interface.into_inner();
        let mut attributes: Vec<TsAttribute> = Vec::new();
        let interface_name = interface_inner.next().unwrap().as_str().to_string();
        for attribute in interface_inner.into_iter() {
            let mut attribute_inner = attribute.into_inner();
            let name = attribute_inner.next().unwrap().as_str().to_string();
            let ts_type = parse_type(attribute_inner.next().unwrap());
            attributes.push(TsAttribute { name, ts_type });
        }
        output.push(TsInterface {
            name: interface_name,
            attributes,
        });
    }
    Ok(output)
}

pub fn parse_type(pair: Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::number => "f64".to_string(),
        Rule::string => "String".to_string(),
        Rule::boolean => "bool".to_string(),
        Rule::array => format!("Vec<{}>", parse_type(pair.into_inner().next().unwrap())),
        Rule::identifier => pair.as_str().to_string().capitalize(),
        _ => panic!("Unknown type"),
    }
}

pub fn derive_struct_def(interface: TsInterface) -> TokenStream {
    let attributes: String = interface
        .attributes
        .into_iter()
        .map(|attribute| {
            let attribute_name = attribute.name;
            let attribute_type = attribute.ts_type;
            format!("{}: {},", attribute_name, attribute_type)
        })
        .collect();
    let struct_name = interface.name.capitalize();
    let struct_def = format!(
        r#"
        #[derive(Debug)]
        pub struct {} {{
            {}
        }}
"#,
        struct_name, attributes
    );
    struct_def.parse().unwrap()
}

#[cfg(test)]
mod interface_tests {
    use crate::interface::{TsAttribute, TsInterface};

    const INTERFACE_TEST_STRING: &str = r#"
        interface Person {
            name: string;
            age: number;
            target: boolean;
            friends: string[];
        }
    "#;

    #[test]
    fn parse_interface() {
        let ts_interface = super::parse_interface(INTERFACE_TEST_STRING);
        assert!(ts_interface.is_ok());
        let interface = TsInterface {
            name: "Person".to_string(),
            attributes: vec![
                TsAttribute {
                    name: "name".to_string(),
                    ts_type: "String".to_string(),
                },
                TsAttribute {
                    name: "age".to_string(),
                    ts_type: "f64".to_string(),
                },
                TsAttribute {
                    name: "target".to_string(),
                    ts_type: "bool".to_string(),
                },
                TsAttribute {
                    name: "friends".to_string(),
                    ts_type: "Vec<String>".to_string(),
                },
            ],
        };
        assert_eq!(ts_interface.unwrap()[0], interface);
    }

    #[test]
    fn generate_token_stream() {
        let ts_interface = super::parse_interface(INTERFACE_TEST_STRING)
            .unwrap()
            .remove(0);
        let struct_def = super::derive_struct_def(ts_interface);
        println!("{}", struct_def);
        assert!(false);
    }
}
