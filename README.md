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


# Create the Unit Language Module for 1.31.1

Checkout and Clone the sources from 1.31.1

```bash
git clone -b 1.31.1 https://github.com/nginx/unit
```

Apply the patch
```bash
git apply wasm-wasi-http.diff
```

Configure Unit and the Module. Make sure Unit is configured with the same configure command as the current package.
```bash
./configure
make
```

Pre-Req for the cargo steps.

```bash
make build/src/nxt_unit.o
```

Build the wasm-wasi-http module
```bash
cargo build --release --manifest-path src/wasm-wasi-http/Cargo.toml
```

This will build the Unit WebAssembly Wasi Language Module. This can now be copied into Units module directory. Make sure the target module name is correct.
In the Unit source directory issue

```
cp src/wasm-wasi-http/target/release/libnxt_wasmtime.so /usr/lib/unit/modules/dir/wasmtime.unit.so
```

The next time Unit starts it will pick up the wasmtime language module.


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



