//! This library is designed to provide a simple way to generate HTML documents from within Rust
//! code. To generate documents, this library uses the builder pattern; calls to add elements are
//! repeatedly chained together to dynamically build up an HTML document. The document is then
//! flushed to an HTML string which can be used elsewhere in your program with `to_html_string()`
//!
//! # Using
//! Everything you need to use this crate has been exported from the crate root. This means that
//! you can get easy access to every element using the import: `use build_html::*`.
//!
//! If compatibility is important, or you don't need access to every element, you can also import
//! elements piecemeal and prefix types with the package name. Note that the traits `HTML` and
//! `HtmlContainer` must be in scope for the library to be useful:
//! ```
//! use build_html::{self, Html, HtmlContainer};
//!
//! let page = build_html::HtmlPage::new()
//!     .with_paragraph("Some Text")
//!     .to_html_string();
//! ```
//!
//! Once the package is imported, the [`HtmlPage`] struct will provide the base for most use cases.
//! This struct provides the boilerplate for an HTML5 page, and allows content to be added to it
//! using chained methods.
//!
//! # Example
//!
//! ```rust
//! use build_html::*;
//!
//! let html: String = HtmlPage::new()
//!     .with_title("My Page")
//!     .with_header(1, "Main Content:")
//!     .with_container(
//!         Container::new(ContainerType::Article)
//!             .with_attributes([("id", "article1")])
//!             .with_header_attr(2, "Hello, World", [("id", "article-head")])
//!             .with_paragraph("This is a simple HTML demo")
//!     )
//!     .to_html_string();
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
//!         <article id="article1">
//!             <h2 id="article-head">Hello World</h2>
//!             <p>This is a simple HTML demo</p>
//!         </article>
//!     </body>
//! </html>
//! ```
//!
//! # Extensibility
//! The majority of the `add_x` methods specified in [`HtmlContainer`] are defined over generic
//! bounds. This means that they are quite flexibile, and you can pass in almost anything
//! implementing the [`ToString`] trait.
//!
//! In the event that you require additional tags or types not implemented in this library, you
//! can achieve this using one of two escape hatches. For a more structured approach, consider
//! seeing the documentation for [`HtmlContainer::add_html`]. For more one-off situations, consider
//! [`HtmlContainer::add_raw`].

mod attributes;
mod container;
mod content;
mod html_container;
mod html_page;
mod table;

// Exports for the `use build_html::*` syntax
pub use self::container::{Container, ContainerType};
pub use self::html_container::HtmlContainer;
pub use self::html_page::HtmlPage;
pub use self::table::Table;

/// An element that can be converted to an HTML string
///
/// This trait is the centerpiece of the entire library: after building up an
/// HTML structure, usually an [`HtmlPage`], [`to_html_string()`](crate::Html::to_html_string)
/// is used to flush the structure to a string.
pub trait Html: std::fmt::Debug {
    /// Convert this element into an HTML string
    ///
    /// This is the method that ultimately flushes each HTML object to a string.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let html = Container::default()
    ///     .with_paragraph("My p element")
    ///     .to_html_string();
    ///
    /// assert_eq!(html, "<div><p>My p element</p></div>")
    /// ```
    fn to_html_string(&self) -> String;
}

impl std::fmt::Display for dyn Html {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_html_string())
    }
}

impl Html for String {
    fn to_html_string(&self) -> String {
        self.clone()
    }
}

/// Escape the provided string.
///
/// All HTML tags will be converted to their escaped versions. The output string should be safe to
/// insert into an HTML document. Any embedded HTML tags will be rendered as text. It is important
/// to *always* escape inputs from untrusted sources!
///
/// Implementation note: The list of escaped characters is pulled from [Svelte](https://github.com/sveltejs/svelte/blob/master/src/compiler/compile/utils/stringify.ts#L14).
///
/// # Example
/// ```
/// # use build_html::*;
/// let html = Container::default()
///     .with_paragraph(escape_html("My <p> element!"))
///     .to_html_string();
///
/// assert_eq!(html, "<div><p>My &lt;p&gt; element!</p></div>");
///
/// ```
pub fn escape_html(data: &str) -> String {
    let mut escaped = String::with_capacity(data.len());
    for c in data.chars() {
        match c {
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            x => escaped.push(x),
        }
    }

    escaped
}
