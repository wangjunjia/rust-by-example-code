enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 部分导入
    use Status::{Poor, Rich};
    // 全部导入
    use Work::*;

    // Status::Poor
    let status = Poor;
    // Work::Civilian
    let work = Civilian;

    match status {
        // 这里也不用全路径了
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilian work~"),
        Soldier => println!("Soldier work~"),
    }
}
