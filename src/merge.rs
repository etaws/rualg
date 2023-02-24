pub fn merge_two(a: &mut [i32], a_len: usize, b: &[i32]) {
    assert!(a.len() >= (a_len + b.len()));

    if b.is_empty() {
        return;
    }

    if a_len == 0 {
        for (j, n) in b.iter().enumerate() {
            a[j] = *n;
        }
        return;
    }

    let mut i = a_len - 1;
    let mut j = b.len() - 1;

    let mut current = a_len + b.len() - 1;

    let mut a_end = false;
    let mut b_end = false;

    loop {
        if b_end || (a[i] > b[j]) {
            a[current] = a[i];
            if i == 0 {
                a_end = true;
            }
            if i > 0 {
                i -= 1;
            }
        } else if a_end || (a[i] <= b[j]) {
            a[current] = b[j];
            if j == 0 {
                b_end = true;
            }
            if j > 0 {
                j -= 1;
            }
        }

        if current == 0 {
            break;
        } else {
            current -= 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_merge_1() {
        let mut a: [i32; 6] = [1, 2, 3, 0, 0, 0];
        let b: [i32; 3] = [2, 5, 6];
        merge_two(&mut a, 3, &b);

        let c: [i32; 6] = [1, 2, 2, 3, 5, 6];
        assert_eq!(a, c);
    }

    #[test]
    fn check_merge_2() {
        let mut a: [i32; 1] = [1];
        let b: [i32; 0] = [];
        merge_two(&mut a, 1, &b);

        let c: [i32; 1] = [1];
        assert_eq!(a, c);
    }

    #[test]
    fn check_merge_3() {
        let mut a: [i32; 1] = [0];
        let b: [i32; 1] = [1];
        merge_two(&mut a, 0, &b);

        let c: [i32; 1] = [1];
        assert_eq!(a, c);
    }
}
