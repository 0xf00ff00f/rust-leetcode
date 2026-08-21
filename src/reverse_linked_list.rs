struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

pub struct Solution {}

impl Solution {
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut cur = head.as_deref();
        while let Some(node) = cur {
            result = Some(Box::new(ListNode {
                val: node.val,
                next: result
            }));
            cur = node.next.as_deref();
        }
        result
    }
}
