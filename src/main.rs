mod data_structures;

use data_structures::BinaryTreeNode;

fn main() {
    let mut node_tree = BinaryTreeNode::new(1);
    node_tree.insert(None);
    // node_tree.insert(None);

    node_tree.insert(Some(7));
    node_tree.insert(Some(8));
    println!("{:?}", node_tree);
}