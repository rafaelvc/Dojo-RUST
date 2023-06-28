//https://leetcode.com/problems/zigzag-conversion/
// https://leetcode.com/problems/zigzag-conversion/solutions/3136168/java-easy-to-understand/
#![allow(dead_code)]

pub mod zigzag_conversion { 
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {return s;}
        let mut result = String::with_capacity( s.len() );
        let mut str_rows = Vec::new();
        for _i in 0..num_rows { str_rows.push(String::new()); }
        let mut row: i32 = 0;
        let mut inc_mode = true;
        for c in s.chars() {
            str_rows[row as usize].push(c);
            //if inc_mode {
            //    if row < num_rows - 1 {row += 1;}
            //    else {inc_mode = false; row -= 1;}
            //    continue;
            //}
            //if row > 0 {row -= 1;}
            //else {
            //    inc_mode = true; 
            //    row += 1;
            //}
            if inc_mode && row < num_rows - 1 { row += 1 ;}
            else if row == num_rows - 1 { inc_mode = false; row -= 1; }
            else if row > 0 {row -= 1;}
            else {
                inc_mode = true;
                row += 1;
            }
        }

        let mut i = 0;
        for row in str_rows.into_iter() {
            for c in row.chars() {
                result.insert(i, c);
                i += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::zigzag_conversion::convert;
    #[test]
    fn test_convert() {
        assert_eq!("PAHNAPLSIIGYIR".to_string(), convert("PAYPALISHIRING".to_string(), 3));
        assert_eq!("PINALSIGYAHRPI".to_string(), convert("PAYPALISHIRING".to_string(), 4));
    }
}