//! This module contains structures used for HTML containers, which can contain other HTML
//! elements. Containers are items such as lists, divs, and articles.
//!
//! Containers implement

use crate::content::{Link, TextContent, TextContentType};
use crate::Html;

use std::fmt::{self, Debug, Display};
use std::marker::Sized;

/// An HTML element that can contain other HTML elements
///
/// HTML containers include tags such as: `article`, `div`, `ol`, `ul`.
pub trait HtmlContainer: Html + Sized {
    /// Adds the specified HTML element to this container
    fn add_html(self, html: Box<dyn Html>) -> Self;

    /// Adds an `<a>` tag to this container
    fn add_a(self, href: &str, text: &str) -> Self {
        let content = Link::new(href, text);
        self.add_html(Box::new(content))
    }

    /// Nest the specified container within this container
    fn add_container(self, container: Container) -> Self {
        self.add_html(Box::new(container))
    }

    /// Adds a header tag with the designated level to this container
    fn add_h(self, level: u8, text: &str) -> Self {
        let content = TextContent::new(TextContentType::Header(level), text);
        self.add_html(Box::new(content))
    }

    /// Adds a `<p>` tag element to this Container
    fn add_p(self, text: &str) -> Self {
        let content = TextContent::new(TextContentType::Paragraph, text);
        self.add_html(Box::new(content))
    }

    /// Adds a `<pre>` tag element to this container
    fn add_pre(self, text: &str) -> Self {
        let content = TextContent::new(TextContentType::Preformatted, text);
        self.add_html(Box::new(content))
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

        format!("<{}>{}</{}>", self.tag, content, self.tag)
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod html_container {
        use super::*;
        use test_case::test_case;

        #[test_case(Container::new(ContainerType::Div), "rust-lang.org", "rust", r#"<div><a href="rust-lang.org">rust</a></div>"#; "test_div")]
        #[test_case(Container::new(ContainerType::OrderedList), "abc", "123", r#"<ol><li><a href="abc">123</a></li></ol>"#; "test_ordered_list")]
        #[test_case(Container::new(ContainerType::UnorderedList), "abc", "123", r#"<ul><li><a href="abc">123</a></li></ul>"#; "test_unordered_list")]
        fn test_add_a(container: Container, href: &str, text: &str, expected: &str) {
            // Act
            let actual = container.add_a(href, text).to_html_string();

            // Assert
            assert_eq!(actual, expected);
        }

        #[test_case(Container::new(ContainerType::Div), "hello world", "<div><p>hello world</p></div>"; "test_div")]
        #[test_case(Container::new(ContainerType::OrderedList), "hello world", "<ol><li><p>hello world</p></li></ol>"; "test_ordered_list")]
        #[test_case(Container::new(ContainerType::UnorderedList), "hello world", "<ul><li><p>hello world</p></li></ul>"; "test_unordered_list")]
        fn test_add_p(container: Container, text: &str, expected: &str) {
            // Act
            let actual = container.add_p(text).to_html_string();

            // Assert
            assert_eq!(actual, expected);
        }
    }
}
