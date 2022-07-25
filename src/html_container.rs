//! Defines the `HtmlContainer` Trait

use crate::attributes::Attributes;
use crate::content;
use crate::Container;
use crate::Html;
use crate::Table;

/// An HTML element that can contain other HTML elements
///
/// The methods on this trait are implemented generically, allowing any type (or combination of
/// types) implementing [`ToString`] to be passed in. Thanks to monomorphization, this can happen
/// without incurring any runtime cost. For example, we can pass an `Ipv4Addr` object into
/// `with_paragraph` directly:
///
/// ```
/// # use build_html::*;
/// # use std::net::Ipv4Addr;
/// let content = Container::default()
///     .with_paragraph(Ipv4Addr::new(127, 0, 0, 1))
///     .to_html_string();
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
///     .with_paragraph_attr("123", [("id", "paragraph"), ("class", "action")])
///     .to_html_string();
/// assert_eq!(content, r#"<div><p id="paragraph" class="action">123</p></div>"#)
/// ```
///
/// There are two different ways of interacting with `HtmlContainer` objects which will suit
/// different use cases. The first is using the *with* API, which consumes the calling container.
/// Because the calling container is consumed and returned, it can be chained effectively. This
/// makes it very useful for building containers whose content is known ahead of time, and for
/// building content using iterators. For example:
///
/// ```
/// # use build_html::*;
/// // Aggregating content
/// let list_items = (0..3)
///     .map(|x| format!("List item {}", x))
///     .fold(Container::new(ContainerType::OrderedList), |a, n| a.with_paragraph(n));
///
/// // Defining content literally
/// let content = Container::default()
///     .with_paragraph("paragraph text")
///     .with_container(list_items)
///     .to_html_string();
///
/// assert_eq!(
///     content,
///     concat!(
///         "<div><p>paragraph text</p><ol>",
///         "<li><p>List item 0</p></li>",
///         "<li><p>List item 1</p></li>",
///         "<li><p>List item 2</p></li></ol></div>"
///     )
/// )
/// ```
///
/// The other is using the *add* API, which acts on mutable references. This method is effective for
/// more imperative programming and more delicate control flow.
///
/// ```
/// # use build_html::*;
/// let mut container = Container::default();
/// if true {
///     container.add_paragraph("optional content");
/// }
/// for i in 0..3 {
///     container.add_paragraph(format!("Item: {}", i));
/// }
/// assert_eq!(
///     container.to_html_string(),
///     concat!(
///         "<div><p>optional content</p>",
///         "<p>Item: 0</p><p>Item: 1</p>",
///         "<p>Item: 2</p></div>"
///     )
/// );
/// ```
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
    /// let mut content = Container::default();
    /// content.add_html(Span::new("inner"));
    /// assert_eq!(content.to_html_string(), "<div><span>inner</span></div>");
    /// ```
    fn add_html<H: Html>(&mut self, html: H);

    /// Consumes the container, returning it with the specified HTML element added to it
    ///
    /// This method can be used as an escape hatch to insert arbitrary types into the HTML document,
    /// helping to make up for those types which are not supported natively by this library. This
    /// can be done by defining your own types that implement the [`Html`] trait.
    ///
    /// If you need a simple one-off, it may be more convenient to insert the element as a raw
    /// string using [`with_raw`](HtmlContainer::with_raw) method instead
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
    ///     .with_html(Span::new("inner"))
    ///     .to_html_string();
    /// assert_eq!(content, "<div><span>inner</span></div>");
    /// ```
    #[inline]
    fn with_html<H: Html>(mut self, html: H) -> Self {
        self.add_html(html);
        self
    }

    /// Add the container to this HTML Container
    ///
    /// Under the covers, this is simply an alias for [`add_html`](HtmlContainer::add_html).
    /// Upon hearing this, you might be asking yourself "Why is this useful?". The answer is simply
    /// that this function should be preferred because it is more descriptive. `add_html` is
    /// intended to be more of an escape hatch than something used in everyday programs.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_container(Container::new(ContainerType::Main).with_paragraph("Inside"));
    /// assert_eq!(content.to_html_string(), "<div><main><p>Inside</p></main></div>");
    /// ```
    #[inline]
    fn add_container(&mut self, container: Container) {
        self.add_html(container)
    }

    /// Nest the specified container within this container
    ///
    /// Under the covers, this is simply an alias for [`with_html`](HtmlContainer::with_html).
    /// Upon hearing this, you might be asking yourself "Why is this useful?". The answer is simply
    /// that this function should be preferred because it is more descriptive. `with_html` is
    /// intended to be more of an escape hatch than something used in everyday programs.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_header(1, "Content Outside")
    ///     .with_container(
    ///         Container::new(ContainerType::Main)
    ///             .with_paragraph("Content Inside")
    ///     )
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     "<div><h1>Content Outside</h1><main><p>Content Inside</p></main></div>"
    /// );
    /// ```
    #[inline]
    fn with_container(self, container: Container) -> Self {
        self.with_html(container)
    }

    /// Add the specified `Table` to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6]
    /// ]).with_header_row(['A', 'B', 'C']);
    /// let mut container = Container::default();
    /// container.add_table(table);
    ///
    /// assert_eq!(
    ///     container.to_html_string(),
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
    fn add_table(&mut self, table: Table) {
        self.add_html(table);
    }

    /// Nest the specified `Table` within this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_table(
    ///         Table::from(&[
    ///             [1, 2, 3],
    ///             [4, 5, 6]
    ///         ])
    ///         .with_header_row(&['A', 'B', 'C'])
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
    fn with_table(self, table: Table) -> Self {
        self.with_html(table)
    }

    /// Adds a header tag with the designated level to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_header(1, "Header Text");
    /// assert_eq!(content.to_html_string(), r#"<div><h1>Header Text</h1></div>"#);
    /// ```
    fn add_header(&mut self, level: u8, text: impl ToString) {
        let content = content::Header {
            level,
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.add_html(content);
    }

    /// Adds a header tag with the designated level to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_header(1, "Header Text")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><h1>Header Text</h1></div>"#);
    /// ```
    fn with_header(self, level: u8, text: impl ToString) -> Self {
        let content = content::Header {
            level,
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.with_html(content)
    }

    /// Adds a header tag with the designated level and attributes to this container.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_header_attr(1, "Header Text", std::iter::once(("id", "main-header")));
    /// assert_eq!(content.to_html_string(), r#"<div><h1 id="main-header">Header Text</h1></div>"#);
    /// ```
    fn add_header_attr<A, S>(&mut self, level: u8, text: impl ToString, attr: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = content::Header {
            level,
            content: text.to_string(),
            attr: attr.into(),
        };
        self.add_html(content);
    }

    /// Adds a header tag with the designated level and attributes to this container.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_header_attr(1, "Header Text", std::iter::once(("id", "main-header")))
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><h1 id="main-header">Header Text</h1></div>"#);
    /// ```
    fn with_header_attr<A, S>(self, level: u8, text: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = content::Header {
            level,
            content: text.to_string(),
            attr: attr.into(),
        };
        self.with_html(content)
    }

    /// Adds an `<img>` tag to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_image("myimage.png", "a test image");
    /// assert_eq!(
    ///     content.to_html_string(),
    ///     r#"<div><img src="myimage.png" alt="a test image"></div>"#
    /// );
    /// ```
    fn add_image(&mut self, src: impl ToString, alt: impl ToString) {
        let content = content::Image {
            src: src.to_string(),
            alt: alt.to_string(),
            attr: Attributes::default(),
        };
        self.add_html(content);
    }

    /// Adds an `<img>` tag to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_image("myimage.png", "a test image")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><img src="myimage.png" alt="a test image"></div>"#);
    /// ```
    fn with_image(self, src: impl ToString, alt: impl ToString) -> Self {
        let content = content::Image {
            src: src.to_string(),
            alt: alt.to_string(),
            attr: Attributes::default(),
        };
        self.with_html(content)
    }

    /// Adds an `<img>` tag with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// # use std::collections::BTreeMap;
    /// let mut attrs = BTreeMap::new();
    /// attrs.insert("id", "sample-image");
    /// let mut content = Container::default();
    /// content.add_image_attr("myimage.png", "a test image", attrs);
    ///
    /// assert_eq!(
    ///     content.to_html_string(),
    ///     r#"<div><img src="myimage.png" alt="a test image" id="sample-image"></div>"#
    /// );
    /// ```
    fn add_image_attr<A, S>(&mut self, src: impl ToString, alt: impl ToString, attr: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = content::Image {
            src: src.to_string(),
            alt: alt.to_string(),
            attr: attr.into(),
        };
        self.add_html(content);
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
    ///     .with_image_attr("myimage.png", "a test image", attrs)
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     r#"<div><img src="myimage.png" alt="a test image" id="sample-image"></div>"#
    /// );
    /// ```
    fn with_image_attr<A, S>(self, src: impl ToString, alt: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = content::Image {
            src: src.to_string(),
            alt: alt.to_string(),
            attr: attr.into(),
        };
        self.with_html(content)
    }

    /// Adds an `<a>` tag to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_link("https://rust-lang.org/", "Rust Homepage");
    ///
    /// assert_eq!(
    ///     content.to_html_string(),
    ///     r#"<div><a href="https://rust-lang.org/">Rust Homepage</a></div>"#
    /// );
    /// ```
    fn add_link(&mut self, href: impl ToString, text: impl ToString) {
        let content = content::Link {
            href: href.to_string(),
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.add_html(content)
    }

    /// Adds an `<a>` tag to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_link("https://rust-lang.org/", "Rust Homepage")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><a href="https://rust-lang.org/">Rust Homepage</a></div>"#)
    /// ```
    fn with_link(self, href: impl ToString, text: impl ToString) -> Self {
        let content = content::Link {
            href: href.to_string(),
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.with_html(content)
    }

    /// Adds an `<a>` tag with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_link_attr("https://rust-lang.org/", "Rust Homepage", [("class", "links")]);
    ///
    /// assert_eq!(
    ///     content.to_html_string(),
    ///     r#"<div><a href="https://rust-lang.org/" class="links">Rust Homepage</a></div>"#
    /// );
    /// ```
    fn add_link_attr<A, S>(&mut self, href: impl ToString, text: impl ToString, attr: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = content::Link {
            href: href.to_string(),
            content: text.to_string(),
            attr: attr.into(),
        };
        self.add_html(content);
    }

    /// Adds an `<a>` tag with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_link_attr("https://rust-lang.org/", "Rust Homepage", [("class", "links")])
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     r#"<div><a href="https://rust-lang.org/" class="links">Rust Homepage</a></div>"#
    /// )
    /// ```
    fn with_link_attr<A, S>(self, href: impl ToString, text: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = content::Link {
            href: href.to_string(),
            content: text.to_string(),
            attr: attr.into(),
        };
        self.with_html(content)
    }

    /// Adds a `<p>` tag element to this Container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_paragraph("This is sample paragraph text");
    /// assert_eq!(content.to_html_string(), r#"<div><p>This is sample paragraph text</p></div>"#);
    /// ```
    fn add_paragraph(&mut self, text: impl ToString) {
        let content = content::Paragraph {
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.add_html(content)
    }

    /// Adds a `<p>` tag element to this Container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_paragraph("This is sample paragraph text")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><p>This is sample paragraph text</p></div>"#);
    /// ```
    fn with_paragraph(self, text: impl ToString) -> Self {
        let content = content::Paragraph {
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.with_html(content)
    }

    /// Adds a `<p>` tag element with the specified attributes to this Container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_paragraph_attr("This is sample paragraph text", [("class", "text")]);
    /// assert_eq!(
    ///     content.to_html_string(),
    ///     r#"<div><p class="text">This is sample paragraph text</p></div>"#
    /// );
    /// ```
    fn add_paragraph_attr<A, S>(&mut self, text: impl ToString, attr: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = content::Paragraph {
            content: text.to_string(),
            attr: attr.into(),
        };
        self.add_html(content);
    }

    /// Adds a `<p>` tag element with the specified attributes to this Container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_paragraph_attr("This is sample paragraph text", [("class", "text")])
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><p class="text">This is sample paragraph text</p></div>"#)
    /// ```
    fn with_paragraph_attr<A, S>(self, text: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = content::Paragraph {
            content: text.to_string(),
            attr: attr.into(),
        };
        self.with_html(content)
    }

    /// Adds a `<pre>` tag element to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_preformatted("This | is   preformatted => text");
    /// assert_eq!(
    ///     content.to_html_string(),
    ///     r#"<div><pre>This | is   preformatted => text</pre></div>"#
    /// );
    /// ```
    fn add_preformatted(&mut self, text: impl ToString) {
        let content = content::Preformatted {
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.add_html(content);
    }

    /// Adds a `<pre>` tag element to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_preformatted("This | is   preformatted => text")
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><pre>This | is   preformatted => text</pre></div>"#);
    /// ```
    fn with_preformatted(self, text: impl ToString) -> Self {
        let content = content::Preformatted {
            content: text.to_string(),
            attr: Attributes::default(),
        };
        self.with_html(content)
    }

    /// Adds a `<pre>` tag element with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_preformatted_attr("This | is   preformatted => text", [("id", "code")]);
    /// assert_eq!(
    ///     content.to_html_string(),
    ///     r#"<div><pre id="code">This | is   preformatted => text</pre></div>"#
    /// );
    /// ```
    fn add_preformatted_attr<A, S>(&mut self, text: impl ToString, attr: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = content::Preformatted {
            content: text.to_string(),
            attr: attr.into(),
        };
        self.add_html(content);
    }

    /// Adds a `<pre>` tag element with the specified attributes to this container
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_preformatted_attr("This | is   preformatted => text", [("id", "code")])
    ///     .to_html_string();
    ///
    /// assert_eq!(content, r#"<div><pre id="code">This | is   preformatted => text</pre></div>"#)
    /// ```
    fn with_preformatted_attr<A, S>(self, text: impl ToString, attr: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let content = content::Preformatted {
            content: text.to_string(),
            attr: attr.into(),
        };
        self.with_html(content)
    }

    /// Add raw content to the container. This content is pasted directly into the HTML
    ///
    /// This is intended to be used as an escape hatch for one-off insertions. If you want a more
    /// reusable escape hatch, consider writing your own type implementing the [`Html`] trait. You
    /// can then use [`add_html`](HtmlContainer::add_html) to insert boxed instances into the
    /// container. See the documentation for that method for examples.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut content = Container::default();
    /// content.add_raw(r#"<video width="250"><source src="video.mp4" type="video/mp4"></video>"#);
    /// assert_eq!(
    ///     content.to_html_string(),
    ///     r#"<div><video width="250"><source src="video.mp4" type="video/mp4"></video></div>"#
    /// );
    /// ```
    fn add_raw(&mut self, content: impl ToString) {
        self.add_html(content.to_string());
    }

    /// Add raw content to this container. The content is pasted directly into the HTML
    ///
    /// This is intended to be used as an escape hatch for one-off insertions. If you want a more
    /// reusable escape hatch, consider writing your own type implementing the [`Html`] trait. You
    /// can then use [`with_html`](HtmlContainer::with_html) to insert boxed instances into the
    /// container. See the documentation for that method for examples.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let content = Container::default()
    ///     .with_raw(r#"<video width="250"><source src="video.mp4" type="video/mp4"></video>"#)
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     content,
    ///     r#"<div><video width="250"><source src="video.mp4" type="video/mp4"></video></div>"#
    /// );
    /// ```
    fn with_raw(self, content: impl ToString) -> Self {
        self.with_html(content.to_string())
    }
}
