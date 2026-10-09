#![cfg(target_env = "msvc")]

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));
}
use bindings::*;
use core::ffi::c_void;

#[test]
fn packed_layout_matches_cpp() {
    let anonymous = std::mem::offset_of!(PackedAnonymous, Anonymous1);
    for (index, value) in [
        size_of::<Packed1>(),
        align_of::<Packed1>(),
        std::mem::offset_of!(Packed1, tag),
        std::mem::offset_of!(Packed1, value),
        std::mem::offset_of!(Packed1, wide),
        std::mem::offset_of!(Packed1, pointer),
        size_of::<PackedChoice>(),
        align_of::<PackedChoice>(),
        std::mem::offset_of!(PackedChoice, record),
        std::mem::offset_of!(PackedChoice, bits),
        size_of::<PackedAnonymous>(),
        align_of::<PackedAnonymous>(),
        std::mem::offset_of!(PackedAnonymous, tag),
        anonymous + std::mem::offset_of!(PackedAnonymous_0, value),
        anonymous + std::mem::offset_of!(PackedAnonymous_0, halves),
        size_of::<Packed2>(),
        align_of::<Packed2>(),
        std::mem::offset_of!(Packed2, tag),
        std::mem::offset_of!(Packed2, values),
        std::mem::offset_of!(Packed2, choice),
        size_of::<Packed4>(),
        align_of::<Packed4>(),
        std::mem::offset_of!(Packed4, tag),
        std::mem::offset_of!(Packed4, wide),
        std::mem::offset_of!(Packed4, pointer),
        size_of::<PackedContainer>(),
        align_of::<PackedContainer>(),
        std::mem::offset_of!(PackedContainer, records),
        std::mem::offset_of!(PackedContainer, tail),
        size_of::<NaturalChoice>(),
        align_of::<NaturalChoice>(),
        std::mem::offset_of!(NaturalChoice, value),
        std::mem::offset_of!(NaturalChoice, real),
        size_of::<PackedField>(),
        align_of::<PackedField>(),
        std::mem::offset_of!(PackedField, tag),
        std::mem::offset_of!(PackedField, choice),
        size_of::<PackedGap>(),
        align_of::<PackedGap>(),
        std::mem::offset_of!(PackedGap, __padding1),
        std::mem::offset_of!(PackedGap, marker),
        std::mem::offset_of!(PackedGap, value),
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            unsafe { PackedLayoutEvidence(index.try_into().unwrap()) },
            u32::try_from(value).unwrap(),
            "packed layout observation {index}"
        );
    }
}

#[test]
fn packed_storage_crosses_pointer_calls_in_both_directions() {
    unsafe extern "C" fn callback(packet: *mut Packed1) {
        unsafe {
            let value = &raw mut (*packet).wide;
            value.write_unaligned(value.read_unaligned() + 7);
        }
    }
    let mut packet = Packed1 {
        tag: 1,
        value: 2,
        wide: 3,
        pointer: core::ptr::null_mut(),
    };
    unsafe {
        PackedMutate(&raw mut packet);
        PackedInvoke(Some(callback), &raw mut packet);
        assert_eq!(packet.tag, 2);
        assert_eq!((&raw const packet.value).read_unaligned(), 5);
        assert_eq!((&raw const packet.wide).read_unaligned(), 15);
        assert_eq!(
            (&raw const packet.pointer).read_unaligned(),
            (&raw mut packet).cast()
        );
    }
}

#[test]
fn union_and_nested_layout_matches_cpp() {
    let anonymous = std::mem::offset_of!(LayoutPacket, Anonymous1_);
    let inner = anonymous + std::mem::offset_of!(LayoutPacket_0, Anonymous1);
    let named = std::mem::offset_of!(LayoutPacket, named);
    for (index, value) in [
        size_of::<LayoutChoice>(),
        align_of::<LayoutChoice>(),
        std::mem::offset_of!(LayoutChoice, number),
        std::mem::offset_of!(LayoutChoice, real),
        std::mem::offset_of!(LayoutChoice, bytes),
        size_of::<LayoutAligned>(),
        align_of::<LayoutAligned>(),
        std::mem::offset_of!(LayoutAligned, bytes),
        std::mem::offset_of!(LayoutAligned, number),
        size_of::<LayoutPacket>(),
        align_of::<LayoutPacket>(),
        std::mem::offset_of!(LayoutPacket, tag),
        anonymous + std::mem::offset_of!(LayoutPacket_0, number),
        inner + std::mem::offset_of!(LayoutPacket_0_0, low),
        inner + std::mem::offset_of!(LayoutPacket_0_0, high),
        anonymous + std::mem::offset_of!(LayoutPacket_0, real),
        std::mem::offset_of!(LayoutPacket, Anonymous1),
        std::mem::offset_of!(LayoutPacket, choices),
        named,
        named + std::mem::offset_of!(LayoutPacket_1, value),
        named + std::mem::offset_of!(LayoutPacket_1, marker),
        std::mem::offset_of!(LayoutPacket, aligned),
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            unsafe { LayoutEvidence(index.try_into().unwrap()) },
            u32::try_from(value).unwrap(),
            "layout observation {index}"
        );
    }
}

#[test]
fn union_storage_crosses_pointer_calls_in_both_directions() {
    let mut packet = LayoutPacket {
        tag: 1,
        Anonymous1: 2,
        named: LayoutPacket_1 {
            value: -5,
            marker: 6,
        },
        ..Default::default()
    };
    packet.Anonymous1_.real = 1.25;
    packet.choices[0].number = 3;
    packet.choices[1].real = 4.25;
    packet.aligned.number = 7;
    unsafe { LayoutMutate(&mut packet) };
    assert_eq!(packet.tag, 2);
    assert_eq!(unsafe { packet.Anonymous1_.real }, 2.5);
    assert_eq!(packet.Anonymous1, 5);
    assert_eq!(unsafe { packet.choices[0].number }, 8);
    assert_eq!(unsafe { packet.choices[1].real }, 4.75);
    assert_eq!(packet.named.value, -12);
    assert_eq!(packet.named.marker, 15);
    assert_eq!(unsafe { packet.aligned.number }, 18);

    unsafe extern "C" fn callback(packet: *mut LayoutPacket) {
        let packet = unsafe { &mut *packet };
        packet.tag += 2;
        packet.Anonymous1_.Anonymous1 = LayoutPacket_0_0 { low: -7, high: 9 };
        packet.named.value = 31;
        packet.aligned.number = 42;
    }
    unsafe { LayoutInvoke(Some(callback), &mut packet) };
    assert_eq!(packet.tag, 4);
    assert_eq!(unsafe { packet.Anonymous1_.Anonymous1.low }, -7);
    assert_eq!(unsafe { packet.Anonymous1_.Anonymous1.high }, 9);
    assert_eq!(packet.named.value, 31);
    assert_eq!(unsafe { packet.aligned.number }, 42);
}

#[test]
fn record_layout_matches_cpp() {
    for (index, value) in [
        size_of::<AbiPacket>(),
        align_of::<AbiPacket>(),
        std::mem::offset_of!(AbiPacket, tag),
        std::mem::offset_of!(AbiPacket, count),
        std::mem::offset_of!(AbiPacket, scale),
        std::mem::offset_of!(AbiPacket, value),
        size_of::<EnumPacket>(),
        align_of::<EnumPacket>(),
        std::mem::offset_of!(EnumPacket, small),
        std::mem::offset_of!(EnumPacket, scoped),
        std::mem::offset_of!(EnumPacket, state),
        std::mem::offset_of!(EnumPacket, wide),
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            unsafe { AbiLayout(index.try_into().unwrap()) },
            u32::try_from(value).unwrap()
        );
    }
}

#[test]
fn enum_values_cross_native_calls_in_both_directions() {
    let mut packet = EnumPacket::default();
    assert_eq!(unsafe { ConvertEnum(&mut packet, High) }, Done);
    assert_eq!(packet.small, Negative);
    assert_eq!(packet.scoped, Last);
    assert_eq!(packet.state, Ready);
    assert_eq!(packet.wide, u64::MAX);
    unsafe extern "system" fn get(_: *mut c_void, value: Scoped, output: *mut State) -> State {
        unsafe { *output = Done };
        if value == Last { Ready } else { Done }
    }
    let vtable = IEnums_Vtbl { Get: get };
    let mut object = &vtable;
    let native = unsafe { AbiEnums() };
    let native_vtable = unsafe { &**native.cast::<*const IEnums_Vtbl>() };
    for _ in 0..256 {
        let mut output = 0;
        assert_eq!(
            unsafe { (native_vtable.Get)(native, Last, &mut output) },
            Done
        );
        assert_eq!(output, Ready);
        assert_eq!(
            unsafe { AbiEnumCall((&raw mut object).cast(), Last, &mut output) },
            Ready
        );
        assert_eq!(output, Done);
    }
}

fn packet() -> AbiPacket {
    AbiPacket {
        tag: 3,
        count: 17,
        scale: 1.25,
        value: -0x123456789,
    }
}

fn assert_packet(actual: &AbiPacket, expected: &AbiPacket) {
    assert_eq!(actual.tag, expected.tag);
    assert_eq!(actual.count, expected.count);
    assert_eq!(actual.scale, expected.scale);
    assert_eq!(actual.value, expected.value);
}

fn transformed(mut packet: AbiPacket) -> AbiPacket {
    packet.tag += 1;
    packet.count += 7;
    packet.scale *= 2.0;
    packet.value -= 9;
    packet
}

#[test]
fn aggregates_pass_and_return_by_value() {
    let input = packet();
    assert_packet(&unsafe { AbiRoundtrip(input) }, &transformed(packet()));
    assert_packet(&input, &packet());
}

#[test]
fn rust_calls_cpp_inherited_vtable() {
    let object = unsafe { AbiGet() };
    assert!(!object.is_null());
    let vtable = unsafe { &**object.cast::<*const IAbiDerived_Vtbl>() };
    for value in 0..256 {
        unsafe {
            assert_eq!((vtable.base__.Base)(object, value), value + 11);
            assert_eq!((vtable.base__.Measure)(object, 3.25), 6.5);
            let mut data = packet();
            let mut buffer = [1, -2, 5];
            assert_eq!(
                (vtable.Mutate)(object, &mut data, buffer.as_mut_ptr(), 3),
                3
            );
            assert_eq!(buffer, [4, 1, 8]);
            assert_packet(&data, &transformed(packet()));
        }
    }
}

#[repr(C)]
struct RustObject {
    vtable: *const IAbiDerived_Vtbl,
    bias: i32,
    calls: u32,
}

unsafe extern "system" fn base(this: *mut c_void, value: i32) -> i32 {
    let this = unsafe { &mut *this.cast::<RustObject>() };
    this.calls += 1;
    value + this.bias
}

unsafe extern "system" fn measure(this: *mut c_void, value: f64) -> f64 {
    let this = unsafe { &mut *this.cast::<RustObject>() };
    this.calls += 1;
    value * 4.0
}

unsafe extern "system" fn mutate(
    this: *mut c_void,
    packet: *mut AbiPacket,
    values: *mut i32,
    count: u32,
) -> u32 {
    let this = unsafe { &mut *this.cast::<RustObject>() };
    this.calls += 1;
    unsafe {
        *packet = transformed(*packet);
        for index in 0..count as usize {
            *values.add(index) -= 4;
        }
    }
    count
}

#[test]
fn cpp_calls_rust_inherited_vtable() {
    let vtable = IAbiDerived_Vtbl {
        base__: IAbiBase_Vtbl {
            Base: base,
            Measure: measure,
        },
        Mutate: mutate,
    };
    let mut object = RustObject {
        vtable: &vtable,
        bias: 17,
        calls: 0,
    };
    for value in 0..256 {
        let mut data = packet();
        let mut buffer = [9, -1, 6];
        let result = unsafe {
            AbiCall(
                (&raw mut object).cast(),
                value,
                2.5,
                &mut data,
                buffer.as_mut_ptr(),
                3,
            )
        };
        assert_eq!(result, value + 30);
        assert_eq!(buffer, [5, -5, 2]);
        assert_packet(&data, &transformed(packet()));
        assert_eq!(object.calls, (value as u32 + 1) * 3);
    }
}
