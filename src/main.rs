#[derive(Debug, PartialEq, Eq)] // derive auto-implements simple features, like debug printing and comparisons
struct Fruit {
    name: String,
    amount: u32,
}

fn convert_to_fruit(line: &str) -> Result<Fruit, &str> {
    Err("Function not implemented")
}

#[test]
fn test_convert_to_fruit() {
    let fruit = convert_to_fruit("banana, 21");

    let name = "banana".to_owned();
    let amount = 21;
    let expected = Ok(Fruit { name, amount });
    assert_eq!(fruit, expected);
}

fn main() {
    println!("Hello Rust lovers!");
}
