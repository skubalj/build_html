//! This module contains structs used to add text content to an HTML page

use crate::Html;

use std::fmt::{self, Display};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TextContentType {
    Paragraph,
    Header(u8),
    Preformatted,
}

impl Display for TextContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextContentType::Paragraph => write!(f, "f"),
            TextContentType::Header(n) => write!(f, "h{}", n),
            TextContentType::Preformatted => write!(f, "pre"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TextContent {
    tag: TextContentType,
    content: String,
}

impl Html for TextContent {
    fn to_html_string(&self) -> String {
        format!("<{}>{}</{}>", self.tag, self.content, self.tag)
    }
}

impl TextContent {
    pub fn new(tag: TextContentType, text_content: &str) -> Self {
        TextContent {
            tag,
            content: text_content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod text_content {
        use super::*;
        use test_case::test_case;

        #[test_case(TextContentType::Paragraph, "abc 123 def", "<p>abc 123 def</p>"; "test_p_tag")]
        #[test_case(TextContentType::Header(1), "hello", "<h1>hello</h1>"; "test_h1_tag")]
        #[test_case(TextContentType::Header(6), "world", "<h6>world</h6>"; "test_h6_tag")]
        fn to_html_string(tag: TextContentType, content: &str, expected: &str) {
            // Arrange

            // Act
            let actual = TextContent::new(tag, content);

            // Assert
            assert_eq!(actual, expected);
        }
    }
}
