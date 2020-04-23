/// Expected struct on leetcode
#[derive(Debug, Eq, PartialEq, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}


fn build_linked_list(mut array: Vec<i32>) -> Option<Box<ListNode>> {
    // array.reverse();
    if array.len() > 0 {
        let val = array.remove(0);
        Some(Box::new(ListNode {
            val,
            next: build_linked_list(array),
        }))
    } else {
        None
    }
}

// leetcode
fn revserse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut previous_node: Option<Box<ListNode>> = None;
    let mut next_node: Option<Box<ListNode>> = head;

    while let Some(mut boxed_node) = next_node {
        next_node = boxed_node.next;
        boxed_node.next = previous_node;
        previous_node = Some(boxed_node);
    }

    previous_node
}

#[cfg(test)]
mod tests {
    use super::{ListNode, revserse_list, build_linked_list};

    #[test]
    fn test_reverse_list() {
        let l1 = build_linked_list(vec![1, 2, 3, 4, 5]);
        let expected = build_linked_list(vec![5, 4, 3, 2, 1]);
        assert_eq!(revserse_list(l1), expected);
    }
}