/**
 * 静态变量和常量的区别：
 * 1. 静态变量有一个固定的内存地址(使用这个值总会访问相同的地址)
 *    常量则允许在任何被用到的时候复制其数据
 * 2. 静态变量可以是可变的，虽然这可能是不安全的(用unsafe包含)
 */

static HELLO_WORLD: &str = "Hello,world!";

static mut COUNTER: u32 = 0;
fn add_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("{}", HELLO_WORLD);
    add_counter(3);
    add_counter(3);
    unsafe {
        println!("counter={}", COUNTER);
    }
}
