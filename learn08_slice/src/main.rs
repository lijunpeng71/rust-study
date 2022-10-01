fn main() {
    let s=String::from("hello1");
    //let h=&s[0..5];
    //let h=&s[0..=4];
    //let h=&s[..=4];
    let h=&s[..5];
    println!("h={}", h);
    println!("Hello, world!");
}
