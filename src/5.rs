use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 5);
    scores.insert(String::from("Green"), 3);
    println!("{:?}", scores);
}
