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

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    if cost.is_empty() {
        return 0;
    }
    let mut m: Vec<i32> = vec![0; cost.len() + 1];

    for n in 2..cost.len() + 1 {
        let p = m[n - 1] + cost[n - 1];
        let pp = m[n - 2] + cost[n - 2];

        if p < pp {
            m[n] = p;
        } else {
            m[n] = pp;
        }
    }

    m[cost.len()]
}

pub fn max_repeating(sequence: String, word: String) -> i32 {
    if word.len() > sequence.len() {
        return 0;
    }

    let ss: Vec<char> = sequence.chars().collect();
    let tt: Vec<char> = word.chars().collect();

    let mut dp: Vec<i32> = vec![0; ss.len() + 101];

    let mut r = 0;
    for i in 0..ss.len() {
        if ss.len() + 1 - i <= tt.len() {
            break;
        }

        if ss_match(&ss, i, &tt, tt.len()) {
            if i >= tt.len() {
                dp[i] = dp[i - tt.len()] + 1;
            } else {
                dp[i] = 1;
            }

            if r < dp[i] {
                r = dp[i];
            }
        }
    }

    r
}

pub fn ss_match(s: &[char], start: usize, t: &[char], len: usize) -> bool {
    let mut i = start;
    let mut j = 0;

    while j < len {
        if s[i] == t[j] {
            i += 1;
            j += 1;
        } else {
            return false;
        }
    }

    true
}

pub fn divisor_game(n: i32) -> bool {
    let i: usize = n as usize;
    let mut dp: Vec<bool> = vec![false; i + 1];
    dp[1] = false;

    for j in 2..i + 1 {
        let mut j_true = false;
        for k in 1..j {
            if j % k == 0 {
                let new_n = j - k;
                if !dp[new_n] {
                    j_true = true;
                    break;
                }
            }
        }
        dp[j] = j_true;
    }

    dp[i]
}

pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    if n == 2 {
        return 1;
    }

    let mut p = 0;
    let mut pp = 1;

    let mut ppp = 1;

    for _ in 3..n + 1 {
        let t = ppp + pp + p;

        p = pp;
        pp = ppp;
        ppp = t;
    }

    ppp
}

pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    if s.len() == 1 {
        return s;
    }

    let ss: Vec<char> = s.chars().collect();
    if s.len() == 2 {
        if ss[0] == ss[1] {
            return s;
        } else {
            return ss[0].to_string();
        }
    }

    let len = s.len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; len]; len];

    let mut r_s = 0;
    let mut r_e = 1;
    for (i, _) in (0..len).enumerate() {
        dp[i][i] = 1;
    }

    for i in 0..len - 1 {
        if ss[i] == ss[i + 1] {
            dp[i][i + 1] = 1;
            if r_e - r_s == 1 {
                r_s = i;
                r_e = i + 2;
            }
        }
    }

    for i in 2..len {
        for j in 0..len - 1 {
            let start = j;
            let end = j + i;
            if end >= len {
                break;
            }
            if start >= end {
                break;
            }
            if dp[start][end] == 1 {
                continue;
            }
            if dp[start + 1][end - 1] == 1 && ss[end] == ss[start] {
                dp[start][end] = 1;
                if r_e - r_s < end + 1 - start {
                    r_s = start;
                    r_e = end + 1;
                }
            }
        }
    }

    ss[r_s..r_e].iter().collect::<String>().to_string()
}

pub fn jump(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }

    let mut far: usize = 0;
    let mut split_pos: usize = 0;
    let mut r = 0;

    for (i, v) in nums.iter().enumerate() {
        if i == nums.len() - 1 {
            break;
        }
        let step: usize = *v as usize;
        far = far.max(i + step);

        if i == split_pos {
            split_pos = far;
            r += 1;
        }
    }

    r
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }

    let mut far: usize = 0;

    for (i, v) in nums.iter().enumerate() {
        let step: usize = *v as usize;
        if far >= i && i + step > far {
            far = i + step;
        }
    }

    far >= nums.len() - 1
}

pub fn unique_paths(m: usize, n: usize) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..m {
        dp[0][i] = 1;
    }

    for (i, _) in (0..n).enumerate() {
        dp[i][0] = 1;
    }

    for j in 1..n {
        for i in 1..m {
            dp[j][i] = dp[j - 1][i] + dp[j][i - 1];
        }
    }

    dp[n - 1][m - 1]
}

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let n = obstacle_grid.len();
    let m = obstacle_grid[0].len();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..m {
        if obstacle_grid[0][i] == 1 {
            break;
        }
        dp[0][i] = 1;
    }

    for (i, _) in (0..n).enumerate() {
        if obstacle_grid[i][0] == 1 {
            break;
        }
        dp[i][0] = 1;
    }

    for j in 1..n {
        for i in 1..m {
            if obstacle_grid[j][i] == 1 {
                dp[j][i] = 0;
            } else {
                dp[j][i] = dp[j - 1][i] + dp[j][i - 1];
            }
        }
    }

    dp[n - 1][m - 1]
}

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];

    let mut t = 0;
    for i in 0..m {
        t += grid[0][i];
        dp[0][i] = t;
    }

    let mut s = 0;
    for (i, _) in (0..n).enumerate() {
        s += grid[i][0];
        dp[i][0] = s;
    }

    for j in 1..n {
        for i in 1..m {
            let a = dp[j - 1][i];
            let b = dp[j][i - 1];
            dp[j][i] = a.min(b) + grid[j][i];
        }
    }

    dp[n - 1][m - 1]
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn check_min_path_sum() {
        let v1: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(min_path_sum(v1), 12);

        let v2: Vec<Vec<i32>> = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(min_path_sum(v2), 7);
    }

    #[test]
    fn check_unique_paths_with_obstacles() {
        let v2: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(unique_paths_with_obstacles(v2), 2);

        let v3: Vec<Vec<i32>> = vec![vec![0, 1], vec![0, 0]];
        assert_eq!(unique_paths_with_obstacles(v3), 1);

        let v: Vec<Vec<i32>> = vec![vec![0, 1, 1, 0], vec![0, 0, 0, 0]];
        assert_eq!(unique_paths_with_obstacles(v), 1);
    }

    #[test]
    fn check_unique_paths() {
        assert_eq!(unique_paths(7, 3), 28);
        assert_eq!(unique_paths(3, 3), 6);
        assert_eq!(unique_paths(2, 2), 2);
        assert_eq!(unique_paths(1, 1), 1);
    }

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

    #[test]
    fn check_min_cost_climbing_stairs() {
        assert_eq!(
            min_cost_climbing_stairs(vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0
            ]),
            1
        );
        assert_eq!(
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn check_max_repeating() {
        assert_eq!(max_repeating("aa".to_string(), "aa".to_string()), 1);
        assert_eq!(max_repeating("a".to_string(), "aa".to_string()), 0);
        assert_eq!(max_repeating("ababc".to_string(), "ac".to_string()), 0);
        assert_eq!(max_repeating("ababc".to_string(), "ba".to_string()), 1);
        assert_eq!(max_repeating("ababc".to_string(), "ab".to_string()), 2);
    }

    #[test]
    fn check_divisor_game() {
        assert!(!divisor_game(3));
        assert!(divisor_game(2));
        assert!(!divisor_game(1));
    }

    #[test]
    fn check_longest_palindrome() {
        assert_eq!(longest_palindrome("ccc".to_string()), "ccc".to_string());
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
        assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
    }

    #[test]
    fn check_jump() {
        assert_eq!(jump(vec![7, 0, 9, 6, 9, 6, 1, 7, 9, 0, 1, 2, 9, 0, 3]), 2);

        assert_eq!(
            jump(vec![
                7, 8, 4, 2, 0, 6, 4, 1, 8, 7, 1, 7, 4, 1, 4, 1, 2, 8, 2, 7, 3, 7, 8, 2, 4, 4, 5, 3,
                5, 6, 8, 5, 4, 4, 7, 4, 3, 4, 8, 1, 1, 9, 0, 8, 2
            ]),
            7
        );

        assert_eq!(
            jump(vec![
                8, 4, 8, 2, 5, 6, 5, 3, 5, 3, 3, 1, 6, 5, 8, 7, 4, 6, 8, 2, 3, 1, 2, 7, 5, 1, 2, 1,
                8, 1, 3, 3, 7, 8, 8, 4, 2, 6, 5, 1, 7, 5, 6, 8, 2, 7, 5, 6, 7, 2, 2, 5, 7, 4, 4, 6,
                8, 7, 2, 4, 8, 5, 2, 3, 6, 3, 5, 1, 6, 8, 3, 1, 7, 7, 1, 8, 2, 3, 5, 8, 6, 5, 3, 4,
                1, 8, 7, 3, 7, 2, 1, 1, 2, 8, 5, 1, 8, 3, 5, 5, 3, 3, 8, 8, 1, 6, 1, 8, 5, 1, 1, 6,
                6, 1, 8, 4, 2, 3, 4, 6, 4, 8, 6, 7, 8, 6, 2, 3, 2, 6, 7, 1, 3, 4, 1, 5, 5, 3, 6, 5,
                1, 5, 5, 1, 1, 1, 4, 2, 5, 2, 6, 1, 5, 3, 5, 3, 7, 6, 7, 7, 1, 1, 6, 3, 5, 2, 6, 7,
                5, 8, 2, 1, 2, 1, 4, 7, 3, 6, 7, 2, 7, 1, 6, 4, 4, 6, 6, 6, 6, 3, 4, 5, 5, 1, 5, 3,
                5, 7, 3, 4, 5, 3, 1, 3, 7, 6, 2, 2, 5, 7, 7, 6, 3, 4, 2, 5, 4, 1, 3, 3, 6, 2, 1, 1,
                3, 5, 7, 4, 5, 4, 8, 4, 5, 7, 6, 7, 5, 5, 5, 4, 1, 6, 1, 6, 6, 3, 1, 8, 6, 3, 8, 5,
                8, 7, 6, 8, 4, 5, 1, 5, 7, 7, 1, 3, 5, 5, 4, 1, 4, 8, 2, 5, 5, 6, 3, 4, 8, 1, 5, 4,
                1, 8, 2, 6, 5, 4, 8, 8, 5, 7, 1, 8, 4, 1, 5, 5, 7, 1, 6, 5, 8, 4, 3, 3, 8, 7, 1, 4,
                3, 1, 4, 5, 2, 7, 8, 3, 4, 4, 6, 7, 7, 5, 4, 3, 2, 4, 2, 5, 2, 6, 8, 8, 2, 7, 8, 2,
                6, 8, 5, 6, 3, 3, 4, 2, 3, 1, 4, 1, 8, 8, 2, 5, 2, 1, 5, 8, 2, 8, 2, 4, 6, 8, 6, 6,
                6, 5, 6, 8, 5, 7, 2, 1, 5, 2, 8, 8, 7, 1, 1, 5, 2, 5, 6, 6, 3, 8, 3, 5, 6, 4, 5, 7,
                8, 2, 6, 7, 4, 5, 7, 3, 8, 2, 4, 5, 1, 8, 7, 5, 2, 8, 1, 7, 1, 3, 1, 1, 4, 4, 1, 1,
                3, 3, 3, 8, 1, 8, 4, 5, 4, 7, 1, 1, 2, 6, 7, 5, 8, 8, 1, 3, 8, 2, 7, 4, 8, 8, 1, 2,
                5, 5, 5, 7, 4, 2, 2, 4, 6, 7, 6, 4, 3, 5, 8, 1, 7, 6, 6, 2, 1, 6, 2, 5, 2, 8, 3, 3,
                5, 7, 2, 1, 8, 5, 5, 6, 8, 8, 8, 8, 1, 3, 5, 2, 1, 6, 3, 8, 4, 7, 8, 2, 8, 4, 2, 4,
                8, 4, 2, 4, 6, 3, 7, 2, 1, 3, 5, 2, 5, 4, 7, 8, 7, 6, 3, 3, 7, 6, 2, 4, 6, 7, 8, 6,
                6, 4, 2, 8, 7, 5, 5, 8, 8, 8, 1, 2, 6, 1, 8, 1, 1, 4, 2, 7, 8, 5, 6, 4, 7, 3, 7, 3,
                2, 6, 5, 7, 8, 5, 1, 3, 3, 3, 6, 8, 7, 3, 3, 4, 7, 5, 8, 2, 4, 7, 8, 1, 6, 8, 7, 5,
                4, 2, 3, 3, 8, 8, 6, 3, 8, 2, 8, 6, 2, 2, 5, 8, 3, 7, 5, 8, 5, 7, 2, 7, 1, 7, 2, 3,
                1, 1, 8, 2, 4, 8, 8, 1, 2, 1, 2, 2, 8, 6, 6, 5, 1, 1, 1, 5, 1, 8, 5, 6, 1, 4, 4, 8,
                5, 8, 3, 3, 3, 5, 2, 5, 3, 7, 3, 5, 4, 3, 2, 4, 8, 7, 6, 4, 4, 4, 3, 8, 7, 8, 2, 4,
                6, 5, 6, 3, 4, 5, 3, 2, 6, 6, 7, 2, 5, 1, 5, 6, 2, 3, 4, 3, 3, 3, 3, 2, 4, 3, 7, 1,
                3, 5, 3, 2, 5, 5, 7, 6, 1, 2, 3, 2, 3, 8, 3, 6, 7, 4, 3, 8, 3, 7, 2, 7, 5, 2, 6, 8,
                2, 5, 1, 2, 8, 7, 8, 3, 1, 1, 7, 3, 6, 5, 7, 2, 8, 3, 3, 7, 2, 3, 7, 6, 1, 8, 4, 5,
                3, 3, 8, 5, 1, 1, 7, 3, 6, 1, 7, 6, 2, 2, 6, 1, 6, 8, 1, 7, 4, 1, 3, 4, 6, 6, 4, 4,
                3, 4, 4, 7, 5, 2, 2, 8, 7, 6, 5, 4, 3, 2, 8, 8, 2, 1, 3, 5, 7, 5, 2, 4, 7, 2, 2, 8,
                3, 8, 7, 4, 8, 5, 3, 3, 5, 5, 2, 1, 7, 6, 7, 1, 3, 3, 2, 2, 8, 8, 6, 2, 8, 3, 2, 3,
                8, 6, 4, 7, 7, 8, 2, 3, 6, 4, 8, 3, 3, 2, 1, 7, 6, 3, 8, 4, 8, 3, 1, 6, 3, 1, 2, 8,
                8, 2, 2, 7, 2, 5, 7, 3, 5, 8, 8, 3, 8, 6, 6, 2, 6, 6, 4, 7, 6, 1, 7, 8, 6, 8, 1, 2,
                3, 3, 6, 2, 7, 1, 2, 1, 1, 6, 8, 6, 6, 1, 2, 6, 8, 2, 4, 7, 1, 1, 3, 3, 7, 4, 8, 3,
                4, 6, 3, 6, 1, 6, 4, 3, 6, 7, 4, 8, 5, 7, 2, 3, 1, 5, 3, 5, 3, 3, 3, 6, 8, 6, 6, 8,
                3, 8, 3, 6, 2, 6, 4, 1, 6, 8, 1, 1, 6, 6, 6, 3, 6, 4, 7, 1, 1, 4, 2, 5, 5, 8, 2, 6,
                8, 1, 7, 5, 4, 7, 4, 7, 3, 1, 5, 7, 1, 5, 1, 1, 8, 2, 2, 3, 3, 4, 3, 7, 6, 1, 7, 2,
                8, 5, 6, 5, 4, 8, 2, 4, 3, 1, 2, 7, 3, 3, 3, 3, 4, 6, 2, 1, 4, 8, 1, 4, 3, 2, 7, 6,
                8, 8, 7, 2, 3, 1, 4, 1, 3, 3, 8, 8, 6, 2, 3, 3, 7, 3, 1, 5, 5, 2, 8, 8, 3, 7, 7, 7,
                7, 3, 7, 3, 7, 4, 5, 5, 8, 4, 8, 1, 4, 3, 7, 8, 5, 7, 1, 6, 2, 4, 3, 6, 5, 7, 2, 7,
                5, 1, 1, 6, 3, 3, 7, 7, 7, 4, 6, 7, 2, 3, 2, 8, 5, 7, 8, 7, 2, 7, 7, 8, 7, 3, 4, 4,
                5, 3, 6, 2, 2, 1, 4, 8, 5, 1, 2, 8, 4, 7, 8, 2, 1, 4, 4, 6, 5, 6, 2, 2, 6, 3, 1, 8,
                1, 3, 3, 3, 8, 1, 3, 7, 7, 5, 8, 3, 7, 3, 8, 3, 7, 8, 2, 1, 4, 4, 2, 7, 3, 8, 1, 8,
                4, 8, 8, 6, 6, 8, 5, 2, 6, 2, 3, 6, 1, 5, 2, 4, 6, 5, 6, 8, 3, 8, 2, 1, 8, 6, 8, 3,
                2, 4, 3, 4, 7, 5, 6, 6, 6, 4, 8, 1, 5, 6, 1, 1, 2, 6, 4, 3, 2, 1, 2, 4, 1, 4, 4, 8,
                2, 8, 8, 2, 1, 2, 4, 4, 5, 1, 5, 5, 6, 2, 4, 8, 4, 7, 3, 4, 2, 5, 7, 7, 3, 5, 5, 8,
                5, 7, 5, 4, 4, 6, 5, 6, 5, 2, 5, 7, 4, 3, 5, 8, 3, 7, 3, 7, 3, 7, 5, 8, 4, 3, 3, 4,
                6, 1, 3, 3, 6, 2, 4, 5, 4, 4, 8, 4, 6, 5, 1, 1, 2, 4, 7, 3, 8, 8, 1, 2, 3, 6, 7, 7,
                4, 5, 3, 5, 7, 3, 4, 8, 8, 6, 6, 2, 3, 3, 8, 3, 1, 3, 3, 2, 8, 3, 5, 7, 2, 6, 2, 7,
                3, 3, 3, 7, 5, 1, 2, 7, 8, 4, 7, 1, 4, 6, 5, 1, 2, 6, 3, 7, 7, 5, 4, 8, 7, 1, 1, 7,
                2, 4, 7, 8, 5, 2, 6, 6, 5, 4, 8, 6, 1, 4, 5, 5, 3, 7, 4, 4, 2, 3, 6, 8, 6, 8, 4, 1,
                8, 2, 3, 8, 3, 1, 6, 2, 8, 6, 1, 4, 3, 4, 8, 6, 6, 5, 8, 7, 4, 2, 1, 3, 7, 6, 7, 1,
                3, 2, 2, 8, 1, 5, 2, 6, 7, 8, 2, 8, 5, 2, 3, 7, 7, 6, 8, 3, 4, 6, 8, 2, 8, 7, 1, 4,
                1, 3, 6, 1, 8, 2, 8, 6, 8, 7, 1, 2, 5, 6, 5, 3, 4, 7, 5, 3, 4, 8, 4, 8, 3, 2, 7, 7,
                6, 2, 4, 8, 1, 1, 2, 8, 6, 6, 2, 2, 4, 3, 8, 6, 7, 7, 1, 8, 7, 2, 2, 3, 2, 4, 1, 2,
                6, 3, 6, 8, 5, 1, 6, 4, 7, 4, 2, 4, 5, 6, 8, 3, 7, 1, 5, 2, 8, 1, 2, 6, 3, 5, 4, 3,
                3, 8, 2, 7, 1, 2, 1, 1, 8, 7, 6, 3, 8, 2, 8, 4, 2, 1, 1, 2, 3, 8, 8, 6, 5, 4, 1, 5,
                5, 7, 8, 2, 8, 6, 6, 3, 7, 1, 5, 2, 2, 5, 2, 6, 5, 7, 3, 2, 8, 7, 8, 3, 7, 6, 5, 6,
                7, 3, 4, 1, 3, 2, 3, 6, 4, 6, 1, 1, 8, 3, 2, 2, 1, 1, 4, 3, 4, 6, 6, 2, 8, 1, 6, 6,
                1, 1, 6, 8, 8, 6, 4, 8, 3, 4, 5, 5, 5, 8, 8, 5, 8, 2, 1, 4, 6, 6, 7, 3, 6, 8, 4, 3,
                4, 6, 3, 7, 8, 6, 1, 7, 1, 5, 1, 1, 6, 3, 3, 3, 7, 4, 1, 3, 1, 5, 1, 5, 4, 3, 4, 4,
                6, 2, 3, 8, 0, 0, 0, 0, 0, 0, 0
            ]),
            284
        );

        assert_eq!(jump(vec![2, 2, 0, 1]), 2);
        assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn check_can_jump() {
        assert!(can_jump(vec![2, 0, 0]));
        assert!(!can_jump(vec![0, 2, 3]));
        assert!(can_jump(vec![0]));
        assert!(can_jump(vec![2, 5, 0, 0]));
        assert!(!can_jump(vec![3, 2, 1, 0, 4]));
        assert!(can_jump(vec![2, 3, 1, 1, 4]));

        assert!(can_jump(vec![
            7, 8, 4, 2, 0, 6, 4, 1, 8, 7, 1, 7, 4, 1, 4, 1, 2, 8, 2, 7, 3, 7, 8, 2, 4, 4, 5, 3, 5,
            6, 8, 5, 4, 4, 7, 4, 3, 4, 8, 1, 1, 9, 0, 8, 2
        ]));
    }
}
