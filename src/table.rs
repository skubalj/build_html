use crate::attributes::Attributes;
use crate::Html;
use std::collections::HashMap;
use std::fmt::Display;

/// Parse the provided slice of elements into a table row
fn parse_table_row<T>(row: T, cell_tag: &str) -> String
where
    T: IntoIterator,
    T::Item: Display,
{
    row.into_iter()
        .map(|element| format!("<{tag}>{content}</{tag}>", tag = cell_tag, content = element))
        .chain(std::iter::once("</tr>".into()))
        .fold(String::from("<tr>"), |a, n| a + &n)
}

/// Represents an HTML `<table>` element with all its children.
///
/// The easiest way to make a table is by simply passing in a table:
///
#[derive(Debug)]
pub struct Table {
    thead: Vec<String>,
    tbody: Vec<String>,
    attr: Attributes,
}

impl Html for Table {
    fn to_html_string(&self) -> String {
        format!(
            "<table{attr}><thead>{thead}</thead><tbody>{tbody}</tbody></table>",
            attr = self.attr,
            thead = self.thead.join(""),
            tbody = self.tbody.join(""),
        )
    }
}

impl<T> From<T> for Table
where
    T: IntoIterator,
    T::Item: IntoIterator,
    <<T as std::iter::IntoIterator>::Item as IntoIterator>::Item: Display,
{
    fn from(source: T) -> Self {
        let mut iterator = source.into_iter();
        let header_row = match iterator.next() {
            Some(row) => parse_table_row(row, "th"),
            None => return Table::new(), // Return an empty table if the source is empty
        };
        let body_rows = iterator.map(|row| parse_table_row(row, "td")).collect();
        Table {
            thead: vec![header_row],
            tbody: body_rows,
            attr: Attributes::default(),
        }
    }
}

impl Table {
    pub fn new() -> Self {
        Table {
            thead: Vec::new(),
            tbody: Vec::new(),
            attr: Attributes::default(),
        }
    }

    /// Associates the specified map of attributes with this `Table`.
    ///
    /// Note that this operation overrides all previous `with_attribute` calls on
    /// this `Table`
    ///
    /// # Example
    /// ```
    /// # use html_gen::*;
    /// # use maplit::hashmap;
    /// let table = Table::new()
    ///     .with_attributes(hashmap! {"id" => "my-table"})
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     r#"<table id="my-table"><thead></thead><tbody></tbody></table>"#
    /// );
    /// ```
    pub fn with_attributes(mut self, attributes: HashMap<&str, &str>) -> Self {
        self.attr = Attributes::from(attributes);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_arr() {
        // Arrange
        let arr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        // Act
        let result = Table::from(&arr).to_html_string();

        // Assert
        assert_eq!(
            result,
            concat!(
                "<table><thead>",
                "<tr><th>1</th><th>2</th><th>3</th></tr>",
                "</thead><tbody>",
                "<tr><td>4</td><td>5</td><td>6</td></tr>",
                "<tr><td>7</td><td>8</td><td>9</td></tr>",
                "</tbody></table>"
            )
        )
    }

    #[test]
    fn test_from_vec() {
        // Arrange
        let arr = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        // Act
        let result = Table::from(&arr).to_html_string();

        // Assert
        assert_eq!(
            result,
            concat!(
                "<table><thead>",
                "<tr><th>1</th><th>2</th><th>3</th></tr>",
                "</thead><tbody>",
                "<tr><td>4</td><td>5</td><td>6</td></tr>",
                "<tr><td>7</td><td>8</td><td>9</td></tr>",
                "</tbody></table>"
            )
        )
    }
}
