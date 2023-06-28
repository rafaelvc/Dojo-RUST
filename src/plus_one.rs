//https://leetcode.com/problems/plus-one/
#![allow(dead_code)]

pub mod plus_one { 
    pub fn calc(digits: Vec<i32>) -> Vec<i32> {
        let mut digits_plus_one = digits;
        for i in (0..digits_plus_one.len()).rev() {
            let sum = digits_plus_one[i] + 1;
            if sum < 10 { digits_plus_one[i] = sum; return digits_plus_one; }
            digits_plus_one[i] = sum % 10
        }
        digits_plus_one.insert(0, 1);
        digits_plus_one
        //digits_plus_one.into_iter().rev().map( | x | if x + 1 > 9 { 0 }  )
        //dbg!(Some(start.next()), Some(end.next()));
        //dbg!(Some(start.next()), Some(end.next()));
    }
}

#[cfg(test)]
mod tests {
    use super::plus_one::{calc};

    #[test]
    fn test_calc() {
        assert_eq!(vec![1,3], calc(vec![1,2]));
        assert_eq!(vec![2,0], calc(vec![1,9]));
        assert_eq!(vec![2,0,0,0], calc(vec![1,9,9,9]));
        assert_eq!(vec![1,0,0], calc(vec![9,9]));
    }
}

