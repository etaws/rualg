pub fn bubbling_sort(a: &mut [usize]) {
    if a.len() <= 1 {
        return;
    }

    let mut i = a.len() - 1;

    while i >= 1 {
        let mut j = 1;
        while j <= i {
            if a[j - 1] > a[j] {
                a.swap(j - 1, j);
            }

            j += 1;
        }

        i -= 1;
    }
}

pub fn insertion_sort(a: &mut Vec<usize>) {
    if a.len() <= 1 {
        return;
    }

    let mut i = 1;
    while i < a.len() {
        let mut j = i;
        let c = a[j];
        while j > 0 {
            if c > a[j - 1] {
                break;
            }

            a[j] = a[j - 1];

            j -= 1;
        }
        a[j] = c;

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bubbling_sort() {
        diff_bubbling_sort(
            &mut vec![3, 5, 8, 10, 0, 0, 0, 0, 0, 0],
            &vec![0, 0, 0, 0, 0, 0, 3, 5, 8, 10],
        );
    }

    #[test]
    fn check_insertion_sort() {
        diff_insertion_sort(
            &mut vec![3, 5, 8, 10, 0, 0, 0, 0, 0, 0],
            &vec![0, 0, 0, 0, 0, 0, 3, 5, 8, 10],
        );

        diff_insertion_sort(
            &mut vec![26, 5, 37, 1, 61, 11, 59, 15, 48, 19],
            &vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61],
        );

        diff_insertion_sort(
            &mut vec![5, 1, 26, 37, 61, 11, 15, 19, 59, 48],
            &vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61],
        );

        diff_insertion_sort(
            &mut vec![11, 7, 20, 9, 18, 21, 19, 8, 10, 22],
            &vec![7, 8, 9, 10, 11, 18, 19, 20, 21, 22],
        );

        diff_insertion_sort(&mut vec![1, 5], &vec![1, 5]);

        diff_insertion_sort(&mut vec![0], &vec![0]);
    }

    fn diff_insertion_sort(sorted: &mut Vec<usize>, expected: &Vec<usize>) {
        insertion_sort(sorted);
        assert_eq!(sorted, expected);
    }

    fn diff_bubbling_sort(sorted: &mut Vec<usize>, expected: &Vec<usize>) {
        bubbling_sort(sorted);
        assert_eq!(sorted, expected);
    }
}
