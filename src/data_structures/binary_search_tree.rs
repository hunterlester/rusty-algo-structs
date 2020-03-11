use std::rc::Rc;
use std::cell::RefCell;

type PotentialNode = Option<Rc<RefCell<BST>>>;

#[derive(Debug, Clone)]
pub struct BST {
    value: i32,
    left: PotentialNode,
    right: PotentialNode,
    count: i32,
}

impl BST {
    pub fn new(value: i32) -> Self {
        BST {
            value,
            left: None,
            right: None,
            count: 1,
        }
    }

    fn insert_left(&mut self, value: i32) -> () {
        match self.left.as_ref() {
            Some(node) => {
                node.borrow_mut().insert(value);
            },
            None => {
                self.left = Some(Rc::new(RefCell::new(BST::new(value))));
            },
        };
    }

    fn insert_right(&mut self, value: i32) -> () {
        match self.right.as_ref() {
            Some(node) => {
                node.borrow_mut().insert(value);
            },
            None => {
                self.right = Some(Rc::new(RefCell::new(BST::new(value))));
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

    fn inorder_traverse_left(&self) -> Vec<i32> {
        if let Some(node) = self.left.as_ref() {
            node.borrow().inorder_traverse()
        } else {
            vec![]
        }
    }

    fn inorder_traverse_right(&self) -> Vec<i32> {
        if let Some(node) = self.right.as_ref() {
            node.borrow().inorder_traverse()
        } else {
            vec![]
        }
    }

    pub fn inorder_traverse(&self) -> Vec<i32> {
        vec![self.inorder_traverse_left(), vec![self.value], self.inorder_traverse_right()].concat()
    }

    fn preorder_traverse_left(&self) -> Vec<i32> {
        if let Some(node) = self.left.as_ref() {
            node.borrow().preorder_traverse()
        } else {
            vec![]
        }
    }

    fn preorder_traverse_right(&self) -> Vec<i32> {
        if let Some(node) = self.right.as_ref() {
            node.borrow().preorder_traverse()
        } else {
            vec![]
        }
    }

    pub fn preorder_traverse(&self) -> Vec<i32> {
        vec![vec![self.value], self.preorder_traverse_left(), self.preorder_traverse_right()].concat()
    }

    fn postorder_traverse_left(&self) -> Vec<i32> {
        if let Some(node) = self.left.as_ref() {
            node.borrow().postorder_traverse()
        } else {
            vec![]
        }
    }

    fn postorder_traverse_right(&self) -> Vec<i32> {
        if let Some(node) = self.right.as_ref() {
            node.borrow().postorder_traverse()
        } else {
            vec![]
        }
    }

    pub fn postorder_traverse(&self) -> Vec<i32> {
        vec![self.postorder_traverse_left(), self.postorder_traverse_right(), vec![self.value]].concat()
    }

    pub fn inorder_iterate(&self) -> Vec<i32> {
        let mut stack: Vec<Rc<RefCell<BST>>> = Vec::new();
        let mut inorder = Vec::new();
        let mut current_node = Some(Rc::new(RefCell::new(self.clone())));
        loop {
            let stack_has_len = stack.len() != 0;
            let some_current_node = match &current_node {
                Some(_n) => true,
                None => false,
            };
            if !stack_has_len && !some_current_node {
                break;
            }
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

    pub fn preorder_iterate(&self) -> Vec<i32> {
        let mut stack: Vec<Rc<RefCell<BST>>> = Vec::new();
        let mut preorder = Vec::new();
        let mut current_node = Some(Rc::new(RefCell::new(self.clone())));
        loop {
            let stack_has_len = stack.len() != 0;
            let some_current_node = match &current_node {
                Some(_n) => true,
                None => false,
            };
            if !stack_has_len && !some_current_node {
                break;
            }
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

    pub fn postorder_iterate(&self) -> Vec<i32> {
        let mut left_stack: Vec<Rc<RefCell<BST>>> = Vec::new();
        let mut right_stack: Vec<Rc<RefCell<BST>>> = Vec::new();
        let mut postorder = Vec::new();
        let mut current_node = Some(Rc::new(RefCell::new(self.clone())));
        loop {
            let stack_has_len = left_stack.len() != 0 || right_stack.len() != 0;
            let some_current_node = match &current_node {
                Some(_n) => true,
                None => false,
            };
            if !stack_has_len && !some_current_node {
                break;
            }
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

    pub fn delete(&mut self, id: i32) -> () {
        let mut preorder: Vec<i32> = self.preorder_iterate().iter().filter_map(|value| {
            if *value != id {
                Some(*value)
            } else {
                None
            }
        }).collect();
        let mut bst = BST::new(preorder.remove(0));
        for node_value in preorder {
            bst.insert(node_value);
        }
        *self = bst;
    }
}

#[cfg(test)]
mod tests {
    use super::{BST};

    #[test]
    fn new_bst() {
        let bst = BST::new(1);
        assert_eq!(bst.value, 1);
        assert_eq!(bst.count, 1);
        if let None = bst.left {
            assert!(true);
        }
        if let None = bst.right {
            assert!(true);
        }
    }

    #[test]
    fn bst_insert() {
        let mut bst = BST::new(1);
        bst.insert(2);
        if let Some(node) = bst.right {
            assert_eq!(node.borrow().value, 2);
        }
    }

    #[test]
    fn bst_insert_two() {
        let mut bst = BST::new(10);
        assert_eq!(bst.value, 10);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        if let Some(node) = bst.left.as_ref() {
            assert_eq!(node.borrow().value, 9);
        }
        if let Some(node) = bst.right.as_ref() {
            assert_eq!(node.borrow().value, 19);
        }
        if let Some(node) = bst.left.as_ref() {
            if let Some(left_node) = node.borrow().left.as_ref() {
                assert_eq!(left_node.borrow().value, 7);
            }
        }
    }

    #[test]
    fn bst_duplicate_count() {
        let mut bst = BST::new(1);
        bst.insert(3);
        bst.insert(3);
        bst.insert(3);
        if let Some(node) = bst.right.as_ref() {
            assert_eq!(node.borrow().count, 3);
        }
    }

    #[test]
    fn inorder_traverse() {
        let mut bst = BST::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        assert_eq!(bst.inorder_traverse(), vec![7, 9, 10, 19, 29]);
    }

    #[test]
    fn preorder_traverse() {
        let mut bst = BST::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        assert_eq!(bst.preorder_traverse(), vec![10, 9, 7, 29, 19]);
    }

    #[test]
    fn postorder_traverse() {
        let mut bst = BST::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        assert_eq!(bst.postorder_traverse(), vec![7, 9, 19, 29, 10]);
    }

    #[test]
    fn inorder_iterate() {
        let mut bst = BST::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        assert_eq!(bst.inorder_iterate(), vec![7, 9, 10, 19, 29]);
    }

    #[test]
    fn preorder_iterate() {
        let mut bst = BST::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        assert_eq!(bst.preorder_iterate(), vec![10, 9, 7, 29, 19]);
    }

    #[test]
    fn postorder_iterate() {
        let mut bst = BST::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        bst.insert(5);
        bst.insert(8);
        bst.insert(1);
        assert_eq!(bst.postorder_iterate(), vec![1, 5, 8, 7, 9, 19, 29, 10]);
    }

    #[test]
    fn delete() {
        let mut bst = BST::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        bst.delete(9);
        assert_eq!(bst.preorder_iterate(), vec![10, 7, 29, 19]);
    }
}