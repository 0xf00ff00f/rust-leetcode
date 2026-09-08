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
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
        assert_eq!(to_vec(Solution::reverse_list(to_list(&[]))), vec![]);
        assert_eq!(to_vec(Solution::reverse_list(to_list(&[1, 2, 3]))), vec![3, 2, 1]);
        assert_eq!(to_vec(Solution::reverse_list(to_list(&[1, 2, 3, 4]))), vec![4, 3, 2, 1]);
    }
}
