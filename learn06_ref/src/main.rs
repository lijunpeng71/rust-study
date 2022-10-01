// fn main() {
//     let s1=gives_ownership();
//     let s2=String::from("hello");
//     let s3=takes_and_gives_back(s2);
//     println!("hello,world!");
// }

// fn gives_ownership() -> String {
//     let s=String::from("hello");
//     s
// }

// fn takes_and_gives_back(s:String) -> String{
//     s
// }


// 引用： 用法&，让我们创建一个只想值的应用，但是并不拥有它，因为不拥有这个值，所以当引用离开其值指向的作用域后也不会被丢弃
fn calcute_length(s:&String) -> usize{
    s.len()
}

fn modify_s(s:&mut String){
    s.push_str(",world")
}
fn main(){
     let mut s1=String::from("hello");
     let s=&s1;
     let len=calcute_length(&s1);
     let ms=&mut s1;
     modify_s(ms);
     println!("len={}", len);
     println!("s1={}", s1);
}
