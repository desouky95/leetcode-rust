
//  [https://leetcode.com/problems/running-sum-of-1d-array/description/]

fn main(){
    running_sum([1,2,3,4].to_vec());
}

fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        nums[i] = nums[i] + nums[i - 1];
    }
    nums
}
