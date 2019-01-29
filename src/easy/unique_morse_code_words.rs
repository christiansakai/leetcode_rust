use std::collections::HashMap;
use std::collections::HashSet;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let map = create_morse_map();
    let mut set = HashSet::new();

    for word in words {
        set.insert(word_to_morse(&map, word));
    }

    set.len() as i32
}

fn word_to_morse(map: &HashMap<char, &'static str>, word: String) -> String {
    let mut result = String::new();
    for c in word.chars() {
        let morse = map.get(&c).unwrap();
        result.push_str(morse);
    }

    result
}

fn create_morse_map() -> HashMap<char, &'static str> {
    let mut map = HashMap::new();
    map.insert('a', ".-");
    
    map.insert('b', "-...");
    map.insert('c', "-.-.");
    map.insert('d', "-..");
    map.insert('e', ".");
    map.insert('f', "..-.");
    map.insert('g', "--.");
    map.insert('h', "....");
    map.insert('i', "..");
    map.insert('j', ".---");
    map.insert('k', "-.-");
    map.insert('l', ".-..");
    map.insert('m', "--");
    map.insert('n', "-.");
    map.insert('o', "---");
    map.insert('p', ".--.");
    map.insert('q', "--.-");
    map.insert('r', ".-.");
    map.insert('s', "...");
    map.insert('t', "-");
    map.insert('u', "..-");
    map.insert('v', "...-");
    map.insert('w', ".--");
    map.insert('x', "-..-");
    map.insert('y', "-.--");
    map.insert('z', "--..");

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("gin"),
            String::from("zen"),
            String::from("gig"),
            String::from("msg"),
        ];

        assert!(unique_morse_representations(words) == 2);
    }
}

