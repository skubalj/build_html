//! This module contains the `HtmlPage` struct, which serves as the major entry point for the program

use crate::attributes::Attributes;
use crate::content::HeadContent;
use crate::html_container::HtmlContainer;
use crate::Html;
use std::collections::HashMap;
use std::fmt::{self, Display};

/// This struct represents an entire page of HTML which can built up by chaining addition methods.
///
/// This creates an effect similar to the [Decorator Pattern](https://en.wikipedia.org/wiki/Decorator_pattern)
///
/// To convert an `HtmlPage` to a [`String`] which can be sent back to a client, use the
/// [`Html::to_html_string()`] method
///
/// # Example
/// ```
/// # use html_gen::*;
/// let page: String = HtmlPage::new()
///     .add_title("My Page")
///     .add_header(1, "Header Text")
///     .to_html_string();
/// ```
#[derive(Debug)]
pub struct HtmlPage {
    head: Vec<Box<dyn Html>>,
    body: Vec<Box<dyn Html>>,
}

impl Html for HtmlPage {
    fn to_html_string(&self) -> String {
        let head = self
            .head
            .iter()
            .map(|element| element.to_html_string())
            .fold(String::new(), |acc, next| acc + &next);
        let body = self
            .body
            .iter()
            .map(|element| element.to_html_string())
            .fold(String::new(), |acc, next| acc + &next);

        format!(
            "<!DOCTYPE html><html><head>{}</head><body>{}</body></html>",
            head, body
        )
    }
}

impl HtmlContainer for HtmlPage {
    fn add_html(mut self, html: Box<dyn Html>) -> Self {
        self.body.push(html);
        self
    }
}

impl Default for HtmlPage {
    fn default() -> Self {
        HtmlPage::new()
    }
}

impl Display for HtmlPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_html_string())
    }
}

impl HtmlPage {
    /// Creates a new HTML page with no content
    pub fn new() -> Self {
        HtmlPage {
            head: Vec::new(),
            body: Vec::new(),
        }
    }

    /// Adds a title to this HTML page
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let page = HtmlPage::new()
    ///     .add_title("My Page")
    ///     .to_html_string();
    ///
    /// assert_eq!(page, "<!DOCTYPE html><html><head><title>My Page</title></head><body></body></html>")
    /// ```
    pub fn add_title(mut self, title_text: &str) -> Self {
        let title = HeadContent::Title {
            content: title_text.into(),
        };
        self.head.push(Box::new(title));
        self
    }

    /// Adds a style to this HTML page
    pub fn add_style(mut self, css: &str, attributes: Option<HashMap<&str, &str>>) -> Self {
        let style = HeadContent::Style {
            css: css.into(),
            attr: attributes.map(Attributes::from).unwrap_or_default(),
        };
        self.head.push(Box::new(style));
        self
    }

    /// Adds a new link to the HTML head.
    ///
    /// This is how to link a stylesheet into the document
    pub fn add_head_link(
        mut self,
        href: &str,
        rel: &str,
        attributes: Option<HashMap<&str, &str>>,
    ) -> Self {
        let link = HeadContent::Link {
            href: href.into(),
            rel: rel.into(),
            attr: attributes.map(Attributes::from).unwrap_or_default(),
        };
        self.head.push(Box::new(link));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        // Arrange
        let sut = HtmlPage::default();

        // Act
        let html_string = sut.to_html_string();

        // Assert
        assert_eq!(
            html_string,
            "<!DOCTYPE html><html><head></head><body></body></html>"
        )
    }
}
