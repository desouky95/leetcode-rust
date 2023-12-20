fn main(){
    let res =create_target_array(vec![0,1,2,3,4],vec![0,1,2,2,1]);

    println!("{:?}",res);
}

fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut rs = vec![];
    for i in 0..index.len() {
        rs.insert(index[i] as usize, nums[i])
    }
    rs       
}