# Intro

# Cargo features

## Installing rustup

`curl https://sh.rustup.rs -sSf | sh`

[Windows installer](https://www.rust-lang.org/tools/install)

## Rustup doc

`rustup doc`

## Toolchains

Default toolchain is `stable`. To try nightly:

`rustup install nightly`

## Code formatting and static analysis

`rustup component add clippy`

`rustup component add rustfmt`

## Cargo commands

### Project scaffolding

Binary, or and "app" (`--bin` implied)

`cargo new`

Library

`cargo new --lib`

### Dependency management

[Crates.io](https://crates.io)

NPM-like commands as a plugin

`cargo install cargo-edit`

> Let's talk about optional features

### Offline docs

`cargo doc`

# Editors and language server

## Language server spec and Rust

[Rust RLS](https://github.com/rust-lang/rls)

[VS Code extension](https://github.com/rust-lang/rls-vscode)

# Rust language features

> See `language_features` subfolder in this repo

