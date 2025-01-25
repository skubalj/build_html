use std::fmt::{self, Display, Formatter};

/// A list of HTML tags
///
/// This non-comprehensive list of tags is a subset of those listed in the MDN Web Docs
/// [Html Elements Reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Element).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
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
    /// Used to mark the title of a cited work
    Cite,
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
    /// An inline quote
    InlineQuote,
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
    /// An unordered, generally numbered, list
    OrderedList,
    /// Paragraph text
    ParagraphText,
    /// Preformatted text, typically rendered in monospace
    PreformattedText,
    /// A generic section of the document
    Section,
    /// A subsection of text
    Span,
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
    /// An unordered, generally bulleted, list
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
            Self::Cite => "cite",
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
            Self::InlineQuote => "q",
            Self::LineBreak => "br",
            Self::Link => "a",
            Self::ListElement => "li",
            Self::Main => "main",
            Self::Navigation => "nav",
            Self::OrderedList => "ol",
            Self::ParagraphText => "p",
            Self::PreformattedText => "pre",
            Self::Section => "section",
            Self::Span => "span",
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
