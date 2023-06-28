//https://leetcode.com/problems/single-number

#![allow(dead_code)]

pub mod single_number { 
    // The brute force
    pub fn get_single1(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0] }
        for i in 0..nums.len() {
            let mut found = false;
            for j in 0..nums.len() {
                if j == i {continue}
                else if nums[i] == nums[j] { 
                    found = true; break 
                }
            }
            if !found { return nums[i] }
        }
        0
    }

    use std::collections::HashMap;
    pub fn get_single(nums: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, i8> = HashMap::new();
        for v in nums {
            if hm.contains_key(&v) {
                hm.insert(v, hm.get(&v).unwrap() + 1 as i8);
            }
            else { hm.insert(v, 1 as i8);}
        }
        for (k,v) in hm.iter() {
            if *v == 1 {return *k}
        }
        0
    }

 
}

#[cfg(test)]
mod tests {
    use super::single_number::get_single;
    #[test]
    fn test_get_single() {
        assert_eq!(1, get_single(vec![2,2,1]));
        assert_eq!(4, get_single(vec![4,1,2,1,2]));
        assert_eq!(1, get_single(vec![1]));
        assert_eq!(true, true);
    }
}