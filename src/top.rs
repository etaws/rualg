use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hash: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs.into_iter() {
        let ss = sort_s(&s);

        if hash.get(&ss).is_none() {
            let v = vec![s];
            hash.insert(ss, v);
        } else {
            let v = hash.get_mut(&ss).unwrap();
            v.push(s);
        }
    }

    hash.into_values().collect()
}

pub fn sort_s(str: &str) -> String {
    let mut l: Vec<char> = str.chars().collect();
    l.sort_unstable();

    l.into_iter().collect()
}

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let offset: i32 = 10000;

    let mut a: Vec<i32> = vec![0; 20001];
    for &n in nums.iter() {
        a[(n + offset) as usize] += 1;
    }

    let mut i = 0;

    let mut s = a.len() - 1;

    loop {
        if a[s] >= 1 {
            i += a[s];
        }

        if i >= k {
            return (s as i32) - offset;
        }

        if s == 0 {
            break;
        }

        s -= 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_kth_1() {
        assert_eq!(find_kth_largest(vec![-99, -99], 1), -99);
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }

    #[test]
    fn check_group() {
        let v = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        let r = group_anagrams(v);

        assert_eq!(r.len(), 3);

        let v_2 = vec!["ddddddddddg".to_string(), "dgggggggggg".to_string()];
        let r_2 = group_anagrams(v_2);

        assert_eq!(r_2.len(), 2);
    }
}
