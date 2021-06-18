//! This module contains the `HtmlPage` struct, which serves as the major entry point for the program

use crate::attributes::Attributes;
use crate::content::HeadContent;
use crate::html_container::HtmlContainer;
use crate::Html;
use std::collections::HashMap;

/// This struct represents an entire page of HTML which can built up by chaining addition methods.
///
/// To convert an `HtmlPage` to a [`String`] which can be sent back to a client, use the
/// [`Html::to_html_string()`] method
///
/// # Example
/// ```
/// # use build_html::*;
/// let page: String = HtmlPage::new()
///     .add_title("My Page")
///     .add_header(1, "Header Text")
///     .to_html_string();
///
/// assert_eq!(page, concat!(
///     "<!DOCTYPE html><html><head><title>My Page</title></head>",
///     "<body><h1>Header Text</h1></body></html>"
/// ));
/// ```
#[derive(Debug)]
pub struct HtmlPage {
    head: Vec<HeadContent>,
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

impl HtmlPage {
    /// Creates a new HTML page with no content
    pub fn new() -> Self {
        HtmlPage {
            head: Vec::new(),
            body: Vec::new(),
        }
    }

    /// Adds a new link to the HTML head.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .add_head_link("favicon.ico", "icon")
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<link href="favicon.ico" rel="icon">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_head_link(mut self, href: &str, rel: &str) -> Self {
        let link = HeadContent::Link {
            href: href.into(),
            rel: rel.into(),
            attr: Attributes::default(),
        };
        self.head.push(link);
        self
    }

    /// Adds a new link to the HTML head with the specified additional attributes
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .add_head_link_attr("print.css", "stylesheet", [("media", "print")])
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<link href="print.css" rel="stylesheet" media="print">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_head_link_attr<A, S>(
        mut self,
        href: impl ToString,
        rel: impl ToString,
        attr: A,
    ) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let link = HeadContent::Link {
            href: href.to_string(),
            rel: rel.to_string(),
            attr: attr.into(),
        };
        self.head.push(link);
        self
    }

    /// Adds the specified metadata elements to this `HtmlPage`
    ///
    /// Attributes are specified in a `HashMap`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    ///
    /// let page = HtmlPage::new()
    ///     .add_meta(vec![("charset", "utf-8")])
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<meta charset="utf-8">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_meta<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let meta = HeadContent::Meta {
            attr: attributes.into(),
        };
        self.head.push(meta);
        self
    }

    /// Adds the specified external script to the `HtmlPage`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .add_script_link("myScript.js")
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<script src="myScript.js"></script>"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_script_link(mut self, src: impl ToString) -> Self {
        let script = HeadContent::ScriptLink {
            src: src.to_string(),
            attr: Attributes::default(),
        };
        self.head.push(script);
        self
    }

    pub fn add_script_link_attr<A, S>(mut self, src: impl ToString, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let script = HeadContent::ScriptLink {
            src: src.to_string(),
            attr: attributes.into(),
        };
        self.head.push(script);
        self
    }

    /// Adds the specified script to this `HtmlPage`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .add_script_literal(r#"window.onload = () => console.log("Hello World");"#)
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head><script>",
    ///     r#"window.onload = () => console.log("Hello World");"#,
    ///     "</script></head><body></body></html>"
    /// ));
    /// ```
    ///
    /// In order to lint the code, it can be helpful to define your script in
    /// its own file. That file can be inserted into the html page using the
    /// [`include_str`] macro:
    ///
    /// ```ignore (cannot-doctest-external-file-dependency)
    /// let page = HtmlPage::new()
    ///     .add_script_literal(include_str!("myScript.js"))
    ///     .to_html_string();
    /// ```
    pub fn add_script_literal(mut self, code: impl ToString) -> Self {
        let script = HeadContent::ScriptLiteral {
            code: code.to_string(),
        };
        self.head.push(script);
        self
    }

    /// Adds raw style data to this `HtmlPage`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .add_style(r#"p{font-family:"Liberation Serif";}"#)
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<style>p{font-family:"Liberation Serif";}</style>"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    ///
    /// To allow for linting, it can be helpful to define CSS in its own file.
    /// That file can be included at compile time using the [`include_str`] macro:
    ///
    /// ```ignore (cannot-doctest-external-file-dependency)
    /// let page = HtmlPage::new()
    ///     .add_style(include_str!("styles.css"))
    ///     .to_html_string();
    /// ```
    pub fn add_style(mut self, css: &str) -> Self {
        let style = HeadContent::Style {
            css: css.into(),
            attr: Attributes::default(),
        };
        self.head.push(style);
        self
    }

    /// Adds the specified style data with the specified attributes
    pub fn add_style_attr(mut self, css: &str, attributes: HashMap<&str, &str>) -> Self {
        let style = HeadContent::Style {
            css: css.into(),
            attr: attributes.into(),
        };
        self.head.push(style);
        self
    }

    /// Adds the specified stylesheet to the HTML head.
    ///
    /// This method uses [`add_head_link`](HtmlPage::add_head_link) internally
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .add_stylesheet("print.css")
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<link href="print.css" rel="stylesheet">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_stylesheet(self, source: &str) -> Self {
        self.add_head_link(source, "stylesheet")
    }

    /// Adds a title to this HTML page
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .add_title("My Page")
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     "<title>My Page</title>",
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_title(mut self, title_text: &str) -> Self {
        let title = HeadContent::Title {
            content: title_text.into(),
        };
        self.head.push(title);
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
