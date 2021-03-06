# Rust Firebird Client 

![Build](https://github.com/fernandobatels/rsfbclient/workflows/testing_changes/badge.svg)
[![Crate](https://img.shields.io/crates/v/rsfbclient.svg)](https://crates.io/crates/rsfbclient)
[![API](https://docs.rs/rsfbclient/badge.svg)](https://docs.rs/rsfbclient)

A [Firebird](https://firebirdsql.org/) client library to [Rust programing language](https://rust-lang.org/)

## How to use it

1. Start choosing the lib variation you want
```rust
// For use the compile time linked version
ConnectionBuilder::linked()
// Or using the runtime loaded lib
ConnectionBuilder::with_client("/my/firebird/here/lib/libfbclient.so")
// Or using the pure rust implementation
ConnectionBuilder::pure_rust()
```

2. Set your connection params
```rust
// For a remote server
let mut conn = ConnectionBuilder::linked()
    .host("my.host.com.br")
    .db_name("awesome.fdb")
    .connect()?
// Or if you need a embedded/local only access
let mut conn = ConnectionBuilder::linked()
    .embedded()
    .db_name("/path/to/awesome.fdb")
    .connect()?
```

3. Now you can use the lib
```rust
let rows = conn.query_iter("select col_a, col_b, col_c from test", ())?;
...
```

More examples [here](https://github.com/fernandobatels/rsfbclient/tree/master/examples).

## Cargo features
All features can be used at the same time if needed.

### `linking`
Will use the dynamic library of the official `fbclient` at runtime and compiletime. Used in systems where there is already a firebird client installed and configured.
### `dynamic_loading`
Can find the official `fbclient` native library by path at runtime, does not need the library at compiletime. Useful when you need to build in a system without a firebird client installed.
### `pure_rust`
Uses a pure rust implementation of the firebird wire protocol, does not need the native library at all. Useful for cross-compilation and allow a single binary to be deployed without needing to install the firebird client.

## Goals 

- [x] Rust Api
- [x] Dynamic link with fbclient
- [x] Dynamic loading the fbclient(.dll or .so)
- [x] ARM support
- [x] Firebird embedded support
- [x] Extern this [api to ruby](https://github.com/fernandobatels/rbfbclient)
- [ ] Extern this api to lua (in a new repo)
