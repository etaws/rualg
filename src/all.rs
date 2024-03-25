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

fn string_to_char_array(s: &str) -> Vec<char> {
    s.chars().collect()
}

pub struct Board<'a> {
    bx: usize,
    by: usize,
    word: &'a Vec<char>,
}

fn exist_bts(
    m: &mut Vec<Vec<i32>>,
    x: usize,
    y: usize,
    i: usize,
    b: &Board,
    board: &Vec<Vec<char>>,
) -> bool {
    m[y][x] = 1;

    if i == b.word.len() - 1 {
        return true;
    }

    let c = b.word.get(i + 1).unwrap();
    let w_len = b.word.len();

    if x > 0 && m[y][x - 1] == 0 {
        let next = board[y][x - 1];
        if (i + 1 != w_len) && (*c == next) {
            let left = exist_bts(m, x - 1, y, i + 1, b, board);
            if left {
                return true;
            }
        }
    }

    if y > 0 && m[y - 1][x] == 0 {
        let next = board[y - 1][x];
        if (i + 1 != w_len) && (*c == next) {
            let up = exist_bts(m, x, y - 1, i + 1, b, board);
            if up {
                return true;
            }
        }
    }

    if x < b.bx && m[y][x + 1] == 0 {
        let next = board[y][x + 1];
        if (i + 1 != w_len) && (*c == next) {
            let right = exist_bts(m, x + 1, y, i + 1, b, board);
            if right {
                return true;
            }
        }
    }

    if y < b.by && m[y + 1][x] == 0 {
        let next = board[y + 1][x];
        if (i + 1 != w_len) && (*c == next) {
            let down = exist_bts(m, x, y + 1, i + 1, b, board);
            if down {
                return true;
            }
        }
    }

    m[y][x] = 0;
    false
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let mut m: Vec<Vec<i32>> = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
    ];

    let w = string_to_char_array(&word);

    let by = board.len() - 1;

    for (y, row) in board.iter().enumerate() {
        let bx = row.len() - 1;
        for (x, c) in row.iter().enumerate() {
            if (*c) != w[0] {
                continue;
            }
            let b: Board = Board { bx, by, word: &w };
            let b = exist_bts(&mut m, x, y, 0, &b, &board);
            if b {
                return true;
            }
        }
    }

    false
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

    #[test]
    fn check_exist() {
        let v: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        let e = exist(v, "ABCCED".to_string());
        assert!(e);
    }

    #[test]
    fn check_exis_2() {
        let v: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        let e = exist(v, "SEE".to_string());
        assert!(e);
    }

    #[test]
    fn check_exis_3() {
        let v: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        let e = exist(v, "ABCB".to_string());
        assert!(!e);
    }

    #[test]
    fn check_exis_4() {
        let v: Vec<Vec<char>> = vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A'],
        ];

        let e = exist(v, "AAAAAAAAAAAAA".to_string());
        assert!(!e);
    }
}
