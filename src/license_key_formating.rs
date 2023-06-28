//https://leetcode.com/problems/license-key-formatting/
#![allow(dead_code)]

pub mod license_key_formating { 
    use std::collections::VecDeque;
    pub fn license_fmt(s: String, k: i32) -> String {
        // let mut fmt_key = String::new();
        let mut fmt_key = VecDeque::new();
        let mut ck = k;
        //let mut counter = s;
        for c in s.chars().rev() {
            if c == '-' { continue }
            //fmt_key.insert(0, c.to_ascii_uppercase());
            fmt_key.push_front(c.to_ascii_uppercase());
            ck -= 1;
            if ck == 0 {
                //fmt_key.insert(0, '-');
                fmt_key.push_front('-');
                ck = k;
            }
        }
        //if fmt_key.len() > 0 && fmt_key.chars().next() == Some('-') {
        if fmt_key.len() > 0 && fmt_key.get(0) == Some(&'-') {
            //fmt_key.remove(0);
            fmt_key.pop_front();
        }
        let mut result = String::new();
        for c in fmt_key { result.push(c); }
        result 
    }
}

#[cfg(test)]
mod tests {
    use super::license_key_formating::license_fmt;
    #[test]
    fn test_license_fmt() {
        //assert_eq!(String::from("0-1-2-3-4-5-6-7-8-9"), license_fmt(String::from("0123456789"), 1));
        assert_eq!("0-1-2-3-4-5-6-7-8-9".to_string(), license_fmt("0123456789".to_string(), 1));
        assert_eq!("2-5G-3J".to_string(), license_fmt("2-5g-3-J".to_string(), 2));
        assert_eq!("".to_string(), license_fmt("---".to_string(), 1));
        assert_eq!("AA-AA".to_string(), license_fmt("--a-a-a-a--".to_string(), 2));
        assert_eq!("5F3Z-2E9W".to_string(), license_fmt("5F3Z-2e-9-w".to_string(), 4));
                //assert_eq!("", license_fmt("---", 1));
    }
}