pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::with_capacity(s.len());

    for c in s.chars() {
        if c != '[' && c != '{' && c != '(' && c != ']' && c != '}' && c != ')' {
            return false;
        }

        if stack.is_empty() && (c == ']' || c == '}' || c == ')') {
            return false;
        }

        if stack.is_empty() || c == '[' || c == '{' || c == '(' {
            stack.push(c);
        }

        if c == ']' {
            if let Some(last) = stack.pop() {
                if last != '[' {
                    return false;
                }
            } else {
                return false;
            }
        }

        if c == ')' {
            if let Some(last) = stack.pop() {
                if last != '(' {
                    return false;
                }
            } else {
                return false;
            }
        }

        if c == '}' {
            if let Some(last) = stack.pop() {
                if last != '{' {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    if stack.is_empty() {
        return true;
    }

    false
}

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut r = vec![0; temperatures.len()];

    let mut stack: Vec<(i32, usize)> = Vec::with_capacity(temperatures.len());

    for (i, n) in temperatures.iter().enumerate() {
        let v = *n;
        if stack.is_empty() {
            stack.push((v, i));
            continue;
        }

        while !stack.is_empty() {
            let top = stack.last().unwrap();
            if v > top.0 {
                r[top.1] = (i - top.1) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push((v, i));
    }

    r
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut x = 0;

    for n in nums.iter() {
        x ^= *n;
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_valid() {
        // positive
        assert!(is_valid("{}".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(is_valid("{[{[{}]}]}".to_string()));

        // negative
        assert!(!is_valid("(]".to_string()));
        assert!(!is_valid("{[{[{}]".to_string()));
        assert!(!is_valid("{(})".to_string()));
    }

    #[test]
    fn check_daily_temperatures() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );

        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);

        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }
}
