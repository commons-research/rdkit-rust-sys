# RDKit-Rust-ffi
This is a simple example of how to use RDKit from Rust using the [cxx.rs](https://cxx.rs/).

A first implementation of RDKit in Rust was made [here](https://github.com/rdkit-rs/rdkit) but it was taking to much time to rewrite every function in the wrapper C++ files. It turns out that we can call directly the C++ functions from RDKit and only change the ones that are needed. 

This is an example with the Atom class. Almost all the functions are implemented and this was made in a few hours of work.

## Installation
First you need to install Boost-python: 
```bash
brew install rdkit
```

Then you can run the tests with:
```bash
cargo test
```