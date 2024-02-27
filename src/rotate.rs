pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    let kk = (k as usize) % n;

    if kk == 0 {
        return;
    }

    reverse(nums, 0, n - 1);
    reverse(nums, 0, kk - 1);
    reverse(nums, kk, n - 1);
}

pub fn reverse(nums: &mut Vec<i32>, start: usize, end: usize) {
    if nums.is_empty() {
        return;
    }

    if nums.len() == 1 {
        return;
    }

    let mut s = start;
    let mut e = end;
    while s < e {
        nums.swap(s, e);
        if s == nums.len() - 1 {
            break;
        }
        if e == 0 {
            break;
        }
        s += 1;
        e -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_rotate() {
        let mut a = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut a, 3);
        assert_eq!(a, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn check_rotate_0() {
        let mut a = vec![1, 2];
        rotate(&mut a, 0);
        assert_eq!(a, vec![1, 2]);
    }

    #[test]
    fn check_rotate_2() {
        let mut a = vec![1, 2];
        rotate(&mut a, 2);
        assert_eq!(a, vec![1, 2]);
    }

    #[test]
    fn check_reverse() {
        diff_reverse(&mut vec![1, 2, 3, 4, 5, 6, 7], &vec![7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn check_reverse_single() {
        diff_reverse(&mut vec![1], &vec![1]);
    }

    #[test]
    fn check_reverse_double() {
        diff_reverse(&mut vec![1, 2], &vec![2, 1]);
    }

    fn diff_reverse(a: &mut Vec<i32>, expected: &Vec<i32>) {
        let len = a.len();
        reverse(a, 0, len - 1);
        assert_eq!(a, expected);
    }
}
