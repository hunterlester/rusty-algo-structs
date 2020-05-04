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
    pub fn new(value: i32) -> PotentialNode {
        Some(Rc::new(RefCell::new(IterativeBinarySearchTree {
            value,
            left: None,
            right: None,
            count: 1,
        })))
    }

    pub fn insert(root: Rc<RefCell<IterativeBinarySearchTree>>, value: i32) {
        let mut current_node = Some(Rc::clone(&root));
        while let Some(node) = current_node {
            let mut borrowed_node = node.borrow_mut();
            if value < borrowed_node.value {
                if let Some(left_child) = borrowed_node.left.as_ref() {
                    current_node = Some(Rc::clone(&left_child));
                } else {
                    borrowed_node.left = IterativeBinarySearchTree::new(value);
                    current_node = None;
                }
            } else {
                if let Some(right_child) = borrowed_node.right.as_ref() {
                    current_node = Some(Rc::clone(&right_child));
                } else {
                    borrowed_node.right = IterativeBinarySearchTree::new(value);
                    current_node = None;
                }
            }
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
    use std::rc::Rc;

    #[test]
    fn inorder() {
        let mut bst = IterativeBinarySearchTree::new(10);
        let values = vec![29, 9, 7, 19];
        if let Some(tree) = bst.as_ref() {
            for num in values {
                IterativeBinarySearchTree::insert(Rc::clone(&tree), num);
            }
            assert_eq!(tree.borrow().inorder(), vec![7, 9, 10, 19, 29]);
        }
    }

    #[test]
    fn preorder() {
        let mut bst = IterativeBinarySearchTree::new(10);
        let values = vec![29, 9, 7, 19];
        if let Some(tree) = bst {
            for num in values {
                IterativeBinarySearchTree::insert(Rc::clone(&tree), num);
            }
            assert_eq!(tree.borrow().preorder(), vec![10, 9, 7, 29, 19]);
        }
    }

    #[test]
    fn postorder() {
        let mut bst = IterativeBinarySearchTree::new(10);
        let values = vec![29, 9, 7, 19, 5, 8, 1];
        if let Some(tree) = bst {
            for num in values {
                IterativeBinarySearchTree::insert(Rc::clone(&tree), num);
            }
            assert_eq!(tree.borrow().postorder(), vec![1, 5, 8, 7, 9, 19, 29, 10]);
        }
    }

    // #[test]
    // fn level_order() {
    //     let mut bst = IterativeBinarySearchTree::new(10);
    //     let values = vec![29, 9, 7, 19, 5, 8, 1];
    //     if let Some(tree) = bst {
    //         for num in values {
    //             IterativeBinarySearchTree::insert(Rc::clone(&tree), num);
    //         }
    //         assert_eq!(tree.borrow().bfs(), vec![10, 9 , 29, 7, 19, 5, 8, 1]);
    //     }
    // }

    #[test]
    fn bst_insert_iteratively() {
        let mut bst = IterativeBinarySearchTree::new(10);
        let values = vec![9, 7, 19];
        if let Some(tree) = bst {
            for num in values {
                IterativeBinarySearchTree::insert(Rc::clone(&tree), num);
            }
            assert_eq!(tree.borrow().preorder(), vec![10, 9, 7, 19]);
        }
    }
}