#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct GameControllerVersionInfo {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl ::core::marker::Copy for GameControllerVersionInfo {}
impl ::core::clone::Clone for GameControllerVersionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GipFirmwareUpdateProgress {
    pub PercentCompleted: f64,
    pub CurrentComponentId: u32,
}
impl ::core::marker::Copy for GipFirmwareUpdateProgress {}
impl ::core::clone::Clone for GipFirmwareUpdateProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GipFirmwareUpdateResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GipFirmwareUpdateResult {}
impl ::core::clone::Clone for GipFirmwareUpdateResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GipFirmwareUpdateStatus(pub i32);
impl GipFirmwareUpdateStatus {
    pub const Completed: Self = Self(0i32);
    pub const UpToDate: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for GipFirmwareUpdateStatus {}
impl ::core::clone::Clone for GipFirmwareUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GipGameControllerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GipGameControllerProvider {}
impl ::core::clone::Clone for GipGameControllerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GipMessageClass(pub i32);
impl GipMessageClass {
    pub const Command: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
    pub const StandardLatency: Self = Self(2i32);
}
impl ::core::marker::Copy for GipMessageClass {}
impl ::core::clone::Clone for GipMessageClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidGameControllerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidGameControllerProvider {}
impl ::core::clone::Clone for HidGameControllerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomGameControllerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomGameControllerFactory {}
impl ::core::clone::Clone for ICustomGameControllerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameControllerFactoryManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameControllerFactoryManagerStatics {}
impl ::core::clone::Clone for IGameControllerFactoryManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameControllerFactoryManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameControllerFactoryManagerStatics2 {}
impl ::core::clone::Clone for IGameControllerFactoryManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameControllerInputSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameControllerInputSink {}
impl ::core::clone::Clone for IGameControllerInputSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameControllerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameControllerProvider {}
impl ::core::clone::Clone for IGameControllerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGipFirmwareUpdateResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGipFirmwareUpdateResult {}
impl ::core::clone::Clone for IGipFirmwareUpdateResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGipGameControllerInputSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGipGameControllerInputSink {}
impl ::core::clone::Clone for IGipGameControllerInputSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGipGameControllerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGipGameControllerProvider {}
impl ::core::clone::Clone for IGipGameControllerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidGameControllerInputSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidGameControllerInputSink {}
impl ::core::clone::Clone for IHidGameControllerInputSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidGameControllerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidGameControllerProvider {}
impl ::core::clone::Clone for IHidGameControllerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXusbGameControllerInputSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXusbGameControllerInputSink {}
impl ::core::clone::Clone for IXusbGameControllerInputSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXusbGameControllerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXusbGameControllerProvider {}
impl ::core::clone::Clone for IXusbGameControllerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XusbDeviceSubtype(pub i32);
impl XusbDeviceSubtype {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
    pub const ArcadePad: Self = Self(2i32);
    pub const ArcadeStick: Self = Self(3i32);
    pub const FlightStick: Self = Self(4i32);
    pub const Wheel: Self = Self(5i32);
    pub const Guitar: Self = Self(6i32);
    pub const GuitarAlternate: Self = Self(7i32);
    pub const GuitarBass: Self = Self(8i32);
    pub const DrumKit: Self = Self(9i32);
    pub const DancePad: Self = Self(10i32);
}
impl ::core::marker::Copy for XusbDeviceSubtype {}
impl ::core::clone::Clone for XusbDeviceSubtype {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XusbDeviceType(pub i32);
impl XusbDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
}
impl ::core::marker::Copy for XusbDeviceType {}
impl ::core::clone::Clone for XusbDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XusbGameControllerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XusbGameControllerProvider {}
impl ::core::clone::Clone for XusbGameControllerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
