use std::fmt::{self, Display};

mod metadata;
mod text_content;

pub trait Html {
    /// Convert this element into an HTML string
    fn to_html_string(&self) -> String;
}

/// This struct represents an entire page of HTML which can built up by chaining addition methods.
/// This creates an effect similar to the [Decorator Pattern](https://en.wikipedia.org/wiki/Decorator_pattern)
///
/// To convert an `HtmlPage` to a [`String`] which can be sent back to a client, use the
/// [`Html::to_html_string()`] method
pub struct HtmlPage {
    head: Vec<Box<dyn Html>>,
    body: Vec<Box<dyn Html>>,
}

impl Html for HtmlPage {
    fn to_html_string(&self) -> String {
        let head = self
            .head
            .iter()
            .map(|element| element.to_html_string())
            .fold(String::new(), |acc, next| acc + &next);
        let body = self
            .body
            .iter()
            .map(|element| element.to_html_string())
            .fold(String::new(), |acc, next| acc + &next);

        format!(
            r#"
        <!DOCTYPE html>
        <html>
        <head>{}</head>
        <body>{}</body>
        </html>
        "#,
            head, body
        )
    }
}

impl Default for HtmlPage {
    fn default() -> Self {
        HtmlPage::new()
            .add_title("Default Page")
            .add_h(1, "Hello World")
    }
}

impl Display for HtmlPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_html_string())
    }
}

impl HtmlPage {
    /// Creates a new HTML page with no content
    pub fn new() -> Self {
        HtmlPage {
            head: Vec::new(),
            body: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
