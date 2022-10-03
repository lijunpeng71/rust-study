use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 20);

    let keys = vec![String::from("blue"), String::from("red")];
    let values = vec![10, 20];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    let k = String::from("blue");
    if let Some(v) = scores.get(&k) {
        println!("v={}", v);
    }
}
