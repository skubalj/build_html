//! This module contains the implementations used to add HTML tables to this library
//!
//! Tables are provided using the `Table` struct, and are loaded from 1 and 2D data
//! structures which implement the `IntoIterator` struct

use crate::Html;
use crate::{attributes::Attributes, HtmlContainer};
use std::fmt::{self, Display, Formatter};

/// The different types of table cells
#[derive(Debug, Default)]
pub enum TableCellType {
    /// Data elements using `<td>` tags
    #[default]
    Data,
    /// Header elements using `<th>` tags
    Header,
}

impl Display for TableCellType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Data => write!(f, "td"),
            Self::Header => write!(f, "th"),
        }
    }
}

/// A single table cell
///
/// `TableCell` implements [`HtmlContainer`], so it can be filled just like any other
/// [`Container`](crate::Container).
///
/// # Example
/// ```
/// # use build_html::*;
/// let cell = TableCell::new(TableCellType::Header)
///     .with_attributes([("id", "header-cell"), ("class", "headers")])
///     .with_paragraph("Here's a paragraph!")
///     .to_html_string();
///
/// assert_eq!(cell, r#"<th id="header-cell" class="headers"><p>Here's a paragraph!</p></th>"#);
/// ```
#[derive(Debug, Default)]
pub struct TableCell {
    cell_type: TableCellType,
    attr: Attributes,
    content: String,
}

impl Html for TableCell {
    fn to_html_string(&self) -> String {
        format!(
            "<{tag}{attr}>{content}</{tag}>",
            tag = self.cell_type,
            attr = self.attr,
            content = self.content.to_html_string()
        )
    }
}

impl HtmlContainer for TableCell {
    fn add_html<H: Html>(&mut self, html: H) {
        self.content.push_str(&html.to_html_string());
    }
}

impl TableCell {
    /// Create a new TableCell with the given type
    pub fn new(cell_type: TableCellType) -> Self {
        Self {
            cell_type,
            attr: Attributes::default(),
            content: String::new(),
        }
    }

    /// Set the attributes for this row.
    ///
    /// Note that this operation overrides all previous invocations of `with_attributes`.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let out = TableCell::default()
    ///     .with_attributes([("id", "first-cell")])
    ///     .with_paragraph("Hello, World!")
    ///     .to_html_string();
    /// assert_eq!(out, r#"<td id="first-cell"><p>Hello, World!</p></td>"#)
    /// ```
    pub fn with_attributes<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.attr = Attributes::from(attributes);
        self
    }
}

/// A builder for more manual control over individual table elements
///
/// # Example
/// ```
/// # use build_html::*;
/// let row = TableRow::new()
///     .with_attributes([("id", "my-row")])
///     .with_cell(TableCell::new(TableCellType::Header).with_raw("Header"))
///     .with_cell(TableCell::default().with_raw(1))
///     .to_html_string();
///
/// assert_eq!(row, r#"<tr id="my-row"><th>Header</th><td>1</td></tr>"#);
/// ```
#[derive(Debug, Default)]
pub struct TableRow {
    attr: Attributes,
    content: String,
}

impl Html for TableRow {
    fn to_html_string(&self) -> String {
        format!("<tr{}>{}</tr>", self.attr, self.content)
    }
}

impl<T> From<T> for TableRow
where
    T: IntoIterator,
    T::Item: Display,
{
    fn from(elements: T) -> Self {
        elements.into_iter().fold(Self::new(), |a, n| {
            a.with_cell(TableCell::default().with_raw(n))
        })
    }
}

impl TableRow {
    /// Create a new, empty TableRow
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the attributes for this row.
    ///
    /// Note that this operation overrides all previous invocations of `with_attributes`.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let out = TableRow::new()
    ///     .with_attributes([("id", "first-row"), ("class", "table-rows")])
    ///     .with_cell(TableCell::default())
    ///     .to_html_string();
    /// assert_eq!(out, r#"<tr id="first-row" class="table-rows"><td></td></tr>"#);
    /// ```
    pub fn with_attributes<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.attr = Attributes::from(attributes);
        self
    }

    /// Add a cell to this row.
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut out = TableRow::new();
    /// out.add_cell(TableCell::default().with_paragraph("Hello, World!"));
    /// assert_eq!(out.to_html_string(), "<tr><td><p>Hello, World!</p></td></tr>");
    /// ```
    pub fn add_cell(&mut self, cell: TableCell) {
        self.content.push_str(&cell.to_html_string());
    }

    /// Nest the given cell inside this row
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let out = TableRow::new()
    ///     .with_cell(TableCell::default().with_paragraph("Hello, World!"))
    ///     .to_html_string();
    /// assert_eq!(out, "<tr><td><p>Hello, World!</p></td></tr>");
    /// ```
    pub fn with_cell(mut self, cell: TableCell) -> Self {
        self.add_cell(cell);
        self
    }
}

/// Represents an HTML `<table>` element with all its children.
///
/// The easiest way to make a table is by simply passing in a 2D Array or `Vec`.
/// Using this method, the entire contents will be placed in `<td>` elements within
/// the `<tbody>`. If a header row is desired, one can be added manually.
///
/// If you need more control, for example to add attributes to individual cells, you can configure
/// custom rows using the [`TableRow`] and [`TableCell`] builders. These rows can then be added to
/// the table body or header with [`with_custom_body_row`](Table::with_custom_body_row) and
/// [`with_custom_header_row`](Table::with_custom_header_row), respectively.
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
    table_attr: Attributes,
    thead_attr: Attributes,
    tbody_attr: Attributes,
}

impl Html for Table {
    fn to_html_string(&self) -> String {
        format!(
            concat!(
                "<table{table_attr}>",
                "<thead{thead_attr}>{thead}</thead>",
                "<tbody{tbody_attr}>{tbody}</tbody>",
                "</table>",
            ),
            table_attr = self.table_attr,
            thead_attr = self.thead_attr,
            thead = self.thead,
            tbody_attr = self.tbody_attr,
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
        source
            .into_iter()
            .fold(Table::new(), |a, n| a.with_body_row(n))
    }
}

impl Table {
    /// Creates a new table with an empty header and body
    pub fn new() -> Self {
        Self::default()
    }

    /// Associates the specified map of attributes with this `Table`.
    ///
    /// Note that this operation overrides all previous `add_attributes` calls on
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
        self.table_attr = Attributes::from(attributes);
    }

    /// Associates the specified map of attributes with this `Table`.
    ///
    /// Note that this operation overrides all previous `with_attributes` calls on
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
        self.add_attributes(attributes);
        self
    }

    /// Associates the specified map of attributes with the `thead` of this `Table`.
    ///
    /// Note that this operation overrides all previous `add_thead_attributes` calls on
    /// this `Table`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut table = Table::new();
    /// table.add_thead_attributes([("id", "table-header")]);
    ///
    /// assert_eq!(
    ///     table.to_html_string(),
    ///     r#"<table><thead id="table-header"></thead><tbody></tbody></table>"#
    /// );
    /// ```
    pub fn add_thead_attributes<A, S>(&mut self, attributes: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.thead_attr = Attributes::from(attributes);
    }

    /// Associates the specified map of attributes with the `thead` of this `Table`.
    ///
    /// Note that this operation overrides all previous `with_thead_attributes` calls on
    /// this `Table`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .with_attributes([("id", "my-table")])
    ///     .with_thead_attributes([("id", "my-thead")])
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     r#"<table id="my-table"><thead id="my-thead"></thead><tbody></tbody></table>"#
    /// );
    /// ```
    pub fn with_thead_attributes<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.add_thead_attributes(attributes);
        self
    }

    /// Associates the specified map of attributes with the `tbody` of this `Table`.
    ///
    /// Note that this operation overrides all previous `add_tbody_attributes` calls on
    /// this `Table`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut table = Table::new();
    /// table.add_tbody_attributes([("id", "table-body")]);
    ///
    /// assert_eq!(
    ///     table.to_html_string(),
    ///     r#"<table><thead></thead><tbody id="table-body"></tbody></table>"#
    /// );
    /// ```
    pub fn add_tbody_attributes<A, S>(&mut self, attributes: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.tbody_attr = Attributes::from(attributes);
    }

    /// Associates the specified map of attributes with the `tbody` of this `Table`.
    ///
    /// Note that this operation overrides all previous `with_tbody_attributes` calls on
    /// this `Table`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .with_attributes([("id", "my-table")])
    ///     .with_tbody_attributes([("id", "my-body")])
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     r#"<table id="my-table"><thead></thead><tbody id="my-body"></tbody></table>"#
    /// );
    /// ```
    pub fn with_tbody_attributes<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.add_tbody_attributes(attributes);
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
        self.add_custom_header_row(row.into_iter().fold(TableRow::new(), |a, n| {
            a.with_cell(TableCell::new(TableCellType::Header).with_raw(n))
        }))
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

    /// Add the specified row to the table header
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut table = Table::new();
    /// table.add_custom_header_row(
    ///     TableRow::new()
    ///         .with_cell(TableCell::new(TableCellType::Header).with_raw("col1"))
    ///         .with_cell(TableCell::new(TableCellType::Header).with_raw("col2"))
    ///         .with_cell(TableCell::new(TableCellType::Header).with_raw("col3"))
    /// );
    ///
    /// assert_eq!(
    ///     table.to_html_string(),
    ///     concat!(
    ///         "<table><thead>",
    ///         "<tr><th>col1</th><th>col2</th><th>col3</th></tr>",
    ///         "</thead><tbody></tbody></table>",
    ///     ),
    /// );
    /// ```
    pub fn add_custom_header_row(&mut self, row: TableRow) {
        self.thead.push_str(&row.to_html_string());
    }

    /// Add the specified row to the table header
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .with_custom_header_row(
    ///         TableRow::new()
    ///             .with_attributes([("class", "long-row")])
    ///             .with_cell(TableCell::new(TableCellType::Header).with_raw("col1"))
    ///             .with_cell(TableCell::new(TableCellType::Data).with_raw("col2"))
    ///             .with_cell(
    ///                 TableCell::new(TableCellType::Header)
    ///                     .with_attributes([("id", "third")])
    ///                     .with_raw("col3")
    ///             ),
    ///     )
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         r#"<table><thead><tr class="long-row">"#,
    ///         r#"<th>col1</th><td>col2</td><th id="third">col3</th>"#,
    ///         "</tr></thead><tbody></tbody></table>",
    ///     ),
    /// );
    /// ```
    pub fn with_custom_header_row(mut self, row: TableRow) -> Self {
        self.add_custom_header_row(row);
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
        self.add_custom_body_row(row.into_iter().fold(TableRow::new(), |a, n| {
            a.with_cell(TableCell::new(TableCellType::Data).with_raw(n))
        }))
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

    /// Add the specified row to the table body
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut table = Table::new();
    /// table.add_custom_body_row(
    ///     TableRow::new()
    ///         .with_cell(TableCell::default().with_raw("col1"))
    ///         .with_cell(TableCell::default().with_raw("col2"))
    ///         .with_cell(TableCell::default().with_raw("col3"))
    /// );
    ///
    /// assert_eq!(
    ///     table.to_html_string(),
    ///     concat!(
    ///         "<table><thead></thead><tbody>",
    ///         "<tr><td>col1</td><td>col2</td><td>col3</td></tr>",
    ///         "</tbody></table>",
    ///     ),
    /// );
    /// ```
    pub fn add_custom_body_row(&mut self, row: TableRow) {
        self.tbody.push_str(&row.to_html_string());
    }

    /// Add the specified row to the table body
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .with_custom_body_row(
    ///         TableRow::new()
    ///             .with_attributes([("class", "long-row")])
    ///             .with_cell(TableCell::default().with_raw("col1"))
    ///             .with_cell(TableCell::default().with_raw("col2"))
    ///             .with_cell(
    ///                 TableCell::default()
    ///                     .with_attributes([("id", "third")])
    ///                     .with_raw("col3")
    ///             ),
    ///     )
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         r#"<table><thead></thead><tbody><tr class="long-row">"#,
    ///         r#"<td>col1</td><td>col2</td><td id="third">col3</td>"#,
    ///         "</tr></tbody></table>",
    ///     ),
    /// );
    /// ```
    pub fn with_custom_body_row(mut self, row: TableRow) -> Self {
        self.add_custom_body_row(row);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Container, ContainerType, Html, HtmlContainer};

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

    #[test]
    fn test_inner_html() {
        // Arrange
        let table = Table::from([
            [
                Container::default()
                    .with_paragraph("This_is_column_one")
                    .to_html_string(),
                Container::new(ContainerType::Article)
                    .with_paragraph("This_is_column_two")
                    .to_html_string(),
            ],
            [
                Container::default().to_html_string(),
                Container::default()
                    .with_table(Table::from([[1, 2], [3, 4]]))
                    .to_html_string(),
            ],
        ]);

        let expected = "<table>
                <thead></thead>
                <tbody>
                    <tr>
                        <td><div><p>This_is_column_one</p></div></td>
                        <td><article><p>This_is_column_two</p></article></td>
                    </tr>
                    <tr>
                        <td><div></div></td>
                        <td><div><table>
                            <thead></thead>
                            <tbody>
                                <tr>
                                    <td>1</td>
                                    <td>2</td>
                                </tr>
                                <tr>
                                    <td>3</td>
                                    <td>4</td>
                                </tr>
                            </tbody>
                        </table></div></td>
                    </tr>
                </tbody>
            </table>";

        assert_eq!(
            table.to_html_string(),
            expected
                .chars()
                .filter(|x| !x.is_ascii_whitespace())
                .collect::<String>()
        );
    }
}
