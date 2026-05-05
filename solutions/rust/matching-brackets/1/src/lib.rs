pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for zeichen in string.chars() {
        match zeichen {
            '(' | '[' | '{' => stack.push(zeichen),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _   => continue,
        }
    }

    stack.is_empty()
}
