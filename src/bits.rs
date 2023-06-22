pub const BITS_WIDTH: usize = 8;

pub struct Bits {
    len: usize,
    vec: Vec<u8>,
}

impl Bits {
    pub fn new(capacity: usize) -> Bits {
        let bits_num = capacity / BITS_WIDTH + 1;

        let mut b = Bits {
            len: capacity,
            vec: Vec::with_capacity(bits_num),
        };

        for _i in 0..bits_num {
            b.vec.push(0);
        }

        b
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn set(&mut self, i: usize) {
        let n = i / BITS_WIDTH;
        let m = i % BITS_WIDTH;

        let mask = 1 << m;

        self.vec[n] |= mask;
    }

    pub fn unset(&mut self, i: usize) {
        let n = i / BITS_WIDTH;
        let m = i % BITS_WIDTH;

        let mask = 1 << m;

        self.vec[n] &= !mask;
    }

    pub fn get(&self, i: usize) -> bool {
        let n = i / BITS_WIDTH;
        let m = i % BITS_WIDTH;

        let mask = 1 << m;

        let r = self.vec[n] & mask;

        r == mask
    }
}

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut v: Vec<Vec<i32>> = Vec::new();

    let mut b = Bits::new(10000);

    for vs in intervals.into_iter() {
        if vs.len() != 2 {
            continue;
        }

        let i = vs[0];
        let j = vs[1];

        if i < 0 || j < 0 {
            continue;
        }

        let mut k = i;
        while k <= j {
            b.set(k as usize);
            k += 1;
        }
    }

    let mut in_it = false;
    let mut s = 0;
    for t in 0..10000 {
        if !in_it {
            if b.get(t) {
                s = t;
                in_it = true;
            }
        } else if !b.get(t) {
            let cv: Vec<i32> = vec![s as i32, (t - 1) as i32];
            v.push(cv);
            in_it = false;
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_create() {
        let mut b = Bits::new(10);

        assert_eq!(b.len(), 10);

        b.set(0);

        assert!(b.get(0));

        b.unset(0);

        assert!(!b.get(0));
    }

    #[test]
    fn check_later() {
        let mut b = Bits::new(10000);

        assert_eq!(b.len(), 10000);

        assert!(!b.get(999));

        b.set(999);

        assert!(b.get(999));

        b.unset(999);

        assert!(!b.get(999));
    }

    #[test]
    fn check_merge_1() {
        let v: Vec<Vec<i32>> = vec![vec![1, 4], vec![4, 5]];

        let r = merge(v);

        let er: Vec<Vec<i32>> = vec![vec![1, 5]];
        assert_eq!(r, er);
    }

    #[test]
    fn check_merge() {
        let v: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];

        let r = merge(v);

        let er: Vec<Vec<i32>> = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(r, er);
    }
}
