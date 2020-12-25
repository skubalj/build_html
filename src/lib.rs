//! This library is designed to provide a simple way to generate HTML documents dynamically from
//! within Rust code. To generate documents, this library uses the decorator pattern,
//!
//! # Example
//!
//! ```rust
//! use html_gen::*;
//! use maplit::hashmap;
//!
//! let html: String = HtmlPage::new()
//!     .add_title("My Page")
//!     .add_header(1, "Main Content:", None)
//!     .add_container(
//!         Container::new(ContainerType::Article)
//!             .add_header(2, "Hello, World", Some(hashmap! {"id" => "article-head"}))
//!             .add_paragraph("This is a simple HTML demo", None)
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
//!             <h2 id="article-head">Hello World</h2>
//!             <p>This is a simple HTML demo</p>
//!         </article>
//!     </body>
//! </html>
//! ```
//!

use attributes::Attributes;
use content::{BodyContent, HeadContent};
use std::collections::HashMap;
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
/// This trait implements the majority of the specific "add x" methods, requiring implementors
/// to add only one method: [`add_html()`](crate::HtmlContainer::add_html)
pub trait HtmlContainer: Html + Sized {
    /// Adds the specified HTML element to this container
    fn add_html(self, html: Box<dyn Html>) -> Self;

    /// Nest the specified container within this container
    fn add_container(self, container: Container) -> Self {
        self.add_html(Box::new(container))
    }

    /// Adds a header tag with the designated level to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// # use maplit::hashmap;
    /// let content = Container::default()
    ///     .add_header(1, "Header Text", Some(hashmap! {"id" => "main-header"}))
    ///     .add_header(2, "Sub-header Text", None)
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><h1 id="main-header">Header Text</h1><h2>Sub-header Text</h2></div>"#)
    /// ```
    fn add_header(self, level: u8, text: &str, attributes: Option<HashMap<&str, &str>>) -> Self {
        let content = BodyContent::Header {
            level,
            content: text.into(),
            attr: attributes
                .map(|map| Attributes::from(map))
                .unwrap_or(Attributes::empty()),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<img>` tag to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let content = Container::default()
    ///     .add_image("myimage.png", "a test image", None)
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><img src="myimage.png" alt="a test image"></div>"#)
    /// ```
    fn add_image(self, src: &str, alt: &str, attributes: Option<HashMap<&str, &str>>) -> Self {
        let content = BodyContent::Image {
            src: src.into(),
            alt: alt.into(),
            attr: attributes
                .map(|map| Attributes::from(map))
                .unwrap_or(Attributes::empty()),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<a>` tag to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// # use maplit::hashmap;
    /// let content = Container::default()
    ///     .add_link("https://rust-lang.org/", "Rust Homepage", None)
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><a href="https://rust-lang.org/">Rust Homepage</a></div>"#)
    /// ```
    fn add_link(self, href: &str, text: &str, attributes: Option<HashMap<&str, &str>>) -> Self {
        let content = BodyContent::Link {
            href: href.into(),
            content: text.into(),
            attr: attributes
                .map(|map| Attributes::from(map))
                .unwrap_or(Attributes::empty()),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<p>` tag element to this Container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// # use maplit::hashmap;
    /// let content = Container::default()
    ///     .add_paragraph("This is sample paragraph text", Some(hashmap! {"class" => "text"}))
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><p class="text">This is sample paragraph text</p></div>"#)
    /// ```
    fn add_paragraph(self, text: &str, attributes: Option<HashMap<&str, &str>>) -> Self {
        let content = BodyContent::Paragraph {
            content: text.into(),
            attr: attributes
                .map(|map| Attributes::from(map))
                .unwrap_or(Attributes::empty()),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<pre>` tag element to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let content = Container::default()
    ///     .add_preformatted("This | is   preformatted => text", None)
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><pre>This | is   preformatted => text</pre></div>"#)
    /// ```
    fn add_preformatted(self, text: &str, attributes: Option<HashMap<&str, &str>>) -> Self {
        let content = BodyContent::Preformatted {
            content: text.into(),
            attr: attributes
                .map(|map| Attributes::from(map))
                .unwrap_or(Attributes::empty()),
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
///     .add_header(1, "Header Text", None)
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
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let page = HtmlPage::new()
    ///     .add_title("My Page")
    ///     .to_html_string();
    ///
    /// assert_eq!(page, "<!DOCTYPE html><html><head><title>My Page</title></head><body></body></html>")
    /// ```
    pub fn add_title(mut self, title_text: &str) -> Self {
        let title = HeadContent::Title {
            content: title_text.into(),
        };
        self.head.push(Box::new(title));
        self
    }

    /// Adds a style to this HTML page
    pub fn add_style(mut self, css: &str, attributes: Option<HashMap<&str, &str>>) -> Self {
        let style = HeadContent::Style {
            css: css.into(),
            attr: attributes
                .map(|map| Attributes::from(map))
                .unwrap_or_default(),
        };
        self.head.push(Box::new(style));
        self
    }
}

/// The different types of Html Containers that can be added to the page
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
                .add_header(1, "header", Some(hashmap! {"id" => "main-header"}))
                .add_image("myimage.png", "test image", None)
                .add_link("rust-lang.org", "Rust Home", None)
                .add_paragraph("Sample Text", Some(hashmap! {"class" => "red-text"}))
                .add_preformatted("Text", Some(hashmap! {"class" => "code"}));

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
                .add_header(1, "header", Some(hashmap! {"id" => "main-header"}))
                .add_image("myimage.png", "test image", None)
                .add_link("rust-lang.org", "Rust Home", None)
                .add_paragraph("Sample Text", Some(hashmap! {"class" => "red-text"}))
                .add_preformatted("Text", Some(hashmap! {"class" => "code"}));

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
}
