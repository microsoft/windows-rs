#![expect(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]

include!(concat!(env!("OUT_DIR"), "/compile_fixtures.rs"));

#[test]
fn buffer_contracts_preserve_storage() {
    use buffer_contracts::Buffers;
    use core::ffi::c_void;
    use windows_core::{GUID, HRESULT, IUnknown, Result};

    let _: unsafe fn(&Buffers, &[u32]) = Buffers::Input;
    let _: unsafe fn(&Buffers, u32, *mut u32) -> HRESULT = Buffers::Output;
    let _: unsafe fn(&Buffers, *mut u32) -> HRESULT = Buffers::Explicit;
    let _: unsafe fn(&Buffers, u32, *mut u32) -> HRESULT = Buffers::Arithmetic;
    let _: unsafe fn(&Buffers, u32, *mut *mut u32) -> HRESULT = Buffers::Returned;
    let _: unsafe fn(&Buffers, Option<*const u32>, *mut u32) -> HRESULT = Buffers::OptionalCount;
    let _: unsafe fn(&Buffers, u32, *const u32) = Buffers::UnknownDirection;
    let _: unsafe fn(&Buffers, u32, *mut u32) -> HRESULT = Buffers::Residual;
    let _: unsafe fn(&Buffers, u32, *const u32) = Buffers::ResidualInput;
    let _: unsafe fn(&Buffers, u32, *const u32) = Buffers::ResidualCount;
    let _: unsafe fn(&Buffers) -> Result<u32> = Buffers::Scalar;
    let _: unsafe fn(&Buffers) -> Result<IUnknown> = Buffers::Query::<IUnknown>;
    let _: unsafe fn(&Buffers, &[GUID], *mut *mut c_void) -> HRESULT = Buffers::GuidBuffer;
    let _: unsafe fn(&Buffers, *const GUID, u32, *mut *mut c_void) -> HRESULT =
        Buffers::OutputBuffer;
    let _: unsafe fn(&Buffers, *const GUID, *mut *mut c_void) -> HRESULT = Buffers::ResidualQuery;
    let _: unsafe fn(&Buffers, *const GUID, *mut *mut c_void) -> HRESULT = Buffers::ResidualOutput;
    let _: unsafe fn(u32, *mut u32) -> HRESULT = buffer_contracts::Output;
}

#[test]
fn string_alias_projections() {
    use string_aliases::IStringAliases;
    use windows_core::{HRESULT, HSTRING, PCSTR, PSTR, PWSTR, Result};

    let _: unsafe fn(&IStringAliases) -> Result<PWSTR> = IStringAliases::get_Language;
    let _: unsafe fn(&IStringAliases, &HSTRING) -> HRESULT =
        |interface, value| unsafe { interface.put_Language(value) };
    let _: unsafe fn(&IStringAliases) -> Result<PSTR> = IStringAliases::get_Ansi;
    let _: unsafe fn(&IStringAliases, PCSTR) -> HRESULT = IStringAliases::put_Ansi::<PCSTR>;
    let _: unsafe fn(&IStringAliases, *const u16) -> HRESULT = IStringAliases::Buffer;
    let _: string_aliases::LPCWSTR = core::ptr::null();
}

#[cfg(target_pointer_width = "32")]
const _: unsafe fn() -> i32 = package_arches::Windows::Win32::legacy::GetWindowLongPtrA;
#[cfg(target_pointer_width = "32")]
const _: unsafe fn() -> i32 = package_arches::Windows::Win32::split_legacy::SetWindowLongPtrW;

#[test]
fn nested_member_alignment() {
    use struct_nested_member_alignment::*;

    const {
        assert!(size_of::<Outer>() == 24);
        assert!(align_of::<Outer>() == 8);
        assert!(core::mem::offset_of!(Outer, value) == 8);
        assert!(size_of::<Outer_0>() == 16);
        assert!(core::mem::offset_of!(Outer_0, second) == 8);
        assert!(size_of::<Named>() == 32);
        assert!(size_of::<External>() == 32);
        assert!(size_of::<Matrix>() == 64);
        assert!(size_of::<Named_0>() == 16);
        assert!(core::mem::offset_of!(Named_0, second) == 8);
        assert!(size_of::<PointerOnly>() == size_of::<*mut ()>());
    }
    let mut value = Outer::default();
    value.value.second = 42;
    let copied = value;
    assert_eq!(copied.value.second, 42);
    let mut external = External::default();
    external.values[1].second = 43;
    assert_eq!(external.values[1].second, 43);
    assert_eq!(PointerOnly::default(), PointerOnly::default());
}

#[test]
fn unspecified_direction_preserves_pointer_parameters() {
    use unspecified_direction::*;
    use windows_core::{GUID, HRESULT, Result};

    let _: unsafe fn(&Unknown, u32) -> HRESULT = Unknown::Scalar;
    let _: unsafe fn(&Unknown, *mut u32) -> HRESULT = Unknown::Raw;
    let _: unsafe fn(&Unknown, Pointer) -> HRESULT = Unknown::Alias;
    let _: unsafe fn(&Unknown, u32, *mut u32) -> HRESULT = Unknown::Buffer;
    let _: unsafe fn(&Unknown, *const GUID, *mut *mut core::ffi::c_void) -> HRESULT =
        Unknown::Query;
    let _: unsafe fn(&Unknown, *mut Option<Unknown>) -> HRESULT = Unknown::Interface;
    let _: unsafe fn(&Unknown, *const u32) -> HRESULT = Unknown::Input;
    let _: unsafe fn(&Unknown) -> Result<u32> = Unknown::Output;
}

#[test]
fn query_id_is_not_a_guid_buffer() {
    use query_buffers::*;
    use windows_core::{GUID, HRESULT, IUnknown, Interface, Result};

    struct Producer;
    impl Factory_Impl for Producer {
        fn Reference(
            &self,
            count: u32,
            filters: *const GUID,
            riid: *const IID,
            result: *mut *mut core::ffi::c_void,
        ) -> Result<()> {
            assert_eq!(count, 2);
            unsafe {
                assert_eq!(*filters, GUID::from_u128(11));
                assert_eq!(*filters.add(1), GUID::from_u128(22));
                assert_eq!(*riid, IUnknown::IID);
            }
            assert!(!result.is_null());
            Err(HRESULT(0x80004002u32 as i32).into())
        }

        fn BufferAfterId(
            &self,
            riid: *const GUID,
            count: u32,
            filters: *const GUID,
            result: *mut *mut core::ffi::c_void,
        ) -> Result<()> {
            self.Reference(count, filters, riid, result)
        }

        fn BufferOnly(&self, _: u32, _: *const GUID, _: *mut *mut core::ffi::c_void) -> Result<()> {
            unreachable!()
        }

        fn ByteBuffer(&self, _: u32, _: *const GUID, _: *mut *mut core::ffi::c_void) -> Result<()> {
            unreachable!()
        }

        fn FixedBuffer(&self, _: *const GUID, _: *mut *mut core::ffi::c_void) -> Result<()> {
            unreachable!()
        }

        fn OutputBuffer(
            &self,
            _: *const GUID,
            _: u32,
            _: *mut *mut core::ffi::c_void,
        ) -> Result<()> {
            unreachable!()
        }
    }
    let factory = Factory::new(&Producer);
    let filters = [GUID::from_u128(11), GUID::from_u128(22)];
    unsafe {
        assert_eq!(
            factory.Reference::<IUnknown>(&filters).unwrap_err().code(),
            HRESULT(0x80004002u32 as i32)
        );
        assert_eq!(
            factory
                .BufferAfterId::<IUnknown>(&filters)
                .unwrap_err()
                .code(),
            HRESULT(0x80004002u32 as i32)
        );
    }
}

#[test]
fn arch_bitfield_storage() {
    use struct_arch_bitfield::*;
    let mut member = Member { storage: 0xa0 };
    member.set_value(13);
    assert_eq!(member.storage, 0xad);
    let expected = if cfg!(target_arch = "x86") {
        -3i64 as Alias
    } else {
        13
    };
    assert_eq!(member.value(), expected);
    let mut storage = Storage { _bitfield: 0xa0 };
    storage.set_value(13);
    storage.set_flag(true);
    assert_eq!(storage._bitfield, 0xbd);
    assert_eq!(storage.value(), expected);
    assert!(storage.flag());
}

#[test]
fn named_bitfield_storage() {
    use struct_named_bitfield::*;
    let mut wrapped = Wrapped { _bitfield: Byte(0) };
    wrapped.set_Low(7);
    wrapped.set_High(15);
    wrapped.set_Flag(true);
    assert_eq!(wrapped._bitfield.0, 255);
    wrapped.set_Low(2);
    assert_eq!(
        (wrapped.Low(), wrapped.High(), wrapped.Flag()),
        (2, 15, true)
    );
    let mut full = Full { _bitfield: Byte(0) };
    full.set_Value(255);
    assert_eq!((full.Value(), full._bitfield.0), (255, 255));
    let mut enumerated = Enumerated { _bitfield: 0 };
    enumerated.set_Signed(-3);
    enumerated.set_Flag(true);
    assert_eq!((enumerated.Signed(), enumerated.Flag()), (-3, true));
    let mut union = ScopedUnion { All: 0x4000 };
    union.set_Signed(-3);
    union.set_Flag(true);
    assert_eq!((union.Signed(), union.Flag()), (-3, true));
    assert_eq!(unsafe { union.All }, 0x403d);
    let mut typed = Typed {
        storage: Byte(0xa0),
    };
    typed.set_Signed(-3);
    typed.set_Flag(true);
    assert_eq!((typed.Signed(), typed.Flag()), (-3, true));
    assert_eq!(typed.storage.0, 0xbd);
    let mut pointer = Pointer {
        _bitfield: usize::MAX,
    };
    pointer.set_Low(3);
    pointer.set_High(0x1234567);
    assert_eq!((pointer.Low(), pointer.High()), (3, 0x1234567));
    assert_eq!(
        pointer._bitfield,
        (usize::MAX & !(u32::MAX as usize)) | 0x12345673
    );
    let mut signed = SignedPointer { _bitfield: -1 };
    signed.set_Low(-3);
    signed.set_High(-7);
    assert_eq!((signed.Low(), signed.High()), (-3, -7));
    assert_eq!(signed._bitfield, -99);
    let mut full_pointer = FullPointer::default();
    full_pointer.set_Unsigned(usize::MAX);
    full_pointer.set_Signed(isize::MIN);
    assert_eq!(full_pointer.Unsigned(), usize::MAX);
    assert_eq!(full_pointer.Signed(), isize::MIN);
    assert_eq!(full_pointer.unsigned, usize::MAX);
    assert_eq!(full_pointer.signed, isize::MIN);
    let mut wide_member = WideMember {
        storage: usize::MAX,
    };
    wide_member.set_Value(0x12345678);
    assert_eq!(wide_member.Value(), 0x12345678);
    assert_eq!(
        wide_member.storage,
        (usize::MAX & !(u32::MAX as usize)) | 0x12345678
    );
    let mut pointer_member = PointerMember { storage: u64::MAX };
    pointer_member.set_Value(-3);
    assert_eq!(pointer_member.Value(), -3);
    assert_eq!(pointer_member.storage, 0xffff_ffff_fffd_ffff);
    let mut logical = Logical::default();
    logical.set_Signed(-3);
    logical.set_Flag(true);
    assert_eq!((logical.Signed(), logical.Flag()), (-3, true));
    assert_eq!(logical._bitfield.0, 0x3d);
}

#[test]
fn narrow_event_delegate_invoke() {
    let handler = narrow_event::Handler::new(|value| {
        assert_eq!(value.Number, 42);
        Err(windows_core::HRESULT(-1).into())
    });
    let error = handler
        .Invoke(narrow_event::Value { Number: 42 })
        .unwrap_err();
    assert_eq!(error.code(), windows_core::HRESULT(-1));
}

#[test]
fn narrow_generic_event_delegate_invoke() {
    let handler =
        narrow_event_generic::Handler::<narrow_event_generic::ISource>::new(|_, value| {
            assert_eq!(value, 42);
            Err(windows_core::HRESULT(-1).into())
        });
    let error = handler
        .Invoke(None::<&narrow_event_generic::ISource>, 42)
        .unwrap_err();
    assert_eq!(error.code(), windows_core::HRESULT(-1));
}

#[test]
fn narrow_collection_iteration_traits() {
    fn assert_into_iterator<T: IntoIterator<Item = i32>>() {}
    fn assert_iterator<T: Iterator<Item = i32>>() {}

    assert_into_iterator::<narrow_iterable_interface_full::IItems<i32>>();
    assert_into_iterator::<&narrow_iterable_interface_full::IItems<i32>>();
    assert_into_iterator::<narrow_iterable_names::IItems<i32>>();
    assert_into_iterator::<&narrow_iterable_names::IItems<i32>>();
    assert_into_iterator::<narrow_iterable_names::Items>();
    assert_into_iterator::<&narrow_iterable_names::Items>();
    assert_into_iterator::<narrow_iterable_first::IIterable<i32>>();
    assert_into_iterator::<&narrow_iterable_first::IIterable<i32>>();
    assert_iterator::<narrow_iterator_full::IIterator<i32>>();
}
