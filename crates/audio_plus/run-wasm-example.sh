EXAMPLE=demo
set -e
rm -rf wasm
cargo build --target wasm32-unknown-unknown --release --example $EXAMPLE
wasm-bindgen --no-typescript --out-name example --out-dir wasm --target web target/wasm32-unknown-unknown/release/examples/$EXAMPLE.wasm
cp -r assets wasm/
cp static/* wasm/
(cd wasm && npx serve)
