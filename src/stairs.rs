pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut pre = nums[0];
    let mut max = nums[0];
    for (i, n) in nums.into_iter().enumerate() {
        if i == 0 {
            continue;
        }

        if (pre + n) > max {
            max = pre + n;
        }

        if n > max {
            max = n;
        }

        if pre >= 0 {
            pre += n;
        } else {
            pre = n;
        }
    }

    max
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut r = 0;
    let mut s = &prices[0];

    for n in prices.iter() {
        if s > n {
            s = n;
        } else if (n - s) > r {
            r = n - s;
        }
    }

    r
}

pub fn count(number: usize) -> usize {
    if number == 0 {
        return 0;
    }

    if number == 1 {
        return 1;
    }

    if number == 2 {
        return 2;
    }

    let mut p = 2;
    let mut pp = 1;

    let mut result = 0;
    for _i in 3..number + 1 {
        result = p + pp;
        pp = p;
        p = result;
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn check_strategies() {
        let mut expected = HashMap::new();
        expected.insert(0, 0);
        expected.insert(1, 1);
        expected.insert(2, 2);
        expected.insert(3, 3);
        expected.insert(4, 5);

        for (k, v) in &expected {
            assert_eq!(count(*k), *v);
        }
    }

    #[test]
    fn check_max_value() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4, 1, 2]), 5);
    }

    #[test]
    fn check_max_sub_array() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(max_sub_array(vec![-2, 1]), 1);
    }
}
