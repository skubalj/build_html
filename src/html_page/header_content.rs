//! Various tags that can be used in the header of an HTML document
//! 
//! This module is not publicly exported, as the types are only for internal use. Modifications 
//! can be made to this file in a patch-level release.

use crate::attributes::Attributes;
use crate::Html;

#[derive(Debug, Clone)]
pub struct Link {
    pub href: String,
    pub rel: String,
    pub attr: Attributes,
}

impl Html for Link {
    fn to_html_string(&self) -> String {
        format!(
            r#"<link href="{}" rel="{}"{}>"#,
            self.href, self.rel, self.attr
        )
    }
}

#[derive(Debug, Clone)]
pub struct Meta {
    pub attr: Attributes,
}

impl Html for Meta {
    fn to_html_string(&self) -> String {
        format!("<meta{}>", self.attr)
    }
}

#[derive(Debug, Clone)]
pub struct ScriptLink {
    pub src: String,
    pub attr: Attributes,
}

impl Html for ScriptLink {
    fn to_html_string(&self) -> String {
        format!(r#"<script src="{}"{}></script>"#, self.src, self.attr)
    }
}

#[derive(Debug, Clone)]
pub struct ScriptLiteral {
    pub code: String,
}

impl Html for ScriptLiteral {
    fn to_html_string(&self) -> String {
        format!("<script>{}</script>", self.code)
    }
}

#[derive(Debug, Clone)]
pub struct Style {
    pub css: String,
    pub attr: Attributes,
}

impl Html for Style {
    fn to_html_string(&self) -> String {
        format!("<style{}>{}</style>", self.attr, self.css)
    }
}

#[derive(Debug, Clone)]
pub struct Title {
    pub content: String,
}

impl Html for Title {
    fn to_html_string(&self) -> String {
        format!("<title>{}</title>", self.content)
    }
}
