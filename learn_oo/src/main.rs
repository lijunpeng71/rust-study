#[derive(Debug)]
struct Dog {
    name: String,
}

impl Dog {
    fn print_name(&self) {
        println!("name={}", self.name);
    }
}

fn main() {
    let d = Dog {
        name: String::from("wangcai"),
    };
    println!("d={:#?}", d);
}
