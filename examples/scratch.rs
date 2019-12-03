#[winrt_macros::echo_target]
use winrt::*;

#[repr(C)]
struct Color {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

#[repr(C)]
struct IColorsStatics {
    query_interface: extern "stdcall" fn(*const *const IColorsStatics, interface: &Guid, result: *mut *mut VOID) -> ErrorCode,
    add_ref: extern "stdcall" fn(*const *const IColorsStatics) -> u32,
    release: extern "stdcall" fn(*const *const IColorsStatics) -> u32,
    iids: extern "C" fn(),
    runtime_class_name: extern "C" fn(),
    trust_level: extern "C" fn(),
    alice_blue: extern "stdcall" fn(*const *const IColorsStatics, value: &mut Color) -> ErrorCode,
}

struct Colors;

impl Colors {
    fn alice_blue() -> Result<Color> {
        unsafe {
            let guid = Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99");

            let mut factory: *mut VOID = std::ptr::null_mut();
            let name = String::from("Windows.UI.Colors");
            println!("[{}]", name.to_string());
            RoGetActivationFactory(name.as_raw_handle(), &guid, &mut factory).result()?;

            let ptr = factory as *const *const IColorsStatics;
            let mut color = Color { a: 0, r: 0, g: 0, b: 0 };
            ((*(*ptr)).alice_blue)(ptr, &mut color).result()?;
            Ok(color)
        }
    }
}

fn run() -> Result<()> {
    init();
    let color = Colors::alice_blue()?;

    assert!(color.a == 255);
    assert!(color.r == 240);
    assert!(color.g == 248);
    assert!(color.b == 255);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        e.code().unwrap();
    }

    println!("ok");
}

