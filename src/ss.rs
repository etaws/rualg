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
    }
}
