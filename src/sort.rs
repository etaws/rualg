pub fn bubbling_sort(a: &mut [usize]) {
    if a.len() <= 1 {
        return;
    }

    let mut i = a.len() - 1;

    while i >= 1 {
        let mut j = 1;
        while j <= i {
            if a[j - 1] > a[j] {
                let t = a[j - 1];
                a[j - 1] = a[j];
                a[j] = t;
            }

            j += 1;
        }

        i -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sort() {
        let expected: [usize; 10] = [0, 0, 0, 0, 0, 0, 3, 5, 8, 10];

        let mut a: [usize; 10] = [3, 5, 8, 10, 0, 0, 0, 0, 0, 0];

        bubbling_sort(&mut a);

        dbg!(a);

        assert_eq!(a, expected);
    }
}
