# Wasm IO 2024 Claw Demo

## Setup

```sh
cargo install claw-cli
cargo install component-opt
cargo install --git https://github.com/peterhuene/wac --locked
cargo build
```

## CI Jobs

```sh
# Look at `job` world in wit/claw.wit

# Look at src/main.rs

# Test out an empty job
claw-cli compile -i programs/empty-job.claw --wit wit -o empty-job.wasm
cargo run -- -i empty-job.wasm

# Test out a basic job
claw-cli compile -i programs/basic-job.claw --wit wit -o basic-job.wasm
cargo run -- -i basic-job.wasm

# Test out a simple proxy and composition
claw-cli compile -i programs/timer-proxy.claw --wit wit -o timer-proxy.wasm
wac encode -d claw:basic-job=basic-job.wasm -d claw:timer-proxy=timer-proxy.wasm -o job-with-timer.wasm programs/job-with-timer.wac
```

## Component Optimization

```sh
# Print file size (expect 1212 bytes)
ls -l timer-proxy.wasm

# Print optimized file size (expect 1064)
component-opt -i timer-proxy.wasm -o timer-proxy-opt.wasm
ls -l timer-proxy-opt.wasm

# Rerun the demo to check it still works
wac encode -d claw:basic-job=basic-job.wasm -d claw:timer-proxy=timer-proxy-opt.wasm -o job-with-timer-opt.wasm programs/job-with-timer.wac
cargo run -- -i job-with-timer-opt.wasm
```

This is a 15% binary size reduction to barely over 1kb!!