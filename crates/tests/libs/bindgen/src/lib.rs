#![expect(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]

include!(concat!(env!("OUT_DIR"), "/compile_fixtures.rs"));

#[cfg(test)]
mod tests {
    use super::struct_aligned::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn aligned_struct_layout() {
        assert_eq!(align_of::<M128A>(), 16);
        assert_eq!(size_of::<M128A>(), 16);
        assert_eq!(offset_of!(M128A, Low), 0);
        assert_eq!(offset_of!(M128A, High), 8);

        assert_eq!(align_of::<XmmFrame>(), 16);
        assert_eq!(size_of::<XmmFrame>(), 32);
        assert_eq!(offset_of!(XmmFrame, Xmm0), 0);
        assert_eq!(offset_of!(XmmFrame, Flags), 16);

        assert_eq!(align_of::<FieldAligned>(), 8);
        assert_eq!(
            size_of::<FieldAligned>(),
            if cfg!(target_pointer_width = "64") {
                32
            } else {
                24
            }
        );
        assert_eq!(offset_of!(FieldAligned, First), 0);
        assert_eq!(offset_of!(FieldAligned, Second), 8);
        assert_eq!(offset_of!(FieldAligned, Third), 16);
        assert_eq!(
            offset_of!(FieldAligned, Pointer),
            if cfg!(target_pointer_width = "64") {
                24
            } else {
                20
            }
        );

        assert_eq!(align_of::<ArrayFieldAligned>(), 8);
        assert_eq!(size_of::<ArrayFieldAligned>(), 16);
        assert_eq!(offset_of!(ArrayFieldAligned, First), 0);
        assert_eq!(offset_of!(ArrayFieldAligned, Reserved), 1);
        assert_eq!(offset_of!(ArrayFieldAligned, Value), 8);

        #[cfg(target_arch = "x86")]
        {
            assert_eq!(align_of::<PointerFieldAligned>(), 4);
            assert_eq!(size_of::<PointerFieldAligned>(), 12);
            assert_eq!(offset_of!(PointerFieldAligned, Pointer), 0);
            assert_eq!(offset_of!(PointerFieldAligned, First), 4);
            assert_eq!(offset_of!(PointerFieldAligned, Value), 8);
        }
        #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
        {
            assert_eq!(align_of::<PointerFieldAligned>(), 8);
            assert_eq!(size_of::<PointerFieldAligned>(), 24);
            assert_eq!(offset_of!(PointerFieldAligned, Pointer), 0);
            assert_eq!(offset_of!(PointerFieldAligned, First), 8);
            assert_eq!(offset_of!(PointerFieldAligned, Value), 16);
        }

        assert_eq!(align_of::<OverAligned>(), 32);
        assert_eq!(size_of::<OverAligned>(), 32);
        assert_eq!(offset_of!(OverAligned, value), 0);
    }
}
