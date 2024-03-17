pub fn pig_latin(word: &str) -> String {
    if word.len() == 0 {
        return String::from("");
    }

    let first_char = word.chars().nth(0).unwrap();
    match first_char {
        'a' | 'i' | 'u' | 'e' | 'o' => return format!("{}-hay", word),
        _ => return format!("{}-{}ay", word[1..].to_string(), first_char),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_char_is_vowel() {
        let res = pig_latin("apple");
        println!("{res}");
        assert_eq!(res, "apple-hay")
    }

    #[test]
    fn first_char_is_not_vowel() {
        let res = pig_latin("first");
        println!("{res}");
        assert_eq!(res, "irst-fay")
    }
}
