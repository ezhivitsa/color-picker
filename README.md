# Color-picker

[![license](https://img.shields.io/github/license/mashape/apistatus.svg?style=flat-square)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

[![Watch on GitHub](https://img.shields.io/github/watchers/ezhivitsa/color-picker.svg?style=social&label=Watch)](https://github.com/ezhivitsa/color-picker/watchers)
[![Star on GitHub](https://img.shields.io/github/stars/ezhivitsa/color-picker.svg?style=social&label=Stars)](https://github.com/ezhivitsa/color-picker/stargazers)

Calculator is based on the following primary technologies:

- webpack
- postcss
- webassembly
- rust
- yew

Application structured in a way that allows us to write rust code for frontend part through WebAssembly. In particular we rust framework `yew` to construct html (something similar to `jsx` in `React`) and add business logic.

### Demo

[https://ezhivitsa.github.io/color-picker/](https://ezhivitsa.github.io/color-picker/)

### How to start development

Type the following command to install dependencies:
```bash
$ make deps
```

To run webpack build of the rust and js files type the following command:
```bash
$ make dev
```

And open in the browser `http://localhost:8080`

### How to make a build

To make a final build type the following command:
```bash
$ make build
```

### How to start tests

To start tests type the following command:
```bash
$ make test
```

### Explanations of the files structure.

1. **[src/client/styles](src/client/styles)** - global styles and css variables, styles for components
2. **[src/wasm](./src/wasm)** - this folder contains `Rust` codebase. We compile `wasm` file using this code.
3. **[src/wasm/agents](src/client/agents)** - this folder contains code with yew agents to handle business logic. For example here we handle all changes in input and recalculate new current color.
4. **[src/wasm/constants](src/wasm/constants)** - this folder contains constants which can be used across the application.
5. **[src/wasm/libs](src/wasm/libs)** - folder with common functions where we can do some common actions, for example transformations or validations. This functions help us to avoid copy-paste of functionality.
6. **[src/wasm/services](src/wasm/services)** - folder with yew services, for example^ here we have a service to handle global mouse events (mouse events for `body` tag)
7. **[src/wasm/components](src/wasm/components)** - `yew` components (similar to `React` components).
8. **[src/wasm/texts](src/wasm/texts)** - static texts

### Important things to keep in mind

- For css class names we use BEM conventions, because it's convenient when your styles are separate from usage. This allows us to not think about intersections of class names.

### Conventions

- For naming we use [rust code style](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- For code format use built-in formatter (`cargo fmt`)

## License

Calculator is released under the [MIT License](LICENSE).
