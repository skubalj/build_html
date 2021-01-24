[![pipeline status](https://gitlab.com/skubalj/html-gen/badges/master/pipeline.svg)](https://gitlab.com/skubalj/html-gen/-/commits/master)

html-gen: Rust HTML Generation
==============================

This project is a library for generating HTML code. It was conceived due to the lack of a simple 
way to dynamically create an HTML document from within Rust. 

I see this crate being useful for outputting HTML reports of an operation, or for providing simple
HTML content from a web server.

## Use
Everything you need to use this crate has been exported from the crate root. This means that
you can get easy access to every element using the import: `use html_gen::*`.

If compatibility is important, or you don't need access to every element, you can also use the
import `use html_gen;` and prefix types with the package name: 
```rust
use html_gen;

let page = html_gen::HtmlPage::new();
```

This project was created with the builder pattern in mind. To create an HTML document, start with
an `HttpPage`, and build up the document with chained method calls. Once the document is built up,
convert it to a `String` using `to_html_string()`. 

While adding content, required attributes are specified as method parameters. Additional optional
parameters (such as `id` or `class` attributes) can be added as a `HashMap`. I recommend using the
[`maplit`](https://crates.io/crates/maplit) library to create `HashMap` literals. 

```rust
use html_gen::*;
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