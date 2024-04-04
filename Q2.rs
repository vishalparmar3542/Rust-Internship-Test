fn first_occurrence(nums: &[i32], target: i32) -> Option<usize> {
    nums.iter().position(|&x| x == target)
}

fn main() {
    let nums = [1, 2, 3, 4, 5, 6, 7];
    let target = 4;
    if let Some(index) = first_occurrence(&nums, target) {
        println!("First occurrence of {} is at index {}", target, index);
    } else {
        println!("{} is not found in the array.", target);
    }
}
