use std::pin::Pin;

use ffi::{Atom, ROMol};

#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {

    unsafe extern "C++" {
        include!("rdkit-rust-ffi/wrapper/graphmol/romol.h");
        type ROMol;
        type Conformer = crate::graphmol::conformer::ffi::Conformer;
        type Atom = crate::graphmol::atom_ffi::Atom;
        fn newMolFromSmiles(smiles: &CxxString) -> Result<UniquePtr<ROMol>>;
        fn newMolFromSmilesAsSharedPtr(smiles: &CxxString) -> Result<SharedPtr<ROMol>>;
        fn clearAllAtomBookmarks(self: Pin<&mut ROMol>);
        fn clearAllBondBookmarks(self: Pin<&mut ROMol>);
        fn clearComputedProps(self: &ROMol, include_rings: bool);
        fn clearConformers(self: Pin<&mut ROMol>);

        /// Returns the atom with a molecule at a given index.
        /// If the index is out of bounds, this will throw an exception.
        fn getAtomWithIdx(mol: &ROMol, idx: u32) -> Result<Pin<&Atom>>;

        fn getAtomWithBookmark(mol: Pin<&mut ROMol>, mark: i32) -> Result<Pin<&mut Atom>>;

        /// Returns the atom with a molecule at a given index in a mutable molecule.
        /// If the index is out of bounds, this will throw an exception.
        #[rust_name = getMutableAtomWithIdx]
        fn getAtomWithIdx(mol: Pin<&mut ROMol>, idx: u32) -> Result<Pin<&mut Atom>>;

        /// Returns the degree of an atom in a molecule.
        /// The atom needs to be owned by the molecule.
        fn getAtomDegree(mol: &ROMol, atom: &Atom) -> u32;

        /// Returns the number of atoms in a molecule.
        fn getNumAtoms(self: &ROMol, only_explicit: bool) -> u32;

        /// Returns the number of bonds in a molecule.
        fn getNumBonds(self: &ROMol, only_heavy: bool) -> u32;

        /// Returns the number of conformers in a molecule.
        fn getNumConformers(self: &ROMol) -> u32;

        /// Returns the number of heavy atoms in a molecule (atomic number > 1).
        fn getNumHeavyAtoms(self: &ROMol) -> u32;

        /// If the Mol has any Query atoms or bonds
        fn hasQuery(self: &ROMol) -> bool;

        fn needsUpdatePropertyCache(self: &ROMol) -> bool;

        /// Return the conformer with a specified ID if the ID is negative the first conformation will be returned
        fn getConformer(self: &ROMol, id: i32) -> &Conformer;

        /// Delete the conformation with the specified ID.
        fn removeConformer(self: Pin<&mut ROMol>, id: u32);

    }
}

impl ROMol {
    pub fn getAtomDegree(&self, atom: &crate::graphmol::atom_ffi::Atom) -> u32 {
        ffi::getAtomDegree(self, atom)
    }
}

pub trait ROMolExt {
    fn getAtomWithIdx(&self, idx: u32) -> Result<Pin<&Atom>, cxx::Exception>;
    fn getMutableAtomWithIdx(
        self: Pin<&mut Self>,
        idx: u32,
    ) -> Result<Pin<&mut Atom>, cxx::Exception>;
}

impl<'a> ROMolExt for ROMol {
    fn getAtomWithIdx(&self, idx: u32) -> Result<Pin<&Atom>, cxx::Exception> {
        ffi::getAtomWithIdx(self, idx)
    }
    fn getMutableAtomWithIdx(
        self: Pin<&mut Self>,
        idx: u32,
    ) -> Result<Pin<&mut Atom>, cxx::Exception> {
        ffi::getMutableAtomWithIdx(self, idx)
    }
}
