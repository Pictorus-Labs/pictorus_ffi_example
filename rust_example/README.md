# Rust Example

This directory contains a simple example that shows how to call a Pictorus app from a Rust binary. In order to accomplish this we need to link against the shared static library provided in the [lib/](../lib/) directory. In addition, we need to create Rust bindings to the header file in the same directory. This is done as part of the [build script](build.rs), where the bindings are automatically generated using the [bindgen](https://crates.io/crates/bindgen) crate. An example of the generated Rust bindings is also included in [src/bindings.rs](src/bindings.rs) for reference.

To build and run the example, simply run `cargo run` from this directory. You should see an output similar to the following:

```bash
Starting app
app_time_s: 0.0, output_ffi: -0.000
app_time_s: 0.1, output_ffi: -0.026
app_time_s: 0.2, output_ffi: 0.510
app_time_s: 0.3, output_ffi: -0.037
app_time_s: 0.4, output_ffi: 0.678
app_time_s: 0.5, output_ffi: 0.713
app_time_s: 0.6, output_ffi: 0.087
app_time_s: 0.7, output_ffi: -0.079
app_time_s: 0.8, output_ffi: -0.115
app_time_s: 0.9, output_ffi: 0.571
app_time_s: 1.0, output_ffi: -0.771
Done running app
```
