fn main() {
    let a = [0, 1, 2, 3, 4, 5, 6, 7];
    let r = two_sum(&a, 11);
    println!("{:?}", r); // [5 6]
}

// Given an array of integers, return indices of the two numbers
// such that they add up to a specific target.
// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.
pub fn two_sum(a: &[isize], target: isize) -> [usize; 2] {
    let mut map = std::collections::HashMap::new();
    // R: O(n)
    for (i, n) in a.iter().enumerate() {
        let key = target - n;
        if map.contains_key(&key) {
            return [map[&key], i];
        }
        map.insert(n, i);
    }
    panic!("No solution");
}
