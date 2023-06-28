#![allow(dead_code)]

pub mod bin_search {
    //use std::*;
    pub fn search(target: i32, nums: &Vec<i32>) -> bool {
        if nums.len() == 0 {return false;}
        let mut l = 0;
        let mut r : i32 = (nums.len() - 1) as i32;
        let mut mid;
        //println! ("{:?} {}", nums, target);
        while l <= r {
            mid = l + ((r - l) / 2);
            // Temporary, we do this because r which is used in mid calc
            // can get negative and there is not negative indexes
            let ix : usize = mid as usize;
            //dbg!(mid, target, nums[ix], l, r);
            if target == nums[ix] {
                return true;
            }
            else if target > nums[ix] {
                l = mid + 1;
            }
            else {
                r = mid - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
//    use super::*;
 //   use bin_search::search;
    use super::bin_search::search;
    #[test]
    fn basic_checkings0() {
        println! ("aaa");
        assert_eq!(false, search(0, &vec![1,2,3,4]));
        assert_eq!(false, search(4, &vec![1,2,3]));
        assert_eq!(false, search(777, &vec![1,2,3]));
        assert_eq!(true, search(5, &vec![1,2,3,5]));
        assert_eq!(true, search(1, &vec![1,2,3,5]));
        assert_eq!(true, search(3, &vec![1,2,3,5]));
        assert_eq!(true, search(3, &vec![1,2,3,5,6]));
        assert_eq!(true, search(6, &vec![1,2,3,5,6]));
        assert_eq!(true, search(1, &vec![1,2,3,5,6]));
        assert_eq!(false, search(4, &vec![1,2,3,5,6]));
    }
}
