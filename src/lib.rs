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
//!     .add_header(1, "Main Content:")
//!     .add_container(
//!         Container::new(ContainerType::Article)
//!             .add_header(2, "Hello, World")
//!             .add_paragraph("This is a simple HTML demo")
//!     )
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
//!         <h1>Main Content:</h1>
//!         <article>
//!             <h2>Hello World</h2>
//!             <p>This is a simple HTML demo</p>
//!         </article>
//!     </body>
//! </html>
//! ```
//!

use attributes::Attributes;
use content::{BodyContent, HeadContent};
use std::fmt::{self, Display};

mod attributes;
mod content;

/// An element that can be converted to HTML
pub trait Html: fmt::Debug {
    /// Convert this element into an HTML string
    fn to_html_string(&self) -> String;
}

/// An HTML element that can contain other HTML elements
///
/// HTML containers include tags such as: `article`, `div`, `ol`, `ul`.
pub trait HtmlContainer: Html + Sized {
    /// Adds the specified HTML element to this container
    fn add_html(self, html: Box<dyn Html>) -> Self;

    /// Nest the specified container within this container
    fn add_container(self, container: Container) -> Self {
        self.add_html(Box::new(container))
    }

    /// Adds a header tag with the designated level to this container
    fn add_header(self, level: u8, text: &str) -> Self {
        let content = BodyContent::Header {
            level,
            content: text.into(),
            attr: Attributes::empty(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<img>` tag to this container
    fn add_image(self, src: &str, alt: &str) -> Self {
        let content = BodyContent::Image {
            src: src.into(),
            alt: alt.into(),
            attr: Attributes::empty(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<a>` tag to this container
    fn add_link(self, href: &str, text: &str) -> Self {
        let content = BodyContent::Link {
            href: href.into(),
            content: text.into(),
            attr: Attributes::empty(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<p>` tag element to this Container
    fn add_paragraph(self, text: &str) -> Self {
        let content = BodyContent::Paragraph {
            content: text.into(),
            attr: Attributes::empty(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<pre>` tag element to this container
    fn add_preformatted(self, text: &str) -> Self {
        let content = BodyContent::Preformatted {
            content: text.into(),
            attr: Attributes::empty(),
        };
        self.add_html(Box::new(content))
    }
}

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
    pub fn add_title(mut self, title_text: &str) -> Self {
        let title = HeadContent::Title {
            content: title_text.into(),
        };
        self.head.push(Box::new(title));
        self
    }
}

/// The different types of Html Containers that can be added to the page
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContainerType {
    Article,
    Div,
    Main,
    OrderedList,
    UnorderedList,
}

impl Display for ContainerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContainerType::Article => write!(f, "article"),
            ContainerType::Div => write!(f, "div"),
            ContainerType::Main => write!(f, "main"),
            ContainerType::OrderedList => write!(f, "ol"),
            ContainerType::UnorderedList => write!(f, "ul"),
        }
    }
}

/// A container for HTML elements.
///
/// As the name would suggest, a `Container` contains other HTML elements. This struct guarantees
/// that the elements added will be converted to HTML strings in the same order as they were
/// added.
///
/// Supported container types are provided by the [`ContainerType`] enum. This struct is what
/// allows Lists (`<ol>` / `<ul>`) as well as `<div>`s to be added to the `HtmlPage`
#[derive(Debug)]
pub struct Container {
    tag: ContainerType,
    elements: Vec<Box<dyn Html>>,
    attr: Attributes,
}

impl Html for Container {
    fn to_html_string(&self) -> String {
        let content = match self.tag {
            ContainerType::OrderedList | ContainerType::UnorderedList => self
                .elements
                .iter()
                .map(|item| format!("<li>{}</li>", item.to_html_string()))
                .fold(String::new(), |acc, next| acc + &next),
            _ => self
                .elements
                .iter()
                .map(|item| item.to_html_string())
                .fold(String::new(), |acc, next| acc + &next),
        };

        format!("<{}{}>{}</{}>", self.tag, self.attr, content, self.tag)
    }
}

impl HtmlContainer for Container {
    fn add_html(mut self, content: Box<dyn Html>) -> Self {
        self.elements.push(content);
        self
    }
}

impl Default for Container {
    fn default() -> Self {
        Container::new(ContainerType::Div)
    }
}

impl Container {
    /// Creates a new container with the specified tag.
    pub fn new(tag: ContainerType) -> Self {
        Container {
            tag,
            elements: Vec::new(),
            attr: Attributes::empty(),
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
                "<!DOCTYPE html><html><head></head><body></body></html>"
            )
        }
    }

    mod container {
        use super::*;
        use test_case::test_case;

        #[test_case(Container::new(ContainerType::Div), "rust-lang.org", "rust", r#"<div><a href="rust-lang.org">rust</a></div>"#; "test_div")]
        #[test_case(Container::new(ContainerType::OrderedList), "abc", "123", r#"<ol><li><a href="abc">123</a></li></ol>"#; "test_ordered_list")]
        #[test_case(Container::new(ContainerType::UnorderedList), "abc", "123", r#"<ul><li><a href="abc">123</a></li></ul>"#; "test_unordered_list")]
        fn test_add_link(container: Container, href: &str, text: &str, expected: &str) {
            // Act
            let actual = container.add_link(href, text).to_html_string();

            // Assert
            assert_eq!(actual, expected);
        }

        #[test_case(Container::new(ContainerType::Div), "hello world", "<div><p>hello world</p></div>"; "test_div")]
        #[test_case(Container::new(ContainerType::OrderedList), "hello world", "<ol><li><p>hello world</p></li></ol>"; "test_ordered_list")]
        #[test_case(Container::new(ContainerType::UnorderedList), "hello world", "<ul><li><p>hello world</p></li></ul>"; "test_unordered_list")]
        fn test_add_paragraph(container: Container, text: &str, expected: &str) {
            // Act
            let actual = container.add_paragraph(text).to_html_string();

            // Assert
            assert_eq!(actual, expected);
        }
    }
}
