pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    if n == 0 {
        return;
    }

    let mut i: i32 = m - 1;
    let mut j: i32 = n - 1;
    let mut k: i32 = m + n - 1;

    while (i >= 0) && (j >= 0) {
        if nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            k -= 1;
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            k -= 1;
            j -= 1;
        }
    }

    while j >= 0 {
        nums1[k as usize] = nums2[j as usize];
        k -= 1;
        j -= 1;
    }
}

pub fn merge_two(a: &mut [i32], a_len: usize, b: &[i32]) {
    assert!(a.len() >= (a_len + b.len()));

    if b.is_empty() {
        return;
    }

    if a_len == 0 {
        for (j, n) in b.iter().enumerate() {
            a[j] = *n;
        }
        return;
    }

    let mut i = a_len - 1;
    let mut j = b.len() - 1;

    let mut current = a_len + b.len() - 1;

    let mut a_end = false;
    let mut b_end = false;

    loop {
        if b_end || (a[i] > b[j]) {
            a[current] = a[i];
            if i == 0 {
                a_end = true;
            }
            if i > 0 {
                i = i.saturating_sub(1);
            }
        } else if a_end || (a[i] <= b[j]) {
            a[current] = b[j];
            if j == 0 {
                b_end = true;
            }
            if j > 0 {
                j = j.saturating_sub(1);
            }
        }

        if current == 0 {
            break;
        } else {
            current -= 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_merge_1() {
        let mut a = vec![1, 2, 3, 0, 0, 0];
        let mut b = vec![2, 5, 6];
        merge(&mut a, 3, &mut b, 3);

        let c = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(a, c);
    }

    #[test]
    fn check_merge_2() {
        let mut a = vec![1];
        let mut b = vec![];
        merge(&mut a, 1, &mut b, 0);

        let c = vec![1];
        assert_eq!(a, c);
    }

    #[test]
    fn check_merge_3() {
        let mut a = vec![0];
        let mut b = vec![1];
        merge(&mut a, 0, &mut b, 1);

        let c = vec![1];
        assert_eq!(a, c);
    }

    #[test]
    fn check_merge_4() {
        let mut a = vec![2, 5, 6, 0, 0, 0];
        let mut b = vec![1, 2, 3];

        merge(&mut a, 3, &mut b, 3);

        let c = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(a, c);
    }

    #[test]
    fn check_merge_5() {
        let mut a = vec![1, 0];
        let mut b = vec![2];

        merge(&mut a, 1, &mut b, 1);

        let c = vec![1, 2];
        assert_eq!(a, c);
    }
}
