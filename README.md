# dprint-plugin-tailwindcss

Sorts [Tailwind CSS](https://tailwindcss.com/) utility classes with [dprint](https://dprint.dev/).

## Usage

Add the npm package to your `dprint.json`:

```jsonc
{
  "plugins": [
    "npm:dprint-plugin-tailwindcss@0.1.0"
  ]
}
```

Or use the wasm file from GitHub Releases directly:

```jsonc
{
  "plugins": [
    "https://github.com/colinaaa/dprint-plugin-tailwindcss/releases/download/0.1.0/plugin.wasm"
  ]
}
```

## Supported File Types

| File type | Extensions | What gets sorted |
|-----------|-----------|-----------------|
| HTML | `.html`, `.htm` | `class="..."` |
| JSX / TSX | `.jsx`, `.tsx` | `class="..."`, `className="..."` |
| Vue | `.vue` | `class="..."`, `:class="'...'"`, `v-bind:class="'...'"` |
| Svelte | `.svelte` | `class="..."` |
| Astro | `.astro` | `class="..."` |
| CSS / SCSS / Less | `.css`, `.scss`, `.less` | `@apply ...;` |

## Configuration

```jsonc
{
  "tailwindcss": {
    // Remove duplicate CSS classes (default: true)
    "removeDuplicates": true,

    // Collapse extra whitespace between classes (default: true)
    "collapseWhitespace": true,

    // Additional attribute names to sort
    // Supports regex patterns in /pattern/ syntax
    "tailwindAttributes": ["myClass", "/data-tw-.*/"],

    // Function names whose string arguments should be sorted
    // Also matches tagged templates and chained member calls (e.g. tw.div`...`)
    "tailwindFunctions": ["clsx", "cn", "tw", "cva"]
  }
}
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `locked` | `boolean` | — | Standard dprint flag that prevents this config block from being overridden or extended. |
| `removeDuplicates` | `boolean` | `true` | Remove duplicate CSS classes. Unknown (non-Tailwind) class duplicates are always preserved. |
| `collapseWhitespace` | `boolean` | `true` | Collapse extra whitespace between classes to a single space. |
| `tailwindAttributes` | `string[]` | `[]` | Additional attribute names to sort. Use `/pattern/` for regex matching. |
| `tailwindFunctions` | `string[]` | `[]` | Function/tag names whose string arguments should be sorted. Matches `fn("...")`, `fn('...')`, `` fn`...` ``, and chained calls like `fn.div("...")`. |

## How It Works

The plugin uses a static class order map based on Tailwind CSS v4's default utility registration order. Classes are sorted following an "outside-in" CSS property model:

> Layout / Position → Box Model (margin, sizing, padding) → Typography → Visual (backgrounds, borders) → Effects (shadows, filters) → Transitions → Miscellaneous

### Sorting Rules

- **Base utilities before variants**: `flex` sorts before `sm:flex`, `hover:flex`, etc.
- **Unknown classes sort to the beginning**: classes not in the Tailwind order map are kept at the front, preserving their relative order.
- **Ellipsis markers (`...`, `…`) sort to the end**: for use with spread/rest patterns.
- **Template interpolation is skipped**: class values containing `{{...}}` (Mustache/Handlebars) or `{...}` (Svelte) are left untouched.
- **Comments are respected**: `class` attributes inside HTML comments (`<!-- -->`), JS block comments (`/* */`), and line comments (`//`) are not sorted.

## JS Formatting API

The plugin can also be used programmatically via the npm package:

```bash
npm install @dprint/formatter dprint-plugin-tailwindcss
```

```js
import fs from "node:fs";
import { createFromBuffer } from "@dprint/formatter";
import { getPath } from "dprint-plugin-tailwindcss";

const formatter = createFromBuffer(fs.readFileSync(getPath()));

const result = formatter.formatText({
  filePath: "test.html",
  fileText: '<div class="p-4 flex absolute">hello</div>',
});

console.log(result);
// '<div class="absolute flex p-4">hello</div>'
```

## Credits

This plugin is inspired by [`prettier-plugin-tailwindcss`](https://github.com/tailwindlabs/prettier-plugin-tailwindcss) by [Tailwind Labs](https://github.com/tailwindlabs). The class sorting order, feature set, and many test cases are derived from that project.

Thanks to the following projects for making this plugin possible:

- [`prettier-plugin-tailwindcss`](https://github.com/tailwindlabs/prettier-plugin-tailwindcss) — the original Prettier plugin for sorting Tailwind CSS classes, which this project aims to bring to the dprint ecosystem.
- [`tailwindcss`](https://github.com/tailwindlabs/tailwindcss) — the utility-first CSS framework whose v4 utility registration order defines our sorting rules.
- [`dprint`](https://github.com/dprint/dprint) — the fast, pluggable code formatter that powers this plugin.

## License

[MIT](LICENSE)
