#![allow(dead_code)]

pub mod contain_dups { 
    use std::collections::HashMap;
    use std::collections::HashSet;
    pub fn contains_nearby_duplicates(nums: Vec<i32>, k: i32) -> bool {
        let mut positions: HashMap<i32, i32> = HashMap::new();
        for (mut i, v) in nums.iter().enumerate() {
            i += 1;
            if let Some(&j) = positions.get(&v)  {
                if (i as i32 - j).abs() <= k {return true}
            }
            positions.insert(*v, i as i32);
        }
        false
    }

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hs: HashSet<i32> = HashSet::new();
        for n in nums {
            if hs.contains(&n) {return true}
            else {hs.insert(n);}
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use super::contain_dups::*;
    #[test]
    fn test_contain_nearby_dups() {
        assert_eq!(true, contains_nearby_duplicates(vec![1,2,3,1], 3));
        assert_eq!(true, contains_nearby_duplicates(vec![1,0,1,1], 1));
        assert_eq!(false, contains_nearby_duplicates(vec![1,2,3,1,2,3], 2));
    }
    #[test]
    fn test_contain_dups() {
        assert_eq!(true, contains_duplicate(vec![1,2,3,1]));
        assert_eq!(false, contains_duplicate(vec![1,2,3,4]));
        assert_eq!(true, contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));
    }
}