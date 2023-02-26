pub fn disappear_number(a: &mut [i32]) -> Vec<usize> {
    let mut v: Vec<usize> = vec![];

    let mut i = 0;
    while i < a.len() {
        let j: usize = if a[i] < 0 {
            (0 - a[i] - 1).try_into().unwrap()
        } else {
            (a[i] - 1).try_into().unwrap()
        };

        if a[j] > 0 {
            assert!(a[j] > 0 && a[j] <= a.len().try_into().unwrap());
            a[j] = 0 - a[j];
        }

        i += 1;
    }

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
