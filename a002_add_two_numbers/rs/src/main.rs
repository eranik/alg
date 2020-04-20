use std::collections::LinkedList;
fn main() {
    let mut a = LinkedList::new();
    a.push_back(2);
    a.push_back(4);
    a.push_back(3);

    let mut b = LinkedList::new();
    b.push_back(5);
    b.push_back(6);
    b.push_back(4);

    println!("{:?}", a); //[2, 4, 3]
    println!("{:?}", b); //[5, 6, 4]
    let c = add_two_numbers(&a, &b);
    println!("{:?}\n", c); //[7, 0, 8]

    a.pop_back();
    println!("{:?}", a); //[2, 4]
    println!("{:?}", b); //[5, 6, 4]
    let c = add_two_numbers(&a, &b);
    println!("{:?}\n", c); //[7, 0, 5]

    a.pop_front();
    println!("{:?}", a); //[4]
    println!("{:?}", b); //[5, 6, 4]
    let c = add_two_numbers(&a, &b);
    println!("{:?}\n", c); //[9, 6, 4]

    a.pop_front();
    println!("{:?}", a); //[]
    println!("{:?}", b); //[5, 6, 4]
    let c = add_two_numbers(&a, &b);
    println!("{:?}\n", c); //[5, 6, 4]
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
pub fn add_two_numbers(a: &LinkedList<u32>, b: &LinkedList<u32>) -> LinkedList<u32> {
    let mut r = LinkedList::new();
    let mut a_iter = a.iter();
    let mut b_iter = b.iter();
    let mut carry = 0;
    loop {
        let mut av = 0;
        let mut bv = 0;
        let mut done = true;
        if let Some(&v) = a_iter.next() {
            av = v;
            done = false;
        }
        if let Some(&v) = b_iter.next() {
            bv = v;
            done = false;
        }
        if done {
            break;
        }
        let digit = carry + av + bv;
        carry = digit / 10;
        r.push_back(digit % 10);
    }
    if carry > 0 {
        r.push_back(carry);
    }
    r
}

// pub fn add_two_numbers(a: LinkedList<u32>, b: LinkedList<u32>) -> LinkedList<u32> {
//     let mut r = LinkedList::new();
//     let mut a_iter = a.iter();
//     let mut b_iter = b.iter();
//     let mut a_done = false;
//     let mut b_done = false;
//     let mut carry = 0;
//     loop {
//         let av = match a_iter.next() {
//             Some(&av) => av,
//             None => {
//                 a_done = true;
//                 0
//             }
//         };
//         let bv = match b_iter.next() {
//             Some(&bv) => bv,
//             None => {
//                 b_done = true;
//                 0
//             }
//         };
//         if a_done && b_done {
//             break;
//         }
//         let digit = carry + av + bv;
//         carry = digit / 10;
//         r.push_back(digit % 10);
//     }
//     if carry > 0 {
//         r.push_back(carry);
//     }
//     r
// }
