//! This module contains the `HtmlPage` struct, which serves as the major entry point for the program

use crate::attributes::Attributes;
use crate::html_container::HtmlContainer;
use crate::Html;

mod header_content;

/// This struct represents an entire page of HTML which can built up by chaining addition methods.
///
/// To convert an `HtmlPage` to a [`String`] which can be sent back to a client, use the
/// [`Html::to_html_string()`] method
///
/// # Example
/// ```
/// # use build_html::*;
/// let page: String = HtmlPage::new()
///     .with_title("My Page")
///     .with_header(1, "Header Text")
///     .to_html_string();
///
/// assert_eq!(page, concat!(
///     "<!DOCTYPE html><html><head><title>My Page</title></head>",
///     "<body><h1>Header Text</h1></body></html>"
/// ));
/// ```
#[derive(Debug)]
pub struct HtmlPage {
    head: Vec<String>,
    body: Vec<String>,
}

impl Html for HtmlPage {
    fn to_html_string(&self) -> String {
        format!(
            "<!DOCTYPE html><html><head>{}</head><body>{}</body></html>",
            self.head.join(""),
            self.body.join("")
        )
    }
}

impl HtmlContainer for HtmlPage {
    #[inline]
    fn add_html<H: Html>(&mut self, html: H) {
        self.body.push(html.to_html_string());
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

    /// Helper function similar to [`HtmlContainer::add_html`]
    #[inline]
    fn add_html_head<H: Html>(&mut self, html: H) {
        self.head.push(html.to_html_string());
    }

    /// Helper function similar to [`HtmlContainer::with_html`]
    #[inline]
    fn with_html_head<H: Html>(mut self, html: H) -> Self {
        self.add_html_head(html);
        self
    }

    /// Adds a new link element to the HTML head.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut page = HtmlPage::new();
    /// page.add_head_link("favicon.ico", "icon");
    ///
    /// assert_eq!(page.to_html_string(), concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<link href="favicon.ico" rel="icon">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_head_link(&mut self, href: impl ToString, rel: impl ToString) {
        self.add_html_head(header_content::Link {
            href: href.to_string(),
            rel: rel.to_string(),
            attr: Attributes::default(),
        })
    }

    /// Adds a new link to the HTML head.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .with_head_link("favicon.ico", "icon")
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<link href="favicon.ico" rel="icon">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn with_head_link(self, href: impl ToString, rel: impl ToString) -> Self {
        self.with_html_head(header_content::Link {
            href: href.to_string(),
            rel: rel.to_string(),
            attr: Attributes::default(),
        })
    }

    /// Adds a new link to the HTML head with the specified additional attributes
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut page = HtmlPage::new();
    /// page.add_head_link_attr("print.css", "stylesheet", [("media", "print")]);
    ///
    /// assert_eq!(page.to_html_string(), concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<link href="print.css" rel="stylesheet" media="print">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_head_link_attr<A, S>(&mut self, href: impl ToString, rel: impl ToString, attr: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.add_html_head(header_content::Link {
            href: href.to_string(),
            rel: rel.to_string(),
            attr: attr.into(),
        })
    }

    /// Adds a new link to the HTML head with the specified additional attributes
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .with_head_link_attr("print.css", "stylesheet", [("media", "print")])
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<link href="print.css" rel="stylesheet" media="print">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn with_head_link_attr<A, S>(self, href: impl ToString, rel: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.with_html_head(header_content::Link {
            href: href.to_string(),
            rel: rel.to_string(),
            attr: attr.into(),
        })
    }

    /// Adds the specified metadata elements to this `HtmlPage`
    ///
    /// Attributes are specified in a `HashMap`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut page = HtmlPage::new();
    /// page.add_meta(vec![("charset", "utf-8")]);
    ///
    /// assert_eq!(page.to_html_string(), concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<meta charset="utf-8">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_meta<A, S>(&mut self, attributes: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.add_html_head(header_content::Meta {
            attr: attributes.into(),
        })
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
    ///     .with_meta(vec![("charset", "utf-8")])
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<meta charset="utf-8">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn with_meta<A, S>(self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.with_html_head(header_content::Meta {
            attr: attributes.into(),
        })
    }

    /// Adds the specified external script to the `HtmlPage`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut page = HtmlPage::new();
    /// page.add_script_link("myScript.js");
    ///
    /// assert_eq!(page.to_html_string(), concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<script src="myScript.js"></script>"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_script_link(&mut self, src: impl ToString) {
        self.add_html_head(header_content::ScriptLink {
            src: src.to_string(),
            attr: Attributes::default(),
        })
    }

    /// Adds the specified external script to the `HtmlPage`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .with_script_link("myScript.js")
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<script src="myScript.js"></script>"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn with_script_link(self, src: impl ToString) -> Self {
        self.with_html_head(header_content::ScriptLink {
            src: src.to_string(),
            attr: Attributes::default(),
        })
    }

    /// Adds a script link with additional attributes to the `HtmlPage`
    pub fn add_script_link_attr<A, S>(&mut self, src: impl ToString, attributes: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.add_html_head(header_content::ScriptLink {
            src: src.to_string(),
            attr: attributes.into(),
        })
    }

    /// Adds a script link with additional attributes to the `HtmlPage`
    pub fn with_script_link_attr<A, S>(self, src: impl ToString, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.with_html_head(header_content::ScriptLink {
            src: src.to_string(),
            attr: attributes.into(),
        })
    }

    /// Adds the specified script to this `HtmlPage`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut page = HtmlPage::new();
    /// page.add_script_literal(r#"window.onload = () => console.log("Hello World");"#);
    ///
    /// assert_eq!(page.to_html_string(), concat!(
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
    /// ```rust, ignore (cannot-doctest-external-file-dependency)
    /// let mut page = HtmlPage::new();
    /// page.add_script_literal(include_str!("myScript.js"));
    /// ```
    pub fn add_script_literal(&mut self, code: impl ToString) {
        self.add_html_head(header_content::ScriptLiteral {
            code: code.to_string(),
        })
    }

    /// Adds the specified script to this `HtmlPage`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .with_script_literal(r#"window.onload = () => console.log("Hello World");"#)
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
    ///     .with_script_literal(include_str!("myScript.js"))
    ///     .to_html_string();
    /// ```
    pub fn with_script_literal(self, code: impl ToString) -> Self {
        self.with_html_head(header_content::ScriptLiteral {
            code: code.to_string(),
        })
    }

    /// Adds raw style data to this `HtmlPage`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut page = HtmlPage::new();
    /// page.add_style(r#"p{font-family:"Liberation Serif";}"#);
    ///
    /// assert_eq!(page.to_html_string(), concat!(
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
    /// let mut page = HtmlPage::new();
    /// page.add_style(include_str!("styles.css"));
    /// ```
    pub fn add_style(&mut self, css: impl ToString) {
        self.add_html_head(header_content::Style {
            css: css.to_string(),
            attr: Attributes::default(),
        })
    }

    /// Adds raw style data to this `HtmlPage`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .with_style(r#"p{font-family:"Liberation Serif";}"#)
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
    ///     .with_style(include_str!("styles.css"))
    ///     .to_html_string();
    /// ```
    pub fn with_style(self, css: impl ToString) -> Self {
        self.with_html_head(header_content::Style {
            css: css.to_string(),
            attr: Attributes::default(),
        })
    }

    /// Adds the specified style data with the specified attributes
    pub fn add_style_attr<A, S>(&mut self, css: impl ToString, attributes: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.add_html_head(header_content::Style {
            css: css.to_string(),
            attr: attributes.into(),
        })
    }

    /// Adds the specified style data with the specified attributes
    pub fn with_style_attr<A, S>(self, css: impl ToString, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.with_html_head(header_content::Style {
            css: css.to_string(),
            attr: attributes.into(),
        })
    }

    /// Adds the specified stylesheet to the HTML head.
    ///
    /// This method uses [`add_head_link`](HtmlPage::add_head_link) internally
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut page = HtmlPage::new();
    /// page.add_stylesheet("print.css");
    ///
    /// assert_eq!(page.to_html_string(), concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<link href="print.css" rel="stylesheet">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    #[inline]
    pub fn add_stylesheet(&mut self, source: impl ToString) {
        self.add_head_link(source, "stylesheet")
    }

    /// Adds the specified stylesheet to the HTML head.
    ///
    /// This method uses [`add_head_link`](HtmlPage::add_head_link) internally
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .with_stylesheet("print.css")
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     r#"<link href="print.css" rel="stylesheet">"#,
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    #[inline]
    pub fn with_stylesheet(self, source: impl ToString) -> Self {
        self.with_head_link(source, "stylesheet")
    }

    /// Adds a title to this HTML page
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut page = HtmlPage::new();
    /// page.add_title("My Page");
    ///
    /// assert_eq!(page.to_html_string(), concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     "<title>My Page</title>",
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn add_title(&mut self, title_text: impl ToString) {
        self.add_html_head(header_content::Title {
            content: title_text.to_string(),
        })
    }

    /// Adds a title to this HTML page
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let page = HtmlPage::new()
    ///     .with_title("My Page")
    ///     .to_html_string();
    ///
    /// assert_eq!(page, concat!(
    ///     "<!DOCTYPE html><html><head>",
    ///     "<title>My Page</title>",
    ///     "</head><body></body></html>"
    /// ));
    /// ```
    pub fn with_title(self, title_text: impl ToString) -> Self {
        self.with_html_head(header_content::Title {
            content: title_text.to_string(),
        })
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
