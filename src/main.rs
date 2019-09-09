mod data_structures;

use data_structures::BinaryTreeNode;

fn main() {
    let nodes = vec![Some(1), None, Some(2), Some(3)];
    let node_tree = BinaryTreeNode::generate_tree(nodes);
    println!("{:?}", node_tree);
}