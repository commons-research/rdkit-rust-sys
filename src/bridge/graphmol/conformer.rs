#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("rdkit-rust-ffi/wrapper/graphmol/conformer.h");
        type Conformer;
    }
}
