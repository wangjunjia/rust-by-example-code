fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 解构赋值
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 多类型
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // 索引
    println!("long tuple first value: {}", long_tuple.0);

    // 嵌套
    let tuple_in_tuple = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    println!("tuple in tuple : {:?}", tuple_in_tuple);

    // 太长不能打印, 最后发现是可以打印的， 教程可能有误
    println!("too long tuple : {:?}", long_tuple);

    let pair = (1, true);
    println!(
        "the pair is {:?}, and reversed pair is {:?}",
        pair,
        reverse(pair)
    );

    // 单元组需要逗号
    println!("single tuple with comma {:?}", (1,));

    // 元组解构体
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!(" matrix : {:?}", matrix);
}
