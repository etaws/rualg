pub fn disappear_number(a: &mut [i32]) -> Vec<usize> {
    let mut v: Vec<usize> = vec![];

    let mut i = 0;
    while i < a.len() {
        // 第几个数已经存在？如果是第 1 个数存在，那么 j 就是 0
        let j: usize = if a[i] < 0 {
            (0 - a[i] - 1).try_into().unwrap()
        } else {
            (a[i] - 1).try_into().unwrap()
        };

        // 如果 j 这个数已经存在，那么把 a[j] 改为负数，作为 j 存在的标志
        if a[j] > 0 {
            assert!(a[j] > 0 && a[j] <= a.len().try_into().unwrap());
            a[j] = 0 - a[j];
        }

        i += 1;
    }

    // 如果 a[j] 还是正数，说明第 j + 1 个数不存在，放如结果中
    for (j, n) in a.iter().enumerate() {
        if *n > 0 {
            v.push(j + 1);
        }
    }

    v
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_disppear_number_1() {
        let mut a: [i32; 8] = [4, 3, 2, 7, 8, 2, 3, 1];

        let r = disappear_number(&mut a);

        assert_eq!(r, [5, 6]);
    }
}
