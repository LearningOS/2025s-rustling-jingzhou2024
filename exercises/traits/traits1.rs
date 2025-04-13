// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.


trait AppendBar {
    // 这里的 self 不指定可变性（mutability）。
    // 在 Rust 中，self 的可变性（如 mut self 或 &mut self）是由实现者决定的，trait 定义只关心所有权语义（值、引用或可变引用）。
    fn append_bar(self) -> Self;
    // 接收 self 的所有权（而不是引用）。

    // 返回 Self 类型（这里是 String）。

    // Self（大写 S）是一个类型别名，表示实现当前 trait 的具体类型，
    // 或者在 impl 块中表示当前类型的本身。
}

// 在 trait 定义中，Self 指代“任何实现这个 trait 的类型”。

// 在 impl 块中，Self 指代 impl 后面的具体类型。



impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        return self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
