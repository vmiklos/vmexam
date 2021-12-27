build:
	wasm-pack build
	cd www && npm install
	cd www && npx webpack --config webpack.config.js

run:
	cd www/dist && ../../server.py
