fn main() {
    let result = smaller_numbers_than_current([8, 1, 2, 2, 3].to_vec());
    println!("{:?}", result);

    // let vec = [8, 1, 2, 2, 3].to_vec();

    // let mut sorted = vec.clone();

    // sorted.sort();

    // println!("{:?}, {:?}", vec, sorted);

    // let result: Vec<i32> = vec
    // .iter()
    // .map(|&num| sorted.iter().position(|&pos| pos == num).unwrap() as i32)
    // .collect();
    // println!("Result :: {:?}", result);
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



fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums.clone();
    sorted.sort();
    return nums
        .iter()
        .map(|&x| sorted.iter().position(|&pos| pos == x).unwrap() as i32)
        .collect();
}
