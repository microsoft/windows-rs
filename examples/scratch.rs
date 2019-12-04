// TODO: the entire `pub mod windows` block should be generated based on metadata using the winmd parser.
pub mod windows {
    pub mod ui {
        #[repr(C)]
        #[derive(Default)]
        pub struct Color {
            pub a: u8,
            pub r: u8,
            pub g: u8,
            pub b: u8,
        }

        pub struct Colors;

        pub struct IColorsStatics {
            ptr: *const std::ffi::c_void,
        }

        #[repr(C)]
        struct abi_IColorsStatics {
            abi_0: usize,
            abi_1: usize,
            abi_2: usize,
            abi_3: usize,
            abi_4: usize,
            abi_5: usize,
            alice_blue: extern "system" fn(*const std::ffi::c_void, &mut Color) -> winrt::ErrorCode,
        }

        impl IColorsStatics {
            pub fn alice_blue(&self) -> winrt::Result<Color> {
                unsafe {
                    let mut color = Default::default();
                    ((*(*(self.ptr as *const *const abi_IColorsStatics))).alice_blue)(
                        self.ptr, &mut color,
                    )
                    .ok_or(color)
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

        impl winrt::TypeInterface for IColorsStatics {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    0xCFF52E04,
                    0xCCA6,
                    0x4614,
                    &[0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99],
                );
                &GUID
            }

            fn take_ownership(ptr: *const std::ffi::c_void) -> Self {
                Self { ptr }
            }
        }

        impl winrt::TypeName for Colors {
            fn type_name() -> &'static str {
                "Windows.UI.Colors"
            }
        }
    }
}

fn run() -> winrt::Result<()> {
    use windows::ui::*;
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
