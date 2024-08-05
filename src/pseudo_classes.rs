//! Collections of known CSS pseudo classes.
//!
//! Data is copied from [Stylelint](https://github.com/stylelint/stylelint/blob/main/lib/reference/keywordSets.js).

/// `<an+b>` notation pseudo classes.
pub static AN_PLUS_B_NOTATION_PSEUDO_CLASSES: [&str; 4] = [
    "nth-column",
    "nth-last-column",
    "nth-last-of-type",
    "nth-of-type",
];

/// Linguistic pseudo classes.
pub static LINGUISTIC_PSEUDO_CLASSES: [&str; 2] = ["dir", "lang"];

/// Logical combinations pseudo classes.
pub static LOGICAL_COMBINATIONS_PSEUDO_CLASSES: [&str; 5] =
    ["has", "is", "matches", "not", "where"];

/// `<an+b>` of s notation pseudo classes.
pub static AN_PLUS_B_OF_S_NOTATION_PSEUDO_CLASSES: [&str; 2] = ["nth-child", "nth-last-child"];

/// Uncategorized pseudo classes.
pub static OTHER_PSEUDO_CLASSES: [&str; 50] = [
    "active",
    "any-link",
    "autofill",
    "blank",
    "checked",
    "current",
    "default",
    "defined",
    "disabled",
    "empty",
    "enabled",
    "first-child",
    "first-of-type",
    "focus",
    "focus-within",
    "focus-visible",
    "fullscreen",
    "fullscreen-ancestor",
    "future",
    "host",
    "host-context",
    "hover",
    "indeterminate",
    "in-range",
    "invalid",
    "last-child",
    "last-of-type",
    "link",
    "only-child",
    "only-of-type",
    "optional",
    "out-of-range",
    "past",
    "placeholder-shown",
    "playing",
    "picture-in-picture",
    "paused",
    "read-only",
    "read-write",
    "required",
    "root",
    "scope",
    "state",
    "target",
    "unresolved",
    "user-invalid",
    "user-valid",
    "valid",
    "visited",
    "window-inactive", // for ::selection (chrome)
];

/// Pseudo classes which are vendor-specific and with vendor prefix.
pub static VENDOR_SPECIFIC_PSEUDO_CLASSES: [&str; 29] = [
    "-khtml-drag",
    "-moz-any",
    "-moz-any-link",
    "-moz-broken",
    "-moz-drag-over",
    "-moz-first-node",
    "-moz-focusring",
    "-moz-full-screen",
    "-moz-full-screen-ancestor",
    "-moz-last-node",
    "-moz-loading",
    "-moz-meter-optimum",
    "-moz-meter-sub-optimum",
    "-moz-meter-sub-sub-optimum",
    "-moz-placeholder",
    "-moz-submit-invalid",
    "-moz-suppressed",
    "-moz-ui-invalid",
    "-moz-ui-valid",
    "-moz-user-disabled",
    "-moz-window-inactive",
    "-ms-fullscreen",
    "-ms-input-placeholder",
    "-webkit-drag",
    "-webkit-any",
    "-webkit-any-link",
    "-webkit-autofill",
    "-webkit-full-screen",
    "-webkit-full-screen-ancestor",
];

/// WebKit scrollbar pseudo classes.
pub static WEBKIT_SCROLLBAR_PSEUDO_CLASSES: [&str; 11] = [
    "horizontal",
    "vertical",
    "decrement",
    "increment",
    "start",
    "end",
    "double-button",
    "single-button",
    "no-button",
    "corner-present",
    "window-inactive",
];
