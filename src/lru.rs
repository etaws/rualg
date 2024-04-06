use std::cell::RefCell;
use std::collections::HashMap;
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

    pub fn find(&self, val: i32) -> i32 {
        if self.len == 0 {
            return -1;
        }

        let mut cur = self.head.as_ref().unwrap().clone();
        let v = cur.borrow().val;
        if v == val {
            return 0;
        }

        let len = self.len;
        for i in 0..len {
            let p = cur.borrow().next.as_ref().unwrap().clone();
            let v = p.borrow().val;
            if v == val {
                return (i + 1) as i32;
            }

            cur = p;
        }

        -1
    }

    pub fn touch(&mut self, val: i32) {
        if self.len == 0 {
            return;
        }

        let mut cur = self.head.as_ref().unwrap().clone();
        let v = cur.borrow().val;
        if v == val {
            return;
        }

        let len = self.len;
        let mut is_touch = false;
        for _ in 0..len {
            let p = cur.borrow().next.as_ref().unwrap().clone();
            let v = p.borrow().val;
            if v == val {
                cur = p;
                is_touch = true;
                break;
            }
            cur = p;
        }

        if is_touch {
            if cur.borrow().next.is_none() {
                if let Some(old_tail) = self.tail.take() {
                    if let Some(prev) = old_tail.borrow_mut().prev.take() {
                        prev.borrow_mut().next = None;
                        self.tail = Some(prev);
                    } else {
                        self.head = None;
                    }
                    self.len -= 1;
                };
            } else {
                let prev_node = cur.borrow_mut().prev.as_ref().unwrap().clone();
                let next_node = cur.borrow_mut().next.as_ref().unwrap().clone();

                next_node.borrow_mut().prev = Some(prev_node.clone());
                prev_node.borrow_mut().next = Some(next_node);
                self.len -= 1;
            }

            self.add_at_head(val);
        }
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

    pub fn delete_at_index(&mut self, index: i32) -> i32 {
        let i: usize = index as usize;

        if i >= self.len || self.is_empty() {
            return -1;
        }

        let mut r = -1;
        if i == 0 {
            if let Some(old_head) = self.head.take() {
                r = old_head.borrow().val;
                if let Some(next) = old_head.borrow_mut().next.take() {
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                } else {
                    self.tail = None;
                }
            };
        } else if i == self.len - 1 {
            if let Some(old_tail) = self.tail.take() {
                r = old_tail.borrow().val;
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

            r = cur.borrow().val;
            let prev_node = cur.borrow_mut().prev.as_ref().unwrap().clone();
            let next_node = cur.borrow_mut().next.as_ref().unwrap().clone();

            next_node.borrow_mut().prev = Some(prev_node.clone());
            prev_node.borrow_mut().next = Some(next_node);
        }

        self.len -= 1;

        r
    }
}

pub struct LRUCache {
    pub m: HashMap<i32, i32>,
    pub lru: MyLinkedList,
    pub capacity: i32,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            m: HashMap::new(),
            lru: MyLinkedList::new(),
            capacity,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(v) = self.m.get(&key) {
            self.lru.touch(key);
            *v
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(v) = self.m.get_mut(&key) {
            *v = value;
            self.lru.touch(key);
            return;
        }

        if self.lru.len() as i32 >= self.capacity {
            let last = self.lru.len() as i32 - 1;
            let tail_key = self.lru.delete_at_index(last);
            self.m.remove(&tail_key);
        }

        self.m.insert(key, value);
        self.lru.add_at_head(key);
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

        assert_eq!(my.find(5), 0);
        assert_eq!(my.find(7), 3);
        assert_eq!(my.find(2), 1);

        my.touch(7);
        assert_eq!(my.get(0), 7);
        assert_eq!(my.get(1), 5);
        assert_eq!(my.get(2), 2);
        assert_eq!(my.get(3), 3);
        assert_eq!(my.get(4), 2);
    }

    #[test]
    fn check_lru() {
        let mut lru = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        let v_1 = lru.get(1);
        assert_eq!(v_1, 1);

        lru.put(3, 3);
        let v_2 = lru.get(2);
        assert_eq!(v_2, -1);

        lru.put(4, 4);
        assert_eq!(lru.get(1), -1);
        assert_eq!(lru.get(3), 3);
        assert_eq!(lru.get(4), 4);
    }

    #[test]
    fn check_lru_more() {
        // null,null,null,null,null,null
        let mut lru = LRUCache::new(10);
        lru.put(10, 13);
        lru.put(3, 17);
        lru.put(6, 11);
        lru.put(10, 5);
        lru.put(9, 10);
        assert_eq!(lru.lru.len(), 4);

        assert_eq!(lru.get(10), 5);

        // -1,null,19,17
        assert_eq!(lru.get(13), -1);

        lru.put(2, 19);
        assert_eq!(lru.get(2), 19);
        assert_eq!(lru.get(3), 17);

        // ,null,-1
        lru.put(5, 25);
        assert_eq!(lru.get(8), -1);

        // ,null,null,
        lru.put(9, 22);
        lru.put(5, 5);

        // null,-1
        lru.put(1, 30);
        assert_eq!(lru.get(11), -1);

        // ,null,-1
        lru.put(9, 12);
        assert_eq!(lru.get(7), -1);

        // ,5,-1,12
        assert_eq!(lru.get(5), 5);
        assert_eq!(lru.get(8), -1);
        assert_eq!(lru.get(9), 12);

        // ,null,null,3,5,5
        lru.put(4, 30);
        lru.put(9, 3);
        assert_eq!(lru.get(9), 3);
        assert_eq!(lru.get(10), 5);
        assert_eq!(lru.get(10), 5);

        //,null,null
        lru.put(6, 14);
        lru.put(3, 1);

        // 1,null,-1
        assert_eq!(lru.get(3), 1);
        lru.put(10, 11);
        assert_eq!(lru.get(8), -1);

        // ,null,30,5,30,
        lru.put(2, 14);
        assert_eq!(lru.get(1), 30);
        assert_eq!(lru.get(5), 5);
    }
}
