use std::rc::Rc;
use std::cell::RefCell;

type PotentialNode = Option<Rc<RefCell<IterativeBinarySearchTree>>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IterativeBinarySearchTree {
    value: i32,
    left: PotentialNode,
    right: PotentialNode,
    count: i32,
}

impl IterativeBinarySearchTree {
    pub fn new(value: i32) -> Self {
        IterativeBinarySearchTree {
            value,
            left: None,
            right: None,
            count: 1,
        }
    }

    // pub fn insert_iterate(&mut self, value: i32) -> () {
    //     let mut node_index = Rc::new(RefCell::new(*self));
    //     println!("node_index: {:?}", &node_index);
    //     let mut insert_left: Option<Rc<RefCell<IterativeBinarySearchTree>>> = None;
    //     let mut insert_right: Option<Rc<RefCell<IterativeBinarySearchTree>>> = None;
    //     let mut increment_count: bool = false;

    //     loop {
    //         let next_node: Rc<RefCell<IterativeBinarySearchTree>>;
    //         if let Some(node) = insert_left.as_ref() {
    //             println!("makes it here, insert_left: {:?}", &insert_left);
    //             node_index.borrow_mut().left = insert_left;
    //             println!("self: {:?}", self);
    //             println!("node_index: {:?}", &node_index);
    //             break;
    //         } else if let Some(node) = insert_right.as_ref() {
    //             node_index.borrow_mut().right = insert_right;
    //             break;
    //         } else if increment_count {
    //             node_index.borrow_mut().count += 1;
    //             break;
    //         }
    //         if value == node_index.borrow().value {
    //             increment_count = true;
    //             continue;
    //         } else if value < node_index.borrow().value {
    //             match node_index.borrow().left.as_ref() {
    //                 Some(node) => {
    //                     println!("setting next node");
    //                     next_node = Rc::clone(&node);
    //                 },
    //                 None => {
    //                     insert_left = Some(Rc::new(RefCell::new(IterativeBinarySearchTree::new(value))));
    //                     continue;
    //                 },
    //             }
    //         } else {
    //             match node_index.borrow().right.as_ref() {
    //                 Some(node) => {
    //                     next_node = Rc::clone(&node);
    //                 },
    //                 None => {
    //                     insert_right = Some(Rc::new(RefCell::new(IterativeBinarySearchTree::new(value))));
    //                     continue;
    //                 },
    //             }
    //         }
    //         println!("next_node: {:?}", &next_node);
    //         node_index = Rc::clone(&next_node);
    //     }
    // }


    fn insert_left(&mut self, value: i32) -> () {
        match self.left.as_ref() {
            Some(node) => {
                node.borrow_mut().insert(value);
            },
            None => {
                self.left = Some(Rc::new(RefCell::new(IterativeBinarySearchTree::new(value))));
            },
        };
    }

    fn insert_right(&mut self, value: i32) -> () {
        match self.right.as_ref() {
            Some(node) => {
                node.borrow_mut().insert(value);
            },
            None => {
                self.right = Some(Rc::new(RefCell::new(IterativeBinarySearchTree::new(value))));
            },
        };
    }

    pub fn insert(&mut self, value: i32) -> () {
        if value == self.value {
            self.count += 1;
        } else if value < self.value {
            // traverse left
            self.insert_left(value);
        } else {
            // traverse right
            self.insert_right(value);
        }
    }

    pub fn preorder(&self) -> Vec<i32> {
        let mut stack: Vec<Option<Rc<RefCell<IterativeBinarySearchTree>>>> = Vec::new();
        stack.push(Some(Rc::new(RefCell::new(self.clone()))));
        let mut preorder = Vec::new();
        while let Some(Some(root)) = stack.pop() {
            let borrowed_root = root.borrow();
            preorder.push(borrowed_root.value);
            if let Some(right_child) = borrowed_root.right.as_ref() {
                stack.push(Some(Rc::clone(&right_child)));
            }
            if let Some(left_child) = borrowed_root.left.as_ref() {
                stack.push(Some(Rc::clone(&left_child)));
            }
        }
        preorder
    }

    pub fn inorder(&self) -> Vec<i32> {
        let mut stack: Vec<Option<Rc<RefCell<IterativeBinarySearchTree>>>> = Vec::new();
        let mut current_node = Some(Rc::new(RefCell::new(self.clone())));
        let mut inorder = Vec::new();
        while current_node != None || stack.len() != 0 {
            while let Some(node) = current_node {
                let borrowed_node = node.borrow();
                stack.push(Some(Rc::clone(&node)));
                current_node = if let Some(left_child) = borrowed_node.left.as_ref() {
                    Some(Rc::clone(&left_child))
                } else {None};
            }
            if let Some(Some(node)) = stack.pop() {
                let borrowed_node = node.borrow();
                inorder.push(borrowed_node.value);
                current_node = if let Some(right_child) = borrowed_node.right.as_ref() {
                    Some(Rc::clone(&right_child))
                } else {None};
            }
        }
        inorder
    }

    pub fn postorder(&self) -> Vec<i32> {
        let mut stack: Vec<Option<Rc<RefCell<IterativeBinarySearchTree>>>> = Vec::new();
        stack.push(Some(Rc::new(RefCell::new(self.clone()))));
        let mut postorder = Vec::new();
        while let Some(Some(root)) = stack.pop() {
            let borrowed_root = root.borrow();
            postorder.push(borrowed_root.value);
            if let Some(left_child) = borrowed_root.left.as_ref() {
                stack.push(Some(Rc::clone(&left_child)));
            }
            if let Some(right_child) = borrowed_root.right.as_ref() {
                stack.push(Some(Rc::clone(&right_child)));
            }
        }
        postorder.reverse();
        postorder
    }

    // pub fn bfs(&self) -> Vec<i32> {
    //     let mut level_order: Vec<i32> = Vec::new();
    //     let mut queue: Vec<Rc<RefCell<IterativeBinarySearchTree>>> = vec![Rc::clone(self)];
    //     let mut level: Vec<Rc<RefCell<IterativeBinarySearchTree>>> = Vec::new();
    //     'outer: loop {
    //         'inner: loop {
    //             let node = queue.remove(0);
    //             if let Some(left_child) = node.borrow().left.as_ref() {
    //                 level.push(Rc::clone(&left_child));
    //             }
    //             if let Some(right_child) = node.borrow().right.as_ref() {
    //                 level.push(Rc::clone(&right_child));
    //             }
    //             level_order.push(node.borrow().value);
    //             if queue.len() == 0 {
    //                 break 'inner;
    //             }
    //         }
    //         queue.append(&mut level);
    //         if queue.len() == 0 && level.len() == 0 {
    //             break 'outer;
    //         }
    //     }
    //     level_order
    // }
}

#[cfg(test)]
mod tests {
    use super::IterativeBinarySearchTree;

    #[test]
    fn inorder() {
        let mut bst = IterativeBinarySearchTree::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        assert_eq!(bst.inorder(), vec![7, 9, 10, 19, 29]);
    }

    #[test]
    fn preorder() {
        let mut bst = IterativeBinarySearchTree::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        assert_eq!(bst.preorder(), vec![10, 9, 7, 29, 19]);
    }

    #[test]
    fn postorder() {
        let mut bst = IterativeBinarySearchTree::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        bst.insert(5);
        bst.insert(8);
        bst.insert(1);
        assert_eq!(bst.postorder(), vec![1, 5, 8, 7, 9, 19, 29, 10]);
    }

    // #[test]
    // fn level_order() {
    //     let mut bst = IterativeBinarySearchTree::new(10);
    //     bst.insert(29);
    //     bst.insert(9);
    //     bst.insert(7);
    //     bst.insert(19);
    //     bst.insert(5);
    //     bst.insert(8);
    //     bst.insert(1);
    //     assert_eq!(bst.bfs(), vec![10, 9 , 29, 7, 19, 5, 8, 1]);
    // }

    // #[test]
    // fn bst_insert_iteratively() {
    //     let mut bst = IterativeBinarySearchTree::new(10);
    //     bst.insert_iterate(9);
    //     // bst.insert_iterate(7);
    //     // bst.insert_iterate(19);
    //     assert_eq!(bst.preorder(), vec![10, 9, 7, 19]);
    // }
}