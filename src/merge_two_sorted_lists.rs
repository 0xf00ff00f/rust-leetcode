use crate::list_node::ListNode;
use crate::solution::Solution;

impl Solution {
    fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match list1 {
            Some(mut head1) => match list2 {
                Some(mut head2) => {
                    if head1.val < head2.val {
                        head1.next = Solution::merge_two_lists(head1.next.take(), Some(head2));
                        Some(head1)
                    } else {
                        head2.next = Solution::merge_two_lists(Some(head1), head2.next.take());
                        Some(head2)
                    }
                }
                None => Some(head1),
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
