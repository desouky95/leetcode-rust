// [https://leetcode.com/problems/sum-of-values-at-indices-with-k-set-bits/]

fn main() {
    let result = sum_indices_with_k_set_bits([5, 10, 1, 5, 2].to_vec(), 1);
    println!("{result}");
}

fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    for i in 0..nums.len() {
        let binary = format!("{:b}", i as i32);
        if binary.chars().map(|c| c.to_digit(2).unwrap()).sum::<u32>() as i32 == k {
            sum += nums[i];
        }
    }
    sum
}
