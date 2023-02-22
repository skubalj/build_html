/// The HTML tag with XML attribute, which is very useful for HTML emails.
pub const HTML_XML: &str = "<html xmlns=\"http://www.w3.org/1999/xhtml\">";
/// The doctype for legacy XHTML header. This is still common in HTML emails for backwards compatibility with different email clients
pub const XHTML_1_DOT_0: &str = "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">";
/// The doctype for newest version of HTML which is HTML5
pub const HTML5: &str = "<!DOCTYPE html>";
/// Plain HTML tag
pub const HTML_PLAIN_TAG: &str = "<html>";
