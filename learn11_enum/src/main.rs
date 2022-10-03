//1.类似于C语言的方式定义
enum IpAddrKindForC {
    V4,
    V6,
}

struct IpAddrForC {
    kind: IpAddrKindForC,
    address: String,
}

//2.rust语言提倡的方式定义
enum IpAddrForRust {
    V4(String),
    V6(String),
}

//3.可以是不同类型
enum IpAddrKindForMore {
    V4(u8, u8, u8, u8),
    V6(String),
}
//4.经典用法
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    Change(i32,i32,i32),
}
//5.枚举类型的方法以及match

fn main() {
    let i1 = IpAddrForC {
        kind: IpAddrKindForC::V4,
        address: String::from("127.0.0.1"),
    };
    let i2 = IpAddrForC {
        kind: IpAddrKindForC::V6,
        address: String::from("::1"),
    };

    let i3 = IpAddrForRust::V4(String::from("127.0.0.1"));
    let i4 = IpAddrForRust::V6(String::from("::1"));
}
