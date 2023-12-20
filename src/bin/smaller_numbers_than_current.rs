// [https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/description/]
fn main() {
    smaller_numbers_than_current([8, 1, 2, 2, 3].to_vec());
}

fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums.clone();
    sorted.sort();
    return nums
        .iter()
        .map(|&x| sorted.iter().position(|&pos| pos == x).unwrap() as i32)
        .collect();
}
