//#[winrt_macros::echo_target]
use winrt::*;

#[repr(C)]
#[derive(Default)]
struct Color {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

#[repr(C)]
struct IUnknown {
    query: extern "system" fn(*const Void, &Guid, *mut *mut Void) -> ErrorCode,
    addref: extern "system" fn(*const Void) -> u32,
    release: extern "system" fn(*const Void) -> u32,
}

impl IUnknown {
    fn query(ptr: *const Void, guid: &Guid) -> *const Void {
        unsafe {
            let mut result: *mut Void = std::ptr::null_mut();
            ((*(*(ptr as *const *const IUnknown))).query)(ptr, guid, &mut result);
            result
        }
    }

    fn addref(ptr: *const Void) -> u32 {
        unsafe { ((*(*(ptr as *const *const IUnknown))).addref)(ptr) }
    }

    fn release(ptr: *const Void) -> u32 {
        unsafe { ((*(*(ptr as *const *const IUnknown))).release)(ptr) }
    }
}

#[repr(C)]
struct IColorsStatics {
    impl_0: usize,
    impl_1: usize,
    impl_2: usize,
    impl_3: usize,
    impl_4: usize,
    impl_5: usize,
    alice_blue: extern "system" fn(*const Void, value: &mut Color) -> ErrorCode,
}

impl IColorsStatics {
    fn alice_blue(ptr: *const Void) -> Result<Color> {
        unsafe {
        let mut color = Default::default();
        ((*(*(ptr as *const *const IColorsStatics))).alice_blue)(ptr, &mut color).ok_or(color)
        }
    }
}

struct Colors;

impl Colors {
    fn alice_blue() -> Result<Color> {
        unsafe {
            let guid = Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99");

            let mut factory: *mut Void = std::ptr::null_mut();
            RoGetActivationFactory(String::from("Windows.UI.Colors").as_raw_handle(), &guid, &mut factory).ok()?;

            IColorsStatics::alice_blue(factory)
        }
    }
}

fn run() -> Result<()> {
    let color = Colors::alice_blue()?;

    assert!(color.a == 255);
    assert!(color.r == 240);
    assert!(color.g == 248);
    assert!(color.b == 255);

    Ok(())
}

fn main() {
    init();

    if let Err(e) = run() {
        e.code().unwrap();
    }

    println!("ok");
}
