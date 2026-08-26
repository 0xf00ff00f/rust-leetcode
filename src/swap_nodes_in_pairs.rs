use std::mem;

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

fn to_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    for &val in values {
        *tail = Some(Box::new(ListNode {
            val: val,
            next: None
        }));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut cur = head.as_deref();
    while let Some(node) = cur {
        result.push(node.val);
        cur = node.next.as_deref();
    }
    result
}

pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = &mut head as *mut Option<Box<ListNode>>;
        unsafe {
            while (*p).is_some() && (*p).as_ref().unwrap().next.is_some() {
                let q = &mut (*p).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                std::mem::swap(p.as_mut().unwrap(), q.as_mut().unwrap());
                std::mem::swap(&mut (*p).as_mut().unwrap().next, &mut (*q).as_mut().unwrap().next);
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
        assert_eq!(to_vec(Solution::swap_pairs(to_list(&[]))), vec![]);
        assert_eq!(to_vec(Solution::swap_pairs(to_list(&[1]))), vec![1]);
        assert_eq!(to_vec(Solution::swap_pairs(to_list(&[1, 2]))), vec![2, 1]);
        assert_eq!(to_vec(Solution::swap_pairs(to_list(&[1, 2, 3]))), vec![2, 1, 3]);
        assert_eq!(to_vec(Solution::swap_pairs(to_list(&[1, 2, 3, 4]))), vec![2, 1, 4, 3]);
    }
}
