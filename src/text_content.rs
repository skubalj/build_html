//! This module contains structs used to add text content to an HTML page

use crate::Html;
use crate::HtmlPage;

/// Represents the HTML `<blockquote>` element
pub struct BlockQuote;

/// Represents the HTML `<blockquote>` element
pub struct Div;

/// Represents any of the HTML header elements, `<h1>` through `<h6>`
pub struct H {
    level: u8,
    text: String,
}

impl Html for H {
    fn to_html_string(&self) -> String {
        format!("<h{}>{}</h{}>", self.level, self.text, self.level)
    }
}

/// Represents the HTML `<blockquote>` element
pub struct Hr;

impl Html for Hr {
    fn to_html_string(&self) -> String {
        String::from("<hr>")
    }
}

/// Represents the HTML `<blockquote>` element
pub struct Li;

/// Represents the HTML `<blockquote>` element
pub struct Ol;

/// Represents the HTML `<blockquote>` element
pub struct P {
    text: String,
}

impl Html for P {
    fn to_html_string(&self) -> String {
        format!("<p>{}</p>", self.text)
    }
}

/// Represents the HTML `<blockquote>` element
pub struct Pre {
    code: String,
}

impl Html for Pre {
    fn to_html_string(&self) -> String {
        format!("<pre>{}</pre>", self.code)
    }
}

/// Represents the HTML `<blockquote>` element
pub struct Ul;

impl HtmlPage {
    /// Adds a header tag of the specified level to this HTML page
    pub fn add_h(mut self, level: u8, text: &str) -> Self {
        let h = H {level, text: text.into()};
        self.body.push(Box::new(h));
        self
    }

    /// Adds a `<p>` tag to the body of this HTML page
    pub fn add_p(mut self, text: &str) -> Self {
        let p = P { text: text.into() };
        self.body.push(Box::new(p));
        self
    }

    /// Adds a `<pre>` tag to the body of this HTML page
    pub fn add_pre(mut self, code: &str) -> Self {
        let pre = Pre {code: code.into() };
        self.body.push(Box::new(pre));
        self
    }
}
