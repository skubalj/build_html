//! This module contains the implementations used to add HTML tables to this library
//!
//! Tables are provided using the `Table` struct, and are loaded from 1 and 2D data
//! structures which implement the `IntoIterator` struct

use crate::attributes::Attributes;
use crate::Html;
use std::fmt::Display;

/// Parse the provided slice of elements into a table row
fn parse_table_row<T>(row: T, cell_tag: &str) -> String
where
    T: IntoIterator,
    T::Item: Display,
{
    row.into_iter()
        .map(|element| {
            format!(
                "<{tag}>{content}</{tag}>",
                tag = cell_tag,
                content = element
            )
        })
        .chain(std::iter::once("</tr>".into()))
        .fold(String::from("<tr>"), |a, n| a + &n)
}

/// Represents an HTML `<table>` element with all its children.
///
/// The easiest way to make a table is by simply passing in a 2D Array or `Vec`.
/// Using this method, the entire contents will be placed in `<td>` elements within
/// the `<tbody>`. If a header row is desired, one can be added manually.
///
/// # Example
/// ```
/// # use build_html::*;
/// let source_table = [
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9]
/// ];
/// let html_table = Table::from(&source_table)
///     .add_header_row(&['A', 'B', 'C'])
///     .to_html_string();
///
/// assert_eq!(
///     html_table,
///     concat!(
///         "<table><thead>",
///         "<tr><th>A</th><th>B</th><th>C</th></tr>",
///         "</thead><tbody>",
///         "<tr><td>1</td><td>2</td><td>3</td></tr>",
///         "<tr><td>4</td><td>5</td><td>6</td></tr>",
///         "<tr><td>7</td><td>8</td><td>9</td></tr>",
///         "</tbody></table>"
///     )
/// );
/// ```
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

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<T> for Table
where
    T: IntoIterator,
    T::Item: IntoIterator,
    <<T as std::iter::IntoIterator>::Item as IntoIterator>::Item: Display,
{
    fn from(source: T) -> Self {
        let body_rows = source
            .into_iter()
            .map(|row| parse_table_row(row, "td"))
            .collect();
        Table {
            thead: Vec::new(),
            tbody: body_rows,
            attr: Attributes::default(),
        }
    }
}

impl Table {
    /// Creates a new table with an empty header and body
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
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .with_attributes([("id", "my-table")])
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     r#"<table id="my-table"><thead></thead><tbody></tbody></table>"#
    /// );
    /// ```
    pub fn with_attributes<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.attr = Attributes::from(attributes);
        self
    }

    /// Adds the specified row to the table header
    ///
    /// Note that no checking is done to ensure that the row is of the proper length
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .add_header_row(vec!["Mon", "Tues", "Wed", "Thurs", "Fri"])
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "<table><thead>",
    ///         "<tr><th>Mon</th><th>Tues</th><th>Wed</th><th>Thurs</th><th>Fri</th></tr>",
    ///         "</thead><tbody></tbody></table>"
    ///     )
    /// )
    /// ```
    pub fn add_header_row<T>(mut self, row: T) -> Self
    where
        T: IntoIterator,
        T::Item: Display,
    {
        self.thead.push(parse_table_row(row, "th"));
        self
    }

    /// Adds the specified row to the table body
    ///
    /// Note that no checking is done to ensure that the row is of the proper length
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .add_body_row(vec![1, 2, 3, 4, 5])
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "<table><thead></thead><tbody>",
    ///         "<tr><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td></tr>",
    ///         "</tbody></table>"
    ///     )
    /// )
    /// ```
    pub fn add_body_row<T>(mut self, row: T) -> Self
    where
        T: IntoIterator,
        T::Item: Display,
    {
        self.tbody.push(parse_table_row(row, "td"));
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
                "<table><thead></thead><tbody>",
                "<tr><td>1</td><td>2</td><td>3</td></tr>",
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
                "<table><thead></thead><tbody>",
                "<tr><td>1</td><td>2</td><td>3</td></tr>",
                "<tr><td>4</td><td>5</td><td>6</td></tr>",
                "<tr><td>7</td><td>8</td><td>9</td></tr>",
                "</tbody></table>"
            )
        )
    }
}
