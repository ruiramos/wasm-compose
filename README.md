WASM Components
==========

Trying to run `hello-runner` while having the `hello-world` component provide the `hello_world()` function.

Running
```
  wasm-tools compose -c config.yml -o composed.wasm hello-runner/target/wasm32-wasi/release/hello-runner.wasm
```
fails with:
```
  error: component `hello-runner/target/wasm32-wasi/release/hello-runner.wasm` has a non-instance import named `hello-world`
```
