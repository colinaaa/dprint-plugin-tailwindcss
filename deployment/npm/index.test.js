const assert = require("node:assert/strict");

const { createFromBuffer } = require("@dprint/formatter");
const { getPath } = require("./index.js");

async function main() {
  const formatter = await createFromBuffer(await require("node:fs/promises").readFile(getPath()));

  // Test 1: basic HTML class sorting
  const result1 = await formatter.formatText({
    filePath: "test.html",
    fileText: '<div class="p-4 flex absolute">hello</div>\n',
    overrideConfig: {},
  });
  assert.equal(
    result1,
    '<div class="absolute flex p-4">hello</div>\n'
  );

  // Test 2: CSS @apply sorting
  const result2 = await formatter.formatText({
    filePath: "styles.css",
    fileText: '.btn {\n  @apply p-4 flex absolute;\n}\n',
    overrideConfig: {},
  });
  assert.equal(
    result2,
    '.btn {\n  @apply absolute flex p-4;\n}\n'
  );

  // Test 3: already sorted returns same text (idempotent)
  const alreadySorted = '<div class="absolute flex p-4">hello</div>\n';
  const result3 = await formatter.formatText({
    filePath: "test.html",
    fileText: alreadySorted,
    overrideConfig: {},
  });
  // formatText may return the same text or undefined when no changes needed
  assert.ok(
    result3 === undefined || result3 === alreadySorted,
    `Expected undefined or same text, got: ${JSON.stringify(result3)}`
  );

  console.log("All npm smoke tests passed.");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
