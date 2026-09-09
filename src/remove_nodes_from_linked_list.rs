use crate::list_node::ListNode;
use crate::solution::Solution;

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                let tail = Solution::remove_nodes(node.next.take());
                match tail.as_deref() {
                    Some(next_node) => {
                        if next_node.val <= node.val {
                            node.next = tail;
                            Some(node)
                        } else {
                            tail
                        }
                    }
                    None => Some(node),
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
    fn test_remove_nodes() {
        use crate::list_node::to_list;
        use crate::list_node::to_vec;

        assert_eq!(
            to_vec(&Solution::remove_nodes(to_list(&[5, 2, 13, 3, 8]))),
            vec![13, 8]
        );
    }
}
