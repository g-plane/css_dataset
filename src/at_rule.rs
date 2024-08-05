use ahash::AHashSet;
use std::sync::LazyLock;

/// Known CSS at-rule names.
pub static AT_RULES: LazyLock<AHashSet<&'static str>> = LazyLock::new(|| {
    let mut set = AHashSet::with_capacity(40);
    set.insert("annotation");
    set.insert("apply");
    set.insert("character-variant");
    set.insert("charset");
    set.insert("counter-style");
    set.insert("custom-media");
    set.insert("custom-selector");
    set.insert("document");
    set.insert("font-face");
    set.insert("font-feature-values");
    set.insert("import");
    set.insert("keyframes");
    set.insert("layer");
    set.insert("media");
    set.insert("namespace");
    set.insert("nest");
    set.insert("ornaments");
    set.insert("page");
    set.insert("property");
    set.insert("styleset");
    set.insert("stylistic");
    set.insert("supports");
    set.insert("swash");
    set.insert("viewport");
    // below are page margin at-rules
    set.insert("top-left-corner");
    set.insert("top-left");
    set.insert("top-center");
    set.insert("top-right");
    set.insert("top-right-corner");
    set.insert("bottom-left-corner");
    set.insert("bottom-left");
    set.insert("bottom-center");
    set.insert("bottom-right");
    set.insert("bottom-right-corner");
    set.insert("left-top");
    set.insert("left-middle");
    set.insert("left-bottom");
    set.insert("right-top");
    set.insert("right-middle");
    set.insert("right-bottom");
    set
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_rules() {
        assert!(AT_RULES.contains("media"));
        assert!(AT_RULES.contains("keyframes"));
    }
}
