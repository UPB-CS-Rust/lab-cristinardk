// This a unfinished implementation of the well-known merge sort algorithm
//
// 1. Fix the language problems in the function merge
//
// 2. Finish the implementation of the function merge_sort
//
// 3. EXTRA: try changing the type from i32 into String everywhere; does your program still compile? What changes are necessary?

/// Merge two array slices (that have to be sorted) into a vector
fn merge<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut dest = Vec::with_capacity(a.len() + b.len());
    let mut a_idx = 0;
    let mut b_idx = 0;

    while a_idx < a.len() && b_idx < b.len() {
        if a[a_idx] <= b[b_idx] {
            dest.push(a[a_idx].clone());
            a_idx += 1;
        } else {
            dest.push(b[b_idx].clone());
            b_idx += 1;
        }
    }

    dest.extend_from_slice(&a[a_idx..]);
    dest.extend_from_slice(&b[b_idx..]);

    dest
}

/// Take an array slice, and sort into a freshly constructed vector using the above function
fn merge_sort<T: Ord + Clone>(data: &[T]) -> Vec<T> {
    if data.len() <= 1 {
        data.to_vec()
    } else {
        let mid = data.len() / 2;
        let left = merge_sort(&data[..mid]);
        let right = merge_sort(&data[mid..]);
        merge(&left, &right)
    }
}

/// Read a bunch of numbers from standard input into a Vec<i32>.
fn read_numbers() -> Vec<i32> {
    use std::io::{self, BufRead};
    let mut result = Vec::new();
    for line in io::stdin().lock().lines().flatten() {
        for word in line.split_whitespace() {
            result.push(word.parse().unwrap());
        }
    }

    result
}

fn main() {
    let input = read_numbers();
    println!("Data to be sorted:");
    println!("{:?}", input);

    let sorted_input = merge_sort(&input);
    println!("Sorted data:");
    println!("{:?}", sorted_input);
}

// you can run these automatic tests by typing 'cargo test'
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(merge_sort(&[]), vec![]);
        assert_eq!(merge_sort(&[5]), vec![5]);
        assert_eq!(merge_sort(&[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(merge_sort(&[47, 42, 5, 1]), vec![1, 5, 42, 47]);
        assert_eq!(merge_sort(&[6, 47, 42, 5, 1, 123]), vec![1, 5, 6, 42, 47, 123]);
    }
}

