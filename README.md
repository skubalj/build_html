[![Rust](https://github.com/skubalj/build_html/actions/workflows/rust.yml/badge.svg)](https://github.com/skubalj/build_html/actions/workflows/rust.yml)

build_html: Rust HTML Generation
==============================

This crate allows HTML strings to be generated from within Rust code using the Builder pattern.
In general, this library will attempt to create a 'minimal' version of the HTML; tags are 
concatinated without additional spaces being added. For this reason, this library may not be
optimal for use in applications intending to display the raw HTML. 

The main application for this crate, and its impetus, is providing simple server-side rendering 
for a web application. However, it could also be used to generate HTML reports from within other
applications.

This crate is written in purely safe Rust with no production dependencies.

## Use
Everything you need to use this crate has been exported from the crate root. This means that
you can get easy access to every element using the import: `use build_html::*`.

While the easiest way to use the library is with a root import, you can also import items
piecemeal and prefix types with the package name. Note that the traits `Html` and 
`HtmlContainer` must be in scope for the library to be useful: 
```rust
use build_html::{self, Html, HtmlContainer};

let page = build_html::HtmlPage::new()
    .with_paragraph("Some Text")
    .to_html_string();
```

This project was created with the builder pattern in mind. To create an HTML document, start with
an `HtmlPage`, and build up the document with chained method calls. Once the document is built up,
convert it to a `String` using `to_html_string()`. The `Display` trait is also implemented for all
`Html` elements, meaning these elements can be converted into HTML using `format!()` as well.

While adding content, required attributes are specified as method parameters. These parameters are
generic on the `ToString` trait, allowing many useful types to be used. Additional optional
parameters (such as `id` or `class` attributes) can be added by passing in some type implementing 
the `IntoIterator` trait which has items which are 2-tuples of objects implementing `ToString`. 
This means that you can use anything from a `HashMap<String, String>` to a `Vec<(&str, &str)>` to 
(new with Rust 1.53) arrays of 2-tuples of static strings. 

```rust
use build_html::*;

let html: String = HtmlPage::new()
    .with_title("My Page")
    .with_header(1, "Main Content:")
    .with_container(
        Container::new(ContainerType::Article)
            .with_attributes([("id", "article1")])
            .with_header_attr(2, "Hello, World", [("id", "article-head"), ("class", "header")])
            .with_paragraph("This is a simple HTML demo")
    )
    .to_html_string();
```

produces a string equivalent to:

```html
<!DOCTYPE html>
<html>
    <head>
        <title>My Page</title>
    </head>
    <body>
        <h1>Main Content:</h1>
        <article id="article1">
            <h2 id="article-head" class="header">Hello World</h2>
            <p>This is a simple HTML demo</p>
        </article>
    </body>
</html>
```

## Supported Features
This library does not intend to support *all* tags and features from the HTML specification.
However, an effort has been made to provide the most common features for a majority of use cases.

Currently, this library supports adding the following HTML features / tags:

### Body Elements
* Containers (`<article>`, `<div>`, `<main>`)
* Headers (`<h1>`, `<h2>`, `<h3>`, ... )
* Images (`<img>`)
* Links (`<a>`)
* Lists (`<li>`, `<ol>`)
* Paragraphs (`<p>`)
* Preformatted Text (`<pre>`)
* Tables (`<table>`)

### Header Elements
* Links (`<link>`)
* Meta (`<meta>`)
* Scripts (`<script>`)
* Style (`<style>`)
* Title (`<title>`)

### Extensibility
In the event that you require additional tags or types not implemented in this library, you
can achieve this using one of two escape hatches.
1. You can implement your own HTML types and add them using `HtmlContainer::add_html`
2. You can add one-off raw content using `HtmlContainer::add_raw`

## Contributing
If you have an idea for making this library better, feel free to open an issue or pull request on 
GitHub! I try to respond within a reasonable amount of time, but please keep in mind that
maintaining this library is not my full time job.

## Acknowledgment
This project was made possible thanks to the following great projects:
* [test-case](https://crates.io/crates/test-case): Marcin Sas-Szymanski, Wojciech Polak
* [Rust](https://rust-lang.org)

## License
This project is licensed under the [MIT license](https://mit-license.org).

Copyright (C) 2020-22 Joseph Skubal and Contributors