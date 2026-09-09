use crate::list_node::ListNode;
use crate::solution::Solution;

impl Solution {
    fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match list1 {
            Some(mut node1) => match list2 {
                Some(mut node2) => {
                    if node1.val < node2.val {
                        node1.next = Solution::merge_two_lists(node1.next.take(), Some(node2));
                        Some(node1)
                    } else {
                        node2.next = Solution::merge_two_lists(Some(node1), node2.next.take());
                        Some(node2)
                    }
                }
                None => Some(node1),
            },
            None => list2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        use crate::list_node::to_list;
        use crate::list_node::to_vec;

        assert_eq!(
            to_vec(&Solution::merge_two_lists(to_list(&[]), to_list(&[]))),
            vec![]
        );
        assert_eq!(
            to_vec(&Solution::merge_two_lists(to_list(&[]), to_list(&[0]))),
            vec![0]
        );
        assert_eq!(
            to_vec(&Solution::merge_two_lists(
                to_list(&[1, 2, 4]),
                to_list(&[1, 3, 4])
            )),
            vec![1, 1, 2, 3, 4, 4]
        );
        assert_eq!(
            to_vec(&Solution::merge_two_lists(
                to_list(&[5]),
                to_list(&[1, 2, 4])
            )),
            vec![1, 2, 4, 5]
        );
    }
}
