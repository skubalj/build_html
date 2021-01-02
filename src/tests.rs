//! This module contains tests for the `lib.rs` file.
//! 
//! Tests were moved to their own file, as the `lib.rs` file grew to be very large

use super::*;

mod html_page {
    use super::*;

    #[test]
    fn default() {
        // Arrange
        let sut = HtmlPage::default();

        // Act
        let html_string = sut.to_html_string();

        // Assert
        assert_eq!(
            html_string,
            "<!DOCTYPE html><html><head></head><body></body></html>"
        )
    }
}

mod container {
    use super::*;
    use maplit::hashmap;
    use test_case::test_case;

    #[test_case(ContainerType::Article; "article")]
    #[test_case(ContainerType::Div; "div")]
    #[test_case(ContainerType::Main; "main")]
    fn test_nesting(container_type: ContainerType) {
        // Expected
        let content = concat!(
            r#"<h1 id="main-header">header</h1>"#,
            r#"<img src="myimage.png" alt="test image">"#,
            r#"<a href="rust-lang.org">Rust Home</a>"#,
            r#"<p class="red-text">Sample Text</p>"#,
            r#"<pre class="code">Text</pre>"#
        );

        // Act
        let sut = Container::new(container_type, None)
            .add_header_attr(1, "header", hashmap! {"id" => "main-header"})
            .add_image("myimage.png", "test image")
            .add_link("rust-lang.org", "Rust Home")
            .add_paragraph_attr("Sample Text", hashmap! {"class" => "red-text"})
            .add_preformatted_attr("Text", hashmap! {"class" => "code"});

        // Assert
        assert_eq!(
            sut.to_html_string(),
            format!(
                "<{tag}>{content}</{tag}>",
                tag = container_type,
                content = content
            )
        )
    }

    #[test_case(ContainerType::OrderedList; "ordered_list")]
    #[test_case(ContainerType::UnorderedList; "unordered_list")]
    fn test_list(container_type: ContainerType) {
        // Expected
        let content = concat!(
            r#"<li><h1 id="main-header">header</h1></li>"#,
            r#"<li><img src="myimage.png" alt="test image"></li>"#,
            r#"<li><a href="rust-lang.org">Rust Home</a></li>"#,
            r#"<li><p class="red-text">Sample Text</p></li>"#,
            r#"<li><pre class="code">Text</pre></li>"#
        );

        // Act
        let sut = Container::new(container_type, None)
            .add_header_attr(1, "header", hashmap! {"id" => "main-header"})
            .add_image("myimage.png", "test image")
            .add_link("rust-lang.org", "Rust Home")
            .add_paragraph_attr("Sample Text", hashmap! {"class" => "red-text"})
            .add_preformatted_attr("Text", hashmap! {"class" => "code"});

        // Assert
        assert_eq!(
            sut.to_html_string(),
            format!(
                "<{tag}>{content}</{tag}>",
                tag = container_type,
                content = content
            )
        )
    }
}