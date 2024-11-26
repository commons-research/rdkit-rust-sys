#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {

    #[repr(i32)]
    #[derive(Debug, PartialEq)]
    pub enum StereoGroupType {
        STEREO_ABSOLUTE = 0,
        STEREO_OR = 1,
        STEREO_AND = 2,
    }
}
