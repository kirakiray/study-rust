fn main() {
    let x: i32 = 5;
    let y = 100;
    // let mut y: i32 = 100; // mut 设置为可变变量
    // y = 50;
    println!("Hello, world! {}", x + y);
    {
        // 命名空间内，事后会销毁
        let x = 100;
        println!("Hello, world2! {}", x + y);
    }
}
