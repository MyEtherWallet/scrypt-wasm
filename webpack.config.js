const path = require("path");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const dist = path.resolve(__dirname, "dist");
module.exports = {
  entry: "./src/index.js",
  output: {
    path: dist,
    filename: "index.js"
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname)
    }),
    new webpack.ProvidePlugin({
      TextDecoder: ["text-encoding", "TextDecoder"],
      TextEncoder: ["text-encoding", "TextEncoder"]
    })
  ]
};
