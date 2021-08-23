# Logging in a TUI app in Rust
This repository contains: 
* a library providing a [log](https://docs.rs/log/0.4.14/log/) ging implementation based on a `writable` trait
* the example folder containing an implementation of a [TUI](https://docs.rs/tui/0.16.0/tui/index.html) widget that implements the `writable` trait

In total this means you can render your widget with with tui and use the [log](https://docs.rs/log/0.4.14/log/) interface methods: `log`, `debug`, `error` etc.