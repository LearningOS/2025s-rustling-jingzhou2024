// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 当结构体包含引用时，必须为引用指定生命周期。

// 每个输入引用都有一个隐式的生命周期。

// 如果只有一个输入引用，返回值的生命周期与该输入引用相同。

// 如果有多个输入引用，返回值的生命周期需要显式声明。

fn main() {
    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}
