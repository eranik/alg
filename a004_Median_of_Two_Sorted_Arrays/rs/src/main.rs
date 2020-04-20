fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let b = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let r = find_median_sorted_arrays(&a, &b);
    println!("{}", r); // 10
}

fn find_median_sorted_arrays(a: &[i32], b: &[i32]) -> f64 {
    let a_len = a.len();
    let b_len = b.len();
    let n = a_len + b_len;
    let mut all = vec![0; n];
    let mut pos = 0;
    let mut i = 0;
    let mut j = 0;
    while i < a_len && j < b_len {
        if a[i] <= b[j] {
            all[pos] = a[i];
            i += 1;
        } else {
            all[pos] = b[j];
            j += 1;
        }
        pos += 1;
    }
    while i < a_len {
        all[pos] = a[i];
        pos += 1;
        i += 1;
    }
    while j < b_len {
        all[pos] = b[j];
        pos += 1;
        j += 1;
    }
    if n % 2 == 1 {
        return all[n / 2] as f64;
    }
    (all[n / 2] + all[n / 2 - 1]) as f64 / 2.0
}
