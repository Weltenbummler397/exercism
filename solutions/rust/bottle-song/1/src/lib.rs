pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();
    let bottle_word: Vec<&str> = vec!["no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
    let mut current_bottles = start_bottles;
    
    for i in 0..take_down {
        let next_bottles_count = current_bottles - 1;

        // Hilfsfunktion, die Singular/Plural für "bottles" wählt
        let bottles_str_1 = if current_bottles == 1 { "bottle" } else { "bottles" };
        let bottles_str_4 = if next_bottles_count == 1 { "bottle" } else { "bottles" };

        // Die Formatierung verwendet jetzt die dynamische bottles_str_X Variable
        let line = format!(
            "{} green {} hanging on the wall,\n\
             {} green {} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {} green {} hanging on the wall.", // Punkt am Ende dieser letzten Zeile
             bottle_word[current_bottles as usize], bottles_str_1, 
             bottle_word[current_bottles as usize], bottles_str_1, 
             bottle_word[next_bottles_count as usize].to_lowercase(), bottles_str_4
        );

        result.push_str(&line);
        
        if i < take_down - 1 {
           result.push_str("\n\n");
        }
        
        current_bottles -= 1;
    }
    
    result
}
