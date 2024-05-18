#[derive(Debug, Clone)]
struct AVLNode<T> {
    value: T,
    height: i32,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

impl<T: Ord + Clone> AVLNode<T> {
    fn new(value: T) -> Self {
        AVLNode {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height(node: &Option<Box<AVLNode<T>>>) -> i32 {
        match node {
            Some(node) => node.height,
            None => 0,
        }
    }

    fn update_height(node: &mut Box<AVLNode<T>>) {
        node.height = 1 + std::cmp::max(Self::height(&node.left), Self::height(&node.right));
    }

    fn balance_factor(node: &Option<Box<AVLNode<T>>>) -> i32 {
        match node {
            Some(node) => Self::height(&node.left) - Self::height(&node.right),
            None => 0,
        }
    }

    fn rotate_right(mut y: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let mut x = y.left.take().unwrap();
        y.left = x.right.take();
        x.right = Some(y);

        Self::update_height(x.right.as_mut().unwrap());
        Self::update_height(&mut x);

        x
    }

    fn rotate_left(mut x: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let mut y = x.right.take().unwrap();
        x.right = y.left.take();
        y.left = Some(x);

        Self::update_height(y.left.as_mut().unwrap());
        Self::update_height(&mut y);

        y
    }

    fn balance(mut node: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        Self::update_height(&mut node);

        let balance_factor = Self::balance_factor(&Some(node.clone()));
        if balance_factor > 1 {
            if Self::balance_factor(&node.left) < 0 {
                node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            }
            return Self::rotate_right(node);
        }
        if balance_factor < -1 {
            if Self::balance_factor(&node.right) > 0 {
                node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            }
            return Self::rotate_left(node);
        }

        node
    }

    fn insert(node: Option<Box<AVLNode<T>>>, value: T) -> Box<AVLNode<T>> {
        match node {
            Some(mut node) => {
                if value < node.value {
                    node.left = Some(Self::insert(node.left.take(), value));
                } else {
                    node.right = Some(Self::insert(node.right.take(), value));
                }
                Self::balance(node)
            }
            None => Box::new(Self::new(value)),
        }
    }
}

#[derive(Debug)]
pub struct AVLTree<T> {
    root: Option<Box<AVLNode<T>>>,
}

impl<T: Ord + Clone> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Some(AVLNode::insert(self.root.take(), value));
    }

    pub fn height(&self) -> i32 {
        AVLNode::height(&self.root)
    }
}

fn main() {
    let mut avl_tree = AVLTree::new();
    avl_tree.insert(10);
    avl_tree.insert(20);
    avl_tree.insert(30);
    avl_tree.insert(40);
    avl_tree.insert(50);
    avl_tree.insert(25);

    println!("{:#?}", avl_tree);
    println!("Height of tree: {}", avl_tree.height());
}

fn insert(x: i64) -> i64{
    return x;
}

fn delete(x: i64) -> i64{
    return x;
}

fn upper(x: i64) -> i64{
    return x;
}

fn lower(x: i64) -> i64{
    return x;
}