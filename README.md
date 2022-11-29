# Doomé

Doom + Cliché = Doomé; a game, made for playing.

## Running

Requirements:

- rustup,
- spirv-tools.

``` shell
$ git clone https://github.com/Patryk27/doome
$ cd doome
$ cargo build-shaders

# ... and then:
$ cargo run-app

# ... or:
$ cargo run-wasm
```

## Building

### Web

``` shell
$ cargo build-wasm
$ wasm-bindgen --out-dir ./web --target web ./target/wasm/wasm32-unknown-unknown/release/doome.wasm
```
