// 1. trait 用于定义与其他类型共享的功能，类似于其他语言中的接口
// (1) 可以通过trait以抽象的方式定义共享的行为
//  (2) 可以使用trait bounds指定泛型是任何拥有特定行为的类型
// 2. 定义trait
pub trait GetInfomation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

pub trait SchoolName {
    fn get_school_name(&self) -> String {
        String::from("HongXingSchool")
    }
}
// 3. 实现trait
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetInfomation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}
//默认实现
impl SchoolName for Student {}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}
//默认实现
impl SchoolName for Teacher {
    fn get_school_name(&self) -> String {
        String::from("GongmingSchool")
    }
}

impl GetInfomation for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}
// 4. 默认实现：可以在定义trait的时候提供默认的行为，trait的类型可以使用默认的行为。
// 5. trait作为参数
fn print_information(item: impl GetInfomation) {
    println!("name={}", item.get_name());
    println!("age={}", item.get_age());
}

fn main() {
    let s = Student {
        name: "xiaoming".to_string(),
        age: 10,
    };
    let s_school_name = s.get_school_name();
    println!("------------------------------------");
    println!("student,name={},age={}", s.get_name(), s.get_age());
    println!("s_school_name={}", s_school_name);
    let t = Teacher {
        name: "xiaohuang".to_string(),
        age: 30,
        subject: "语文".to_string(),
    };
    let t_school_name = t.get_school_name();
    println!("------------------------------------");
    println!(
        "teacher,name={},age={},subject={}",
        t.get_name(),
        t.get_age(),
        t.subject
    );
    println!("t_school_name={}", t_school_name);
    println!("------------------------------------");
    print_information(s);
    print_information(t);
}
