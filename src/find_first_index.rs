// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/

#![allow(dead_code)]

pub mod find_first_index { 
    pub fn get_index(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() { return -1 }
        let hchars = haystack.chars();
        let mut i = 0;
        let start = needle.chars().next().unwrap();
        for c in hchars {
            if  i + needle.len() > haystack.len() {return -1}
            if c == start {
                let sub = &haystack[i..i+needle.len()];
                if sub.eq(&needle) {return i as i32}
            }
            i += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::find_first_index::get_index;
    #[test]
    fn test_first_index() {
        assert_eq!(true, true);
        assert_eq!(6, get_index("zadbutsad".to_string(), "sad".to_string()));
        assert_eq!(-1, get_index("xad".to_string(), "sad".to_string()));
        assert_eq!(0, get_index("sadbutsad".to_string(), "sad".to_string()));
        assert_eq!(3, get_index("xadsadsad".to_string(), "sad".to_string()));
    }
}