//! Lists of known HTML tags, MathML tags and SVG tags.
//!
//! Though tags aren't directly related to CSS, but may be useful.

use ahash::AHashSet;
use once_cell::sync::Lazy;

/// Standard HTML tags.
pub static STANDARD_HTML_TAGS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(117);
    set.insert("a");
    set.insert("abbr");
    set.insert("address");
    set.insert("area");
    set.insert("article");
    set.insert("aside");
    set.insert("audio");
    set.insert("b");
    set.insert("base");
    set.insert("bdi");
    set.insert("bdo");
    set.insert("blockquote");
    set.insert("body");
    set.insert("br");
    set.insert("button");
    set.insert("canvas");
    set.insert("caption");
    set.insert("cite");
    set.insert("code");
    set.insert("col");
    set.insert("colgroup");
    set.insert("data");
    set.insert("datalist");
    set.insert("dd");
    set.insert("del");
    set.insert("details");
    set.insert("dfn");
    set.insert("dialog");
    set.insert("div");
    set.insert("dl");
    set.insert("dt");
    set.insert("em");
    set.insert("embed");
    set.insert("fieldset");
    set.insert("figcaption");
    set.insert("figure");
    set.insert("footer");
    set.insert("form");
    set.insert("h1");
    set.insert("h2");
    set.insert("h3");
    set.insert("h4");
    set.insert("h5");
    set.insert("h6");
    set.insert("head");
    set.insert("header");
    set.insert("hgroup");
    set.insert("hr");
    set.insert("html");
    set.insert("i");
    set.insert("iframe");
    set.insert("img");
    set.insert("input");
    set.insert("ins");
    set.insert("kbd");
    set.insert("label");
    set.insert("legend");
    set.insert("li");
    set.insert("link");
    set.insert("main");
    set.insert("map");
    set.insert("mark");
    set.insert("math");
    set.insert("menu");
    set.insert("menuitem");
    set.insert("meta");
    set.insert("meter");
    set.insert("nav");
    set.insert("noscript");
    set.insert("object");
    set.insert("ol");
    set.insert("optgroup");
    set.insert("option");
    set.insert("output");
    set.insert("p");
    set.insert("param");
    set.insert("picture");
    set.insert("pre");
    set.insert("progress");
    set.insert("q");
    set.insert("rb");
    set.insert("rp");
    set.insert("rt");
    set.insert("rtc");
    set.insert("ruby");
    set.insert("s");
    set.insert("samp");
    set.insert("script");
    set.insert("search");
    set.insert("section");
    set.insert("select");
    set.insert("slot");
    set.insert("small");
    set.insert("source");
    set.insert("span");
    set.insert("strong");
    set.insert("style");
    set.insert("sub");
    set.insert("summary");
    set.insert("sup");
    set.insert("svg");
    set.insert("table");
    set.insert("tbody");
    set.insert("td");
    set.insert("template");
    set.insert("textarea");
    set.insert("tfoot");
    set.insert("th");
    set.insert("thead");
    set.insert("time");
    set.insert("title");
    set.insert("tr");
    set.insert("track");
    set.insert("u");
    set.insert("ul");
    set.insert("var");
    set.insert("video");
    set.insert("wbr");
    set
});

/// Non-standard HTML tags.
pub static NON_STANDARD_HTML_TAGS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(23);
    set.insert("acronym");
    set.insert("applet");
    set.insert("basefont");
    set.insert("big");
    set.insert("blink");
    set.insert("center");
    set.insert("content");
    set.insert("dir");
    set.insert("font");
    set.insert("frame");
    set.insert("frameset");
    set.insert("hgroup");
    set.insert("isindex");
    set.insert("keygen");
    set.insert("listing");
    set.insert("marquee");
    set.insert("nobr");
    set.insert("noembed");
    set.insert("plaintext");
    set.insert("spacer");
    set.insert("strike");
    set.insert("tt");
    set.insert("xmp");
    set
});

/// Known SVG tags.
///
/// Exported from <https://github.com/element-io/svg-tags>.
pub static SVG_TAGS: Lazy<AHashSet<&'static str>> =
    Lazy::new(|| include!(concat!(env!("OUT_DIR"), "/svg_tags.rs")));

/// Known MathML tags.
///
/// Copied from <https://github.com/wooorm/mathml-tag-names>.
pub static MATH_ML_TAGS: Lazy<AHashSet<&'static str>> = Lazy::new(|| {
    let mut set = AHashSet::with_capacity(189);
    set.insert("abs");
    set.insert("and");
    set.insert("annotation");
    set.insert("annotation-xml");
    set.insert("apply");
    set.insert("approx");
    set.insert("arccos");
    set.insert("arccosh");
    set.insert("arccot");
    set.insert("arccoth");
    set.insert("arccsc");
    set.insert("arccsch");
    set.insert("arcsec");
    set.insert("arcsech");
    set.insert("arcsin");
    set.insert("arcsinh");
    set.insert("arctan");
    set.insert("arctanh");
    set.insert("arg");
    set.insert("bvar");
    set.insert("card");
    set.insert("cartesianproduct");
    set.insert("ceiling");
    set.insert("ci");
    set.insert("cn");
    set.insert("codomain");
    set.insert("complexes");
    set.insert("compose");
    set.insert("condition");
    set.insert("conjugate");
    set.insert("cos");
    set.insert("cosh");
    set.insert("cot");
    set.insert("coth");
    set.insert("csc");
    set.insert("csch");
    set.insert("csymbol");
    set.insert("curl");
    set.insert("declare");
    set.insert("degree");
    set.insert("determinant");
    set.insert("diff");
    set.insert("divergence");
    set.insert("divide");
    set.insert("domain");
    set.insert("domainofapplication");
    set.insert("emptyset");
    set.insert("encoding");
    set.insert("eq");
    set.insert("equivalent");
    set.insert("eulergamma");
    set.insert("exists");
    set.insert("exp");
    set.insert("exponentiale");
    set.insert("factorial");
    set.insert("factorof");
    set.insert("false");
    set.insert("floor");
    set.insert("fn");
    set.insert("forall");
    set.insert("function");
    set.insert("gcd");
    set.insert("geq");
    set.insert("grad");
    set.insert("gt");
    set.insert("ident");
    set.insert("image");
    set.insert("imaginary");
    set.insert("imaginaryi");
    set.insert("implies");
    set.insert("in");
    set.insert("infinity");
    set.insert("int");
    set.insert("integers");
    set.insert("intersect");
    set.insert("interval");
    set.insert("inverse");
    set.insert("lambda");
    set.insert("laplacian");
    set.insert("lcm");
    set.insert("leq");
    set.insert("limit");
    set.insert("list");
    set.insert("ln");
    set.insert("log");
    set.insert("logbase");
    set.insert("lowlimit");
    set.insert("lt");
    set.insert("maction");
    set.insert("malign");
    set.insert("maligngroup");
    set.insert("malignmark");
    set.insert("malignscope");
    set.insert("math");
    set.insert("matrix");
    set.insert("matrixrow");
    set.insert("max");
    set.insert("mean");
    set.insert("median");
    set.insert("menclose");
    set.insert("merror");
    set.insert("mfenced");
    set.insert("mfrac");
    set.insert("mfraction");
    set.insert("mglyph");
    set.insert("mi");
    set.insert("min");
    set.insert("minus");
    set.insert("mlabeledtr");
    set.insert("mmultiscripts");
    set.insert("mn");
    set.insert("mo");
    set.insert("mode");
    set.insert("moment");
    set.insert("momentabout");
    set.insert("mover");
    set.insert("mpadded");
    set.insert("mphantom");
    set.insert("mprescripts");
    set.insert("mroot");
    set.insert("mrow");
    set.insert("ms");
    set.insert("mspace");
    set.insert("msqrt");
    set.insert("mstyle");
    set.insert("msub");
    set.insert("msubsup");
    set.insert("msup");
    set.insert("mtable");
    set.insert("mtd");
    set.insert("mtext");
    set.insert("mtr");
    set.insert("munder");
    set.insert("munderover");
    set.insert("naturalnumbers");
    set.insert("neq");
    set.insert("none");
    set.insert("not");
    set.insert("notanumber");
    set.insert("notin");
    set.insert("notprsubset");
    set.insert("notsubset");
    set.insert("or");
    set.insert("otherwise");
    set.insert("outerproduct");
    set.insert("partialdiff");
    set.insert("pi");
    set.insert("piece");
    set.insert("piecewice");
    set.insert("piecewise");
    set.insert("plus");
    set.insert("power");
    set.insert("primes");
    set.insert("product");
    set.insert("prsubset");
    set.insert("quotient");
    set.insert("rationals");
    set.insert("real");
    set.insert("reals");
    set.insert("reln");
    set.insert("rem");
    set.insert("root");
    set.insert("scalarproduct");
    set.insert("sdev");
    set.insert("sec");
    set.insert("sech");
    set.insert("select");
    set.insert("selector");
    set.insert("semantics");
    set.insert("sep");
    set.insert("set");
    set.insert("setdiff");
    set.insert("sin");
    set.insert("sinh");
    set.insert("subset");
    set.insert("sum");
    set.insert("tan");
    set.insert("tanh");
    set.insert("tendsto");
    set.insert("times");
    set.insert("transpose");
    set.insert("true");
    set.insert("union");
    set.insert("uplimit");
    set.insert("var");
    set.insert("variance");
    set.insert("vector");
    set.insert("vectorproduct");
    set.insert("xor");
    set
});
