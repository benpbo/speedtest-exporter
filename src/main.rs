use response::Response;

mod response;

fn main() {
    let file = std::fs::read_to_string("result.json").unwrap();

    let response: Response = serde_json::from_str(&file).unwrap();

    println!("{response:?}");
}
