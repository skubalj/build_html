//! This module contains the `Attributes` struct which defines a collection of
//! attributes which can be added to an HTML tag.

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attributes {
    attributes: HashMap<String, String>,
}

impl fmt::Display for Attributes {
    /// Converts this set of `Attributes` to an attribute string.
    ///
    /// Note that the attributes are automatically sorted.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attribute_tags: Vec<String> = self
            .attributes
            .iter()
            .map(|(key, value)| format!(r#" {}="{}""#, key, value))
            .collect();
        attribute_tags.sort();
        let content = attribute_tags
            .iter()
            .fold(String::new(), |acc, next| acc + &next);

        write!(f, "{}", content)
    }
}

impl From<HashMap<String, String>> for Attributes {
    /// Creates a new `Attributes` collection from the specified HashMap
    fn from(hash: HashMap<String, String>) -> Self {
        Attributes { attributes: hash }
    }
}

impl From<HashMap<&str, &str>> for Attributes {
    /// Creates a new `Attributes` collection from the specified HashMap
    fn from(hash: HashMap<&str, &str>) -> Self {
        Attributes {
            attributes: hash
                .into_iter()
                .map(|(k, v)| (String::from(k), String::from(v)))
                .collect(),
        }
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Attributes {
            attributes: HashMap::new(),
        }
    }
}

impl Attributes {
    /// Create a new empty set of attributes. This is the default way of
    /// creating an attribute without any content. To create an attribute
    /// set with pre-defined content, see [`Attributes::from()`]
    pub fn empty() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use test_case::test_case;

    #[test]
    fn empty() {
        // Act
        let sut = Attributes::empty();

        // Assert
        assert!(sut.attributes.is_empty())
    }

    #[test]
    fn from_hash() {
        // Act
        let sut = Attributes::from(hashmap! {
            "id" => "my-element",
            "class" => "my-css-class"
        });

        // Assert
        assert_eq!(sut.attributes.len(), 2);
        assert_eq!(sut.attributes["id"], "my-element");
        assert_eq!(sut.attributes["class"], "my-css-class");
    }

    #[test_case(HashMap::new(), "" ; "test_0")]
    #[test_case(hashmap! {"id" => "my-id"}, r#" id="my-id""# ; "test_1")]
    #[test_case(hashmap! {"id" => "my-id", "class" => "my-class"}, r#" class="my-class" id="my-id""# ; "test_2")]
    fn display(map: HashMap<&str, &str>, expected: &str) {
        // Arrange
        let sut = Attributes::from(map);

        // Act
        let actual = format!("{}", sut);

        // Assert
        assert_eq!(actual, expected);
    }
}
