//! This library is designed to provide a simple way to generate HTML documents dynamically from
//! within Rust code. To generate documents, this library uses the builder pattern,
//!
//! # Example
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

// Exports for the `use html_gen::*` syntax
pub use self::container::{Container, ContainerType};
pub use self::html_container::HtmlContainer;
pub use self::html_page::HtmlPage;

/// An element that can be converted to HTML
///
/// This trait is the centerpiece of the entire library.
pub trait Html: std::fmt::Debug {
    /// Convert this element into an HTML string
    fn to_html_string(&self) -> String;
}
