build:
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web

dev:
	fswatch -o -r ./src | xargs -I {} sh -c "./dev_watch.sh"
# 	fswatch -o -r ./src | xargs -I {} wasm-pack build --no-pack --no-typescript --target web .
# 	fswatch -o -r ./src | xargs -I {} RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --no-pack --no-typescript --target web .
# 	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' fswatch -o -r ./src | xargs -I {} wasm-pack build --no-pack --no-typescript --target web ./pkg

serve:
	static-web-server --port 3000 --root .

# dev:
# 	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo watch -x 'run --target wasm32-unknown-unknown'
# 	cargo watch -x 'run --target wasm32-unknown-unknown'
#  'RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
