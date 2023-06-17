## Boids with WGPU & WASM

This project is a rust implementation of the [WebGPU boids sample](https://webgpu.github.io/webgpu-samples/samples/computeBoids).

### How to run

#### Native

You can run the project natively with cargo:

```bash
cargo run
```

#### Web

You can also run the project in the browser with WASM:

```bash
wasm-pack build --target web --out-dir client/pkg
cd client
yarn install
yarn dev
```
