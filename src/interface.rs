#i[derive(Parser)]
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
    pub derives: Vec<String>,
    pub skip_serde: bool,
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

pub fn parse_interface(input: &str) -> Result<Vec<TsInterface>, Error<Rule>> {
    let mut output: Vec<TsInterface> = Vec::new();
    let mut interfaces = TsInterfaceParser::parse(Rule::declaration_file, input)?;
    let interfaces = interfaces.next().unwrap().into_inner();
    let interfaces = interfaces
        .filter(|pair| pair.as_rule() == Rule::interface)
        .into_iter();
    'interface_loop: for interface in interfaces {
        let mut interface_inner = interface.into_inner();
        let mut attributes: Vec<TsAttribute> = Vec::new();
        let mut interface_name = interface_inner.next().unwrap().as_str().to_string();
        let interface_inner = interface_inner.into_iter();
        let mut interface_options = None;
        'attribute_loop: for attribute in interface_inner {
            if attribute.as_rule() == Rule::attribute {
                let mut attribute_inner = attribute.into_inner();
                let mut attribute_name = attribute_inner.next().unwrap().as_str().to_string();
                let mut ts_type = parse_type(attribute_inner.next().unwrap());
                if attribute_name.ends_with("?") {
                    attribute_name = attribute_name.trim_end_matches("?").to_string();
                    ts_type = format!("Option<{}>", ts_type);
                }
                if let Some(attribute_options) = attribute_inner.next() {
                    for option in attribute_options.into_inner() {
                        match option.as_rule() {
                            Rule::rename_option => {
                                attribute_name =
                                    option.into_inner().next().unwrap().as_str().to_string();
                            }
                            Rule::retype_option => {
                                ts_type = option.into_inner().next().unwrap().as_str().to_string();
                            }
                            Rule::skip_option => {
                                continue 'attribute_loop;
                            }
                            _ => panic!("Unexpected option: {:?}", option.as_rule()),
                        }
                    }
                }
                attributes.push(TsAttribute {
                    name: attribute_name,
                    ts_type,
                });
            } else if attribute.as_rule() == Rule::option {
                interface_options = Some(attribute.into_inner());
            }
        }
        let mut skip_serde = false;
        let mut derives = vec!["Debug".to_string()];
        if let Some(options) = interface_options {
            for option in options {
                match option.as_rule() {
                    Rule::skip_option => continue 'interface_loop,
                    Rule::skip_derive_serde_option => skip_serde = true,
                    Rule::derive_option => {
                        let mut derive_inner = option.into_inner();
                        let derive_name = derive_inner.next().unwrap().as_str().to_string();
                        derives.push(derive_name);
                    }
                    Rule::rename_option => {
                        let mut rename_inner = option.into_inner();
                        interface_name = rename_inner.next().unwrap().as_str().to_string();
                    }
                    _ => panic!("Unexpected option: {:?}", option.as_rule()),
                }
            }
        }
        output.push(TsInterface {
            name: interface_name,
            attributes,
            derives,
            skip_serde,
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
    let mut derives = interface.derives;
    if cfg!(feature = "serde") && !interface.skip_serde {
        derives.push("serde::Serialize".to_string());
        derives.push("serde::Deserialize".to_string());
    }
    let derives = derives.join(", ");
    let struct_def = format!(
        r#"
            #[derive({})]
            pub struct {} {{
                {}
            }}
        "#,
        derives, struct_name, attributes
    );
    struct_def.parse().expect(
        "Failed to generate struct definition, check if the interface has an invalid name or field",
    )
}

#[cfg(test)]
mod interface_tests {
    use crate::interface::{TsAttribute, TsInterface};

    const INTERFACE_TEST_STRING: &str = r#"
        interface Person {
            name: string;
            age: number;
            target: boolean;
            weight: number?; //-skip;
            friends?: string[];
        } //-derive: PartialEq;
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
                    ts_type: "Option<Vec<String>>".to_string(),
                },
            ],
            derives: vec!["Debug".to_string(), "PartialEq".to_string()],
            skip_serde: false,
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
