#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBitmapEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayBitmapEffect {
    type Vtable = ILampArrayBitmapEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3238e065_d877_4627_89e5_2a88f7052fa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapEffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBitmapEffectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayBitmapEffectFactory {
    type Vtable = ILampArrayBitmapEffectFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13608090_e336_4c8f_9053_a92407ca7b1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapEffectFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: ::windows::core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBitmapRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayBitmapRequestedEventArgs {
    type Vtable = ILampArrayBitmapRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8b4af9e_fe63_4d51_babd_619defb454ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBlinkEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayBlinkEffect {
    type Vtable = ILampArrayBlinkEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebbf35f6_2fc5_4bb3_b3c3_6221a7680d13);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBlinkEffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayRepetitionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayRepetitionMode) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBlinkEffectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayBlinkEffectFactory {
    type Vtable = ILampArrayBlinkEffectFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x879f1d97_9f50_49b2_a56f_013aa08d55e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBlinkEffectFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: ::windows::core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayColorRampEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayColorRampEffect {
    type Vtable = ILampArrayColorRampEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b004437_40a7_432e_a0b9_0d570c2153ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayColorRampEffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayEffectCompletionBehavior) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayColorRampEffectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayColorRampEffectFactory {
    type Vtable = ILampArrayColorRampEffectFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520bd133_0c74_4df5_bea7_4899e0266b0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayColorRampEffectFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: ::windows::core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayCustomEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayCustomEffect {
    type Vtable = ILampArrayCustomEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec579170_3c34_4876_818b_5765f78b0ee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayCustomEffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayCustomEffectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayCustomEffectFactory {
    type Vtable = ILampArrayCustomEffectFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68b4774d_63e5_4af0_a58b_3e535b94e8c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayCustomEffectFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: ::windows::core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct ILampArrayEffect(::windows::core::IUnknown);
impl ILampArrayEffect {
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn ZIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::convert::From<ILampArrayEffect> for ::windows::core::IInspectable {
    fn from(value: ILampArrayEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILampArrayEffect> for ::windows::core::IInspectable {
    fn from(value: &ILampArrayEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILampArrayEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ILampArrayEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILampArrayEffect> for ::windows::core::IUnknown {
    fn from(value: ILampArrayEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILampArrayEffect> for ::windows::core::IUnknown {
    fn from(value: &ILampArrayEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILampArrayEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILampArrayEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILampArrayEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILampArrayEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILampArrayEffect {}
impl ::core::fmt::Debug for ILampArrayEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILampArrayEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILampArrayEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{11d45590-57fb-4546-b1ce-863107f740df}");
}
unsafe impl ::windows::core::Interface for ILampArrayEffect {
    type Vtable = ILampArrayEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d45590_57fb_4546_b1ce_863107f740df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayEffectPlaylist(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayEffectPlaylist {
    type Vtable = ILampArrayEffectPlaylistVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7de58bfe_6f61_4103_98c7_d6632f7b9169);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffectPlaylistVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zindex: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectStartMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayEffectStartMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayRepetitionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayRepetitionMode) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayEffectPlaylistStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayEffectPlaylistStatics {
    type Vtable = ILampArrayEffectPlaylistStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb15235c_ea35_4c7f_a016_f3bfc6a6c47d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffectPlaylistStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArraySolidEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArraySolidEffect {
    type Vtable = ILampArraySolidEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x441f8213_43cc_4b33_80eb_c6ddde7dc8ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArraySolidEffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayEffectCompletionBehavior) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArraySolidEffectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArraySolidEffectFactory {
    type Vtable = ILampArraySolidEffectFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf862a32c_5576_4341_961b_aee1f13cf9dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArraySolidEffectFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: ::windows::core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayUpdateRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayUpdateRequestedEventArgs {
    type Vtable = ILampArrayUpdateRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73560d6a_576a_48af_8539_67ffa0ab3516);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayUpdateRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lampindex: i32, desiredcolor: super::super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredColors_array_size: u32, desiredcolors: *const super::super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArrayBitmapEffect(::windows::core::IUnknown);
impl LampArrayBitmapEffect {
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartDelay<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUpdateInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SuggestedBitmapSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BitmapRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<LampArrayBitmapEffect, LampArrayBitmapRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBitmapRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayBitmapEffect> {
        Self::ILampArrayBitmapEffectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr()), &mut result__).from_abi::<LampArrayBitmapEffect>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn ZIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayBitmapEffectFactory<R, F: FnOnce(&ILampArrayBitmapEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LampArrayBitmapEffect, ILampArrayBitmapEffectFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArrayBitmapEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayBitmapEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayBitmapEffect {}
impl ::core::fmt::Debug for LampArrayBitmapEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayBitmapEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayBitmapEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBitmapEffect;{3238e065-d877-4627-89e5-2a88f7052fa6})");
}
unsafe impl ::windows::core::Interface for LampArrayBitmapEffect {
    type Vtable = ILampArrayBitmapEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3238e065_d877_4627_89e5_2a88f7052fa6);
}
impl ::windows::core::RuntimeName for LampArrayBitmapEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBitmapEffect";
}
impl ::core::convert::From<LampArrayBitmapEffect> for ::windows::core::IUnknown {
    fn from(value: LampArrayBitmapEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBitmapEffect> for ::windows::core::IUnknown {
    fn from(value: &LampArrayBitmapEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayBitmapEffect> for ::windows::core::IInspectable {
    fn from(value: LampArrayBitmapEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBitmapEffect> for ::windows::core::IInspectable {
    fn from(value: &LampArrayBitmapEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LampArrayBitmapEffect> for ILampArrayEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: LampArrayBitmapEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LampArrayBitmapEffect> for ILampArrayEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &LampArrayBitmapEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILampArrayEffect> for LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ILampArrayEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILampArrayEffect> for &LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ILampArrayEffect> {
        ::core::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArrayBitmapEffect {}
unsafe impl ::core::marker::Sync for LampArrayBitmapEffect {}
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArrayBitmapRequestedEventArgs(::windows::core::IUnknown);
impl LampArrayBitmapRequestedEventArgs {
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SinceStarted(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Graphics_Imaging'*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn UpdateBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), bitmap.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for LampArrayBitmapRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayBitmapRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayBitmapRequestedEventArgs {}
impl ::core::fmt::Debug for LampArrayBitmapRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayBitmapRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayBitmapRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs;{c8b4af9e-fe63-4d51-babd-619defb454ba})");
}
unsafe impl ::windows::core::Interface for LampArrayBitmapRequestedEventArgs {
    type Vtable = ILampArrayBitmapRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8b4af9e_fe63_4d51_babd_619defb454ba);
}
impl ::windows::core::RuntimeName for LampArrayBitmapRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs";
}
impl ::core::convert::From<LampArrayBitmapRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LampArrayBitmapRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBitmapRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LampArrayBitmapRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayBitmapRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LampArrayBitmapRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBitmapRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LampArrayBitmapRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LampArrayBitmapRequestedEventArgs {}
unsafe impl ::core::marker::Sync for LampArrayBitmapRequestedEventArgs {}
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArrayBlinkEffect(::windows::core::IUnknown);
impl LampArrayBlinkEffect {
    #[doc = "*Required features: 'Devices_Lights_Effects', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows::core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AttackDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAttackDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SustainDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSustainDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DecayDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDecayDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RepetitionDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRepetitionDelay<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartDelay<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn Occurrences(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetOccurrences(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn RepetitionMode(&self) -> ::windows::core::Result<LampArrayRepetitionMode> {
        let this = self;
        unsafe {
            let mut result__: LampArrayRepetitionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayRepetitionMode>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayBlinkEffect> {
        Self::ILampArrayBlinkEffectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr()), &mut result__).from_abi::<LampArrayBlinkEffect>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn ZIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayBlinkEffectFactory<R, F: FnOnce(&ILampArrayBlinkEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LampArrayBlinkEffect, ILampArrayBlinkEffectFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArrayBlinkEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayBlinkEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayBlinkEffect {}
impl ::core::fmt::Debug for LampArrayBlinkEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayBlinkEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayBlinkEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBlinkEffect;{ebbf35f6-2fc5-4bb3-b3c3-6221a7680d13})");
}
unsafe impl ::windows::core::Interface for LampArrayBlinkEffect {
    type Vtable = ILampArrayBlinkEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebbf35f6_2fc5_4bb3_b3c3_6221a7680d13);
}
impl ::windows::core::RuntimeName for LampArrayBlinkEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBlinkEffect";
}
impl ::core::convert::From<LampArrayBlinkEffect> for ::windows::core::IUnknown {
    fn from(value: LampArrayBlinkEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBlinkEffect> for ::windows::core::IUnknown {
    fn from(value: &LampArrayBlinkEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayBlinkEffect> for ::windows::core::IInspectable {
    fn from(value: LampArrayBlinkEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBlinkEffect> for ::windows::core::IInspectable {
    fn from(value: &LampArrayBlinkEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LampArrayBlinkEffect> for ILampArrayEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: LampArrayBlinkEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LampArrayBlinkEffect> for ILampArrayEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &LampArrayBlinkEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILampArrayEffect> for LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ILampArrayEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILampArrayEffect> for &LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ILampArrayEffect> {
        ::core::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArrayBlinkEffect {}
unsafe impl ::core::marker::Sync for LampArrayBlinkEffect {}
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArrayColorRampEffect(::windows::core::IUnknown);
impl LampArrayColorRampEffect {
    #[doc = "*Required features: 'Devices_Lights_Effects', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows::core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RampDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRampDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartDelay<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn CompletionBehavior(&self) -> ::windows::core::Result<LampArrayEffectCompletionBehavior> {
        let this = self;
        unsafe {
            let mut result__: LampArrayEffectCompletionBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayEffectCompletionBehavior>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayColorRampEffect> {
        Self::ILampArrayColorRampEffectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr()), &mut result__).from_abi::<LampArrayColorRampEffect>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn ZIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayColorRampEffectFactory<R, F: FnOnce(&ILampArrayColorRampEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LampArrayColorRampEffect, ILampArrayColorRampEffectFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArrayColorRampEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayColorRampEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayColorRampEffect {}
impl ::core::fmt::Debug for LampArrayColorRampEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayColorRampEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayColorRampEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayColorRampEffect;{2b004437-40a7-432e-a0b9-0d570c2153ff})");
}
unsafe impl ::windows::core::Interface for LampArrayColorRampEffect {
    type Vtable = ILampArrayColorRampEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b004437_40a7_432e_a0b9_0d570c2153ff);
}
impl ::windows::core::RuntimeName for LampArrayColorRampEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayColorRampEffect";
}
impl ::core::convert::From<LampArrayColorRampEffect> for ::windows::core::IUnknown {
    fn from(value: LampArrayColorRampEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayColorRampEffect> for ::windows::core::IUnknown {
    fn from(value: &LampArrayColorRampEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayColorRampEffect> for ::windows::core::IInspectable {
    fn from(value: LampArrayColorRampEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayColorRampEffect> for ::windows::core::IInspectable {
    fn from(value: &LampArrayColorRampEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LampArrayColorRampEffect> for ILampArrayEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: LampArrayColorRampEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LampArrayColorRampEffect> for ILampArrayEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &LampArrayColorRampEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILampArrayEffect> for LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ILampArrayEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILampArrayEffect> for &LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ILampArrayEffect> {
        ::core::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArrayColorRampEffect {}
unsafe impl ::core::marker::Sync for LampArrayColorRampEffect {}
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArrayCustomEffect(::windows::core::IUnknown);
impl LampArrayCustomEffect {
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUpdateInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<LampArrayCustomEffect, LampArrayUpdateRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdateRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayCustomEffect> {
        Self::ILampArrayCustomEffectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr()), &mut result__).from_abi::<LampArrayCustomEffect>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn ZIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayCustomEffectFactory<R, F: FnOnce(&ILampArrayCustomEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LampArrayCustomEffect, ILampArrayCustomEffectFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArrayCustomEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayCustomEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayCustomEffect {}
impl ::core::fmt::Debug for LampArrayCustomEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayCustomEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayCustomEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayCustomEffect;{ec579170-3c34-4876-818b-5765f78b0ee4})");
}
unsafe impl ::windows::core::Interface for LampArrayCustomEffect {
    type Vtable = ILampArrayCustomEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec579170_3c34_4876_818b_5765f78b0ee4);
}
impl ::windows::core::RuntimeName for LampArrayCustomEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayCustomEffect";
}
impl ::core::convert::From<LampArrayCustomEffect> for ::windows::core::IUnknown {
    fn from(value: LampArrayCustomEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayCustomEffect> for ::windows::core::IUnknown {
    fn from(value: &LampArrayCustomEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LampArrayCustomEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LampArrayCustomEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayCustomEffect> for ::windows::core::IInspectable {
    fn from(value: LampArrayCustomEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayCustomEffect> for ::windows::core::IInspectable {
    fn from(value: &LampArrayCustomEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LampArrayCustomEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LampArrayCustomEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LampArrayCustomEffect> for ILampArrayEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: LampArrayCustomEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LampArrayCustomEffect> for ILampArrayEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &LampArrayCustomEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILampArrayEffect> for LampArrayCustomEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ILampArrayEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILampArrayEffect> for &LampArrayCustomEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ILampArrayEffect> {
        ::core::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArrayCustomEffect {}
unsafe impl ::core::marker::Sync for LampArrayCustomEffect {}
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArrayEffectCompletionBehavior(pub i32);
impl LampArrayEffectCompletionBehavior {
    pub const ClearState: Self = Self(0i32);
    pub const KeepState: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayEffectCompletionBehavior {}
impl ::core::clone::Clone for LampArrayEffectCompletionBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LampArrayEffectCompletionBehavior {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LampArrayEffectCompletionBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayEffectCompletionBehavior {}
impl ::core::fmt::Debug for LampArrayEffectCompletionBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayEffectCompletionBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayEffectCompletionBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayEffectCompletionBehavior;i4)");
}
impl ::windows::core::DefaultType for LampArrayEffectCompletionBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArrayEffectPlaylist(::windows::core::IUnknown);
impl LampArrayEffectPlaylist {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LampArrayEffectPlaylist, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<ILampArrayEffect>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<ILampArrayEffect>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ILampArrayEffect>>(&self, effect: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), effect.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn OverrideZIndex(&self, zindex: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), zindex).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn EffectStartMode(&self) -> ::windows::core::Result<LampArrayEffectStartMode> {
        let this = self;
        unsafe {
            let mut result__: LampArrayEffectStartMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayEffectStartMode>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetEffectStartMode(&self, value: LampArrayEffectStartMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn Occurrences(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetOccurrences(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn RepetitionMode(&self) -> ::windows::core::Result<LampArrayRepetitionMode> {
        let this = self;
        unsafe {
            let mut result__: LampArrayRepetitionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayRepetitionMode>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartAll<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>>(value: Param0) -> ::windows::core::Result<()> {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StopAll<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>>(value: Param0) -> ::windows::core::Result<()> {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PauseAll<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>>(value: Param0) -> ::windows::core::Result<()> {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<ILampArrayEffect> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ILampArrayEffect>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ILampArrayEffect>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ILampArrayEffect as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ILampArrayEffectPlaylistStatics<R, F: FnOnce(&ILampArrayEffectPlaylistStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LampArrayEffectPlaylist, ILampArrayEffectPlaylistStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArrayEffectPlaylist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayEffectPlaylist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayEffectPlaylist {}
impl ::core::fmt::Debug for LampArrayEffectPlaylist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayEffectPlaylist").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayEffectPlaylist {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayEffectPlaylist;{7de58bfe-6f61-4103-98c7-d6632f7b9169})");
}
unsafe impl ::windows::core::Interface for LampArrayEffectPlaylist {
    type Vtable = ILampArrayEffectPlaylistVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7de58bfe_6f61_4103_98c7_d6632f7b9169);
}
impl ::windows::core::RuntimeName for LampArrayEffectPlaylist {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayEffectPlaylist";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for LampArrayEffectPlaylist {
    type Item = ILampArrayEffect;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &LampArrayEffectPlaylist {
    type Item = ILampArrayEffect;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<LampArrayEffectPlaylist> for ::windows::core::IUnknown {
    fn from(value: LampArrayEffectPlaylist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayEffectPlaylist> for ::windows::core::IUnknown {
    fn from(value: &LampArrayEffectPlaylist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayEffectPlaylist> for ::windows::core::IInspectable {
    fn from(value: LampArrayEffectPlaylist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayEffectPlaylist> for ::windows::core::IInspectable {
    fn from(value: &LampArrayEffectPlaylist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<LampArrayEffectPlaylist> for super::super::super::Foundation::Collections::IIterable<ILampArrayEffect> {
    type Error = ::windows::core::Error;
    fn try_from(value: LampArrayEffectPlaylist) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&LampArrayEffectPlaylist> for super::super::super::Foundation::Collections::IIterable<ILampArrayEffect> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LampArrayEffectPlaylist) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>> for &LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<LampArrayEffectPlaylist> for super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect> {
    type Error = ::windows::core::Error;
    fn try_from(value: LampArrayEffectPlaylist) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&LampArrayEffectPlaylist> for super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LampArrayEffectPlaylist) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>> for &LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArrayEffectPlaylist {}
unsafe impl ::core::marker::Sync for LampArrayEffectPlaylist {}
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArrayEffectStartMode(pub i32);
impl LampArrayEffectStartMode {
    pub const Sequential: Self = Self(0i32);
    pub const Simultaneous: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayEffectStartMode {}
impl ::core::clone::Clone for LampArrayEffectStartMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LampArrayEffectStartMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LampArrayEffectStartMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayEffectStartMode {}
impl ::core::fmt::Debug for LampArrayEffectStartMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayEffectStartMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayEffectStartMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayEffectStartMode;i4)");
}
impl ::windows::core::DefaultType for LampArrayEffectStartMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArrayRepetitionMode(pub i32);
impl LampArrayRepetitionMode {
    pub const Occurrences: Self = Self(0i32);
    pub const Forever: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayRepetitionMode {}
impl ::core::clone::Clone for LampArrayRepetitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LampArrayRepetitionMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LampArrayRepetitionMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayRepetitionMode {}
impl ::core::fmt::Debug for LampArrayRepetitionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayRepetitionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayRepetitionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayRepetitionMode;i4)");
}
impl ::windows::core::DefaultType for LampArrayRepetitionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArraySolidEffect(::windows::core::IUnknown);
impl LampArraySolidEffect {
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn ZIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows::core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartDelay<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn CompletionBehavior(&self) -> ::windows::core::Result<LampArrayEffectCompletionBehavior> {
        let this = self;
        unsafe {
            let mut result__: LampArrayEffectCompletionBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayEffectCompletionBehavior>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects'*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArraySolidEffect> {
        Self::ILampArraySolidEffectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr()), &mut result__).from_abi::<LampArraySolidEffect>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILampArraySolidEffectFactory<R, F: FnOnce(&ILampArraySolidEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LampArraySolidEffect, ILampArraySolidEffectFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArraySolidEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArraySolidEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArraySolidEffect {}
impl ::core::fmt::Debug for LampArraySolidEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArraySolidEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArraySolidEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArraySolidEffect;{441f8213-43cc-4b33-80eb-c6ddde7dc8ed})");
}
unsafe impl ::windows::core::Interface for LampArraySolidEffect {
    type Vtable = ILampArraySolidEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x441f8213_43cc_4b33_80eb_c6ddde7dc8ed);
}
impl ::windows::core::RuntimeName for LampArraySolidEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArraySolidEffect";
}
impl ::core::convert::From<LampArraySolidEffect> for ::windows::core::IUnknown {
    fn from(value: LampArraySolidEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArraySolidEffect> for ::windows::core::IUnknown {
    fn from(value: &LampArraySolidEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LampArraySolidEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LampArraySolidEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArraySolidEffect> for ::windows::core::IInspectable {
    fn from(value: LampArraySolidEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArraySolidEffect> for ::windows::core::IInspectable {
    fn from(value: &LampArraySolidEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LampArraySolidEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LampArraySolidEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LampArraySolidEffect> for ILampArrayEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: LampArraySolidEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LampArraySolidEffect> for ILampArrayEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &LampArraySolidEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILampArrayEffect> for LampArraySolidEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ILampArrayEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILampArrayEffect> for &LampArraySolidEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ILampArrayEffect> {
        ::core::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArraySolidEffect {}
unsafe impl ::core::marker::Sync for LampArraySolidEffect {}
#[doc = "*Required features: 'Devices_Lights_Effects'*"]
#[repr(transparent)]
pub struct LampArrayUpdateRequestedEventArgs(::windows::core::IUnknown);
impl LampArrayUpdateRequestedEventArgs {
    #[doc = "*Required features: 'Devices_Lights_Effects', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SinceStarted(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::Color>>(&self, desiredcolor: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredcolor.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetColorForIndex<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::UI::Color>>(&self, lampindex: i32, desiredcolor: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), lampindex, desiredcolor.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetSingleColorForIndices<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::Color>>(&self, desiredcolor: Param0, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), desiredcolor.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr())).ok() }
    }
    #[doc = "*Required features: 'Devices_Lights_Effects', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetColorsForIndices(&self, desiredcolors: &[<super::super::super::UI::Color as ::windows::core::DefaultType>::DefaultType], lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), desiredcolors.len() as u32, ::core::mem::transmute(desiredcolors.as_ptr()), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for LampArrayUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayUpdateRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayUpdateRequestedEventArgs {}
impl ::core::fmt::Debug for LampArrayUpdateRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayUpdateRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayUpdateRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs;{73560d6a-576a-48af-8539-67ffa0ab3516})");
}
unsafe impl ::windows::core::Interface for LampArrayUpdateRequestedEventArgs {
    type Vtable = ILampArrayUpdateRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73560d6a_576a_48af_8539_67ffa0ab3516);
}
impl ::windows::core::RuntimeName for LampArrayUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs";
}
impl ::core::convert::From<LampArrayUpdateRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LampArrayUpdateRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayUpdateRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LampArrayUpdateRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayUpdateRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LampArrayUpdateRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayUpdateRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LampArrayUpdateRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LampArrayUpdateRequestedEventArgs {}
unsafe impl ::core::marker::Sync for LampArrayUpdateRequestedEventArgs {}
