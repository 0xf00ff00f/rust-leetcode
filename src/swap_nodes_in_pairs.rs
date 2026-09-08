use crate::list_node::ListNode;
use std::mem;

pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = &mut head as *mut Option<Box<ListNode>>;
        unsafe {
            while (*p).is_some() && (*p).as_ref().unwrap().next.is_some() {
                let q = &mut (*p).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                std::mem::swap(p.as_mut().unwrap(), q.as_mut().unwrap());
                std::mem::swap(
                    &mut (*p).as_mut().unwrap().next,
                    &mut (*q).as_mut().unwrap().next,
                );
                p = q;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_pairs() {
        use crate::list_node::to_list;
        use crate::list_node::to_vec;

        assert_eq!(to_vec(&Solution::swap_pairs(to_list(&[]))), vec![]);
        assert_eq!(to_vec(&Solution::swap_pairs(to_list(&[1]))), vec![1]);
        assert_eq!(to_vec(&Solution::swap_pairs(to_list(&[1, 2]))), vec![2, 1]);
        assert_eq!(
            to_vec(&Solution::swap_pairs(to_list(&[1, 2, 3]))),
            vec![2, 1, 3]
        );
        assert_eq!(
            to_vec(&Solution::swap_pairs(to_list(&[1, 2, 3, 4]))),
            vec![2, 1, 4, 3]
        );
    }
}
