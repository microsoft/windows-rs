// #![allow(unused_variables)]
// #![allow(dead_code)]

mod runtime;
use runtime::*;
use std::io::*;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

// trait IUnknown{
//     unsafe fn QueryInterface(&self, interface: &GUID, result: *mut *mut VOID) -> HRESULT;
//     fn AddRef(&self)->u32;
//     fn Release(&self)->u32;
// }

// trait IInspectable : IUnknown{
//     fn GetIids(&self) -> HRESULT;
//     fn GetRuntimeClassName(&self) -> HRESULT;
//     fn GetTrustLevel(&self) -> HRESULT;
// }

// trait IColorsStatics : IInspectable{
//     fn get_AliceBlue(&self) -> HRESULT;
// }

fn run() -> std::io::Result<()> {
    unsafe {
        let hr = RoInitialize(ApartmentType::MultiThreaded);
        println!("{:#x}", hr);
        assert!(hr == 0);

        let mut hstring: HSTRING = std::ptr::null_mut();
        let wchars: Vec<u16> = "Windows.UI.Colors".encode_utf16().collect();
        let hr = WindowsCreateString(wchars.as_ptr(), "Windows.UI.Colors".len() as u32, &mut hstring);
        assert!(hr == 0);

        let mut length = 0;
        let raw = WindowsGetStringRawBuffer(hstring, &mut length);
        let value = String::from_utf16(std::slice::from_raw_parts(raw, length as usize)).unwrap();
        println!("{} : {} ({})", length, value, value.len());

        let guid_IColorsStatics = GUID::from_values(0xCFF52E04, 0xCCA6, 0x4614, &[0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99]);

        let mut factory: *mut VOID = std::ptr::null_mut();
        let hr = RoGetActivationFactory(hstring, &guid_IColorsStatics, &mut factory);
        println!("{:#x}", hr);
        assert!(hr == 0);
    }
    Ok(())
}
