#![warn(missing_docs)]

//! This crate contains CSS reference data set, such as known functions and properties.
//!
//! This crate should be used for CSS tooling, not normal users.

mod at_rule;
pub mod autoprefixable;
pub mod media;
pub mod pseudo_classes;
pub mod pseudo_elements;
pub mod tags;

pub use at_rule::AT_RULES;
use std::{collections::HashMap, sync::LazyLock};

/// Known CSS functions.
pub static FUNCTIONS: [&str; 645] = include!(concat!(env!("OUT_DIR"), "/css_functions.rs"));

/// Known CSS properties.
pub static PROPERTIES: [&str; 1225] = include!(concat!(env!("OUT_DIR"), "/css_properties.rs"));

/// Data of properties shorthand.
///
/// Copied from <https://github.com/stylelint/stylelint/blob/main/lib/reference/shorthandData.js>.
pub static PROPERTIES_SHORTHAND: LazyLock<HashMap<&'static str, Vec<&'static str>>> =
    LazyLock::new(|| include!(concat!(env!("OUT_DIR"), "/css_properties_shorthand.rs")));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions() {
        assert!(FUNCTIONS.contains(&"var"));
        assert!(FUNCTIONS.contains(&"env"));
        assert!(FUNCTIONS.contains(&"linear-gradient"));
    }

    #[test]
    fn test_properties() {
        assert!(PROPERTIES.contains(&"padding"));
        assert!(PROPERTIES.contains(&"animation"));
        assert!(PROPERTIES.contains(&"accent-color"));
    }

    #[test]
    fn test_properties_shorthand() {
        assert_eq!(
            PROPERTIES_SHORTHAND.get("margin").unwrap(),
            &["margin-top", "margin-bottom", "margin-left", "margin-right"]
        );
    }
}
