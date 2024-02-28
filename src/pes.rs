pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();

    let mut a: Vec<i32> = Vec::with_capacity(len);
    if len <= 2 {
        a.push(nums[1]);
        a.push(nums[0]);
        return a;
    }

    let mut l: Vec<i32> = Vec::with_capacity(len);
    let mut r: Vec<i32> = Vec::with_capacity(len);

    let mut lv = 1;
    let mut li: usize = 1;
    while li <= len {
        l.push(lv);
        lv *= nums[li - 1];
        li += 1;
    }

    let mut rv = 1;
    let mut ri: usize = len - 2;
    while ri != 0 {
        r.push(rv);
        rv *= nums[ri + 1];
        ri -= 1;
    }
    r.push(rv);

    rv *= nums[1];
    r.push(rv);

    r.reverse();

    a.push(r[0]);
    let mut j: usize = 1;
    while j < len - 1 {
        a.push(l[j] * r[j]);
        j += 1;
    }

    a.push(l[len - 1]);

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_product_except_self() {
        let a = vec![1, 2, 3, 4];
        let b = product_except_self(a);

        assert_eq!(b, vec![24, 12, 8, 6]);
    }

    #[test]
    fn check_product_except_self_0() {
        let a = vec![-1, 1, 0, -3, 3];
        let b = product_except_self(a);

        assert_eq!(b, vec![0, 0, 9, 0, 0]);
    }
}
