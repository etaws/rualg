use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut hash = HashMap::new();

    for &n in nums.iter() {
        let count = hash.entry(n).or_insert(0);
        *count += 1;
    }

    let mut a: Vec<(i32, i32)> = vec![(0, 0); hash.len()];
    for (j, (key, val)) in hash.iter().enumerate() {
        a[j] = (*val, *key);
    }

    sort_on_tuple(&mut a, k)
}

pub fn adjust_on_tuple(a: &mut Vec<(i32, i32)>, root: usize, len: usize) {
    if a.len() <= 1 {
        return;
    }

    if root >= a.len() {
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
            if a[left].0 >= a[i].0 {
                a.swap(i, left);
            }
            break;
        }

        if (a[i].0 >= a[left].0) && (a[i].0 >= a[right].0) {
            break;
        }

        if a[left].0 >= a[right].0 {
            a.swap(i, left);
            i = left;
        } else {
            a.swap(i, right);
            i = right;
        }
    }
}

pub fn sort_on_tuple(a: &mut Vec<(i32, i32)>, k: i32) -> Vec<i32> {
    let mut v = vec![0; k as usize];

    if a.len() <= 1 {
        v[0] = a[0].1;
        return v;
    }

    let mut i = a.len() / 2 - 1;

    loop {
        if i == 0 {
            break;
        }

        adjust_on_tuple(a, i, a.len());

        i -= 1;
    }

    adjust_on_tuple(a, 0, a.len());

    let mut j = a.len() - 1;
    let mut s = 0;
    loop {
        v[s] = a[0].1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_top_k() {
        let a = vec![1, 1, 1, 2, 2, -1, -1, -1, -1];
        let b = vec![-1, 1, 2];

        let c = top_k_frequent(a, 3);

        assert_eq!(c, b);
    }
}
