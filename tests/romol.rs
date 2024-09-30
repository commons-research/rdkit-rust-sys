use cxx::let_cxx_string;

#[test]
fn test_smiles_to_mol() {
    let_cxx_string!(smiles = "CCO");
    let mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles);
    assert!(mol.is_ok());
}

#[test]
fn test_get_atom_degree() {
    let_cxx_string!(smiles = "CCO");
    let mol = rdkit_rust_ffi::graphmol::romol_ffi::newMolFromSmiles(&smiles).unwrap();
    // let atom = mol.getAtom(0);
    // assert_eq!(mol.getAtomDegree(&atom), 2);
}
