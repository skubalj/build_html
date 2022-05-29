//! Simple content items that can be used inside of an HtmlContainer
//!
//! This module is not publicly exported as the types are only for internal use. Modifications can
//! be made to this file in a patch-level release.

use crate::attributes::Attributes;
use crate::Html;

#[derive(Debug, Clone)]
pub struct Header {
    pub level: u8,
    pub content: String,
    pub attr: Attributes,
}

impl Html for Header {
    fn to_html_string(&self) -> String {
        format!(
            "<h{level}{attr}>{content}</h{level}>",
            level = self.level,
            attr = self.attr,
            content = self.content,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub src: String,
    pub alt: String,
    pub attr: Attributes,
}

impl Html for Image {
    fn to_html_string(&self) -> String {
        format!(
            r#"<img src="{}" alt="{}"{}>"#,
            self.src, self.alt, self.attr
        )
    }
}

#[derive(Debug, Clone)]
pub struct Link {
    pub href: String,
    pub content: String,
    pub attr: Attributes,
}

impl Html for Link {
    fn to_html_string(&self) -> String {
        format!(
            r#"<a href="{}"{}>{}</a>"#,
            self.href, self.attr, self.content
        )
    }
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub content: String,
    pub attr: Attributes,
}

impl Html for Paragraph {
    fn to_html_string(&self) -> String {
        format!("<p{}>{}</p>", self.attr, self.content)
    }
}

#[derive(Debug, Clone)]
pub struct Preformatted {
    pub content: String,
    pub attr: Attributes,
}

impl Html for Preformatted {
    fn to_html_string(&self) -> String {
        format!("<pre{}>{}</pre>", self.attr, self.content)
    }
}
