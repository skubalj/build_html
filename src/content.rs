//! This module contains structs used to add text content to an HTML page

use crate::Html;

use std::fmt::{self, Display};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum TextContentType {
    Header(u8),
    Paragraph,
    Preformatted,
    Title,
}

impl Display for TextContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextContentType::Header(n) => write!(f, "h{}", n),
            TextContentType::Paragraph => write!(f, "p"),
            TextContentType::Preformatted => write!(f, "pre"),
            TextContentType::Title => write!(f, "title"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Link {
    href: String,
    text: String,
}

impl Html for Link {
    fn to_html_string(&self) -> String {
        format!(r#"<a href="{}">{}</a>"#, self.href, self.text)
    }
}

impl Link {
    /// Make a new link element with the specified `href` and text
    pub fn new(href: &str, text: &str) -> Self {
        Link {
            href: href.into(),
            text: text.into(),
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
        #[test_case(TextContentType::Preformatted, "i => is | code", "<pre>i => is | code</pre>"; "test_pre_tag")]
        fn to_html_string(tag: TextContentType, content: &str, expected: &str) {
            // Arrange
            let sut = TextContent::new(tag, content);

            // Act
            let actual = sut.to_html_string();

            // Assert
            assert_eq!(actual, expected);
        }
    }

    mod link {
        use super::*;
        use test_case::test_case;

        #[test_case("https://rust-lang.org/", "Rust Home", r#"<a href="https://rust-lang.org/">Rust Home</a>"#; "test_link_1")]
        #[test_case("localhost:8080", "local", r#"<a href="localhost:8080">local</a>"#; "test_link_2")]
        fn to_html_string(href: &str, text: &str, expected: &str) {
            // Arrange
            let sut = Link::new(href, text);

            // Act
            let actual = sut.to_html_string();

            // Assert
            assert_eq!(actual, expected);
        }
    }
}
