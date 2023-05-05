use std::mem;

fn analyze_slice(slice: &[i32]) {
    // 索引取值
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 定义数组
    // 可以推导类型, 类型标记可以省略
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 初始化相同
    let ys: [i32; 500] = [0; 500];

    println!("second element of the array: {}", xs[1]);

    print!("array size: {}", ys.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动被借用为 slice
    analyze_slice(&xs);

    // 可以指定一部分
    analyze_slice(&ys[1..4]);
}
