// 1. 在任意给定时间，有了可变引用之后不能再有不可变引用


fn main() {
   let refs = dangle();
   println!("hello,world!");
}

fn dangle() -> &String {
    let s=String::from("hello");
    &s
}
