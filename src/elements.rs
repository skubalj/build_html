//! Definitions for generic HTML tags

use crate::{Html, HtmlContainer};
use std::fmt::{self, Display, Formatter};

/// A list of HTML tags
///
/// This non-comprehensive list of tags is a subset of those listed in the MDN Web Docs
/// [Html Elements Reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Element).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HtmlTag {
    /// A contact address
    Address,
    /// A self-contained article
    Article,
    /// Indicates side content to the main content
    Aside,
    /// Indicates a blockquote
    Blockquote,
    /// HTML canvas element
    Canvas,
    /// A text block containing code
    ///
    /// Generally, this causes it to be rendered in a monospace font, and to preserve whitespace
    CodeText,
    /// The outer wrapper for a description list
    ///
    /// A `dl` generally consists of alternating [`dt`](HtmlTag::DescriptionListTerm) and
    /// [`dd`](HtmlTag::DescriptionListDescription) elements.
    DescriptionList,
    /// A description or definition for a term in a description list
    DescriptionListDescription,
    /// A term to be defined in a description list
    DescriptionListTerm,
    /// The almighty div -- a generic container with no predefined meaning
    Div,
    /// The caption for the contents of a figure
    Figcaption,
    /// A figure, such as an image
    Figure,
    /// A page footer
    Footer,
    /// A page header, or introductory content
    Header,
    /// A top level heading
    Heading1,
    /// A second-level heading
    Heading2,
    /// A third-level heading
    Heading3,
    /// A fourth-level heading
    Heading4,
    /// A fifth-level heading
    Heading5,
    /// A sixth (and lowest) level heading
    Heading6,
    /// A wrapper to associate a heading with related content
    HeadingGroup,
    /// A horiztonal rule across the page
    HorizontalRule,
    /// A frame to embed one page within another
    Iframe,
    /// An image element
    Image,
    /// A manual line break
    LineBreak,
    /// A link to another page or resource
    Link,
    /// A list element, used within OrderedList and UnorderedList elements
    ListElement,
    /// A container for the main content on a page
    Main,
    /// A container for the navigation contenton a page
    Navigation,
    /// An unordered list, generally a bulleted list
    OrderedList,
    /// Paragraph text
    ParagraphText,
    /// Preformatted text, typically rendered in monospace
    PreformattedText,
    /// A generic section of the document
    Section,
    /// A table element
    Table,
    /// The table body
    TableBody,
    /// A table caption
    TableCaption,
    /// A single data cell within a table row (`td`)
    TableCell,
    /// A table column, generally found inside a [`TableColumnGroup`](HtmlTag::TableColumnGroup)
    TableColumn,
    /// A group of table columns
    TableColumnGroup,
    /// The footer of a table
    TableFooter,
    /// The section of the table containing header rows
    TableHeader,
    /// A header cell within a table row (`th`)
    TableHeaderCell,
    /// A table row
    TableRow,
    /// An unordered list, generally numbered
    UnorderedList,
    /// An embedded video element
    Video,
}

impl Display for HtmlTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl HtmlTag {
    /// Get the tag code that this tag represents
    fn as_str(&self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::Article => "article",
            Self::Aside => "aside",
            Self::Blockquote => "blockquote",
            Self::Canvas => "canvas",
            Self::CodeText => "code",
            Self::DescriptionList => "dl",
            Self::DescriptionListDescription => "dd",
            Self::DescriptionListTerm => "dt",
            Self::Div => "div",
            Self::Figcaption => "figcaption",
            Self::Figure => "figure",
            Self::Footer => "footer",
            Self::Header => "header",
            Self::Heading1 => "h1",
            Self::Heading2 => "h2",
            Self::Heading3 => "h3",
            Self::Heading4 => "h4",
            Self::Heading5 => "h5",
            Self::Heading6 => "h6",
            Self::HeadingGroup => "hgroup",
            Self::HorizontalRule => "hr",
            Self::Iframe => "iframe",
            Self::Image => "img",
            Self::LineBreak => "br",
            Self::Link => "a",
            Self::ListElement => "li",
            Self::Main => "main",
            Self::Navigation => "nav",
            Self::OrderedList => "ol",
            Self::ParagraphText => "p",
            Self::PreformattedText => "pre",
            Self::Section => "section",
            Self::Table => "table",
            Self::TableBody => "tbody",
            Self::TableCaption => "caption",
            Self::TableCell => "td",
            Self::TableColumn => "col",
            Self::TableColumnGroup => "colgroup",
            Self::TableFooter => "tfoot",
            Self::TableHeader => "thead",
            Self::TableHeaderCell => "th",
            Self::TableRow => "tr",
            Self::UnorderedList => "ul",
            Self::Video => "video",
        }
    }
}

/// A child of an [`HtmlElement`]: either another element, or some raw text
///
/// Generally, `HtmlContent` shouldn't need to be used directly. You can use `.into()` to convert
/// strings and [`HtmlElement`]s into this type seamlessly.
#[derive(Debug, Clone)]
pub enum HtmlChild {
    /// An element that can have more children of its own
    Element(HtmlElement),
    /// A raw string that will be appended into the output HTML
    Raw(String),
}

impl Display for HtmlChild {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Element(e) => write!(f, "{e}"),
            Self::Raw(r) => write!(f, "{r}"),
        }
    }
}

impl Html for HtmlChild {
    fn to_html_string(&self) -> String {
        match self {
            Self::Element(e) => e.to_html_string(),
            Self::Raw(r) => r.to_owned(),
        }
    }
}

impl From<HtmlElement> for HtmlChild {
    fn from(value: HtmlElement) -> Self {
        Self::Element(value)
    }
}

impl<S: AsRef<str>> From<S> for HtmlChild {
    fn from(value: S) -> Self {
        Self::Raw(value.as_ref().to_owned())
    }
}

/// A structured HTML element, with a tag, attributes, and children. This allows much greater
/// flexibility than the traditional [`HtmlContainer`] interface.
///
/// ```
/// # use build_html::*;
/// let output = HtmlElement::new(HtmlTag::Div)
///     .with_child(
///         HtmlElement::new(HtmlTag::Heading1)
///             .with_attribute("class", "big-text")
///             .with_child("Header Text".into())
///             .into(),
///     )
///     .with_child(
///         HtmlElement::new(HtmlTag::ParagraphText)
///             .with_child("Paragraph Text".into())
///             .with_child(HtmlElement::new(HtmlTag::LineBreak).into())
///             .with_child("Paragraph Text Line 2".into())
///             .into(),
///     )
///     .to_html_string();
///
/// assert_eq!(output, r#"<div><h1 class="big-text">Header Text</h1><p>Paragraph Text<br/>Paragraph Text Line 2</p></div>"#);
/// ```
#[derive(Debug, Clone)]
pub struct HtmlElement {
    /// The tag to be used for this element
    pub tag: HtmlTag,
    /// A list of the attributes that will be printed in this element in the form `(key, value)`
    pub attributes: Vec<(String, String)>,
    /// A list of the child elements contained within this element
    pub children: Vec<HtmlChild>,
}

impl Display for HtmlElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.children.is_empty() {
            write!(f, "<{}", self.tag)?;
            self.write_attributes(f)?;
            write!(f, "/>")
        } else {
            write!(f, "<{}", self.tag,)?;
            self.write_attributes(f)?;
            write!(f, ">")?;
            self.write_children(f)?;
            write!(f, "</{}>", self.tag)
        }
    }
}

impl Html for HtmlElement {
    fn to_html_string(&self) -> String {
        format!("{}", self)
    }
}

/// This implementation of HtmlContainer allows seamless for compatibility between the "easy"
/// interface and this more complete one
impl HtmlContainer for HtmlElement {
    fn add_html<H: Html>(&mut self, html: H) {
        self.children.push(HtmlChild::Raw(html.to_html_string()))
    }
}

impl HtmlElement {
    /// Create a new empty HTML element with the given tag
    ///
    /// ```
    /// # use build_html::*;
    /// assert_eq!(HtmlElement::new(HtmlTag::Div).to_html_string(), "<div/>");
    /// ```
    pub fn new(tag: HtmlTag) -> Self {
        Self {
            tag,
            attributes: Default::default(),
            children: Default::default(),
        }
    }

    /// Add a new child to this element
    ///
    /// A child can be either a raw string ([`HtmlChild::Raw`]) or another element
    /// ([`HtmlChild::Element`]). You can use the `into` function to append `&str`s and
    /// `HtmlElement`s directly.
    ///
    /// ```
    /// # use build_html::*;
    /// let mut element = HtmlElement::new(HtmlTag::ParagraphText);
    /// element.add_child("First Line".into());
    /// element.add_child(HtmlElement::new(HtmlTag::LineBreak).into());
    /// element.add_child("Second Line".into());
    /// assert_eq!(element.to_html_string(), "<p>First Line<br/>Second Line</p>");
    /// ```
    pub fn add_child(&mut self, content: HtmlChild) {
        self.children.push(content);
    }

    /// Consume this element and return it with the new child appended
    ///
    /// A child can be either a raw string ([`HtmlChild::Raw`]) or another element
    /// ([`HtmlChild::Element`]). You can use the `into` function to append `&str`s and
    /// `HtmlElement`s directly.
    ///
    /// ```
    /// # use build_html::*;
    /// let output = HtmlElement::new(HtmlTag::ParagraphText)
    ///     .with_child("First Line".into())
    ///     .with_child(HtmlElement::new(HtmlTag::LineBreak).into())
    ///     .with_child("Second Line".into())
    ///     .to_html_string();
    /// assert_eq!(output, "<p>First Line<br/>Second Line</p>");
    /// ```
    pub fn with_child(mut self, content: HtmlChild) -> Self {
        self.add_child(content);
        self
    }

    /// Add an attribute to this element
    ///
    /// This attribute will simply be appended to the others that have been specified. If the same
    /// attribute is specified twice, it will be duplicated, which may result in strange behavior.
    ///
    /// ```
    /// # use build_html::*;
    /// let mut element = HtmlElement::new(HtmlTag::Div);
    /// element.add_attribute("class", "container");
    /// assert_eq!(element.to_html_string(), r#"<div class="container"/>"#);
    /// ```
    pub fn add_attribute(&mut self, k: impl ToString, v: impl ToString) {
        self.attributes.push((k.to_string(), v.to_string()));
    }

    /// Consume this element and return it with the given attribute set.
    ///
    /// This attribute will simply be appended to the others that have been specified. If the same
    /// attribute is specified twice, it will be duplicated, which may result in strange behavior.
    ///
    /// ```
    /// # use build_html::*;
    /// let output = HtmlElement::new(HtmlTag::Div)
    ///     .with_attribute("class", "container")
    ///     .with_attribute("id", "first-div")
    ///     .to_html_string();
    /// assert_eq!(output, r#"<div class="container" id="first-div"/>"#);
    /// ```
    pub fn with_attribute(mut self, k: impl ToString, v: impl ToString) -> Self {
        self.add_attribute(k, v);
        self
    }

    fn write_attributes(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (k, v) in self.attributes.iter() {
            write!(f, r#" {}="{}""#, k, v)?;
        }
        Ok(())
    }

    fn write_children(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for child in self.children.iter() {
            write!(f, "{}", child)?;
        }
        Ok(())
    }
}
