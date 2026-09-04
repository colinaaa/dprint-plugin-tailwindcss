const path = require("node:path");

function getPath() {
  return path.join(__dirname, "plugin.wasm");
}

module.exports = {
  getPath
};
