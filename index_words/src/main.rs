use std::collections::HashMap;

const LARGE_TEXT: &str = "Work, in its simplest form, is effort. But effort without direction is wasted work. 
                        Sometimes work becomes routine, and the routine of work can dull the mind. 
                        Yet, the absence of work can dull the spirit even more. 
                        That is why people search for balance — balance between work and rest, between work and play, 
                        between time for others and time for themselves.";

fn main() {
    let map = index_words(LARGE_TEXT);
    println!("{map:?}");
}

fn index_words(text: &str) -> HashMap<&str, Vec<usize>> {
    let mut map = HashMap::new();
    let mut index = 0;

    for word in text.split_whitespace() {
        let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());

        if !clean_word.is_empty() {
            map.entry(clean_word)
                .or_insert_with(Vec::new)
                .push(index);
        }
        else {
            index -= 2;
        }

        index += word.len() + 1;
    }
    return map;
}
