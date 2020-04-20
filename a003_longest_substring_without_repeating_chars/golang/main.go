package main

import "fmt"

func main() {
	r := longestSubstringWithoutRepeatingChars("abcbd")
	fmt.Println(r) // 3

	r = longestSubstringWithoutRepeatingChars("abcabcbb")
	fmt.Println(r) // 3

	r = longestSubstringWithoutRepeatingChars("abcab")
	fmt.Println(r) // 3

	r = longestSubstringWithoutRepeatingChars("aaaaa")
	fmt.Println(r) // 1
}

// Longest Substring Without Repeating Characters - LeetCode.
// Given a string, find the length of the longest substring without
// repeating characters.
// Example 1:
// Input: "abcabcbb"
// Output: 3
// Explanation: The answer is "abc" , with the length of 3.
func longestSubstringWithoutRepeatingChars(s string) int {
	maxLen := 0
	m := map[rune]int{} // char index
	startIndex := 0
	for i, char := range s {
		if index, ok := m[char]; ok && index >= startIndex {
			startIndex = index + 1
		}
		m[char] = i
		maxLen = max(maxLen, i-startIndex+1)
	}
	return maxLen
}

func max(a, b int) int {
	if a >= b {
		return a
	}
	return b
}
