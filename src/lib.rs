use std::fmt::{self, Display};
use std::marker::Sized;

use containers::Container;
use content::{TextContent, TextContentType};

pub mod containers;
mod content;
mod metadata;

/// An element that can be converted to HTML
pub trait Html: fmt::Debug {
    /// Convert this element into an HTML string
    fn to_html_string(&self) -> String;
}

/// An HTML element that can contain other HTML elements
pub trait HtmlContainer: Html + Sized {
    /// Adds a header tag with the designated level to this container
    fn add_h(self, level: u8, text: &str) -> Self {
        let content = TextContent::new(TextContentType::Header(level), text);
        self.add_text(content)
    }

    /// Nest the specified container within this container
    fn add_container(self, container: Container) -> Self;

    /// Adds a `<p>` tag element to this Container
    fn add_p(self, text: &str) -> Self {
        let content = TextContent::new(TextContentType::Paragraph, text);
        self.add_text(content)
    }

    /// Adds a `<pre>` tag element to this container
    fn add_pre(self, text: &str) -> Self {
        let content = TextContent::new(TextContentType::Preformatted, text);
        self.add_text(content)
    }

    /// Adds the specified text content element to this container
    fn add_text(self, content: TextContent) -> Self;
}

/// This struct represents an entire page of HTML which can built up by chaining addition methods.
/// This creates an effect similar to the [Decorator Pattern](https://en.wikipedia.org/wiki/Decorator_pattern)
///
/// To convert an `HtmlPage` to a [`String`] which can be sent back to a client, use the
/// [`Html::to_html_string()`] method
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
    fn add_container(mut self, container: Container) -> Self {
        self.body.push(Box::new(container));
        self
    }

    fn add_text(mut self, content: TextContent) -> Self {
        self.body.push(Box::new(content));
        self
    }
}

impl Default for HtmlPage {
    fn default() -> Self {
        HtmlPage::new().add_title("Default Page")
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
}

#[cfg(test)]
mod tests {
    use super::*;

    mod html_page {
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
                "<!DOCTYPE html><html><head><title>Default Page</title></head><body></body></html>"
            )
        }
    }
}
