pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    if n == 0 {
        return;
    }

    if m == 0 {
        for (j, n) in nums2.iter().enumerate() {
            nums1[j] = *n;
        }
        return;
    }

    let mut i: usize = (m - 1) as usize;
    let mut j: usize = (n - 1) as usize;

    let mut k: usize = (m + n - 1) as usize;

    let mut end_1 = false;
    loop {
        if nums1[i] > nums2[j] {
            nums1[k] = nums1[i];
            if i == 0 {
                end_1 = true;
                break;
            }
            i -= 1;
        } else {
            nums1[k] = nums2[j];
            if j == 0 {
                break;
            }
            j -= 1;
        }

        if k == 0 {
            break;
        }
        k -= 1;
    }

    if k != 0 && end_1 {
        let mut s = 0;
        while s < k {
            nums1[s] = nums2[s];
            s += 1;
        }
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
                i -= 1;
            }
        } else if a_end || (a[i] <= b[j]) {
            a[current] = b[j];
            if j == 0 {
                b_end = true;
            }
            if j > 0 {
                j -= 1;
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
        let mut a: [i32; 6] = [1, 2, 3, 0, 0, 0];
        let b: [i32; 3] = [2, 5, 6];
        merge_two(&mut a, 3, &b);

        let c: [i32; 6] = [1, 2, 2, 3, 5, 6];
        assert_eq!(a, c);
    }

    #[test]
    fn check_merge_2() {
        let mut a: [i32; 1] = [1];
        let b: [i32; 0] = [];
        merge_two(&mut a, 1, &b);

        let c: [i32; 1] = [1];
        assert_eq!(a, c);
    }

    #[test]
    fn check_merge_3() {
        let mut a: [i32; 1] = [0];
        let b: [i32; 1] = [1];
        merge_two(&mut a, 0, &b);

        let c: [i32; 1] = [1];
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
