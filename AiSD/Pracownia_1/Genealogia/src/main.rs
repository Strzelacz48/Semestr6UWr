struct TreeNode {
    value: i32,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(value: i32, children: Vec<TreeNode>) -> TreeNode {
        TreeNode { value, children }
    }

    fn display(&self, depth: usize) {
        println!("{}{}", "  ".repeat(depth), self.value);
        for child in &self.children {
            child.display(depth + 1);
        }
    }

}
fn is_ancestor(tree: &TreeNode, query: (i32,i32)) -> String{
    //bin search na dzieciach po ich wartosciach aby wchodzic głębiej bo można być pewnym że dzieci po lewej mają mniejszy indeks bo tak je wpisujemy
    "NIE".to_string()
}

fn make_tree(tree_input: Vec<i32>) -> TreeNode{
    let mut tree = TreeNode::new(1,vec![]);
    //let mut tree = TreeNode::new(tree_input[0],vec![]);
    for i in 0..tree_input.len(){
        tree.children.push(TreeNode::new(tree_input[i],vec![]));
    }
    tree
}

fn main() {
    let tree = TreeNode::new(1, vec![
        TreeNode::new(2, vec![
            TreeNode::new(4, vec![]),
            TreeNode::new(5, vec![])
        ]),
        TreeNode::new(3, vec![
            TreeNode::new(6, vec![]),
            TreeNode::new(7, vec![])
        ])
    ]);
    //let mut tree = TreeNode::new(1,vec![]);
    tree.display(0);
    //input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n = input[0];
    let m = input[1];
    let mut tree_vec: Vec<i32> = Vec::new();
    //Tree input
    for i in 1..n+1{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        tree_vec.push(input.trim().parse().unwrap());
    }
    let tree = make_tree(tree_vec);
    //Query
    let mut query: Vec<(i32,i32)> = Vec::new();
    for i in 0..m{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        query.push((input.trim().parse().unwrap(),input.trim().parse().unwrap()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let tree = TreeNode::new(1, vec![
            TreeNode::new(2, vec![
                TreeNode::new(4, vec![]),
                TreeNode::new(5, vec![])
            ]),
            TreeNode::new(3, vec![
                TreeNode::new(6, vec![]),
                TreeNode::new(7, vec![])
            ])
        ]);
        tree.display(0);
    }
    #[test]
    fn test_a(){
        let tree_input = vec![1,1,3,3];
        let query = vec![(1,2),(2,1),(1,4),(2,5)];
        let tree = TreeNode::new(1, vec![
            TreeNode::new(2, vec![]),
            TreeNode::new(3, vec![
                TreeNode::new(4, vec![]),
                TreeNode::new(5, vec![])
            ])
        ]);
        //assert_eq!(make_tree(tree_input),tree);
        assert_eq!(is_ancestor(&tree,query[0]),"TAK");
        assert_eq!(is_ancestor(&tree,query[1]),"NIE");
        assert_eq!(is_ancestor(&tree,query[2]),"TAK");
        assert_eq!(is_ancestor(&tree,query[3]),"NIE");
    }
    #[test]
    fn test_b(){
        let tree_input = vec![1];
        let query = vec![(1,2),(2,1)];
        let tree = TreeNode::new(1, vec![
            TreeNode::new(2, vec![])
        ]);
        assert_eq!(is_ancestor(&tree,query[0]),"TAK");
        assert_eq!(is_ancestor(&tree,query[1]),"NIE");
        assert_eq!(is_ancestor(&tree,query[2]),"TAK");
        assert_eq!(is_ancestor(&tree,query[3]),"NIE");
    }
}
