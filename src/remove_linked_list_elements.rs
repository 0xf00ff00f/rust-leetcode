use crate::list_node::ListNode;
use crate::solution::Solution;

impl Solution {
    fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                if node.val == val {
                    Solution::remove_elements(node.next.take(), val)
                } else {
                    node.next = Solution::remove_elements(node.next.take(), val);
                    Some(node)
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_elements() {
        use crate::list_node::to_list;
        use crate::list_node::to_vec;

        assert_eq!(
            to_vec(&Solution::remove_elements(
                to_list(&[1, 2, 6, 3, 4, 5, 6]),
                6
            )),
            vec![1, 2, 3, 4, 5]
        );
    }
}
