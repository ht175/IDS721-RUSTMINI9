use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<&str, usize> {
    let mut word_count: HashMap<&str, usize> = HashMap::new();

    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }

    word_count
}

fn main() {
    let text = "The quick brown fox jumps over the lazy dog";
    let word_count = count_words(text);

    println!("{:?}", word_count);
}
