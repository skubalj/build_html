[![pipeline status](https://gitlab.com/skubalj/html-gen/badges/master/pipeline.svg)](https://gitlab.com/skubalj/html-gen/-/commits/master)

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

If compatibility is important, or you don't need access to every element, you can also import 
piecemeal and prefix types with the package name. Note that the traits `Html` and 
`HtmlContainer` must be exported for the library to be useful: 
```rust
use build_html::{self, Html, HtmlContainer};

let page = build_html::HtmlPage::new()
    .add_paragraph("Some Text")
    .to_html_string();
```

This project was created with the builder pattern in mind. To create an HTML document, start with
an `HtmlPage`, and build up the document with chained method calls. Once the document is built up,
convert it to a `String` using `to_html_string()`. The `Display` trait is also implemented for all
`Html` elements, meaning these elements can be converted into HTML using `format!()` as well.

While adding content, required attributes are specified as method parameters. Additional optional
parameters (such as `id` or `class` attributes) can be added as a `HashMap`. I recommend using the
[`maplit`](https://crates.io/crates/maplit) library to create `HashMap` literals. 

```rust
use build_html::*;
use maplit::hashmap;

let html: String = HtmlPage::new()
    .add_title("My Page")
    .add_header(1, "Main Content:")
    .add_container(
        Container::new(ContainerType::Article)
            .with_attributes(hashmap! {"id" => "article1"})
            .add_header_attr(2, "Hello, World", hashmap! {"id" => "article-head"})
            .add_paragraph("This is a simple HTML demo")
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
            <h2 id="article-head">Hello World</h2>
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

## Acknowledgment
Special thanks to Sean McArthur; the way that filters work in [warp](https://crates.io/crates/warp)
was a major inspiration for how programmers would interact with this library.

This project was made possible thanks to the following great projects:
* [maplit](https://crates.io/crates/maplit): Bluss
* [test-case](https://crates.io/crates/test-case): Marcin Sas-Szymanski, Wojciech Polak
* [Rust](https://rust-lang.org)

## License
This project is licensed under the [MIT license](https://mit-license.org). In other words, it's
free for you to use for whatever purpose you want. However, to the maximum extent allowed under the
law, this software has NO WARRANTY.

Copyright (C) 2020 Joseph Skubal