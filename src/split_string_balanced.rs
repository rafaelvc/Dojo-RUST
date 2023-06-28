// https://leetcode.com/problems/split-a-string-in-balanced-strings/
#![allow(dead_code)]

pub mod split_string_balanced { 
    pub fn balanced_strs(s: &str) -> i32 {
        let mut balance = -1;
        let mut look_for = ' ';
        let mut substrs_balanced = 0;
        for c in s.chars() {
            if balance == 0 || balance == -1 {
                if balance == 0 { substrs_balanced += 1 }
                match c {
                    'R' => look_for = 'L',
                    'L' => look_for = 'R',
                    _ => panic!("Invalid letter {}", c)
                };
                balance = 1;
            }
            else if look_for == c { balance -= 1 }
            else {                  balance += 1 }
        }
        if balance == 0 { substrs_balanced += 1 }
        substrs_balanced
    }
}

#[cfg(test)]
mod tests {
    use super::split_string_balanced::balanced_strs;
    #[test]
    fn test_balanced_strs() {
        //assert_eq!(4, balanced_strs("RLRRLLRLRL".to_string()));
        assert_eq!(4, balanced_strs("RLRRLLRLRL"));
        assert_eq!(2, balanced_strs("RLRRRLLRLL"));
        assert_eq!(1, balanced_strs("LLLLRRRR"));
    }
    #[test]
    #[should_panic]
    fn test_panic() {
        balanced_strs("AB");
    }
}