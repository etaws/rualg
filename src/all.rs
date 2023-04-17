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
}
