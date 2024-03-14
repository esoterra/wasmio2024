# Wasm IO 2024 Claw Demo

## Setup

```sh
cargo install claw-cli
cargo install component-opt
```

## Basic

```sh
claw-cli compile -i timer-proxy.claw -o timer-proxy.wasm --wit wit
cargo run -- -i timer-proxy.wasm
```

## Component Optimization

Use `component-opt` to optimize the binary size.
This currently just runs `wasm-opt` on each module.

```sh
# Optimize the file
component-opt -i timer-proxy.wasm -o timer-proxy-opt.wasm
# Print file size (expect 1212 bytes)
ls -l timer-proxy.wasm
# Print optimized file size (expect 1064)
ls -l timer-proxy-opt.wasm
# Rerun the demo to check it still works
cargo run -- -i timer-proxy-opt.wasm
```

This is a 15% binary size reduction to barely over 1kb!!