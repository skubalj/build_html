//! This library is designed to provide a simple way to generate HTML documents dynamically from
//! within Rust code. To generate documents, this library uses the decorator pattern,
//!
//! # Example
//!
//! ```rust
//! use html_gen::*;
//!
//! let html: String = HtmlPage::new()
//!     .add_title("My Page")
//!     .add_h(1, "Hello, World")
//!     .add_p("This is a simple HTML demo")
//!     .to_html_string();
//!    
//! println!("{}", html);
//! ```
//!
//! produces a string equivalent to:
//!
//! ```html
//! <!DOCTYPE html>
//! <html>
//!     <head>
//!         <title>My Page</title>
//!     </head>
//!     <body>
//!         <h1>Hello World</h1>
//!         <p>This is a simple HTML demo</p>
//!     </body>
//! </html>
//! ```
//!

use std::fmt::{self, Display};

pub use containers::HtmlContainer;
use content::{TextContent, TextContentType};

pub mod containers;
mod content;

/// An element that can be converted to HTML
pub trait Html: fmt::Debug {
    /// Convert this element into an HTML string
    fn to_html_string(&self) -> String;
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
    fn add_html(mut self, html: Box<dyn Html>) -> Self {
        self.body.push(html);
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

    /// Adds a title to this HTML page
    pub fn add_title(mut self, title_text: &str) -> Self {
        let title = TextContent::new(TextContentType::Title, title_text);
        self.head.push(Box::new(title));
        self
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
