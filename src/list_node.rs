pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

pub fn to_list(values: &[i32]) -> Option<Box<ListNode>> {
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

pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut cur = head.as_deref();
    while let Some(node) = cur {
        result.push(node.val);
        cur = node.next.as_deref();
    }
    result
}
