use std::cell::RefCell;
use std::rc::Rc;

pub struct ListNode {
    pub val: i32,
    pub prev: Link,
    pub next: Link,
}

type Link = Option<Rc<RefCell<ListNode>>>;

impl ListNode {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode {
            val,
            next: None,
            prev: None,
        }))
    }
}

pub struct MyLinkedList {
    pub head: Link,
    pub tail: Link,
    len: usize,
}

impl Default for MyLinkedList {
    fn default() -> Self {
        MyLinkedList::new()
    }
}

impl MyLinkedList {
    pub fn new() -> Self {
        MyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn get(&self, index: i32) -> i32 {
        let i: usize = index as usize;
        if i >= self.len {
            return -1;
        }

        let mut cur = self.head.as_ref().unwrap().clone();

        for _ in 0..index {
            let p = cur.borrow().next.as_ref().unwrap().clone();
            cur = p;
        }

        let v = cur.borrow().val;

        v
    }

    pub fn add_at_head(&mut self, val: i32) {
        let new_head = ListNode::new(val);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }

        self.len += 1;
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let new_tail = ListNode::new(val);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }

        self.len += 1;
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        let i: usize = index as usize;

        if i > self.len {
            return;
        }

        if i == 0 {
            self.add_at_head(val);
        } else if i == self.len {
            self.add_at_tail(val);
        } else {
            let n = ListNode::new(val);

            let mut cur = self.head.as_ref().unwrap().clone();

            for _ in 0..index {
                let p = cur.borrow().next.as_ref().unwrap().clone();
                cur = p;
            }

            let prev_node = cur.borrow().prev.as_ref().unwrap().clone();
            prev_node.borrow_mut().next = Some(n.clone());
            n.borrow_mut().prev = Some(prev_node);
            n.borrow_mut().next = Some(cur.clone());
            cur.borrow_mut().prev = Some(n);

            self.len += 1;
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        let i: usize = index as usize;

        if i >= self.len || self.is_empty() {
            return;
        }

        if i == 0 {
            if let Some(old_head) = self.head.take() {
                if let Some(next) = old_head.borrow_mut().next.take() {
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                } else {
                    self.tail = None;
                }
            };
        } else if i == self.len - 1 {
            if let Some(old_tail) = self.tail.take() {
                if let Some(prev) = old_tail.borrow_mut().prev.take() {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                } else {
                    self.head = None;
                }
            };
        } else {
            let mut cur = self.head.as_ref().unwrap().clone();
            for _ in 0..index {
                let p = cur.borrow().next.as_ref().unwrap().clone();
                cur = p;
            }

            let prev_node = cur.borrow_mut().prev.as_ref().unwrap().clone();
            let next_node = cur.borrow_mut().next.as_ref().unwrap().clone();

            next_node.borrow_mut().prev = Some(prev_node.clone());
            prev_node.borrow_mut().next = Some(next_node);
        }

        self.len -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_list() {
        let mut my = MyLinkedList::new();
        assert_eq!(my.get(0), -1);

        my.add_at_head(32);
        assert_eq!(my.get(0), 32);
        assert_eq!(my.get(1), -1);

        my.add_at_tail(33);
        assert_eq!(my.get(0), 32);
        assert_eq!(my.get(1), 33);

        my.add_at_head(34);
        assert_eq!(my.get(0), 34);
        assert_eq!(my.get(1), 32);
        assert_eq!(my.get(2), 33);

        assert_eq!(my.len(), 3);

        my.add_at_index(2, 1);
        assert_eq!(my.len(), 4);

        assert_eq!(my.get(0), 34);
        assert_eq!(my.get(1), 32);
        assert_eq!(my.get(2), 1);
        assert_eq!(my.get(3), 33);

        my.delete_at_index(0);
        assert_eq!(my.len(), 3);
        assert_eq!(my.get(0), 32);
        assert_eq!(my.get(1), 1);
        assert_eq!(my.get(2), 33);

        my.delete_at_index(1);
        assert_eq!(my.len(), 2);
        assert_eq!(my.get(0), 32);
        assert_eq!(my.get(1), 33);
    }

    #[test]
    fn check_new_list_1() {
        let mut my = MyLinkedList::new();
        my.add_at_head(2);
        assert_eq!(my.len(), 1);
        my.delete_at_index(1);
        assert_eq!(my.len(), 1);

        my.add_at_head(2);
        my.add_at_head(7);
        my.add_at_head(3);
        my.add_at_head(2);
        my.add_at_head(5);
        assert_eq!(my.len(), 6);
        assert_eq!(my.get(0), 5);
        assert_eq!(my.get(1), 2);
        assert_eq!(my.get(2), 3);
        assert_eq!(my.get(3), 7);
        assert_eq!(my.get(4), 2);
        assert_eq!(my.get(5), 2);

        my.add_at_tail(5);
        assert_eq!(my.len(), 7);
        assert_eq!(my.get(0), 5);
        assert_eq!(my.get(6), 5);

        let v = my.get(5);
        assert_eq!(v, 2);

        my.delete_at_index(6);
        assert_eq!(my.len(), 6);
        my.delete_at_index(4);
        assert_eq!(my.len(), 5);

        assert_eq!(my.get(0), 5);
        assert_eq!(my.get(1), 2);
        assert_eq!(my.get(2), 3);
        assert_eq!(my.get(3), 7);
        assert_eq!(my.get(4), 2);
    }
}
