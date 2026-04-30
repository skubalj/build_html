use std::fmt::{self, Display, Formatter};

macro_rules! def_tags {
    (
        $( #[$outer_attrs:meta] )*
        $vis:vis enum $name:ident {
            $(
                $( #[$inner_attrs:meta] )*
                $variant:ident = $str:literal
            ),* $(,)?
        }
    ) => {
        // enum definition
        $( #[$outer_attrs] )*
        $vis enum $name {
            $(
                $( #[$inner_attrs] )*
                $variant
            ),*
        }

        // original as_str impl
        impl $name {
            /// Get the tag code that this tag represents
            fn to_str(self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $str
                    ),*
                }
            }
        }

        // FromStr impl
        impl std::str::FromStr for $name {
            type Err = UnknownTagError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $str => Result::Ok(Self::$variant),
                    )*
                    x => Result::Err(UnknownTagError{got: x.into()})
                }
            }
        }

        /// Invalid HtmlTag parsed from string
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct UnknownTagError {
            /// Invalid Tag
            got: String,
        }

        impl std::error::Error for UnknownTagError {}

        impl std::fmt::Display for UnknownTagError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "unknown HTML element '{}'",
                    self.got,
                )
            }
        }

    };
}

def_tags! {
    /// A list of HTML tags
    ///
    /// This non-comprehensive list of tags is a subset of those listed in the MDN Web Docs
    /// [Html Elements Reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Element).
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[non_exhaustive]
    pub enum HtmlTag {
        /// A contact address
        Address = "address",
        /// A self-contained article
        Article = "article",
        /// Indicates side content to the main content
        Aside = "aside",
        /// Used to embed sound content in documents
        Audio = "audio",
        /// Bold text
        Bold = "b",
        /// Indicates a blockquote
        Blockquote = "blockquote",
        /// Button input element
        Button = "button",
        /// HTML canvas element
        Canvas = "canvas",
        /// Used to mark the title of a cited work
        Cite = "cite",
        /// A text block containing code
        ///
        /// Generally, this causes it to be rendered in a monospace font, and to preserve whitespace
        CodeText = "code",
        /// Deleted text
        Deleted = "del",
        /// Indicates a term to be defined
        ///
        /// The `<dfn>` element should be used in a complete definition statement, where the full
        /// definition of the term can be one of the following:
        /// * The ancestor paragraph (a block of text, sometimes marked by a <p> element)
        /// * The <dt>/<dd> pairing
        /// * The nearest section ancestor of the <dfn> element
        Definition = "dfn",
        /// The outer wrapper for a description list
        ///
        /// A `dl` generally consists of alternating [`dt`](HtmlTag::DescriptionListTerm) and
        /// [`dd`](HtmlTag::DescriptionListDescription) elements.
        DescriptionList = "dl",
        /// A description or definition for a term in a description list
        DescriptionListDescription = "dd",
        /// A term to be defined in a description list
        DescriptionListTerm = "dt",
        /// The almighty div -- a generic container with no predefined meaning
        Div = "div",
        /// Emphasized text
        Emphasized = "em",
        /// The caption for the contents of a figure
        Figcaption = "figcaption",
        /// A figure, such as an image
        Figure = "figure",
        /// A page footer
        Footer = "footer",
        /// A page header, or introductory content
        Header = "header",
        /// A top level heading
        Heading1 = "h1",
        /// A second-level heading
        Heading2 = "h2",
        /// A third-level heading
        Heading3 = "h3",
        /// A fourth-level heading
        Heading4 = "h4",
        /// A fifth-level heading
        Heading5 = "h5",
        /// A sixth (and lowest) level heading
        Heading6 = "h6",
        /// A wrapper to associate a heading with related content
        HeadingGroup = "hgroup",
        /// A horiztonal rule across the page
        HorizontalRule = "hr",
        /// A frame to embed one page within another
        Iframe = "iframe",
        /// An image element
        Image = "img",
        /// An inline quote
        InlineQuote = "q",
        /// Form controls
        Input = "input",
        /// Inserted text
        Inserted = "ins",
        /// Italic text
        Italic = "i",
        /// Represents a span of inline text denoting textual user input (eg:`<kbd>Ctrl</kbd>`)
        Keyboard = "kbd",
        /// A caption for an item in a user interface
        Label = "label",
        /// A manual line break
        LineBreak = "br",
        /// A link to another page or resource
        Link = "a",
        /// A list element, used within OrderedList and UnorderedList elements
        ListElement = "li",
        /// A container for the main content on a page
        Main = "main",
        /// Marked text
        Mark = "mark",
        /// A "meter bar" to display a value within a range
        Meter = "meter",
        /// A container for the navigation contenton a page
        Navigation = "nav",
        /// A placeholder for when scripting is unsupported by the browser
        NoScript = "noscript",
        /// An unordered, generally numbered, list
        OrderedList = "ol",
        /// Paragraph text
        ParagraphText = "p",
        /// Preformatted text, typically rendered in monospace
        PreformattedText = "pre",
        /// A generic section of the document
        Section = "section",
        /// Small text
        Small = "small",
        /// A subsection of text
        Span = "span",
        /// Important text
        Strong = "strong",
        /// Subscript text
        Subscript = "sub",
        /// Superscript text
        Superscript = "sup",
        /// A table element
        Table = "table",
        /// The table body
        TableBody = "tbody",
        /// A table caption
        TableCaption = "caption",
        /// A single data cell within a table row (`td`)
        TableCell = "td",
        /// A table column, generally found inside a [`TableColumnGroup`](HtmlTag::TableColumnGroup)
        TableColumn = "col",
        /// A group of table columns
        TableColumnGroup = "colgroup",
        /// The footer of a table
        TableFooter = "tfoot",
        /// The section of the table containing header rows
        TableHeader = "thead",
        /// A header cell within a table row (`th`)
        TableHeaderCell = "th",
        /// A table row
        TableRow = "tr",
        /// A text area
        TextArea = "textarea",
        /// An unordered, generally bulleted, list
        UnorderedList = "ul",
        /// An embedded video element
        Video = "video",
    }
}

impl Display for HtmlTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::str::FromStr};

    #[test]
    fn from_str_test() {
        assert_eq!(HtmlTag::from_str("div"), Ok(HtmlTag::Div));
        assert_eq!(HtmlTag::from_str("p"), Ok(HtmlTag::ParagraphText));
        assert_eq!(HtmlTag::from_str("ins"), Ok(HtmlTag::Inserted));
        assert_eq!(HtmlTag::from_str("figcaption"), Ok(HtmlTag::Figcaption));
        assert_eq!(HtmlTag::from_str("h4"), Ok(HtmlTag::Heading4));
        assert_eq!(
            HtmlTag::from_str("unknown"),
            Err(UnknownTagError {
                got: "unknown".into()
            })
        );
    }
}
