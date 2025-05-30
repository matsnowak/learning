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
use std::cmp::max;
use std::fmt::Display;
use std::ops::Deref;
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
impl<T> Node<T>
where
    T: Copy,
{
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
fn build_tree_from_str<T>(input: &str) -> Option<Box<Node<T>>>
where
    T: FromStr,
{
    let tokens: Vec<&str> = input.split(' ').collect();
    let token_iterator = &mut tokens.iter();
    build_tree(token_iterator)
}

#[allow(dead_code)]
fn build_tree<T>(nodes: &mut Iter<&str>) -> Option<Box<Node<T>>>
where
    T: FromStr,
{
    let val = *nodes
        .next()
        .expect("Correct your input, value should be available");
    if val == "x" {
        return None;
    }
    let left = build_tree::<T>(nodes);
    let right = build_tree::<T>(nodes);
    let parsed = val.parse::<T>();
    match parsed {
        Err(_) => None,
        Ok(v) => Some(Box::new(Node::<T> {
            val: v,
            left,
            right,
        })),
    }
}

#[allow(dead_code)]
fn pre_order_traversal_recursive<T>(node: &Node<T>, output: &mut Vec<T>)
where
    T: Copy,
{
    output.push(node.val);
    if let Some(node_box) = &node.left {
        pre_order_traversal_recursive(node_box.deref(), output)
    }
    if let Some(node_box) = &node.right {
        pre_order_traversal_recursive(node_box.deref(), output)
    }
}

#[allow(dead_code)]
fn pre_order_traversal<T>(node: &Node<T>) -> Vec<T>
where
    T: Copy,
{
    let mut output: Vec<T> = Vec::new();
    pre_order_traversal_recursive(node, &mut output);
    output
}

#[allow(dead_code)]
fn in_order_traversal_recursive<T>(node: &Node<T>, output: &mut Vec<T>)
where
    T: Copy,
{
    if let Some(node_box) = &node.left {
        in_order_traversal_recursive(node_box.deref(), output)
    }
    output.push(node.val);
    if let Some(node_box) = &node.right {
        in_order_traversal_recursive(node_box.deref(), output)
    }
}

#[allow(dead_code)]
fn in_order_traversal<T>(node: &Node<T>) -> Vec<T>
where
    T: Copy,
{
    let mut output: Vec<T> = Vec::new();
    in_order_traversal_recursive(node, &mut output);
    output
}

#[allow(dead_code)]
fn post_order_traversal_recursive<T>(node: &Node<T>, output: &mut Vec<T>)
where
    T: Copy,
{
    if let Some(node_box) = &node.left {
        post_order_traversal_recursive(node_box.deref(), output)
    }
    if let Some(node_box) = &node.right {
        post_order_traversal_recursive(node_box.deref(), output)
    }
    output.push(node.val);
}

#[allow(dead_code)]
fn post_order_traversal<T>(node: &Node<T>) -> Vec<T>
where
    T: Copy,
{
    let mut output: Vec<T> = Vec::new();
    post_order_traversal_recursive(node, &mut output);
    output
}

#[allow(dead_code)]
fn tree_max_depth<T>(node: Node<T>) -> usize {
    fn dfs<T>(maybe_node: Option<Box<Node<T>>>) -> usize {
        match maybe_node {
            None => 0,
            Some(node) => 1 + max(dfs(node.left), dfs(node.right)),
        }
    }
    dfs(Some(Box::new(node))) - 1
    // Alternate solution
    // let left_depth: usize = if let Some(left) = &node.left {
    //     tree_max_depth(left) + 1
    // } else { 0 };
    // let right_depth: usize = if let Some(right) = &node.right {
    //     tree_max_depth(right) + 1
    // } else { 0 };
    // max(left_depth, right_depth)
}

#[allow(dead_code)]
fn serialize<T>(node: Option<Box<Node<T>>>) -> String
where
    T: Copy + Display,
{
    let mut results: Vec<String> = Vec::new();
    fn dfs<T>(node: Option<Box<Node<T>>>, results: &mut Vec<String>)
    where
        T: Copy + Display,
    {
        results.push(
            node.as_ref()
                .map(|node| node.val.to_string())
                .unwrap_or("x".to_string()),
        );
        if let Some(some_node) = node {
            dfs(some_node.left, results);
            dfs(some_node.right, results);
        }
    }
    dfs(node, &mut results);
    results.join(" ")
}

#[allow(dead_code)]
fn lowest_common_ancestor_in_binary_tree<'a, T: PartialEq>(
    tree: &'a Option<Box<Node<T>>>,
    node1: &'a Option<Box<Node<T>>>,
    node2: &'a Option<Box<Node<T>>>,
) -> &'a Option<Box<Node<T>>> {
    if tree.is_none() {
        return &None;
    }
    if tree == node1 || tree == node2 {
        return tree;
    }
    if let Some(some_tree) = tree {
        let left = lowest_common_ancestor_in_binary_tree(&some_tree.left, node1, node2);
        let right = lowest_common_ancestor_in_binary_tree(&some_tree.right, node1, node2);

        if left.is_some() && right.is_some() {
            return tree;
        }
        if left.is_some() {
            return left;
        }
        if right.is_some() {
            return right;
        }
    }
    &None
}

#[allow(dead_code)]
#[allow(clippy::bind_instead_of_map)]
fn invert_binary_tree<T>(node: &Option<Box<Node<T>>>) -> Option<Box<Node<T>>>
where
    T: Copy,
{
    node.as_ref().and_then(|node| {
        Some(Box::new(Node {
            val: node.val,
            left: invert_binary_tree(&node.right),
            right: invert_binary_tree(&node.left),
        }))
    })
}

#[allow(dead_code)]
fn is_valid_binary_search_tree(node: Option<Box<Node<i32>>>) -> bool {
    fn dfs(node: &Option<Box<Node<i32>>>, min_value: i32, max_value: i32) -> bool {
        match node {
            None => true,
            Some(some_node) => {
                if !((min_value < some_node.val) && (some_node.val <= max_value)) {
                    return false;
                }
                dfs(&some_node.left, min_value, some_node.val)
                    && dfs(&some_node.right, some_node.val, max_value)
            }
        }
    }
    dfs(&node, i32::MIN, i32::MAX)
}

#[allow(dead_code)]
fn find_node_binary_tree<T: PartialEq + Copy>(
    tree: &Option<Box<Node<T>>>,
    val: T,
) -> &Option<Box<Node<T>>> {
    if let Some(ref some_tree) = tree {
        return if some_tree.val == val {
            tree
        } else {
            let left = find_node_binary_tree(&some_tree.left, val);
            let right = find_node_binary_tree(&some_tree.right, val);
            if left.is_some() {
                left
            } else {
                right
            }
        };
    }
    &None
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let expected_tree: Node<i32> = Node::new(5)
            .left(Node::new(4).left(Node::new(3)).right(Node::new(8)))
            .right(Node::new(6));

        let result: Option<Box<Node<i32>>> = build_tree_from_str(input);
        assert_eq!(result, Some(Box::new(expected_tree)));
    }

    #[test]
    fn pre_order_traversal_non_empty_tree() {
        let tree = build_tree_from_str("1 2 4 x x 5 x x 3 6 x x 7 x x");
        //        ┌─── 1 ───┐
        //        ▼         ▼
        //     ┌─ 2 ─┐   ┌─ 3 ─┐
        //     ▼     ▼   ▼     ▼
        //     4     5   6     7

        let result: Vec<i32> = pre_order_traversal(&tree.unwrap());

        assert_eq!(result, vec![1, 2, 4, 5, 3, 6, 7])
    }

    #[test]
    fn in_order_traversal_non_empty_tree() {
        let tree = build_tree_from_str("1 2 4 x x 5 x x 3 6 x x 7 x x");
        //        ┌─── 1 ───┐
        //        ▼         ▼
        //     ┌─ 2 ─┐   ┌─ 3 ─┐
        //     ▼     ▼   ▼     ▼
        //     4     5   6     7

        let result: Vec<i32> = in_order_traversal(&tree.unwrap());

        assert_eq!(result, vec![4, 2, 5, 1, 6, 3, 7])
    }

    #[test]
    fn post_order_traversal_non_empty_tree() {
        let tree = build_tree_from_str("1 2 4 x x 5 x x 3 6 x x 7 x x");
        //        ┌─── 1 ───┐
        //        ▼         ▼
        //     ┌─ 2 ─┐   ┌─ 3 ─┐
        //     ▼     ▼   ▼     ▼
        //     4     5   6     7

        let result: Vec<i32> = post_order_traversal(&tree.unwrap());

        assert_eq!(result, vec![4, 5, 2, 6, 7, 3, 1])
    }

    #[test]
    fn tree_max_depth_should_calculate_for_non_empty() {
        let tree = build_tree_from_str("1 2 4 x x 5 x x 3 6 x x 7 x x");
        let result: usize = tree_max_depth::<i32>(*tree.unwrap());
        assert_eq!(result, 2)
    }

    #[test]
    fn tree_max_depth_should_calculate_for_root_only() {
        let tree = build_tree_from_str("1 x x");
        let result: usize = tree_max_depth::<i32>(*tree.unwrap());
        assert_eq!(result, 0)
    }

    #[test]
    fn serialize_non_empty_tree() {
        let tree: Option<Box<Node<i32>>> = build_tree_from_str("6 4 3 x x 5 x x 8 x x");
        let result = serialize(tree);
        assert_eq!(result, "6 4 3 x x 5 x x 8 x x")
    }

    #[test]
    fn invert_binary_tree_should_return_inverted_tree() {
        let tree: Option<Box<Node<i32>>> = build_tree_from_str("1 2 4 x x 5 6 x x x 3 x x");
        let inverted_tree = invert_binary_tree(&tree);
        let serialized_inverted_tree = serialize(inverted_tree);
        assert_eq!(serialized_inverted_tree, "1 3 x x 2 5 x 6 x x 4 x x")
    }

    #[test]
    fn is_valid_binary_search_tree_test_valid_tree() {
        let tree: Option<Box<Node<i32>>> = build_tree_from_str("6 4 3 x x 5 x x 8 x x");
        assert!(is_valid_binary_search_tree(tree))
    }

    #[test]
    fn is_valid_binary_search_tree_test_invalid_tree() {
        let tree: Option<Box<Node<i32>>> = build_tree_from_str("6 4 3 x x 8 x x 8 x x");
        assert!(!is_valid_binary_search_tree(tree))
    }

    #[test]
    fn find_node_in_binary_tree_test() {
        //        ┌─── 1 ───┐
        //        ▼         ▼
        //     ┌─ 2 ─┐   ┌─ 3 ─┐
        //     ▼     ▼   ▼     ▼
        //     4     5   6     7
        let tree: Option<Box<Node<i32>>> = build_tree_from_str("1 2 4 x x 5 x x 3 6 x x 7 x x");
        let result = find_node_binary_tree(&tree, 3).as_ref().unwrap();
        assert_eq!(result.val, 3);
    }

    #[test]
    fn lowest_common_ancestor_in_binary_tree_test() {
        //        ┌─── 6 ───┐
        //        ▼         ▼
        //     ┌─ 4 ─┐      8
        //     ▼     ▼
        //     3     5
        let tree: Option<Box<Node<i32>>> = build_tree_from_str("6 4 3 x x 5 x x 8 x x");
        let node1 = find_node_binary_tree(&tree, 4);
        let node2 = find_node_binary_tree(&tree, 8);
        let result = lowest_common_ancestor_in_binary_tree(&tree, node1, node2);
        assert_eq!(result, find_node_binary_tree(&tree, 6));
    }
}
