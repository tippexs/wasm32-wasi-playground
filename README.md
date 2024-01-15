# Deliverables

- [ ] Unit with wasm-wasi support for snapshot 18-10-2023 including build steps
    Build the module as a patch from 1.31.1! 
- [ ] Build steps for wasm component binary - power.rs -> power-component.rs
- [ ] Unit example configuration
- [ ] wasmtime serve command

# Rust Ecosystem

## Installation of `cargo component`
Note: By now, this is an optional step and not needed to work with this tutorial.
Make sure `libssl-dev` and `pkg-config` is installed on your system


# The Rust Project

The project source code can be found in the `application` directory and was inspired by
[Fermyon Spins HTTP Example](https://github.com/fermyon/spin/blob/v2.1/examples/http-rust/src/lib.rs)

Navigate into `application`

To build the Wasm binary issue
```bash
cargo build --target wasm32-wasi --release
```
This will generate the Wasm binary. To port this into a WebAssembly Component run

```bash
wasm-tools component new target/wasm32-wasi/release/application.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm -o app-component.wasm
```

wasm-tools can be installed using cargo as well.
```bash
cargo install wasm-tools
```

Now you should see a `app-component.wasm` in your `application` directory. This component can now be hosted on wasmtime-v14 AND NGINX Unit.

# Run on wasmtime

To run this component on wasmtime V14 issue
```
wasmtime serve app-component.wasm --addr 127.0.0.1:9090
```

The `--addr` is optional. Per default wasmtime will use port `8080`. If thats good enough, there is no need to define `--addr`.

# Run on NGINX Unit

To run this component on NGINX Unit with the wasm-wasi-http module use the following configuration

```json
{
      "listeners": {
          "127.0.0.1:8085": {
              "pass": "applications/wasm"
          }
      },

      "applications": {
          "wasm": {
              "type": "wasm-wasi-http",
              "component": "/path/to/power-spin-rs/application/app-component.wasm"
          }
      }
  }
```



