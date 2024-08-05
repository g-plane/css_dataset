//! Data of media query, such as media feature names.

use ahash::AHashSet;
use std::sync::LazyLock;

/// Known media feature names.
// Copied from:
// https://github.com/stylelint/stylelint/blob/e6d6740c581c7289d18337f0fb78a776bf5f654c/lib/reference/keywordSets.js#L660
// https://github.com/stylelint/stylelint/blob/e6d6740c581c7289d18337f0fb78a776bf5f654c/lib/reference/keywordSets.js#L673
pub static MEDIA_FEATURE_NAMES: LazyLock<AHashSet<&'static str>> = LazyLock::new(|| {
    let mut set = AHashSet::with_capacity(52);

    set.insert("any-hover");
    set.insert("any-pointer");
    set.insert("aspect-ratio");
    set.insert("color");
    set.insert("color-gamut");
    set.insert("color-index");
    set.insert("display-mode");
    set.insert("dynamic-range");
    set.insert("forced-colors");
    set.insert("grid");
    set.insert("height");
    set.insert("hover");
    set.insert("inverted-colors");
    set.insert("light-level");
    set.insert("max-aspect-ratio");
    set.insert("max-color");
    set.insert("max-color-index");
    set.insert("max-height");
    set.insert("max-monochrome");
    set.insert("max-resolution");
    set.insert("max-width");
    set.insert("min-aspect-ratio");
    set.insert("min-color");
    set.insert("min-color-index");
    set.insert("min-height");
    set.insert("min-monochrome");
    set.insert("min-resolution");
    set.insert("min-width");
    set.insert("monochrome");
    set.insert("orientation");
    set.insert("overflow-block");
    set.insert("overflow-inline");
    set.insert("pointer");
    set.insert("prefers-color-scheme");
    set.insert("prefers-contrast");
    set.insert("prefers-reduced-motion");
    set.insert("prefers-reduced-transparency");
    set.insert("resolution");
    set.insert("scan");
    set.insert("scripting");
    set.insert("update");
    set.insert("video-dynamic-range");
    set.insert("width");

    // deprecated media features
    set.insert("device-aspect-ratio");
    set.insert("device-height");
    set.insert("device-width");
    set.insert("max-device-aspect-ratio");
    set.insert("max-device-height");
    set.insert("max-device-width");
    set.insert("min-device-aspect-ratio");
    set.insert("min-device-height");
    set.insert("min-device-width");

    set
});
