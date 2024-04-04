fn kth_smallest(nums: &[i32], k: usize) -> Option<i32> {
    nums.iter().copied().nth(k - 1)
}

fn main() {
    let nums = [4, 2, 1, 3, 5];
    let k = 3;
    if let Some(kth_smallest) = kth_smallest(&nums, k) {
        println!("{}th smallest element: {}", k, kth_smallest);
    } else {
        println!("Invalid input.");
    }
}
