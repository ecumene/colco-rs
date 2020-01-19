const path = require('path');

module.exports = {
  bail: true,
  devtool: 'source-map',
  mode: 'production',
  entry: './src',
  node: {
    fs: "empty"
  },
  output: {
    filename: 'index.js',
    chunkFilename: '[name].chunk.js',
    publicPath: './'
  },
  module: {
    rules: [
      {
        oneOf: [
          {
            test: /\.(js|jsx|mjs)$/,
            loader: 'babel-loader',
            options: {
              compact: true,
            },
          },
          {
            test: /\.rs$/,
            use: [
              {
                loader: 'babel-loader',
                options: {
                  compact: true,
                }
              },
              {
                loader: 'rust-native-wasm-loader',
                options: {
                  release: true,
                  cargoWeb: true,
                  name: '[name].wasm'
                }
              }
            ]
          },
        ],
      },
    ], 
  },
  resolve: {
    extensions: ['.js'],
  },
};
