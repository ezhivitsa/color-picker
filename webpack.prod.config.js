/* eslint-disable @typescript-eslint/no-var-requires */
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const dist = path.resolve(__dirname, 'dist');
const publicPath = process.env.PUBLIC_PATH ? `${process.env.PUBLIC_PATH}/` : '';

module.exports = {
  mode: 'production',
  entry: {
    index: './src/client/index.js',
  },
  output: {
    path: dist,
    filename: '[name].js',
    chunkFilename: '[name].chunk.js',
    publicPath: `/${publicPath}`,
  },

  resolve: {
    // Add '.ts' and '.tsx' as resolvable extensions.
    extensions: ['.ts', '.tsx', '.js', '.jsx'],
    modules: ['node_modules', path.resolve(__dirname, 'src/client'), __dirname],
  },

  module: {
    rules: [
      {
        test: /\.(ts|js)x?$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
        },
      },

      // css-loader to bundle all the css files into one file and style-loader
      // to add all the styles inside the style tag of the document
      {
        test: /\.pcss$/,
        use: [
          MiniCssExtractPlugin.loader,
          {
            loader: 'css-loader',
            options: {
              importLoaders: 1,
              modules: {
                mode: 'local',
                localIdentName: '[local]--[hash:base64:5]',
                context: path.resolve(__dirname, 'src/client'),
              },
              localsConvention: 'dashesOnly',
            },
          },
          'postcss-loader',
        ],
      },
    ],
  },

  plugins: [
    new HtmlWebpackPlugin({
      template: './src/client/index.html',
    }),
    new MiniCssExtractPlugin({
      filename: 'main.[hash].css',
      chunkFilename: 'main.[id].[hash].css',
    }),
  ],
};
