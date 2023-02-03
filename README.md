# DOOM Fire

[![Screenshot](./misc/animation.gif)](https://lev875.github.io/doom_fire/)

A variation on the famous [DOOM Fire algorithm](https://fabiensanglard.net/doom_fire_psx/) written using WebAssembly, Rust and JS.

[Check it out](https://lev875.github.io/doom_fire/)

## Dependencies

- cargo
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- yarn
- [taskfile (Optional)](https://taskfile.dev/)

## Building

### Rust

```bash
cd rust
rm -rf pkg
wasm-pack build --target web
cd ..
cp -r rust/pkg/ ./src/wasm/
```

### JS
```bash
yarn build
```

Alternatively, you can just install [Taskfile](https://taskfile.dev/) and run ```task build``` from the root of the repository
