pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let offset: i32 = 10000;

    let mut a: Vec<i32> = vec![0; 20001];
    for &n in nums.iter() {
        a[(n + offset) as usize] += 1;
    }

    let mut i = 0;

    let mut s = a.len() - 1;

    loop {
        if a[s] >= 1 {
            i += a[s];
        }

        if i >= k {
            return (s as i32) - offset;
        }

        if s == 0 {
            break;
        }

        s -= 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_kth_1() {
        assert_eq!(find_kth_largest(vec![-99, -99], 1), -99);
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}
