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

struct Solution {}

impl Solution {
    fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left: Option<Box<ListNode>> = None;
        let mut right: Option<Box<ListNode>> = None;
        let mut tail_left = &mut left;
        let mut tail_right = &mut right;
        let mut cur = head.as_deref();
        while let Some(node) = cur {
            if node.val < x {
                *tail_left = Some(Box::new(ListNode {
                    val: node.val,
                    next: None
                }));
                tail_left = &mut tail_left.as_mut().unwrap().next;
            } else {
                *tail_right = Some(Box::new(ListNode {
                    val: node.val,
                    next: None
                }));
                tail_right = &mut tail_right.as_mut().unwrap().next;
            }
            cur = node.next.as_deref();
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
        assert_eq!(to_vec(Solution::partition(to_list(&[1, 4, 3, 2, 5, 2]), 3)), vec![1, 2, 2, 4, 3, 5]);
    }
}
