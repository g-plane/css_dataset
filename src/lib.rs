use ahash::AHashSet;
use once_cell::sync::Lazy;

/// Known CSS functions.
pub static FUNCTIONS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let json = include_str!("../vendor/css-functions/index.json");
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
}
