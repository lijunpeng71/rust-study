fn main() {
    //bool
    let is_true:bool=true;
    println!("is_true={}",is_true);
    let is_false:bool=false;
    println!("is_false={}",is_false);
    //char 在rust里面，char是32位的
    let a='a';
    println!("a={}",a);
    let b='你';
    println!("b={}",b);
    //i8,i16,i32,i64,u8,u16,u32,u64,f32,f64
    let c:i8=-111;
    println!("c={}",c);

    let d:f32=0.0009;
    println!("d={}", d);

    //自适应类型 isize,usize
    println!("max={}", usize::max_value());

    //数组 [Type: size]
    let arr: [u32;5]=[1,2,3,4,5];
    show(arr);
}


fn show(arr:[u32;5]){
    println!("--------------------------");
    for i in &arr{
        println!("i={}",i);
    }
}
