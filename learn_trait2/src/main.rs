//1. trait_bound 语法  fn print_information<T: GetInfomation>(item: T) {}
//2. 指定多个trait bound
//3. 返回 trait的类型
trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

//使用trait_bound写法1
// fn print_information<T: GetName + GetAge>(item: T) {
//     println!("name={}", item.get_name());
//     println!("age={}", item.get_age());
// }

//使用trait_bound写法2
fn print_information<T>(item: T)
where
    T: GetName + GetAge,
{
    println!("name={}", item.get_name());
    println!("age={}", item.get_age());
}

#[derive(Debug)]
pub struct Student {
    name: String,
    age: u32,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

//trait作为返回值
fn produce_item_with_age() -> impl GetAge {
    Student {
        name: String::from("xiaoming"),
        age: 15,
    }
}

fn main() {
    let s = Student {
        name: "xiaoming".to_string(),
        age: 10,
    };
    print_information(s);
    println!("-----------------------------");
    let s = produce_item_with_age();
    println!("{:#?}", s.get_age());
}
