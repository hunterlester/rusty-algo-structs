use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct BinaryTreeNode {
    pub value: i32,
    pub left: Option<Option<Rc<RefCell<BinaryTreeNode>>>>,
    pub right: Option<Option<Rc<RefCell<BinaryTreeNode>>>>,
}

impl BinaryTreeNode {
    pub fn new(value: i32) -> Self {
        BinaryTreeNode {
            value,
            left: Some(None),
            right: Some(None),
        }
    }

    fn insert_left(&mut self, value: Option<i32>) -> () {
        match self.left.as_ref() {
            Some(Some(node)) => {
                if let Some(None) = self.right.as_ref() {
                    self.insert_right(value)
                } else {
                    node.borrow_mut().insert(value);
                }
           },
            Some(None) => {
                if let Some(integer) = value {
                  self.left = Some(Some(Rc::new(RefCell::new(BinaryTreeNode::new(integer)))));
                } else {
                  self.left = None;
                }
            },
            None => {
                self.insert_right(value);
            },
        }
    }

    fn insert_right(&mut self, value: Option<i32>) -> () {
        match self.right.as_ref() {
          Some(Some(node)) => {
              node.borrow_mut().insert(value);
          },
          Some(None) => {
              if let Some(integer) = value {
                self.right = Some(Some(Rc::new(RefCell::new(BinaryTreeNode::new(integer)))));
              } else {
                self.right = None;
              }
          },
          None => {
              self.insert_left(value);
          },
        }
    }

    pub fn insert(&mut self, value: Option<i32>) -> () {
        if let None = self.left.as_ref() {
          if let None = self.right.as_ref() {
            ()
          } else {
            self.insert_left(value);
          }
        } else {
          self.insert_left(value);
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
    use super::{BinaryTreeNode};

    #[test]
    fn new() {
        let bt = BinaryTreeNode::new(1);
        assert_eq!(bt.value, 1);
        if let None = bt.left {
            assert!(true);
        }
        if let None = bt.right {
            assert!(true);
        }
    }

    #[test]
    fn insert() {
        let mut bt = BinaryTreeNode::new(1);
        bt.insert(Some(2));
        if let Some(Some(node)) = bt.right {
            assert_eq!(node.borrow().value, 2);
        }
    }

    #[test]
    fn insert_two() {
        let mut bt = BinaryTreeNode::new(10);
        assert_eq!(bt.value, 10);
        bt.insert(Some(9));
        bt.insert(Some(7));
        bt.insert(Some(19));
        if let Some(Some(node)) = bt.left.as_ref() {
            assert_eq!(node.borrow().value, 9);
        }
        if let Some(Some(node)) = bt.right.as_ref() {
            assert_eq!(node.borrow().value, 7);
        }
        if let Some(Some(node)) = bt.left.as_ref() {
            if let Some(Some(left_node)) = node.borrow().left.as_ref() {
                assert_eq!(left_node.borrow().value, 19);
            }
        }
    }

    #[test]
    fn inorder_traverse() {
        let mut bt = BinaryTreeNode::new(10);
        bt.insert(Some(29));
        bt.insert(Some(9));
        bt.insert(Some(7));
        bt.insert(Some(19));
        assert_eq!(bt.inorder_traverse(), vec![7, 29, 19, 10, 9]);
    }

    #[test]
    fn preorder_traverse() {
        let mut bt = BinaryTreeNode::new(10);
        bt.insert(Some(29));
        bt.insert(Some(9));
        bt.insert(Some(7));
        bt.insert(Some(19));
        assert_eq!(bt.preorder_traverse(), vec![10, 29, 7, 19, 9]);
    }

    #[test]
    fn postorder_traverse() {
        let mut bt = BinaryTreeNode::new(10);
        bt.insert(Some(29));
        bt.insert(Some(9));
        bt.insert(Some(7));
        bt.insert(Some(19));
        assert_eq!(bt.postorder_traverse(), vec![7, 19, 29, 9, 10]);
    }
}