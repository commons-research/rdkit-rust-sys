use ffi::ROMol;

#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("rdkit-rust-ffi/wrapper/graphmol/romol.h");
        type ROMol;
        type Atom = crate::graphmol::atom_ffi::Atom;
        fn newMolFromSmiles(smiles: &CxxString) -> Result<UniquePtr<ROMol>>;
        fn clearAllAtomBookmarks(self: Pin<&mut ROMol>);
        fn clearAllBondBookmarks(self: Pin<&mut ROMol>);
        fn clearComputedProps(self: &ROMol, include_rings: bool);
        fn clearConformers(self: Pin<&mut ROMol>);
        // fn getAtomWithIdx(self: &ROMol, idx: u32) -> UniquePtr<Atom>;
        fn getAtomDegree(mol: &ROMol, atom: &Atom) -> u32;

    }
}

impl ROMol {
    pub fn getAtomDegree(&self, atom: &crate::graphmol::atom_ffi::Atom) -> u32 {
        ffi::getAtomDegree(self, atom)
    }
}
