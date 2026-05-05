pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    
    let mut result = String::new();
    
    for i in 0..list.len() -1 {
        let line = format!("For want of a {} the {} was lost.", list[i], list[i+1]);
        result.push_str(&line);
        result.push('\n');
    }
    
    let final_line = format!("And all for the want of a {}.", list[0]);
    result.push_str(&final_line);
    
    result
}
