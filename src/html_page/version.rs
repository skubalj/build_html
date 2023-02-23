//! This module contains definitions of the various HTML versions

use crate::attributes::Attributes;

/// Versions of the HTML (or XHTML) standard
///
/// These can be used to change the doctype and apply attributes to an [`HtmlPage`](crate::HtmlPage).
///
/// # Example
/// ```
/// # use build_html::{Html, HtmlPage, HtmlVersion};
/// assert_eq!(
///     HtmlPage::with_version(HtmlVersion::HTML5).to_html_string(),
///     "<!DOCTYPE html><html><head></head><body></body></html>"
/// );
///
/// assert_eq!(
///     HtmlPage::with_version(HtmlVersion::XHTML1_0).to_html_string(),
///     concat!(
///         r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "#,
///         r#""http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#,
///         r#"<html xmlns="http://www.w3.org/1999/xhtml"><head></head><body></body></html>"#,
///     ),
/// )
/// ```
///
/// # Compliance With HTML Standards
/// Please note that while we allow users to specify the version of the HTML standard their page
/// is written in, this library *does not* and *will not* check whether your page is actually valid
/// in that standard. Our feature set is targeting development in HTML5 and it is possible that
/// some tags or attributes may not be valid in older HTML versions. You are responsible for
/// knowing which subset of the provided features are valid for your chosen version. Use this
/// feature at your own risk.
#[derive(Debug, Default)]
#[non_exhaustive]
pub enum HtmlVersion {
    /// HTML 5. The current and preferred version of the HTML standard.
    #[default]
    HTML5,
    /// Legacy HTML 4.01. Potentially useful for supporting old browsers.
    HTML4,
    /// Legacy XHTML 1.0. This is still common in HTML emails for backwards
    /// compatibility with different email clients.
    XHTML1_0,
    /// Legacy XHTML 1.1.
    XHTML1_1,
}

impl HtmlVersion {
    /// Return the DOCTYPE (DTD) that corresponds to this version of the HTML standard
    pub fn doctype(&self) -> &'static str {
        match self {
            Self::HTML5 => "<!DOCTYPE html>",
            Self::HTML4 => {
                r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/HTML4/loose.dtd">"#
            }
            Self::XHTML1_0 => {
                r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#
            }
            Self::XHTML1_1 => {
                r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">"#
            }
        }
    }

    /// Return the set of attributes that should be applied to the `HtmlPage`'s opening HTML tag
    pub fn html_attrs(&self) -> Attributes {
        match self {
            Self::XHTML1_0 => Attributes::from([("xmlns", "http://www.w3.org/1999/xhtml")]),
            Self::XHTML1_1 => Attributes::from([
                ("xmlns", "http://www.w3.org/1999/xhtml"),
                ("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"),
                (
                    "xsi:schemaLocation",
                    "http://www.w3.org/MarkUp/SCHEMA/xhtml11.xsd",
                ),
                ("xml:lang", "en"),
            ]),

            _ => Attributes::default(),
        }
    }
}
