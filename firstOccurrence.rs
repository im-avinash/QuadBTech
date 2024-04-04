fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result: Option<usize> = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1; // continue searching towards left for first occurrence
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let array = [1, 2, 3, 4, 4, 4, 5, 6, 7];
    let target = 4;
    match first_occurrence(&array, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
