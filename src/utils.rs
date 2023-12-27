pub fn strip_html(s: String) -> String {
    let mut in_tag = false;
    let mut result = String::new();

    for c in s.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }

    result
}
