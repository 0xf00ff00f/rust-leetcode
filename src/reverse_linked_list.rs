use crate::list_node::ListNode;
use crate::solution::Solution;

impl Solution {
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        use crate::list_node::to_list;
        use crate::list_node::to_vec;

        assert_eq!(to_vec(&Solution::reverse_list(to_list(&[]))), vec![]);
        assert_eq!(
            to_vec(&Solution::reverse_list(to_list(&[1, 2, 3]))),
            vec![3, 2, 1]
        );
        assert_eq!(
            to_vec(&Solution::reverse_list(to_list(&[1, 2, 3, 4]))),
            vec![4, 3, 2, 1]
        );
    }
}
