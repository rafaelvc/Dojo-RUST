//https://leetcode.com/problems/palindrome-number/
#![allow(dead_code)]

pub mod palindorme_nr { 
    pub fn is_palindrome(x: i32) -> bool {
        let str1 = x.to_string();
        let mut start = str1.chars();
        let mut end = str1.chars().rev();
        let max_iters = str1.len() / 2;
        let mut count_iter = 0;
        while count_iter < max_iters {
            let a = start.next().unwrap_or('a');
            let b = end.next().unwrap_or('b');
            if a != b {return false}
            count_iter += 1;
        }
        true   
        //dbg!(Some(start.next()), Some(end.next()));
        //dbg!(Some(start.next()), Some(end.next()));
    }
}

#[cfg(test)]
mod tests {
    use super::palindorme_nr::is_palindrome;
    #[test]
    fn test_palindrome() {
        assert_eq!(true, true);
        assert_eq!(true, is_palindrome(1221));
        assert_eq!(true, is_palindrome(121));
        assert_eq!(true, is_palindrome(11));
        assert_eq!(false, is_palindrome(10));
        //assert_eq!(vec![1,1,2,2,3,4][..], nums[..6]);
    }
}

