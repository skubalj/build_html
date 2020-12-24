//! This module contains structs used to add text content to an HTML page

use crate::Html;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum HeadContent {
    Title { content: String },
}

impl Html for HeadContent {
    fn to_html_string(&self) -> String {
        match self {
            HeadContent::Title { content } => format!("<title>{}</title>", content),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BodyContent {
    /// An HTML header element `<h#>`
    Header { level: u8, content: String },
    /// An HTML image element `<img>`
    Image { src: String, alt: String },
    /// An HTML link element `<a>`
    Link { href: String, content: String },
    /// An HTML text element `<p>`
    Paragraph { content: String },
    /// An HTML preformatted text element `<pre>`
    Preformatted { content: String },
}

impl Html for BodyContent {
    fn to_html_string(&self) -> String {
        match self {
            BodyContent::Header { level, content } => {
                format!("<h{}>{}</h{}>", level, content, level)
            }
            BodyContent::Paragraph { content } => format!("<p>{}</p>", content),
            BodyContent::Preformatted { content } => format!("<pre>{}</pre>", content),
            BodyContent::Link { href, content } => format!(r#"<a href="{}">{}</a>"#, href, content),
            BodyContent::Image { src, alt } => format!(r#"<img src="{}" alt="{}" />"#, src, alt),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod head_content {
        use super::{HeadContent, Html};
        use test_case::test_case;

        #[test_case(HeadContent::Title {content: "Page Title".into()}, "<title>Page Title</title>"; "test_title")]
        fn to_html_string(sut: HeadContent, expected: &str) {
            assert_eq!(sut.to_html_string(), expected);
        }
    }

    mod body_content {
        use super::{BodyContent, Html};
        use test_case::test_case;

        #[test_case(BodyContent::Header {level: 1, content: "hello".into()}, "<h1>hello</h1>"; "test_header_1")]
        #[test_case(BodyContent::Header {level: 6, content: "world".into()}, "<h6>world</h6>"; "test_header_6")]
        #[test_case(BodyContent::Image {src: "abc.jpg".into(), alt: "test".into()}, r#"<img src="abc.jpg" alt="test">"#; "test_image")]
        #[test_case(BodyContent::Link {href: "https://rust-lang.org/".into(), content: "rust".into()}, r#"<a href="https://rust-lang.org">rust</a>"#; "test_link")]
        #[test_case(BodyContent::Paragraph {content: "abc 123 def".into()}, "<p>abc 123 def</p>"; "test_paragraph")]
        #[test_case(BodyContent::Preformatted {content: "i => is | code".into()}, "<pre>i => is | code</pre>"; "test_pre_tag")]
        fn to_html_string(sut: BodyContent, expected: &str) {
            assert_eq!(sut.to_html_string(), expected);
        }
    }
}
