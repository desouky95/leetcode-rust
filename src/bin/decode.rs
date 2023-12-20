fn main() {
    // let result = decode([1, 2, 3].to_vec(), 1);

    // print!("{:?}", result);

    assert_eq!([1,0,2,1].to_vec(),decode([1,2,3].to_vec(), 1));
}

fn decode(mut encoded: Vec<i32>, first: i32) -> Vec<i32> {
    encoded.insert(0, first);


    for i in 1..encoded.len() {
        encoded[i] = encoded[i - 1] ^ encoded[i]
    }
    encoded
}
