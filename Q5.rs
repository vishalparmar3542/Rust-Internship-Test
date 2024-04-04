fn median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 == 0 {
        (nums[len / 2 - 1] + nums[len / 2]) as f64 / 2.0
    } else {
        nums[len / 2] as f64
    }
}

fn main() {
    let nums = [1, 2, 3, 4, 5];
    let median_val = median(&nums);
    println!("Median: {}", median_val);
}
