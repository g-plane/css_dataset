/// Known CSS at-rule names.
pub static AT_RULES: [&str; 40] = [
    "annotation",
    "apply",
    "character-variant",
    "charset",
    "counter-style",
    "custom-media",
    "custom-selector",
    "document",
    "font-face",
    "font-feature-values",
    "import",
    "keyframes",
    "layer",
    "media",
    "namespace",
    "nest",
    "ornaments",
    "page",
    "property",
    "styleset",
    "stylistic",
    "supports",
    "swash",
    "viewport",
    // below are page margin at-rules
    "top-left-corner",
    "top-left",
    "top-center",
    "top-right",
    "top-right-corner",
    "bottom-left-corner",
    "bottom-left",
    "bottom-center",
    "bottom-right",
    "bottom-right-corner",
    "left-top",
    "left-middle",
    "left-bottom",
    "right-top",
    "right-middle",
    "right-bottom",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_rules() {
        assert!(AT_RULES.contains(&"media"));
        assert!(AT_RULES.contains(&"keyframes"));
    }
}
