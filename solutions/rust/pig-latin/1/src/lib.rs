pub fn translate(input: &str) -> String {
    let input = input.to_lowercase();
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut translated_words = Vec::new();
    for word in words {
        if ["a", "e", "i", "o", "u"].iter().any(|v| word.starts_with(v)) || word.starts_with("xr") || word.starts_with("yt") {
            translated_words.push(format!("{}ay", word));
        } else if word.starts_with("qu") || (word.find("qu").map_or(false, |pos| pos > 0 && word[..pos].chars().all(|c| !"aeiou".contains(c)))) {
            let qu_pos = word.find("qu").unwrap() + 2;
            translated_words.push(format!("{}{}ay", word.chars().skip(qu_pos).collect::<String>(), word.chars().take(qu_pos).collect::<String>()));
        } else {
            let mut count_consonants = 0;
            for (i, c) in word.chars().enumerate() {
                if "aeiou".contains(c) || (c == 'y' && i != 0) {
                    break;
                }
                count_consonants += 1;
            }
            translated_words.push(format!("{}{}ay", word.chars().skip(count_consonants).collect::<String>(), word.chars().take(count_consonants).collect::<String>()));
        }
    }
    translated_words.join(" ")
}

