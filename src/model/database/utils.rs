pub fn sanitize_title(title: &str) -> String {
    let lowercase_title = title.to_lowercase();
    let mut sanitized_title = String::new();
    let mut last_char_was_dash = false;

    for c in lowercase_title.chars() {
        if c.is_alphanumeric() {
            sanitized_title.push(c);
            last_char_was_dash = false;
        } else {
            if !last_char_was_dash {
                sanitized_title.push('-');
            }
            last_char_was_dash = true;
        }
    }

    if last_char_was_dash && !sanitized_title.is_empty() {
        sanitized_title.pop();
    }

    sanitized_title
}