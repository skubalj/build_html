//! Defines the `HtmlContainer` Trait

use crate::attributes::Attributes;
use crate::content::BodyContent;
use crate::Container;
use crate::Html;
use crate::Table;
use std::collections::HashMap;
use std::fmt::Display;

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

    /// Nest the specified `Table` within this container
    fn add_table(self, table: Table) -> Self {
        self.add_html(Box::new(table))
    }

    /// Adds a header tag with the designated level to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let content = Container::default()
    ///     .add_header(1, "Header Text")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><h1>Header Text</h1></div>"#)
    /// ```
    fn add_header<T: Display>(self, level: u8, text: T) -> Self {
        let content = BodyContent::Header {
            level,
            content: format!("{}", text),
            attr: Attributes::default(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a header tag with the designated level and attributes to this container.
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// # use maplit::hashmap;
    /// let content = Container::default()
    ///     .add_header_attr(1, "Header Text", hashmap! {"id" => "main-header"})
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><h1 id="main-header">Header Text</h1></div>"#)
    /// ```
    fn add_header_attr<T: Display>(self, level: u8, text: T, attr: HashMap<&str, &str>) -> Self {
        let content = BodyContent::Header {
            level,
            content: format!("{}", text),
            attr: attr.into(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<img>` tag to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let content = Container::default()
    ///     .add_image("myimage.png", "a test image")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><img src="myimage.png" alt="a test image"></div>"#)
    /// ```
    fn add_image(self, src: &str, alt: &str) -> Self {
        let content = BodyContent::Image {
            src: src.into(),
            alt: alt.into(),
            attr: Attributes::default(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<img>` tag with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// # use maplit::hashmap;
    /// let content = Container::default()
    ///     .add_image_attr("myimage.png", "a test image", hashmap! {"id" => "sample-image"})
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     r#"<div><img src="myimage.png" alt="a test image" id="sample-image"></div>"#
    /// )
    /// ```
    fn add_image_attr(self, src: &str, alt: &str, attr: HashMap<&str, &str>) -> Self {
        let content = BodyContent::Image {
            src: src.into(),
            alt: alt.into(),
            attr: attr.into(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<a>` tag to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let content = Container::default()
    ///     .add_link("https://rust-lang.org/", "Rust Homepage")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><a href="https://rust-lang.org/">Rust Homepage</a></div>"#)
    /// ```
    fn add_link<T: Display>(self, href: &str, text: T) -> Self {
        let content = BodyContent::Link {
            href: href.into(),
            content: format!("{}", text),
            attr: Attributes::default(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<a>` tag with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// # use maplit::hashmap;
    /// let content = Container::default()
    ///     .add_link_attr("https://rust-lang.org/", "Rust Homepage", hashmap! {"class" => "links"})
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     r#"<div><a href="https://rust-lang.org/" class="links">Rust Homepage</a></div>"#
    /// )
    /// ```
    fn add_link_attr<T: Display>(self, href: &str, text: T, attr: HashMap<&str, &str>) -> Self {
        let content = BodyContent::Link {
            href: href.into(),
            content: format!("{}", text),
            attr: attr.into(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<p>` tag element to this Container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let content = Container::default()
    ///     .add_paragraph("This is sample paragraph text")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><p>This is sample paragraph text</p></div>"#)
    /// ```
    fn add_paragraph<T: Display>(self, text: T) -> Self {
        let content = BodyContent::Paragraph {
            content: format!("{}", text),
            attr: Attributes::default(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<p>` tag element with the specified attributes to this Container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// # use maplit::hashmap;
    /// let content = Container::default()
    ///     .add_paragraph_attr("This is sample paragraph text", hashmap! {"class" => "text"})
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><p class="text">This is sample paragraph text</p></div>"#)
    /// ```
    fn add_paragraph_attr<T: Display>(self, text: T, attr: HashMap<&str, &str>) -> Self {
        let content = BodyContent::Paragraph {
            content: format!("{}", text),
            attr: attr.into(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<pre>` tag element to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// let content = Container::default()
    ///     .add_preformatted("This | is   preformatted => text")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><pre>This | is   preformatted => text</pre></div>"#)
    /// ```
    fn add_preformatted<T: Display>(self, text: T) -> Self {
        let content = BodyContent::Preformatted {
            content: format!("{}", text),
            attr: Attributes::default(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<pre>` tag element with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// # use maplit::hashmap;
    /// let content = Container::default()
    ///     .add_preformatted_attr("This | is   preformatted => text", hashmap! {"id" => "code"})
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><pre id="code">This | is   preformatted => text</pre></div>"#)
    /// ```
    fn add_preformatted_attr<T: Display>(self, text: T, attr: HashMap<&str, &str>) -> Self {
        let content = BodyContent::Preformatted {
            content: format!("{}", text),
            attr: attr.into(),
        };
        self.add_html(Box::new(content))
    }
}
