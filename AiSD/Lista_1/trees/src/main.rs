use std::cmp;

// Definicja struktury węzła drzewa binarnego
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

// Funkcja obliczająca maksymalną głębokość drzewa
fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            cmp::max(left_depth, right_depth) + 1
        }
    }
}

// Funkcja obliczająca maksymalną odległość między wierzchołkami
fn max_distance(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let max_left = max_depth(&node.left);
            let max_right = max_depth(&node.right);
            let left_distance = max_distance(&node.left);
            let right_distance = max_distance(&node.right);
            cmp::max(max_left + max_right, cmp::max(left_distance, right_distance))
        }
    }
}

fn main() {
    // Konstruowanie drzewa binarnego:
    //         1
    //        / \
    //       2   3
    //      / \
    //     4   5
    let mut root = TreeNode::new(1);
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(3)));
    root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));

    println!("Maksymalna odległość między wierzchołkami: {}", max_distance(&Some(Box::new(root))));
}
