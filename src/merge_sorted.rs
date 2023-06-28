//https://leetcode.com/problems/merge-sorted-array/

#![allow(dead_code)]

pub mod merge_sorted{ 

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = (m+n) - 1;
        let mut m = m-1;
        let mut n = n-1;
        while m >= 0 && n >= 0 {
            if nums1[m as usize] > nums2[n as usize] {
                nums1[i as usize] = nums1[m as usize];
                m -= 1;
            } else if nums1[m as usize] < nums2[n as usize] {
                nums1[i as usize] = nums2[n as usize];
                n -= 1;
            }
            else {
                nums1[i as usize] = nums2[n as usize];
                m -= 1;
            }
            i -= 1;
        }
        if m == -1 && n >= 0 {
            while n >= 0 && i >= 0 {
                nums1[i as usize] = nums2[n as usize];
                n -= 1;
                i -= 1;
            }
        }
    } 

}

#[cfg(test)]
mod tests {
    use super::merge_sorted::{merge};

    #[test]
    fn test_profit() {
        let mut nums1 : Vec<i32> = vec![1,2,3,0,0,0];
        let m = 3;
        let mut nums2 : Vec<i32> = vec![2,5,6];
        let n = 3;
        merge(& mut nums1, m, & mut nums2, n);
        assert_eq!(vec![1,2,2,3,5,6], nums1);
    }

    #[test]
    fn test_profit1() {
        let mut nums1 : Vec<i32> = vec![1,2,0,0];
        let m = 2;
        let mut nums2 : Vec<i32> = vec![3,4];
        let n = 2;
        merge(& mut nums1, m, & mut nums2, n);
        assert_eq!(vec![1,2,3,4], nums1);
    }

    #[test]
    fn test_profit2() {
        let mut nums1 : Vec<i32> = vec![3,4,0,0];
        let m = 2;
        let mut nums2 : Vec<i32> = vec![1,2];
        let n = 2;
        merge(& mut nums1, m, & mut nums2, n);
        assert_eq!(vec![1,2,3,4], nums1);
    }

    #[test]
    fn test_profit3() {
        let mut nums1 : Vec<i32> = vec![0];
        let m = 0;
        let mut nums2 : Vec<i32> = vec![1];
        let n = 1;
        merge(& mut nums1, m, & mut nums2, n);
        assert_eq!(vec![1], nums1);
    }

    #[test]
    fn test_profit4() {
        let mut nums1 : Vec<i32> = vec![1];
        let m = 1;
        let mut nums2 : Vec<i32> = vec![];
        let n = 0;
        merge(& mut nums1, m, & mut nums2, n);
        assert_eq!(vec![1], nums1);
    }
}
