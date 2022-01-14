use ts2rs::import;
import!("examples/car.ts");

fn main() {
    let transportaion = Car {
        brand: "Tesla".to_string(),
        model: "Model S".to_string(),
        year: 2016f64,
    };
    println!(
        "I wouldn't trust the brakes on the {} {} {}.",
        transportaion.year, transportaion.brand, transportaion.model
    );
}
