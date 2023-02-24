//! This module contains the `Attributes` struct which defines a collection of
//! attributes which can be added to an HTML tag.

use std::fmt;
use std::fmt::Write;
use std::iter::FromIterator;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Attributes(String);

impl fmt::Display for Attributes {
    /// Converts this set of `Attributes` to an attribute string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<I: IntoIterator<Item = (S, S)>, S: ToString> From<I> for Attributes {
    fn from(iter: I) -> Self {
        let mut attributes = String::new();
        for (k, v) in iter.into_iter() {
            write!(attributes, r#" {}="{}""#, k.to_string(), v.to_string())
                .expect("Failed to write into String");
        }
        Self(attributes)
    }
}

impl<S: ToString> FromIterator<(S, S)> for Attributes {
    fn from_iter<T: IntoIterator<Item = (S, S)>>(iter: T) -> Self {
        iter.into()
    }
}
