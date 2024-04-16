web:
	wasm-pack build --target web

dev:
	cargo watch -x 'run --target wasm32-unknown-unknown'

serve:
	static-web-server --port 3000 --root .
