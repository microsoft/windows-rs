#[cfg(feature = "Devices_Input_Preview")]
pub mod Preview;
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyboardCapabilities {
    type Vtable = IKeyboardCapabilities_Vtbl;
}
impl ::core::clone::Clone for IKeyboardCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IKeyboardCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a3f9b56_6798_4bbc_833e_0f34b17c65ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub KeyboardPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMouseCapabilities {
    type Vtable = IMouseCapabilities_Vtbl;
}
impl ::core::clone::Clone for IMouseCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMouseCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbca5e023_7dd9_4b6b_9a92_55d43cb38f73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MousePresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub VerticalWheelPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub HorizontalWheelPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SwapButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMouseDevice {
    type Vtable = IMouseDevice_Vtbl;
}
impl ::core::clone::Clone for IMouseDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMouseDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88edf458_f2c8_49f4_be1f_c256b388bc11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MouseMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MouseMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMouseMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMouseMoved: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMouseDeviceStatics {
    type Vtable = IMouseDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IMouseDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMouseDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x484a9045_6d70_49db_8e68_46ffbd17d38d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMouseEventArgs {
    type Vtable = IMouseEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMouseEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMouseEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf625aa5d_2354_4cc7_9230_96941c969fde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MouseDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MouseDelta) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenButtonListener(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenButtonListener {
    type Vtable = IPenButtonListener_Vtbl;
}
impl ::core::clone::Clone for IPenButtonListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenButtonListener {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8245c376_1ee3_53f7_b1f7_8334a16f2815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenButtonListener_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TailButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TailButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTailButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTailButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub TailButtonDoubleClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TailButtonDoubleClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTailButtonDoubleClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTailButtonDoubleClicked: usize,
    #[cfg(feature = "Foundation")]
    pub TailButtonLongPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TailButtonLongPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTailButtonLongPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTailButtonLongPressed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenButtonListenerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenButtonListenerStatics {
    type Vtable = IPenButtonListenerStatics_Vtbl;
}
impl ::core::clone::Clone for IPenButtonListenerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenButtonListenerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19a8a584_862f_5f69_bfea_05f6584f133f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenButtonListenerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenDevice {
    type Vtable = IPenDevice_Vtbl;
}
impl ::core::clone::Clone for IPenDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31856eba_a738_5a8c_b8f6_f97ef68d18ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PenId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDevice2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenDevice2 {
    type Vtable = IPenDevice2_Vtbl;
}
impl ::core::clone::Clone for IPenDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenDevice2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0207d327_7fb8_5566_8c34_f8342037b7f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDevice2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenDeviceStatics {
    type Vtable = IPenDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IPenDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dfbbe01_0966_5180_bcb4_b85060e39479);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetFromPointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDockListener(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenDockListener {
    type Vtable = IPenDockListener_Vtbl;
}
impl ::core::clone::Clone for IPenDockListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenDockListener {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x759f4d90_1dc0_55cb_ad18_b9101456f592);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDockListener_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub Docked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Docked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDocked: usize,
    #[cfg(feature = "Foundation")]
    pub Undocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Undocked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUndocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUndocked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDockListenerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenDockListenerStatics {
    type Vtable = IPenDockListenerStatics_Vtbl;
}
impl ::core::clone::Clone for IPenDockListenerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenDockListenerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcab75e9a_0016_5c72_969e_a97e11992a93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDockListenerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDockedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenDockedEventArgs {
    type Vtable = IPenDockedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPenDockedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenDockedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd4277c6_ca63_5d4e_9ed3_a28a54521c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDockedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenTailButtonClickedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenTailButtonClickedEventArgs {
    type Vtable = IPenTailButtonClickedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPenTailButtonClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenTailButtonClickedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2fb7b6_6ad3_5d3e_ab29_05ea2410e390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenTailButtonClickedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenTailButtonDoubleClickedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenTailButtonDoubleClickedEventArgs {
    type Vtable = IPenTailButtonDoubleClickedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPenTailButtonDoubleClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenTailButtonDoubleClickedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x846321a2_618a_5478_b04c_b358231da4a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenTailButtonDoubleClickedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenTailButtonLongPressedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenTailButtonLongPressedEventArgs {
    type Vtable = IPenTailButtonLongPressedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPenTailButtonLongPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenTailButtonLongPressedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf37c606e_c60a_5f42_b818_a53112406c13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenTailButtonLongPressedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenUndockedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenUndockedEventArgs {
    type Vtable = IPenUndockedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPenUndockedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPenUndockedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccd09150_261b_59e6_a5d4_c1964cd03feb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenUndockedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerDevice {
    type Vtable = IPointerDevice_Vtbl;
}
impl ::core::clone::Clone for IPointerDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPointerDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93c9bafc_ebcb_467e_82c6_276feae36b5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PointerDeviceType) -> ::windows::core::HRESULT,
    pub IsIntegrated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaxContacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PhysicalDeviceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalDeviceRect: usize,
    #[cfg(feature = "Foundation")]
    pub ScreenRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedUsages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedUsages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerDevice2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerDevice2 {
    type Vtable = IPointerDevice2_Vtbl;
}
impl ::core::clone::Clone for IPointerDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPointerDevice2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8a6d2a0_c484_489f_ae3e_30d2ee1ffd3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDevice2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MaxPointersWithZDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerDeviceStatics {
    type Vtable = IPointerDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IPointerDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPointerDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8b89aa1_d1c6_416e_bd8d_5790914dc563);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetPointerDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPointerDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPointerDevices: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITouchCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITouchCapabilities {
    type Vtable = ITouchCapabilities_Vtbl;
}
impl ::core::clone::Clone for ITouchCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITouchCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20dd55f9_13f1_46c8_9285_2c05fa3eda6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITouchCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TouchPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Contacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct KeyboardCapabilities(::windows::core::IUnknown);
impl KeyboardCapabilities {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<KeyboardCapabilities, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn KeyboardPresent(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).KeyboardPresent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for KeyboardCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyboardCapabilities {}
impl ::core::fmt::Debug for KeyboardCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardCapabilities").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for KeyboardCapabilities {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.KeyboardCapabilities;{3a3f9b56-6798-4bbc-833e-0f34b17c65ff})");
}
impl ::core::clone::Clone for KeyboardCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for KeyboardCapabilities {
    type Vtable = IKeyboardCapabilities_Vtbl;
}
unsafe impl ::windows::core::ComInterface for KeyboardCapabilities {
    const IID: ::windows::core::GUID = <IKeyboardCapabilities as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for KeyboardCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.KeyboardCapabilities";
}
::windows::imp::interface_hierarchy!(KeyboardCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for KeyboardCapabilities {}
unsafe impl ::core::marker::Sync for KeyboardCapabilities {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct MouseCapabilities(::windows::core::IUnknown);
impl MouseCapabilities {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MouseCapabilities, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MousePresent(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).MousePresent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VerticalWheelPresent(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).VerticalWheelPresent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HorizontalWheelPresent(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).HorizontalWheelPresent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SwapButtons(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).SwapButtons)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberOfButtons(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).NumberOfButtons)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MouseCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseCapabilities {}
impl ::core::fmt::Debug for MouseCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseCapabilities").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MouseCapabilities {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.MouseCapabilities;{bca5e023-7dd9-4b6b-9a92-55d43cb38f73})");
}
impl ::core::clone::Clone for MouseCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MouseCapabilities {
    type Vtable = IMouseCapabilities_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MouseCapabilities {
    const IID: ::windows::core::GUID = <IMouseCapabilities as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MouseCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.MouseCapabilities";
}
::windows::imp::interface_hierarchy!(MouseCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MouseCapabilities {}
unsafe impl ::core::marker::Sync for MouseCapabilities {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct MouseDevice(::windows::core::IUnknown);
impl MouseDevice {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MouseMoved(&self, handler: &super::super::Foundation::TypedEventHandler<MouseDevice, MouseEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MouseMoved)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMouseMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMouseMoved)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<MouseDevice> {
        Self::IMouseDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<MouseDevice>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMouseDeviceStatics<R, F: FnOnce(&IMouseDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MouseDevice, IMouseDeviceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MouseDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseDevice {}
impl ::core::fmt::Debug for MouseDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseDevice").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MouseDevice {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.MouseDevice;{88edf458-f2c8-49f4-be1f-c256b388bc11})");
}
impl ::core::clone::Clone for MouseDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MouseDevice {
    type Vtable = IMouseDevice_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MouseDevice {
    const IID: ::windows::core::GUID = <IMouseDevice as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MouseDevice {
    const NAME: &'static str = "Windows.Devices.Input.MouseDevice";
}
::windows::imp::interface_hierarchy!(MouseDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct MouseEventArgs(::windows::core::IUnknown);
impl MouseEventArgs {
    pub fn MouseDelta(&self) -> ::windows::core::Result<MouseDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MouseDelta>();
            (::windows::core::Interface::vtable(this).MouseDelta)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MouseEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseEventArgs {}
impl ::core::fmt::Debug for MouseEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MouseEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.MouseEventArgs;{f625aa5d-2354-4cc7-9230-96941c969fde})");
}
impl ::core::clone::Clone for MouseEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MouseEventArgs {
    type Vtable = IMouseEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MouseEventArgs {
    const IID: ::windows::core::GUID = <IMouseEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MouseEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.MouseEventArgs";
}
::windows::imp::interface_hierarchy!(MouseEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct PenButtonListener(::windows::core::IUnknown);
impl PenButtonListener {
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedChanged(&self, handler: &super::super::Foundation::TypedEventHandler<PenButtonListener, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).IsSupportedChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsSupportedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsSupportedChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TailButtonClicked(&self, handler: &super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).TailButtonClicked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTailButtonClicked(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTailButtonClicked)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TailButtonDoubleClicked(&self, handler: &super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).TailButtonDoubleClicked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTailButtonDoubleClicked(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTailButtonDoubleClicked)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TailButtonLongPressed(&self, handler: &super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).TailButtonLongPressed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTailButtonLongPressed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTailButtonLongPressed)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<PenButtonListener> {
        Self::IPenButtonListenerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PenButtonListener>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPenButtonListenerStatics<R, F: FnOnce(&IPenButtonListenerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PenButtonListener, IPenButtonListenerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PenButtonListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenButtonListener {}
impl ::core::fmt::Debug for PenButtonListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenButtonListener").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PenButtonListener {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenButtonListener;{8245c376-1ee3-53f7-b1f7-8334a16f2815})");
}
impl ::core::clone::Clone for PenButtonListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PenButtonListener {
    type Vtable = IPenButtonListener_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PenButtonListener {
    const IID: ::windows::core::GUID = <IPenButtonListener as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PenButtonListener {
    const NAME: &'static str = "Windows.Devices.Input.PenButtonListener";
}
::windows::imp::interface_hierarchy!(PenButtonListener, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PenButtonListener {}
unsafe impl ::core::marker::Sync for PenButtonListener {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct PenDevice(::windows::core::IUnknown);
impl PenDevice {
    pub fn PenId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).PenId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Haptics\"`*"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<super::Haptics::SimpleHapticsController> {
        let this = &::windows::core::ComInterface::cast::<IPenDevice2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Haptics::SimpleHapticsController>();
            (::windows::core::Interface::vtable(this).SimpleHapticsController)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetFromPointerId(pointerid: u32) -> ::windows::core::Result<PenDevice> {
        Self::IPenDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PenDevice>();
            (::windows::core::Interface::vtable(this).GetFromPointerId)(::windows::core::Interface::as_raw(this), pointerid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPenDeviceStatics<R, F: FnOnce(&IPenDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PenDevice, IPenDeviceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PenDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenDevice {}
impl ::core::fmt::Debug for PenDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenDevice").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PenDevice {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenDevice;{31856eba-a738-5a8c-b8f6-f97ef68d18ef})");
}
impl ::core::clone::Clone for PenDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PenDevice {
    type Vtable = IPenDevice_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PenDevice {
    const IID: ::windows::core::GUID = <IPenDevice as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PenDevice {
    const NAME: &'static str = "Windows.Devices.Input.PenDevice";
}
::windows::imp::interface_hierarchy!(PenDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PenDevice {}
unsafe impl ::core::marker::Sync for PenDevice {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct PenDockListener(::windows::core::IUnknown);
impl PenDockListener {
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedChanged(&self, handler: &super::super::Foundation::TypedEventHandler<PenDockListener, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).IsSupportedChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsSupportedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsSupportedChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Docked(&self, handler: &super::super::Foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Docked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDocked(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDocked)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Undocked(&self, handler: &super::super::Foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Undocked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUndocked(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUndocked)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<PenDockListener> {
        Self::IPenDockListenerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PenDockListener>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPenDockListenerStatics<R, F: FnOnce(&IPenDockListenerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PenDockListener, IPenDockListenerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PenDockListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenDockListener {}
impl ::core::fmt::Debug for PenDockListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenDockListener").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PenDockListener {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenDockListener;{759f4d90-1dc0-55cb-ad18-b9101456f592})");
}
impl ::core::clone::Clone for PenDockListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PenDockListener {
    type Vtable = IPenDockListener_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PenDockListener {
    const IID: ::windows::core::GUID = <IPenDockListener as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PenDockListener {
    const NAME: &'static str = "Windows.Devices.Input.PenDockListener";
}
::windows::imp::interface_hierarchy!(PenDockListener, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PenDockListener {}
unsafe impl ::core::marker::Sync for PenDockListener {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct PenDockedEventArgs(::windows::core::IUnknown);
impl PenDockedEventArgs {}
impl ::core::cmp::PartialEq for PenDockedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenDockedEventArgs {}
impl ::core::fmt::Debug for PenDockedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenDockedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PenDockedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenDockedEventArgs;{fd4277c6-ca63-5d4e-9ed3-a28a54521c8c})");
}
impl ::core::clone::Clone for PenDockedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PenDockedEventArgs {
    type Vtable = IPenDockedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PenDockedEventArgs {
    const IID: ::windows::core::GUID = <IPenDockedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PenDockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenDockedEventArgs";
}
::windows::imp::interface_hierarchy!(PenDockedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PenDockedEventArgs {}
unsafe impl ::core::marker::Sync for PenDockedEventArgs {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct PenTailButtonClickedEventArgs(::windows::core::IUnknown);
impl PenTailButtonClickedEventArgs {}
impl ::core::cmp::PartialEq for PenTailButtonClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenTailButtonClickedEventArgs {}
impl ::core::fmt::Debug for PenTailButtonClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTailButtonClickedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PenTailButtonClickedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenTailButtonClickedEventArgs;{5d2fb7b6-6ad3-5d3e-ab29-05ea2410e390})");
}
impl ::core::clone::Clone for PenTailButtonClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PenTailButtonClickedEventArgs {
    type Vtable = IPenTailButtonClickedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PenTailButtonClickedEventArgs {
    const IID: ::windows::core::GUID = <IPenTailButtonClickedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PenTailButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonClickedEventArgs";
}
::windows::imp::interface_hierarchy!(PenTailButtonClickedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PenTailButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for PenTailButtonClickedEventArgs {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct PenTailButtonDoubleClickedEventArgs(::windows::core::IUnknown);
impl PenTailButtonDoubleClickedEventArgs {}
impl ::core::cmp::PartialEq for PenTailButtonDoubleClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenTailButtonDoubleClickedEventArgs {}
impl ::core::fmt::Debug for PenTailButtonDoubleClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTailButtonDoubleClickedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PenTailButtonDoubleClickedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs;{846321a2-618a-5478-b04c-b358231da4a7})");
}
impl ::core::clone::Clone for PenTailButtonDoubleClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PenTailButtonDoubleClickedEventArgs {
    type Vtable = IPenTailButtonDoubleClickedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PenTailButtonDoubleClickedEventArgs {
    const IID: ::windows::core::GUID = <IPenTailButtonDoubleClickedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PenTailButtonDoubleClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs";
}
::windows::imp::interface_hierarchy!(PenTailButtonDoubleClickedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PenTailButtonDoubleClickedEventArgs {}
unsafe impl ::core::marker::Sync for PenTailButtonDoubleClickedEventArgs {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct PenTailButtonLongPressedEventArgs(::windows::core::IUnknown);
impl PenTailButtonLongPressedEventArgs {}
impl ::core::cmp::PartialEq for PenTailButtonLongPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenTailButtonLongPressedEventArgs {}
impl ::core::fmt::Debug for PenTailButtonLongPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTailButtonLongPressedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PenTailButtonLongPressedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenTailButtonLongPressedEventArgs;{f37c606e-c60a-5f42-b818-a53112406c13})");
}
impl ::core::clone::Clone for PenTailButtonLongPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PenTailButtonLongPressedEventArgs {
    type Vtable = IPenTailButtonLongPressedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PenTailButtonLongPressedEventArgs {
    const IID: ::windows::core::GUID = <IPenTailButtonLongPressedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PenTailButtonLongPressedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonLongPressedEventArgs";
}
::windows::imp::interface_hierarchy!(PenTailButtonLongPressedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PenTailButtonLongPressedEventArgs {}
unsafe impl ::core::marker::Sync for PenTailButtonLongPressedEventArgs {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct PenUndockedEventArgs(::windows::core::IUnknown);
impl PenUndockedEventArgs {}
impl ::core::cmp::PartialEq for PenUndockedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenUndockedEventArgs {}
impl ::core::fmt::Debug for PenUndockedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenUndockedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PenUndockedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenUndockedEventArgs;{ccd09150-261b-59e6-a5d4-c1964cd03feb})");
}
impl ::core::clone::Clone for PenUndockedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PenUndockedEventArgs {
    type Vtable = IPenUndockedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PenUndockedEventArgs {
    const IID: ::windows::core::GUID = <IPenUndockedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PenUndockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenUndockedEventArgs";
}
::windows::imp::interface_hierarchy!(PenUndockedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PenUndockedEventArgs {}
unsafe impl ::core::marker::Sync for PenUndockedEventArgs {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct PointerDevice(::windows::core::IUnknown);
impl PointerDevice {
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PointerDeviceType>();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsIntegrated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsIntegrated)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxContacts(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxContacts)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhysicalDeviceRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).PhysicalDeviceRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).ScreenRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedUsages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PointerDeviceUsage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<PointerDeviceUsage>>();
            (::windows::core::Interface::vtable(this).SupportedUsages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxPointersWithZDistance(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IPointerDevice2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxPointersWithZDistance)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPointerDevice(pointerid: u32) -> ::windows::core::Result<PointerDevice> {
        Self::IPointerDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PointerDevice>();
            (::windows::core::Interface::vtable(this).GetPointerDevice)(::windows::core::Interface::as_raw(this), pointerid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPointerDevices() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PointerDevice>> {
        Self::IPointerDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<PointerDevice>>();
            (::windows::core::Interface::vtable(this).GetPointerDevices)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointerDeviceStatics<R, F: FnOnce(&IPointerDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PointerDevice, IPointerDeviceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PointerDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerDevice {}
impl ::core::fmt::Debug for PointerDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDevice").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PointerDevice {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PointerDevice;{93c9bafc-ebcb-467e-82c6-276feae36b5a})");
}
impl ::core::clone::Clone for PointerDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PointerDevice {
    type Vtable = IPointerDevice_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PointerDevice {
    const IID: ::windows::core::GUID = <IPointerDevice as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PointerDevice {
    const NAME: &'static str = "Windows.Devices.Input.PointerDevice";
}
::windows::imp::interface_hierarchy!(PointerDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
pub struct TouchCapabilities(::windows::core::IUnknown);
impl TouchCapabilities {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TouchCapabilities, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TouchPresent(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).TouchPresent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Contacts(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Contacts)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TouchCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TouchCapabilities {}
impl ::core::fmt::Debug for TouchCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TouchCapabilities").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TouchCapabilities {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.TouchCapabilities;{20dd55f9-13f1-46c8-9285-2c05fa3eda6f})");
}
impl ::core::clone::Clone for TouchCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TouchCapabilities {
    type Vtable = ITouchCapabilities_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TouchCapabilities {
    const IID: ::windows::core::GUID = <ITouchCapabilities as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TouchCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.TouchCapabilities";
}
::windows::imp::interface_hierarchy!(TouchCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TouchCapabilities {}
unsafe impl ::core::marker::Sync for TouchCapabilities {}
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PointerDeviceType(pub i32);
impl PointerDeviceType {
    pub const Touch: Self = Self(0i32);
    pub const Pen: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
impl ::core::marker::Copy for PointerDeviceType {}
impl ::core::clone::Clone for PointerDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PointerDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PointerDeviceType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PointerDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDeviceType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PointerDeviceType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Input.PointerDeviceType;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Input\"`*"]
pub struct MouseDelta {
    pub X: i32,
    pub Y: i32,
}
impl ::core::marker::Copy for MouseDelta {}
impl ::core::clone::Clone for MouseDelta {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MouseDelta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MouseDelta").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::windows::core::TypeKind for MouseDelta {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for MouseDelta {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Input.MouseDelta;i4;i4)");
}
impl ::core::cmp::PartialEq for MouseDelta {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for MouseDelta {}
impl ::core::default::Default for MouseDelta {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Input\"`*"]
pub struct PointerDeviceUsage {
    pub UsagePage: u32,
    pub Usage: u32,
    pub MinLogical: i32,
    pub MaxLogical: i32,
    pub MinPhysical: i32,
    pub MaxPhysical: i32,
    pub Unit: u32,
    pub PhysicalMultiplier: f32,
}
impl ::core::marker::Copy for PointerDeviceUsage {}
impl ::core::clone::Clone for PointerDeviceUsage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PointerDeviceUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PointerDeviceUsage").field("UsagePage", &self.UsagePage).field("Usage", &self.Usage).field("MinLogical", &self.MinLogical).field("MaxLogical", &self.MaxLogical).field("MinPhysical", &self.MinPhysical).field("MaxPhysical", &self.MaxPhysical).field("Unit", &self.Unit).field("PhysicalMultiplier", &self.PhysicalMultiplier).finish()
    }
}
impl ::windows::core::TypeKind for PointerDeviceUsage {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for PointerDeviceUsage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Input.PointerDeviceUsage;u4;u4;i4;i4;i4;i4;u4;f4)");
}
impl ::core::cmp::PartialEq for PointerDeviceUsage {
    fn eq(&self, other: &Self) -> bool {
        self.UsagePage == other.UsagePage && self.Usage == other.Usage && self.MinLogical == other.MinLogical && self.MaxLogical == other.MaxLogical && self.MinPhysical == other.MinPhysical && self.MaxPhysical == other.MaxPhysical && self.Unit == other.Unit && self.PhysicalMultiplier == other.PhysicalMultiplier
    }
}
impl ::core::cmp::Eq for PointerDeviceUsage {}
impl ::core::default::Default for PointerDeviceUsage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
