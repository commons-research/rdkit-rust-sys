use std::ops::Deref;

use cxx::let_cxx_string;
use rdkit_rust_ffi::graphmol::atom_ffi;

#[test]
fn test_new_atom_from_atomic_num() {
    let atom = atom_ffi::newAtomFromAtomicNum(4);
    assert_eq!(atom.getAtomicNum(), 4);
}

#[test]
fn test_init_atom_from_symbol() {
    let_cxx_string!(symbol = "C");
    let atom = atom_ffi::newAtomFromSymbol(&symbol);
    assert_eq!(atom.getSymbol(), "C");
}

#[test]
fn test_get_symbol() {
    let atom = atom_ffi::newAtomFromAtomicNum(6);
    assert_eq!(atom.getSymbol(), "C");
}

#[test]
fn test_set_and_get_atomic_num() {
    let mut atom = atom_ffi::newAtom();

    // convert atom which is a UniquePtr<Atom> to Pin<&mut Atom>
    atom.pin_mut().setAtomicNum(7);
    assert_eq!(atom.getAtomicNum(), 7);

    assert_eq!(atom.hasOwningMol(), false);
}

#[test]
fn test_match() {
    let atom1 = atom_ffi::newAtomFromAtomicNum(6);
    let atom2 = atom_ffi::newAtomFromAtomicNum(6);
    assert_eq!(atom1.Match(&atom2), true);
}

#[test]
fn test_needsUpdatePropertyCache() {
    let atom = atom_ffi::newAtomFromAtomicNum(6);
    assert_eq!(atom.needsUpdatePropertyCache(), true);
}

#[test]
fn test_fail_calc_explicit_valence() {
    let mut atom = atom_ffi::newAtomFromAtomicNum(6);
    let result = atom.pin_mut().calcExplicitValence(true);
    // check that the result is an error
    assert!(result.is_err());
}

#[test]
fn test_copy_of_atom() {
    let atom = atom_ffi::newAtomFromAtomicNum(6);
    let atom_copy = atom_ffi::newAtomFromOther(atom.deref());
    assert_eq!(atom.Match(atom_copy.deref()), true);
    assert_eq!(atom.getAtomicNum(), atom_copy.getAtomicNum());
}

#[test]
fn test_set_chiral_tag() {
    let mut atom = atom_ffi::newAtomFromAtomicNum(6);
    atom.pin_mut()
        .setChiralTag(atom_ffi::ChiralType::CHI_TETRAHEDRAL);

    assert_eq!(atom.getChiralTag(), atom_ffi::ChiralType::CHI_TETRAHEDRAL);
}

#[test]
fn test_get_atomic_map_num() {
    let mut atom = atom_ffi::newAtomFromAtomicNum(6);
    atom.pin_mut().setAtomMapNum(1, true);
    assert_eq!(atom.getAtomMapNum(), 1);
}

#[test]
fn test_getFormalCharge() {
    let atom = atom_ffi::newAtomFromAtomicNum(6);
    assert_eq!(atom.getFormalCharge(), 0);
}

#[test]
fn test_getIsAromatic() {
    let atom = atom_ffi::newAtomFromAtomicNum(6);
    assert_eq!(atom.getIsAromatic(), false);
}

#[test]
fn test_getIsotope() {
    let atom = atom_ffi::newAtomFromAtomicNum(6);
    assert_eq!(atom.getIsotope(), 0);

    let mut atom = atom_ffi::newAtomFromAtomicNum(6);
    atom.pin_mut().setIsotope(13);
    assert_eq!(atom.getIsotope(), 13);
}

#[test]
fn test_getMass() {
    let atom = atom_ffi::newAtomFromAtomicNum(6);
    assert_eq!(atom.getMass(), 12.011);
}

#[test]
fn test_hasQuery() {
    let atom = atom_ffi::newAtomFromAtomicNum(6);
    assert_eq!(atom.hasQuery(), false);
}
