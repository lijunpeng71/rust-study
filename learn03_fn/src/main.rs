
fn other_fun(){
    println!("This is a function");
}

fn other_fun1(a:i32,b:u32){
    println!("a={},b={}",a,b);
}   

fn other_fun2(a:i32,b:i32) -> i32 {
    let result=a+b;
    return result;
}

fn other_fun3(a:i32,b:i32) -> i32 {
    a+b
}

fn main() {
    other_fun();
    other_fun1(18, 66);
    let result2=other_fun2(18,66);
    println!("result2={}",result2);
    println!("Hello, world!");
}
