//! This library is designed to provide a simple way to generate HTML documents from within Rust
//! code. To generate documents, this library uses the builder pattern; calls to add elements are
//! repeatedly chained together to dynamically build up an HTML document. The document is then
//! flushed to an HTML string which can be used elsewhere in your program with `to_html_string()`
//!
//! # Using
//! Everything you need to use this crate has been exported from the crate root. This means that
//! you can get easy access to every element using the import: `use html_gen::*`.
//!
//! If compatibility is important, or you don't need access to every element, you can also use the
//! import `use html_gen;` and prefix types with the package name. Note that the traits `HTML` and 
//! `HtmlContainer` must be in scope for the library to be useful:
//! ```
//! use html_gen::{self, Html, HtmlContainer};
//!
//! let page = html_gen::HtmlPage::new()
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
//! use html_gen::*;
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

mod attributes;
mod container;
mod content;
mod html_container;
mod html_page;
mod table;

// Exports for the `use html_gen::*` syntax
pub use self::container::{Container, ContainerType};
pub use self::html_container::HtmlContainer;
pub use self::html_page::HtmlPage;
pub use self::table::Table;

/// An element that can be converted to an HTML string
///
/// This trait is the centerpiece of the entire library: after building up an
/// HTML structure, usually an [`HtmlPage`], [`to_html_string()`](crate::Html::to_html_string)
/// is used to flush the structure to a string. This works by iterating through all
/// the elements inside each Html element, and converting each
pub trait Html: std::fmt::Debug {
    /// Convert this element into an HTML string
    ///
    /// This is the method that ultimately flushes each HTML object to a string.
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
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
