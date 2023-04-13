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

pub struct NumMatrix {
    pub sums: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sums: Vec<Vec<i32>> = Vec::with_capacity(matrix.len());

        // i 是这个 Matrix 有多少行，就按多少行来遍历
        for i in 0..matrix.len() {
            sums.push(vec![0; matrix[i].len()]);
            let mut sum = 0;
            // j 是这个 Matrix 有多少列，在第 i 行中按列来遍历
            for j in 0..matrix[i].len() {
                if i == 0 && j == 0 {
                    sums[i][j] = matrix[i][j];
                    sum = matrix[i][j];
                } else if i == 0 && j > 0 {
                    // 第一行，累加
                    sum += matrix[i][j];
                    sums[i][j] = sum;
                } else if i > 0 && j == 0 {
                    // 第一列，累加
                    let cell = sums[i - 1][j];
                    sums[i][j] = matrix[i][j] + cell;
                } else {
                    let cell = sums[i - 1][j] + sums[i][j - 1] - sums[i - 1][j - 1];
                    sums[i][j] = matrix[i][j] + cell;
                }
            }
        }

        NumMatrix { sums }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let i1: usize = row1 as usize;
        let j1: usize = col1 as usize;
        let i2: usize = row2 as usize;
        let j2: usize = col2 as usize;

        if i1 == 0 && j1 == 0 {
            return self.sums[i2][j2];
        }

        if i1 == 0 && j1 > 0 {
            return self.sums[i2][j2] - self.sums[i2][j1 - 1];
        }

        if i1 > 0 && j1 == 0 {
            return self.sums[i2][j2] - self.sums[i1 - 1][j2];
        }

        let a = self.sums[i2][j2];
        let b = self.sums[i1 - 1][j2];
        let c = self.sums[i2][j1 - 1];
        let d = self.sums[i1 - 1][j1 - 1];

        a - b - c + d
    }
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

    #[test]
    fn check_sum() {
        let v: Vec<Vec<i32>> = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];

        let my_matrix: NumMatrix = NumMatrix::new(v);

        assert_eq!(my_matrix.sums.len(), 5);

        assert_eq!(my_matrix.sums[0][1], 3);
        assert_eq!(my_matrix.sums[0][2], 4);
        assert_eq!(my_matrix.sums[0][3], 8);
        assert_eq!(my_matrix.sums[0][4], 10);

        assert_eq!(my_matrix.sums[4][0], 14);
        assert_eq!(my_matrix.sums[4][1], 23);
        assert_eq!(my_matrix.sums[4][2], 30);
        assert_eq!(my_matrix.sums[4][3], 38);
        assert_eq!(my_matrix.sums[4][4], 58);

        assert_eq!(my_matrix.sum_region(0, 0, 0, 0), 3);
        assert_eq!(my_matrix.sum_region(0, 1, 0, 2), 1);
        assert_eq!(my_matrix.sum_region(2, 1, 4, 3), 8);
        assert_eq!(my_matrix.sum_region(1, 1, 2, 2), 11);
        assert_eq!(my_matrix.sum_region(1, 2, 2, 4), 12);
    }
}
