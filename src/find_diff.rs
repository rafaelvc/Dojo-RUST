//https://leetcode.com/problems/find-the-difference/

#![allow(dead_code)]

pub mod find_diff { 
    //use std::iter::repeat;
    pub fn has_diff(s: String, t: String) -> char {
        //let f: [i8; 26] = repeat(0).take(26).collect();
        let mut f = vec![0; 26];
        let a = 'a' as usize;
        for t1 in t.chars(){
            f[(t1 as usize) - a] += 1; 
        }
        for s1 in s.chars() {
            f[(s1 as usize) - a] -= 1; 
        }
        let a = a as u8;
        for i in 0..26 {
            if f[i] == 1 { return (a + (i as u8)) as char }
        }
        ' '
    }
}

#[cfg(test)]
mod tests {
    use super::find_diff::has_diff;
    #[test]
    fn test_has_diff() {
        assert_eq!(true, true);
        assert_eq!(' ', has_diff("aaa".to_string(), "bbb".to_string()));
        assert_eq!('e', has_diff("abcd".to_string(), "abcde".to_string()));
        assert_eq!('y', has_diff("".to_string(), "y".to_string()));
        assert_eq!('a', has_diff("a".to_string(), "aa".to_string()));
    }
}