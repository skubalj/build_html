html-gen: Rust HTML Generation
==============================

> THIS PROJECT IS STILL UNDER HEAVY DEVELOPMENT

This project is a library for generating HTML code from within Rust. It was conceived due to the 
lack of a simple way to dynamically create an HTML document from within Rust. 

## Use
To use this crate, simply use the import: 
```rust
use html_gen::*;
```

This project was created with the decorator pattern in mind. To create an HTML document, start with
an `HttpPage`, and build up the document with chained method calls. Once the document is built up,
convert it to a `String` using `to_html_string()`. 

```rust
use html_gen::*;

let html: String = HtmlPage::new()
    .add_title("My Page")
    .add_header(1, "Main Content:")
    .add_container(
        Container::new(ContainerType::Article)
            .add_header(2, "Hello, World")
            .add_paragraph("This is a simple HTML demo")
    )
    .to_html_string();
```

produces a `String` equivalent to:

```html
<!DOCTYPE html>
<html>
    <head>
        <title>My Page</title>
    </head>
    <body>
        <h1>Main Content:</h1>
        <article>
            <h2>Hello World</h2>
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