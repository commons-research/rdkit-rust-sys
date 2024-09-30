use std::error::Error;

use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    // load the RDKIT_DIR
    let rdkit_dir = dotenvy::var("RDKIT_DIR")?;
    let rdkit_lib = format!("{}/lib", rdkit_dir);
    let rdkit_include = format!("{}/Code", rdkit_dir);

    println!("cargo:rerun-if-changed=src/lib.rs");

    // now we recursively get all the files in the bridge directory that are not
    // mod.rs but that are rust files
    // Use WalkDir to recursively find .rs files excluding mod.rs
    let rust_files: Vec<_> = WalkDir::new("src/bridge")
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension().map(|ext| ext == "rs").unwrap_or(false) {
                if !path.ends_with("mod.rs") {
                    Some(path.to_path_buf())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // now we do the same for the c++ files
    let cpp_files: Vec<_> = WalkDir::new("wrapper")
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension().map(|ext| ext == "cc").unwrap_or(false) {
                Some(path.to_path_buf())
            } else {
                None
            }
        })
        .collect();

    for file in &cpp_files {
        println!("cargo:rerun-if-changed=wrapper/{}", file.display());
    }

    // and now the same with the header files
    let header_files: Vec<_> = WalkDir::new("wrapper")
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension().map(|ext| ext == "h").unwrap_or(false) {
                Some(path.to_path_buf())
            } else {
                None
            }
        })
        .collect();

    for file in &header_files {
        println!("cargo:rerun-if-changed=wrapper/{}", file.display());
    }

    cxx_build::bridges(rust_files)
        .files(cpp_files)
        .std("c++17")
        .includes(vec![rdkit_include])
        .warnings(false)
        .compile("rdkit_rust_ffi");

    // wehave compiled rdkit in our system we now want to tell rust where the
    // dylib file are
    println!("cargo:rustc-link-search=native={}", rdkit_lib);

    // we also want to tell rust to link the rdkit dylib file
    for lib in &[
        "Catalogs",
        "ChemReactions",
        "ChemTransforms",
        "CIPLabeler",
        "DataStructs",
        "Depictor",
        "Deprotect",
        "Descriptors",
        "DistGeometry",
        "EigenSolvers",
        "FileParsers",
        "Fingerprints",
        "ForceField",
        "ForceFieldHelpers",
        "GenericGroups",
        "GraphMol",
        "MolStandardize",
        "MolTransforms",
        "FragCatalog",
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

    Ok(())
}
