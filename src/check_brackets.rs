#![allow(dead_code)]

pub mod check_brackets { 
    use std::collections::VecDeque;
    pub fn check(s: &str) -> bool {
        let mut brack_stack : VecDeque<char> = VecDeque::new();
        for c in s.chars() {
            if c == '{' || c == '[' || c == '(' {
                brack_stack.push_back(c);
            } else if brack_stack.len() == 0 { return false; }
            else {
                let c1 = brack_stack.pop_back().unwrap_or(' ');
                if c == '}' && c1 != '{' ||
                   c == ')' && c1 != '(' ||
                   c == ']' && c1 != '[' { return false; }
            }
//          let brack_stack_dbg = brack_stack.clone();
//          dbg!(brack_stack_dbg);
        }
        if brack_stack.len() == 0 { return true; }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::check_brackets::*;
    #[test]
    fn test_check() {
       assert_eq!(true, check("({{{}{}}})"));
       assert_eq!(true, check("()[]{}"));
       assert_eq!(false, check("("));
    }
}
