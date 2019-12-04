//#[winrt_macros::echo_target]

mod windows { pub mod ui {

#[repr(C)]
#[derive(Default)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Colors;

// Only make the interface public if it is not exclusive/overridable.
struct IColorsStatics {
    ptr: *const winrt::abi::Void,
}

impl IColorsStatics {
    pub fn alice_blue(&self) -> winrt::Result<Color> {
        unsafe {
            let mut color = Default::default();
            ((*(*(self.ptr as *const *const winrt_impl::IColorsStatics))).alice_blue)(self.ptr, &mut color).ok_or(color)
        }
    }
}

impl Drop for IColorsStatics {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            winrt::abi::IUnknown::release(self.ptr);
        }
    }
}

impl Colors {
    pub fn alice_blue() -> winrt::Result<Color> {
        winrt::factory::<Colors, IColorsStatics>()?.alice_blue()
    }
}


// This should be generated in such a sub module to avoid making it public to the rest of the package
// in which it may be generated.
mod winrt_impl {
    use super::*;

    #[repr(C)]
    pub struct IColorsStatics {
        impl_0: usize,
        impl_1: usize,
        impl_2: usize,
        impl_3: usize,
        impl_4: usize,
        impl_5: usize,
        pub alice_blue: extern "system" fn(*const winrt::abi::Void, value: &mut Color) -> winrt::ErrorCode,
    }

    impl winrt::TypeInterface for super::IColorsStatics {
        fn type_guid() -> &'static winrt::Guid {
            // TODO: Not sure if this should be static or const. Either way, we need it to be the equivalent of "constexpr" in C++
            // so that it's baked into the resulting binary and merely inlines as an address (e.g. __uuidof).
            const GUID: winrt::Guid = winrt::Guid::from_values(0xCFF52E04, 0xCCA6, 0x4614, &[0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99]);
            &GUID
        }

        fn take_ownership(ptr: *const winrt::abi::Void) -> Self {
            Self { ptr }
        }
    }

    impl winrt::TypeName for Colors {
        fn type_name() -> &'static str {
            "Windows.UI.Colors"
        }
    }
}

}}

use winrt::*;
use windows::ui::*;

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
