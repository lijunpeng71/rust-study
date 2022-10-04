/**
 * 1. 不安全的Rust存在的量大原因：
 *  （1）静态分析本质上是保守的，就意味着某些代码可能是合法的，但是Rust也会拒绝。在此情况下，可以使用不安全的代码
 *  （2）底层计算机硬件固有的不安全性。如果Rust不允许进行不安全的操作，有些任务根本就完不成。
 * 2. 不安全的Rust具有的超级力量
 *  Rust会通过unsafe关键字切换到不安全的Rust。不安全的Rust具有以下超级力量：
 *  （1）解引用裸指针
 *  （2）调用不安全的函数或者方法
 *  （3）访问或修改可变静态变量
 *  （4）实现不安全的trait
 * 注意：unsafe并不会关闭借用刚检查器或者禁用任何其他的Rust安全检查规则，它只提供上述几个不被编译器检查内存安全的功能。
 *      unsafe也不意味着代码块中的代码一定就是不ok的，它只是表示程序员来确保安全。
 */

unsafe fn dangerous() {
    println!("do something dangerous");
}
fn main() {
    let mut num = 5;
    //创建不可变和可变的裸指针可以在安全的代码中，只是不能在不安全代码块之外解引用裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x12345usize;
    let _r = address as *const i32;
    println!("hello,world!");

    unsafe {
        dangerous();
    }
    dangerous();
}
