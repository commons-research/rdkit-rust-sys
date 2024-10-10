use rdkit_rust_ffi::graphmol::ROMolExt;
use std::{borrow::BorrowMut, ops::Deref};

use cxx::{let_cxx_string, memory::SharedPtrTarget, SharedPtr, UniquePtr};

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

    let carbon = mol.getAtomWithIdx(0);

    assert_eq!(carbon.getIdx().unwrap(), 0);
    assert_eq!(carbon.getAtomicNum(), 6);

    let oxygen = mol.getAtomWithIdx(2);
    assert_eq!(oxygen.getIdx().unwrap(), 2);
}

#[test]
fn test_get_mutable_atom_with_idx() {
    let_cxx_string!(smiles = "CCO");
    let mut mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles).unwrap();

    let carbon = mol.pin_mut().getMutableAtomWithIdx(0);
    assert_eq!(carbon.getIdx().unwrap(), 0);
    assert_eq!(carbon.getAtomicNum(), 6);

    let oxygen = mol.pin_mut().getMutableAtomWithIdx(2);
    assert_eq!(oxygen.getIdx().unwrap(), 2);
}
