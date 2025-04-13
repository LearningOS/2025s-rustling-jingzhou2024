// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.



// Cow（Clone-On-Write，写时克隆）类型。Cow 是一种智能指针，
// 它可以在写操作或需要所有权时懒惰地克隆数据，同时提供对借用数据的不可变访问。
// Cow 通过 Borrow trait 来处理一般的借用数据。
// Cow::Borrowed(&T)：表示数据是借用的（引用），不需要克隆，保持高效。
// Cow::Owned(T)：表示数据是拥有的（通常是一个 Vec 或其他拥有所有权的类型），可能是通过克隆或直接传入拥有的数据创建的。
// Cow 的典型用途是当你希望尽量避免数据拷贝，但在某些情况下（如需要修改）又不得不创建数据的副本时。Cow 会在调用 to_mut() 时根据需要克隆数据。


use std::borrow::Cow;
// abs_all 函数接收一个 Cow<'b, [i32]> 的可变引用，并将其中所有负数变成正数
fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"), 
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value")
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value")
        }
    }
}
