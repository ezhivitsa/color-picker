const postcssImport = require('postcss-import');
const postcssPresetEnv = require('postcss-preset-env');
const postcssNested = require('postcss-nested');
const postcssCustomProperties = require('postcss-custom-properties');

const cssnano = require('cssnano');

module.exports = () => ({
  plugins: [
    postcssImport({
      path: ['src/client/styles']
    }),
    postcssNested,
    postcssPresetEnv({
      stage: 2,
      feature: {
        'custom-media-queries': true,
        'custom-properties': false
      }
    }),
    cssnano()
  ]
});
