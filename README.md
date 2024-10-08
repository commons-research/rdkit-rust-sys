# RDKit-Rust-ffi
This is a simple example of how to use RDKit from Rust using the [cxx.rs](https://cxx.rs/).

A first implementation of RDKit in Rust was made [here](https://github.com/rdkit-rs/rdkit) but it was taking to much time to rewrite every function in the wrapper C++ files. It turns out that we can call directly the C++ functions from RDKit and only change the ones that are needed. 

This is an example with the Atom class. Almost all the functions are implemented and this was made in a few hours of work.

## Installation
First you need to install RDKit. 

```bash
wget https://github.com/rdkit/rdkit/archive/refs/tags/Release_2024_03_6.tar.gz
tar -xvzf Release_2024_03_6.tar.gz
cd rdkit-Release_2024_03_6
mkdir build
cd build
cmake .. -DRDK_BUILD_INCHI_SUPPORT=ON -DRDK_BUILD_PYTHON_WRAPPERS=OFF
make
```

Once this is done you can download this repository and create a `.env` file with the path to the RDKit installation. 
```bash
RDKIT_DIR=/path/to/rdkit-Release_2024_03_6
```

Then you can run the tests with:
```bash
cargo test
```