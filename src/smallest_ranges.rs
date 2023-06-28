//https://leetcode.com/problems/summary-ranges/
#![allow(dead_code)]

pub mod smallest_ranges{ 
    pub fn ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }
        let mut result: Vec<String> = Vec::new();
        let mut range = nums[0].to_string();
        let mut expected_next = nums[0] + 1;
        let mut range_start = nums[0];
        for i in 1..nums.len() {
            if nums[i] == expected_next { expected_next += 1 }
            else {
                if nums[i-1] != range_start {
                    range.push_str(&"->".to_string());
                    range.push_str(&(nums[i-1].to_string()));
                }
                result.push( range );
                range = nums[i].to_string();
                expected_next = nums[i] + 1;
                range_start = nums[i];
            }
        }
        if nums[nums.len()-1] != range_start {
            range.push_str(&"->".to_string());
            range.push_str(&(nums[nums.len()-1].to_string()));
        }
        result.push( range );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::smallest_ranges::ranges;
    #[test]
    fn test_ranges() {
        assert_eq!(vec!["0->2".to_string(),"4->5".to_string(),"7".to_string()], ranges( vec![0,1,2,4,5,7] ) );
        assert_eq!(vec!["0".to_string(),"2->4".to_string(),"6".to_string(),"8->9".to_string()], ranges( vec![0,2,3,4,6,8,9] ) );
    }
}