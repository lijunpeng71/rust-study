/**
 * 1. 关联类型在trait定义中指定占位符类型
 * 关联类型是一个将类型占位符与trait想关联的方式
 * trait的实现者会针对特定的实现在这个类型的位置指定相应的具体类型
 * 如此可以定义一个使用多种类型的trait
 */

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    println!("Hello, world!");
}
