#![allow(dead_code)]

enum Number {
    Zero, // 默认从 0 开始
    One,
    Two,
}

enum Color {
    Red = 0xff0000, // 显示指定值
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // 转类型
    println!("zero is {}", Number::Zero as i32);

    println!("roses are #{:06x}", Color::Red as i32);
}
