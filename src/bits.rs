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
}
