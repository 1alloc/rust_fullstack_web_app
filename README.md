# PROJECT IS ABANDONED DUE NEW VERSIONS OF FRAMEWORKS AND MY LAZINESS TO UPDATE IT

I'll try from scratch

### CLIENT SIDE:

Run app

If you haven't already, install Trunk:

```
cargo install trunk wasm-bindgen-cli
```

If you haven't already installed it, you need to add the wasm32-unknown-unknown target. To install this with Rustup:

```
rustup target add wasm32-unknown-unknown
```

Now all you have to do is run the following:

```
trunk serve
```

This will start a development server which continually updates the app every time you change something.


Production:

```
trunk build
```

trunk build runs a cargo build targeting the wasm32 instruction set, runs wasm-bindgen on the built WASM, and spawns asset build pipelines for any assets defined in the target index.html.

Trunk leverages Rust's powerful concurrency primitives for maximum build speeds & throughput.

https://github.com/thedodd/trunk

TODO:

Locales - https://lib.rs/crates/i18n_codegen - DONE

https://github.com/yewstack/yew_router - active nav class

https://docs.rs/cookie/0.15.0/cookie/


Communicate with server side - https://github.com/yewstack/yew/issues/1385

api routes exception

https://yew.rs/concepts/services/fetch

reqwest - https://crates.io/crates/reqwest


# SERVER SIDE

Start server
```bash
cargo run
```
