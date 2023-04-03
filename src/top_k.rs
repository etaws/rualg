use std::cmp::Ordering;
use std::collections::HashMap;

pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let mut a: Vec<&str> = Vec::with_capacity(words.len());

    for s in words.iter() {
        a.push(s);
    }

    let r = top_k_frequent_s(a, k);

    let mut v: Vec<String> = Vec::with_capacity(r.len());

    for rr in r.iter() {
        v.push(rr.to_string());
    }

    v
}

pub fn top_k_frequent_s(nums: Vec<&str>, k: i32) -> Vec<&str> {
    let mut hash = HashMap::new();

    for &n in nums.iter() {
        let count = hash.entry(n).or_insert(0);
        *count += 1;
    }

    let mut a: Vec<(i32, &str)> = Vec::with_capacity(hash.len());
    for (key, val) in hash.iter() {
        a.push((*val, key));
    }

    sort_on_tuple(&mut a, k)
}

pub fn sort_on_tuple<'a>(a: &mut Vec<(i32, &'a str)>, k: i32) -> Vec<&'a str> {
    let a_len = a.len();

    let mut v = Vec::with_capacity(a_len);

    if a_len <= 1 {
        v.push(a[0].1);
        return v;
    }

    let mut i = a_len / 2 - 1;

    // 先把整个数组作为一个「完全二叉树」调整成「最大堆」
    loop {
        adjust_on_tuple(a, i, a_len);
        if i == 0 {
            break;
        }

        i -= 1;
    }

    let mut j = a_len - 1;
    let mut s = 0;
    loop {
        // 拿走「最大堆」顶端最大的元素
        v.push(a[0].1);
        s += 1;
        if s == k as usize {
            break;
        }

        // 把数组最后一个元素拿上来，重新做调整（j 每次减一）
        a.swap(0, j);

        adjust_on_tuple(a, 0, j);

        j -= 1;
    }

    v
}

pub fn diff_tuple(t: &(i32, &str), s: &(i32, &str)) -> bool {
    if t.0 > s.0 {
        return true;
    }

    if t.0 < s.0 {
        return false;
    }

    match t.1.cmp(s.1) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => true,
    }
}

/// 把一个数组看成是一个「完全二叉树」；而且这个「完全二叉树」的左右子树都已经是「最大堆」了。
/// 如果需要把整颗「完全二叉树」调整成为「最大堆」，需要从 a[root] 这个根节点元素开始进行调整。
///
/// # Arguments
///
/// * root - 「完全二叉树」根节点的数组下标（>= 0）
/// * len  - 「完全二叉树」的 size（注意：并不是数组的大小，这个「完全二叉树」可能只是数组的一部分）
pub fn adjust_on_tuple(a: &mut [(i32, &str)], root: usize, len: usize) {
    if len <= 1 {
        return;
    }

    if root >= len {
        return;
    }

    let mut i = root;

    while i < len {
        let left = (i + 1) * 2 - 1;
        let right = (i + 1) * 2 + 1 - 1;

        if left >= len {
            break;
        }

        if right >= len {
            // a[left] >= a[i]
            if diff_tuple(&a[left], &a[i]) {
                a.swap(i, left);
            }
            break;
        }

        // a[i] >= a[left] && a[i] > a[right]
        // a[i] 已经是最大的节点了，结束
        if diff_tuple(&a[i], &a[left]) && diff_tuple(&a[i], &a[right]) {
            break;
        }

        // a[left] > a[right] 的话 a[left] 应该调整上去，否则应该把 a[right] 调整上去
        if diff_tuple(&a[left], &a[right]) {
            a.swap(i, left);
            i = left;
        } else {
            a.swap(i, right);
            i = right;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_top_k_0() {
        let a = vec![
            "the".to_string(),
            "day".to_string(),
            "is".to_string(),
            "sunny".to_string(),
            "the".to_string(),
            "the".to_string(),
            "the".to_string(),
            "sunny".to_string(),
            "is".to_string(),
            "is".to_string(),
        ];
        let b = vec![
            "the".to_string(),
            "is".to_string(),
            "sunny".to_string(),
            "day".to_string(),
        ];

        let c = top_k_frequent(a, 4);

        assert_eq!(c, b);
    }

    #[test]
    fn check_top_k_1() {
        let a = vec!["i", "love", "leetcode", "i", "love", "coding"];
        let b = vec!["i", "love", "coding"];

        let c = top_k_frequent_s(a, 3);

        assert_eq!(c, b);
    }
}
