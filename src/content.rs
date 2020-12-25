//! This module contains structs used to add text content to an HTML page

use crate::attributes::Attributes;
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
}

impl Html for BodyContent {
    fn to_html_string(&self) -> String {
        match self {
            BodyContent::Header {
                level,
                content,
                attr,
            } => {
                format!("<h{}{}>{}</h{}>", level, attr, content, level)
            }
            BodyContent::Paragraph { content, attr } => format!("<p{}>{}</p>", attr, content),
            BodyContent::Preformatted { content, attr } => {
                format!("<pre{}>{}</pre>", attr, content)
            }
            BodyContent::Link {
                href,
                content,
                attr,
            } => format!(r#"<a href="{}"{}>{}</a>"#, href, attr, content),
            BodyContent::Image { src, alt, attr } => {
                format!(r#"<img src="{}" alt="{}"{}>"#, src, alt, attr)
            }
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
                attr: Attributes::from(hashmap! {
                    "id" => "sub",
                    "class" => "heading"
                }),
            };

            // Act / Assert
            assert_eq!(
                sut.to_html_string(),
                r#"<h6 class="heading" id="sub">Sub Heading</h6>"#
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
                attr: Attributes::from(hashmap! {
                    "id" => "test-text",
                    "onclick" => "something()",
                    "class" => "text"
                }),
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
