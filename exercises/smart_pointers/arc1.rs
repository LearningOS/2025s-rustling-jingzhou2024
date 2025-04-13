// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.


#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;


// Arc
// 允许多个所有者（通过引用计数）。
// 线程安全（内部使用原子操作）。
// 不可变引用（Arc 本身不提供写访问，除非结合 Mutex 或其他同步原语）。

// Send：表示一个类型可以安全地移动到另一个线程。
// Sync：表示一个类型可以安全地被多个线程同时引用（共享）。

// Vec<u32> 是 Send，所以可以安全地移动到另一个线程。但它 不是 Sync，意味着不能安全地被多个线程同时引用。这是因为：
// 如果多个线程同时访问 Vec，可能导致数据竞争（data race），即使是只读操作，Rust 编译器也无法保证底层内存访问的安全性。
// Rust 的所有权规则要求数据在线程间共享时必须明确线程安全机制。

// 在代码中，我们需要 8 个线程同时访问 numbers（Vec<u32>）。如果直接将 numbers 传给线程，Rust 会报错，因为：
// 直接借用 numbers（如 &numbers）会导致借用检查失败，多个线程不能安全共享同一个引用。
// 移动 numbers 到一个线程会使其在其他线程不可用，违背了“同时访问”的需求。
// 复制 numbers（如 numbers.clone()）会创建副本，违反题目“尽量不复制”的要求。



fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    // Arc<Vec<u32>>
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        // 由于 Arc 支持多所有权，我们可以通过克隆 Arc 来为每个线程创建一个新的引用
        // Arc::clone 增加引用计数，创建一个新的 Arc<Vec<u32>>，指向相同的底层数据。
        // 这不会复制 Vec<u32> 的内容，只是复制了 Arc 智能指针本身。
        // child_numbers 的类型也是 Arc<Vec<u32>>。

        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
