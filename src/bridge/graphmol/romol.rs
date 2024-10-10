use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
    pin::{pin, Pin},
};

use cxx::{SharedPtr, UniquePtr};
use ffi::{Atom, ROMol};

#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("rdkit-rust-ffi/wrapper/graphmol/romol.h");
        type ROMol;
        type Atom = crate::graphmol::atom_ffi::Atom;
        fn newMolFromSmiles(smiles: &CxxString) -> Result<UniquePtr<ROMol>>;
        fn newMolFromSmilesAsSharedPtr(smiles: &CxxString) -> Result<SharedPtr<ROMol>>;
        fn clearAllAtomBookmarks(self: Pin<&mut ROMol>);
        fn clearAllBondBookmarks(self: Pin<&mut ROMol>);
        fn clearComputedProps(self: &ROMol, include_rings: bool);
        fn clearConformers(self: Pin<&mut ROMol>);

        /// Returns the atom with a molecule at a given index.
        fn getAtomWithIdx(mol: &ROMol, idx: u32) -> Pin<&Atom>;

        /// Returns the atom with a molecule at a given index in a mutable molecule.
        fn getMutableAtomWithIdx(mol: Pin<&mut ROMol>, idx: u32) -> Pin<&mut Atom>;

        /// Returns the degree of an atom in a molecule.
        /// The atom needs to be owned by the molecule.
        fn getAtomDegree(mol: &ROMol, atom: &Atom) -> u32;

    }
}

impl ROMol {
    pub fn getAtomDegree(&self, atom: &crate::graphmol::atom_ffi::Atom) -> u32 {
        ffi::getAtomDegree(self, atom)
    }
}

pub trait ROMolExt {
    fn getAtomWithIdx(&self, idx: u32) -> Pin<&Atom>;
    fn getMutableAtomWithIdx(self: Pin<&mut Self>, idx: u32) -> Pin<&mut Atom>;
}

impl<'a> ROMolExt for ROMol {
    fn getAtomWithIdx(&self, idx: u32) -> Pin<&Atom> {
        ffi::getAtomWithIdx(self, idx)
    }
    fn getMutableAtomWithIdx(self: Pin<&mut Self>, idx: u32) -> Pin<&mut Atom> {
        ffi::getMutableAtomWithIdx(self, idx)
    }
}
