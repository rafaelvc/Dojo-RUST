// https://leetcode.com/problems/longest-common-prefix/
#![allow(dead_code)]

pub mod longest_common_prefix { 
    pub fn max_prefix_words(words: &Vec<&String>) -> String {
        // let words = &strs[..]; // Leetcode does not use refs (strs: Vec<String>)
        //let lens: Vec<_> = words.into_iter().map(|w| w.len()).collect();
        //let min = lens.iter().skip(1).fold(lens[0], |min, &x| if x < min  { x } else { min });
        let min = words.into_iter().skip(1).fold(words[0].len(), |min, x| if x.len() < min {x.len()} else {min});
        let fw = &words[0];
        let mut max_common_prefix = 0;
        //let fw_chars : Vec<char> = fw.chars().collect();
        let mut fw_chars = fw.chars();
        for i in 0..min {
        
            // let prefix_char = fw_chars[i];
            if let Some(prefix_char) = fw_chars.next() { 

                // There is no known function to reset the iterator (Research more)
                // An idea is to use iter.cycle and breaks the for loop when a counter
                // is bigger than the collections len so we don't need recreate the iterator
                //let iter_words = words.iter().skip(1); 
                let mut diff = false;
                //for w in iter_words {
                for j in 1..(words.len()) {
                    let w = words[j];
                    if w.chars().collect::<Vec<char>>()[i] != (prefix_char) {
                        diff = true;
                        break;
                    }
                }
                if !diff { max_common_prefix += 1}
                else if i == 0 {return String::new()}
                else {break}

            }
            else { panic!("Wrong iter.") }
            
        }
        String::from(&fw[..max_common_prefix])
    }
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix::max_prefix_words;
    #[test]
    fn test_max_common_prefix() {
        assert_eq!("fl".to_string(), max_prefix_words(&vec![&"flower".to_string(),&"flow".to_string(),&"flight".to_string()]));
        assert_eq!("".to_string(), max_prefix_words(&vec![&"dog".to_string(),&"racecar".to_string(),&"car".to_string()]));
        assert_eq!("c".to_string(), max_prefix_words(&vec![&"cir".to_string(),&"car".to_string()]));
    }
}