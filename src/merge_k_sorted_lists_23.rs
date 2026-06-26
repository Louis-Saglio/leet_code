/**
You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.

Merge all the linked-lists into one sorted linked-list and return it.



Example 1:

Input: lists = [[1,4,5],[1,3,4],[2,6]]
Output: [1,1,2,3,4,4,5,6]
Explanation: The linked-lists are:
[
  1->4->5,
  1->3->4,
  2->6
]
merging them into one sorted linked list:
1->1->2->3->4->4->5->6

Example 2:

Input: lists = []
Output: []

Example 3:

Input: lists = [[]]
Output: []



Constraints:

    k == lists.length
    0 <= k <= 10^4
    0 <= lists[i].length <= 500
    -10^4 <= lists[i][j] <= 10^4
    lists[i] is sorted in ascending order.
    The sum of lists[i].length will not exceed 10^4.

**/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut root_node = ListNode::new(0);
        let mut ptr = &mut root_node;
        let mut lists = lists;
        let mut min_node_ptr: Option<&Box<ListNode>> = None;
        let mut min = 10_i32.pow(4);
        let mut index = 0;
        loop {
            for i in 0..lists.len() {
                match &lists[i] {
                    None => {}
                    Some(node) => {
                        if node.val < min {
                            min = node.val;
                            min_node_ptr = Some(node);
                            index = i;
                        }
                    }
                }
            }
            ptr.next = min_node_ptr.map(|node| node.clone());
            ptr = ptr.next.as_mut().unwrap();
            lists[index] = Some(min_node_ptr.unwrap().clone());
        }
        root_node.next
    }
}