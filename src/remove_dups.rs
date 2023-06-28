//https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/submissions/
#![allow(dead_code)]

pub mod remove_dups { 
    pub fn remove (nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 3 {return nums.len() as i32}
        let mut prev = nums[0];
        let mut count = 2;
        let mut pos = 0;
        for i in 0..nums.len() {
            if prev == nums[i] {
                if count == 0 { continue }
                nums[pos] = nums[i]; 
                count -= 1;
            }
            else {
                count = 1;
                nums[pos] = nums[i]; 
                prev = nums[i];
            }
            pos += 1;
            dbg!(pos, count, i);
        }
        pos as i32
    }
}

#[cfg(test)]
mod tests {
    use super::remove_dups::remove;
    #[test]
    fn test_remove_dups() {
        let mut nums: Vec<i32> = vec![1,1,1,2,2,3];
        assert_eq!(5, remove(&mut nums));
        assert_eq!(vec![1,1,2,2,3][..], nums[..5]);
        nums = vec![1,1,1,1,2,2,2,3,4];
        assert_eq!(6, remove(&mut nums));
        assert_eq!(vec![1,1,2,2,3,4][..], nums[..6]);
        nums = vec![0,0,1,1,1,1,2,3,3];
        assert_eq!(7, remove(&mut nums));
        assert_eq!(vec![0,0,1,1,2,3,3][..], nums[..7]);
        dbg!(nums);
        //assert_eq!(vec![1,1,2,2,3,4][..], nums[..6]);
    }
}