//#[winrt_macros::echo_target]
use winrt::*;

trait TypeGuid {
    fn type_guid() -> &'static Guid;
}

trait TypeName {
    fn type_name() -> &'static str;
}

#[repr(C)]
#[derive(Default)]
pub struct Color {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

#[repr(C)]
struct abi_IUnknown {
    query: extern "system" fn(*const Void, &Guid, *mut *mut Void) -> ErrorCode,
    addref: extern "system" fn(*const Void) -> u32,
    release: extern "system" fn(*const Void) -> u32,
}

impl abi_IUnknown {
    fn query(ptr: *const Void, guid: &Guid) -> *const Void {
        unsafe {
            let mut result: *mut Void = std::ptr::null_mut();
            ((*(*(ptr as *const *const Self))).query)(ptr, guid, &mut result);
            result
        }
    }

    fn addref(ptr: *const Void) -> u32 {
        unsafe { ((*(*(ptr as *const *const Self))).addref)(ptr) }
    }

    fn release(ptr: *const Void) -> u32 {
        unsafe { ((*(*(ptr as *const *const Self))).release)(ptr) }
    }
}

#[repr(C)]
struct abi_IInspectable {
    impl_0: usize,
    impl_1: usize,
    impl_2: usize,
    impl_3: usize,
    type_name: extern "system" fn(*const Void, *mut *mut Void) -> ErrorCode,
}

impl abi_IInspectable {
    fn type_name(ptr: *const Void) -> String {
        unsafe {
            let mut hstring: *mut Void = std::ptr::null_mut();
            ((*(*(ptr as *const *const Self))).type_name)(ptr, &mut hstring);
            String { hstring }
        }
    }
}

#[repr(C)]
pub struct abi_IColorsStatics {
    impl_0: usize,
    impl_1: usize,
    impl_2: usize,
    impl_3: usize,
    impl_4: usize,
    impl_5: usize,
    alice_blue: extern "system" fn(*const Void, value: &mut Color) -> ErrorCode,
}

impl abi_IColorsStatics {
    pub fn alice_blue(ptr: *const Void) -> Result<Color> {
        unsafe {
            let mut color = Default::default();
            ((*(*(ptr as *const *const Self))).alice_blue)(ptr, &mut color).ok_or(color)
        }
    }
}

struct IColorsStatics {
    ptr: *const Void,
}

impl IColorsStatics {
    pub fn alice_blue(&self) -> Result<Color> {
        unsafe {
            let mut color = Default::default();
            ((*(*(self.ptr as *const *const abi_IColorsStatics))).alice_blue)(self.ptr, &mut color).ok_or(color)
        }
    }
}

impl Drop for IColorsStatics {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            abi_IUnknown::release(self.ptr);
        }
    }
}

impl TypeGuid for IColorsStatics {
    fn type_guid() -> &'static Guid {
        static GUID: Guid = Guid::from_values(0xCFF52E04,0xCCA6,0x4614,&[ 0xA1,0x7E,0x75,0x49,0x10,0xC8,0x4A,0x99 ]);
        &GUID
    }
}

struct Colors;

impl TypeName for Colors {
    fn type_name() -> &'static str {
        "Windows.UI.Colors"
    }
}

impl Colors {
    fn alice_blue() -> Result<Color> {
        unsafe {
            let mut ptr: *mut Void = std::ptr::null_mut();
            RoGetActivationFactory(String::from(Colors::type_name()).as_raw_handle(), IColorsStatics::type_guid(), &mut ptr).ok()?;
            let statics = IColorsStatics{ ptr };

            statics.alice_blue()
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
