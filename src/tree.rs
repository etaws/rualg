use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}

pub fn inorder_traverse_i<F: FnMut(i32)>(root: Option<&Rc<RefCell<TreeNode>>>, consumer: &mut F) {
    if let Some(node) = root {
        inorder_traverse_i(node.borrow().left.as_ref(), consumer);
        consumer(root.as_ref().unwrap().borrow().val);
        inorder_traverse_i(node.borrow().right.as_ref(), consumer);
    }
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut r: Vec<i32> = Vec::new();

    inorder_traverse_i(root.as_ref(), &mut (|v| r.push(v)));

    r
}

pub fn inorder_traversal_p(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut re: Vec<i32> = Vec::new();
    if root.is_none() {
        return re;
    }

    let mut stack = Vec::new();

    let mut r = root;

    while r.is_some() || !stack.is_empty() {
        while let Some(n) = r {
            stack.push(n.clone());
            r = n.borrow().left.clone();
        }

        r = stack.pop();

        if let Some(n) = r {
            re.push(n.borrow().val);
            r = n.borrow().right.clone();
        }
    }

    re
}

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut re: Vec<i32> = Vec::new();
    if root.is_none() {
        return re;
    }

    let mut stack = Vec::new();

    let mut r = root;

    while r.is_some() || !stack.is_empty() {
        while let Some(n) = r {
            re.push(n.borrow().val);
            stack.push(n.clone());
            r = n.borrow().left.clone();
        }

        r = stack.pop();

        if let Some(n) = r {
            r = n.borrow().right.clone();
        }
    }

    re
}

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut re: Vec<i32> = Vec::new();
    if root.is_none() {
        return re;
    }

    let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let mut result: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

    stack.push(root);
    while let Some(Some(n)) = stack.pop() {
        if n.borrow().left.is_some() {
            stack.push(n.borrow().left.clone());
        }
        if n.borrow().right.is_some() {
            stack.push(n.borrow().right.clone());
        }

        result.push(Some(n));
    }

    while let Some(Some(node)) = result.pop() {
        re.push(node.borrow().val);
    }

    re
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut re: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    re.push_front(root);

    let mut current_size = 1;
    let mut r = 0;

    while !re.is_empty() {
        let mut next_size = 0;
        for _i in 0..current_size {
            if let Some(Some(n)) = re.pop_back() {
                if n.borrow().left.is_some() {
                    re.push_front(n.borrow().left.clone());
                    next_size += 1;
                }
                if n.borrow().right.is_some() {
                    re.push_front(n.borrow().right.clone());
                    next_size += 1;
                }
            }
        }

        current_size = next_size;

        r += 1;
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inorder_traversal() {
        assert_eq!(inorder_traversal_p(tree![1, null, 2, 3]), vec![1, 3, 2]);
        assert_eq!(
            inorder_traversal_p(tree![1, 2, 3, 4, 5, 6, 7]),
            vec![4, 2, 5, 1, 6, 3, 7]
        );
    }

    #[test]
    fn test_preorder_traversal() {
        assert_eq!(preorder_traversal(tree![1, null, 2, 3]), vec![1, 2, 3]);
        assert_eq!(
            preorder_traversal(tree![1, 2, 3, 4, 5, 6, 7]),
            vec![1, 2, 4, 5, 3, 6, 7]
        );
    }

    #[test]
    fn test_postorder_traversal() {
        assert_eq!(postorder_traversal(tree![1, null, 2, 3]), vec![3, 2, 1]);
        assert_eq!(
            postorder_traversal(tree![1, 2, 3, 4, 5, 6, 7]),
            vec![4, 5, 2, 6, 7, 3, 1]
        );
    }

    #[test]
    fn test_max_depth() {
        assert_eq!(max_depth(tree![3, 9, 20, null, null, 15, 7]), 3);
    }
}
