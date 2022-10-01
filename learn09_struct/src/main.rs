fn main() {
    //1.定义结构体
    struct User{
        name: String,
        count: String,
        nonce: u64,
        active: bool,
    }
    //2.创建结构体实例
    let xiaoming=User{
        name:String::from("小明"),
        count:String::from("80001000"),
        nonce: 1000,
        active: true,
    };
    //3.修改结构体
    let mut xiaohuang=User{
        name:String::from("小明"),
        count:String::from("80001000"),
        nonce: 1000,
        active: true,
    };
    xiaohuang.nonce=20000;
    //4.参数名字和字段名字同名的简写方法
    let name=String::from("xiaoming");
    let count=String::from("89077777");
    let nonce=200000;
    let active= false;
    let user1=User{
        name,
        count,
        nonce,
        active
    };
    //5.从其他结构体创建实例
    let user2=User{
        ..user1
    };
    println!("user2={}", user2.active);
    println!("Hello, world!");
}
