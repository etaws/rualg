use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut st: HashMap<i32, i32> = HashMap::new();

    st.insert(0, 1);

    let mut sum = 0;
    let mut count = 0;
    for n in nums.into_iter() {
        sum += n;

        let w = sum - k;
        if let Some(c) = st.get(&w) {
            count += c;
        }

        let mc = st.entry(sum).or_insert(0);
        *mc += 1;
    }

    count
}

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut v = Vec::new();

    if s.len() < p.len() {
        return v;
    }

    let mut d: [i32; 26] = [0; 26];

    let mut a: Vec<usize> = Vec::new();
    for c in s.chars() {
        a.push((c as u8 - b'a').into());
    }

    let mut b: Vec<usize> = Vec::new();
    for c in p.chars() {
        b.push((c as u8 - b'a').into());
    }

    let mut i = 0;
    let mut diff = 0;
    while i < p.len() {
        let pv = b[i];
        d[pv] -= 1;

        if d[pv] == 0 {
            diff -= 1;
        }
        if d[pv] == -1 {
            diff += 1;
        }

        let sv = a[i];
        d[sv] += 1;
        if d[sv] == 0 {
            diff -= 1;
        }
        if d[sv] == 1 {
            diff += 1;
        }

        i += 1;
    }

    if diff == 0 {
        v.push(0);
    }

    let mut j = 0;
    while i < s.len() {
        let jv = a[j];
        d[jv] -= 1;
        if d[jv] == 0 {
            diff -= 1;
        }
        if d[jv] == -1 {
            diff += 1;
        }
        j += 1;

        let iv = a[i];
        d[iv] += 1;
        if d[iv] == 0 {
            diff -= 1;
        }
        if d[iv] == 1 {
            diff += 1;
        }
        i += 1;

        if diff == 0 {
            v.push(j as i32)
        }
    }

    v
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let len = s.len();

    if s.is_empty() || (s.len() == 1) {
        return len as i32;
    }

    let mut a: Vec<char> = Vec::new();
    for c in s.chars() {
        a.push(c);
    }

    let mut st: HashMap<char, usize> = HashMap::new();

    let mut i = 0;
    let mut max = 0;
    let mut cur: usize = 0;
    while i < s.len() {
        let c = a[i];

        match st.get(&c) {
            Some(j) => {
                if cur > max {
                    max = cur;
                }

                if *j == (len - 1) {
                    break;
                }

                i = j + 1;
                cur = 0;

                st.clear();
            }
            None => {
                st.insert(c, i);
                i += 1;
                cur += 1;
            }
        };
    }

    if cur > max {
        max = cur;
    }

    max as i32
}

pub fn trap(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let mut v: Vec<(usize, i32)> = Vec::new();
    let mut top = -1;
    for (i, n) in height.iter().enumerate() {
        if (*n == 0) && v.is_empty() {
            continue;
        }

        if v.is_empty() {
            v.push((i, *n));
            top = *n;
            continue;
        }

        if (v.len() == 1) && (top <= *n) {
            v[0] = (i, *n);
            top = *n;
            continue;
        }

        if (v.len() >= 2) && (top >= *n) {
            v.push((i, *n));
            top = *n;
            continue;
        }

        let cur = v.pop();
    }

    0
}

pub fn max_area(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let mut i = 0;
    let mut j = height.len() - 1;

    let mut max = 0;
    while i < j {
        let short_h: i32 = if height[i] < height[j] {
            height[i]
        } else {
            height[j]
        };
        let c = (j - i) as i32 * short_h;
        if c > max {
            max = c;
        }

        if short_h == height[i] {
            i += 1;
        } else {
            j -= 1;
        }
    }

    max
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut n = nums.clone();
    n.sort_unstable();

    let mut max_r = 0;

    let mut r = 1;

    let mut i = 0;
    while i < nums.len() {
        if i == 0 {
            r = 1;
        } else {
            let pre = n[i - 1];
            if n[i] == pre {
            } else if n[i] == (pre + 1) {
                r += 1;
            } else {
                if r > max_r {
                    max_r = r;
                }
                r = 1;
            }
        }

        i += 1;
    }

    if r > max_r {
        max_r = r;
    }

    max_r
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return;
    }

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < nums.len() {
        if nums[i] != 0 {
            nums[j] = nums[i];
            j += 1;
        }

        i += 1;
    }

    while j < nums.len() {
        nums[j] = 0;

        j += 1;
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut s: HashMap<i32, usize> = HashMap::new();

    for (j, n) in nums.iter().enumerate() {
        let another = target - n;

        if s.get(&another).is_some() {
            let one = j;
            let two = *(s.get(&another).unwrap());

            // 返回的时候规范一下：总是小的数放在前面
            if one > two {
                return vec![two as i32, one as i32];
            } else {
                return vec![one as i32, two as i32];
            }
        } else {
            s.insert(*n, j);
        }
    }

    vec![0, 0]
}

pub fn seek_two(a: &[i32], target: i32) -> (usize, usize) {
    let mut s: HashMap<i32, usize> = HashMap::new();

    for (j, n) in a.iter().enumerate() {
        let another = target - n;

        if s.get(&another).is_some() {
            let one = j;
            let two = *(s.get(&another).unwrap());

            // 返回的时候规范一下：总是小的数放在前面
            if one > two {
                return (two, one);
            } else {
                return (one, two);
            }
        } else {
            s.insert(*n, j);
        }
    }

    (0, 0)
}

pub struct NumMatrix {
    pub sums: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sums: Vec<Vec<i32>> = Vec::with_capacity(matrix.len());

        // i 是这个 Matrix 有多少行，就按多少行来遍历
        for i in 0..matrix.len() {
            sums.push(vec![0; matrix[i].len()]);
            let mut sum = 0;
            // j 是这个 Matrix 有多少列，在第 i 行中按列来遍历
            for j in 0..matrix[i].len() {
                if i == 0 && j == 0 {
                    sums[i][j] = matrix[i][j];
                    sum = matrix[i][j];
                } else if i == 0 && j > 0 {
                    // 第一行，累加
                    sum += matrix[i][j];
                    sums[i][j] = sum;
                } else if i > 0 && j == 0 {
                    // 第一列，累加
                    let cell = sums[i - 1][j];
                    sums[i][j] = matrix[i][j] + cell;
                } else {
                    let cell = sums[i - 1][j] + sums[i][j - 1] - sums[i - 1][j - 1];
                    sums[i][j] = matrix[i][j] + cell;
                }
            }
        }

        NumMatrix { sums }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let i1: usize = row1 as usize;
        let j1: usize = col1 as usize;
        let i2: usize = row2 as usize;
        let j2: usize = col2 as usize;

        if i1 == 0 && j1 == 0 {
            return self.sums[i2][j2];
        }

        if i1 == 0 && j1 > 0 {
            return self.sums[i2][j2] - self.sums[i2][j1 - 1];
        }

        if i1 > 0 && j1 == 0 {
            return self.sums[i2][j2] - self.sums[i1 - 1][j2];
        }

        let a = self.sums[i2][j2];
        let b = self.sums[i1 - 1][j2];
        let c = self.sums[i2][j1 - 1];
        let d = self.sums[i1 - 1][j1 - 1];

        a - b - c + d
    }
}

pub fn to_num(s: &[u8]) -> u64 {
    let mut r: u64 = 0;
    for u in s.iter() {
        r = r * 10 + (u - b'0') as u64;
    }

    r
}

pub fn get_first_num(s: &[u8]) -> bool {
    let len: usize = s.len();

    for j in 1..len - 1 {
        for k in j + 1..len {
            let s1 = &s[0..j];
            let s2 = &s[j..k];

            if s1.len() >= 2 && s1[0] == b'0' {
                continue;
            }
            if s2.len() >= 2 && s2[0] == b'0' {
                continue;
            }
            if k >= len {
                continue;
            }
            if s1.len() > (len / 2) {
                continue;
            }
            if s2.len() > (len / 2) {
                continue;
            }

            let n1 = to_num(s1);
            let n2 = to_num(s2);

            if let Some(r) = find_num(n1 + n2, &s[k..]) {
                let mut u1 = n2;
                let mut u2 = n1 + n2;
                let mut i = k + r + 1;

                if i == s.len() {
                    return true;
                }

                while let Some(r) = find_num(u1 + u2, &s[i..]) {
                    let t = u2;
                    u2 += u1;
                    u1 = t;

                    if (i + r + 1) == s.len() {
                        return true;
                    }

                    i = i + r + 1;
                }
            }
        }
    }

    false
}

pub fn is_additive_number(num: String) -> bool {
    let s = num.as_bytes();
    if s.len() < 3 {
        return false;
    }

    get_first_num(s)
}

pub fn find_num(u: u64, s: &[u8]) -> Option<usize> {
    if s.len() >= 2 && s[0] == b'0' {
        return None;
    }

    let sn = u.to_string();
    if sn.len() > s.len() {
        return None;
    }

    let a = sn.as_bytes();

    for i in 0..sn.len() {
        let n1 = a[i];
        let n2 = s[i];

        if n1 != n2 {
            return None;
        }
    }

    Some(sn.len() - 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_seek_two_1() {
        let a: [i32; 4] = [11, 15, 2, 7];
        let target = 9;

        let r = seek_two(&a, target);

        assert_eq!(r, (2, 3));
    }

    #[test]
    fn check_seek_two_2() {
        let a: [i32; 3] = [3, 2, 4];
        let target = 6;

        let r = seek_two(&a, target);

        assert_eq!(r, (1, 2));
    }

    #[test]
    fn check_seek_two_3() {
        let a: [i32; 2] = [3, 3];
        let target = 6;

        let r = seek_two(&a, target);

        assert_eq!(r, (0, 1));
    }

    #[test]
    fn check_move_zeroes() {
        let mut v = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut v);
        assert_eq!(v, vec![1, 3, 12, 0, 0]);

        let mut v1 = vec![0];
        move_zeroes(&mut v1);
        assert_eq!(v1, vec![0]);
    }

    #[test]
    fn check_sum() {
        let v: Vec<Vec<i32>> = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];

        let my_matrix: NumMatrix = NumMatrix::new(v);

        assert_eq!(my_matrix.sums.len(), 5);

        assert_eq!(my_matrix.sums[0][1], 3);
        assert_eq!(my_matrix.sums[0][2], 4);
        assert_eq!(my_matrix.sums[0][3], 8);
        assert_eq!(my_matrix.sums[0][4], 10);

        assert_eq!(my_matrix.sums[4][0], 14);
        assert_eq!(my_matrix.sums[4][1], 23);
        assert_eq!(my_matrix.sums[4][2], 30);
        assert_eq!(my_matrix.sums[4][3], 38);
        assert_eq!(my_matrix.sums[4][4], 58);

        assert_eq!(my_matrix.sum_region(0, 0, 0, 0), 3);
        assert_eq!(my_matrix.sum_region(0, 1, 0, 2), 1);
        assert_eq!(my_matrix.sum_region(2, 1, 4, 3), 8);
        assert_eq!(my_matrix.sum_region(1, 1, 2, 2), 11);
        assert_eq!(my_matrix.sum_region(1, 2, 2, 4), 12);
    }

    #[test]
    fn check_is_additive_number() {
        assert!(is_additive_number("199100".to_string()));
        assert!(is_additive_number("199100199".to_string()));
        assert!(!is_additive_number("199100198".to_string()));
        assert!(is_additive_number("112358".to_string()));
        assert!(is_additive_number("123".to_string()));
        assert!(!is_additive_number("1203".to_string()));
        assert!(is_additive_number("12122436".to_string()));
        assert!(is_additive_number(
            "1999999999999999910000000000000000".to_string()
        ));
    }

    #[test]
    fn check_longest_consecutive() {
        assert_eq!(longest_consecutive(vec![1, 2, 0, 1]), 3);
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }

    #[test]
    fn check_max_area() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn check_longest_sub() {
        assert_eq!(length_of_longest_substring("".to_string()), 0);
        assert_eq!(length_of_longest_substring("a".to_string()), 1);
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
    }

    #[test]
    fn check_find_anagrams() {
        assert_eq!(
            find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );

        assert_eq!(
            find_anagrams("abab".to_string(), "ab".to_string()),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn check_subarray_sum() {
        assert_eq!(subarray_sum(vec![1], 0), 0);
        assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
        assert_eq!(subarray_sum(vec![-1, -1, 1], 0), 1);
        assert_eq!(subarray_sum(vec![1, -1, 0], 0), 3);
    }
}
