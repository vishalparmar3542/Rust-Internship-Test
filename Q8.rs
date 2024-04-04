struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: Option<&TreeNode>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left.as_ref());
            let right_depth = max_depth(node.right.as_ref());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Define your binary tree here and pass it to max_depth function
}
