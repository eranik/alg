package main

import "fmt"

func main() {
	a := []int{0, 1, 2, 3, 4, 5, 6, 7}
	r := twoSum(a, 11)
	fmt.Println(r) // [5 6]
}

// Given an array of integers, return indices of the two numbers
// such that they add up to a specific target.
// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.
func twoSum(a []int, target int) [2]int {
	m := map[int]int{}
	// R: O(n)
	for i, n := range a {
		key := target - n
		if j, ok := m[key]; ok {
			return [2]int{j, i}
		}
		m[n] = i
	}
	panic("No solution")
}
