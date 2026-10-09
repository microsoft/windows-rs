use windows_clang2::{Input, ProjectionOptions, capture};
use windows_metadata::{Value, reader::*};

const SOURCE: &str = include_str!("../input/bitfields.h");
const ROOTS: &[&str] = &["BitUnits", "BitPacked", "BitNested", "BitFull"];

#[cfg(target_env = "msvc")]
#[allow(
    non_snake_case,
    non_camel_case_types,
    dead_code,
    clippy::erasing_op,
    clippy::identity_op
)]
mod native {
    include!(concat!(env!("OUT_DIR"), "/bitfields_types.rs"));
    unsafe extern "C" {
        pub fn BitLayoutEvidence(index: u32) -> u32;
        pub fn BitRead(value: *const BitUnits, index: u32) -> u64;
        pub fn BitWrite(value: *mut BitUnits, index: u32, input: u64);
        pub fn BitPackedRead(value: *const BitPacked) -> u32;
        pub fn BitPackedWrite(value: *mut BitPacked, input: u32);
        pub fn BitInvoke(
            callback: Option<unsafe extern "C" fn(*mut BitUnits, *mut BitPacked)>,
            value: *mut BitUnits,
            packed: *mut BitPacked,
        );
    }
}

#[test]
fn unsigned_bitfield_storage_preserves_source_positions_and_metadata() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        for reversed in [false, true] {
            let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", SOURCE)];
            if reversed {
                inputs.reverse();
            }
            let snapshot = capture(inputs, &["-x", "c++", &target], ROOTS).unwrap();
            let rdl = snapshot
                .resolve()
                .unwrap()
                .project(&ProjectionOptions::new("Test"))
                .unwrap()
                .rdl();
            let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("expected")
                .join("bitfields.rdl");
            if std::env::var_os("UPDATE_EXPECT").is_some() {
                std::fs::write(&expected, &rdl).unwrap();
            }
            assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
            let output = std::path::Path::new(env!("OUT_DIR"))
                .join(format!("bitfield-{arch}-{reversed}.winmd"));
            windows_rdl::reader()
                .input_text(&rdl)
                .output(&output)
                .write()
                .unwrap();
            let roundtrip = output.with_extension("rdl");
            windows_rdl::writer()
                .input(&output)
                .filter("Test")
                .output(&roundtrip)
                .write()
                .unwrap();
            let reencoded = output.with_extension("roundtrip.winmd");
            windows_rdl::reader()
                .input(&roundtrip)
                .output(&reencoded)
                .write()
                .unwrap();
            let before = Index::read(output).unwrap();
            let after = Index::read(reencoded).unwrap();
            for name in ROOTS {
                let before_root = before.expect("Test", name);
                let after_root = after.expect("Test", name);
                let before_nested = before.nested_recursive(before_root);
                let after_nested = after.nested_recursive(after_root);
                assert_eq!(before_nested.len(), after_nested.len());
                for (before, after) in std::iter::once(before_root)
                    .chain(before_nested)
                    .zip(std::iter::once(after_root).chain(after_nested))
                {
                    assert_eq!(before.name(), after.name());
                    assert_eq!(before.flags(), after.flags());
                    assert_eq!(
                        before.class_layout().map(|layout| layout.packing_size()),
                        after.class_layout().map(|layout| layout.packing_size())
                    );
                    let fields = |ty: TypeDef| {
                        ty.fields()
                            .map(|field| {
                                let members: Vec<_> = field
                                    .attributes()
                                    .filter(|attr| attr.name() == "NativeBitfieldAttribute")
                                    .map(|attr| attr.value())
                                    .collect();
                                (field.name().to_owned(), field.ty(), members)
                            })
                            .collect::<Vec<_>>()
                    };
                    assert_eq!(fields(before), fields(after));
                }
            }
            let record = before.expect("Test", "BitUnits");
            let members: Vec<_> = record
                .fields()
                .flat_map(|field| field.attributes())
                .filter(|attr| attr.name() == "NativeBitfieldAttribute")
                .map(|attr| attr.value())
                .collect();
            assert_eq!(members.len(), 10);
            assert_eq!(
                members[1]
                    .iter()
                    .map(|(_, value)| value.clone())
                    .collect::<Vec<_>>(),
                [Value::Utf8("mode".into()), Value::I64(3), Value::I64(5)]
            );
        }
    }
}

#[test]
fn bitfields_reject_unmodeled_member_contracts_and_calls() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        for root in [
            "BitEnum",
            "BitSigned",
            "BitMixed",
            "BitBool",
            "BitConst",
            "BitVolatile",
            "BitAnnotated",
            "BitUnion",
            "BitUnderscore",
            "BitConstAlias",
            "BitVolatileAlias",
        ] {
            let snapshot = capture(
                [Input::new(
                    "rejected.hpp",
                    include_str!("../input/bitfields_rejected.h"),
                )],
                &["-x", "c++", &target],
                &[root],
            )
            .unwrap();
            let error = snapshot
                .resolve()
                .unwrap()
                .project(&ProjectionOptions::new("Test"))
                .unwrap_err()
                .to_string();
            assert!(error.contains("bitfield projection"), "{root}: {error}");
        }
        for declaration in [
            "extern \"C\" void Use(BitUnits value);",
            "extern \"C\" BitUnits Use();",
            "struct Container { BitUnits values[2]; }; extern \"C\" void Use(Container value);",
            "typedef void (*Callback)(BitUnits value); extern \"C\" void Use(Callback callback);",
        ] {
            let snapshot = capture(
                [Input::new("calls.hpp", format!("{SOURCE}\n{declaration}"))],
                &["-x", "c++", &target],
                &["Use"],
            )
            .unwrap();
            let mut options = ProjectionOptions::new("Test");
            options.library = Some("test.dll".into());
            let error = snapshot
                .resolve()
                .unwrap()
                .project(&options)
                .unwrap_err()
                .to_string();
            assert!(error.contains("adjusted record layouts"), "{error}");
        }
    }
}

#[test]
fn bitfield_allocation_policy_is_not_assumed_for_other_windows_abis() {
    let snapshot = capture(
        [Input::new("bits.hpp", SOURCE)],
        &["-x", "c++", "--target=x86_64-w64-windows-gnu"],
        &["BitUnits"],
    )
    .unwrap();
    let error = snapshot
        .resolve()
        .unwrap()
        .project(&ProjectionOptions::new("Test"))
        .unwrap_err()
        .to_string();
    assert!(error.contains("MSVC struct allocation units"), "{error}");
}

#[test]
#[cfg(target_env = "msvc")]
fn native_unsigned_bitfield_layout_matches_msvc() {
    use native::*;
    for (index, value) in [
        size_of::<BitUnits>(),
        align_of::<BitUnits>(),
        std::mem::offset_of!(BitUnits, __bitfield0),
        std::mem::offset_of!(BitUnits, end),
        size_of::<BitPacked>(),
        align_of::<BitPacked>(),
        std::mem::offset_of!(BitPacked, tag),
        std::mem::offset_of!(BitPacked, value),
        size_of::<BitNested>(),
        align_of::<BitNested>(),
        std::mem::offset_of!(BitNested, entries),
        std::mem::offset_of!(BitNested, choice),
        size_of::<BitFull>(),
        align_of::<BitFull>(),
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            unsafe { BitLayoutEvidence(index.try_into().unwrap()) },
            u32::try_from(value).unwrap(),
            "layout {index}"
        );
    }
}

#[cfg(target_env = "msvc")]
fn read_members(value: &native::BitUnits) -> [u64; 10] {
    [
        u64::from(value.flag()),
        u64::from(value.mode()),
        u64::from(value.low()),
        u64::from(value.high()),
        u64::from(value.first()),
        u64::from(value.second()),
        u64::from(value.next()),
        u64::from(value.after()),
        value.wide(),
        value.tail(),
    ]
}

#[test]
#[cfg(target_env = "msvc")]
fn native_unsigned_accessors_agree_with_cpp_members() {
    use native::*;
    let widths = [1, 5, 4, 12, 3, 29, 2, 7, 40, 24];
    let mut value = BitUnits::default();
    let mut expected = [0u64; 10];
    for input in [u64::MAX, 0, 0xa5a5a5a5a5a5a5a5] {
        for (index, width) in widths.iter().enumerate() {
            unsafe {
                BitWrite(&raw mut value, index.try_into().unwrap(), input);
            }
            expected[index] = input & ((1u64 << width) - 1);
            assert_eq!(read_members(&value), expected);
        }
    }
    value.set_flag(false);
    value.set_mode(21);
    value.set_low(7);
    value.set_high(2049);
    value.set_first(5);
    value.set_second(0x1234567);
    value.set_next(3);
    value.set_after(69);
    value.set_wide(0x123456789a);
    value.set_tail(0xabcdef);
    for (index, member) in read_members(&value).into_iter().enumerate() {
        assert_eq!(
            unsafe { BitRead(&raw const value, index.try_into().unwrap()) },
            member
        );
    }
    let mut packed = BitPacked::default();
    unsafe {
        BitPackedWrite(&raw mut packed, 0xa7);
    }
    assert!(packed.enabled());
    assert_eq!(packed.level(), 0x53);
    packed.set_enabled(false);
    packed.set_level(0x7f);
    assert_eq!(unsafe { BitPackedRead(&raw const packed) }, 0xfe);
    let mut full = BitFull::default();
    full.set_whole(u64::MAX);
    assert_eq!(full.whole(), u64::MAX);
    unsafe extern "C" fn callback(value: *mut BitUnits, packed: *mut BitPacked) {
        unsafe {
            (*value).set_mode(1);
            (*packed).set_level(2);
        }
    }
    unsafe {
        BitInvoke(Some(callback), &raw mut value, &raw mut packed);
    }
    assert_eq!(value.mode(), 1);
    assert_eq!(packed.level(), 2);
}
