// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a

// 创建一个 HashMap，键是水果名称（String 类型），值是水果数量（u32 类型）。

// 篮子里至少有 3 种不同的水果。

// 所有水果的总数至少为 5 个。

// 已经预先放了 2 个香蕉（banana），我们需要添加更多的水果。


use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // TODO: declare your hash map here.

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mango"), 1);

    // TODO: Put more fruits in your basket here.

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
