const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  entry: './web/index.js',
  output: {
    path: path.resolve(__dirname, 'www'),
    filename: 'index.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './web/index.html',
      favicon: './web/favicon.svg'
    }),
    new WasmPackPlugin({crateDirectory: path.resolve(__dirname, '.')}),
  ],
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
    ]
  },
  mode: 'development',
  experiments: {
    asyncWebAssembly: true,
    topLevelAwait: true
  }
};
