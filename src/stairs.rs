pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut pre: i32 = -10000;
    let mut max: i32 = nums[0];
    for n in nums.iter() {
        pre = std::cmp::max(pre + n, *n);
        max = std::cmp::max(pre, max);
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

pub fn fib(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }

    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut v: Vec<i32> = vec![0; n as usize + 1];
    v[1] = 1;

    fib_help(n - 2, &mut v) + fib_help(n - 1, &mut v)
}

pub fn fib_help(n: i32, m: &mut [i32]) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    if m[n as usize] > 0 {
        return m[n as usize];
    }

    let v = fib_help(n - 2, m) + fib_help(n - 1, m);
    m[n as usize] = v;

    v
}

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp: Vec<i32> = vec![amount + 1; amount as usize + 1];

    dp[0] = 0;
    for i in 0..dp.len() {
        for c in coins.iter() {
            if (i as i32) < *c {
                continue;
            }

            let pre: usize = i - (*c as usize);
            if dp[pre] + 1 < dp[i] {
                dp[i] = dp[pre] + 1;
            }
        }
    }

    if dp[amount as usize] <= amount {
        dp[amount as usize]
    } else {
        -1
    }
}

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let n: usize = num_rows as usize;

    let mut r: Vec<Vec<i32>> = Vec::new();

    r.push(vec![1; 1]);
    let mut count = 2;

    for i in 1..n {
        let mut dp: Vec<i32> = vec![1; count];
        let pre: &Vec<i32> = r.get(i - 1).unwrap();
        for (j, v) in dp.iter_mut().enumerate().take(count - 1).skip(1) {
            let left = pre.get(j - 1).unwrap();
            let right = pre.get(j).unwrap();
            (*v) = *left + *right;
        }
        r.push(dp);
        count += 1;
    }

    r
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let ss: Vec<char> = s.chars().collect();
    let tt: Vec<char> = t.chars().collect();

    let mut i = 0;
    let mut j = 0;
    let mut ec = 0;
    while i < s.len() && j < t.len() {
        if ss[i] != tt[j] {
            j += 1;
        } else {
            i += 1;
            j += 1;
            ec += 1;
        }
    }

    ec == s.len()
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

    #[test]
    fn check_fib() {
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
    }

    #[test]
    fn check_coin_change() {
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change(vec![1, 2, 5], 2), 1);
        assert_eq!(coin_change(vec![1, 2, 5], 0), 0);
        assert_eq!(coin_change(vec![3], 2), -1);
    }

    #[test]
    fn check_generate() {
        let v: Vec<Vec<i32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];

        assert_eq!(generate(5), v);

        assert_eq!(generate(1), vec![vec![1]]);
    }

    #[test]
    fn check_is_subsequencee() {
        assert!(is_subsequence("abc".to_string(), "ahbgdc".to_string()),);
        assert!(!is_subsequence("axc".to_string(), "ahbgdc".to_string()),);
    }
}
