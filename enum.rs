#![allow(dead_code)]

enum WebEvent {
    // 单结构体， unit-like
    PageLoad,
    PageUnload,
    // 元组结构体
    KeyPress(char),
    Paste(String),
    // 普通结构体
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 解构出 c
        WebEvent::KeyPress(c) => println!("pressed {}. ", c),
        WebEvent::Paste(s) => println!("pasted \n{}\n", s),
        // 解构 x, y
        WebEvent::Click { x, y } => println!("clicked at x = {}, y = {}. ", x, y),
    }
}

// 类型别名
enum VeryVerboseEnumOfThingsToDoWidthNumbers {
    Add,
    Subtract,
}

impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// 创建类型别名
type Operations = VeryVerboseEnumOfThingsToDoWidthNumbers;

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("My Text".to_owned());
    let clicked = WebEvent::Click { x: 20, y: 80 };
    let loaded = WebEvent::PageLoad;
    let unloaded = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(clicked);
    inspect(loaded);
    inspect(unloaded);

    let x = Operations::Add;
    println!("1 + 2 is {}", x.run(1, 2));
}
