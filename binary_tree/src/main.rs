use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct BinaryTreeNode {
    pub value: i32,
    pub left: Option<Rc<RefCell<BinaryTreeNode>>>,
    pub right: Option<Rc<RefCell<BinaryTreeNode>>>,
}

impl BinaryTreeNode {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(BinaryTreeNode {
            value,
            left: None,
            right: None,
        }))
    }

    pub fn generate_tree(mut node_values: Vec<Option<i32>>) -> Rc<RefCell<Self>> {
        node_values.reverse();
        let tree_node: Rc<RefCell<BinaryTreeNode>> = BinaryTreeNode::new(node_values.pop().unwrap().unwrap());
        let mut stack: Vec<Rc<RefCell<BinaryTreeNode>>> = Vec::new();
        stack.push(Rc::clone(&tree_node));

        loop {
            if node_values.len() == 0 {
                break;
            }
            let mut leaves = Vec::new();
            for node in &stack {
                if let Some(Some(v)) = node_values.pop() {
                    let child_node = BinaryTreeNode::new(v);
                    node.borrow_mut().left = Some(Rc::clone(&child_node));
                    leaves.push(Rc::clone(&child_node));
                } else {
                    node.borrow_mut().left = None;
                }

                if let Some(Some(v)) = node_values.pop() {
                    let child_node = BinaryTreeNode::new(v);
                    node.borrow_mut().right = Some(Rc::clone(&child_node));
                    leaves.push(Rc::clone(&child_node));
                } else {
                    node.borrow_mut().right = None;
                }
            }
            stack.clear();
            stack.append(&mut leaves);
        }

        tree_node
    }
}

fn main() {
    let nodes = vec![Some(1), None, Some(2), Some(3)];
    let node_tree = BinaryTreeNode::generate_tree(nodes);
    println!("{:?}", node_tree);
}