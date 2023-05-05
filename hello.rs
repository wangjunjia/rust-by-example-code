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
                        // #[derive(Debug)] // {:?}
                        // #[derive(Display)] // {}
    struct Structure(i32);

    // 这个会报错
    // println!("This struct {} won't print..", Structure(3));
}
