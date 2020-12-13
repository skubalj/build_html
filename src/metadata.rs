//! This module contains structs used to create metadata for HTML modules
//!
//! These metadata objects will be added to the head section of an [`HtmlPage`](crate::HtmlPage)

use crate::Html;
use crate::HtmlPage;

/// Represents the HTML `<base>` element
pub struct Base;

/// Represents the HTML `<link>` element
pub struct Link;

/// Represents the HTML `<meta>` element
pub struct Meta;

/// Represents the HTML `<style>` element
pub struct Style;

/// Represents the HTML `<title>` element
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Title {
    title: String,
}

impl Html for Title {
    fn to_html_string(&self) -> String {
        format!("<title>{}</title>", self.title)
    }
}

impl Title {
    /// Returns a new Title element with the specified text
    pub fn new(title: &str) -> Self {
        Title {
            title: title.into(),
        }
    }
}

impl HtmlPage {
    /// Adds a `<title>` tag to the head of this HTML page
    pub fn add_title(mut self, title_text: &str) -> Self {
        let title = Title::new(title_text);
        self.head.push(Box::new(title));
        self
    }
}