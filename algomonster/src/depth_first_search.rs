//! ## Binary Search Tree

//! **Binary Tree** - every node has 0 to 2 children.
//!
//! Binary Tree Types:  
//!   - Full - ever node has 0 or 2 childern
//!   - Complete - all levels completely filled except possibly last levels, and all nodes are to the left
//!   - Perfect - All internals nodes have two children and all leaf nodes have the same level.
//!     - Number of nodes in a perfect binary tree is `2^n-1` where `n` is the number of levels.
//!     - Number of internal nodes = number of leaf nodes - 1.
//!     - Total number of nodes = 2 * number of leaf nodes - 1
//!
//! **Binary Search Tree (BST)** - every nodes follows the ordering property of `all left descendents < node < all right descendents`. 
//!
//! **Balanced Binary Tree** - the height difference of the left and right subtree of the node is not more than 1 
//!
use std::slice::Iter;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            val: value,
            left: None,
            right: None,
        }
    }
    pub fn left(mut self, node: Node<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: Node<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

#[allow(dead_code)]
fn build_tree_from_str<T>(input: &str) -> Option<Box<Node<T>>> where T: FromStr {
    let tokens: Vec<&str> = input.split(' ').collect();
    let token_iterator= &mut tokens.iter();
    build_tree(token_iterator)
}

#[allow(dead_code)]
fn build_tree<'a, T>(nodes: &mut Iter<&str>) -> Option<Box<Node<T>>> where T: FromStr {
    let val = *nodes.next().expect("Correct your input, value should be available");
    if val == "x" {
        return None;
    }
    let left = build_tree::<T>(nodes);
    let right = build_tree::<T>(nodes);
    let parsed = val.parse::<T>();
    match parsed {
        Err(_) => None,
        Ok(v) =>
            Some(
                Box::new(
                    Node::<T> { val: v, left, right }
                )
            )
    }
}

#[test]
fn build_tree_from_str_should_return_empty_tree() {
    let input = "x";
    let result: Option<Box<Node<i32>>> = build_tree_from_str(input);
    assert!(result.is_none());
}

#[test]
fn build_tree_from_str_should_return_a_tree() {
    let input = "5 4 3 x x 8 x x 6 x x";

//             ┌── 5 ──┐
//             ▼       ▼
//          ┌─ 4 ─┐    6
//          ▼     ▼
//          3     8
    let expected_tree: Node<i32> =
        Node::new(5)
            .left(
                Node::new(4)
                    .left(Node::new(3))
                    .right(Node::new(8))
            )
            .right(
                Node::new(6)
            );

    let result: Option<Box<Node<i32>>> = build_tree_from_str(input);
    assert_eq!(result, Some(Box::new(expected_tree)));
}

