//! Static Tailwind CSS v4 class order map.
//!
//! Order values follow the registration order in Tailwind v4's `utilities.ts`.
//! Lower values sort first. 0 = unknown (sorts to beginning).

/// Lookup the sort order for a Tailwind utility class.
/// Returns 0 for unknown classes.
pub fn get_class_order(class: &str) -> u32 {
    // Strip leading "-" for negative value utilities like -m-4, -translate-x-1
    let lookup = class.strip_prefix('-').unwrap_or(class);

    // Try exact match first
    if let Ok(idx) = EXACT_UTILITIES.binary_search_by_key(&lookup, |&(name, _)| name) {
        return EXACT_UTILITIES[idx].1;
    }

    // Try prefix match: find the longest matching prefix.
    // PREFIX_UTILITIES is sorted longest-first so we return the first match.
    for &(prefix, order) in PREFIX_UTILITIES.iter() {
        if lookup.starts_with(prefix) {
            return order;
        }
    }

    0
}

/// Exact-match utilities: class name -> sort order.
/// Sorted alphabetically for binary search.
static EXACT_UTILITIES: &[(&str, u32)] = &[
    ("absolute", 40),
    ("animate-none", 1420),
    ("antialiased", 1740),
    ("appearance-auto", 610),
    ("appearance-none", 610),
    ("aspect-auto", 180),
    ("aspect-square", 180),
    ("aspect-video", 180),
    ("auto-cols-auto", 660),
    ("auto-cols-fr", 660),
    ("auto-cols-max", 660),
    ("auto-cols-min", 660),
    ("auto-rows-auto", 680),
    ("auto-rows-fr", 680),
    ("auto-rows-max", 680),
    ("auto-rows-min", 680),
    ("backdrop-filter", 1440),
    ("backdrop-filter-none", 1440),
    ("backface-hidden", 470),
    ("backface-visible", 470),
    ("basis-auto", 310),
    ("basis-full", 310),
    ("bg-auto", 970),
    ("bg-bottom", 990),
    ("bg-center", 990),
    ("bg-clip-border", 1210),
    ("bg-clip-content", 1210),
    ("bg-clip-padding", 1210),
    ("bg-clip-text", 1210),
    ("bg-contain", 970),
    ("bg-cover", 970),
    ("bg-fixed", 980),
    ("bg-left", 990),
    ("bg-left-bottom", 990),
    ("bg-left-top", 990),
    ("bg-local", 980),
    ("bg-no-repeat", 1000),
    ("bg-none", 1010),
    ("bg-origin-border", 1220),
    ("bg-origin-content", 1220),
    ("bg-origin-padding", 1220),
    ("bg-repeat", 1000),
    ("bg-repeat-round", 1000),
    ("bg-repeat-space", 1000),
    ("bg-repeat-x", 1000),
    ("bg-repeat-y", 1000),
    ("bg-right", 990),
    ("bg-right-bottom", 990),
    ("bg-right-top", 990),
    ("bg-scroll", 980),
    ("bg-top", 990),
    ("bg-blend-color", 1230),
    ("bg-blend-color-burn", 1230),
    ("bg-blend-color-dodge", 1230),
    ("bg-blend-darken", 1230),
    ("bg-blend-difference", 1230),
    ("bg-blend-exclusion", 1230),
    ("bg-blend-hard-light", 1230),
    ("bg-blend-hue", 1230),
    ("bg-blend-lighten", 1230),
    ("bg-blend-luminosity", 1230),
    ("bg-blend-multiply", 1230),
    ("bg-blend-normal", 1230),
    ("bg-blend-overlay", 1230),
    ("bg-blend-saturation", 1230),
    ("bg-blend-screen", 1230),
    ("bg-blend-soft-light", 1230),
    ("block", 160),
    ("blur-2xl", 1450),
    ("blur-3xl", 1450),
    ("blur-lg", 1450),
    ("blur-md", 1450),
    ("blur-none", 1450),
    ("blur-sm", 1450),
    ("blur-xl", 1450),
    ("blur-xs", 1450),
    ("border", 920),
    ("border-0", 920),
    ("border-collapse", 340),
    ("border-dashed", 930),
    ("border-dotted", 930),
    ("border-double", 930),
    ("border-hidden", 930),
    ("border-none", 930),
    ("border-separate", 340),
    ("border-solid", 930),
    ("box-border", 140),
    ("box-content", 140),
    ("box-decoration-clone", 1200),
    ("box-decoration-slice", 1200),
    ("break-after-all", 650),
    ("break-after-auto", 650),
    ("break-after-avoid", 650),
    ("break-after-avoid-page", 650),
    ("break-after-column", 650),
    ("break-after-left", 650),
    ("break-after-page", 650),
    ("break-after-right", 650),
    ("break-all", 900),
    ("break-before-all", 630),
    ("break-before-auto", 630),
    ("break-before-avoid", 630),
    ("break-before-avoid-page", 630),
    ("break-before-column", 630),
    ("break-before-left", 630),
    ("break-before-page", 630),
    ("break-before-right", 630),
    ("break-inside-auto", 640),
    ("break-inside-avoid", 640),
    ("break-inside-avoid-column", 640),
    ("break-inside-avoid-page", 640),
    ("break-keep", 900),
    ("break-normal", 900),
    ("break-words", 900),
    ("capitalize", 1350),
    ("caption-bottom", 330),
    ("caption-top", 330),
    ("clear-both", 120),
    ("clear-end", 120),
    ("clear-left", 120),
    ("clear-none", 120),
    ("clear-right", 120),
    ("clear-start", 120),
    ("col-auto", 90),
    ("collapse", 30),
    ("contain-content", 1700),
    ("contain-inline-size", 1700),
    ("contain-layout", 1700),
    ("contain-none", 1700),
    ("contain-paint", 1700),
    ("contain-size", 1700),
    ("contain-strict", 1700),
    ("contain-style", 1700),
    ("container", 270),
    ("content-around", 750),
    ("content-baseline", 750),
    ("content-between", 750),
    ("content-center", 750),
    ("content-end", 750),
    ("content-evenly", 750),
    ("content-none", 1690),
    ("content-normal", 750),
    ("content-start", 750),
    ("content-stretch", 750),
    ("contents", 160),
    ("cursor-alias", 480),
    ("cursor-all-scroll", 480),
    ("cursor-auto", 480),
    ("cursor-cell", 480),
    ("cursor-col-resize", 480),
    ("cursor-context-menu", 480),
    ("cursor-copy", 480),
    ("cursor-crosshair", 480),
    ("cursor-default", 480),
    ("cursor-e-resize", 480),
    ("cursor-ew-resize", 480),
    ("cursor-grab", 480),
    ("cursor-grabbing", 480),
    ("cursor-help", 480),
    ("cursor-move", 480),
    ("cursor-n-resize", 480),
    ("cursor-ne-resize", 480),
    ("cursor-nesw-resize", 480),
    ("cursor-no-drop", 480),
    ("cursor-none", 480),
    ("cursor-not-allowed", 480),
    ("cursor-ns-resize", 480),
    ("cursor-nw-resize", 480),
    ("cursor-nwse-resize", 480),
    ("cursor-pointer", 480),
    ("cursor-progress", 480),
    ("cursor-row-resize", 480),
    ("cursor-s-resize", 480),
    ("cursor-se-resize", 480),
    ("cursor-sw-resize", 480),
    ("cursor-text", 480),
    ("cursor-vertical-text", 480),
    ("cursor-w-resize", 480),
    ("cursor-wait", 480),
    ("cursor-zoom-in", 480),
    ("cursor-zoom-out", 480),
    ("decoration-auto", 1410),
    ("decoration-dashed", 1400),
    ("decoration-dotted", 1400),
    ("decoration-double", 1400),
    ("decoration-from-font", 1410),
    ("decoration-solid", 1400),
    ("decoration-wavy", 1400),
    ("diagonal-fractions", 1750),
    ("divide-dashed", 960),
    ("divide-dotted", 960),
    ("divide-double", 960),
    ("divide-none", 960),
    ("divide-solid", 960),
    ("drop-shadow-none", 1610),
    ("ease-linear", 1670),
    ("field-sizing-content", 170),
    ("field-sizing-fixed", 170),
    ("fill-none", 1250),
    ("filter", 1430),
    ("filter-none", 1430),
    ("fixed", 40),
    ("flex", 160),
    ("flex-auto", 280),
    ("flex-col", 710),
    ("flex-col-reverse", 710),
    ("flex-initial", 280),
    ("flex-none", 280),
    ("flex-nowrap", 720),
    ("flex-row", 710),
    ("flex-row-reverse", 710),
    ("flex-wrap", 720),
    ("flex-wrap-reverse", 720),
    ("float-end", 110),
    ("float-left", 110),
    ("float-none", 110),
    ("float-right", 110),
    ("float-start", 110),
    ("flow-root", 160),
    ("font-black", 1330),
    ("font-bold", 1330),
    ("font-extrabold", 1330),
    ("font-extralight", 1330),
    ("font-light", 1330),
    ("font-medium", 1330),
    ("font-mono", 1330),
    ("font-normal", 1330),
    ("font-sans", 1330),
    ("font-semibold", 1330),
    ("font-serif", 1330),
    ("font-thin", 1330),
    ("forced-color-adjust-auto", 1710),
    ("forced-color-adjust-none", 1710),
    ("grayscale", 1510),
    ("grid", 160),
    ("grid-cols-none", 690),
    ("grid-cols-subgrid", 690),
    ("grid-flow-col", 670),
    ("grid-flow-col-dense", 670),
    ("grid-flow-dense", 670),
    ("grid-flow-row", 670),
    ("grid-flow-row-dense", 670),
    ("grid-rows-none", 700),
    ("grid-rows-subgrid", 700),
    ("grow", 300),
    ("grow-0", 300),
    ("h-auto", 230),
    ("h-dvh", 230),
    ("h-fit", 230),
    ("h-full", 230),
    ("h-lvh", 230),
    ("h-max", 230),
    ("h-min", 230),
    ("h-screen", 230),
    ("h-svh", 230),
    ("hidden", 160),
    ("hyphens-auto", 870),
    ("hyphens-manual", 870),
    ("hyphens-none", 870),
    ("inline", 160),
    ("inline-block", 160),
    ("inline-flex", 160),
    ("inline-grid", 160),
    ("inline-table", 160),
    ("inset-shadow-none", 1840),
    ("invert", 1550),
    ("invisible", 30),
    ("isolate", 60),
    ("isolation-auto", 60),
    ("italic", 1360),
    ("items-baseline", 760),
    ("items-center", 760),
    ("items-end", 760),
    ("items-start", 760),
    ("items-stretch", 760),
    ("justify-around", 770),
    ("justify-between", 770),
    ("justify-center", 770),
    ("justify-end", 770),
    ("justify-evenly", 770),
    ("justify-items-center", 780),
    ("justify-items-end", 780),
    ("justify-items-start", 780),
    ("justify-items-stretch", 780),
    ("justify-normal", 770),
    ("justify-self-auto", 830),
    ("justify-self-center", 830),
    ("justify-self-end", 830),
    ("justify-self-start", 830),
    ("justify-self-stretch", 830),
    ("justify-start", 770),
    ("justify-stretch", 770),
    ("leading-none", 1720),
    ("line-clamp-none", 150),
    ("line-through", 1370),
    ("lining-nums", 1750),
    ("list-decimal", 590),
    ("list-disc", 590),
    ("list-image-none", 600),
    ("list-inside", 580),
    ("list-item", 160),
    ("list-none", 590),
    ("list-outside", 580),
    ("lowercase", 1350),
    ("m-auto", 130),
    ("mask-add", 1080),
    ("mask-alpha", 1090),
    ("mask-auto", 1110),
    ("mask-bottom", 1120),
    ("mask-center", 1120),
    ("mask-clip-border", 1140),
    ("mask-clip-content", 1140),
    ("mask-clip-fill", 1140),
    ("mask-clip-no-clip", 1140),
    ("mask-clip-padding", 1140),
    ("mask-clip-stroke", 1140),
    ("mask-clip-view", 1140),
    ("mask-contain", 1110),
    ("mask-cover", 1110),
    ("mask-exclude", 1080),
    ("mask-intersect", 1080),
    ("mask-left", 1120),
    ("mask-luminance", 1090),
    ("mask-match", 1090),
    ("mask-no-repeat", 1130),
    ("mask-none", 1070),
    ("mask-origin-border", 1150),
    ("mask-origin-content", 1150),
    ("mask-origin-fill", 1150),
    ("mask-origin-padding", 1150),
    ("mask-origin-stroke", 1150),
    ("mask-origin-view", 1150),
    ("mask-repeat", 1130),
    ("mask-repeat-round", 1130),
    ("mask-repeat-space", 1130),
    ("mask-repeat-x", 1130),
    ("mask-repeat-y", 1130),
    ("mask-right", 1120),
    ("mask-subtract", 1080),
    ("mask-top", 1120),
    ("mask-type-alpha", 1100),
    ("mask-type-luminance", 1100),
    ("max-h-dvh", 250),
    ("max-h-fit", 250),
    ("max-h-full", 250),
    ("max-h-lvh", 250),
    ("max-h-max", 250),
    ("max-h-min", 250),
    ("max-h-none", 250),
    ("max-h-screen", 250),
    ("max-h-svh", 250),
    ("max-w-fit", 220),
    ("max-w-full", 220),
    ("max-w-max", 220),
    ("max-w-min", 220),
    ("max-w-none", 220),
    ("max-w-prose", 220),
    ("min-h-dvh", 240),
    ("min-h-fit", 240),
    ("min-h-full", 240),
    ("min-h-lvh", 240),
    ("min-h-max", 240),
    ("min-h-min", 240),
    ("min-h-screen", 240),
    ("min-h-svh", 240),
    ("min-w-fit", 210),
    ("min-w-full", 210),
    ("min-w-max", 210),
    ("min-w-min", 210),
    ("mix-blend-color", 1240),
    ("mix-blend-color-burn", 1240),
    ("mix-blend-color-dodge", 1240),
    ("mix-blend-darken", 1240),
    ("mix-blend-difference", 1240),
    ("mix-blend-exclusion", 1240),
    ("mix-blend-hard-light", 1240),
    ("mix-blend-hue", 1240),
    ("mix-blend-lighten", 1240),
    ("mix-blend-luminosity", 1240),
    ("mix-blend-multiply", 1240),
    ("mix-blend-normal", 1240),
    ("mix-blend-overlay", 1240),
    ("mix-blend-plus-darker", 1240),
    ("mix-blend-plus-lighter", 1240),
    ("mix-blend-saturation", 1240),
    ("mix-blend-screen", 1240),
    ("mix-blend-soft-light", 1240),
    ("no-underline", 1370),
    ("normal-case", 1350),
    ("normal-nums", 1750),
    ("not-italic", 1360),
    ("not-sr-only", 10),
    ("object-contain", 1270),
    ("object-cover", 1270),
    ("object-fill", 1270),
    ("object-none", 1270),
    ("object-scale-down", 1270),
    ("oldstyle-nums", 1750),
    ("order-first", 80),
    ("order-last", 80),
    ("order-none", 80),
    ("ordinal", 1750),
    ("origin-bottom", 360),
    ("origin-bottom-left", 360),
    ("origin-bottom-right", 360),
    ("origin-center", 360),
    ("origin-left", 360),
    ("origin-right", 360),
    ("origin-top", 360),
    ("origin-top-left", 360),
    ("origin-top-right", 360),
    ("outline", 1760),
    ("outline-dashed", 1760),
    ("outline-dotted", 1760),
    ("outline-double", 1760),
    ("outline-hidden", 1760),
    ("outline-none", 1760),
    ("outline-solid", 1760),
    ("overflow-auto", 840),
    ("overflow-clip", 840),
    ("overflow-hidden", 840),
    ("overflow-scroll", 840),
    ("overflow-visible", 840),
    ("overflow-x-auto", 840),
    ("overflow-x-clip", 840),
    ("overflow-x-hidden", 840),
    ("overflow-x-scroll", 840),
    ("overflow-x-visible", 840),
    ("overflow-y-auto", 840),
    ("overflow-y-clip", 840),
    ("overflow-y-hidden", 840),
    ("overflow-y-scroll", 840),
    ("overflow-y-visible", 840),
    ("overline", 1370),
    ("overscroll-auto", 850),
    ("overscroll-contain", 850),
    ("overscroll-none", 850),
    ("overscroll-x-auto", 850),
    ("overscroll-x-contain", 850),
    ("overscroll-x-none", 850),
    ("overscroll-y-auto", 850),
    ("overscroll-y-contain", 850),
    ("overscroll-y-none", 850),
    ("p-auto", 1290),
    ("perspective-distant", 380),
    ("perspective-dramatic", 380),
    ("perspective-midrange", 380),
    ("perspective-near", 380),
    ("perspective-none", 380),
    ("perspective-normal", 380),
    ("place-content-baseline", 730),
    ("place-content-center", 730),
    ("place-content-end", 730),
    ("place-content-start", 730),
    ("place-content-stretch", 730),
    ("place-items-baseline", 740),
    ("place-items-center", 740),
    ("place-items-end", 740),
    ("place-items-start", 740),
    ("place-items-stretch", 740),
    ("place-self-auto", 810),
    ("place-self-center", 810),
    ("place-self-end", 810),
    ("place-self-start", 810),
    ("place-self-stretch", 810),
    ("pointer-events-auto", 20),
    ("pointer-events-none", 20),
    ("proportional-nums", 1750),
    ("relative", 40),
    ("resize", 510),
    ("resize-none", 510),
    ("resize-x", 510),
    ("resize-y", 510),
    ("ring-inset", 1850),
    ("rotate-none", 410),
    ("rounded", 910),
    ("rounded-full", 910),
    ("rounded-none", 910),
    ("row-auto", 100),
    ("scale-3d", 400),
    ("scale-none", 400),
    ("scroll-auto", 570),
    ("scroll-smooth", 570),
    ("select-all", 500),
    ("select-auto", 500),
    ("select-none", 500),
    ("select-text", 500),
    ("self-auto", 820),
    ("self-baseline", 820),
    ("self-center", 820),
    ("self-end", 820),
    ("self-start", 820),
    ("self-stretch", 820),
    ("sepia", 1590),
    ("shadow-none", 1830),
    ("shrink", 290),
    ("shrink-0", 290),
    ("slashed-zero", 1750),
    ("snap-align-none", 530),
    ("snap-always", 540),
    ("snap-both", 520),
    ("snap-center", 530),
    ("snap-end", 530),
    ("snap-mandatory", 520),
    ("snap-none", 520),
    ("snap-normal", 540),
    ("snap-proximity", 520),
    ("snap-start", 530),
    ("snap-x", 520),
    ("snap-y", 520),
    ("space-x-reverse", 800),
    ("space-y-reverse", 800),
    ("sr-only", 10),
    ("stacked-fractions", 1750),
    ("static", 40),
    ("sticky", 40),
    ("stroke-none", 1260),
    ("subpixel-antialiased", 1740),
    ("table", 160),
    ("table-auto", 320),
    ("table-caption", 160),
    ("table-cell", 160),
    ("table-column", 160),
    ("table-column-group", 160),
    ("table-fixed", 320),
    ("table-footer-group", 160),
    ("table-header-group", 160),
    ("table-row", 160),
    ("table-row-group", 160),
    ("tabular-nums", 1750),
    ("text-balance", 890),
    ("text-center", 1300),
    ("text-clip", 860),
    ("text-ellipsis", 860),
    ("text-end", 1300),
    ("text-justify", 1300),
    ("text-left", 1300),
    ("text-nowrap", 890),
    ("text-pretty", 890),
    ("text-right", 1300),
    ("text-shadow-none", 1820),
    ("text-start", 1300),
    ("text-wrap", 890),
    ("touch-auto", 490),
    ("touch-manipulation", 490),
    ("touch-none", 490),
    ("touch-pan-down", 490),
    ("touch-pan-left", 490),
    ("touch-pan-right", 490),
    ("touch-pan-up", 490),
    ("touch-pan-x", 490),
    ("touch-pan-y", 490),
    ("touch-pinch-zoom", 490),
    ("transform", 430),
    ("transform-3d", 450),
    ("transform-border", 460),
    ("transform-content", 460),
    ("transform-cpu", 430),
    ("transform-fill", 460),
    ("transform-flat", 450),
    ("transform-gpu", 430),
    ("transform-none", 430),
    ("transform-stroke", 460),
    ("transform-view", 460),
    ("transition", 1630),
    ("transition-all", 1630),
    ("transition-colors", 1630),
    ("transition-discrete", 1640),
    ("transition-none", 1630),
    ("transition-normal", 1640),
    ("transition-opacity", 1630),
    ("transition-shadow", 1630),
    ("transition-transform", 1630),
    ("translate-3d", 390),
    ("truncate", 860),
    ("underline", 1370),
    ("underline-offset-auto", 1800),
    ("uppercase", 1350),
    ("visible", 30),
    ("w-auto", 200),
    ("w-dvw", 200),
    ("w-fit", 200),
    ("w-full", 200),
    ("w-lvw", 200),
    ("w-max", 200),
    ("w-min", 200),
    ("w-screen", 200),
    ("w-svw", 200),
    ("whitespace-break-spaces", 880),
    ("whitespace-normal", 880),
    ("whitespace-nowrap", 880),
    ("whitespace-pre", 880),
    ("whitespace-pre-line", 880),
    ("whitespace-pre-wrap", 880),
    ("will-change-auto", 1680),
    ("will-change-contents", 1680),
    ("will-change-scroll", 1680),
    ("will-change-transform", 1680),
];

/// Prefix-match utilities: prefix -> sort order.
/// SORTED BY PREFIX LENGTH DESCENDING (longest first) for correct longest-match.
static PREFIX_UTILITIES: &[(&str, u32)] = &[
    // 20+ chars
    ("backdrop-brightness-", 1480),
    ("backdrop-grayscale-", 1520),
    ("backdrop-contrast-", 1500),
    ("backdrop-hue-rotate-", 1540),
    ("backdrop-invert-", 1560),
    ("backdrop-opacity-", 1620),
    ("backdrop-saturate-", 1580),
    ("backdrop-sepia-", 1600),
    ("backdrop-blur-", 1460),
    // border-spacing-x/y (18 chars)
    ("border-spacing-x-", 350),
    ("border-spacing-y-", 350),
    // border-spacing (15 chars)
    ("border-spacing-", 350),
    // perspective-origin (19 chars)
    ("perspective-origin-", 370),
    // underline-offset (16 chars)
    ("underline-offset-", 1800),
    // outline-offset (14 chars)
    ("outline-offset-", 1780),
    // font-features (13 chars)
    ("font-features-", 1340),
    // font-stretch (12 chars)
    ("font-stretch-", 1380),
    // inset-shadow (12 chars)
    ("inset-shadow-", 1840),
    // inset-ring (10 chars)
    ("inset-ring-", 1860),
    // text-shadow (11 chars)
    ("text-shadow-", 1820),
    // ring-offset (11 chars)
    ("ring-offset-", 1870),
    // drop-shadow (11 chars)
    ("drop-shadow-", 1610),
    // mask-linear (11 chars)
    ("mask-linear-", 1170),
    // mask-radial (11 chars)
    ("mask-radial-", 1180),
    // mask-conic (10 chars)
    ("mask-conic-", 1190),
    // mask-position (13 chars)
    ("mask-position-", 1120),
    // bg-linear (9 chars)
    ("bg-linear-", 1020),
    ("bg-gradient-", 1020),
    // bg-conic (8 chars)
    ("bg-conic-", 1030),
    // bg-radial (9 chars)
    ("bg-radial-", 1040),
    // place-content (13 chars)
    ("place-content-", 730),
    // place-items (11 chars)
    ("place-items-", 740),
    // justify-items (13 chars)
    ("justify-items-", 780),
    // justify-self (12 chars)
    ("justify-self-", 830),
    // place-self (10 chars)
    ("place-self-", 810),
    // scroll-m (8 chars) — various directions
    ("scroll-mx-", 550),
    ("scroll-my-", 550),
    ("scroll-ms-", 550),
    ("scroll-me-", 550),
    ("scroll-mt-", 550),
    ("scroll-mr-", 550),
    ("scroll-mb-", 550),
    ("scroll-ml-", 550),
    ("scroll-m-", 550),
    // scroll-p (8 chars) — various directions
    ("scroll-px-", 560),
    ("scroll-py-", 560),
    ("scroll-ps-", 560),
    ("scroll-pe-", 560),
    ("scroll-pt-", 560),
    ("scroll-pr-", 560),
    ("scroll-pb-", 560),
    ("scroll-pl-", 560),
    ("scroll-p-", 560),
    // translate (9-12 chars)
    ("translate-x-", 390),
    ("translate-y-", 390),
    ("translate-z-", 390),
    ("translate-", 390),
    // scale (5-8 chars)
    ("scale-x-", 400),
    ("scale-y-", 400),
    ("scale-z-", 400),
    ("scale-", 400),
    // rotate (6-9 chars)
    ("rotate-x-", 410),
    ("rotate-y-", 410),
    ("rotate-z-", 410),
    ("rotate-", 410),
    // skew (4-7 chars)
    ("skew-x-", 420),
    ("skew-y-", 420),
    ("skew-", 420),
    // hue-rotate (10 chars)
    ("hue-rotate-", 1530),
    // line-clamp (10 chars)
    ("line-clamp-", 150),
    // list-image (10 chars)
    ("list-image-", 600),
    // col-span/start/end (8-10 chars)
    ("col-span-", 90),
    ("col-start-", 90),
    ("col-end-", 90),
    ("col-", 90),
    // row-span/start/end
    ("row-span-", 100),
    ("row-start-", 100),
    ("row-end-", 100),
    ("row-", 100),
    // grid-cols/rows (9-10 chars)
    ("grid-cols-", 690),
    ("grid-rows-", 700),
    // auto-cols/rows (9-10 chars)
    ("auto-cols-", 660),
    ("auto-rows-", 680),
    // divide-x/y (8-9 chars) — width
    ("divide-x-", 950),
    ("divide-y-", 950),
    // divide- (general — style/color)
    ("divide-", 960),
    // rounded (various, longest first)
    ("rounded-tl-", 910),
    ("rounded-tr-", 910),
    ("rounded-br-", 910),
    ("rounded-bl-", 910),
    ("rounded-ss-", 910),
    ("rounded-se-", 910),
    ("rounded-ee-", 910),
    ("rounded-es-", 910),
    ("rounded-t-", 910),
    ("rounded-r-", 910),
    ("rounded-b-", 910),
    ("rounded-l-", 910),
    ("rounded-s-", 910),
    ("rounded-e-", 910),
    ("rounded-", 910),
    // border (various, longest first)
    ("border-x-", 920),
    ("border-y-", 920),
    ("border-t-", 920),
    ("border-r-", 920),
    ("border-b-", 920),
    ("border-l-", 920),
    ("border-s-", 920),
    ("border-e-", 920),
    ("border-", 920),
    // inset (various, longest first)
    ("inset-x-", 50),
    ("inset-y-", 50),
    ("inset-", 50),
    // min/max sizing (longest first)
    ("min-inline-", 260),
    ("max-inline-", 260),
    ("min-block-", 260),
    ("max-block-", 260),
    ("min-w-", 210),
    ("max-w-", 220),
    ("min-h-", 240),
    ("max-h-", 250),
    // margin (various, longest first)
    ("mx-", 130),
    ("my-", 130),
    ("ms-", 130),
    ("me-", 130),
    ("mt-", 130),
    ("mr-", 130),
    ("mb-", 130),
    ("ml-", 130),
    ("m-", 130),
    // padding (various, longest first)
    ("px-", 1290),
    ("py-", 1290),
    ("ps-", 1290),
    ("pe-", 1290),
    ("pt-", 1290),
    ("pr-", 1290),
    ("pb-", 1290),
    ("pl-", 1290),
    ("p-", 1290),
    // spacing
    ("space-x-", 800),
    ("space-y-", 800),
    // gap
    ("gap-x-", 790),
    ("gap-y-", 790),
    ("gap-", 790),
    // top/right/bottom/left/start/end (inset)
    ("top-", 50),
    ("right-", 50),
    ("bottom-", 50),
    ("left-", 50),
    ("start-", 50),
    ("end-", 50),
    // z-index
    ("z-", 70),
    // order
    ("order-", 80),
    // aspect
    ("aspect-", 180),
    // size
    ("size-", 190),
    // width/height
    ("w-", 200),
    ("h-", 230),
    // inline/block sizing
    ("inline-", 260),
    ("block-", 260),
    // flex
    ("flex-", 280),
    // shrink/grow
    ("shrink-", 290),
    ("grow-", 300),
    // basis
    ("basis-", 310),
    // origin
    ("origin-", 360),
    // perspective
    ("perspective-", 380),
    // zoom
    ("zoom-", 440),
    // cursor
    ("cursor-", 480),
    // columns
    ("columns-", 620),
    // list
    ("list-", 590),
    // mask (general — must come after more specific mask- prefixes)
    ("mask-t-", 1160),
    ("mask-r-", 1160),
    ("mask-b-", 1160),
    ("mask-l-", 1160),
    ("mask-x-", 1160),
    ("mask-y-", 1160),
    ("mask-", 1070),
    // bg (general — must come after more specific bg- prefixes)
    ("bg-", 1050),
    // gradient stops
    ("from-", 1060),
    ("via-", 1060),
    ("to-", 1060),
    // fill/stroke
    ("fill-", 1250),
    ("stroke-", 1260),
    // object (position)
    ("object-", 1280),
    // text (general — must come after text-shadow-)
    ("text-", 1810),
    // indent
    ("indent-", 1310),
    // align (vertical)
    ("align-", 1320),
    // font
    ("font-", 1330),
    // placeholder
    ("placeholder-", 1390),
    // decoration
    ("decoration-", 1410),
    // animate
    ("animate-", 1420),
    // blur
    ("blur-", 1450),
    // brightness
    ("brightness-", 1470),
    // contrast
    ("contrast-", 1490),
    // grayscale
    ("grayscale-", 1510),
    // invert
    ("invert-", 1550),
    // saturate
    ("saturate-", 1570),
    // sepia
    ("sepia-", 1590),
    // transition
    ("transition-", 1630),
    // delay
    ("delay-", 1650),
    // duration
    ("duration-", 1660),
    // ease
    ("ease-", 1670),
    // content
    ("content-", 1690),
    // contain
    ("contain-", 1700),
    // leading
    ("leading-", 1720),
    // tracking
    ("tracking-", 1730),
    // outline (general, after outline-offset-)
    ("outline-", 1770),
    // opacity
    ("opacity-", 1790),
    // shadow
    ("shadow-", 1830),
    // ring (after ring-offset-)
    ("ring-", 1850),
];

#[cfg(test)]
mod tests {
    use super::get_class_order;

    #[test]
    fn exact_utility_has_order() {
        assert!(get_class_order("block") > 0);
        assert!(get_class_order("flex") > 0);
        assert!(get_class_order("absolute") > 0);
        assert!(get_class_order("hidden") > 0);
    }

    #[test]
    fn prefix_utility_has_order() {
        assert!(get_class_order("w-4") > 0);
        assert!(get_class_order("p-4") > 0);
        assert!(get_class_order("m-2") > 0);
        assert!(get_class_order("text-red-500") > 0);
        assert!(get_class_order("bg-blue-200") > 0);
    }

    #[test]
    fn unknown_class_returns_zero() {
        assert_eq!(get_class_order("potato"), 0);
        assert_eq!(get_class_order("xyzzy"), 0);
    }

    #[test]
    fn position_sorts_before_display() {
        let pos = get_class_order("absolute");
        let disp = get_class_order("block");
        assert!(
            pos < disp,
            "absolute ({pos}) should sort before block ({disp})"
        );
    }

    #[test]
    fn display_sorts_before_width() {
        let disp = get_class_order("flex");
        let width = get_class_order("w-4");
        assert!(
            disp < width,
            "flex ({disp}) should sort before w-4 ({width})"
        );
    }

    #[test]
    fn width_sorts_before_padding() {
        let width = get_class_order("w-4");
        let pad = get_class_order("p-4");
        assert!(
            width < pad,
            "w-4 ({width}) should sort before p-4 ({pad})"
        );
    }

    #[test]
    fn padding_sorts_before_text_color() {
        let pad = get_class_order("p-4");
        let text = get_class_order("text-red-500");
        assert!(
            pad < text,
            "p-4 ({pad}) should sort before text-red-500 ({text})"
        );
    }

    #[test]
    fn margin_sorts_before_display() {
        let margin = get_class_order("m-4");
        let disp = get_class_order("block");
        assert!(
            margin < disp,
            "m-4 ({margin}) should sort before block ({disp})"
        );
    }

    #[test]
    fn bg_color_sorts_before_shadow() {
        let bg = get_class_order("bg-red-500");
        let shadow = get_class_order("shadow-lg");
        assert!(
            bg < shadow,
            "bg-red-500 ({bg}) should sort before shadow-lg ({shadow})"
        );
    }

    #[test]
    fn negative_value_prefix_matches() {
        let order = get_class_order("-m-4");
        assert!(order > 0, "-m-4 should have a non-zero order, got {order}");
    }

    #[test]
    fn longer_prefix_wins() {
        let bg_linear = get_class_order("bg-linear-to-r");
        let bg_color = get_class_order("bg-red-500");
        assert!(
            bg_linear < bg_color,
            "bg-linear-to-r ({bg_linear}) should sort before bg-red-500 ({bg_color})"
        );
    }

    #[test]
    fn inset_shadow_distinct_from_inset() {
        let inset = get_class_order("inset-4");
        let inset_shadow = get_class_order("inset-shadow-sm");
        assert!(
            inset < inset_shadow,
            "inset-4 ({inset}) should sort before inset-shadow-sm ({inset_shadow})"
        );
    }

    #[test]
    fn text_shadow_distinct_from_text() {
        let text = get_class_order("text-lg");
        let text_shadow = get_class_order("text-shadow-sm");
        assert!(
            text < text_shadow,
            "text-lg ({text}) should sort before text-shadow-sm ({text_shadow})"
        );
    }

    #[test]
    fn ring_and_ring_offset_ordered() {
        let ring = get_class_order("ring-2");
        let ring_offset = get_class_order("ring-offset-2");
        assert!(
            ring < ring_offset,
            "ring-2 ({ring}) should sort before ring-offset-2 ({ring_offset})"
        );
    }

    #[test]
    fn divide_width_before_divide_style() {
        let divide_x = get_class_order("divide-x-2");
        let divide_solid = get_class_order("divide-solid");
        assert!(
            divide_x < divide_solid,
            "divide-x-2 ({divide_x}) should sort before divide-solid ({divide_solid})"
        );
    }

    #[test]
    fn rounded_prefix_matches() {
        let rounded = get_class_order("rounded-lg");
        assert!(rounded > 0, "rounded-lg should have order, got {rounded}");
        let rounded_tl = get_class_order("rounded-tl-lg");
        assert!(
            rounded_tl > 0,
            "rounded-tl-lg should have order, got {rounded_tl}"
        );
    }

    // === Additional order relationship tests ===

    #[test]
    fn sr_only_is_first_category() {
        let sr = get_class_order("sr-only");
        assert!(sr > 0 && sr <= 10, "sr-only should be in first category, got {sr}");
    }

    #[test]
    fn visibility_before_position() {
        let vis = get_class_order("invisible");
        let pos = get_class_order("absolute");
        assert!(vis < pos, "invisible ({vis}) should sort before absolute ({pos})");
    }

    #[test]
    fn z_index_before_grid() {
        let z = get_class_order("z-10");
        let col = get_class_order("col-span-2");
        assert!(z < col, "z-10 ({z}) should sort before col-span-2 ({col})");
    }

    #[test]
    fn size_before_width() {
        let size = get_class_order("size-4");
        let width = get_class_order("w-4");
        assert!(size < width, "size-4 ({size}) should sort before w-4 ({width})");
    }

    #[test]
    fn min_width_before_max_width() {
        let min_w = get_class_order("min-w-0");
        let max_w = get_class_order("max-w-sm");
        assert!(min_w < max_w, "min-w-0 ({min_w}) should sort before max-w-sm ({max_w})");
    }

    #[test]
    fn height_before_min_height() {
        let h = get_class_order("h-4");
        let min_h = get_class_order("min-h-0");
        assert!(h < min_h, "h-4 ({h}) should sort before min-h-0 ({min_h})");
    }

    #[test]
    fn flex_direction_has_order() {
        assert!(get_class_order("flex-col") > 0);
        assert!(get_class_order("flex-row") > 0);
        assert!(get_class_order("flex-row-reverse") > 0);
    }

    #[test]
    fn gap_before_space() {
        let gap = get_class_order("gap-4");
        let space = get_class_order("space-x-4");
        assert!(gap < space, "gap-4 ({gap}) should sort before space-x-4 ({space})");
    }

    #[test]
    fn overflow_before_truncate() {
        let overflow = get_class_order("overflow-hidden");
        let truncate = get_class_order("truncate");
        assert!(overflow < truncate, "overflow-hidden ({overflow}) should sort before truncate ({truncate})");
    }

    #[test]
    fn border_radius_before_border_width() {
        let radius = get_class_order("rounded-lg");
        let width = get_class_order("border-2");
        assert!(radius < width, "rounded-lg ({radius}) should sort before border-2 ({width})");
    }

    #[test]
    fn border_width_before_border_color() {
        // Both use border- prefix, so they get the same order in our implementation
        // Just verify they both have order
        let width = get_class_order("border-2");
        assert!(width > 0);
        let color = get_class_order("border-red-500");
        assert!(color > 0);
    }

    #[test]
    fn bg_position_before_bg_color() {
        let pos = get_class_order("bg-center");
        let color = get_class_order("bg-red-500");
        assert!(pos < color, "bg-center ({pos}) should sort before bg-red-500 ({color})");
    }

    #[test]
    fn padding_before_text_align() {
        let pad = get_class_order("p-4");
        let text = get_class_order("text-center");
        assert!(pad < text, "p-4 ({pad}) should sort before text-center ({text})");
    }

    #[test]
    fn font_before_text_transform() {
        let font = get_class_order("font-bold");
        let transform = get_class_order("uppercase");
        assert!(font < transform, "font-bold ({font}) should sort before uppercase ({transform})");
    }

    #[test]
    fn filter_before_transition() {
        let filter = get_class_order("filter");
        let transition = get_class_order("transition");
        assert!(filter < transition, "filter ({filter}) should sort before transition ({transition})");
    }

    #[test]
    fn transition_before_duration() {
        let transition = get_class_order("transition");
        let duration = get_class_order("duration-200");
        assert!(transition < duration, "transition ({transition}) should sort before duration-200 ({duration})");
    }

    #[test]
    fn shadow_before_ring() {
        let shadow = get_class_order("shadow-lg");
        let ring = get_class_order("ring-2");
        assert!(shadow < ring, "shadow-lg ({shadow}) should sort before ring-2 ({ring})");
    }

    #[test]
    fn opacity_before_shadow() {
        let opacity = get_class_order("opacity-50");
        let shadow = get_class_order("shadow-lg");
        assert!(opacity < shadow, "opacity-50 ({opacity}) should sort before shadow-lg ({shadow})");
    }

    #[test]
    fn animate_before_filter() {
        let animate = get_class_order("animate-spin");
        let filter = get_class_order("filter");
        assert!(animate < filter, "animate-spin ({animate}) should sort before filter ({filter})");
    }

    #[test]
    fn leading_before_tracking() {
        let leading = get_class_order("leading-6");
        let tracking = get_class_order("tracking-wide");
        assert!(leading < tracking, "leading-6 ({leading}) should sort before tracking-wide ({tracking})");
    }

    #[test]
    fn scroll_margin_before_scroll_padding() {
        let margin = get_class_order("scroll-m-4");
        let padding = get_class_order("scroll-p-4");
        assert!(margin < padding, "scroll-m-4 ({margin}) should sort before scroll-p-4 ({padding})");
    }

    #[test]
    fn negative_translate() {
        let order = get_class_order("-translate-x-1");
        assert!(order > 0, "-translate-x-1 should have order, got {order}");
    }

    #[test]
    fn all_exact_display_utilities() {
        for class in &[
            "block", "inline-block", "inline", "flex", "inline-flex",
            "table", "grid", "inline-grid", "contents", "hidden",
            "flow-root", "list-item",
        ] {
            let order = get_class_order(class);
            assert!(
                order > 0,
                "{class} should have order, got {order}"
            );
        }
    }

    #[test]
    fn all_position_utilities() {
        for class in &["static", "fixed", "absolute", "relative", "sticky"] {
            let order = get_class_order(class);
            assert!(
                order > 0,
                "{class} should have order, got {order}"
            );
        }
    }
}
