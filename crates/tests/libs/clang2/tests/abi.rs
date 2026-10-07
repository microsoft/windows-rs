#![cfg(target_env = "msvc")]

#[allow(non_snake_case, non_camel_case_types, dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));
}
use bindings::*;
use core::ffi::c_void;

#[test]
fn record_layout_matches_cpp() {
    for (index, value) in [
        size_of::<AbiPacket>(),
        align_of::<AbiPacket>(),
        std::mem::offset_of!(AbiPacket, tag),
        std::mem::offset_of!(AbiPacket, count),
        std::mem::offset_of!(AbiPacket, scale),
        std::mem::offset_of!(AbiPacket, value),
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
