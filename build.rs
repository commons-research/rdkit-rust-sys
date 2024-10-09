use std::error::Error;

use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    env_logger::init();
    let use_conda = dotenvy::var("CARGO_FEATURE_DYNAMIC_LINKING_FROM_CONDA").is_ok();
    println!("use_conda: {}", use_conda);

    let mut lib_paths = vec![];
    let mut include_paths = vec![];

    match (std::env::consts::OS, std::env::consts::ARCH, use_conda) {
        (_, _, true) => {
            // prefer the prefix env var, if not, fall back to the base from the CLI
            match dotenvy::var("CONDA_PREFIX") {
                Ok(prefix) => {
                    include_paths.push(format!("{prefix}/include"));
                    include_paths.push(format!("{prefix}/include/rdkit"));
                    lib_paths.push(format!("{prefix}/lib"));
                }
                Err(_) => {
                    let conda = which::which("conda")
                        .map(|p| p.to_str().unwrap().to_string())
                        .unwrap_or_else(|_| panic!("conda not found"));
                    let mut conda = std::process::Command::new(conda);
                    conda.args(["info", "--base"]);

                    let output = conda.output().unwrap();
                    let stdout = String::from_utf8(output.stdout).unwrap();
                    let conda_root = stdout.trim().to_string();

                    lib_paths.push(format!("{}/lib", conda_root));
                    include_paths.push(format!("{}/include", conda_root));
                    include_paths.push(format!("{}/include/rdkit", conda_root));
                }
            }
        }
        ("macos", "x86_64", _) => {
            include_paths.push("/usr/local/include".to_string());
            include_paths.push("/usr/local/include/rdkit".to_string());
        }
        ("macos", "aarch64", _) => {
            include_paths.push("/opt/homebrew/include".to_string());
            include_paths.push("/opt/homebrew/include/rdkit".to_string());
            lib_paths.push("/opt/homebrew/lib".to_string())
        }
        ("linux", _, _) => {
            include_paths.push("/usr/local/include".to_string());
            include_paths.push("/usr/local/include/rdkit".to_string());
            include_paths.push("/usr/include".to_string());
            include_paths.push("/usr/include/rdkit".to_string());
        }
        (unsupported_os, unsupported_arch, use_conda) => panic!(
            "sorry, rdkit-sys doesn't support {}/{}/use_conda={} at this time",
            unsupported_os, unsupported_arch, use_conda
        ),
    };

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
        .includes(include_paths)
        .warnings(false)
        .compile("rdkit_rust_ffi");

    // wehave compiled rdkit in our system we now want to tell rust where the
    // dylib file are
    for path in lib_paths {
        println!("cargo:rustc-link-search=native={}", path);
    }

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
