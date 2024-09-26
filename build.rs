use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    // load the RDKIT_DIR
    let rdkit_dir = dotenvy::var("RDKIT_DIR")?;
    let rdkit_lib = format!("{}/lib", rdkit_dir);
    let rdkit_include = format!("{}/Code", rdkit_dir);

    // wehave compiled rdkit in our system we now want to tell rust where the
    // dylib file are
    println!("cargo:rustc-link-search=native={}", rdkit_lib);

    // we also want to tell rust to link the rdkit dylib file
    for lib in &[
        "Catalogs",
        "ChemReactions",
        "ChemTransforms",
        "DataStructs",
        "Depictor",
        "Descriptors",
        "FileParsers",
        "Fingerprints",
        "GenericGroups",
        "GraphMol",
        "MolStandardize",
        "MolTransforms",
        "PartialCharges",
        "RDGeneral",
        "RDGeometryLib",
        "RingDecomposerLib",
        "ScaffoldNetwork",
        "SmilesParse",
        "Subgraphs",
        "SubstructMatch",
    ] {
        println!("cargo:rustc-link-lib=dylib=RDKit{}", lib);
    }
    println!("cargo:rustc-link-lib=dylib=boost_serialization");

    cxx_build::bridge("src/lib.rs")
        .file("src/wrapper.cc")
        .std("c++17")
        .include(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .includes(vec![rdkit_include])
        .warnings(false)
        .compile("test");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/wrapper.cc");
    println!("cargo:rerun-if-changed=include/wrapper.h");
    Ok(())
}
