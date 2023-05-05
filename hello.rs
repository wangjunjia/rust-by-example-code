// 导入包, 第三方库， 标准库之类的
use std::fmt;

// 单行注释
fn main() {
    /**
     *  块注释，/* 可以嵌套 */
     */
    let x = 5 + /* 90 + */ 5;
    // {}, {x} , 都可以
    println!("Is `x` 10 or 100 ? x = {x}");
    println!("Hello World~");

    // {} 会转成任意内容
    // 31 默认 i32, 可以指定 类型 31i64 (i64类型)
    println!("{} days~", 31);

    // 可以使用位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数
    println!(
        "{subject} {verb} {object} ",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 可以指定宽度右对齐文本
    // "     1", 输出五个空格跟一个1
    println!("{number:>width$}", number = 1, width = 6);

    // 左补0， "000001"
    println!("{number:>0width$}", number = 1, width = 6);

    // 创建一个 i32 的结构体
    #[allow(dead_code)] // 未使用不要警告了
    #[derive(Debug)] // {:?}
                     // #[derive(Display)] // display 需要自己实现，不是宏，因为每个结构不一样{}
    struct Structure(i32);

    // 这个会报错
    println!("This struct {:?} won't print..", Structure(3));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let peter = Person {
        name: "Peter",
        age: 27,
    };
    // {:#?} 美化打印
    println!("peter is {:#?}", peter);

    let minmax = MinMax(0, 14);
    println!("Display formatter minmax : {}", minmax);
    println!("Debug formatter minmax : {:?}", minmax);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display formatter point : {}", point);
    println!("Debug formatter point : {:?}", point);

    // Display formatter minmax : (0, 14)
    // Debug formatter minmax : MinMax(0, 14)
    // Display formatter point : x: 3.3, y: 7.2
    // Debug formatter point : Point2D { x: 3.3, y: 7.2 }
}

// 定义一个结构体， 保护一个元组 , 元素类型 i32
struct Counter(i32);

// 实现 {} Display
impl fmt::Display for Counter {
    // 实现方法
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 输出第一个元素
        write!(f, "{}", self.0)
    }
}

// debug 默认输出 vs display
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 具名的结构体
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 看得出来， 就是指定顺序 write 出去
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
