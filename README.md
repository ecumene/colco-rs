# Colco (colcothar) - 3D molecule viewer via glow-rs

# How to Build

## Native (in-dev...)

TBD

## Web

`cd` to `colco` directory

Currently only stdweb is supported. To run with stdweb:

```shell
cargo web start --no-default-features --features stdweb --target wasm32-unknown-unknown
```

To minify and deploy to static files for production, run:

```shell
cargo web deploy --release --no-default-features --features stdweb --target wasm32-unknown-unknown
```
