
fn main() {
    left_right_difference([10, 4, 8, 3].to_vec());

    // println!("{:?}",result)
}

fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let mut left_sum = nums.clone();
    let mut right_sum = nums.clone();

    for i in (0..left_sum.len()).rev() {
        left_sum[i] = left_sum[0..i].iter().sum();
    }
    for i in 0..right_sum.len() {
        right_sum[i] = right_sum[i + 1..right_sum.len()].iter().sum()
    }

    return left_sum
        .iter()
        .enumerate()
        .map(|(index, x)| (x - right_sum[index]))
        .map(|x| x.abs())
        .collect();
}
