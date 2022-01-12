use ts2rs::parse_interface;

fn main() {
    let ts_interface = r#"
    export interface Dish {
            name: string;
            price: number;
            ingredients: string[];
        }
    "#;
    let parse_result = parse_interface(ts_interface);
    println!("{:#?}", parse_result);
}
