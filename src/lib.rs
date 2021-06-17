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
//!     .add_paragraph("Some Text")
//!     .to_html_string();
//! ```
//!
//! Once the package is imported, the [`HtmlPage`] struct will provide the base for most use cases.
//! This struct provides the boilerplate for an HTML5 page, and allows content to be added to it
//! using chained methods.
//!
//! # Example
//! This example uses the [`maplit`](https://crates.io/crates/maplit) crate to create `HashMap`
//! literals. If your use case requires adding additional attributes, I would recommend this crate
//! for its simplicity.
//!
//! ```rust
//! use build_html::*;
//! use maplit::hashmap;
//!
//! let html: String = HtmlPage::new()
//!     .add_title("My Page")
//!     .add_header(1, "Main Content:")
//!     .add_container(
//!         Container::new(ContainerType::Article)
//!             .with_attributes(hashmap! {"id" => "article1"})
//!             .add_header_attr(2, "Hello, World", hashmap! {"id" => "article-head"})
//!             .add_paragraph("This is a simple HTML demo")
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
    ///     .add_paragraph("My p element")
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
