// [https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/]

fn main() {
    most_words_found(vec![
        "alice and bob love leetcode".to_string(),
        "i think so too".to_string(),
        "this is great thanks very much".to_string(),
    ]);
}
fn most_words_found(sentences: Vec<String>) -> i32 {
    return sentences
        .iter()
        .map(|s| s.split(' ').collect::<Vec<&str>>().len() as i32)
        .max()
        .unwrap();
}
