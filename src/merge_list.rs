use std::collections::LinkedList;

// Definition for singly-linked list.
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
}

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut output = None;

    let mut next_node_pos = &mut output;
    let mut l1_opt = l1;
    let mut l2_opt = l2;
    loop {
        let mut l1 = match l1_opt {
            Some(l1) => l1,
            None => {
                *next_node_pos = l2_opt;
                break;
            }
        };
        let mut l2 = match l2_opt {
            Some(l2) => l2,
            None => {
                *next_node_pos = Some(l1);
                break;
            }
        };

        if l1.val < l2.val {
            l1_opt = l1.next.take();
            l2_opt = Some(l2);
            *next_node_pos = Some(l1);
        } else {
            l2_opt = l2.next.take();
            l1_opt = Some(l1);
            *next_node_pos = Some(l2);
        }

        next_node_pos = &mut next_node_pos.as_mut().unwrap().next;
    }

    output
}

pub fn linkedlist_to_list(mut ll: LinkedList<i32>) -> Option<Box<ListNode>> {
    let mut tail = None;

    while ll.front().is_some() {
        let v = *ll.front().unwrap();
        let node = ListNode { val: v, next: tail };
        tail = Some(Box::new(node));
        ll.pop_front();
    }

    tail
}

pub fn list_to_linkedlist(l: Option<Box<ListNode>>) -> LinkedList<i32> {
    let mut result = LinkedList::new();
    let mut curr = l;

    while curr != None {
        let inner = curr.unwrap();
        result.push_back(inner.val);
        curr = inner.next;
    }

    result
}

pub fn duplicated_list(l1: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut output = None;

    let mut next_node_pos = &mut output;
    let mut l1_opt = l1;

    let mut i: usize = 0;
    while let Some(mut l1) = l1_opt {
        // 指针移动到下个节点
        l1_opt = l1.next.take();

        if i == 0 {
            *next_node_pos = Some(l1);
        } else if l1.val != next_node_pos.as_ref().unwrap().val {
            next_node_pos = &mut next_node_pos.as_mut().unwrap().next;
            *next_node_pos = Some(l1);
        }

        i += 1;
    }

    output
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_create_list() {
        let link = list_to_linkedlist(merge_two_lists(
            to_list(vec![1, 2, 4]),
            to_list(vec![1, 3, 4]),
        ));
        let expect_link: LinkedList<i32> = LinkedList::from([1, 1, 2, 3, 4, 4]);
        assert_eq!(link, expect_link);

        assert_eq!(
            merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }

    #[test]
    fn check_duplicated_list() {
        // duplicated_list(to_list(vec![1, 1, 2, 3, 4, 4]));
        assert_eq!(
            duplicated_list(to_list(vec![1, 1, 2, 3, 4, 4])),
            to_list(vec![1, 2, 3, 4])
        );
    }
}
