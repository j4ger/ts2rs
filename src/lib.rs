extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{error::Error, iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "ts_interface.pest"]
struct TsInterfaceParser;

#[derive(Debug)]
pub enum TsType {
    Number,
    String,
    Boolean,
    Array(Box<TsType>),
}

#[derive(Debug)]
pub struct TsAttribute {
    pub name: String,
    pub ts_type: TsType,
}

#[derive(Debug)]
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
