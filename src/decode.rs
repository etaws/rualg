pub fn decode_string(s: &str) -> String {
    let mut str_stack: Vec<char> = vec![];

    let mut iter = s.chars();
    let mut i = 0;

    while i < s.len() {
        let c = iter.next().unwrap();
        if !c.is_ascii() {
            panic!("bad str: not acsii")
        }

        if c != ']' {
            str_stack.push(c);
        } else if c == ']' {
            let mut top = str_stack.pop().unwrap();
            let mut inner: Vec<char> = vec![];
            while top != '[' {
                inner.push(top);
                top = str_stack.pop().unwrap();
            }

            if str_stack.is_empty() {
                i += 1;
                continue;
            }

            top = str_stack.pop().unwrap();
            let mut n = 0;
            let mut d = 1;
            loop {
                n += top.to_digit(10).unwrap() * d;
                d *= 10;

                if str_stack.is_empty() || !str_stack.last().unwrap().is_numeric() {
                    break;
                }

                top = str_stack.pop().unwrap();
            }

            let mut j = 0;
            while j < n {
                let mut k = inner.len();
                while k > 0 {
                    str_stack.push(inner[k - 1]);
                    k -= 1;
                }

                j += 1;
            }
        }

        i += 1;
    }

    let mut ii = 0;

    let mut r = "".to_string();
    while ii < str_stack.len() {
        if str_stack[ii].is_numeric() {
            return "".to_string();
        }
        r.push(str_stack[ii]);
        ii += 1;
    }

    r.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_decode_string() {
        assert_eq!(
            decode_string("3[z]2[2[y]pq4[2[jk]e1[f]]]ef"),
            "zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef".to_string()
        );
        assert_eq!(decode_string("10[ab]"), "abababababababababab".to_string());
        assert_eq!(decode_string("3[a]2[bc]"), "aaabcbc".to_string());
        assert_eq!(decode_string("3[a2[c]]"), "accaccacc".to_string());
        assert_eq!(decode_string("2[abc]3[cd]ef"), "abcabccdcdcdef".to_string());
        assert_eq!(decode_string("abc3[cd]xyz"), "abccdcdcdxyz".to_string());
        assert_eq!(decode_string("3"), "".to_string());
    }

    #[test]
    #[should_panic]
    fn check_decode_string_badcases() {
        assert_eq!(decode_string("3[a"), "3a".to_string());
        assert_eq!(decode_string("a[2]"), "a".to_string());
    }
}
