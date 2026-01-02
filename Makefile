build:
	wasm-pack build --target web
run: build
	python -m http.server 8000
