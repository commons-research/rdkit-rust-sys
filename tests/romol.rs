use rdkit_rust_ffi::graphmol::ROMolExt;

use cxx::let_cxx_string;

#[test]
fn test_smiles_to_mol() {
    let_cxx_string!(smiles = "CCO");
    let mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles);
    assert!(mol.is_ok());
}

#[test]
fn test_get_atom_with_idx() {
    let_cxx_string!(smiles = "CCO");
    let mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles).unwrap();

    let carbon = mol.getAtomWithIdx(0).unwrap();

    assert_eq!(carbon.getIdx().unwrap(), 0);
    assert_eq!(carbon.getAtomicNum(), 6);

    let oxygen = mol.getAtomWithIdx(2).unwrap();
    assert_eq!(oxygen.getIdx().unwrap(), 2);
}

#[test]
fn test_get_atom_with_idx_fail() {
    let_cxx_string!(smiles = "CCO");
    let mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles).unwrap();

    let carbon = mol.getAtomWithIdx(10);
    assert!(carbon.is_err());
}

#[test]
fn test_get_mutable_atom_with_idx() {
    let_cxx_string!(smiles = "CCO");
    let mut mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles).unwrap();

    let carbon = mol.pin_mut().getMutableAtomWithIdx(0).unwrap();
    assert_eq!(carbon.getIdx().unwrap(), 0);
    assert_eq!(carbon.getAtomicNum(), 6);

    let oxygen = mol.pin_mut().getMutableAtomWithIdx(2).unwrap();
    assert_eq!(oxygen.getIdx().unwrap(), 2);
}

#[test]
fn test_get_num_atoms() {
    let_cxx_string!(smiles = "CCO");
    let mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles).unwrap();
    assert_eq!(mol.getNumAtoms(true), 3);
    assert_eq!(mol.getNumAtoms(false), 9);
}

#[test]
fn test_get_num_bonds() {
    let_cxx_string!(smiles = "CCO");
    let mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles).unwrap();
    assert_eq!(mol.getNumBonds(true), 2);
    assert_eq!(mol.getNumBonds(false), 8);
}

#[test]
fn get_num_conformers() {
    let_cxx_string!(smiles = "CCO");
    let mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles).unwrap();
    assert_eq!(mol.getNumConformers(), 0);
}

#[test]
fn num_heavy_atoms() {
    let_cxx_string!(smiles = "CCO");
    let mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles).unwrap();
    assert_eq!(mol.getNumHeavyAtoms(), 3);
}
