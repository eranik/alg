package main

import (
	"container/list"
	"fmt"
)

func main() {
	a := list.New()
	a.PushBack(2)
	a.PushBack(4)
	a.PushBack(3)

	b := list.New()
	b.PushBack(5)
	b.PushBack(6)
	b.PushBack(4)

	show(a) // 2 -> 4 -> 3
	show(b) // 5 -> 6 -> 4
	c := addTwoNumbers(a, b)
	show(c) // 7 -> 0 -> 8
}

// Add Two Numbers - LeetCode.
// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order and each of their nodes contain a
// single digit. Add the two numbers and return it as a linked list.
// ```
// 2 -> 4 -> 3
// +
// 5 -> 6 -> 4
// =
// 7 -> 0 -> 8
// (342 + 465 = 807)
// ```
func addTwoNumbers(list1, list2 *list.List) *list.List {
	r := list.New()
	a := list1.Front()
	b := list2.Front()
	carry := 0
	for a != nil || b != nil {
		av, bv := 0, 0
		if a != nil {
			av, _ = a.Value.(int)
			a = a.Next()
		}
		if b != nil {
			bv, _ = b.Value.(int)
			b = b.Next()
		}
		digit := carry + av + bv
		carry = digit / 10
		r.PushBack(digit % 10)
	}
	if carry > 0 {
		r.PushBack(carry)
	}
	return r
}

func show(a *list.List) {
	space := ""
	for elm := a.Front(); elm != nil; elm = elm.Next() {
		fmt.Print(space, elm.Value)
		space = " -> "
	}
	fmt.Println()
}
