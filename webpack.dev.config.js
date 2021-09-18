/* eslint-disable @typescript-eslint/no-var-requires */
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const port = process.env.PORT || 8080;
const dist = path.resolve(__dirname, 'dist');

module.exports = {
  mode: 'development',

  // Enable sourcemaps for debugging webpack's output.
  devtool: 'source-map',

  entry: './src/client/index.js',
  output: {
    path: dist,
    filename: '[name].js',
  },

  devServer: {
    hot: true,
    port,
    allowedHosts: 'all',
    static: {
      directory: path.join(__dirname, 'pkg'),
      watch: {
        aggregateTimeout: 300,
        poll: 100,
      },
    },
  },

  experiments: {
    asyncWebAssembly: true
  },

  resolve: {
    // Add '.ts' and '.tsx' as resolvable extensions.
    extensions: ['.ts', '.tsx', '.js', '.jsx'],
    modules: ['node_modules', path.resolve(__dirname, 'src/client'), __dirname],
  },

  module: {
    rules: [
      // css-loader to bundle all the css files into one file and style-loader
      // to add all the styles inside the style tag of the document
      {
        test: /\.pcss$/,
        use: [
          'style-loader',
          {
            loader: 'css-loader',
            options: {
              sourceMap: true
            },
          },
          {
            loader: 'postcss-loader',
            options: {
              sourceMap: true,
            },
          },
        ],
      },
    ],
  },

  plugins: [
    new HtmlWebpackPlugin({
      template: './src/client/index.html',
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      outName: 'color_picker',
      outDir: 'pkg',
      extraArgs: '--target web --mode normal',
    }),
  ],
};
