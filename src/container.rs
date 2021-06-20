//! This module contains information about containers and container types

use crate::attributes::Attributes;
use crate::html_container::HtmlContainer;
use crate::Html;
use std::fmt::{self, Display};

/// The different types of Html Containers that can be added to the page
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ContainerType {
    /// Corresponds to `<address>` tags
    Address,
    /// Corresponds to `<article>` tags
    Article,
    /// Corresponds to `<div>` tags
    ///
    /// This type is also the default for `Container`s
    Div,
    /// Corresponds to `<footer>` tags
    Footer,
    /// Corresponds to `<header>` tags
    Header,
    /// Corresponds to `<main>` tags
    Main,
    /// Corresponds to `<ol>` tags
    OrderedList,
    /// Corresponds to `<ul>` tags
    UnorderedList,
}

impl Display for ContainerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Address => write!(f, "address"),
            Self::Article => write!(f, "article"),
            Self::Div => write!(f, "div"),
            Self::Footer => write!(f, "footer"),
            Self::Header => write!(f, "header"),
            Self::Main => write!(f, "main"),
            Self::OrderedList => write!(f, "ol"),
            Self::UnorderedList => write!(f, "ul"),
        }
    }
}

/// A container for HTML elements.
///
/// As the name would suggest, a `Container` contains other HTML elements. This struct guarantees
/// that the elements added will be converted to HTML strings in the same order as they were
/// added.
///
/// Supported container types are provided by the [`ContainerType`] enum.
///
/// Note that `Container` elements can be nested inside of each other.
/// ```rust
/// # use build_html::*;
/// let text = Container::new(ContainerType::Main)
///     .with_header(1, "My Container")
///     .with_container(
///         Container::new(ContainerType::Article)
///             .with_container(
///                 Container::new(ContainerType::Div)
///                     .with_paragraph("Inner Text")
///             )
///     )
///     .to_html_string();
///
/// assert_eq!(
///     text,
///     "<main><h1>My Container</h1><article><div><p>Inner Text</p></div></article></main>"
/// );
/// ```
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

        format!(
            "<{tag}{attr}>{content}</{tag}>",
            tag = self.tag,
            attr = self.attr,
            content = content
        )
    }
}

impl HtmlContainer for Container {
    fn add_html(&mut self, content: Box<dyn Html>) {
        self.elements.push(content);
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
            attr: Attributes::default(),
        }
    }

    /// Associates the specified map of attributes with this Container.
    ///
    /// Note that this operation overrides all previous `with_attribute` calls on
    /// this `Container`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let container = Container::default()
    ///     .with_attributes(vec![("class", "defaults")])
    ///     .with_paragraph("text")
    ///     .to_html_string();
    ///
    /// assert_eq!(container, r#"<div class="defaults"><p>text</p></div>"#)
    /// ```
    pub fn with_attributes<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.attr = Attributes::from(attributes);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(ContainerType::Article; "article")]
    #[test_case(ContainerType::Div; "div")]
    #[test_case(ContainerType::Main; "main")]
    fn test_content(container_type: ContainerType) {
        // Expected
        let content = concat!(
            r#"<h1 id="main-header">header</h1>"#,
            r#"<img src="myimage.png" alt="test image">"#,
            r#"<a href="rust-lang.org">Rust Home</a>"#,
            r#"<p class="red-text">Sample Text</p>"#,
            r#"<pre class="code">Text</pre>"#
        );

        // Act
        let sut = Container::new(container_type)
            .with_header_attr(1, "header", [("id", "main-header")])
            .with_image("myimage.png", "test image")
            .with_link("rust-lang.org", "Rust Home")
            .with_paragraph_attr("Sample Text", [("class", "red-text")])
            .with_preformatted_attr("Text", [("class", "code")]);

        // Assert
        assert_eq!(
            sut.to_html_string(),
            format!(
                "<{tag}>{content}</{tag}>",
                tag = container_type,
                content = content
            )
        )
    }

    #[test_case(ContainerType::OrderedList; "ordered_list")]
    #[test_case(ContainerType::UnorderedList; "unordered_list")]
    fn test_list(container_type: ContainerType) {
        // Expected
        let content = concat!(
            r#"<li><h1 id="main-header">header</h1></li>"#,
            r#"<li><img src="myimage.png" alt="test image"></li>"#,
            r#"<li><a href="rust-lang.org">Rust Home</a></li>"#,
            r#"<li><p class="red-text">Sample Text</p></li>"#,
            r#"<li><pre class="code">Text</pre></li>"#
        );

        // Act
        let sut = Container::new(container_type)
            .with_header_attr(1, "header", [("id", "main-header")])
            .with_image("myimage.png", "test image")
            .with_link("rust-lang.org", "Rust Home")
            .with_paragraph_attr("Sample Text", [("class", "red-text")])
            .with_preformatted_attr("Text", [("class", "code")]);

        // Assert
        assert_eq!(
            sut.to_html_string(),
            format!(
                "<{tag}>{content}</{tag}>",
                tag = container_type,
                content = content
            )
        )
    }

    #[test]
    fn test_nesting() {
        // Act
        let container = Container::new(ContainerType::Main)
            .with_paragraph("paragraph")
            .with_container(
                Container::new(ContainerType::OrderedList)
                    .with_container(Container::default().with_paragraph(1))
                    .with_container(Container::default().with_paragraph('2'))
                    .with_container(Container::default().with_paragraph("3")),
            )
            .with_paragraph("done");

        // Assert
        assert_eq!(
            container.to_html_string(),
            concat!(
                "<main><p>paragraph</p><ol>",
                "<li><div><p>1</p></div></li>",
                "<li><div><p>2</p></div></li>",
                "<li><div><p>3</p></div></li>",
                "</ol><p>done</p></main>"
            )
        )
    }
}
