//! Collections of known CSS pseudo elements.
//!
//! Data is copied from [Stylelint](https://github.com/stylelint/stylelint/blob/main/lib/reference/keywordSets.js).

use ahash::AHashSet;
use once_cell::sync::Lazy;

/// Level one and two pseudo elements.
pub static LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::new();
    set.insert("before");
    set.insert("after");
    set.insert("first-line");
    set.insert("first-letter");
    set
});

/// Level three and up pseudo elements.
pub static LEVEL_THREE_AND_UP_PSEUDO_ELEMENTS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::new();
    set.insert("before");
    set.insert("after");
    set.insert("first-line");
    set.insert("first-letter");
    // These are the ones that require double-colon notation
    set.insert("backdrop");
    set.insert("content");
    set.insert("cue");
    set.insert("file-selector-button");
    set.insert("grammar-error");
    set.insert("highlight");
    set.insert("marker");
    set.insert("placeholder");
    set.insert("selection");
    set.insert("shadow");
    set.insert("slotted");
    set.insert("spelling-error");
    set.insert("target-text");
    set
});

/// Shadow tree pseudo elements.
pub static SHADOW_TREE_PSEUDO_ELEMENTS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::new();
    set.insert("part");
    set
});

/// WebKit scrollbar pseudo elements.
pub static WEBKIT_SCROLLBAR_PSEUDO_ELEMENTS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::new();
    set.insert("-webkit-resizer");
    set.insert("-webkit-scrollbar");
    set.insert("-webkit-scrollbar-button");
    set.insert("-webkit-scrollbar-corner");
    set.insert("-webkit-scrollbar-thumb");
    set.insert("-webkit-scrollbar-track");
    set.insert("-webkit-scrollbar-track-piece");
    set
});

/// Pseudo elements which are vendor-specific and with vendor prefix.
pub static VENDOR_SPECIFIC_PSEUDO_ELEMENTS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::new();
    set.insert("-moz-focus-inner");
    set.insert("-moz-focus-outer");
    set.insert("-moz-list-bullet");
    set.insert("-moz-meter-bar");
    set.insert("-moz-placeholder");
    set.insert("-moz-progress-bar");
    set.insert("-moz-range-progress");
    set.insert("-moz-range-thumb");
    set.insert("-moz-range-track");
    set.insert("-ms-browse");
    set.insert("-ms-check");
    set.insert("-ms-clear");
    set.insert("-ms-expand");
    set.insert("-ms-fill");
    set.insert("-ms-fill-lower");
    set.insert("-ms-fill-upper");
    set.insert("-ms-reveal");
    set.insert("-ms-thumb");
    set.insert("-ms-ticks-after");
    set.insert("-ms-ticks-before");
    set.insert("-ms-tooltip");
    set.insert("-ms-track");
    set.insert("-ms-value");
    set.insert("-webkit-color-swatch");
    set.insert("-webkit-color-swatch-wrapper");
    set.insert("-webkit-calendar-picker-indicator");
    set.insert("-webkit-clear-button");
    set.insert("-webkit-date-and-time-value");
    set.insert("-webkit-datetime-edit");
    set.insert("-webkit-datetime-edit-ampm-field");
    set.insert("-webkit-datetime-edit-day-field");
    set.insert("-webkit-datetime-edit-fields-wrapper");
    set.insert("-webkit-datetime-edit-hour-field");
    set.insert("-webkit-datetime-edit-millisecond-field");
    set.insert("-webkit-datetime-edit-minute-field");
    set.insert("-webkit-datetime-edit-month-field");
    set.insert("-webkit-datetime-edit-second-field");
    set.insert("-webkit-datetime-edit-text");
    set.insert("-webkit-datetime-edit-week-field");
    set.insert("-webkit-datetime-edit-year-field");
    set.insert("-webkit-details-marker");
    set.insert("-webkit-distributed");
    set.insert("-webkit-file-upload-button");
    set.insert("-webkit-input-placeholder");
    set.insert("-webkit-keygen-select");
    set.insert("-webkit-meter-bar");
    set.insert("-webkit-meter-even-less-good-value");
    set.insert("-webkit-meter-inner-element");
    set.insert("-webkit-meter-optimum-value");
    set.insert("-webkit-meter-suboptimum-value");
    set.insert("-webkit-progress-bar");
    set.insert("-webkit-progress-inner-element");
    set.insert("-webkit-progress-value");
    set.insert("-webkit-search-cancel-button");
    set.insert("-webkit-search-decoration");
    set.insert("-webkit-search-results-button");
    set.insert("-webkit-search-results-decoration");
    set.insert("-webkit-slider-runnable-track");
    set.insert("-webkit-slider-thumb");
    set.insert("-webkit-textfield-decoration-container");
    set.insert("-webkit-validation-bubble");
    set.insert("-webkit-validation-bubble-arrow");
    set.insert("-webkit-validation-bubble-arrow-clipper");
    set.insert("-webkit-validation-bubble-heading");
    set.insert("-webkit-validation-bubble-message");
    set.insert("-webkit-validation-bubble-text-block");
    set
});
