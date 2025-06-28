RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --dev --no-pack --no-typescript --target web .
