enum BinaryTree<T> {
    Empty,
    Node(Box<Node<T>>),
}

struct Node<T> {
    Value: T,
    Left: BinaryTree<T>,
    Right: BinaryTree<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            Value: value,
            Left: BinaryTree::Empty,
            Right: BinaryTree::Empty,
        }
    }
}

fn main() {
    let root = BinaryTree::Node(Box::new(Node::<i32>::new(1)));
    let left = BinaryTree::Node(Box::new(Node::new(2)));
    let right = BinaryTree::Node(Box::new(Node::new(3)));
    let mut root_node = match root {
        BinaryTree::Node(node) => node,
        _ => panic!("root is not a node"),
    };
    root_node.Left = left;
    root_node.Right = right;
    let left_val = match root_node.Left {
        BinaryTree::Node(node) => node.Value,
        _ => 0,
    };
    println!("{:?}", left_val);
}
