// https://leetcode.com/problems/split-a-string-in-balanced-strings/
#![allow(dead_code)]

pub mod last_word_len { 
    pub fn lw_len(s: &str) -> i32 {
        let mut lw_counter = 0;
        //let sentence: Vec<char> = s.chars().collect();
        //let mut i = sentence.len() - 1;
        //while sentence[i] == ' ' && i >= 0 { i -= 1 }
        //while i >= 0 {
        //    if sentence[i] == ' ' {break}
        //    lw_counter += 1;
        //    i -= 1;
        //}

        for c in s.chars().rev() {
            if c == ' ' && lw_counter == 0 { continue }
            else if c != ' ' {lw_counter += 1}
            else {break}
        }

        //for i in (0..sentence.len()).rev() {
        //    if sentence[i] == ' ' && lw_counter == 0 { continue }
        //    else if sentence[i] != ' ' {lw_counter += 1}
        //    else {break}
        //}

        //loop {
        //    if sentence[i] != ' ' || i == 0  {break}
        //    i -= 1;
        //}
        //loop {
        //    if sentence[i] == ' ' || i < 0 {break}
        //    lw_counter += 1;
        //    i -= 1;
        //}
        lw_counter
    }
}

#[cfg(test)]
mod tests {
    use super::last_word_len::lw_len;
    #[test]
    fn test_lw_len() {
        assert_eq!(5, lw_len("Hello World            "));
        assert_eq!(4, lw_len("   fly me   to   the moon  "));
        assert_eq!(6, lw_len("Rafael"));
        assert_eq!(1, lw_len("a"));
        assert_eq!(0, lw_len(""));
    }
}