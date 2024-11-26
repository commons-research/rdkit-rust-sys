#[cxx::bridge(namespace = "RDKit")]
mod ffi {

    #[repr(i32)]
    #[derive(Debug, PartialEq, Clone)]
    enum BondDir {
        NONE = 0,
        BEGINWEDGE,
        BEGINDASH,
        // FIX: this may not really be adequate
        ENDDOWNRIGHT,
        ENDUPRIGHT,
        EITHERDOUBLE,
        UNKNOWN,
    }

    #[repr(i32)]
    #[derive(Debug, PartialEq, Clone)]
    enum BondStereo {
        // stereochemistry of double bonds
        STEREONONE = 0, // no special style
        STEREOANY,      // intentionally unspecified
        // -- Put any true specifications about this point so
        // that we can do comparisons like if(bond->getStereo()>Bond::STEREOANY)
        STEREOZ,        // Z double bond
        STEREOE,        // E double bond
        STEREOCIS,      // cis double bond
        STEREOTRANS,    // trans double bond
        STEREOATROPCW,  //  atropisomer clockwise rotation
        STEREOATROPCCW, //  atropisomer counter clockwise rotation
    }

    #[repr(i32)]
    #[derive(Debug, PartialEq, Clone)]
    enum BondType {
        UNSPECIFIED = 0,
        SINGLE,
        DOUBLE,
        TRIPLE,
        QUADRUPLE,
        QUINTUPLE,
        HEXTUPLE,
        ONEANDAHALF,
        TWOANDAHALF,
        THREEANDAHALF,
        FOURANDAHALF,
        FIVEANDAHALF,
        AROMATIC,
        IONIC,
        HYDROGEN,
        THREECENTER,
        DATIVEONE,
        DATIVE,
        DATIVEL,
        DATIVER,
        OTHER,
        ZERO, // http://pubs.acs.org/doi/abs/10.1021/ci200488k)
    }
    unsafe extern "C++" {
        include!("rdkit-rust-ffi/wrapper/graphmol/bond.h");
        type Bond;
        type BondDir;
        type BondStereo;
        type BondType;
        type ROMol = crate::graphmol::romol::ffi::ROMol;
        fn getBondDir(self: &Bond) -> BondDir;
        fn getBondType(self: &Bond) -> BondType;
        fn getStereo(self: &Bond) -> BondStereo;
        fn getBondTypeAsDouble(self: &Bond) -> f64;

        /// Returns our index within the ROMol
        fn getIdx(self: &Bond) -> Result<u32>;

        /// Returns the status of our isAromatic flag
        fn getIsAromatic(self: &Bond) -> bool;

        /// Returns the status of our isConjugated flag
        fn getIsConjugated(self: &Bond) -> bool;

        fn getStereoAtoms(self: &Bond) -> &CxxVector<i32>;

        /// Returns whether or not this instance belongs to a molecule
        fn hasOwningMol(self: &Bond) -> bool;

        fn hasQuery(self: &Bond) -> bool;

        /// Sets the index of our begin Atom
        /// # Notes
        /// - requires an owning molecule
        fn setBeginAtomIdx(self: Pin<&mut Bond>, idx: u32);

        fn setBondDir(self: Pin<&mut Bond>, dir: BondDir);
        fn setBondType(self: Pin<&mut Bond>, what: BondType);
        fn setStereo(self: Pin<&mut Bond>, stereo: BondStereo);

        /// Sets the index of our end Atom
        /// # Notes
        /// - requires an owning molecule
        fn setEndAtomIdx(self: Pin<&mut Bond>, idx: u32);

        /// sets our index within the ROMol
        fn setIdx(self: Pin<&mut Bond>, idx: u32);

        fn setIsAromatic(self: Pin<&mut Bond>, val: bool);
        fn setIsConjugated(self: Pin<&mut Bond>, val: bool);
        fn setOwningMol(self: Pin<&mut Bond>, mol: Pin<&mut ROMol>);
        fn setStereoAtoms(self: Pin<&mut Bond>, bgnIdx: u32, endIdx: u32);
        fn updatePropertyCache(self: Pin<&mut Bond>, strict: bool);

    }
}

unsafe impl Send for ffi::Bond {}
unsafe impl Sync for ffi::Bond {}
