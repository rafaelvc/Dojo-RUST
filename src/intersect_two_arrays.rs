//https://leetcode.com/problems/intersection-of-two-arrays-ii/

#![allow(dead_code)]

pub mod intersect_two_arrays { 
    //use std::collections::HashSet;
    use std::collections::HashMap;
    // Gets intersection but does not match requirements
    //pub fn intersect0(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>{
    //    let mut result: Vec<i32> = Vec::new(); 
    //    if nums1.len() == 0 || nums2.len() == 0 {
    //        return result;
    //    }
    //    let set1: HashSet<i32> = nums1.into_iter().collect();
    //    let set2: HashSet<i32> = nums2.into_iter().collect();
    //    let intersec: HashSet<_> = set1.intersection(&set2).collect();
    //    if intersec.len() == 0 { return result }
    //    let result: Vec<*i32> = intersec.iter().collect();
    //    for &x in intersec {
    //        result.push(x);
    //    }
    //    result
    //}
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new(); 
        if nums1.len() == 0 || nums2.len() == 0 {
            return result;
        }
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for x in nums1 {
            //if let Some(&j) = counter.get(&x) { counter.insert(x, j + 1); () }
            //else { counter.insert(x, 1); () }
            //match counter.get(&x) {
            //    Some(&j) => { counter.insert(x, j + 1); () } ,
            //   None => { counter.insert(x, 1); () }
            //}
            counter.insert(x, counter.get(&x).unwrap_or(&0) + 1);
        }

        for x in nums2 {
            let j = counter.get(&x).unwrap_or(&0);
            if *j > 0 {
                counter.insert(x, j - 1);
                result.push(x);
            }
            //if let Some(&j) = counter.get(&x) {
            //    if j > 0 {
            //        counter.insert(x, j - 1);
            //        result.push(x);
            //    }
            //    ()
            //}
        }
        //dbg!(counter);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::intersect_two_arrays::intersect;
    #[test]
    fn test_intersect() {
        assert_eq!(true, true);
        assert_eq!(vec![2,2], intersect(vec![1,2,2,1], vec![2,2]));
        assert_eq!(vec![9,4], intersect(vec![4,9,5], vec![9,4,9,8,4]));
        assert_eq!(Vec::<i32>::new(), intersect(vec![1,2,2,1], vec![3,3]));
    }
}