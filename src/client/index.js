import './styles/global.pcss';

async function initColorPicker() {
  const { default: init } = await import('../../pkg/color_picker');
  init();
}

initColorPicker();
