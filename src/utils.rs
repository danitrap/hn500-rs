pub fn strip_html(s: String) -> String {
    let mut in_tag = false;
    let mut result = String::new();

    let s = s.replace('\n', "");

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_strips_html() {
        let snippet = "<p>snippet</p>".to_string();
        assert_eq!(strip_html(snippet), "snippet");
    }

    #[test]
    fn it_strips_html_with_multiple_tags() {
        let snippet = "<p><a>snippet</a></p>".to_string();
        assert_eq!(strip_html(snippet), "snippet");
    }

    #[test]
    fn it_strips_html_with_multiple_tags_and_attributes() {
        let snippet =
            "<p class=\"foo\"><a href=\"https://example.com\">snippet</a></p>".to_string();
        assert_eq!(strip_html(snippet), "snippet");
    }

    #[test]
    fn it_strips_html_with_multiple_tags_and_attributes_and_newlines() {
        let snippet =
            "<p class=\"foo\">\n<a href=\"https://example.com\">\nsnippet\n</a>\n</p>".to_string();
        assert_eq!(strip_html(snippet), "snippet");
    }
}
