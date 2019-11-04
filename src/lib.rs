// #![allow(unused_variables)]
// #![allow(dead_code)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

mod runtime;
use runtime::*;
use std::io::*;
//use syn::*;

// #[proc_macro_attribute]
// pub fn hello(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     let expanded = quote! {

//     };

//     TokenStream::from(expanded)
// }

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

#[repr(C)]
struct Color{
    a:u8,
    r:u8,
    g:u8,
    b:u8,
}

#[repr(C)]
struct IColorsStatics{
    QueryInterface: extern "stdcall" fn(*const *const IColorsStatics, interface: &GUID, result: *mut *mut VOID) -> HRESULT,
    AddRef: extern "stdcall" fn(*const *const IColorsStatics)->u32,
    Release: extern "stdcall" fn(*const *const IColorsStatics)->u32,
    GetIids: extern fn(),
    GetRuntimeClassName: extern fn(),
    GetTrustLevel: extern fn(),
    get_AliceBlue:extern "stdcall"  fn(*const *const IColorsStatics, value: &mut Color) -> HRESULT
}

struct Colors;

impl Colors{
    fn alice_blue() -> Result<Color>{
    unsafe {
        let hr = RoInitialize(ApartmentType::MultiThreaded);
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

        let ptr = factory as *const *const IColorsStatics;

        let count = ((*(*ptr)).AddRef)(ptr);
        println!("ref {}", count);
        let count = ((*(*ptr)).Release)(ptr);
        println!("ref {}", count);

        let mut color = Color{a:0,r:0,g:0,b:0};

        let hr = ((*(*ptr)).get_AliceBlue)(ptr, &mut color);
        assert!(hr == 0);
        Ok(color)
    }
    }
}

fn run() -> std::io::Result<()> {
    
    let color = Colors::alice_blue()?;

    assert!(color.a == 255);
    assert!(color.r == 240);
    assert!(color.g == 248);
    assert!(color.b == 255);
    
    Ok(())
}
