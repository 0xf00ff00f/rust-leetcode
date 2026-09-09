use crate::list_node::ListNode;
use crate::solution::Solution;

impl Solution {
    fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut left: Option<Box<ListNode>> = None;
        let mut right: Option<Box<ListNode>> = None;
        let mut tail_left = &mut left;
        let mut tail_right = &mut right;
        let mut cur = head;
        let mut index = 1;
        while let Some(mut node) = cur {
            let next = node.next.take();
            if index % 2 != 0 {
                *tail_left = Some(node);
                tail_left = &mut tail_left.as_mut().unwrap().next;
            } else {
                *tail_right = Some(node);
                tail_right = &mut tail_right.as_mut().unwrap().next;
            }
            cur = next;
            index += 1;
        }
        *tail_left = right;
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_even_list() {
        use crate::list_node::to_list;
        use crate::list_node::to_vec;

        assert_eq!(
            to_vec(&Solution::odd_even_list(to_list(&[1, 2, 3, 4, 5]))),
            vec![1, 3, 5, 2, 4]
        );
        assert_eq!(
            to_vec(&Solution::odd_even_list(to_list(&[2, 1, 3, 5, 6, 4, 7]))),
            vec![2, 3, 6, 7, 1, 5, 4]
        );
    }
}
