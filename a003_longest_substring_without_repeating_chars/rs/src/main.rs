fn main() {
    let r = longest_substring_without_repeating_chars("abcbd");
    println!("{}", r); // 3

    let r = longest_substring_without_repeating_chars("abcabcbb");
    println!("{}", r); // 3

    let r = longest_substring_without_repeating_chars("abcab");
    println!("{}", r); // 3

    let r = longest_substring_without_repeating_chars("aaaaa");
    println!("{}", r); // 1
}

// Longest Substring Without Repeating Characters - LeetCode.
// Given a string, find the length of the longest substring without
// repeating characters.
// Example 1:
// Input: "abcabcbb"
// Output: 3
// Explanation: The answer is "abc" , with the length of 3.
pub fn longest_substring_without_repeating_chars(s: &str) -> usize {
    let mut max_len = 0;
    use std::collections::HashMap;
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut start_index = 0;
    for (i, c) in s.chars().enumerate() {
        if let Some(&index) = map.get(&c) {
            if index >= start_index {
                start_index = index + 1
            }
        }
        map.insert(c, i);
        max_len = max_len.max(i - start_index + 1);
    }
    max_len
}
