//! Collections of known CSS pseudo classes.
//!
//! Data is copied from [Stylelint](https://github.com/stylelint/stylelint/blob/main/lib/reference/keywordSets.js).

use ahash::AHashSet;
use once_cell::sync::Lazy;

/// `<an+b>` notation pseudo classes.
pub static AN_PLUS_B_NOTATION_PSEUDO_CLASSES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(4);
    set.insert("nth-column");
    set.insert("nth-last-column");
    set.insert("nth-last-of-type");
    set.insert("nth-of-type");
    set
});

/// Linguistic pseudo classes.
pub static LINGUISTIC_PSEUDO_CLASSES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(2);
    set.insert("dir");
    set.insert("lang");
    set
});

/// Logical combinations pseudo classes.
pub static LOGICAL_COMBINATIONS_PSEUDO_CLASSES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(5);
    set.insert("has");
    set.insert("is");
    set.insert("matches");
    set.insert("not");
    set.insert("where");
    set
});

/// `<an+b>` of s notation pseudo classes.
pub static AN_PLUS_B_OF_S_NOTATION_PSEUDO_CLASSES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(2);
    set.insert("nth-child");
    set.insert("nth-last-child");
    set
});

/// Uncategorized pseudo classes.
pub static OTHER_PSEUDO_CLASSES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(50);
    set.insert("active");
    set.insert("any-link");
    set.insert("autofill");
    set.insert("blank");
    set.insert("checked");
    set.insert("current");
    set.insert("default");
    set.insert("defined");
    set.insert("disabled");
    set.insert("empty");
    set.insert("enabled");
    set.insert("first-child");
    set.insert("first-of-type");
    set.insert("focus");
    set.insert("focus-within");
    set.insert("focus-visible");
    set.insert("fullscreen");
    set.insert("fullscreen-ancestor");
    set.insert("future");
    set.insert("host");
    set.insert("host-context");
    set.insert("hover");
    set.insert("indeterminate");
    set.insert("in-range");
    set.insert("invalid");
    set.insert("last-child");
    set.insert("last-of-type");
    set.insert("link");
    set.insert("only-child");
    set.insert("only-of-type");
    set.insert("optional");
    set.insert("out-of-range");
    set.insert("past");
    set.insert("placeholder-shown");
    set.insert("playing");
    set.insert("picture-in-picture");
    set.insert("paused");
    set.insert("read-only");
    set.insert("read-write");
    set.insert("required");
    set.insert("root");
    set.insert("scope");
    set.insert("state");
    set.insert("target");
    set.insert("unresolved");
    set.insert("user-invalid");
    set.insert("user-valid");
    set.insert("valid");
    set.insert("visited");
    set.insert("window-inactive"); // for ::selection (chrome)
    set
});

/// Pseudo classes which are vendor-specific and with vendor prefix.
pub static VENDOR_SPECIFIC_PSEUDO_CLASSES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(29);
    set.insert("-khtml-drag");
    set.insert("-moz-any");
    set.insert("-moz-any-link");
    set.insert("-moz-broken");
    set.insert("-moz-drag-over");
    set.insert("-moz-first-node");
    set.insert("-moz-focusring");
    set.insert("-moz-full-screen");
    set.insert("-moz-full-screen-ancestor");
    set.insert("-moz-last-node");
    set.insert("-moz-loading");
    set.insert("-moz-meter-optimum");
    set.insert("-moz-meter-sub-optimum");
    set.insert("-moz-meter-sub-sub-optimum");
    set.insert("-moz-placeholder");
    set.insert("-moz-submit-invalid");
    set.insert("-moz-suppressed");
    set.insert("-moz-ui-invalid");
    set.insert("-moz-ui-valid");
    set.insert("-moz-user-disabled");
    set.insert("-moz-window-inactive");
    set.insert("-ms-fullscreen");
    set.insert("-ms-input-placeholder");
    set.insert("-webkit-drag");
    set.insert("-webkit-any");
    set.insert("-webkit-any-link");
    set.insert("-webkit-autofill");
    set.insert("-webkit-full-screen");
    set.insert("-webkit-full-screen-ancestor");
    set
});

/// WebKit scrollbar pseudo classes.
pub static WEBKIT_SCROLLBAR_PSEUDO_CLASSES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(11);
    set.insert("horizontal");
    set.insert("vertical");
    set.insert("decrement");
    set.insert("increment");
    set.insert("start");
    set.insert("end");
    set.insert("double-button");
    set.insert("single-button");
    set.insert("no-button");
    set.insert("corner-present");
    set.insert("window-inactive");
    set
});
