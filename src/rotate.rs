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
