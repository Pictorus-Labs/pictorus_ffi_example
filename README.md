# FFI Examples

This repository contains examples of how to use the FFI (Foreign Function Interface) in different programming languages to interface with a Pictorus app. See the [Interfacing with Existing Code](https://docs.pictor.us/features/interfacing_with_existing_code.html) section in the Pictorus documentation for more details. Open the relevant example directory to see the instructions for that language.

## Requirements

- The static library in this example is compiled for x86 Linux.
- C examples require `gcc` to compile the example.
- Rust examples require `cargo` to compile the example.

## Example App

The Pictorus app used to generate the output in these examples is a simple app that accepts a single input, multiplies it by a random number, and returns the result.

## Project layout

- [lib/](lib/) This directory contains the shared static library and header file needed to link against the exported Pictorus app.
- [c_example/](c_example/) This directory contains an example that shows how to call a Pictorus app from a C program.
- [rust_example/](rust_example/) This directory contains an example that shows how to call a Pictorus app from a Rust program.
