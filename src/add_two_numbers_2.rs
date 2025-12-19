#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn new_from_vec(vec: Vec<i32>) -> Box<Self> {
        let mut node = ListNode::new(vec[vec.len() - 1]);
        for num in vec[0..vec.len() - 1].iter().rev() {
            let mut new_node = ListNode::new(*num);
            new_node.next = Some(Box::new(node));
            node = new_node;
        }
        Box::new(node)
    }
}

#[allow(dead_code)]
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut node_1 = l1;
    let mut node_2 = l2;
    let mut carry = 0;
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;
    while node_1.is_some() || node_2.is_some() || carry == 1 {
        let mut val = carry;
        if let Some(ref node_1) = node_1 {
            val += node_1.val;
        }
        if let Some(ref node_2) = node_2 {
            val += node_2.val;
        }
        carry = val / 10;
        val = val % 10;

        current.next = Some(Box::new(ListNode::new(val)));

        current = current.next.as_mut().unwrap();

        node_1 = node_1.and_then(|it| it.next);
        node_2 = node_2.and_then(|it| it.next);
    }
    dummy_head.next
}

#[test]
fn test_add_two_numbers() {
    assert_eq!(
        add_two_numbers(Some(Box::new(ListNode::new(2))), None),
        Some(Box::new(ListNode::new(2)))
    );
    assert_eq!(
        add_two_numbers(
            Some(ListNode::new_from_vec(vec![2, 3, 3])),
            Some(ListNode::new_from_vec(vec![5, 6, 3]))
        ),
        Some(ListNode::new_from_vec(vec![7, 9, 6])),
    );
    assert_eq!(
        add_two_numbers(
            Some(ListNode::new_from_vec(vec![2, 4, 3])),
            Some(ListNode::new_from_vec(vec![5, 6, 4]))
        ),
        Some(ListNode::new_from_vec(vec![7, 0, 8])),
    );
    assert_eq!(
        add_two_numbers(
            Some(ListNode::new_from_vec(vec![9, 9, 9, 9, 9, 9, 9])),
            Some(ListNode::new_from_vec(vec![9, 9, 9, 9]))
        ),
        Some(ListNode::new_from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1])),
    )
}
