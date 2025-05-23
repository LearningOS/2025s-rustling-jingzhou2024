// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// TODO: Complete this use statement
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
fn main() {
    // SystemTime 是一个表示时间点的类型，通常用来获取系统当前时间或进行时间相关的操作。

    // UNIX_EPOCH 是一个 SystemTime
    // 表示 Unix 纪元时间：1970年1月1日 00:00:00 UTC。

    // 计算从 Unix 纪元到当前时间的秒数或毫秒数（时间戳）

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
