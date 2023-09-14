use std::collections::HashMap;

fn group_anagrams(words: Vec<String>) -> Vec<Vec<String>>{
    let mut words_hash = HashMap::new();

    let mut char_freq = vec![0;26];

    for word in words{
        for char in word.to_lowercase().chars(){
            char_freq[(char as u32 - 'a' as u32 ) as usize] += 1;
        }

        let char_freq_str = char_freq.into_iter().map(|i| i.to_string()).collect::<String>();
        words_hash.entry(char_freq_str).or_insert(Vec::new()).push(word);

        char_freq = vec![0;26];
    }

    for (key, value) in &words_hash{
        println!("Key: {:?} value: {:?}", key,value);
    }
    words_hash.into_iter().map(|(_,v)| v).collect()
}
fn main() {
    let words = vec!["anagram".to_string(), "nagaram".to_string(), "bored".to_string(), "robed".to_string(), "peach".to_string(), "cheap".to_string()];

    println!("{:?}",group_anagrams(words));
}
