struct MinStack {
    #[allow(dead_code)]
    elements: Vec<(i32, i32)>,
}

impl MinStack {
    #[allow(dead_code)]
    fn new() -> Self {
        MinStack {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.elements.is_empty() {
            self.elements.push((val, val));
        } else {
            let top_min = self.elements.last().unwrap().1;
            if top_min < val {
                self.elements.push((val, top_min));
            } else {
                self.elements.push((val, val));
            }
        }
    }

    fn pop(&mut self) {
        if self.elements.is_empty() {
            return;
        }

        self.elements.pop();
    }

    fn top(&self) -> i32 {
        if self.elements.is_empty() {
            0
        } else {
            let top_v = self.elements.last().unwrap().0;
            top_v
        }
    }

    fn get_min(&self) -> i32 {
        if self.elements.is_empty() {
            0
        } else {
            let top_min = self.elements.last().unwrap().1;
            top_min
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_create() {
        let mut ms = MinStack::new();
        assert_eq!(ms.get_min(), 0);

        ms.push(5);
        assert_eq!(ms.get_min(), 5);

        ms.push(6);
        assert_eq!(ms.get_min(), 5);
        assert_eq!(ms.top(), 6);

        ms.pop();
        assert_eq!(ms.get_min(), 5);
        assert_eq!(ms.top(), 5);
    }
}
