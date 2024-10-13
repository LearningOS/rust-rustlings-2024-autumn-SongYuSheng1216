/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + std::fmt::Debug,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        println!("value : {:?}", value);
        if let None = self.root {
            self.root = Some(Box::new(TreeNode::new(value)));  // 有没有其他合适的错误处理方式？
        } else if let Some(root_node) = &mut self.root {
            println!("insert value is in");
            root_node.insert(value);
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut node_searching = self.root.as_ref();    // 有没有其他的解开方法？
        while let Some(node) = node_searching {
            if node.value == value {
                println!("11111111111");
                break;
            } else if node.value < value {  // 右节点
                println!("2111111111111111");
                node_searching = node.right.as_ref();
            } else if node.value > value {  // 左节点
                println!("32222222222");
                node_searching = node.left.as_ref();
            }
        }
        if let None = node_searching {
            return false;
        }
        true
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        // 这样吧
        // 先判断value和self目前节点的值
        // 判断左节点还是右节点是否为None
        // 然后如果不是None则进入递归
        // 传入的value不变，但是调用insert的就是非None的节点
        if self.value > value { // 放左边
            match self.left {
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(ref mut left_node) => left_node.insert(value),
                // ref和&和as_ref的区别?
                // 测试一下match和unwrap()的区别
                // Option<Box<TreeNode<T>, Global>>
                // 使用match，解开Option，获得Some
                // 然后因为insert是TreeNode的方法，&mut self
                // 所以调用insert的对象是&mut TreeNode
                // Box<TreeNode<T>, Global>转变为&mut TreeNode
                // Box如何解开？方法是？
                // 以及为什么这里这样解开
            }
        } else if self.value < value {  // 放右边
            match self.right {
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(ref mut right_node) => right_node.insert(value),
            }
        }
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    
