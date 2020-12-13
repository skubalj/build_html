html-gen: Rust HTML Generation
==============================

> THIS PROJECT IS STILL UNDER HEAVY DEVELOPMENT

This project is a library for generating HTML code from within Rust. It was conceived due to the 
lack of a simple way to dynamically create an HTML document from within Rust. 

## Use
This project was created with the decorator pattern in mind. To create an HTML document, start with
an `HttpPage`, and build up the document with chained method calls. Once the document is built up,
convert it to a `String` using `to_html_string()`. 

```rust
use http_gen::*;

fn main() {
    let html: String = HtmlPage::new()
        .add_title("My Page")
        .add_h(1, "Hello, World")
        .add_p("This is a simple HTML demo")
        .to_html_string();
    
    println!("{}", html);
}
```

produces a string equivalent to: 

```html
<!DOCTYPE html>
<html>
    <head>
        <title>My Page</title>
    </head>
    <body>
        <h1>Hello World</h1>
        <p>This is a simple HTML demo</p>
    </body>
</html>
```

## Acknowledgment
Special thanks to Sean McArthur; the way that filters work in [warp](https://crates.io/crates/warp)
was a major inspiration for how programmers would interact with this library.

This project was made possible thanks to the following great projects:
* [test-case](https://crates.io/crates/test-case): Marcin Sas-Szymanski, Wojciech Polak
* [Rust](https://rust-lang.org)

## License
This project is licensed under the [MIT license](https://mit-license.org). In other words, it's
free for you to use for whatever purpose you want. However, to the maximum extent allowed under the
law, this software has NO WARRANTY.
Copyright (C) 2020 Joseph Skubal