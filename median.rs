fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        // Array length is even
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] + arr[mid_right]) as f64 / 2.0
    } else {
        // Array length is odd
        arr[len / 2] as f64
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    println!("Median: {}", find_median(&arr1)); // Output: 3

    let arr2 = [1, 2, 3, 4, 5, 6];
    println!("Median: {}", find_median(&arr2)); // Output: 3.5
}
