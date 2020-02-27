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
        if let Some(outer_option_value) = self.left.as_ref() {
            if let Some(node_value) = outer_option_value {
                match self.right.as_ref() {
                    Some(Some(_v)) => {
                      node_value.borrow_mut().insert(value);
                    },
                    Some(None) => {
                      self.insert_right(value)
                    },
                    None => {
                      node_value.borrow_mut().insert(value);
                    },
                }
            } else {
                if let Some(integer) = value {
                  self.left = Some(Some(Rc::new(RefCell::new(BinaryTreeNode::new(integer)))));
                } else {
                  self.left = None;
                }
            }
        } else {
            self.insert_right(value);
        }
    }

    fn insert_right(&mut self, value: Option<i32>) -> () {
          if let Some(outer_option_value) = self.right.as_ref() {
              if let Some(node_value) = outer_option_value {
                  node_value.borrow_mut().insert(value);
              } else {
                  if let Some(integer) = value {
                    self.right = Some(Some(Rc::new(RefCell::new(BinaryTreeNode::new(integer)))));
                  } else {
                    self.right = None;
                  }
              }
          } else {
            self.insert_left(value);
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
}