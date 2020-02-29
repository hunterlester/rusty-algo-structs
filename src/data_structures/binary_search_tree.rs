use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct BST {
    value: i32,
    left: Option<Option<Rc<RefCell<BST>>>>,
    right: Option<Option<Rc<RefCell<BST>>>>,
    count: i32,
}

impl BST {
    pub fn new(value: i32) -> Self {
        BST {
            value,
            left: Some(None),
            right: Some(None),
            count: 1,
        }
    }

    fn insert_left(&mut self, value: i32) -> () {
        match self.left.as_ref() {
            Some(Some(node)) => {
                node.borrow_mut().insert(value);
            },
            Some(None) => {
                self.left = Some(Some(Rc::new(RefCell::new(BST::new(value)))));
            },
            None => (),
        };
    }

    fn insert_right(&mut self, value: i32) -> () {
        match self.right.as_ref() {
            Some(Some(node)) => {
                node.borrow_mut().insert(value);
            },
            Some(None) => {
                self.right = Some(Some(Rc::new(RefCell::new(BST::new(value)))));
            },
            None => (),
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
        if let Some(Some(node)) = self.left.as_ref() {
            node.borrow().inorder_traverse()
        } else {
            vec![]
        }
    }

    fn inorder_traverse_right(&self) -> Vec<i32> {
        if let Some(Some(node)) = self.right.as_ref() {
            node.borrow().inorder_traverse()
        } else {
            vec![]
        }
    }

    pub fn inorder_traverse(&self) -> Vec<i32> {
        vec![self.inorder_traverse_left(), vec![self.value], self.inorder_traverse_right()].concat()
    }

    fn preorder_traverse_left(&self) -> Vec<i32> {
        if let Some(Some(node)) = self.left.as_ref() {
            node.borrow().preorder_traverse()
        } else {
            vec![]
        }
    }

    fn preorder_traverse_right(&self) -> Vec<i32> {
        if let Some(Some(node)) = self.right.as_ref() {
            node.borrow().preorder_traverse()
        } else {
            vec![]
        }
    }

    pub fn preorder_traverse(&self) -> Vec<i32> {
        vec![vec![self.value], self.preorder_traverse_left(), self.preorder_traverse_right()].concat()
    }

    fn postorder_traverse_left(&self) -> Vec<i32> {
        if let Some(Some(node)) = self.left.as_ref() {
            node.borrow().postorder_traverse()
        } else {
            vec![]
        }
    }

    fn postorder_traverse_right(&self) -> Vec<i32> {
        if let Some(Some(node)) = self.right.as_ref() {
            node.borrow().postorder_traverse()
        } else {
            vec![]
        }
    }

    pub fn postorder_traverse(&self) -> Vec<i32> {
        vec![self.postorder_traverse_left(), self.postorder_traverse_right(), vec![self.value]].concat()
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
        if let Some(Some(node)) = bst.right {
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
        if let Some(Some(node)) = bst.left.as_ref() {
            assert_eq!(node.borrow().value, 9);
        }
        if let Some(Some(node)) = bst.right.as_ref() {
            assert_eq!(node.borrow().value, 19);
        }
        if let Some(Some(node)) = bst.left.as_ref() {
            if let Some(Some(left_node)) = node.borrow().left.as_ref() {
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
        if let Some(Some(node)) = bst.right.as_ref() {
            assert_eq!(node.borrow().count, 3);
        }
    }

//            10
//        9        29
//    7         19

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
}