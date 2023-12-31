#![deny(clippy::all)]

pub fn strip_html(s: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_strips_html() {
        let snippet = "<p>snippet</p>".to_string();
        assert_eq!(strip_html(&snippet), "snippet");
    }

    #[test]
    fn it_strips_html_with_multiple_tags() {
        let snippet = "<p><a>snippet</a></p>".to_string();
        assert_eq!(strip_html(&snippet), "snippet");
    }

    #[test]
    fn it_strips_html_with_multiple_tags_and_attributes() {
        let snippet =
            "<p class=\"foo\"><a href=\"https://example.com\">snippet</a></p>".to_string();
        assert_eq!(strip_html(&snippet), "snippet");
    }
}
