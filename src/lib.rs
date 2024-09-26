#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ops::Deref;

use cxx::UniquePtr;
use ffi::Atom;

#[cxx::bridge(namespace = "RDKit")]
mod ffi {

    #[repr(i32)]
    #[derive(Debug, PartialEq)]
    pub enum HybridizationType {
        UNSPECIFIED,
        S,
        SP,
        SP2,
        SP3,
        SP2D,
        SP3D,
        SP3D2,
        OTHER,
    }

    #[repr(i32)]
    #[derive(Debug, PartialEq)]
    pub enum ChiralType {
        CHI_UNSPECIFIED,
        CHI_TETRAHEDRAL_CW,
        CHI_TETRAHEDRAL_CCW,
        CHI_OTHER,
        CHI_TETRAHEDRAL,
        CHI_ALLENE,
        CHI_SQUAREPLANAR,
        CHI_TRIGONALBIPYRAMIDAL,
        CHI_OCTAHEDRAL,
    }

    unsafe extern "C++" {
        include!("rdkit-rust-ffi/include/wrapper.h");
        type Atom;
        type ChiralType;
        type HybridizationType;
        fn make_shared(atom: UniquePtr<Atom>) -> SharedPtr<Atom>;
        fn newAtom() -> UniquePtr<Atom>;
        fn newAtomFromAtomicNum(atomicNum: i32) -> UniquePtr<Atom>;
        fn newAtomFromSymbol(symbol: &CxxString) -> UniquePtr<Atom>;
        fn newAtomFromOther(other: &Atom) -> UniquePtr<Atom>;
        fn getAtomicNum(self: &Atom) -> i32;
        fn getAtomMapNum(self: &Atom) -> i32;
        fn setAtomicNum(self: Pin<&mut Atom>, atomicNum: i32);
        fn getSymbolAsString(atom: &Atom) -> String;
        fn getTotalDegree(self: &Atom) -> u32;
        fn getTotalNumHs(self: &Atom, includeNeighbors: bool) -> u32;
        fn getTotalValence(self: &Atom) -> u32;
        fn hasOwningMol(self: &Atom) -> bool;
        fn hasQuery(self: &Atom) -> bool;
        fn hasValenceViolation(self: &Atom) -> Result<bool>;
        fn invertChirality(self: Pin<&mut Atom>) -> bool;
        fn MatchRust(atom: &Atom, other: UniquePtr<Atom>) -> bool;
        fn needsUpdatePropertyCache(self: &Atom) -> bool;

        /// Requires an owning molecule. If the atom is not owned by a molecule, this will
        /// throw an exception.
        fn calcExplicitValence(self: Pin<&mut Atom>, strict: bool) -> Result<i32>;
        fn getExplicitValence(self: &Atom) -> Result<i32>;

        /// Requires an owning molecule. If the atom is not owned by a molecule, this will
        /// throw an exception.
        fn calcImplicitValence(self: Pin<&mut Atom>, strict: bool) -> Result<i32>;
        fn getImplicitValence(self: &Atom) -> Result<i32>;

        /// Set the atom map Number of the atom.
        fn setAtomMapNum(self: Pin<&mut Atom>, mapno: i32, strict: bool);
        fn setChiralTag(self: Pin<&mut Atom>, tag: ChiralType);
        fn getChiralTag(self: &Atom) -> ChiralType;
        /// Requires an owning molecule
        fn getDegree(self: &Atom) -> Result<u32>;
        fn getFormalCharge(self: &Atom) -> i32;
        fn setFormalCharge(self: Pin<&mut Atom>, charge: i32);
        fn getHybridization(self: &Atom) -> HybridizationType;
        fn setHybridization(self: Pin<&mut Atom>, hyb: HybridizationType);

        /// Returns our index within the ROMol
        fn getIdx(self: &Atom) -> Result<u32>;

        fn getIsAromatic(self: &Atom) -> bool;
        fn setIsotope(self: Pin<&mut Atom>, isotope: u32);
        fn getIsotope(self: &Atom) -> u32;
        fn getMass(self: &Atom) -> f64;
        fn getNoImplicit(self: &Atom) -> bool;
        fn getNumExplicitHs(self: &Atom) -> u32;
        fn getNumImplicitHs(self: &Atom) -> u32;
        /// Requires an owning molecule
        fn getNumRadicalElectrons(self: &Atom) -> Result<u32>;
        // fn getOwningMol(self: &Atom) -> UniquePtr<ROMol>;
        fn getQueryTypeRust(atom: &Atom) -> String;

        /// Sets our index within the ROMol
        fn setIdx(self: Pin<&mut Atom>, idx: u32);
        fn setIsAromatic(self: Pin<&mut Atom>, val: bool);
        fn setNoImplicit(self: Pin<&mut Atom>, val: bool);
        fn setNumExplicitHs(self: Pin<&mut Atom>, val: u32);
        fn setNumRadicalElectrons(self: Pin<&mut Atom>, val: u32);
        // fn setOwningMol(self: Pin<&mut Atom>, mol: UniquePtr<ROMol>);
        fn updatePropertyCache(self: Pin<&mut Atom>, strict: bool);
    }
}

impl crate::ffi::Atom {
    pub fn getSymbol(&self) -> String {
        ffi::getSymbolAsString(self)
    }

    pub fn Match(&self, other: &crate::ffi::Atom) -> bool {
        ffi::MatchRust(self, ffi::newAtomFromOther(other))
    }

    pub fn getQueryType(&self) -> String {
        ffi::getQueryTypeRust(self)
    }
}

#[cfg(test)]
mod tests {
    use cxx::let_cxx_string;

    use super::*;
    use std::pin::Pin;

    #[test]
    fn test_new_atom_from_atomic_num() {
        let atom = ffi::newAtomFromAtomicNum(4);
        assert_eq!(atom.getAtomicNum(), 4);
    }

    #[test]
    fn test_init_atom_from_symbol() {
        let_cxx_string!(symbol = "C");
        let atom = ffi::newAtomFromSymbol(&symbol);
        assert_eq!(atom.getSymbol(), "C");
    }

    #[test]
    fn test_get_symbol() {
        let atom = ffi::newAtomFromAtomicNum(6);
        assert_eq!(atom.getSymbol(), "C");
    }

    #[test]
    fn test_set_and_get_atomic_num() {
        let mut atom = ffi::newAtom();

        // convert atom which is a UniquePtr<Atom> to Pin<&mut Atom>
        atom.pin_mut().setAtomicNum(7);
        assert_eq!(atom.getAtomicNum(), 7);

        assert_eq!(atom.hasOwningMol(), false);
    }

    #[test]
    fn test_match() {
        let atom1 = ffi::newAtomFromAtomicNum(6);
        let atom2 = ffi::newAtomFromAtomicNum(6);
        assert_eq!(atom1.Match(&atom2), true);
    }

    #[test]
    fn test_needsUpdatePropertyCache() {
        let atom = ffi::newAtomFromAtomicNum(6);
        assert_eq!(atom.needsUpdatePropertyCache(), true);
    }

    #[test]
    fn test_fail_calc_explicit_valence() {
        let mut atom = ffi::newAtomFromAtomicNum(6);
        let result = atom.pin_mut().calcExplicitValence(true);
        // check that the result is an error
        assert!(result.is_err());
    }

    #[test]
    fn test_copy_of_atom() {
        let atom = ffi::newAtomFromAtomicNum(6);
        let atom_copy = ffi::newAtomFromOther(atom.deref());
        assert_eq!(atom.Match(atom_copy.deref()), true);
        assert_eq!(atom.getAtomicNum(), atom_copy.getAtomicNum());
    }

    #[test]
    fn test_set_chiral_tag() {
        let mut atom = ffi::newAtomFromAtomicNum(6);
        atom.pin_mut()
            .setChiralTag(ffi::ChiralType::CHI_TETRAHEDRAL);

        assert_eq!(atom.getChiralTag(), ffi::ChiralType::CHI_TETRAHEDRAL);
    }

    #[test]
    fn test_get_atomic_map_num() {
        let mut atom = ffi::newAtomFromAtomicNum(6);
        atom.pin_mut().setAtomMapNum(1, true);
        assert_eq!(atom.getAtomMapNum(), 1);
    }

    #[test]
    fn test_getFormalCharge() {
        let atom = ffi::newAtomFromAtomicNum(6);
        assert_eq!(atom.getFormalCharge(), 0);
    }

    #[test]
    fn test_getIsAromatic() {
        let atom = ffi::newAtomFromAtomicNum(6);
        assert_eq!(atom.getIsAromatic(), false);
    }

    #[test]
    fn test_getIsotope() {
        let atom = ffi::newAtomFromAtomicNum(6);
        assert_eq!(atom.getIsotope(), 0);

        let mut atom = ffi::newAtomFromAtomicNum(6);
        atom.pin_mut().setIsotope(13);
        assert_eq!(atom.getIsotope(), 13);
    }

    #[test]
    fn test_getMass() {
        let atom = ffi::newAtomFromAtomicNum(6);
        assert_eq!(atom.getMass(), 12.011);
    }

    #[test]
    fn test_hasQuery() {
        let atom = ffi::newAtomFromAtomicNum(6);
        assert_eq!(atom.hasQuery(), false);
    }
}
