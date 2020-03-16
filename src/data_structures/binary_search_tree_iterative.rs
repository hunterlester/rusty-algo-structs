use std::rc::Rc;
use std::cell::RefCell;

type PotentialNode = Option<Rc<RefCell<IterativeBinarySearchTree>>>;

#[derive(Debug, Clone)]
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


    // pub fn insert_iterate(&self, new_value: i32) -> () {
    //     let mut node_index = Rc::new(RefCell::new(self.clone()));

    //     loop {
    //         let IterativeBinarySearchTree { value, left, right, mut count } = &*node_index.borrow_mut();
    //         let next_node: Rc<RefCell<IterativeBinarySearchTree>>;
    //         if new_value == *value {
    //             count += 1;
    //             break;
    //         } else if new_value < *value {
    //             match left.as_ref() {
    //                 Some(node) => {
    //                     next_node = Rc::clone(&node);
    //                 },
    //                 None => {
    //                     left = &Some(Rc::new(RefCell::new(IterativeBinarySearchTree::new(new_value))));
    //                     continue;
    //                 },
    //             };
    //         } else {
    //             match right.as_ref() {
    //                 Some(node) => {
    //                     next_node = Rc::clone(&node);
    //                 },
    //                 None => {
    //                     right = &Some(Rc::new(RefCell::new(IterativeBinarySearchTree::new(new_value))));
    //                     continue;
    //                 },
    //             };
    //         };
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

    pub fn inorder(&self) -> Vec<i32> {
        let mut stack: Vec<Rc<RefCell<IterativeBinarySearchTree>>> = Vec::new();
        let mut inorder = Vec::new();
        let mut current_node = Some(Rc::new(RefCell::new(self.clone())));
        loop {
            match (stack.len() != 0, &current_node) {
                (false, None) => break,
                _ => (),
            };
            if let Some(node) = current_node {
                stack.push(Rc::clone(&node));
                if let Some(n) = node.borrow().left.as_ref() {
                  current_node = Some(Rc::clone(&n));
                }  else {
                    current_node = None;
                }
            } else {
                if let Some(node) = stack.pop() {
                    inorder.push(node.borrow().value);
                    if let Some(n) = node.borrow().right.as_ref() {
                      current_node = Some(Rc::clone(&n));
                    }  else {
                        current_node = None;
                    }
                } else {
                    continue;
                }
            }
        }
        inorder
    }

    pub fn preorder(&self) -> Vec<i32> {
        let mut stack: Vec<Rc<RefCell<IterativeBinarySearchTree>>> = Vec::new();
        let mut preorder = Vec::new();
        let mut current_node = Some(Rc::new(RefCell::new(self.clone())));
        loop {
            match (stack.len() != 0, &current_node) {
                (false, None) => break,
                _ => (),
            };
            if let Some(node) = current_node {
                preorder.push(node.borrow().value);
                stack.push(Rc::clone(&node));
                if let Some(n) = node.borrow().left.as_ref() {
                  current_node = Some(Rc::clone(&n));
                }  else {
                    current_node = None;
                }
            } else {
                if let Some(node) = stack.pop() {
                    if let Some(n) = node.borrow().right.as_ref() {
                      current_node = Some(Rc::clone(&n));
                    }  else {
                        current_node = None;
                    }
                } else {
                    continue;
                }
            }
        }
        preorder
    }

    pub fn postorder(&self) -> Vec<i32> {
        let mut left_stack: Vec<Rc<RefCell<IterativeBinarySearchTree>>> = Vec::new();
        let mut right_stack: Vec<Rc<RefCell<IterativeBinarySearchTree>>> = Vec::new();
        let mut postorder = Vec::new();
        let mut current_node = Some(Rc::new(RefCell::new(self.clone())));
        loop {
            let stack_has_len = left_stack.len() != 0 || right_stack.len() != 0;
            match (stack_has_len, &current_node) {
                (false, None) => break,
                _ => (),
            };
            if let Some(node) = current_node {
                if right_stack.len() != 0 {
                    right_stack.push(Rc::clone(&node));
                } else {
                    left_stack.push(Rc::clone(&node));
                }
                if let Some(n) = node.borrow().left.as_ref() {
                  current_node = Some(Rc::clone(&n));
                }  else {
                    current_node = None;
                }
            } else {
                let stack;
                if right_stack.len() != 0 {
                    stack = &mut right_stack;
                } else {
                    stack = &mut left_stack;
                }
                if let Some(node) = stack.pop() {
                    if let Some(n) = node.borrow().right.as_ref() {
                      right_stack.push(Rc::clone(&node));
                      current_node = Some(Rc::clone(&n));
                    }  else {
                        postorder.push(node.borrow().value);
                        while let Some(r_node) = right_stack.pop() {
                            postorder.push(r_node.borrow().value);
                        }
                        current_node = None;
                    }
                } else {
                    continue;
                }
            }
        }
        postorder
    }
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
    // fn bst_insert_iteratively() {
    //     let bst = IterativeBinarySearchTree::new(10);
    //     bst.insert_iterate(9);
    //     bst.insert_iterate(7);
    //     bst.insert_iterate(19);
    //     assert_eq!(bst.preorder(), vec![10, 9, 7, 19]);
    // }
}