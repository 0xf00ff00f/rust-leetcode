use crate::list_node::ListNode;

struct Solution {}

impl Solution {
    fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left: Option<Box<ListNode>> = None;
        let mut right: Option<Box<ListNode>> = None;
        let mut tail_left = &mut left;
        let mut tail_right = &mut right;
        let mut cur = head;
        while let Some(mut node) = cur {
            let next = node.next.take();
            if node.val < x {
                *tail_left = Some(node);
                tail_left = &mut tail_left.as_mut().unwrap().next;
            } else {
                *tail_right = Some(node);
                tail_right = &mut tail_right.as_mut().unwrap().next;
            }
            cur = next;
        }
        *tail_left = right;
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        use crate::list_node::to_list;
        use crate::list_node::to_vec;

        assert_eq!(
            to_vec(&Solution::partition(to_list(&[1, 4, 3, 2, 5, 2]), 3)),
            vec![1, 2, 2, 4, 3, 5]
        );
    }
}
