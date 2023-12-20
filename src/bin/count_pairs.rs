fn main() {
    count_pairs([-1, 1, 2, 3, 1].to_vec(),2);
}
fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut copy = nums.clone();
    copy.sort();
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut count = 0;
    println!("i=>{} || j=>{} ,sorted {:?}", i, j, copy);

    while i < j {
        let sum = copy[i] + copy[j];
        if sum < target {
            count += j - i;
            i += 1;
        } else {
            j -= 1;
        }
    }

    return count as i32;
}
