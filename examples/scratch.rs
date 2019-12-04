//#[winrt_macros::echo_target]
use winrt::*;
use abi::Void;

#[repr(C)]
#[derive(Default)]
pub struct Color {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

struct Colors;

struct IColorsStatics {
    ptr: *const Void,
}

impl IColorsStatics {
    pub fn alice_blue(&self) -> Result<Color> {
        unsafe {
            let mut color = Default::default();
            ((*(*(self.ptr as *const *const winrt_impl::IColorsStatics))).alice_blue)(self.ptr, &mut color).ok_or(color)
        }
    }
}

impl Drop for IColorsStatics {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            abi::IUnknown::release(self.ptr);
        }
    }
}

impl Colors {
    fn alice_blue() -> Result<Color> {
        factory::<Colors, IColorsStatics>()?.alice_blue()
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
        pub alice_blue: extern "system" fn(*const Void, value: &mut Color) -> ErrorCode,
    }

    impl TypeInterface for super::IColorsStatics {
        fn type_guid() -> &'static Guid {
            static GUID: Guid = Guid::from_values(0xCFF52E04, 0xCCA6, 0x4614, &[0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99]);
            &GUID
        }
        fn take_ownership(ptr: *const Void) -> Self {
            Self { ptr }
        }
    }

    impl TypeName for Colors {
        fn type_name() -> &'static str {
            "Windows.UI.Colors"
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
