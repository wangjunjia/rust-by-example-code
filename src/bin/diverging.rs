/**
 * 发散函数: 绝不返回，使用 ! 标记, 是空类型， 不是 () 单元元组
 * 类似空集，不是0
 *
 */

#[feature(never_type)]
fn main() {
    let x: ! = panic!("this call never returns");
    // println!("you will never see this line!");
}
