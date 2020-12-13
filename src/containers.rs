//! This module contains structures used for HTML containers, which can contain other HTML elements

use crate::content::{TextContent, TextContentType};
use crate::{Html, HtmlContainer};

use std::fmt::{self, Debug, Display};

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
    fn add_container(mut self, container: Container) -> Self {
        self.elements.push(Box::new(container));
        self
    }

    fn add_text(mut self, content: TextContent) -> Self {
        self.elements.push(Box::new(content));
        self
    }
}

impl Default for Container {
    fn default() -> Self {
        Container::new(ContainerType::Div)
    }
}

impl Container {
    /// Creates a new list with the specified tag.
    pub fn new(tag: ContainerType) -> Self {
        Container {
            tag,
            elements: Vec::new(),
        }
    }
}
