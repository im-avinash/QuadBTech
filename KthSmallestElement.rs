fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None;
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}

fn main() {
    let array = [5, 3, 4, 2, 7, 24, 11];
    let k = 4;
    match kth_smallest(&array, k) {
        Some(result) => println!("The {}th smallest element is {}", k, result),
        None => println!("Invalid input: k is out of bounds"),
    }
}
