//! This module contains the `HtmlPage` struct, which serves as the major entry point for the program

use crate::attributes::Attributes;
use crate::content::HeadContent;
use crate::html_container::HtmlContainer;
use crate::Html;
use std::collections::HashMap;
use std::fmt::{self, Display};

/// This struct represents an entire page of HTML which can built up by chaining addition methods.
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
/// 
/// assert_eq!(
///     page,
///     "<!DOCTYPE html><html><head><title>My Page</title></head><body><h1>Header Text</h1></body></html>"
/// )
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
    /// assert_eq!(
    ///     page,
    ///     "<!DOCTYPE html><html><head><title>My Page</title></head><body></body></html>"
    /// );
    /// ```
    pub fn add_title(mut self, title_text: &str) -> Self {
        let title = HeadContent::Title {
            content: title_text.into(),
        };
        self.head.push(Box::new(title));
        self
    }

    /// Adds the specified metadata elements to this `HtmlPage`
    ///
    /// Attributes are specified in a `HashMap`
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// use maplit::hashmap;
    ///
    /// let page = HtmlPage::new()
    ///     .add_meta(hashmap! {"charset" => "utf-8"})
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     page,
    ///     r#"<!DOCTYPE html><html><head><meta charset="utf-8"></head><body></body></html>"#
    /// );
    /// ```
    pub fn add_meta(mut self, attributes: HashMap<&str, &str>) -> Self {
        let meta = HeadContent::Meta {
            attr: Attributes::from(attributes),
        };
        self.head.push(Box::new(meta));
        self
    }

    /// Adds raw style data to this `HtmlPage`
    pub fn add_style(mut self, css: &str, attributes: Option<HashMap<&str, &str>>) -> Self {
        let style = HeadContent::Style {
            css: css.into(),
            attr: attributes.map(Attributes::from).unwrap_or_default(),
        };
        self.head.push(Box::new(style));
        self
    }

    /// Adds the specified stylesheet to the HTML head.
    ///
    /// This method uses [`add_head_link`](HtmlPage::add_head_link) internally
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let page = HtmlPage::new()
    ///     .add_stylesheet("print.css")
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     page,
    ///     r#"<!DOCTYPE html><html><head><link href="print.css" rel="stylesheet"></head><body></body></html>"#
    /// )
    /// ```
    pub fn add_stylesheet(self, source: &str) -> Self {
        self.add_head_link(source, "stylesheet")
    }

    /// Adds a new link to the HTML head.
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let page = HtmlPage::new()
    ///     .add_head_link("favicon.ico", "icon")
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     page,
    ///     r#"<!DOCTYPE html><html><head><link href="favicon.ico" rel="icon"></head><body></body></html>"#
    /// )
    /// ```
    pub fn add_head_link(mut self, href: &str, rel: &str) -> Self {
        let link = HeadContent::Link {
            href: href.into(),
            rel: rel.into(),
            attr: Attributes::default(),
        };
        self.head.push(Box::new(link));
        self
    }

    /// Adds a new link to the HTML head with the specified additional attributes
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// use maplit::hashmap;
    ///
    /// let page = HtmlPage::new()
    ///     .add_head_link_attr("print.css", "stylesheet", hashmap! {"media" => "print"})
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     page,
    ///     r#"<!DOCTYPE html><html><head><link href="print.css" rel="stylesheet" media="print"></head><body></body></html>"#
    /// )
    /// ```
    pub fn add_head_link_attr(
        mut self,
        href: &str,
        rel: &str,
        attributes: HashMap<&str, &str>,
    ) -> Self {
        let link = HeadContent::Link {
            href: href.into(),
            rel: rel.into(),
            attr: Attributes::from(attributes),
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
