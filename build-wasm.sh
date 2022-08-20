set -e
rm -rf wasm
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --no-typescript --out-name jam --out-dir wasm --target web target/wasm32-unknown-unknown/release/jam.wasm
cp -r assets wasm/
cp static/* wasm/
