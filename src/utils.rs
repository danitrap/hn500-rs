//! This module contains utility functions.

#![deny(clippy::all)]

/// Strips HTML tags from a string.
pub fn strip_html(s: &str) -> String {
    let mut in_tag = false;
    let mut result = String::new();

    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
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

    #[test]
    fn it_keeps_greater_than_characters_outside_tags() {
        let snippet = "1 > 2".to_string();
        assert_eq!(strip_html(&snippet), "1 > 2");
    }
}
