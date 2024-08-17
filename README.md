# Bevy craft

## Run

```shell
cargo run --features bevy/dynamic_linking
```

## Build Web

```shell
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "bevy-craft" ./target/wasm32-unknown-unknown/release/bevy-craft.wasm
```
