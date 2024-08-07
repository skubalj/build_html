//! This library is designed to provide a simple way to generate HTML strings from within Rust
//! code. To generate HTML, this library uses the builder pattern; calls to add elements are
//! repeatedly chained together to build up an HTML document. The struct is then flushed to a
//! string which can be used elsewhere in your program.
//!
//! The simplest building block for this library is [`HtmlElement`]. This type allows creating
//! elements in a structured way and affords the greatest flexibility. However for simpler use
//! cases, the older [`HtmlContainer`] trait interface can be used, which provides a less verbose,
//! function-based way to build up HTML strings. Regardless of how your build your HTML, the
//! [`Html::to_html_string`] method allows you to render it to a `String`.
//!
//! The strings generated by this library are unformatted, but are not explicitly minimized.
//! Whitespace passed into a string will generally be preserved. Note that escaping strings is also
//! not automatic. You should use the [`escape_html`] function if you are displaying untrusted text.
//!
//! # Use Cases
//! The primary intention of this library is to provide an easy way to build dynamic elements that
//! can be injected into an HTML page or framework that is written in its own file. The advantage
//! to this is that it allows you to write the majority of your HTML with modern editor features
//! such as linting and syntax highlighting. You can use the standard library's `include_str!`
//! macro to "import" your html file and the `format!` macro to "inject" your new element.
//!
//! ```
//! use build_html::{HtmlElement, HtmlTag, Html};
//!
//! let element = HtmlElement::new(HtmlTag::Div)
//!     .with_child(
//!         HtmlElement::new(HtmlTag::ParagraphText)
//!             .with_child("Paragraph Text".into())
//!             .into()
//!     )
//!     .with_child(
//!         HtmlElement::new(HtmlTag::PreformattedText)
//!             .with_child("Preformatted Text".into())
//!             .into()
//!     )
//!     .to_html_string();
//!
//! assert_eq!(element, "<div><p>Paragraph Text</p><pre>Preformatted Text</pre></div>");
//!
//! ```
//!
//! However, if your page is very simple or the entire page is dynamic, you may want to create the
//! entire thing from within your Rust code. To meet this use case, the library provides the
//! [`HtmlPage`] struct. This struct implements the [`HtmlContainer`] interface, which can be used
//! to easily add body content.
//!
//! ```
//! use build_html::{HtmlPage, Html, HtmlContainer};
//!
//! let page = HtmlPage::new()
//!     .with_title("TITLE")
//!     .with_paragraph("PARAGRAPH")
//!     .to_html_string();
//!
//! assert_eq!(page, concat!(
//!     "<!DOCTYPE html><html>",
//!     "<head><title>TITLE</title></head>",
//!     "<body><p>PARAGRAPH</p></body>",
//!     "</html>"
//! ));
//! ```
//! # `add_` vs `with_`
//! Throughout this library, there are "pairs" of methods that use the `add_` and `with_` prefixes.
//! The `add_` methods take a mutable reference and act via side effects, while the `with_` methods
//! are self-consuming. While it makes the documentation a little noisy, this allows you to build
//! up relatively complicated logic without having to continually re-assign to the same variable or
//! create intermediate values:
//!
//! ```
//! # use build_html::{HtmlElement, HtmlTag, Html, HtmlContainer};
//! let mut root = HtmlElement::new(HtmlTag::Div)
//!     .with_child(HtmlElement::new(HtmlTag::Heading1).with_child("Counts".into()).into());
//!
//! for x in 1..=3 {
//!     // Here, we're adding by reference using an `add` method while also building
//!     // our inner element with a `with` method.
//!     root.add_child(HtmlElement::new(HtmlTag::Div).with_paragraph(x).into());
//! }
//!
//! assert_eq!(root.to_html_string(), concat!(
//!     "<div><h1>Counts</h1>",
//!     "<div><p>1</p></div>",
//!     "<div><p>2</p></div>",
//!     "<div><p>3</p></div>",
//!     "</div>"
//! ));
//! ```
//!
//! # Extensibility
//! In the event that you require additional tags or types not implemented in this library, you
//! can achieve this using one of the escape hatches.
//!
//! If you are using `HtmlElement` directly, you can use [`HtmlElement::add_child`] with the `Raw`
//! variant of `HtmlChild`. To make this even simpler, you can use the `into()` function to make
//! the conversion nearly seamless:
//!
//! ```
//! # use build_html::*;
//! let tag = HtmlElement::new(HtmlTag::Div).with_child("RAW TEXT".into()).to_html_string();
//! assert_eq!(tag, "<div>RAW TEXT</div>")
//! ```
//!
//! If you are using the `HtmlContainer` interface, you can make a type implementing the [`Html`]
//! interface and add it with [`HtmlContainer::add_html`] or add it directly as a string with
//! [`HtmlContainer::add_raw`]. (Note that `HtmlElement` implements `HtmlContainer`, so these
//! methods will work for that type too.)

mod attributes;
mod container;
mod elements;
mod html_container;
mod html_page;
mod table;
mod tags;

pub use self::container::{Container, ContainerType};
pub use self::elements::{HtmlChild, HtmlElement};
pub use self::html_container::HtmlContainer;
pub use self::html_page::{HtmlPage, HtmlVersion};
pub use self::table::{Table, TableCell, TableCellType, TableRow};
pub use self::tags::HtmlTag;

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
    /// let html = HtmlElement::new(HtmlTag::Div)
    ///     .with_paragraph("My p element")
    ///     .to_html_string();
    ///
    /// assert_eq!(html, "<div><p>My p element</p></div>");
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

impl Html for &str {
    fn to_html_string(&self) -> String {
        self.to_string()
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
/// let html = HtmlElement::new(HtmlTag::Div)
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
