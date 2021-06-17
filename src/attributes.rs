//! This module contains the `Attributes` struct which defines a collection of
//! attributes which can be added to an HTML tag.

use std::fmt;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attributes {
    attributes: Vec<(String, String)>,
}

impl fmt::Display for Attributes {
    /// Converts this set of `Attributes` to an attribute string.
    ///
    /// Note that the attributes are automatically sorted.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let attribute_tags: Vec<String> = self
            .attributes
            .iter()
            .map(|(key, value)| format!(r#" {}="{}""#, key, value))
            .collect();
        write!(f, "{}", attribute_tags.join(""))
    }
}

impl<I: IntoIterator<Item = (S, S)>, S: ToString> From<I> for Attributes {
    fn from(iter: I) -> Self {
        let attributes: Vec<(String, String)> = iter
            .into_iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        Self { attributes }
    }
}

impl<S: ToString> FromIterator<(S, S)> for Attributes {
    fn from_iter<T: IntoIterator<Item = (S, S)>>(iter: T) -> Self {
        iter.into()
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::empty()
    }
}

impl Attributes {
    /// Create a new empty set of attributes. This is the default way of
    /// creating an attribute without any content. To create an attribute
    /// set with pre-defined content, see [`Attributes::from()`]
    pub fn empty() -> Self {
        Attributes {
            attributes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use test_case::test_case;

    #[test]
    fn from_hash() {
        // Act
        let mut sut = Attributes::from(hashmap! {
            "id" => "my-element",
            "class" => "my-css-class"
        });
        sut.attributes.sort();

        // Assert
        assert_eq!(sut.attributes.len(), 2);
        assert_eq!(
            sut.attributes,
            vec![
                (String::from("class"), String::from("my-css-class")),
                (String::from("id"), String::from("my-element")),
            ]
        );
    }

    #[test_case(Vec::new(), "" ; "test_0")]
    #[test_case(vec![("id", "my-id")], r#" id="my-id""# ; "test_1")]
    #[test_case(vec![("id", "my-id"), ("class", "my-class")], r#" id="my-id" class="my-class""# ; "test_2")]
    fn display(map: Vec<(&str, &str)>, expected: &str) {
        // Arrange
        let sut = Attributes::from(map);

        // Act / Assert
        assert_eq!(sut.to_string(), expected);
    }
}
