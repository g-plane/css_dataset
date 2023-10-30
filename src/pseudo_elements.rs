//! Collections of known CSS pseudo elements.
//!
//! Data is copied from [Stylelint](https://github.com/stylelint/stylelint/blob/main/lib/reference/keywordSets.js).

/// Level one and two pseudo elements.
pub static LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS: phf::Set<&'static str> = phf::phf_set! {
    "before",
    "after",
    "first-line",
    "first-letter",
};

/// Level three and up pseudo elements.
pub static LEVEL_THREE_AND_UP_PSEUDO_ELEMENTS: phf::Set<&'static str> = phf::phf_set! {
    "before",
    "after",
    "first-line",
    "first-letter",
    // These are the ones that require double-colon notation
    "backdrop",
    "content",
    "cue",
    "file-selector-button",
    "grammar-error",
    "highlight",
    "marker",
    "placeholder",
    "selection",
    "shadow",
    "slotted",
    "spelling-error",
    "target-text",
};

/// Shadow tree pseudo elements.
pub static SHADOW_TREE_PSEUDO_ELEMENTS: phf::Set<&'static str> = phf::phf_set! {
    "part",
};

/// WebKit scrollbar pseudo elements.
pub static WEBKIT_SCROLLBAR_PSEUDO_ELEMENTS: phf::Set<&'static str> = phf::phf_set! {
    "-webkit-resizer",
    "-webkit-scrollbar",
    "-webkit-scrollbar-button",
    "-webkit-scrollbar-corner",
    "-webkit-scrollbar-thumb",
    "-webkit-scrollbar-track",
    "-webkit-scrollbar-track-piece",
};

/// Pseudo elements which are vendor-specific and with vendor prefix.
pub static VENDOR_SPECIFIC_PSEUDO_ELEMENTS: phf::Set<&'static str> = phf::phf_set! {
    "-moz-focus-inner",
    "-moz-focus-outer",
    "-moz-list-bullet",
    "-moz-meter-bar",
    "-moz-placeholder",
    "-moz-progress-bar",
    "-moz-range-progress",
    "-moz-range-thumb",
    "-moz-range-track",
    "-ms-browse",
    "-ms-check",
    "-ms-clear",
    "-ms-expand",
    "-ms-fill",
    "-ms-fill-lower",
    "-ms-fill-upper",
    "-ms-reveal",
    "-ms-thumb",
    "-ms-ticks-after",
    "-ms-ticks-before",
    "-ms-tooltip",
    "-ms-track",
    "-ms-value",
    "-webkit-color-swatch",
    "-webkit-color-swatch-wrapper",
    "-webkit-calendar-picker-indicator",
    "-webkit-clear-button",
    "-webkit-date-and-time-value",
    "-webkit-datetime-edit",
    "-webkit-datetime-edit-ampm-field",
    "-webkit-datetime-edit-day-field",
    "-webkit-datetime-edit-fields-wrapper",
    "-webkit-datetime-edit-hour-field",
    "-webkit-datetime-edit-millisecond-field",
    "-webkit-datetime-edit-minute-field",
    "-webkit-datetime-edit-month-field",
    "-webkit-datetime-edit-second-field",
    "-webkit-datetime-edit-text",
    "-webkit-datetime-edit-week-field",
    "-webkit-datetime-edit-year-field",
    "-webkit-details-marker",
    "-webkit-distributed",
    "-webkit-file-upload-button",
    "-webkit-input-placeholder",
    "-webkit-keygen-select",
    "-webkit-meter-bar",
    "-webkit-meter-even-less-good-value",
    "-webkit-meter-inner-element",
    "-webkit-meter-optimum-value",
    "-webkit-meter-suboptimum-value",
    "-webkit-progress-bar",
    "-webkit-progress-inner-element",
    "-webkit-progress-value",
    "-webkit-search-cancel-button",
    "-webkit-search-decoration",
    "-webkit-search-results-button",
    "-webkit-search-results-decoration",
    "-webkit-slider-runnable-track",
    "-webkit-slider-thumb",
    "-webkit-textfield-decoration-container",
    "-webkit-validation-bubble",
    "-webkit-validation-bubble-arrow",
    "-webkit-validation-bubble-arrow-clipper",
    "-webkit-validation-bubble-heading",
    "-webkit-validation-bubble-message",
    "-webkit-validation-bubble-text-block",
};
