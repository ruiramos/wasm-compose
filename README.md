WASM Components
==========

To compose and run:

```
  wasm-tools compose -c config.yml -o composed.wasm hello-runner/target/wasm32-wasi/release/hello-runner.wasm
  wasmtime --wasm component-model composed.wasm
```
