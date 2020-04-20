package main

import (
	"fmt"
)

func main() {
	a := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	b := []int{10, 20, 30, 40, 50, 60, 70, 80, 90, 100}
	r := findMedianSortedArrays(a, b)
	fmt.Println(r) // 10
}

func findMedianSortedArrays(a, b []int) float64 {
	aLen := len(a)
	bLen := len(b)
	n := aLen + bLen
	all := make([]int, n)
	pos := 0
	i := 0
	j := 0
	for i < aLen && j < bLen {
		if a[i] <= b[j] {
			all[pos] = a[i]
			i++
		} else {
			all[pos] = b[j]
			j++
		}
		pos++
	}
	for i < aLen {
		all[pos] = a[i]
		pos++
		i++
	}
	for j < bLen {
		all[pos] = b[j]
		pos++
		j++
	}
	if n%2 == 1 {
		return float64(all[n/2])
	}
	return float64((all[n/2] + all[n/2-1]) / 2.0)
}
