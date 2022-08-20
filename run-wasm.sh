set -e
./build-wasm.sh
(cd wasm && npx serve)
