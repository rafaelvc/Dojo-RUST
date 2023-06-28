//https://leetcode.com/problems/find-the-difference/

#![allow(dead_code)]



struct MyData<'a> {
    param1 : &'a Vec<i32>
}

pub mod lifetimes { 


    pub fn from_second_generic_<'a, T>(x: &'a [T], y: &'a [T]) -> &'a [T] {
        &x[1..]
    }
 

    pub fn from_second_generic<T>(x: &[T]) -> &[T] {
        &x[1..]
    }
    
    //use std::mem; // CHAT gpt
    //pub fn from_second_generic<T, U>(x: &[T]) -> &[U] {
    //    let ptr = x.as_ptr();
    //    let len = x.len();
    //    let size = mem::size_of::<T>();
    //    let new_size = len - 1;
    //    let new_len = new_size * size / mem::size_of::<U>();
    //    unsafe {
    //        mem::transmute::<&[T], &[U]>(core::slice::from_raw_parts(ptr.add(size), new_len))
    //    }
    //}

    pub fn from_second(x: &Vec<i32>)  -> &[i32] {
        &x[1..]
    }

    pub fn func_params<'a>(x : &'a mut Vec<i32>, y: &'a Vec<i32>, z: bool) -> &'a Vec<i32> {
        x[0] = 5;
        if x[0] > y[0] && z { return x }
        y
    }

    pub fn some_tests() {
        let mut x = vec![1,2,4,6];
        let mut z = vec!["a".to_string(), String::from("b")];
        let y = vec![3,4];
        print!("{:?}", func_params(& mut x, &y, true));
        print!("{:?}", from_second(&x));
        //print!("{:?}", from_second_generic::<i32>(&x));
        print!("{:?}", from_second_generic(&x));
        print!("{:?}", from_second_generic(&z));
    }
}

#[cfg(test)]
mod tests {
    use super::lifetimes::{some_tests, from_second_generic};
    #[test]
    fn test_lifetimes() {
        //some_tests();
        //assert_eq!(true, true);
        
        // Chat GPT unit tests

        // Test with a vector of integers
        let vec1 = vec![1, 2, 3, 4, 5];
        let result1 = from_second_generic(&vec1);
        assert_eq!(result1, &[2, 3, 4, 5]);

        // Test with a vector of strings
        let vec2 = vec!["hello", "world", "rust"];
        let result2 = from_second_generic(&vec2);
        assert_eq!(result2, &["world", "rust"]);

        // Test with an empty vector
        //let vec3: Vec<i32> = vec![];
        //let result3 = from_second_generic(&vec3);
        //assert_eq!(result3, &[]);
    
        //// Test with a single element vector
        //let vec4 = vec![10];
        //let result4 = from_second_generic(&vec4);
        //assert_eq!(result4, &[]);
    }
}