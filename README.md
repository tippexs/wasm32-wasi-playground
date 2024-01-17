# Deliverables

- [x] Unit with wasm-wasi support for snapshot 18-10-2023 including build steps
      Build the module as a patch for 1.31.1!
      Build mdoules for Linux and MacOS targets
- [x] Build steps for wasm component binary - power.rs -> power-component.rs
- [x] Unit example configuration
- [x] wasmtime serve command

# Rust Ecosystem

## Installation of `cargo component`
Cargo components is an alternative to the steps shown below to create a Wasm Component from a Wasm binary.  

Note: By now, this is an optional step and not needed to work with this tutorial.
Make sure `libssl-dev` and `pkg-config` is installed on your system.

To install `cargo-component` issue
```bash
cargo install cargo-component
```

# Create the Unit Language Module for 1.31.1

Checkout and Clone the sources from 1.31.1

```bash
git clone -b 1.31.1 https://github.com/nginx/unit
```

Apply the patch
```bash
git apply unit-wasi.diff
```

Configure Unit and the Module. Make sure Unit is configured with the same configure command as the current package.
```bash
./configure --prefix=$PWD/build
make
```

Now, we can build the Unit WASI-HTTP Language module
```bash
./configure wasm-wasi-http
```

The output of the commands mentioned above should look like

```bash
configuring wasm-wasi-http module
looking for rust compiler ... found.
looking for cargo ... found.
 + wasm-wasi-http module: wasm_wasi_http.unit.so
```
Issue make to to start the build process of the Unit WASI-HTTP Language Module
```bash
make
```

The Module will be available in `src/wasm-wasi-http/target/release/`. Copy the Module into Units Modules directory to use it.

The next time Unit starts it will pick up the wasmtime language module.


# The Rust Project

The project source code can be found in the `application` directory and was inspired by
[Fermyon Spins HTTP Example](https://github.com/fermyon/spin/blob/v2.1/examples/http-rust/src/lib.rs)

Navigate into `application`

## Build using `wasm-tools`

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

## Build using `cargo component` instead of  `wasm-tools`
More Information check [Zulip Chat](https://bytecodealliance.zulipchat.com/#narrow/stream/407292-cargo-component/topic/.E2.9C.94.20Create.20Component.20from.20Rust.20Example)

Another way to build the component is to use `cargo component`. As mentioned above `cargo-component` must be installed first.

To build the Wasm component in a single build step without having the need of `wasm-tools` type
```bash
cargo component build --release
```
The Wasm component will be availabe as `target/wasm32-wasi/release/application.wasm`

# Run on wasmtime

To run this component on wasmtime V14 issue
```
wasmtime serve application.wasm --addr 127.0.0.1:9090
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
              "component": "/path/to/power-spin-rs/application/application.wasm"
          }
      }
  }
```



