# Wasm IO 2024 Claw Demo

## Setup

```sh
cargo install claw-cli
cargo install component-opt
cargo install --git https://github.com/peterhuene/wac --locked
cargo build
```

## Basic

```sh
# Compile and run empty job
claw-cli compile -i ./programs/empty-job.claw --wit wit -o empty-job.wasm
cargo run -- -i empty-job.wasm

# Compile and run basic job
claw-cli compile -i ./programs/basic-job.claw --wit wit -o basic-job.wasm
cargo run -- -i basic-job.wasm

# Compile the timer-proxy, compose it, and run
claw-cli compile -i ./programs/timer-proxy.claw --wit wit -o timer-proxy.wasm
wac encode -d claw:basic-job=basic-job.wasm -d claw:timer-proxy=timer-proxy.wasm -o job-with-timer.wasm programs/job-with-timer.wac

plugin-with-timer.wac
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