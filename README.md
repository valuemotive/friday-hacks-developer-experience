# Friday Hacks: Rust and the developer experience

## About Friday Hacks

Friday Hacks is an informal event that takes place on most Fridays at 1pm, at the Valuemotive office.

Valuemotive has a culture of sharing interesting ideas. Our employees are curious, which leads to ad hoc exchanges. Folks will test the validity of their ideas, ask for feedback on complicated things and discuss their work with others.

We play with an interesting piece of tech and have a discussion about it with an expert present. We often weight on pros and cons of using the tech at work. We always learn something new! You are looking at a demo/boilerplate/guide that was prepared in advance to support the ’Hack.

Examples of previous Friday Hacks: AWS Lambda + Serverless framework, Apache Airflow, Rust, Elm.

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

