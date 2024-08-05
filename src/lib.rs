#![warn(missing_docs)]

//! This crate contains CSS reference data set, such as known functions and properties.
//!
//! This crate should be used for CSS tooling, not normal users.

#[cfg(feature = "css_at_rules")]
mod at_rule;
#[cfg(feature = "autoprefixable")]
pub mod autoprefixable;
#[cfg(feature = "media_features")]
pub mod media;
#[cfg(feature = "pseudo_classes")]
pub mod pseudo_classes;
#[cfg(feature = "pseudo_elements")]
pub mod pseudo_elements;
#[cfg(feature = "tags")]
pub mod tags;

#[cfg(feature = "css_at_rules")]
pub use at_rule::AT_RULES;

#[cfg(feature = "css_functions")]
/// Known CSS functions.
pub static FUNCTIONS: [&str; 645] = include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/css-functions/index.json"
));

#[cfg(feature = "css_properties")]
/// Known CSS properties.
pub static PROPERTIES: [&str; 1225] = include!(concat!(env!("OUT_DIR"), "/css_properties.rs"));

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
}
