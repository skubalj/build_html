//! This module contains the implementations used to add HTML tables to this library
//!
//! Tables are provided using the `Table` struct, and are loaded from 1 and 2D data
//! structures which implement the `IntoIterator` struct

use crate::{Html, HtmlChild, HtmlContainer, HtmlElement, HtmlTag};
use std::fmt::{self, Display, Formatter};

/// The different types of table cells
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum TableCellType {
    /// Data elements using `<td>` tags
    #[default]
    Data,
    /// Header elements using `<th>` tags
    Header,
}

impl From<TableCellType> for HtmlTag {
    fn from(value: TableCellType) -> Self {
        match value {
            TableCellType::Data => HtmlTag::TableCell,
            TableCellType::Header => HtmlTag::TableHeaderCell,
        }
    }
}

impl Display for TableCellType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        HtmlTag::from(*self).fmt(f)
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
#[derive(Debug)]
pub struct TableCell(HtmlElement);

impl Default for TableCell {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Html for TableCell {
    fn to_html_string(&self) -> String {
        self.0.to_html_string()
    }
}

impl HtmlContainer for TableCell {
    fn add_html<H: Html>(&mut self, html: H) {
        self.0.add_child(HtmlChild::Raw(html.to_html_string()));
    }
}

impl TableCell {
    /// Create a new TableCell with the given type
    pub fn new(cell_type: TableCellType) -> Self {
        Self(HtmlElement::new(cell_type.into()))
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
        for (k, v) in attributes {
            self.0.add_attribute(k, v);
        }
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
#[derive(Debug)]
pub struct TableRow(HtmlElement);

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}

impl Html for TableRow {
    fn to_html_string(&self) -> String {
        self.0.to_html_string()
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
        Self(HtmlElement::new(HtmlTag::TableRow))
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
    /// assert_eq!(out, r#"<tr id="first-row" class="table-rows"><td/></tr>"#);
    /// ```
    pub fn with_attributes<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        for (k, v) in attributes {
            self.0.add_attribute(k, v);
        }
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
        self.0.add_child(cell.0.into())
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
#[derive(Debug)]
pub struct Table {
    table: HtmlElement,
    thead: HtmlElement,
    tbody: HtmlElement,
    tfoot: HtmlElement,
    caption: Option<HtmlElement>,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Html for Table {
    fn to_html_string(&self) -> String {
        let mut table = self
            .table
            .clone()
            .with_child(self.thead.clone().into())
            .with_child(self.tbody.clone().into());

        // To keep the output the same between versions, only add a footer if there's data in it.
        // This can be made imperative at the next major version.
        if !self.tfoot.children.is_empty() || !self.tfoot.attributes.is_empty() {
            table.add_child(self.tfoot.clone().into());
        }

        if let Some(caption) = self.caption.as_ref() {
            table.add_child(caption.clone().into());
        }

        table.to_html_string()
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
        Self {
            table: HtmlElement::new(HtmlTag::Table),
            thead: HtmlElement::new(HtmlTag::TableHeader),
            tbody: HtmlElement::new(HtmlTag::TableBody),
            tfoot: HtmlElement::new(HtmlTag::TableFooter),
            caption: None,
        }
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
    ///     r#"<table id="my-table"><thead/><tbody/></table>"#
    /// );
    /// ```
    pub fn add_attributes<A, S>(&mut self, attributes: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        for (k, v) in attributes {
            self.table.add_attribute(k, v);
        }
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
    /// assert_eq!(table, r#"<table id="my-table"><thead/><tbody/></table>"#);
    /// ```
    pub fn with_attributes<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.add_attributes(attributes);
        self
    }

    /// Set the caption for the table
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut table = Table::new();
    /// table.add_caption("Demo table");
    /// assert_eq!(
    ///     table.to_html_string(),
    ///     "<table><thead/><tbody/><caption>Demo table</caption></table>",
    /// );
    /// ```
    pub fn add_caption<H: Html>(&mut self, caption: H) {
        self.caption = Some(HtmlElement::new(HtmlTag::TableCaption).with_html(caption));
    }

    /// Set the caption for the table
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::from([[1, 2],[3, 4]])
    ///     .with_header_row(['a', 'b'])
    ///     .with_caption("A demo table")
    ///     .to_html_string();
    /// assert_eq!(table, concat!(
    ///     "<table>",
    ///     "<thead><tr><th>a</th><th>b</th></tr></thead>",
    ///     "<tbody><tr><td>1</td><td>2</td></tr>",
    ///     "<tr><td>3</td><td>4</td></tr></tbody>",
    ///     "<caption>A demo table</caption></table>"
    /// ));
    /// ```
    pub fn with_caption<H: Html>(mut self, caption: H) -> Self {
        self.add_caption(caption);
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
    /// assert_eq!(table.to_html_string(), r#"<table><thead id="table-header"/><tbody/></table>"#);
    /// ```
    pub fn add_thead_attributes<A, S>(&mut self, attributes: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        for (k, v) in attributes {
            self.thead.add_attribute(k, v);
        }
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
    /// assert_eq!(table, r#"<table id="my-table"><thead id="my-thead"/><tbody/></table>"#);
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
    /// assert_eq!(table.to_html_string(), r#"<table><thead/><tbody id="table-body"/></table>"#);
    /// ```
    pub fn add_tbody_attributes<A, S>(&mut self, attributes: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        for (k, v) in attributes {
            self.tbody.add_attribute(k, v);
        }
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
    /// assert_eq!(table, r#"<table id="my-table"><thead/><tbody id="my-body"/></table>"#);
    /// ```
    pub fn with_tbody_attributes<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.add_tbody_attributes(attributes);
        self
    }

    /// Associates the specified map of attributes with the `tfoot` of this `Table`.
    ///
    /// Note that this operation overrides all previous `add_tfoot_attributes` calls on
    /// this `Table`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut table = Table::new();
    /// table.add_tfoot_attributes([("id", "table-footer")]);
    ///
    /// assert_eq!(table.to_html_string(), r#"<table><thead/><tbody/><tfoot id="table-footer"/></table>"#);
    /// ```
    pub fn add_tfoot_attributes<A, S>(&mut self, attributes: A)
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        for (k, v) in attributes {
            self.tfoot.add_attribute(k, v);
        }
    }

    /// Associates the specified map of attributes with the `tfoot` of this `Table`.
    ///
    /// Note that this operation overrides all previous `with_tfoot_attributes` calls on
    /// this `Table`
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .with_attributes([("id", "my-table")])
    ///     .with_tfoot_attributes([("id", "my-foot")])
    ///     .to_html_string();
    ///
    /// assert_eq!(table, r#"<table id="my-table"><thead/><tbody/><tfoot id="my-foot"/></table>"#);
    /// ```
    pub fn with_tfoot_attributes<A, S>(mut self, attributes: A) -> Self
    where
        A: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        self.add_tfoot_attributes(attributes);
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
    ///         "</thead><tbody/></table>"
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
    ///         "</thead><tbody/></table>"
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
    ///         "</thead><tbody/></table>",
    ///     ),
    /// );
    /// ```
    pub fn add_custom_header_row(&mut self, row: TableRow) {
        self.thead.add_child(row.0.into());
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
    ///         "</tr></thead><tbody/></table>",
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
    ///         "<table><thead/><tbody>",
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
            a.with_cell(TableCell::default().with_raw(n))
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
    ///         "<table><thead/><tbody>",
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
    ///         "<table><thead/><tbody>",
    ///         "<tr><td>col1</td><td>col2</td><td>col3</td></tr>",
    ///         "</tbody></table>",
    ///     ),
    /// );
    /// ```
    pub fn add_custom_body_row(&mut self, row: TableRow) {
        self.tbody.add_child(row.0.into());
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
    ///         r#"<table><thead/><tbody><tr class="long-row">"#,
    ///         r#"<td>col1</td><td>col2</td><td id="third">col3</td>"#,
    ///         "</tr></tbody></table>",
    ///     ),
    /// );
    /// ```
    pub fn with_custom_body_row(mut self, row: TableRow) -> Self {
        self.add_custom_body_row(row);
        self
    }

    /// Adds the specified row to the table footer
    ///
    /// Note that no checking is done to ensure that the row is of the proper length
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut table = Table::new();
    /// table.add_footer_row(vec!["Mon", "Tues", "Wed", "Thurs", "Fri"]);
    /// assert_eq!(
    ///     table.to_html_string(),
    ///     concat!(
    ///         "<table>",
    ///         "<thead/><tbody/><tfoot>",
    ///         "<tr><th>Mon</th><th>Tues</th><th>Wed</th><th>Thurs</th><th>Fri</th></tr>",
    ///         "</tfoot></table>"
    ///     )
    /// )
    /// ```
    pub fn add_footer_row<T>(&mut self, row: T)
    where
        T: IntoIterator,
        T::Item: Display,
    {
        self.add_custom_footer_row(row.into_iter().fold(TableRow::new(), |a, n| {
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
    ///     .with_footer_row(vec!["Mon", "Tues", "Wed", "Thurs", "Fri"])
    ///     .to_html_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "<table><thead/><tbody/><tfoot>",
    ///         "<tr><th>Mon</th><th>Tues</th><th>Wed</th><th>Thurs</th><th>Fri</th></tr>",
    ///         "</tfoot></table>"
    ///     )
    /// )
    /// ```
    pub fn with_footer_row<T>(mut self, row: T) -> Self
    where
        T: IntoIterator,
        T::Item: Display,
    {
        self.add_footer_row(row);
        self
    }

    /// Add the specified row to the table header
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let mut table = Table::new();
    /// table.add_custom_footer_row(
    ///     TableRow::new()
    ///         .with_cell(TableCell::new(TableCellType::Header).with_raw("col1"))
    ///         .with_cell(TableCell::new(TableCellType::Header).with_raw("col2"))
    ///         .with_cell(TableCell::new(TableCellType::Header).with_raw("col3"))
    /// );
    ///
    /// assert_eq!(
    ///     table.to_html_string(),
    ///     concat!(
    ///         "<table><thead/><tbody/><tfoot>",
    ///         "<tr><th>col1</th><th>col2</th><th>col3</th></tr>",
    ///         "</tfoot></table>",
    ///     ),
    /// );
    /// ```
    pub fn add_custom_footer_row(&mut self, row: TableRow) {
        self.tfoot.add_child(row.0.into());
    }

    /// Add the specified row to the table header
    ///
    /// # Example
    /// ```
    /// # use build_html::*;
    /// let table = Table::new()
    ///     .with_custom_footer_row(
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
    ///         r#"<table><thead/><tbody/><tfoot><tr class="long-row">"#,
    ///         r#"<th>col1</th><td>col2</td><th id="third">col3</th>"#,
    ///         "</tr></tfoot></table>",
    ///     ),
    /// );
    /// ```
    pub fn with_custom_footer_row(mut self, row: TableRow) -> Self {
        self.add_custom_footer_row(row);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Html, HtmlContainer, HtmlElement, HtmlTag};

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
                "<table><thead/><tbody>",
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
                "<table><thead/><tbody>",
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
                HtmlElement::new(HtmlTag::Div)
                    .with_paragraph("This_is_column_one")
                    .to_html_string(),
                HtmlElement::new(HtmlTag::Article)
                    .with_paragraph("This_is_column_two")
                    .to_html_string(),
            ],
            [
                HtmlElement::new(HtmlTag::Div).to_html_string(),
                HtmlElement::new(HtmlTag::Div)
                    .with_table(Table::from([[1, 2], [3, 4]]))
                    .to_html_string(),
            ],
        ]);

        let expected = "<table>
                <thead/>
                <tbody>
                    <tr>
                        <td><div><p>This_is_column_one</p></div></td>
                        <td><article><p>This_is_column_two</p></article></td>
                    </tr>
                    <tr>
                        <td><div/></td>
                        <td><div><table>
                            <thead/>
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
