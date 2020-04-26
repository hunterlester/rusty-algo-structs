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

fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(head_node) = head {
        if let None = head_node.next {
            return Some(head_node);
        }
        let mut node_1 = head_node.clone();
        let node_2 = head_node.next;

        if let Some(mut next_node) = node_2 {
            node_1.next = swap_pairs(next_node.next);
            next_node.next = Some(node_1);
            return Some(next_node);
        } else {
            return Some(node_1);
        }
    } else {
        return head;
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, revserse_list, build_linked_list, swap_pairs};

    #[test]
    fn test_swap_pairs() {
        let l1 = build_linked_list(vec![1, 2, 3, 4]);
        let expected = build_linked_list(vec![2, 1, 4, 3]);
        assert_eq!(swap_pairs(l1), expected);
    }

    #[test]
    fn test_reverse_list() {
        let l1 = build_linked_list(vec![1, 2, 3, 4, 5]);
        let expected = build_linked_list(vec![5, 4, 3, 2, 1]);
        assert_eq!(revserse_list(l1), expected);
    }
}