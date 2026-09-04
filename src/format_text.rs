use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::configuration::Configuration;
use crate::sort::sort_classes;

pub type FormatRange = Option<std::ops::Range<usize>>;

// === Static regexes for built-in attribute patterns ===

/// class="..." (double quote) — standalone class attr only (not :class, not-class, etc.)
/// Uses \s to ensure class is preceded by whitespace (i.e., it's an attribute start)
static CLASS_DQ: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?:^|\s)(?i)class\s*=\s*"([^"]*)""#).unwrap());

/// class='...' (single quote)
static CLASS_SQ: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?:^|\s)(?i)class\s*=\s*'([^']*)'"#).unwrap());

/// className="..." (double quote) — must not be preceded by word char or colon (ns:className)
static CLASSNAME_DQ: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?:^|\s)className\s*=\s*"([^"]*)""#).unwrap());

/// className='...' (single quote)
static CLASSNAME_SQ: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?:^|\s)className\s*=\s*'([^']*)'"#).unwrap());

/// Vue :class="'...'" or v-bind:class="'...'" with single-quoted string inside double-quoted attr
static VUE_CLASS_SQ_IN_DQ: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?:v-bind)?:class\s*=\s*"'([^']*)'"#).unwrap());

/// Vue :class='\"...\"' — double-quoted string inside single-quoted attr (rare but valid)
/// Vue :class="`...`" — template literal inside double-quoted attr
static VUE_CLASS_TL_IN_DQ: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?:v-bind)?:class\s*=\s*"`([^`]*)`"#).unwrap());

/// Angular [class]="'...'" or [ngClass]="'...'" with single-quoted string
static NG_CLASS_SQ_IN_DQ: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\[(?:class|ngClass)\]\s*=\s*"'([^']*)'"#).unwrap());

/// Angular [class]="`...`" — template literal
static NG_CLASS_TL_IN_DQ: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\[(?:class|ngClass)\]\s*=\s*"`([^`]*)`"#).unwrap());

/// CSS @apply directive: `@apply <classes>;` with optional `!important`
static APPLY_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"@apply\s+([^;!]+?)(\s*!important)?\s*;"#).unwrap());

/// HTML comment: <!-- ... -->
static HTML_COMMENT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"<!--[\s\S]*?-->").unwrap());

/// CSS/JS comment blocks: /* ... */ and // ... (to end of line)
static CSS_COMMENT_BLOCK: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"/\*[\s\S]*?\*/").unwrap());

static CSS_COMMENT_LINE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"//[^\n]*").unwrap());

/// Determine the file category from extension.
#[derive(Clone, Copy)]
enum FileKind {
    Html,
    Jsx,
    Css,
    Vue,
    Angular,
}

fn file_kind(ext: &str) -> FileKind {
    match ext {
        "jsx" | "tsx" => FileKind::Jsx,
        "css" | "scss" | "less" => FileKind::Css,
        "vue" => FileKind::Vue,
        "angular" => FileKind::Angular,
        _ => FileKind::Html, // html, htm, svelte, astro, etc.
    }
}

/// For custom attribute patterns, we use a two-step approach:
///
/// 1. A generic regex matches `attrName="value"` capturing both name and value
/// 2. We then check if the attr name matches the user's pattern
///
/// This avoids greedy regex issues with patterns like `/data-.*/`
struct AttrMatcher {
    /// Compiled regex for the user's attr name pattern (None if compilation failed)
    name_re: Option<Regex>,
    /// Generic regex: captures attr name (group 1) and value (group 2) for double quotes
    generic_dq: Regex,
    /// Generic regex for single quotes
    generic_sq: Regex,
}

impl AttrMatcher {
    fn new(attr: &str) -> Self {
        let name_pattern = if attr.starts_with('/') && attr.ends_with('/') && attr.len() > 2 {
            &attr[1..attr.len() - 1]
        } else {
            &regex::escape(attr)
        };
        // Anchor with ^ and $ so the full attr name must match
        let name_re = Regex::new(&format!("^(?:{name_pattern})$")).ok();

        AttrMatcher {
            name_re,
            generic_dq: Regex::new(r#"(?:^|\s)([\w-]+)\s*=\s*"([^"]*)""#).unwrap(),
            generic_sq: Regex::new(r#"(?:^|\s)([\w-]+)\s*=\s*'([^']*)'"#).unwrap(),
        }
    }

    fn collect_matches(
        &self,
        file_text: &str,
        config: &Configuration,
        comment_ranges: &[(usize, usize)],
        replacements: &mut Vec<(usize, usize, String)>,
    ) {
        let name_re = match &self.name_re {
            Some(re) => re,
            None => return,
        };

        for re in [&self.generic_dq, &self.generic_sq] {
            for cap in re.captures_iter(file_text) {
                let attr_name = cap.get(1).unwrap().as_str();
                if !name_re.is_match(attr_name) {
                    continue;
                }
                let m = cap.get(2).unwrap();
                if is_in_comment(m.start(), comment_ranges) {
                    continue;
                }
                let class_str = m.as_str();
                if class_str.is_empty() || class_str.trim().is_empty() {
                    continue;
                }
                let sorted = sort_classes(class_str, config);
                if sorted != class_str {
                    replacements.push((m.start(), m.end(), sorted));
                }
            }
        }
    }
}

/// Build regex patterns for a user-supplied function name.
/// Matches: funcName("..."), funcName('...'), funcName`...` (tagged template)
/// Also matches chained calls: funcName.foo("..."), funcName.foo.bar`...`
/// Uses (?:^|[\s;,=(]) to ensure funcName starts at a proper position (not after `.`)
fn build_func_regexes(func: &str) -> Vec<Regex> {
    let func_pattern = if func.starts_with('/') && func.ends_with('/') && func.len() > 2 {
        func[1..func.len() - 1].to_string()
    } else {
        regex::escape(func)
    };

    // Prefix ensures func is not preceded by `.` (which would mean it's a member, not the root)
    let start = r#"(?:^|[\s;,=({\[\]:?!<>+&|])"#;
    let chain = r#"(?:\.[a-zA-Z_$][a-zA-Z0-9_$]*)*"#;

    let mut result = Vec::new();
    // funcName("...")
    if let Ok(re) = Regex::new(&format!(
        r#"{start}(?:{func_pattern}){chain}\(\s*"([^"]*)"\s*\)"#
    )) {
        result.push(re);
    }
    // funcName('...')
    if let Ok(re) = Regex::new(&format!(
        r#"{start}(?:{func_pattern}){chain}\(\s*'([^']*)'\s*\)"#
    )) {
        result.push(re);
    }
    // funcName`...` (tagged template literal)
    if let Ok(re) = Regex::new(&format!(
        r#"{start}(?:{func_pattern}){chain}`([^`]*)`"#
    )) {
        result.push(re);
    }
    result
}

/// Collect byte ranges of all comments in the text.
fn collect_comment_ranges(file_text: &str, kind: FileKind) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();

    // HTML comments (for HTML/Vue/Svelte/Astro/Angular/JSX)
    if !matches!(kind, FileKind::Css) {
        for m in HTML_COMMENT.find_iter(file_text) {
            ranges.push((m.start(), m.end()));
        }
    }

    // CSS block comments (for CSS and also in Vue/Svelte <style> blocks, JSX)
    for m in CSS_COMMENT_BLOCK.find_iter(file_text) {
        ranges.push((m.start(), m.end()));
    }

    // Line comments (for JSX/CSS)
    if matches!(kind, FileKind::Jsx | FileKind::Css) {
        for m in CSS_COMMENT_LINE.find_iter(file_text) {
            ranges.push((m.start(), m.end()));
        }
    }

    ranges.sort_by_key(|r| r.0);
    ranges
}

/// Check if a byte position falls inside any comment range.
fn is_in_comment(pos: usize, comment_ranges: &[(usize, usize)]) -> bool {
    comment_ranges
        .iter()
        .any(|&(start, end)| pos >= start && pos < end)
}

pub fn format_text(
    file_path: &Path,
    file_text: &str,
    config: &Configuration,
) -> Result<Option<String>> {
    format_text_with_range(file_path, file_text, None, config)
}

pub fn format_text_with_range(
    file_path: &Path,
    file_text: &str,
    range: FormatRange,
    config: &Configuration,
) -> Result<Option<String>> {
    if range.is_some() {
        return Ok(None);
    }

    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let kind = file_kind(ext);

    // Collect comment ranges to skip matches inside comments
    let comment_ranges = collect_comment_ranges(file_text, kind);

    // Build dynamic matchers
    let mut owned_regexes: Vec<Regex> = Vec::new();
    let mut attr_matchers: Vec<AttrMatcher> = Vec::new();

    if !matches!(kind, FileKind::Css) {
        for attr in &config.tailwind_attributes {
            attr_matchers.push(AttrMatcher::new(attr));
        }
        for func in &config.tailwind_functions {
            owned_regexes.extend(build_func_regexes(func));
        }
    }

    let mut replacements: Vec<(usize, usize, String)> = Vec::new();

    match kind {
        FileKind::Html => {
            collect_matches_filtered(&CLASS_DQ, file_text, config, &comment_ranges, &mut replacements);
            collect_matches_filtered(&CLASS_SQ, file_text, config, &comment_ranges, &mut replacements);
        }
        FileKind::Jsx => {
            collect_matches_filtered(&CLASSNAME_DQ, file_text, config, &comment_ranges, &mut replacements);
            collect_matches_filtered(&CLASSNAME_SQ, file_text, config, &comment_ranges, &mut replacements);
            collect_matches_filtered(&CLASS_DQ, file_text, config, &comment_ranges, &mut replacements);
            collect_matches_filtered(&CLASS_SQ, file_text, config, &comment_ranges, &mut replacements);
        }
        FileKind::Vue => {
            // Static class="..."
            collect_matches_filtered(&CLASS_DQ, file_text, config, &comment_ranges, &mut replacements);
            collect_matches_filtered(&CLASS_SQ, file_text, config, &comment_ranges, &mut replacements);
            // Dynamic :class="'...'" and v-bind:class="'...'"
            collect_matches_filtered(&VUE_CLASS_SQ_IN_DQ, file_text, config, &comment_ranges, &mut replacements);
            collect_matches_filtered(&VUE_CLASS_TL_IN_DQ, file_text, config, &comment_ranges, &mut replacements);
        }
        FileKind::Angular => {
            collect_matches_filtered(&CLASS_DQ, file_text, config, &comment_ranges, &mut replacements);
            collect_matches_filtered(&CLASS_SQ, file_text, config, &comment_ranges, &mut replacements);
            collect_matches_filtered(&NG_CLASS_SQ_IN_DQ, file_text, config, &comment_ranges, &mut replacements);
            collect_matches_filtered(&NG_CLASS_TL_IN_DQ, file_text, config, &comment_ranges, &mut replacements);
        }
        FileKind::Css => {
            collect_apply_matches_filtered(file_text, config, &comment_ranges, &mut replacements);
        }
    }

    // Custom attribute matchers
    for matcher in &attr_matchers {
        matcher.collect_matches(file_text, config, &comment_ranges, &mut replacements);
    }

    // Custom function regexes
    for re in &owned_regexes {
        collect_matches_filtered(re, file_text, config, &comment_ranges, &mut replacements);
    }

    if replacements.is_empty() {
        return Ok(None);
    }

    replacements.sort_by_key(|r| r.0);
    replacements.dedup_by(|b, a| b.0 < a.1);

    let mut result = file_text.to_string();
    for (start, end, new_text) in replacements.into_iter().rev() {
        result.replace_range(start..end, &new_text);
    }

    Ok(Some(result))
}

/// Collect class-sorting replacements, skipping matches inside comments.
fn collect_matches_filtered(
    re: &Regex,
    file_text: &str,
    config: &Configuration,
    comment_ranges: &[(usize, usize)],
    replacements: &mut Vec<(usize, usize, String)>,
) {
    for cap in re.captures_iter(file_text) {
        let m = cap.get(1).unwrap();
        if is_in_comment(m.start(), comment_ranges) {
            continue;
        }
        let class_str = m.as_str();
        if class_str.is_empty() {
            continue;
        }
        // Skip whitespace-only values
        if class_str.trim().is_empty() {
            continue;
        }
        let sorted = sort_classes(class_str, config);
        if sorted != class_str {
            replacements.push((m.start(), m.end(), sorted));
        }
    }
}

/// Collect @apply replacements, skipping matches inside comments.
fn collect_apply_matches_filtered(
    file_text: &str,
    config: &Configuration,
    comment_ranges: &[(usize, usize)],
    replacements: &mut Vec<(usize, usize, String)>,
) {
    for cap in APPLY_RE.captures_iter(file_text) {
        let m = cap.get(1).unwrap();
        if is_in_comment(m.start(), comment_ranges) {
            continue;
        }
        let class_str = m.as_str().trim();
        if class_str.is_empty() {
            continue;
        }
        let sorted = sort_classes(class_str, config);
        if sorted != m.as_str().trim() {
            replacements.push((m.start(), m.end(), sorted));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{format_text, format_text_with_range};
    use crate::configuration::Configuration;

    fn default_config() -> Configuration {
        Configuration {
            remove_duplicates: true,
            collapse_whitespace: true,
            tailwind_attributes: vec![],
            tailwind_functions: vec![],
        }
    }

    // === HTML basic tests ===

    #[test]
    fn sorts_html_class_attribute() {
        let input = r#"<div class="p-4 flex absolute">hello</div>"#;
        let result = format_text(Path::new("test.html"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<div class="absolute flex p-4">hello</div>"#.to_string())
        );
    }

    #[test]
    fn sorts_html_single_quote_class() {
        let input = "<div class='p-4 flex absolute'>hello</div>";
        let result = format_text(Path::new("test.html"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some("<div class='absolute flex p-4'>hello</div>".to_string())
        );
    }

    #[test]
    fn returns_none_when_already_sorted() {
        let input = r#"<div class="absolute flex p-4">hello</div>"#;
        let result = format_text(Path::new("test.html"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn handles_multiple_class_attributes() {
        let input = r#"<div class="p-4 flex"><span class="m-2 block"></span></div>"#;
        let result = format_text(Path::new("test.html"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<div class="flex p-4"><span class="m-2 block"></span></div>"#.to_string())
        );
    }

    #[test]
    fn handles_empty_class() {
        let input = r#"<div class="">hello</div>"#;
        let result = format_text(Path::new("test.html"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn returns_none_for_range_requests() {
        let input = r#"<div class="p-4 flex">hello</div>"#;
        let result = format_text_with_range(
            Path::new("test.html"),
            input,
            Some(0..input.len()),
            &default_config(),
        )
        .unwrap();
        assert_eq!(result, None);
    }

    // === HTML comment skipping ===

    #[test]
    fn skips_class_inside_html_comment() {
        let input = r#"<!-- <div class="p-4 flex absolute">hello</div> -->"#;
        let result = format_text(Path::new("test.html"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn sorts_class_outside_comment_only() {
        let input = r#"<!-- <div class="p-4 flex"> --> <div class="p-4 flex">hello</div>"#;
        let result = format_text(Path::new("test.html"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<!-- <div class="p-4 flex"> --> <div class="flex p-4">hello</div>"#.to_string())
        );
    }

    // === HTML template interpolation ===

    #[test]
    fn skips_class_with_mustache_interpolation() {
        let input = r#"<div class="p-4 flex {{ someVar }}">hello</div>"#;
        let result = format_text(Path::new("test.html"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    // === HTML non-class attribute not matched ===

    #[test]
    fn non_class_attribute_not_sorted() {
        let input = r#"<div not-class="p-4 flex absolute">hello</div>"#;
        let result = format_text(Path::new("test.html"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    // === JSX tests ===

    #[test]
    fn sorts_jsx_classname_attribute() {
        let input = r#"<div className="p-4 flex absolute">hello</div>"#;
        let result = format_text(Path::new("test.tsx"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<div className="absolute flex p-4">hello</div>"#.to_string())
        );
    }

    #[test]
    fn skips_jsx_expression_classname() {
        let input = r#"<div className={styles.foo}>hello</div>"#;
        let result = format_text(Path::new("test.tsx"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn html_does_not_match_classname() {
        let input = r#"<div class="p-4 flex" className="m-2 block">hello</div>"#;
        let result = format_text(Path::new("test.html"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<div class="flex p-4" className="m-2 block">hello</div>"#.to_string())
        );
    }

    #[test]
    fn jsx_file_matches_both_class_and_classname() {
        let input = r#"<div class="p-4 flex" className="m-2 block">hello</div>"#;
        let result = format_text(Path::new("test.jsx"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<div class="flex p-4" className="m-2 block">hello</div>"#.to_string())
        );
    }

    #[test]
    fn skips_class_inside_js_comment() {
        let input = "// <div className=\"p-4 flex absolute\" />\n<div className=\"p-4 flex absolute\" />";
        let result = format_text(Path::new("test.tsx"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some("// <div className=\"p-4 flex absolute\" />\n<div className=\"absolute flex p-4\" />".to_string())
        );
    }

    #[test]
    fn skips_class_inside_block_comment() {
        let input = "/* <div className=\"p-4 flex absolute\" /> */";
        let result = format_text(Path::new("test.tsx"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    // === Vue tests ===

    #[test]
    fn sorts_vue_template_class() {
        let input = r#"<template><div class="p-4 flex absolute">hello</div></template>"#;
        let result = format_text(Path::new("App.vue"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<template><div class="absolute flex p-4">hello</div></template>"#.to_string())
        );
    }

    #[test]
    fn sorts_vue_dynamic_class_string() {
        let input = r#"<div :class="'p-4 flex absolute'"></div>"#;
        let result = format_text(Path::new("App.vue"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<div :class="'absolute flex p-4'"></div>"#.to_string())
        );
    }

    #[test]
    fn sorts_vue_vbind_class_string() {
        let input = r#"<div v-bind:class="'p-4 flex absolute'"></div>"#;
        let result = format_text(Path::new("App.vue"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<div v-bind:class="'absolute flex p-4'"></div>"#.to_string())
        );
    }

    #[test]
    fn sorts_vue_dynamic_class_template_literal() {
        let input = r#"<div :class="`p-4 flex absolute`"></div>"#;
        let result = format_text(Path::new("App.vue"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<div :class="`absolute flex p-4`"></div>"#.to_string())
        );
    }

    #[test]
    fn skips_vue_comment() {
        let input = r#"<!-- <div :class="'p-4 flex absolute'"></div> -->"#;
        let result = format_text(Path::new("App.vue"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    // === Svelte/Astro tests ===

    #[test]
    fn sorts_svelte_class() {
        let input = r#"<div class="p-4 flex absolute">hello</div>"#;
        let result = format_text(Path::new("App.svelte"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<div class="absolute flex p-4">hello</div>"#.to_string())
        );
    }

    #[test]
    fn sorts_astro_class() {
        let input = r#"<div class="p-4 flex absolute">hello</div>"#;
        let result = format_text(Path::new("page.astro"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(r#"<div class="absolute flex p-4">hello</div>"#.to_string())
        );
    }

    // === CSS @apply tests ===

    #[test]
    fn sorts_css_apply_directive() {
        let input = ".btn {\n  @apply p-4 flex absolute;\n}";
        let result = format_text(Path::new("styles.css"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(".btn {\n  @apply absolute flex p-4;\n}".to_string())
        );
    }

    #[test]
    fn sorts_css_apply_with_important() {
        let input = ".btn {\n  @apply p-4 flex absolute !important;\n}";
        let result = format_text(Path::new("styles.css"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(".btn {\n  @apply absolute flex p-4 !important;\n}".to_string())
        );
    }

    #[test]
    fn sorts_scss_apply() {
        let input = ".card {\n  @apply p-4 flex;\n}";
        let result = format_text(Path::new("styles.scss"), input, &default_config()).unwrap();
        assert_eq!(
            result,
            Some(".card {\n  @apply flex p-4;\n}".to_string())
        );
    }

    #[test]
    fn css_apply_already_sorted() {
        let input = ".btn {\n  @apply absolute flex p-4;\n}";
        let result = format_text(Path::new("styles.css"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn skips_apply_inside_css_comment() {
        let input = "/* @apply p-4 flex absolute; */";
        let result = format_text(Path::new("styles.css"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn not_apply_not_matched() {
        let input = "@not-apply p-4 flex absolute;";
        let result = format_text(Path::new("styles.css"), input, &default_config()).unwrap();
        assert_eq!(result, None);
    }

    // === tailwindAttributes tests ===

    #[test]
    fn sorts_custom_attribute() {
        let config = Configuration {
            tailwind_attributes: vec!["myClass".to_string()],
            ..default_config()
        };
        let input = r#"<div myClass="p-4 flex absolute">hello</div>"#;
        let result = format_text(Path::new("test.html"), input, &config).unwrap();
        assert_eq!(
            result,
            Some(r#"<div myClass="absolute flex p-4">hello</div>"#.to_string())
        );
    }

    #[test]
    fn sorts_regex_attribute_pattern() {
        let config = Configuration {
            tailwind_attributes: vec!["/data-tw-.*/".to_string()],
            ..default_config()
        };
        let input = r#"<div data-tw-classes="p-4 flex absolute">hello</div>"#;
        let result = format_text(Path::new("test.html"), input, &config).unwrap();
        assert_eq!(
            result,
            Some(r#"<div data-tw-classes="absolute flex p-4">hello</div>"#.to_string())
        );
    }

    #[test]
    fn custom_attributes_not_applied_to_css() {
        let config = Configuration {
            tailwind_attributes: vec!["myClass".to_string()],
            ..default_config()
        };
        let input = r#"myClass="p-4 flex absolute""#;
        let result = format_text(Path::new("styles.css"), input, &config).unwrap();
        assert_eq!(result, None);
    }

    // === tailwindFunctions tests ===

    #[test]
    fn sorts_function_string_arg() {
        let config = Configuration {
            tailwind_functions: vec!["clsx".to_string()],
            ..default_config()
        };
        let input = r#"const cls = clsx("p-4 flex absolute");"#;
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some(r#"const cls = clsx("absolute flex p-4");"#.to_string())
        );
    }

    #[test]
    fn sorts_function_single_quote_arg() {
        let config = Configuration {
            tailwind_functions: vec!["cn".to_string()],
            ..default_config()
        };
        let input = "const cls = cn('p-4 flex absolute');";
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some("const cls = cn('absolute flex p-4');".to_string())
        );
    }

    #[test]
    fn sorts_tagged_template_literal() {
        let config = Configuration {
            tailwind_functions: vec!["tw".to_string()],
            ..default_config()
        };
        let input = "const cls = tw`p-4 flex absolute`;";
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some("const cls = tw`absolute flex p-4`;".to_string())
        );
    }

    #[test]
    fn sorts_chained_function_call() {
        let config = Configuration {
            tailwind_functions: vec!["tw".to_string()],
            ..default_config()
        };
        let input = r#"const cls = tw.div("p-4 flex absolute");"#;
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some(r#"const cls = tw.div("absolute flex p-4");"#.to_string())
        );
    }

    #[test]
    fn sorts_chained_tagged_template() {
        let config = Configuration {
            tailwind_functions: vec!["tw".to_string()],
            ..default_config()
        };
        let input = "const cls = tw.div`p-4 flex absolute`;";
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some("const cls = tw.div`absolute flex p-4`;".to_string())
        );
    }

    #[test]
    fn function_already_sorted() {
        let config = Configuration {
            tailwind_functions: vec!["clsx".to_string()],
            ..default_config()
        };
        let input = r#"const cls = clsx("absolute flex p-4");"#;
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn functions_not_applied_to_css() {
        let config = Configuration {
            tailwind_functions: vec!["clsx".to_string()],
            ..default_config()
        };
        let input = r#"clsx("p-4 flex absolute")"#;
        let result = format_text(Path::new("styles.css"), input, &config).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn unlisted_function_not_sorted() {
        let config = Configuration {
            tailwind_functions: vec!["clsx".to_string()],
            ..default_config()
        };
        let input = r#"const cls = otherFn("p-4 flex absolute");"#;
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(result, None);
    }

    // === tailwindFunctions in JSX expression contexts ===

    #[test]
    fn sorts_function_in_jsx_curly_braces() {
        let config = Configuration {
            tailwind_functions: vec!["cn".to_string()],
            ..default_config()
        };
        let input = r#"<div className={cn("p-4 flex absolute")}>"#;
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some(r#"<div className={cn("absolute flex p-4")}>"#.to_string())
        );
    }

    #[test]
    fn sorts_function_in_jsx_curly_braces_single_quote() {
        let config = Configuration {
            tailwind_functions: vec!["cn".to_string()],
            ..default_config()
        };
        let input = "<div className={cn('p-4 flex absolute')}>";
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some("<div className={cn('absolute flex p-4')}>".to_string())
        );
    }

    #[test]
    fn sorts_function_in_array_context() {
        let config = Configuration {
            tailwind_functions: vec!["cn".to_string()],
            ..default_config()
        };
        let input = r#"const cls = [cn("p-4 flex absolute")];"#;
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some(r#"const cls = [cn("absolute flex p-4")];"#.to_string())
        );
    }

    #[test]
    fn sorts_function_in_ternary_context() {
        let config = Configuration {
            tailwind_functions: vec!["cn".to_string()],
            ..default_config()
        };
        let input = r#"const cls = isActive ? cn("p-4 flex absolute") : "";"#;
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some(r#"const cls = isActive ? cn("absolute flex p-4") : "";"#.to_string())
        );
    }

    #[test]
    fn sorts_function_after_logical_operator() {
        let config = Configuration {
            tailwind_functions: vec!["cn".to_string()],
            ..default_config()
        };
        let input = r#"const cls = isActive && cn("p-4 flex absolute");"#;
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some(r#"const cls = isActive && cn("absolute flex p-4");"#.to_string())
        );
    }

    #[test]
    fn sorts_function_in_jsx_return() {
        let config = Configuration {
            tailwind_functions: vec!["cn".to_string(), "clsx".to_string()],
            ..default_config()
        };
        let input = r#"<p className={cn("px-4 flex items-center mt-2 bg-white text-sm font-medium")}>"#;
        let result = format_text(Path::new("test.tsx"), input, &config).unwrap();
        assert_eq!(
            result,
            Some(
                r#"<p className={cn("mt-2 flex items-center bg-white px-4 font-medium text-sm")}>"#
                    .to_string()
            )
        );
    }
}
