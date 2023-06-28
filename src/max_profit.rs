//https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
#![allow(dead_code)]

pub mod max_profit { 

    pub fn profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 { return 0 }
        let mut res = 0;
        let mut min = prices[0];
        let mut i = 1;
        while i <= prices.len() - 1 {
            let cur_res = prices[i] - min;
            if cur_res > res {res = cur_res}
            if prices[i] < min {min = prices[i]}
            i += 1;
        }
        res 
    }

}

#[cfg(test)]
mod tests {
    use super::max_profit::{profit};

    #[test]
    fn test_profit() {
        assert_eq!(3, profit(vec![1,4,2,1]));
        assert_eq!(0, profit(vec![1]));
        assert_eq!(0, profit(vec![1,1]));
        assert_eq!(0, profit(vec![3,2,1]));
        assert_eq!(9, profit(vec![3,1,10,4]));
        assert_eq!(5, profit(vec![7,1,5,3,6,4]));
        assert_eq!(1, profit(vec![1,2]));
    }
}

