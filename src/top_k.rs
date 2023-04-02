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

    loop {
        if i == 0 {
            break;
        }

        adjust_on_tuple(a, i, a_len);

        i -= 1;
    }

    adjust_on_tuple(a, 0, a_len);

    let mut j = a_len - 1;
    let mut s = 0;
    loop {
        v.push(a[0].1);
        s += 1;
        if s == k as usize {
            break;
        }
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
            if diff_tuple(&a[left], &a[i]) {
                a.swap(i, left);
            }
            break;
        }

        if diff_tuple(&a[i], &a[left]) && diff_tuple(&a[i], &a[right]) {
            break;
        }

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
