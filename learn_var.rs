const MAX_POINT:u32=100000;

fn main(){
    //1.变量定义   格式：let name: type 定义变量用let,如果没有用mut修饰，则变量为不可变变量
    //定义不可变的变量
    let a=1;
    println!("a={}",a);
    //定义可变的变量
    let mut b:u32=1;
    println!("b={}",b);
    b=2;
    println!("b={}",b);
    //2.隐藏  上面已经定义了，下面再次定义，会把上面的变量覆盖掉
    let b:f32=1.1;
    println!("b={}",b);

    //3.常量
    println!("MAX_POINT={}",MAX_POINT);
}