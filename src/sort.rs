use std::cmp::Ordering;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut r: Vec<Vec<i32>> = Vec::new();
    if nums.len() <= 2 {
        return r;
    }

    let mut a = nums;

    quick_sort(&mut a);

    for (i, &n) in a.iter().enumerate() {
        if n > 0 {
            return r;
        }
        if (i >= 1) && (a[i - 1] == a[i]) {
            continue;
        }

        let mut j = i + 1;
        let mut k = a.len() - 1;
        while j < k {
            let sum = a[i] + a[j] + a[k];
            match sum.cmp(&0) {
                Ordering::Greater => {
                    k -= 1;
                }
                Ordering::Less => {
                    j += 1;
                }
                Ordering::Equal => {
                    r.push(vec![n, a[j], a[k]]);
                    while j < k && (a[j] == a[j + 1]) {
                        j += 1;
                    }
                    while j < k && (a[k - 1] == a[k]) {
                        k -= 1;
                    }

                    j += 1;
                    k -= 1;
                }
            }
        }
    }
    r
}

pub fn inner_merge(a: &mut [usize], b: &mut [usize], i: usize, m: usize, n: usize) {
    let len = a.len();

    let mut s = i;
    let mut t = m + 1;

    let mut j = i;

    while s <= m && t <= n {
        if (s >= len) || (t >= len) {
            break;
        }
        if a[s] < a[t] {
            b[j] = a[s];
            s += 1;
        } else {
            b[j] = a[t];
            t += 1;
        }

        j += 1;
    }

    while (s <= m) && (s < len) {
        b[j] = a[s];
        j += 1;
        s += 1;
    }

    while (t <= n) && (t < len) {
        b[j] = a[t];
        j += 1;
        t += 1;
    }
}

pub fn merge_pass(a: &mut [usize], b: &mut [usize], size: usize) {
    let mut i = 0;
    loop {
        inner_merge(a, b, i, i + size - 1, i + size * 2 - 1);

        if (i + size * 2) >= a.len() {
            break;
        }

        i += size * 2;
    }
}

pub fn merge_sort(a: &mut [usize]) {
    let mut b = vec![0; a.len()];

    let mut size = 1;
    while size < a.len() {
        merge_pass(a, &mut b, size);
        size *= 2;

        merge_pass(&mut b, a, size);
        size *= 2;
    }
}

pub fn adjust_heap(a: &mut [usize], root: usize, len: usize) {
    if a.len() <= 1 {
        return;
    }

    if root >= a.len() {
        return;
    }

    let mut i = root;

    while i < len {
        let left = (i + 1) * 2 - 1;
        let right = (i + 1) * 2 + 1 - 1;

        if left >= len {
            break;
        }

        if right >= len {
            if a[left] >= a[i] {
                a.swap(i, left);
            }
            break;
        }

        if (a[i] >= a[left]) && (a[i] >= a[right]) {
            break;
        }

        if a[left] >= a[right] {
            a.swap(i, left);
            i = left;
        } else {
            a.swap(i, right);
            i = right;
        }
    }
}

pub fn heap_sort(a: &mut [usize]) {
    if a.len() <= 1 {
        return;
    }

    let mut i = a.len() / 2 - 1;

    loop {
        if i == 0 {
            break;
        }

        adjust_heap(a, i, a.len());

        i -= 1;
    }

    adjust_heap(a, 0, a.len());

    let mut j = a.len() - 1;
    loop {
        if j == 0 {
            break;
        }
        a.swap(0, j);

        adjust_heap(a, 0, j);

        j -= 1;
    }
}

pub fn quick_sort(a: &mut [i32]) {
    if a.len() <= 1 {
        return;
    }

    qsort(a, 0, a.len() - 1);
}

pub fn qsort(a: &mut [i32], left: usize, right: usize) {
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

pub fn search_insert(nums: &[i32], target: i32) -> i32 {
    let len = nums.len();

    if len == 0 {
        return -1;
    }

    let mut i: i32 = 0;
    let mut j: i32 = (len as i32) - 1;

    while i <= j {
        let mid = i + (j - i) / 2;

        match target.cmp(&nums[mid as usize]) {
            Ordering::Greater => {
                i = mid + 1;
            }
            Ordering::Less => {
                j = mid - 1;
            }
            Ordering::Equal => {
                return mid;
            }
        }
    }

    i
}

pub fn search(nums: &[i32], target: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = (nums.len() as i32) - 1;

    while i <= j {
        let mid = i + (j - i) / 2;

        match target.cmp(&nums[mid as usize]) {
            Ordering::Greater => {
                i = mid + 1;
            }
            Ordering::Less => {
                j = mid - 1;
            }
            Ordering::Equal => {
                return mid;
            }
        }
    }

    -1
}

pub fn search_small(nums: &[i32], len: usize, target: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = (nums.len() as i32) - 1;

    while i <= j {
        let mid = i + (j - i) / 2;

        let m = nums[mid as usize];

        if m < target {
            i = mid + 1;
        } else if m >= target {
            j = mid - 1;
        }
    }

    if j >= len as i32 {
        -1
    } else {
        j
    }
}

pub fn search_big(nums: &[i32], len: usize, target: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = (nums.len() as i32) - 1;

    while i <= j {
        let mid = i + (j - i) / 2;

        match target.cmp(&nums[mid as usize]) {
            Ordering::Greater => {
                i = mid + 1;
            }
            Ordering::Less => {
                j = mid - 1;
            }
            Ordering::Equal => {
                i = mid + 1;
            }
        }
    }

    if i == 0 {
        return -1;
    }

    if i >= len as i32 {
        len as i32
    } else {
        i
    }
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    let mut i: i32 = 0;
    let mut j: i32 = (nums.len() as i32) - 1;

    let mut r = -1;
    while i <= j {
        let mid = i + (j - i) / 2;

        match target.cmp(&nums[mid as usize]) {
            Ordering::Greater => {
                i = mid + 1;
            }
            Ordering::Less => {
                j = mid - 1;
            }
            Ordering::Equal => {
                r = mid;
                break;
            }
        }
    }

    if r == -1 {
        v.push(-1);
        v.push(-1);

        return v;
    }

    let i = search_small(&nums, nums.len(), target);
    let j = search_big(&nums, nums.len(), target);

    v.push(i + 1);
    v.push(j - 1);

    v
}

pub fn search_insert_first(first: &[i32], target: i32) -> i32 {
    let i = search_insert(first, target);

    if i == 0 {
        if target == first[0] {
            return 0;
        } else {
            return -1;
        }
    }

    let len = first.len();
    if i >= (len as i32) {
        return i - 1;
    }

    if target == first[i as usize] {
        i
    } else {
        i - 1
    }
}

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut first: Vec<i32> = Vec::new();

    let mut j = 0;
    while j < matrix.len() {
        first.push(matrix[j][0]);
        j += 1;
    }

    let i = search_insert_first(&first, target);
    if i == -1 {
        return false;
    }

    let k = search(&matrix[i as usize], target);

    k != -1
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
        check_q_sort_suits(quick_sort);
    }

    #[test]
    fn check_heap_sort() {
        check_sort_suits(heap_sort);
    }

    #[test]
    fn check_merge_sort() {
        check_sort_suits(merge_sort);
    }

    fn check_q_sort_suits(sort_fn: fn(a: &mut [i32])) {
        diff_qsort(
            &mut vec![3, 5, 8, 10, 0, 0, 0, 0, 0, 0],
            &vec![0, 0, 0, 0, 0, 0, 3, 5, 8, 10],
            sort_fn,
        );

        diff_qsort(
            &mut vec![5, 1, 26, 37, 61, 11, 15, 19, 59, 48],
            &vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61],
            sort_fn,
        );

        diff_qsort(
            &mut vec![26, 5, 37, 1, 61, 11, 59, 15, 48, 19],
            &vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61],
            sort_fn,
        );

        diff_qsort(
            &mut vec![11, 7, 20, 9, 18, 21, 19, 8, 10, 22],
            &vec![7, 8, 9, 10, 11, 18, 19, 20, 21, 22],
            sort_fn,
        );

        diff_qsort(&mut vec![1, 5], &vec![1, 5], sort_fn);

        diff_qsort(&mut vec![0], &vec![0], sort_fn);
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

    fn diff_qsort(sorted: &mut Vec<i32>, expected: &Vec<i32>, sort_fn: fn(a: &mut [i32])) {
        sort_fn(sorted);
        assert_eq!(sorted, expected);
    }

    fn diff_sort(sorted: &mut Vec<usize>, expected: &Vec<usize>, sort_fn: fn(a: &mut [usize])) {
        sort_fn(sorted);
        assert_eq!(sorted, expected);
    }

    #[test]
    fn check_adjust_heap() {
        let mut a = vec![26, 7, 72, 6, 4, 63, 15, 8];
        let b = vec![72, 7, 63, 6, 4, 26, 15, 8];

        let n = a.len();
        adjust_heap(&mut a, 0, n);

        assert_eq!(a, b);
    }

    #[test]
    fn check_inner_merge_1() {
        let mut a = vec![1, 11, 15, 37, 59, 61, 5, 19, 26, 48];
        let mut b = vec![0; a.len()];

        let c = vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61];

        inner_merge(&mut a, &mut b, 0, 5, 9);
        assert_eq!(b, c);
    }

    #[test]
    fn check_inner_merge_2() {
        let mut a = vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61];
        let mut b = vec![0; a.len()];

        let c = vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61];

        inner_merge(&mut a, &mut b, 0, 9, 9);
        assert_eq!(b, c);
    }

    #[test]
    fn check_inner_merge_3() {
        let mut a = vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61];
        let mut b = vec![0; a.len()];

        let c = vec![1, 5, 11, 15, 19, 26, 37, 48, 59, 61];

        inner_merge(&mut a, &mut b, 0, 0, 9);
        assert_eq!(b, c);
    }

    #[test]
    fn check_merge_pass() {
        let mut a = vec![26, 5, 37, 1, 61, 11, 59, 15, 48, 19];
        let mut b = vec![0; a.len()];

        merge_pass(&mut a, &mut b, 1);

        let c_1 = vec![5, 26, 1, 37, 11, 61, 15, 59, 19, 48];
        assert_eq!(b, c_1);

        merge_pass(&mut b, &mut a, 2);
        let c_2 = vec![1, 5, 26, 37, 11, 15, 59, 61, 19, 48];
        assert_eq!(a, c_2);
    }

    #[test]
    fn check_three_sum_1() {
        let b = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(b.len(), 2);
        assert_eq!(b[0], vec![-1, -1, 2]);
        assert_eq!(b[1], vec![-1, 0, 1]);
    }

    #[test]
    fn check_three_sum_2() {
        let b = three_sum(vec![1, 1, 1]);
        assert_eq!(b.len(), 0);
    }

    #[test]
    fn check_three_sum_3() {
        let b = three_sum(vec![0, 0, 0]);
        assert_eq!(b.len(), 1);
        assert_eq!(b[0], vec![0, 0, 0]);
    }

    #[test]
    fn check_three_sum_4() {
        let b = three_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4]);
        assert_eq!(b.len(), 9);
        assert_eq!(b[0], vec![-4, 0, 4]);
        assert_eq!(b[1], vec![-4, 1, 3]);
    }

    #[test]
    fn check_search_insert() {
        let v = vec![1, 3, 5, 6];

        assert_eq!(search_insert(&v, 3), 1);
        assert_eq!(search_insert(&v, 5), 2);
        assert_eq!(search_insert(&v, 2), 1);
        assert_eq!(search_insert(&v, 7), 4);
        assert_eq!(search_insert(&v, 1), 0);
    }

    #[test]
    fn check_search() {
        let v = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(search(&v, 9), 4);
        assert_eq!(search(&v, 2), -1);

        assert_eq!(search(&[5], -5), -1);
    }

    #[test]
    fn check_search_matrix() {
        assert!(!search_matrix(vec![vec![1]], 0));
        assert!(search_matrix(vec![vec![1]], 1));

        assert!(search_matrix(
            vec![vec![2, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            2
        ));

        assert!(search_matrix(
            vec![vec![2, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            30
        ));

        assert!(!search_matrix(
            vec![vec![2, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            21
        ));
    }

    #[test]
    fn check_search_big() {
        let v = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(search_big(&v, v.len(), 9), 5);
        assert_eq!(search_big(&v, v.len(), 2), 2);
        assert_eq!(search_big(&v, v.len(), 5), 4);
    }

    #[test]
    fn check_search_big_2() {
        let v = vec![-1, 0, 3, 5, 9, 9, 12];
        assert_eq!(search_big(&v, v.len(), 9), 6);
        assert_eq!(search_big(&v, v.len(), 12), 7);
        assert_eq!(search_big(&v, v.len(), 5), 4);

        assert_eq!(search_big(&v, v.len(), -1), 1);
        assert_eq!(search_big(&v, v.len(), -2), -1);
    }

    #[test]
    fn check_search_small() {
        let v = vec![-1, 0, 3, 5, 9, 9, 12];
        assert_eq!(search_small(&v, v.len(), 9), 3);
        assert_eq!(search_small(&v, v.len(), 12), 5);
        assert_eq!(search_small(&v, v.len(), 5), 2);

        assert_eq!(search_small(&v, v.len(), -1), -1);
        assert_eq!(search_small(&v, v.len(), -2), -1);
    }

    #[test]
    fn check_search_small_1() {
        let v = vec![1, 4];
        assert_eq!(search_small(&v, v.len(), 4), 0);
    }

    #[test]
    fn check_search_range() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    }

    #[test]
    fn check_search_range_2() {
        assert_eq!(search_range(vec![1, 4], 4), vec![1, 1]);
        assert_eq!(search_range(vec![1], 1), vec![0, 0]);
    }
}
