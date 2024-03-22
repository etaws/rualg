pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut r: Vec<Vec<i32>> = Vec::new();

    let len = nums.len();

    let mut path = Vec::new();
    ddd(&nums, &mut r, &mut path, 0, len);

    r
}

pub fn ddd(nums: &Vec<i32>, r: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, start: usize, len: usize) {
    r.push(path.clone());
    for i in start..len {
        path.push(nums[i]);
        ddd(nums, r, path, i + 1, len);
        path.pop();
    }
}

fn permute_btrace(
    nums: &Vec<i32>,
    path: &mut Vec<i32>,
    used: &mut Vec<i32>,
    r: &mut Vec<Vec<i32>>,
) {
    if path.len() == nums.len() {
        r.push(path.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] == 1 {
            continue;
        }

        path.push(nums[i]);
        used[i] = 1;
        permute_btrace(nums, path, used, r);

        path.pop();
        used[i] = 0;
    }
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut r: Vec<Vec<i32>> = Vec::new();

    let mut path: Vec<i32> = Vec::new();

    let mut used = vec![0; nums.len()];

    permute_btrace(&nums, &mut path, &mut used, &mut r);

    r
}

fn char_array_to_string(chars: &[char]) -> String {
    chars.iter().collect::<String>()
}

fn letter_combinations_btrace(
    arr: &Vec<Vec<char>>,
    d: &Vec<u32>,
    depth: usize,
    path: &mut Vec<char>,
    r: &mut Vec<String>,
) {
    let path_len = d.len();

    if path.len() == path_len {
        let s = char_array_to_string(path);
        r.push(s);
        return;
    }

    let i: usize = d[depth] as usize;
    let row = arr.get(i).unwrap();
    for &c in row.iter() {
        path.push(c);
        letter_combinations_btrace(arr, d, depth + 1, path, r);
        path.pop();
    }
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut r: Vec<String> = Vec::new();

    if digits.is_empty() {
        return r;
    }

    let arr: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
        vec!['j', 'k', 'l'],
        vec!['m', 'o', 'n'],
        vec!['p', 'q', 'r', 's'],
        vec!['t', 'u', 'v'],
        vec!['w', 'x', 'y', 'z'],
    ];

    let mut d: Vec<u32> = Vec::new();
    for c in digits.chars() {
        if let Some(n) = c.to_digit(10) {
            d.push(n);
        }
    }

    let mut path: Vec<char> = Vec::new();

    letter_combinations_btrace(&arr, &d, 0, &mut path, &mut r);

    r
}

fn combination_btrace(
    candidates: &Vec<i32>,
    path: &mut Vec<i32>,
    mut path_sum: i32,
    target: i32,
    r: &mut Vec<Vec<i32>>,
    start: usize,
) {
    if path_sum > target {
        return;
    }

    if path_sum == target {
        r.push(path.clone());
        return;
    }

    for i in 0..candidates.len() {
        if i < start {
            continue;
        }
        let n = candidates[i];
        path.push(n);
        path_sum += n;
        combination_btrace(candidates, path, path_sum, target, r, i);
        path_sum -= n;
        path.pop();
    }
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut r: Vec<Vec<i32>> = Vec::new();

    let mut path: Vec<i32> = Vec::new();
    let path_sum = 0;
    combination_btrace(&candidates, &mut path, path_sum, target, &mut r, 0);

    r
}

fn generate_parenthesis_bts(
    n: i32,
    mut left: i32,
    right: i32,
    path: &mut Vec<char>,
    r: &mut Vec<String>,
) {
    if path.len() == (2 * n as usize) {
        r.push(char_array_to_string(path));
        return;
    }

    if (n - left) > 0 {
        path.push('(');
        left += 1;
        generate_parenthesis_bts(n, left, right + 1, path, r);
        left -= 1;
        path.pop();
    }

    if right > 0 {
        path.push(')');
        generate_parenthesis_bts(n, left, right - 1, path, r);
        path.pop();
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut r: Vec<String> = Vec::new();

    let mut path: Vec<char> = Vec::new();
    path.push('(');

    generate_parenthesis_bts(n, 1, 1, &mut path, &mut r);

    r
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_sub_1() {
        let r = subsets(vec![]);

        let v: Vec<Vec<i32>> = vec![vec![]];

        assert_eq!(r, v);
    }

    #[test]
    fn check_sub_2() {
        let r = subsets(vec![1]);

        let v: Vec<Vec<i32>> = vec![vec![], vec![1]];

        assert_eq!(r, v);
    }

    #[test]
    fn check_sub_3() {
        let r = subsets(vec![1, 2]);

        let v: Vec<Vec<i32>> = vec![vec![], vec![1], vec![1, 2], vec![2]];

        assert_eq!(r, v);
    }

    #[test]
    fn check_permute() {
        let nums = vec![1, 2, 3];
        let e = permute(nums);

        assert_eq!(e.len(), 6);

        dbg!(e);
    }

    #[test]
    fn check_letter() {
        let e = letter_combinations("23".to_string());
        assert_eq!(e.len(), 9);
    }

    #[test]
    fn check_combination_sum() {
        let e = combination_sum(vec![2, 3, 6, 7], 7);
        dbg!(e);
    }

    #[test]
    fn check_generate_parenthesism() {
        let e = generate_parenthesis(3);
        dbg!(e);
    }
}
