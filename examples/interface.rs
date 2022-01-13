use ts2rs::raw_import;

raw_import! {
    export interface Dish {
            name: string;
            price: number;
            ingredients: string[];
        }
    interface Drink {
            name: string;
            price: number;
            ingredients?: string[];
        }
}

fn main() {
    let dish = Dish {
        name: "Spaghetti".to_string(),
        price: 12.0,
        ingredients: vec!["noodles".to_string(), "tomato".to_string()],
    };
    let drink = Drink {
        name: "Coca-cola".to_string(),
        price: 2.0,
        ingredients: vec!["water".to_string(), "sugar".to_string()],
    };
    println!("{}", dish.name);
    println!("{}", drink.name);
}
