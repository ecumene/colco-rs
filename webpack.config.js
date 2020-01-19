const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
  mode: 'production',
  entry: './target/wasm32-unknown-unknown/debug/colco.js',
  node: {
    fs: "empty"
  },
  output: {
    filename: 'env.mjs',
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx|mjs)$/,
        loader: 'babel-loader',
        options: {
          compact: true,
        },
      },
    ],
  },
  plugins: [
    new CopyPlugin([
      { from: 'target/deploy/colco.wasm', to: 'dist' },
    ]),
  ],
  resolve: {
    extensions: ['.js'],
  },
};
