pub fn quick_sort(a: &mut [usize]) {
    if a.len() <= 1 {
        return;
    }

    qsort(a, 0, a.len() - 1);
}

pub fn qsort(a: &mut [usize], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let p: usize = 232834 % (right - left) + left;
    a.swap(left, p);

    let mut i = left;
    let mut j = right + 1;

    loop {
        loop {
            i += 1;
            if (i == right) || (a[i] >= a[left]) {
                break;
            }
        }

        loop {
            j -= 1;
            if (j == left) || (a[left] >= a[j]) {
                break;
            }
        }

        if i < j {
            a.swap(i, j);
        }

        if i >= j {
            break;
        }
    }
    a.swap(left, j);

    if left + 1 < j {
        qsort(a, left, j - 1);
    }
    if j + 1 < right {
        qsort(a, j + 1, right);
    }
}

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

pub fn select_sort(a: &mut [usize]) {
    if a.len() <= 1 {
        return;
    }

    let mut i = a.len() - 1;
    while i > 0 {
        let c = a[i];

        let mut j = 1;
        let mut max_i = 0;
        while j < i {
            if a[max_i] < a[j] {
                max_i = j;
            }

            j += 1;
        }

        if c < a[max_i] {
            a[i] = a[max_i];
            a[max_i] = c;
        }

        i -= 1;
    }
}

pub fn insertion_sort(a: &mut [usize]) {
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
    fn check_select_sort() {
        check_sort_suits(select_sort);
    }

    #[test]
    fn check_bubbling_sort() {
        check_sort_suits(bubbling_sort);
    }

    #[test]
    fn check_insertion_sort() {
        check_sort_suits(insertion_sort);
    }

    #[test]
    fn check_quick_sort() {
        check_sort_suits(quick_sort);
    }

    fn check_sort_suits(sort_fn: fn(a: &mut [usize])) {
        diff_sort(
            &mut vec![3, 5, 8, 10, 0, 0, 0, 0, 0, 0],
            &vec![0, 0, 0, 0, 0, 0, 3, 5, 8, 10],
            sort_fn,
        );

        diff_sort(
            &mut vec![5, 1, 26, 37, 61, 11, 15, 19, 59, 48],
            &vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61],
            sort_fn,
        );

        diff_sort(
            &mut vec![26, 5, 37, 1, 61, 11, 59, 15, 48, 19],
            &vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61],
            sort_fn,
        );

        diff_sort(
            &mut vec![11, 7, 20, 9, 18, 21, 19, 8, 10, 22],
            &vec![7, 8, 9, 10, 11, 18, 19, 20, 21, 22],
            sort_fn,
        );

        diff_sort(&mut vec![1, 5], &vec![1, 5], sort_fn);

        diff_sort(&mut vec![0], &vec![0], sort_fn);
    }

    fn diff_sort(sorted: &mut Vec<usize>, expected: &Vec<usize>, sort_fn: fn(a: &mut [usize])) {
        sort_fn(sorted);
        assert_eq!(sorted, expected);
    }
}
