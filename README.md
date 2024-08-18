⚠️ WIP – does not work at the moment, help needed

This repo is an tttempt to compile unmodified [tree-sitter](https://crates.io/crates/tree-sitter) to WebAssembly based on discussion in https://github.com/tree-sitter/tree-sitter/discussions/1550. Alternative approach is to use [https://github.com/shadaj/tree-sitter-c2rust](tree-sitter-c2rust).

- Repo setup is based on https://github.com/jrieken/rust-wasm
- `src/wasm_libc.rs` is copied from https://gist.github.com/nicx256/c86f172cece6370b6d503fa440473896
- `wasm/sysroot` is copied from https://github.com/cacticouncil/lilypad/tree/main/wasm-sysroot.

Trying to compile with `rustc 1.82.0-nightly` and `wasm-pack 0.13.0` fails with an error:

```
CFLAGS_wasm32_unknown_unknown="-I$(pwd)/wasm-sysroot" RUSTFLAGS="-Zwasm-c-abi=spec" wasm-pack build --target web --dev
```

```
warning: tree-sitter@0.22.6: In file included from /Users/agentcooper/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tree-sitter-0.22.6/src/lib.c:3:
warning: tree-sitter@0.22.6: In file included from /Users/agentcooper/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tree-sitter-0.22.6/src/./alloc.c:1:
warning: tree-sitter@0.22.6: /Users/agentcooper/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tree-sitter-0.22.6/src/./alloc.h:18:45: error: a parameter list without types is only allowed in a function definition
warning: tree-sitter@0.22.6:    18 | TS_PUBLIC extern void *(*ts_current_malloc)(size_t);
```
