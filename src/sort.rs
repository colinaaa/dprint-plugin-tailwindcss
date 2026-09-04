use std::collections::HashSet;

use crate::configuration::Configuration;
use crate::order::get_class_order;

/// Extract the base utility from a class with variant prefixes.
/// e.g., "hover:sm:flex" -> "flex", "dark:bg-red-500" -> "bg-red-500"
fn extract_base_utility(class: &str) -> &str {
    let bytes = class.as_bytes();
    let mut last_colon = None;
    let mut bracket_depth = 0u32;

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'[' => bracket_depth += 1,
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b':' if bracket_depth == 0 => last_colon = Some(i),
            _ => {}
        }
    }

    match last_colon {
        Some(pos) => &class[pos + 1..],
        None => class,
    }
}

/// Check if a class has variant prefixes (e.g., "hover:flex" has variants, "flex" does not).
fn has_variants(class: &str) -> bool {
    let bytes = class.as_bytes();
    let mut bracket_depth = 0u32;
    for &b in bytes {
        match b {
            b'[' => bracket_depth += 1,
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b':' if bracket_depth == 0 => return true,
            _ => {}
        }
    }
    false
}

/// Strip the `!` important modifier prefix from a class for order lookup.
/// e.g., "!p-4" -> "p-4", "!hover:flex" -> "hover:flex"
fn strip_important(class: &str) -> &str {
    class.strip_prefix('!').unwrap_or(class)
}

/// Check if a class token is an ellipsis/spread marker.
fn is_ellipsis(class: &str) -> bool {
    class == "..." || class == "\u{2026}"
}

/// Sort a space-separated class string according to Tailwind v4 order.
///
/// If the class string contains template interpolation markers (`{{`, `{`, etc.),
/// we skip sorting to avoid mangling template expressions.
pub fn sort_classes(input: &str, config: &Configuration) -> String {
    if input.is_empty() {
        return String::new();
    }

    // Skip sorting if the value contains template interpolation (Mustache/Handlebars/Svelte)
    if contains_template_interpolation(input) {
        return input.to_string();
    }

    if config.collapse_whitespace {
        sort_classes_collapse(input, config)
    } else {
        sort_classes_preserve_whitespace(input, config)
    }
}

/// Detect template interpolation markers in a class string.
/// Matches `{{ ... }}` (Mustache/Handlebars/Angular/Vue) and `{ ... }` (Svelte).
fn contains_template_interpolation(input: &str) -> bool {
    // Mustache/Handlebars/Angular double-brace: {{ ... }}
    if input.contains("{{") {
        return true;
    }
    // Svelte expression: { ... } but not arbitrary value brackets like [...]
    // We detect this as: a `{` that is NOT inside `[...]`
    let bytes = input.as_bytes();
    let mut bracket_depth = 0u32;
    for &b in bytes {
        match b {
            b'[' => bracket_depth += 1,
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'{' if bracket_depth == 0 => return true,
            _ => {}
        }
    }
    false
}

fn sort_classes_collapse(input: &str, config: &Configuration) -> String {
    let classes: Vec<&str> = input.split_whitespace().collect();
    if classes.is_empty() {
        return String::new();
    }

    // Separate ellipsis markers from regular classes
    let mut regular: Vec<(u32, bool, usize, &str)> = Vec::new();
    let mut ellipses: Vec<&str> = Vec::new();

    for (i, &c) in classes.iter().enumerate() {
        if is_ellipsis(c) {
            ellipses.push(c);
        } else {
            let stripped = strip_important(c);
            let base = extract_base_utility(stripped);
            let order = get_class_order(base);
            let has_var = has_variants(stripped);
            regular.push((order, has_var, i, c));
        }
    }

    // Sort by (order, has_variants, original_index)
    // This ensures base utilities sort before their variant counterparts
    regular.sort_by_key(|&(order, has_var, idx, _)| (order, has_var, idx));

    // Dedup if enabled: only deduplicate KNOWN classes (order > 0).
    // Unknown classes (order == 0) are never deduped.
    if config.remove_duplicates {
        let mut seen = HashSet::new();
        regular.retain(|&(order, _, _, class)| {
            if order == 0 {
                true // always keep unknown classes
            } else {
                seen.insert(class)
            }
        });
    }

    let mut result: Vec<&str> = regular.iter().map(|&(_, _, _, c)| c).collect();
    // Ellipsis markers always go to the end
    result.extend(ellipses);
    result.join(" ")
}

fn sort_classes_preserve_whitespace(input: &str, config: &Configuration) -> String {
    let mut classes: Vec<&str> = Vec::new();
    let mut whitespaces: Vec<&str> = Vec::new();

    let bytes = input.as_bytes();
    let mut i = 0;
    let len = bytes.len();

    // Leading whitespace
    let ws_start = i;
    while i < len && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    whitespaces.push(&input[ws_start..i]);

    while i < len {
        let class_start = i;
        while i < len && !bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        classes.push(&input[class_start..i]);

        let ws_start = i;
        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        whitespaces.push(&input[ws_start..i]);
    }

    if classes.is_empty() {
        return input.to_string();
    }

    let mut indexed: Vec<(u32, bool, usize, &str)> = classes
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let stripped = strip_important(c);
            let base = extract_base_utility(stripped);
            let order = if is_ellipsis(c) { u32::MAX } else { get_class_order(base) };
            let has_var = has_variants(stripped);
            (order, has_var, i, c)
        })
        .collect();

    indexed.sort_by_key(|&(order, has_var, idx, _)| (order, has_var, idx));

    if config.remove_duplicates {
        let mut seen = HashSet::new();
        indexed.retain(|&(order, _, _, class)| {
            if order == 0 {
                true
            } else {
                seen.insert(class)
            }
        });
    }

    let mut result = String::with_capacity(input.len());
    result.push_str(whitespaces[0]);

    for (j, &(_, _, _, class)) in indexed.iter().enumerate() {
        result.push_str(class);
        let ws_idx = j + 1;
        if ws_idx < whitespaces.len() {
            result.push_str(whitespaces[ws_idx]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::sort_classes;
    use crate::configuration::Configuration;

    fn default_config() -> Configuration {
        Configuration {
            remove_duplicates: true,
            collapse_whitespace: true,
            tailwind_attributes: vec![],
            tailwind_functions: vec![],
        }
    }

    #[test]
    fn sorts_classes_by_tailwind_order() {
        let result = sort_classes("p-4 flex absolute", &default_config());
        assert_eq!(result, "absolute flex p-4");
    }

    #[test]
    fn already_sorted_returns_same() {
        let result = sort_classes("absolute flex p-4", &default_config());
        assert_eq!(result, "absolute flex p-4");
    }

    #[test]
    fn removes_duplicates() {
        let result = sort_classes("flex p-4 flex", &default_config());
        assert_eq!(result, "flex p-4");
    }

    #[test]
    fn preserves_duplicates_when_disabled() {
        let config = Configuration {
            remove_duplicates: false,
            collapse_whitespace: true,
            tailwind_attributes: vec![],
            tailwind_functions: vec![],
        };
        let result = sort_classes("flex p-4 flex", &config);
        assert_eq!(result, "flex flex p-4");
    }

    #[test]
    fn collapses_whitespace() {
        let result = sort_classes("  flex   p-4  absolute  ", &default_config());
        assert_eq!(result, "absolute flex p-4");
    }

    #[test]
    fn preserves_whitespace_when_disabled() {
        let config = Configuration {
            remove_duplicates: true,
            collapse_whitespace: false,
            tailwind_attributes: vec![],
            tailwind_functions: vec![],
        };
        let result = sort_classes("  p-4   flex  ", &config);
        assert_eq!(result, "  flex   p-4  ");
    }

    #[test]
    fn unknown_classes_sort_to_beginning() {
        let result = sort_classes("flex potato p-4", &default_config());
        assert_eq!(result, "potato flex p-4");
    }

    #[test]
    fn unknown_classes_preserve_relative_order() {
        let result = sort_classes("flex zebra alpha p-4", &default_config());
        assert_eq!(result, "zebra alpha flex p-4");
    }

    #[test]
    fn empty_string() {
        let result = sort_classes("", &default_config());
        assert_eq!(result, "");
    }

    #[test]
    fn single_class() {
        let result = sort_classes("flex", &default_config());
        assert_eq!(result, "flex");
    }

    #[test]
    fn variant_classes_sort_by_base_utility() {
        let result = sort_classes("hover:p-4 hover:flex sm:absolute", &default_config());
        assert_eq!(result, "sm:absolute hover:flex hover:p-4");
    }

    #[test]
    fn negative_values_handled() {
        let result = sort_classes("p-4 -m-2 flex", &default_config());
        assert_eq!(result, "-m-2 flex p-4");
    }

    #[test]
    fn dedup_only_removes_exact_duplicates() {
        let result = sort_classes("flex hover:flex p-4", &default_config());
        assert_eq!(result, "flex hover:flex p-4");
    }

    // === Ellipsis marker tests ===

    #[test]
    fn ellipsis_sorts_to_end() {
        let result = sort_classes("... p-4 flex", &default_config());
        assert_eq!(result, "flex p-4 ...");
    }

    #[test]
    fn unicode_ellipsis_sorts_to_end() {
        let result = sort_classes("\u{2026} p-4 flex", &default_config());
        assert_eq!(result, "flex p-4 \u{2026}");
    }

    #[test]
    fn ellipsis_in_middle() {
        let result = sort_classes("p-4 ... flex", &default_config());
        assert_eq!(result, "flex p-4 ...");
    }

    // === Template interpolation tests ===

    #[test]
    fn skips_mustache_interpolation() {
        let input = "p-4 flex {{ someVar }}";
        let result = sort_classes(input, &default_config());
        assert_eq!(result, input); // unchanged
    }

    #[test]
    fn skips_svelte_expression() {
        let input = "p-4 flex {someVar}";
        let result = sort_classes(input, &default_config());
        assert_eq!(result, input); // unchanged
    }

    #[test]
    fn does_not_skip_arbitrary_value_brackets() {
        // [&>div]:flex uses brackets but not curly braces -- should still sort
        let result = sort_classes("p-4 [&>div]:flex", &default_config());
        assert_eq!(result, "[&>div]:flex p-4");
    }

    // === Additional edge cases from prettier-plugin-tailwindcss ===

    #[test]
    fn unknown_duplicates_preserved_known_deduped() {
        // Unknown class dups kept, known dups removed (matches prettier behavior)
        let result = sort_classes(
            "idonotexist flex p-4 idonotexist p-4 idonotexist",
            &default_config(),
        );
        assert_eq!(
            result,
            "idonotexist idonotexist idonotexist flex p-4"
        );
    }

    #[test]
    fn only_unknown_classes_preserve_order() {
        let result = sort_classes("potato zebra alpha", &default_config());
        assert_eq!(result, "potato zebra alpha");
    }

    #[test]
    fn stacked_variants() {
        let result = sort_classes(
            "dark:hover:bg-red-500 sm:hover:flex hover:p-4",
            &default_config(),
        );
        assert_eq!(
            result,
            "sm:hover:flex dark:hover:bg-red-500 hover:p-4"
        );
    }

    #[test]
    fn important_modifier() {
        // ! prefix is part of the class, not a variant
        let result = sort_classes("!p-4 flex !m-2", &default_config());
        assert_eq!(result, "!m-2 flex !p-4");
    }

    #[test]
    fn arbitrary_value_with_url() {
        let result = sort_classes(
            "p-4 bg-[url('https://example.com')] flex",
            &default_config(),
        );
        assert_eq!(
            result,
            "flex bg-[url('https://example.com')] p-4"
        );
    }

    #[test]
    fn arbitrary_value_with_calc() {
        let result = sort_classes("p-4 w-[calc(100%-2rem)] flex", &default_config());
        assert_eq!(result, "flex w-[calc(100%-2rem)] p-4");
    }

    #[test]
    fn arbitrary_value_with_colon_in_brackets() {
        // The colon inside [] should NOT be treated as a variant separator
        let result = sort_classes(
            "p-4 [&>div:hover]:flex absolute",
            &default_config(),
        );
        assert_eq!(result, "absolute [&>div:hover]:flex p-4");
    }

    #[test]
    fn many_classes_long_chain() {
        let result = sort_classes(
            "shadow-lg rounded-lg bg-blue-500 text-white font-bold p-4 m-2 flex items-center",
            &default_config(),
        );
        assert_eq!(
            result,
            "m-2 flex items-center rounded-lg bg-blue-500 p-4 font-bold text-white shadow-lg"
        );
    }

    #[test]
    fn all_same_class_deduped_to_one() {
        let result = sort_classes("flex flex flex flex", &default_config());
        assert_eq!(result, "flex");
    }

    #[test]
    fn whitespace_only_returns_empty() {
        let result = sort_classes("   ", &default_config());
        assert_eq!(result, "");
    }

    #[test]
    fn tabs_and_newlines_collapsed() {
        let result = sort_classes("p-4\tflex\nabsolute", &default_config());
        assert_eq!(result, "absolute flex p-4");
    }

    #[test]
    fn group_and_peer_are_unknown_sort_to_beginning() {
        // group and peer are not in our static order map (they're component-level),
        // so they sort as unknowns to the front
        let result = sort_classes("group peer p-4 flex", &default_config());
        assert_eq!(result, "group peer flex p-4");
    }

    #[test]
    fn responsive_variant_after_base() {
        // sm:p-0 should come after p-0
        let result = sort_classes("sm:p-0 p-0", &default_config());
        assert_eq!(result, "p-0 sm:p-0");
    }

    #[test]
    fn multiple_responsive_variants_stable() {
        // Variants sort after base but preserve relative order among themselves
        let result = sort_classes("lg:flex md:flex sm:flex flex", &default_config());
        assert_eq!(result, "flex lg:flex md:flex sm:flex");
    }

    #[test]
    fn preserve_whitespace_with_dedup() {
        let config = Configuration {
            remove_duplicates: true,
            collapse_whitespace: false,
            tailwind_attributes: vec![],
            tailwind_functions: vec![],
        };
        let result = sort_classes("  flex  p-4  flex  ", &config);
        // After dedup (one flex removed), 3 ws slots for 2 classes
        assert_eq!(result, "  flex  p-4  ");
    }

    #[test]
    fn preserve_whitespace_no_dedup() {
        let config = Configuration {
            remove_duplicates: false,
            collapse_whitespace: false,
            tailwind_attributes: vec![],
            tailwind_functions: vec![],
        };
        let result = sort_classes("  p-4  flex  ", &config);
        assert_eq!(result, "  flex  p-4  ");
    }

    #[test]
    fn both_disabled() {
        let config = Configuration {
            remove_duplicates: false,
            collapse_whitespace: false,
            tailwind_attributes: vec![],
            tailwind_functions: vec![],
        };
        let result = sort_classes("  p-4  flex  p-4  ", &config);
        assert_eq!(result, "  flex  p-4  p-4  ");
    }
}
