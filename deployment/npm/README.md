# dprint-plugin-tailwindcss

Wasm module for `dprint-plugin-tailwindcss`. Sorts Tailwind CSS classes in HTML, JSX/TSX, Vue, Svelte, Astro, and CSS files.

## Install

```bash
npm install @dprint/formatter dprint-plugin-tailwindcss
```

## Usage

```js
import fs from "node:fs";
import { createFromBuffer } from "@dprint/formatter";
import { getPath } from "dprint-plugin-tailwindcss";

const formatter = createFromBuffer(fs.readFileSync(getPath()));

const formattedText = formatter.formatText({
  filePath: "test.html",
  fileText: '<div class="p-4 flex absolute">hello</div>',
});

console.log(formattedText);
// '<div class="absolute flex p-4">hello</div>'
```

## Links

- Repository: https://github.com/colinaaa/dprint-plugin-tailwindcss
- Issues: https://github.com/colinaaa/dprint-plugin-tailwindcss/issues
