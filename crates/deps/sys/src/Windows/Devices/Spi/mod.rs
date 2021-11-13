#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_Spi_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpiBusInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpiBusInfo {}
impl ::core::clone::Clone for ISpiBusInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpiConnectionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpiConnectionSettings {}
impl ::core::clone::Clone for ISpiConnectionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpiConnectionSettingsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpiConnectionSettingsFactory {}
impl ::core::clone::Clone for ISpiConnectionSettingsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpiController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpiController {}
impl ::core::clone::Clone for ISpiController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpiControllerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpiControllerStatics {}
impl ::core::clone::Clone for ISpiControllerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpiDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpiDevice {}
impl ::core::clone::Clone for ISpiDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpiDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpiDeviceStatics {}
impl ::core::clone::Clone for ISpiDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpiBusInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpiBusInfo {}
impl ::core::clone::Clone for SpiBusInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpiConnectionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpiConnectionSettings {}
impl ::core::clone::Clone for SpiConnectionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpiController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpiController {}
impl ::core::clone::Clone for SpiController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpiDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpiDevice {}
impl ::core::clone::Clone for SpiDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpiMode(pub i32);
impl SpiMode {
    pub const Mode0: Self = Self(0i32);
    pub const Mode1: Self = Self(1i32);
    pub const Mode2: Self = Self(2i32);
    pub const Mode3: Self = Self(3i32);
}
impl ::core::marker::Copy for SpiMode {}
impl ::core::clone::Clone for SpiMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpiSharingMode(pub i32);
impl SpiSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for SpiSharingMode {}
impl ::core::clone::Clone for SpiSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
