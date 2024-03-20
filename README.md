#### Build for web
```wasm-pack build --target web```

or serve locally

```cargo run --target wasm32-unknown-unknown```

or watch

```cargo watch -x 'run --target wasm32-unknown-unknown'```
requires load wasm in html file directly like
```
async function run() {
  await init("/pkg/your_project_name_bg.wasm");
}
```
