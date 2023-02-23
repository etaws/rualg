use std::collections::HashMap;

pub fn seek_two(a: &[i32], target: i32) -> (usize, usize) {
    let mut s: HashMap<i32, usize> = HashMap::new();

    for (j, n) in a.iter().enumerate() {
        let another = target - n;

        if s.get(&another).is_some() {
            let one = j;
            let two = *(s.get(&another).unwrap());

            // 返回的时候规范一下：总是小的数放在前面
            if one > two {
                return (two, one);
            } else {
                return (one, two);
            }
        } else {
            s.insert(*n, j);
        }
    }

    (0, 0)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_seek_two_1() {
        let a: [i32; 4] = [11, 15, 2, 7];
        let target = 9;

        let r = seek_two(&a, target);

        assert_eq!(r, (2, 3));
    }

    #[test]
    fn check_seek_two_2() {
        let a: [i32; 3] = [3, 2, 4];
        let target = 6;

        let r = seek_two(&a, target);

        assert_eq!(r, (1, 2));
    }

    #[test]
    fn check_seek_two_3() {
        let a: [i32; 2] = [3, 3];
        let target = 6;

        let r = seek_two(&a, target);

        assert_eq!(r, (0, 1));
    }
}
