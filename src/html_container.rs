//! Defines the `HtmlContainer` Trait

use crate::attributes::Attributes;
use crate::content::BodyContent;
use crate::Container;
use crate::Html;
use crate::Table;

/// An HTML element that can contain other HTML elements
///
/// The vast majority of methods on this trait are defined generically, allowing any type (or
/// combination of types) implementing [`ToString`] to be passed in. Thanks to monomorphization,
/// this can happen without incurring any runtime cost. For example:
///
/// ```
/// # use build_html::*;
/// # use std::net::Ipv4Addr;
/// let addr = Ipv4Addr::new(127, 0, 0, 1);
/// let content = Container::default().add_paragraph(addr).to_html_string();
/// assert_eq!(content, "<div><p>127.0.0.1</p></div>")
/// ```
///
/// Attributes can be passed in using any type that implements [`IntoIterator`] for 2-tuples of
/// objects implementing `ToString`. That includes (as of Rust 1.53) arrays of `&str`s, which are
/// very handy when content is known. For more dynamic attribute action, 
/// [`HashMap`](std::collections::HashMap)s can also be used.
///
/// ```
/// # use build_html::*;
/// let content = Container::default()
///     .add_paragraph_attr("123", [("id", "paragraph"), ("class", "action")])
///     .to_html_string();
/// assert_eq!(content, r#"<div><p id="paragraph" class="action">123</p></div>"#)
/// ```
///
/// # Implementing
///
/// This trait implements the majority of the specific "add x" methods, requiring implementors
/// to add only one method: [`add_html()`](crate::HtmlContainer::add_html)
pub trait HtmlContainer: Html + Sized {
    /// Adds the specified HTML element to this container
    ///
    /// This method can be used as an escape hatch to insert arbitrary types into the HTML document,
    /// helping to make up for those types which are not supported natively by this library. This
    /// can be done by defining your own types that implement the [`Html`] trait.
    ///
    /// If you need a simple one-off, it may be more convenient to insert the element as a raw
    /// string using [`add_raw`](HtmlContainer::add_raw) method instead
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// #[derive(Debug)]
    /// struct Span {
    ///     content: String
    /// }
    ///
    /// impl Span {
    ///     pub fn new(content: impl ToString) -> Self {
    ///         Span { content: content.to_string() }
    ///     }
    /// }
    ///
    /// impl Html for Span {
    ///     fn to_html_string(&self) -> String {
    ///         format!("<span>{}</span>", self.content)
    ///     }
    /// }
    ///
    /// let content = Container::default()
    ///     .add_html(Box::new(Span::new("inner")))
    ///     .to_html_string();
    /// assert_eq!(content, "<div><span>inner</span></div>");
    /// ```
    fn add_html(self, html: Box<dyn Html>) -> Self;

    /// Nest the specified container within this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_header(1, "Content Outside")
    ///     .add_container(
    ///         Container::new(ContainerType::Main)
    ///             .add_paragraph("Content Inside")
    ///     )
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     "<div><h1>Content Outside</h1><main><p>Content Inside</p></main></div>"
    /// );
    /// ```
    fn add_container(self, container: Container) -> Self {
        self.add_html(Box::new(container))
    }

    /// Nest the specified `Table` within this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_table(
    ///         Table::from(&[
    ///             [1, 2, 3],
    ///             [4, 5, 6]
    ///         ])
    ///         .add_header_row(&['A', 'B', 'C'])
    ///     )
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     concat!(
    ///         "<div><table><thead>",
    ///         "<tr><th>A</th><th>B</th><th>C</th></tr>",
    ///         "</thead><tbody>",
    ///         "<tr><td>1</td><td>2</td><td>3</td></tr>",
    ///         "<tr><td>4</td><td>5</td><td>6</td></tr>",
    ///         "</tbody></table></div>"
    ///     )
    /// );
    /// ```
    fn add_table(self, table: Table) -> Self {
        self.add_html(Box::new(table))
    }

    /// Adds a header tag with the designated level to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_header(1, "Header Text")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><h1>Header Text</h1></div>"#)
    /// ```
    fn add_header(self, level: u8, text: impl ToString) -> Self {
        let content = BodyContent::Header {
            level,
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a header tag with the designated level and attributes to this container.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_header_attr(1, "Header Text", std::iter::once(("id", "main-header")))
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><h1 id="main-header">Header Text</h1></div>"#)
    /// ```
    fn add_header_attr<A, S>(self, level: u8, text: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = BodyContent::Header {
            level,
            content: text.to_string(),
            attr: attr.into(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<img>` tag to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_image("myimage.png", "a test image")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><img src="myimage.png" alt="a test image"></div>"#)
    /// ```
    fn add_image(self, src: impl ToString, alt: impl ToString) -> Self {
        let content = BodyContent::Image {
            src: src.to_string(),
            alt: alt.to_string(),
            attr: Attributes::default(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<img>` tag with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// # use std::collections::BTreeMap;
    /// let mut attrs = BTreeMap::new();
    /// attrs.insert("id", "sample-image");
    /// let content = Container::default()
    ///     .add_image_attr("myimage.png", "a test image", attrs)
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     r#"<div><img src="myimage.png" alt="a test image" id="sample-image"></div>"#
    /// )
    /// ```
    fn add_image_attr<A, S>(self, src: impl ToString, alt: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = BodyContent::Image {
            src: src.to_string(),
            alt: alt.to_string(),
            attr: attr.into(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<a>` tag to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_link("https://rust-lang.org/", "Rust Homepage")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><a href="https://rust-lang.org/">Rust Homepage</a></div>"#)
    /// ```
    fn add_link(self, href: impl ToString, text: impl ToString) -> Self {
        let content = BodyContent::Link {
            href: href.to_string(),
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds an `<a>` tag with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_link_attr("https://rust-lang.org/", "Rust Homepage", [("class", "links")])
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     r#"<div><a href="https://rust-lang.org/" class="links">Rust Homepage</a></div>"#
    /// )
    /// ```
    fn add_link_attr<A, S>(self, href: impl ToString, text: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = BodyContent::Link {
            href: href.to_string(),
            content: text.to_string(),
            attr: attr.into(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<p>` tag element to this Container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_paragraph("This is sample paragraph text")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><p>This is sample paragraph text</p></div>"#)
    /// ```
    fn add_paragraph(self, text: impl ToString) -> Self {
        let content = BodyContent::Paragraph {
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<p>` tag element with the specified attributes to this Container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_paragraph_attr("This is sample paragraph text", [("class", "text")])
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><p class="text">This is sample paragraph text</p></div>"#)
    /// ```
    fn add_paragraph_attr<A, S>(self, text: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = BodyContent::Paragraph {
            content: text.to_string(),
            attr: attr.into(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<pre>` tag element to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_preformatted("This | is   preformatted => text")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><pre>This | is   preformatted => text</pre></div>"#)
    /// ```
    fn add_preformatted(self, text: impl ToString) -> Self {
        let content = BodyContent::Preformatted {
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.add_html(Box::new(content))
    }

    /// Adds a `<pre>` tag element with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_preformatted_attr("This | is   preformatted => text", [("id", "code")])
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><pre id="code">This | is   preformatted => text</pre></div>"#)
    /// ```
    fn add_preformatted_attr<A, S>(self, text: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = BodyContent::Preformatted {
            content: text.to_string(),
            attr: attr.into(),
        };
        self.add_html(Box::new(content))
    }

    /// Add raw content to this container. The content is pasted directly into the HTML
    ///
    /// This is intended to be used as an escape hatch for one-off insertions. If you want a more
    /// reusable escape hatch, consider writing your own type implementing the [`Html`] trait. You
    /// can then use [`add_html`](HtmlContainer::add_html) to insert boxed instances into the
    /// container. See the documentation for that method for examples.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .add_raw(r#"<video width="250"><source src="video.mp4" type="video/mp4"></video>"#)
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     r#"<div><video width="250"><source src="video.mp4" type="video/mp4"></video></div>"#
    /// );
    /// ```
    fn add_raw(self, content: impl ToString) -> Self {
        self.add_html(Box::new(BodyContent::Raw {
            content: content.to_string(),
        }))
    }
}
