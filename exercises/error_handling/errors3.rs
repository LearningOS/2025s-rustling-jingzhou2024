// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    match total_cost(pretend_user_input) {
        Ok(cost) => {
            if cost > tokens {
                println!("You can't afford that many!");
            } else {
                tokens -= cost;
                println!("You now have {} tokens.", tokens);
            }
        }
        Err(_) => println!("Error: Please enter a valid number!"),
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    // 在 main 函数中，let cost = total_cost(pretend_user_input)?; 也使用了 ? 运算符。这意味着如果 total_cost 返回 Err，main 函数会尝试将错误向上传播。

    // 在 total_cost 内部，item_quantity.parse::<i32>()? 使用了 ? 运算符，如果解析失败，会返回 Err。
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
