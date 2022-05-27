mod at_rule;

use ahash::AHashSet;
pub use at_rule::AT_RULES;
use once_cell::sync::Lazy;

/// Known CSS functions.
pub static FUNCTIONS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let json = include_str!("../vendor/css-functions/index.json");
    serde_json::from_str(json).unwrap()
});

/// Known CSS properties.
pub static PROPERTIES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let json = include_str!(concat!(env!("OUT_DIR"), "/css-properties.json"));
    serde_json::from_str(json).unwrap()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions() {
        assert!(FUNCTIONS.contains("var"));
        assert!(FUNCTIONS.contains("env"));
        assert!(FUNCTIONS.contains("linear-gradient"));
    }

    #[test]
    fn test_properties() {
        assert!(PROPERTIES.contains("padding"));
        assert!(PROPERTIES.contains("animation"));
        assert!(PROPERTIES.contains("accent-color"));
    }
}
