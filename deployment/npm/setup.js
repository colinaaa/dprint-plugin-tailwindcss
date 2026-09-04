const fs = require("node:fs");
const path = require("node:path");

const packageJsonPath = path.join(__dirname, "package.json");
const cargoTomlPath = path.join(__dirname, "..", "..", "Cargo.toml");
const wasmSourcePath = path.join(
  __dirname,
  "..",
  "..",
  "target",
  "wasm32-unknown-unknown",
  "release",
  "dprint_plugin_tailwindcss.wasm"
);
const wasmDestPath = path.join(__dirname, "plugin.wasm");

syncVersionFromCargoToml();
copyPluginWasm();

function syncVersionFromCargoToml() {
  if (!fs.existsSync(cargoTomlPath) || !fs.existsSync(packageJsonPath)) {
    return;
  }

  const cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
  const versionMatch = cargoToml.match(/^version\s*=\s*"([^"]+)"/m);
  if (versionMatch == null) {
    return;
  }

  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
  if (packageJson.version === versionMatch[1]) {
    return;
  }

  packageJson.version = versionMatch[1];
  fs.writeFileSync(packageJsonPath, `${JSON.stringify(packageJson, null, 2)}\n`);
}

function copyPluginWasm() {
  if (!fs.existsSync(wasmSourcePath)) {
    throw new Error(`Missing wasm build output at ${wasmSourcePath}. Run cargo build --release --target wasm32-unknown-unknown --features wasm first.`);
  }

  fs.copyFileSync(wasmSourcePath, wasmDestPath);
}
