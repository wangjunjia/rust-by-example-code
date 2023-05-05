/**
 * 结构体 分三种
 * 1. 元组结构体，就是指定名字的元组
 * 2. C 语言风格的结构体, 有字段
 * 3. 单元结构体， 不带字段， 通常用于范型中
 *
 */

// C 风格
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元
struct Unit;

// 元组结构体
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

// 结构体中使用另一个结构体
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 复用之前的代码， 但是和 js 的 assign 不一样， 这个是复用未标明的字段
    // 所以 y 是一样的
    let bottom_right = Point { x: 5.2, ..point };

    // 解构重命名
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    // 和 js 一样， 同名直接使用
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    let _unit = Unit;

    let _pair = Pair(1, 0.1);

    // 可以看出解构需要类型的
    let Pair(integer, decimal) = _pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
