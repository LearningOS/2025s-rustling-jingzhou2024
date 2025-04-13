/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/


#[derive(Debug)]
struct Stack<T> {
    size: usize,    // 栈中元素数量
    data: Vec<T>,   // 存储栈元素的向量
}

impl<T> Stack<T> {
    // 创建一个空栈
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    // 判断栈是否为空
    fn is_empty(&self) -> bool {
        0 == self.size
    }

    // 返回栈中元素数量
    fn len(&self) -> usize {
        self.size
    }

    // 清空栈
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    // 向栈顶压入一个元素
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    // 弹出栈顶元素，并更新栈大小
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None; // 栈为空，返回 None
        }
        self.size -= 1; // 减少栈大小
        self.data.pop() // 弹出向量最后一个元素
    }

    // 查看栈顶元素（不可变引用）
    fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            return None;
        }
        self.data.get(self.size - 1)
    }

    // 查看栈顶元素（可变引用）
    fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }

    // 转换为拥有所有权的迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // 返回不可变迭代器
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter {
            stack: Vec::new(),
        };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    // 返回可变迭代器
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut {
            stack: Vec::new(),
        };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

// 拥有所有权的迭代器
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

// 不可变迭代器
struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

// 可变迭代器
struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

// 判断括号序列是否有效
fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new(); // 创建一个字符栈

    // 遍历字符串中的每个字符
    for c in bracket.chars() {
        match c {
            // 遇到开括号，压入栈中
            '(' | '[' | '{' => stack.push(c),
            // 遇到闭合括号，检查栈顶是否匹配
            ')' => {
                // 如果栈为空或栈顶不是 '('，返回 false
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                // 如果栈为空或栈顶不是 '['，返回 false
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                // 如果栈为空或栈顶不是 '{'，返回 false
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            // 非括号字符（如数字、字母）直接忽略
            _ => continue,
        }
    }

    // 栈为空表示所有括号都匹配，返回 true；否则返回 false
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true); // 有效括号序列，包含非括号字符
    }

    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false); // 未闭合的括号
    }

    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true); // 有效嵌套括号
    }

    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false); // 嵌套错误
    }

    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false); // 多余的闭合括号
    }

    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true); // 空字符串
    }
}