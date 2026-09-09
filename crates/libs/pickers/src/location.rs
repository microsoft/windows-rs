use windows_core::GUID;

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
            Self::Desktop => GUID::from_u128(0xb4bfcc3a_db2c_424c_b029_7fe99a87c641),
            Self::Documents => GUID::from_u128(0x7b0db17d_9cd2_4a93_9733_46cc89022e7c),
            Self::Downloads => GUID::from_u128(0x374de290_123f_4565_9164_39c4925e467b),
            Self::Music => GUID::from_u128(0x2112ab0a_c86a_4ffe_a368_0de96e47012e),
            Self::Pictures => GUID::from_u128(0xa990ae9f_a03b_4e80_94bc_9912d7504104),
            Self::Videos => GUID::from_u128(0x491e922f_5643_4af4_a7eb_4e7a138d8174),
            Self::Computer => GUID::from_u128(0x0ac0837c_bbf8_452a_850d_79d08e667ca7),
            Self::Objects3D => GUID::from_u128(0x31c0dd25_9439_4f12_bf41_7ff4eda38722),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locations_map_to_native_known_folder_ids() {
        assert_eq!(
            PickerLocation::Desktop.id(),
            GUID::from_u128(0xb4bfcc3a_db2c_424c_b029_7fe99a87c641)
        );
        assert_eq!(
            PickerLocation::Documents.id(),
            GUID::from_u128(0x7b0db17d_9cd2_4a93_9733_46cc89022e7c)
        );
        assert_eq!(
            PickerLocation::Downloads.id(),
            GUID::from_u128(0x374de290_123f_4565_9164_39c4925e467b)
        );
        assert_eq!(
            PickerLocation::Music.id(),
            GUID::from_u128(0x2112ab0a_c86a_4ffe_a368_0de96e47012e)
        );
        assert_eq!(
            PickerLocation::Pictures.id(),
            GUID::from_u128(0xa990ae9f_a03b_4e80_94bc_9912d7504104)
        );
        assert_eq!(
            PickerLocation::Videos.id(),
            GUID::from_u128(0x491e922f_5643_4af4_a7eb_4e7a138d8174)
        );
        assert_eq!(
            PickerLocation::Computer.id(),
            GUID::from_u128(0x0ac0837c_bbf8_452a_850d_79d08e667ca7)
        );
        assert_eq!(
            PickerLocation::Objects3D.id(),
            GUID::from_u128(0x31c0dd25_9439_4f12_bf41_7ff4eda38722)
        );
    }
}
