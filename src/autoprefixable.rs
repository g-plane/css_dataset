//! Properties, property values, pseudo classes and pseudo elements that can be
//! prefixed by [Autoprefixer](https://github.com/postcss/autoprefixer).
//!
//! Copied from [Stylelint](https://github.com/stylelint/stylelint/blob/main/lib/utils/isAutoprefixable.js).

use ahash::AHashSet;
use once_cell::sync::Lazy;

/// Autoprefixable properties.
pub static PROPERTIES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(142);

    set.insert("align-content");
    set.insert("align-items");
    set.insert("align-self");
    set.insert("animation");
    set.insert("animation-delay");
    set.insert("animation-direction");
    set.insert("animation-duration");
    set.insert("animation-fill-mode");
    set.insert("animation-iteration-count");
    set.insert("animation-name");
    set.insert("animation-play-state");
    set.insert("animation-timing-function");
    set.insert("appearance");
    set.insert("backdrop-filter");
    set.insert("backface-visibility");
    set.insert("background-clip");
    set.insert("background-origin");
    set.insert("background-size");
    set.insert("border-block-end");
    set.insert("border-block-start");
    set.insert("border-bottom-left-radius");
    set.insert("border-bottom-right-radius");
    set.insert("border-image");
    set.insert("border-inline-end");
    set.insert("border-inline-start");
    set.insert("border-radius");
    set.insert("border-top-left-radius");
    set.insert("border-top-right-radius");
    set.insert("box-decoration-break");
    set.insert("box-shadow");
    set.insert("box-sizing");
    set.insert("break-after");
    set.insert("break-before");
    set.insert("break-inside");
    set.insert("clip-path");
    set.insert("color-adjust");
    set.insert("column-count");
    set.insert("column-fill");
    set.insert("column-gap");
    set.insert("column-rule");
    set.insert("column-rule-color");
    set.insert("column-rule-style");
    set.insert("column-rule-width");
    set.insert("column-span");
    set.insert("column-width");
    set.insert("columns");
    set.insert("filter");
    set.insert("flex");
    set.insert("flex-basis");
    set.insert("flex-direction");
    set.insert("flex-flow");
    set.insert("flex-grow");
    set.insert("flex-shrink");
    set.insert("flex-wrap");
    set.insert("flow-from");
    set.insert("flow-into");
    set.insert("font-feature-settings");
    set.insert("font-kerning");
    set.insert("font-language-override");
    set.insert("font-variant-ligatures");
    set.insert("grid-area");
    set.insert("grid-column");
    set.insert("grid-column-align");
    set.insert("grid-column-end");
    set.insert("grid-column-start");
    set.insert("grid-row");
    set.insert("grid-row-align");
    set.insert("grid-row-end");
    set.insert("grid-row-start");
    set.insert("grid-template");
    set.insert("grid-template-areas");
    set.insert("grid-template-columns");
    set.insert("grid-template-rows");
    set.insert("hyphens");
    set.insert("image-rendering");
    set.insert("justify-content");
    set.insert("margin-block-end");
    set.insert("margin-block-start");
    set.insert("margin-inline-end");
    set.insert("margin-inline-start");
    set.insert("mask");
    set.insert("mask-border");
    set.insert("mask-border-outset");
    set.insert("mask-border-repeat");
    set.insert("mask-border-slice");
    set.insert("mask-border-source");
    set.insert("mask-border-width");
    set.insert("mask-clip");
    set.insert("mask-composite");
    set.insert("mask-image");
    set.insert("mask-origin");
    set.insert("mask-position");
    set.insert("mask-repeat");
    set.insert("mask-size");
    set.insert("object-fit");
    set.insert("object-position");
    set.insert("order");
    set.insert("overscroll-behavior");
    set.insert("padding-block-end");
    set.insert("padding-block-start");
    set.insert("padding-inline-end");
    set.insert("padding-inline-start");
    set.insert("perspective");
    set.insert("perspective-origin");
    set.insert("place-self");
    set.insert("region-fragment");
    set.insert("scroll-snap-coordinate");
    set.insert("scroll-snap-destination");
    set.insert("scroll-snap-points-x");
    set.insert("scroll-snap-points-y");
    set.insert("scroll-snap-type");
    set.insert("shape-image-threshold");
    set.insert("shape-margin");
    set.insert("shape-outside");
    set.insert("tab-size");
    set.insert("text-align-last");
    set.insert("text-decoration");
    set.insert("text-decoration-color");
    set.insert("text-decoration-line");
    set.insert("text-decoration-skip");
    set.insert("text-decoration-skip-ink");
    set.insert("text-decoration-style");
    set.insert("text-emphasis");
    set.insert("text-emphasis-color");
    set.insert("text-emphasis-position");
    set.insert("text-emphasis-style");
    set.insert("text-orientation");
    set.insert("text-overflow");
    set.insert("text-size-adjust");
    set.insert("text-spacing");
    set.insert("touch-action");
    set.insert("transform");
    set.insert("transform-origin");
    set.insert("transform-style");
    set.insert("transition");
    set.insert("transition-delay");
    set.insert("transition-duration");
    set.insert("transition-property");
    set.insert("transition-timing-function");
    set.insert("user-select");
    set.insert("writing-mode");

    // Not a standard property, just from `-ms-interpolation-mode`.
    set.insert("interpolation-mode");

    set
});

/// Autoprefixable pseudo classes. All items are starting with vendor prefix.
pub static PSEUDO_CLASSES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(10);

    set.insert(":-moz-any-link");
    set.insert(":-moz-full-screen");
    set.insert(":-moz-placeholder");
    set.insert(":-moz-placeholder-shown");
    set.insert(":-moz-read-only");
    set.insert(":-moz-read-write");
    set.insert(":-ms-fullscreen");
    set.insert(":-ms-input-placeholder");
    set.insert(":-webkit-any-link");
    set.insert(":-webkit-full-screen");

    set
});

/// Autoprefixable pseudo elements. All items are starting with vendor prefix.
pub static PSEUDO_ELEMENTS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(5);

    set.insert("::-moz-placeholder");
    set.insert("::-moz-selection");
    set.insert("::-ms-input-placeholder");
    set.insert("::-webkit-backdrop");
    set.insert("::-webkit-input-placeholder");

    set
});

/// Autoprefixable at-rules. All items are starting with `@` and vendor prefix, except `@resolution`.
pub static AT_RULES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(8);

    set.insert("@-khtml-keyframes");
    set.insert("@-moz-keyframes");
    set.insert("@-ms-keyframes");
    set.insert("@-ms-viewport");
    set.insert("@-o-keyframes");
    set.insert("@-o-viewport");
    set.insert("@-webkit-keyframes");
    set.insert("@resolution");

    set
});

/// Autoprefixable property values. All items are starting with vendor prefix.
pub static PROPERTY_VALUES: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(56);

    set.insert("-moz-available");
    set.insert("-moz-box");
    set.insert("-moz-calc");
    set.insert("-moz-crisp-edges");
    set.insert("-moz-element");
    set.insert("-moz-fit-content");
    set.insert("-moz-grab");
    set.insert("-moz-grabbing");
    set.insert("-moz-inline-box");
    set.insert("-moz-isolate");
    set.insert("-moz-isolate-override");
    set.insert("-moz-linear-gradient");
    set.insert("-moz-max-content");
    set.insert("-moz-min-content");
    set.insert("-moz-plaintext");
    set.insert("-moz-radial-gradient");
    set.insert("-moz-repeating-linear-gradient");
    set.insert("-moz-repeating-radial-gradient");
    set.insert("-moz-zoom-in");
    set.insert("-moz-zoom-out");
    set.insert("-ms-flexbox");
    set.insert("-ms-grid");
    set.insert("-ms-inline-flexbox");
    set.insert("-ms-inline-grid");
    set.insert("-ms-linear-gradient");
    set.insert("-ms-radial-gradient");
    set.insert("-ms-repeating-linear-gradient");
    set.insert("-ms-repeating-radial-gradient");
    set.insert("-o-linear-gradient");
    set.insert("-o-pixelated");
    set.insert("-o-radial-gradient");
    set.insert("-o-repeating-linear-gradient");
    set.insert("-o-repeating-radial-gradient");
    set.insert("-webkit-box");
    set.insert("-webkit-calc");
    set.insert("-webkit-cross-fade");
    set.insert("-webkit-fill-available");
    set.insert("-webkit-filter");
    set.insert("-webkit-fit-content");
    set.insert("-webkit-flex");
    set.insert("-webkit-grab");
    set.insert("-webkit-grabbing");
    set.insert("-webkit-image-set");
    set.insert("-webkit-inline-box");
    set.insert("-webkit-inline-flex");
    set.insert("-webkit-isolate");
    set.insert("-webkit-linear-gradient");
    set.insert("-webkit-max-content");
    set.insert("-webkit-min-content");
    set.insert("-webkit-optimize-contrast");
    set.insert("-webkit-radial-gradient");
    set.insert("-webkit-repeating-linear-gradient");
    set.insert("-webkit-repeating-radial-gradient");
    set.insert("-webkit-sticky");
    set.insert("-webkit-zoom-in");
    set.insert("-webkit-zoom-out");

    set
});
