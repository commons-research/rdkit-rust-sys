use ffi::Atom;

#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {

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
        include!("rdkit-rust-ffi/wrapper/graphmol/atom.h");
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

        /// Requires an owning molecule. If the atom is not owned by a molecule,
        /// this will throw an exception.
        fn calcExplicitValence(self: Pin<&mut Atom>, strict: bool) -> Result<i32>;
        fn getExplicitValence(self: &Atom) -> Result<i32>;

        /// Requires an owning molecule. If the atom is not owned by a molecule,
        /// this will throw an exception.
        fn calcImplicitValence(self: Pin<&mut Atom>, strict: bool) -> Result<i32>;
        fn getImplicitValence(self: &Atom) -> Result<i32>;

        /// Set the atom map Number of the atom.
        fn setAtomMapNum(self: Pin<&mut Atom>, mapno: i32, strict: bool);
        pub fn setChiralTag(self: Pin<&mut Atom>, tag: ChiralType);
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

impl Atom {
    pub fn getSymbol(&self) -> String {
        ffi::getSymbolAsString(self)
    }

    pub fn Match(&self, other: &Atom) -> bool {
        ffi::MatchRust(self, ffi::newAtomFromOther(other))
    }

    pub fn getQueryType(&self) -> String {
        ffi::getQueryTypeRust(self)
    }
}
