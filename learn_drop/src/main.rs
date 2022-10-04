//1. Drop trait 类似与其他语言中的析构函数，当值离开作用域的时候执行此函数的代码。
#[derive(Debug)]
struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Dog {} leave", &(self.name));
    }
}
fn main() {
    let a = Dog {
        name: String::from("wangcai"),
    };
    println!("a={:#?}", a);
    println!("hello,world");
}
