use std::rc::Rc;
use std::cell::RefCell;

type PotentialNode = Option<Rc<RefCell<BinarySearchTree>>>;

// for leetcode problem
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[derive(Debug, Clone)]
pub struct BinarySearchTree {
    value: i32,
    left: PotentialNode,
    right: PotentialNode,
    count: i32,
}

impl BinarySearchTree {
    pub fn new(value: i32) -> Self {
        BinarySearchTree {
            value,
            left: None,
            right: None,
            count: 1,
        }
    }

    /// Best case: O(log n), when tree is balanced
    /// Worst case: O(n) 
    pub fn insert(&mut self, value: i32) -> () {
        if value == self.value {
            self.count += 1;
        } else if value < self.value {
            if let Some(node) = self.left.as_ref() {
                node.borrow_mut().insert(value);
            } else {
                self.left = Some(Rc::new(RefCell::new(BinarySearchTree::new(value))));
            }
        } else {
            if let Some(node) = self.right.as_ref() {
                node.borrow_mut().insert(value);
            } else {
                self.right = Some(Rc::new(RefCell::new(BinarySearchTree::new(value))));
            }
        }
    }

    /// Takes about twice as long to complete however, time seems to grow 10x as fast as iterative counterpart as nodes scale
    pub fn inorder(&self) -> Vec<i32> {
        let left_child = if let Some(node) = self.left.as_ref() {
            node.borrow().inorder()
        } else {vec![]};

        let right_child = if let Some(node) = self.right.as_ref() {
            node.borrow().inorder()
        } else {vec![]};

        vec![left_child, vec![self.value], right_child].concat()
    }

    pub fn preorder(&self) -> Vec<i32> {
        let left_child = if let Some(node) = self.left.as_ref() {
            node.borrow().preorder()
        } else {vec![]};

        let right_child = if let Some(node) = self.right.as_ref() {
            node.borrow().preorder()
        } else {vec![]};

        vec![vec![self.value], left_child, right_child].concat()
    }

    pub fn postorder(&self) -> Vec<i32> {
        let left_child = if let Some(node) = self.left.as_ref() {
            node.borrow().postorder()
        } else {vec![]};

        let right_child = if let Some(node) = self.right.as_ref() {
            node.borrow().postorder()
        } else {vec![]};

        vec![left_child, right_child, vec![self.value]].concat()
    }

    /// Best case: O(log n), when tree is balanced
    /// Worst case: O(n) 
    pub fn delete(&mut self, id: i32) -> () {
        let mut preorder: Vec<i32> = self.preorder().iter().filter_map(|value| {
            if *value != id {
                Some(*value)
            } else {
                None
            }
        }).collect();
        let mut bst = BinarySearchTree::new(preorder.remove(0));
        for node_value in preorder {
            bst.insert(node_value);
        }
        *self = bst;
    }
}

fn build_bst(array: &Vec<Option<i32>>, index: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if index <= array.len() - 1 {
        let next_left_child_index = (index*2) + 1;
        let next_right_child_index = (index*2) + 2;
        if let Some(value) = array[index] {
            Some(Rc::new(RefCell::new(TreeNode {
                val: value,
                left: build_bst(&array, next_left_child_index),
                right: build_bst(&array, next_right_child_index),
            })))
        } else {
            None
        }
    } else {
        None
    }
}

fn search(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let inner_node = node.borrow();
        if val == inner_node.val {
            return Some(Rc::clone(&node));
        }
        if val < inner_node.val {
            if let Some(left_node) = inner_node.left.as_ref() {
                return search(Some(Rc::clone(&left_node)), val);
            } else {
                return None;
            }
        } else {
            if let Some(right_node) = inner_node.right.as_ref() {
                return search(Some(Rc::clone(&right_node)), val);
            } else {
                return None;
            }
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{BinarySearchTree, search, build_bst};

    #[test]
    fn test_search() {
        let array = vec![Some(4), Some(2), Some(7), Some(1), Some(3)];
        let bst = build_bst(&array, 0);

        let expected_array = vec![Some(2), Some(1) , Some(3)];
        let expected = build_bst(&expected_array, 0);
        assert_eq!(search(bst, 2), expected);
    }

    #[test]
    fn new_bst() {
        let bst = BinarySearchTree::new(1);
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
        let mut bst = BinarySearchTree::new(1);
        bst.insert(2);
        if let Some(node) = bst.right {
            assert_eq!(node.borrow().value, 2);
        }
    }

    #[test]
    fn bst_insert_two() {
        let mut bst = BinarySearchTree::new(10);
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
        let mut bst = BinarySearchTree::new(1);
        bst.insert(3);
        bst.insert(3);
        bst.insert(3);
        if let Some(node) = bst.right.as_ref() {
            assert_eq!(node.borrow().count, 3);
        }
    }

    #[test]
    fn inorder() {
        let mut bst = BinarySearchTree::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        assert_eq!(bst.inorder(), vec![7, 9, 10, 19, 29]);
    }

    #[test]
    fn preorder() {
        let mut bst = BinarySearchTree::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        assert_eq!(bst.preorder(), vec![10, 9, 7, 29, 19]);
    }

    #[test]
    fn postorder() {
        let mut bst = BinarySearchTree::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        assert_eq!(bst.postorder(), vec![7, 9, 19, 29, 10]);
    }

    #[test]
    fn delete() {
        let mut bst = BinarySearchTree::new(10);
        bst.insert(29);
        bst.insert(9);
        bst.insert(7);
        bst.insert(19);
        bst.delete(9);
        assert_eq!(bst.preorder(), vec![10, 7, 29, 19]);
    }
}