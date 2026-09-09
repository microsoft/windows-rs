use super::*;

/// A Windows known folder used as a picker location.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PickerLocation {
    Desktop,
    Documents,
    Downloads,
    Music,
    Pictures,
    Videos,
    Computer,
    Objects3D,
}

impl PickerLocation {
    pub(crate) const fn id(self) -> GUID {
        match self {
            Self::Desktop => FOLDERID_Desktop,
            Self::Documents => FOLDERID_DocumentsLibrary,
            Self::Downloads => FOLDERID_Downloads,
            Self::Music => FOLDERID_MusicLibrary,
            Self::Pictures => FOLDERID_PicturesLibrary,
            Self::Videos => FOLDERID_VideosLibrary,
            Self::Computer => FOLDERID_ComputerFolder,
            Self::Objects3D => FOLDERID_Objects3D,
        }
    }
}
