//https://leetcode.com/problems/set-mismatch/
#![allow(dead_code)]

pub mod set_mismatch{ 
    // Sort the vec then check for first copy and the missing
    // O(N) = nlogn + n
    // This solution beats 53%.  
    // nums is moved to nums1 as Rust ownership concept
    // I can do it O(N) using Hashmap where h(n) = 0 is the missing and h(n) = 2 is the repeated one
    pub fn get_missing(nums: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums;
        nums1.sort();
        let mut missing = 1;
        let mut repeated = 0;
        let mut prev = 0;
        let mut missing_found = false;
        for n in &nums1 {

            if repeated == 0 && prev == *n {
                repeated = *n;
                if missing_found {break}
                continue;
            }
            else  { prev = *n; }
            
            if *n != missing { 
                missing_found = true;
                if repeated != 0 {break}
            }
            else { missing += 1; }

        }
        if !missing_found  {missing = nums1.len() as i32;}
        vec![repeated, missing]
    }
}

#[cfg(test)]
mod tests {
    use super::set_mismatch::get_missing;
    #[test]
    fn test_missing() {
        assert_eq!(true, true);
        assert_eq!(vec![2,3], get_missing(vec![1,2,2,4]));
        assert_eq!(vec![1,2], get_missing(vec![1,1]));
        assert_eq!(vec![2,1], get_missing(vec![2,2]));
        assert_eq!(vec![2,1], get_missing(vec![2,3,2]));
        assert_eq!(vec![2,1], get_missing(vec![3,2,2]));
        assert_eq!(vec![2,10], get_missing(vec![1,5,3,2,2,7,6,4,8,9]));
        assert_eq!(vec![2,9], get_missing(vec![1,5,3,2,2,7,6,4,8,10]));
    }
}