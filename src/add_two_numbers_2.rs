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
    let mut sum = vec![];
    let mut carry = 0;
    loop {
        match (&node_1, &node_2) {
            (None, None) => {
                if carry == 1 {
                    sum.push(1);
                }
                return Some(ListNode::new_from_vec(sum));
            }
            (node_1, node_2) => {
                let mut val = carry
                    + node_1.as_ref().map(|it| it.val).unwrap_or(0)
                    + node_2.as_ref().map(|it| it.val).unwrap_or(0);
                carry = val / 10;
                val %= 10;
                sum.push(val);
            }
        }
        node_1 = node_1.and_then(|it| it.next);
        node_2 = node_2.and_then(|it| it.next);
    }
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
