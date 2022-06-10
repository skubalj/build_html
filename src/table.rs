//! This module contains the implementations used to add HTML tables to this library
//!
//! Tables are provided using the `Table` struct, and are loaded from 1 and 2D data
//! structures which implement the `IntoIterator` struct

use crate::attributes::Attributes;
use crate::Html;
use std::fmt::{Display, Write};

/// Parse the provided slice of elements into a table row
fn write_table_row<T>(aggregator: &mut String, row: T, cell_tag: &str)
where
    T: IntoIterator,
    T::Item: Display,
{
    aggregator.push_str("<tr>");
    for element in row.into_iter() {
        write!(
            aggregator,
            "<{tag}>{content}</{tag}>",
            tag = cell_tag,
            content = element
        )
        .unwrap();
    }
    aggregator.push_str("</tr>");
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
/// let html_table = Table::from(source_table)
///     .with_header_row(['A', 'B', 'C'])
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
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Table {
    thead: String,
    tbody: String,
    attr: Attributes,
}

impl Html for Table {
    fn to_html_string(&self) -> String {
        format!(
            "<table{attr}><thead>{thead}</thead><tbody>{tbody}</tbody></table>",
            attr = self.attr,
            thead = self.thead,
            tbody = self.tbody,
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
        let mut body = String::new();
        for row in source.into_iter() {
            write_table_row(&mut body, row, "td");
        }
        Table {
            thead: String::new(),
            tbody: body,
            attr: Attributes::default(),
        }
    }
}

impl Table {
    /// Creates a new table with an empty header and body
    pub fn new() -> Self {
        Self::default()
    }

    /// Associates the specified map of attributes with this `Table`.
    ///
    /// Note that this operation overrides all previous `with_attribute` calls on
    /// this `Table`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut table = Table::new();
    /// table.add_attributes([("id", "my-table")]);
    ///
    /// assert_eq!(
    ///     table.to_html_string(),
    ///     r#"<table id="my-table"><thead></thead><tbody></tbody></table>"#
    /// );
    /// ```
    pub fn add_attributes<A, S>(&mut self, attributes: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.attr = Attributes::from(attributes);
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
    /// let mut table = Table::new();
    /// table.add_header_row(vec!["Mon", "Tues", "Wed", "Thurs", "Fri"]);
    /// assert_eq!(
    ///     table.to_html_string(),
    ///     concat!(
    ///         "<table><thead>",
    ///         "<tr><th>Mon</th><th>Tues</th><th>Wed</th><th>Thurs</th><th>Fri</th></tr>",
    ///         "</thead><tbody></tbody></table>"
    ///     )
    /// )
    /// ```
    pub fn add_header_row<T>(&mut self, row: T)
    where
        T: IntoIterator,
        T::Item: Display,
    {
        write_table_row(&mut self.thead, row, "th");
    }

    /// Adds the specified row to the table header
    ///
    /// Note that no checking is done to ensure that the row is of the proper length
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .with_header_row(vec!["Mon", "Tues", "Wed", "Thurs", "Fri"])
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
    pub fn with_header_row<T>(mut self, row: T) -> Self
    where
        T: IntoIterator,
        T::Item: Display,
    {
        self.add_header_row(row);
        self
    }

    /// Adds the specified row to the table body
    ///
    /// Note that no checking is done to ensure that the row is of the proper length
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut table = Table::new();
    /// table.add_body_row(vec![1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(
    ///     table.to_html_string(),
    ///     concat!(
    ///         "<table><thead></thead><tbody>",
    ///         "<tr><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td></tr>",
    ///         "</tbody></table>"
    ///     )
    /// )
    /// ```
    pub fn add_body_row<T>(&mut self, row: T)
    where
        T: IntoIterator,
        T::Item: Display,
    {
        write_table_row(&mut self.tbody, row, "td");
    }

    /// Adds the specified row to the table body
    ///
    /// Note that no checking is done to ensure that the row is of the proper length
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .with_body_row(vec![1, 2, 3, 4, 5])
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
    pub fn with_body_row<T>(mut self, row: T) -> Self
    where
        T: IntoIterator,
        T::Item: Display,
    {
        self.add_body_row(row);
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
        let result = Table::from(arr).to_html_string();

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
