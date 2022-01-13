#[derive(Parser)]
#[grammar = "ts_interface.pest"]
pub struct TsInterfaceParser;

use pest::{error::Error, iterators::Pair, Parser};
use proc_macro2::TokenStream;

#[derive(Debug, PartialEq, Eq)]
pub enum TsType {
    Number,
    String,
    Boolean,
    Array(Box<TsType>),
}

impl TsType {
    fn to_rust_type(&self) -> String {
        match self {
            TsType::Number => "f64".to_string(),
            TsType::String => "String".to_string(),
            TsType::Boolean => "bool".to_string(),
            TsType::Array(inner) => format!("Vec<{}>", inner.to_rust_type()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TsAttribute {
    pub name: String,
    pub ts_type: TsType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TsInterface {
    pub name: String,
    pub attributes: Vec<TsAttribute>,
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

pub fn parse_type(pair: Pair<Rule>) -> TsType {
    match pair.as_rule() {
        Rule::number => TsType::Number,
        Rule::string => TsType::String,
        Rule::boolean => TsType::Boolean,
        Rule::array => TsType::Array(Box::new(parse_type(pair.into_inner().next().unwrap()))),
        _ => panic!("Unknown type"),
    }
}

pub fn derive_struct_def(interface: TsInterface) -> TokenStream {
    let attributes: String = interface
        .attributes
        .into_iter()
        .map(|attribute| {
            let attribute_name = attribute.name;
            let attribute_type = attribute.ts_type.to_rust_type();
            format!("{}: {},", attribute_name, attribute_type)
        })
        .collect();
    let struct_name = interface.name;
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
    use crate::interface::{TsAttribute, TsInterface, TsType};

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
                    ts_type: TsType::String,
                },
                TsAttribute {
                    name: "age".to_string(),
                    ts_type: TsType::Number,
                },
                TsAttribute {
                    name: "target".to_string(),
                    ts_type: TsType::Boolean,
                },
                TsAttribute {
                    name: "friends".to_string(),
                    ts_type: TsType::Array(Box::new(TsType::String)),
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
        // assert!(false);
    }
}
