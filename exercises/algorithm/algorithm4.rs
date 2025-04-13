/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;

// 定义树节点结构体，包含值和左右子节点的引用
#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord, // 要求类型 T 必须实现 Ord trait，以便比较大小
{
    value: T,                        // 节点的值
    left: Option<Box<TreeNode<T>>>,  // 左子树，可能为空
    right: Option<Box<TreeNode<T>>>, // 右子树，可能为空
}

// 定义二分搜索树结构体，包含根节点
#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>, // 树的根节点，可能为空
}

// 为 TreeNode 实现方法
impl<T> TreeNode<T>
where
    T: Ord,
{
    // 创建一个新节点
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 在当前节点为根的子树中插入新值
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 如果新值小于当前节点值，插入到左子树
                if let Some(ref mut left) = self.left {
                    // 如果左子树存在，继续递归插入
                    left.insert(value);
                } else {
                    // 如果左子树为空，创建新节点
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                // 如果新值大于当前节点 value，插入到右子树
                if let Some(ref mut right) = self.right {
                    // 如果右子树存在，继续递归插入
                    right.insert(value);
                } else {
                    // 如果右子树为空，创建新节点
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {
                // 如果值相等，不插入（忽略重复值）
                // 也可以根据需求选择覆盖或抛出错误
            }
        }
    }
}

// 为 BinarySearchTree 实现方法
impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    // 创建一个空的二分搜索树
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 向树中插入新值
    fn insert(&mut self, value: T) {
        if let Some(ref mut root) = self.root {
            // 如果根节点存在，调用节点的插入方法
            root.insert(value);
        } else {
            // 如果树为空，创建新节点作为根节点
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // 在树中搜索某个值，返回是否存在
    fn search(&self, value: T) -> bool {
        let mut current = &self.root; // 从根节点开始
        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Equal => return true, // 找到值，返回 true
                Ordering::Less => current = &node.left, // 值较小，搜索左子树
                Ordering::Greater => current = &node.right, // 值较大，搜索右子树
            }
        }
        false // 未找到值，返回 false
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 测试空树搜索
        assert_eq!(bst.search(1), false);

        // 插入多个值
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // 测试插入的值是否存在
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        // 测试不存在的值
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // 插入重复值
        bst.insert(1);
        bst.insert(1);

        // 确认值存在
        assert_eq!(bst.search(1), true);

        // 检查树结构，确认没有不必要的子节点
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("根节点不应为空"),
        }
    }
}