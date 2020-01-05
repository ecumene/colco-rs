const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
  mode: 'production',
  entry: './target/deploy/colco.js',
  node: {
    fs: "empty"
  },
  output: {
    path: path.resolve('dist'),
    filename: 'index.js',
    libraryTarget: 'commonjs2',
  },
  module: {
    rules: [
      {
        test: /\.js?$/,
        exclude: /(node_modules)/,
        use: 'babel-loader',
      },
      {
        test: /\.wasm$/,
        exclude: /(node_modules)/,
        use: 'wasm-loader'
      }
    ],
  },
  plugins: [
    new CopyPlugin([
      { 
        from: './target/deploy/colco.wasm',
      },
    ]),
  ],
  resolve: {
    extensions: ['.js'],
  },
};