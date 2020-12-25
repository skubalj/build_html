use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attributes {
    attributes: HashMap<String, String>,
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = self
            .attributes
            .iter()
            .map(|(key, value)| format!(r#" {}="{}""#, key, value))
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
    /// Create a new empty set of attributes
    pub fn empty() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        // Act
        let sut = Attributes::empty();

        // Assert
        assert!(sut.attributes.is_empty())
    }
}
