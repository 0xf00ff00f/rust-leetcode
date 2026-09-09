use crate::list_node::ListNode;

pub struct Solution {}

impl Solution {
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut prev: Option<i32> = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            let next = node.next.take();
            let val = node.val;
            if prev.is_none() || prev.unwrap() != val {
                *tail = Some(node);
                prev = Some(val);
                tail = &mut tail.as_mut().unwrap().next;
            }
            cur = next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        use crate::list_node::to_list;
        use crate::list_node::to_vec;

        assert_eq!(
            to_vec(&Solution::delete_duplicates(to_list(&[1, 1, 2]))),
            vec![1, 2]
        );
        assert_eq!(
            to_vec(&Solution::delete_duplicates(to_list(&[1, 1, 2, 3, 3]))),
            vec![1, 2, 3]
        );
    }
}
