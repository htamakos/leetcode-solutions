use std::cmp::max;
use std::collections::HashMap;

struct Solution {}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start_window_num = 0i32;
        let mut max_len = 0i32;
        let mut char_index_map: HashMap<char, i32> = HashMap::new();

        match s.chars().count() {
            0 => 0,
            1 => 1,
            _ => {
                for (i, c) in s.chars().enumerate() {
                    if let Some(&j) = char_index_map.get(&c) {
                        start_window_num = max(j, start_window_num);
                    }

                    max_len = max((i as i32) + 1 - start_window_num, max_len);
                    char_index_map.insert(c, (i + 1) as i32);
                }

                max_len
            }
        }
    }
}

#[test]
fn test_func() {
    let s1 = String::from("abcabcb");
    let s2 = String::from("bbbbb");
    let s3 = String::from("");
    let s4 = String::from("a");
    let s5 = String::from("au");
    let s6 = String::from("dvdfdf");

    assert_eq!(3, Solution::length_of_longest_substring(s1));
    assert_eq!(1, Solution::length_of_longest_substring(s2));
    assert_eq!(0, Solution::length_of_longest_substring(s3));
    assert_eq!(1, Solution::length_of_longest_substring(s4));
    assert_eq!(2, Solution::length_of_longest_substring(s5));
    assert_eq!(3, Solution::length_of_longest_substring(s6));
}
