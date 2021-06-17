//! This module contains the `BodyContent` and `HeadContent` enums which define
//! the different types of content that can be added to HTML containers.
//!
//! Users of this crate should not need to create instances of these enums
//! directly, and so the content of this module is not exported. To add content,
//! the methods of the [`HtmlContainer`](crate::HtmlContainer) trait should be
//! used instead.

use crate::attributes::Attributes;
use crate::Html;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HeadContent {
    /// A head `<link>` element used to link to things such as stylesheets
    Link {
        href: String,
        rel: String,
        attr: Attributes,
    },
    /// A head `<meta>` element used to add metadata to the document
    Meta { attr: Attributes },
    /// A head `<script>` element that references a link to a remote script
    ScriptLink { src: String, attr: Attributes },
    /// A head `<script>` element that contains literal code
    ScriptLiteral { code: String },
    /// A head `<style>` element used to specify literal styles
    Style { css: String, attr: Attributes },
    /// A `<title>` element used to set the document title
    Title { content: String },
}

impl Html for HeadContent {
    fn to_html_string(&self) -> String {
        match self {
            Self::Link { href, rel, attr } => {
                format!(r#"<link href="{}" rel="{}"{}>"#, href, rel, attr)
            }
            Self::Meta { attr } => format!("<meta{}>", attr),
            Self::ScriptLink { src, attr } => format!(r#"<script src="{}"{}></script>"#, src, attr),
            Self::ScriptLiteral { code } => format!("<script>{}</script>", code),
            Self::Style { css, attr } => format!("<style{}>{}</style>", attr, css),
            Self::Title { content } => format!("<title>{}</title>", content),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BodyContent {
    /// An HTML header element `<h#>`
    Header {
        level: u8,
        content: String,
        attr: Attributes,
    },
    /// An HTML image element `<img>`
    Image {
        src: String,
        alt: String,
        attr: Attributes,
    },
    /// An HTML link element `<a>`
    Link {
        href: String,
        content: String,
        attr: Attributes,
    },
    /// An HTML text element `<p>`
    Paragraph { content: String, attr: Attributes },
    /// An HTML preformatted text element `<pre>`
    Preformatted { content: String, attr: Attributes },
    /// Raw string content, to be used as an escape hatch
    Raw { content: String },
}

impl Html for BodyContent {
    fn to_html_string(&self) -> String {
        match self {
            BodyContent::Header {
                level,
                content,
                attr,
            } => format!(
                "<h{level}{attr}>{content}</h{level}>",
                level = level,
                attr = attr,
                content = content
            ),
            BodyContent::Paragraph { content, attr } => {
                format!("<p{attr}>{content}</p>", attr = attr, content = content)
            }
            BodyContent::Preformatted { content, attr } => {
                format!("<pre{attr}>{content}</pre>", attr = attr, content = content)
            }
            BodyContent::Link {
                href,
                content,
                attr,
            } => format!(
                r#"<a href="{href}"{attr}>{content}</a>"#,
                href = href,
                attr = attr,
                content = content
            ),
            BodyContent::Image { src, alt, attr } => format!(
                r#"<img src="{src}" alt="{alt}"{attr}>"#,
                src = src,
                alt = alt,
                attr = attr
            ),
            BodyContent::Raw { content } => content.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests for the `HeadContent` enum
    mod head_content {
        use super::*;
        use test_case::test_case;

        #[test_case(HeadContent::Title {content: "Page Title".into()}, "<title>Page Title</title>"; "test_title")]
        fn to_html_string(sut: HeadContent, expected: &str) {
            assert_eq!(sut.to_html_string(), expected);
        }
    }

    /// Tests for the `BodyContent` enum
    mod body_content {
        use super::*;
        use maplit::hashmap;

        #[test]
        fn test_header_1() {
            // Arrange
            let sut = BodyContent::Header {
                level: 1,
                content: "Main Heading".into(),
                attr: Attributes::empty(),
            };

            // Act / Assert
            assert_eq!(sut.to_html_string(), "<h1>Main Heading</h1>")
        }

        #[test]
        fn test_header_6() {
            // Arrange
            let sut = BodyContent::Header {
                level: 6,
                content: "Sub Heading".into(),
                attr: vec![("id", "sub"), ("class", "heading")].into(),
            };

            // Act / Assert
            assert_eq!(
                sut.to_html_string(),
                r#"<h6 id="sub" class="heading">Sub Heading</h6>"#
            )
        }

        #[test]
        fn test_image() {
            // Arrange
            let sut = BodyContent::Image {
                src: "myImage.jpg".into(),
                alt: "This is alternate text".into(),
                attr: Attributes::from(hashmap! {"class" => "images"}),
            };

            // Act / Assert
            assert_eq!(
                sut.to_html_string(),
                r#"<img src="myImage.jpg" alt="This is alternate text" class="images">"#
            )
        }

        #[test]
        fn test_link() {
            // Arrange
            let sut = BodyContent::Link {
                href: "https://rust-lang.org".into(),
                content: "Rust Homepage".into(),
                attr: Attributes::empty(),
            };

            // Act / Assert
            assert_eq!(
                sut.to_html_string(),
                r#"<a href="https://rust-lang.org">Rust Homepage</a>"#
            )
        }

        #[test]
        fn test_paragraph() {
            // Arrange
            let sut = BodyContent::Paragraph {
                content: "This is sample text".into(),
                attr: vec![
                    ("class", "text"),
                    ("id", "test-text"),
                    ("onclick", "something()"),
                ]
                .into(),
            };

            // Act / Assert
            assert_eq!(
                sut.to_html_string(),
                r#"<p class="text" id="test-text" onclick="something()">This is sample text</p>"#
            )
        }

        #[test]
        fn test_preformatted() {
            // Arrange
            let sut = BodyContent::Preformatted {
                content: "Pre => formatted".into(),
                attr: Attributes::empty(),
            };

            // Act / Assert
            assert_eq!(sut.to_html_string(), r#"<pre>Pre => formatted</pre>"#)
        }
    }
}
