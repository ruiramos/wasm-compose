WASM Components
==========

## Build, Compose, Run

1. From the components directories
```
  cargo component build --release
```

2. From the project main dir, componse and run;

```
  wasm-tools compose -c config.yml -o composed.wasm hello-runner/target/wasm32-wasi/release/hello-runner.wasm
  wasmtime --wasm component-model composed.wasm
```
