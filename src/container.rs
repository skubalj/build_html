use crate::attributes::Attributes;
use crate::html_container::HtmlContainer;
use crate::Html;
use std::collections::HashMap;
use std::fmt::{self, Display};

/// The different types of Html Containers that can be added to the page
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ContainerType {
    /// Corresponds to `<article>` tags
    Article,
    /// Corresponds to `<div>` tags
    ///
    /// This type is also the default for `Container`s
    Div,
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

        format!(
            "<{tag}{attr}>{content}</{tag}>",
            tag = self.tag,
            attr = self.attr,
            content = content
        )
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
    /// # use html_gen::*;
    /// # use maplit::hashmap;
    /// let container = Container::default()
    ///     .with_attributes(hashmap! {"class" => "defaults"})
    ///     .add_paragraph("text")
    ///     .to_html_string();
    ///
    /// assert_eq!(container, r#"<div class="defaults"><p>text</p></div>"#)
    /// ```
    pub fn with_attributes(mut self, attributes: HashMap<&str, &str>) -> Self {
        self.attr = Attributes::from(attributes);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use test_case::test_case;

    #[test_case(ContainerType::Article; "article")]
    #[test_case(ContainerType::Div; "div")]
    #[test_case(ContainerType::Main; "main")]
    fn test_nesting(container_type: ContainerType) {
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
            .add_header_attr(1, "header", hashmap! {"id" => "main-header"})
            .add_image("myimage.png", "test image")
            .add_link("rust-lang.org", "Rust Home")
            .add_paragraph_attr("Sample Text", hashmap! {"class" => "red-text"})
            .add_preformatted_attr("Text", hashmap! {"class" => "code"});

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
            .add_header_attr(1, "header", hashmap! {"id" => "main-header"})
            .add_image("myimage.png", "test image")
            .add_link("rust-lang.org", "Rust Home")
            .add_paragraph_attr("Sample Text", hashmap! {"class" => "red-text"})
            .add_preformatted_attr("Text", hashmap! {"class" => "code"});

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
}
