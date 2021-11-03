#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayBitmapEffect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayBitmapEffect {
    type Vtable = ILampArrayBitmapEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(842588261, 55415, 17959, [137, 229, 42, 136, 247, 5, 47, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayBitmapEffectFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayBitmapEffectFactory {
    type Vtable = ILampArrayBitmapEffectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(325091472, 58166, 19599, [144, 83, 169, 36, 7, 202, 123, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lamparray: ::windows::runtime::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayBitmapRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayBitmapRequestedEventArgs {
    type Vtable = ILampArrayBitmapRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3367284638, 65123, 19793, [186, 189, 97, 157, 239, 180, 84, 186]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayBlinkEffect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayBlinkEffect {
    type Vtable = ILampArrayBlinkEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3955176950, 12229, 19379, [179, 195, 98, 33, 167, 104, 13, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBlinkEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LampArrayRepetitionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: LampArrayRepetitionMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayBlinkEffectFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayBlinkEffectFactory {
    type Vtable = ILampArrayBlinkEffectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275351959, 40784, 18866, [165, 111, 1, 58, 160, 141, 85, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBlinkEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lamparray: ::windows::runtime::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayColorRampEffect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayColorRampEffect {
    type Vtable = ILampArrayColorRampEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(721437751, 16551, 17198, [160, 185, 13, 87, 12, 33, 83, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayColorRampEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: LampArrayEffectCompletionBehavior) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayColorRampEffectFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayColorRampEffectFactory {
    type Vtable = ILampArrayColorRampEffectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1376506163, 3188, 19957, [190, 167, 72, 153, 224, 38, 107, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayColorRampEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lamparray: ::windows::runtime::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayCustomEffect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayCustomEffect {
    type Vtable = ILampArrayCustomEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3965161840, 15412, 18550, [129, 139, 87, 101, 247, 139, 14, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayCustomEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayCustomEffectFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayCustomEffectFactory {
    type Vtable = ILampArrayCustomEffectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1756657485, 25573, 19184, [165, 139, 62, 83, 91, 148, 232, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayCustomEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lamparray: ::windows::runtime::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Lights_Effects`*"]
pub struct ILampArrayEffect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayEffect {
    type Vtable = ILampArrayEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(299128208, 22523, 17734, [177, 206, 134, 49, 7, 247, 64, 223]);
}
impl ILampArrayEffect {
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn ZIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILampArrayEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{11d45590-57fb-4546-b1ce-863107f740df}");
}
impl ::std::convert::From<ILampArrayEffect> for ::windows::runtime::IUnknown {
    fn from(value: ILampArrayEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILampArrayEffect> for ::windows::runtime::IUnknown {
    fn from(value: &ILampArrayEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILampArrayEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ILampArrayEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ILampArrayEffect> for ::windows::runtime::IInspectable {
    fn from(value: ILampArrayEffect) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ILampArrayEffect> for ::windows::runtime::IInspectable {
    fn from(value: &ILampArrayEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILampArrayEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILampArrayEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayEffectPlaylist(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayEffectPlaylist {
    type Vtable = ILampArrayEffectPlaylist_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2112195582, 28513, 16643, [152, 199, 214, 99, 47, 123, 145, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffectPlaylist_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effect: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, zindex: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LampArrayEffectStartMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: LampArrayEffectStartMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LampArrayRepetitionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: LampArrayRepetitionMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayEffectPlaylistStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayEffectPlaylistStatics {
    type Vtable = ILampArrayEffectPlaylistStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4212466524, 59957, 19583, [160, 22, 243, 191, 198, 166, 196, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffectPlaylistStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArraySolidEffect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArraySolidEffect {
    type Vtable = ILampArraySolidEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1142915603, 17356, 19251, [128, 235, 198, 221, 222, 125, 200, 237]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArraySolidEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: LampArrayEffectCompletionBehavior) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArraySolidEffectFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArraySolidEffectFactory {
    type Vtable = ILampArraySolidEffectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4167213868, 21878, 17217, [150, 27, 174, 225, 241, 60, 249, 221]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArraySolidEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lamparray: ::windows::runtime::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILampArrayUpdateRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILampArrayUpdateRequestedEventArgs {
    type Vtable = ILampArrayUpdateRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935019370, 22378, 18607, [133, 57, 103, 255, 160, 171, 53, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayUpdateRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredcolor: super::super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lampindex: i32, desiredcolor: super::super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredcolor: super::super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredColors_array_size: u32, desiredcolors: *const super::super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampArrayBitmapEffect(::windows::runtime::IInspectable);
impl LampArrayBitmapEffect {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn StartDelay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetStartDelay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn UpdateInterval(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetUpdateInterval<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SuggestedBitmapSize(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn BitmapRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<LampArrayBitmapEffect, LampArrayBitmapRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn RemoveBitmapRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn ZIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<LampArrayBitmapEffect> {
        Self::ILampArrayBitmapEffectFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::std::mem::transmute(lampindexes.as_ptr()), &mut result__).from_abi::<LampArrayBitmapEffect>(result__)
        })
    }
    pub fn ILampArrayBitmapEffectFactory<R, F: FnOnce(&ILampArrayBitmapEffectFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LampArrayBitmapEffect, ILampArrayBitmapEffectFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayBitmapEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBitmapEffect;{3238e065-d877-4627-89e5-2a88f7052fa6})");
}
unsafe impl ::windows::runtime::Interface for LampArrayBitmapEffect {
    type Vtable = ILampArrayBitmapEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(842588261, 55415, 17959, [137, 229, 42, 136, 247, 5, 47, 166]);
}
impl ::windows::runtime::RuntimeName for LampArrayBitmapEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBitmapEffect";
}
impl ::std::convert::From<LampArrayBitmapEffect> for ::windows::runtime::IUnknown {
    fn from(value: LampArrayBitmapEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampArrayBitmapEffect> for ::windows::runtime::IUnknown {
    fn from(value: &LampArrayBitmapEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampArrayBitmapEffect> for ::windows::runtime::IInspectable {
    fn from(value: LampArrayBitmapEffect) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampArrayBitmapEffect> for ::windows::runtime::IInspectable {
    fn from(value: &LampArrayBitmapEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<LampArrayBitmapEffect> for ILampArrayEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LampArrayBitmapEffect) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LampArrayBitmapEffect> for ILampArrayEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LampArrayBitmapEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILampArrayEffect> for LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILampArrayEffect> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILampArrayEffect> for &LampArrayBitmapEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILampArrayEffect> {
        ::std::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LampArrayBitmapEffect {}
unsafe impl ::std::marker::Sync for LampArrayBitmapEffect {}
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampArrayBitmapRequestedEventArgs(::windows::runtime::IInspectable);
impl LampArrayBitmapRequestedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SinceStarted(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Graphics_Imaging`*"]
    pub fn UpdateBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), bitmap.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayBitmapRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs;{c8b4af9e-fe63-4d51-babd-619defb454ba})");
}
unsafe impl ::windows::runtime::Interface for LampArrayBitmapRequestedEventArgs {
    type Vtable = ILampArrayBitmapRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3367284638, 65123, 19793, [186, 189, 97, 157, 239, 180, 84, 186]);
}
impl ::windows::runtime::RuntimeName for LampArrayBitmapRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs";
}
impl ::std::convert::From<LampArrayBitmapRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LampArrayBitmapRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampArrayBitmapRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LampArrayBitmapRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampArrayBitmapRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LampArrayBitmapRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampArrayBitmapRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LampArrayBitmapRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LampArrayBitmapRequestedEventArgs {}
unsafe impl ::std::marker::Sync for LampArrayBitmapRequestedEventArgs {}
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampArrayBlinkEffect(::windows::runtime::IInspectable);
impl LampArrayBlinkEffect {
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `UI`*"]
    pub fn Color(&self) -> ::windows::runtime::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `UI`*"]
    pub fn SetColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn AttackDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetAttackDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SustainDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetSustainDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn DecayDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetDecayDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn RepetitionDelay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetRepetitionDelay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn StartDelay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetStartDelay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn Occurrences(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetOccurrences(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn RepetitionMode(&self) -> ::windows::runtime::Result<LampArrayRepetitionMode> {
        let this = self;
        unsafe {
            let mut result__: LampArrayRepetitionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayRepetitionMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn ZIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<LampArrayBlinkEffect> {
        Self::ILampArrayBlinkEffectFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::std::mem::transmute(lampindexes.as_ptr()), &mut result__).from_abi::<LampArrayBlinkEffect>(result__)
        })
    }
    pub fn ILampArrayBlinkEffectFactory<R, F: FnOnce(&ILampArrayBlinkEffectFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LampArrayBlinkEffect, ILampArrayBlinkEffectFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayBlinkEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBlinkEffect;{ebbf35f6-2fc5-4bb3-b3c3-6221a7680d13})");
}
unsafe impl ::windows::runtime::Interface for LampArrayBlinkEffect {
    type Vtable = ILampArrayBlinkEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3955176950, 12229, 19379, [179, 195, 98, 33, 167, 104, 13, 19]);
}
impl ::windows::runtime::RuntimeName for LampArrayBlinkEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBlinkEffect";
}
impl ::std::convert::From<LampArrayBlinkEffect> for ::windows::runtime::IUnknown {
    fn from(value: LampArrayBlinkEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampArrayBlinkEffect> for ::windows::runtime::IUnknown {
    fn from(value: &LampArrayBlinkEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampArrayBlinkEffect> for ::windows::runtime::IInspectable {
    fn from(value: LampArrayBlinkEffect) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampArrayBlinkEffect> for ::windows::runtime::IInspectable {
    fn from(value: &LampArrayBlinkEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<LampArrayBlinkEffect> for ILampArrayEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LampArrayBlinkEffect) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LampArrayBlinkEffect> for ILampArrayEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LampArrayBlinkEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILampArrayEffect> for LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILampArrayEffect> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILampArrayEffect> for &LampArrayBlinkEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILampArrayEffect> {
        ::std::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LampArrayBlinkEffect {}
unsafe impl ::std::marker::Sync for LampArrayBlinkEffect {}
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampArrayColorRampEffect(::windows::runtime::IInspectable);
impl LampArrayColorRampEffect {
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `UI`*"]
    pub fn Color(&self) -> ::windows::runtime::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `UI`*"]
    pub fn SetColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn RampDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetRampDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn StartDelay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetStartDelay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn CompletionBehavior(&self) -> ::windows::runtime::Result<LampArrayEffectCompletionBehavior> {
        let this = self;
        unsafe {
            let mut result__: LampArrayEffectCompletionBehavior = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayEffectCompletionBehavior>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn ZIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<LampArrayColorRampEffect> {
        Self::ILampArrayColorRampEffectFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::std::mem::transmute(lampindexes.as_ptr()), &mut result__).from_abi::<LampArrayColorRampEffect>(result__)
        })
    }
    pub fn ILampArrayColorRampEffectFactory<R, F: FnOnce(&ILampArrayColorRampEffectFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LampArrayColorRampEffect, ILampArrayColorRampEffectFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayColorRampEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayColorRampEffect;{2b004437-40a7-432e-a0b9-0d570c2153ff})");
}
unsafe impl ::windows::runtime::Interface for LampArrayColorRampEffect {
    type Vtable = ILampArrayColorRampEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(721437751, 16551, 17198, [160, 185, 13, 87, 12, 33, 83, 255]);
}
impl ::windows::runtime::RuntimeName for LampArrayColorRampEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayColorRampEffect";
}
impl ::std::convert::From<LampArrayColorRampEffect> for ::windows::runtime::IUnknown {
    fn from(value: LampArrayColorRampEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampArrayColorRampEffect> for ::windows::runtime::IUnknown {
    fn from(value: &LampArrayColorRampEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampArrayColorRampEffect> for ::windows::runtime::IInspectable {
    fn from(value: LampArrayColorRampEffect) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampArrayColorRampEffect> for ::windows::runtime::IInspectable {
    fn from(value: &LampArrayColorRampEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<LampArrayColorRampEffect> for ILampArrayEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LampArrayColorRampEffect) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LampArrayColorRampEffect> for ILampArrayEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LampArrayColorRampEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILampArrayEffect> for LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILampArrayEffect> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILampArrayEffect> for &LampArrayColorRampEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILampArrayEffect> {
        ::std::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LampArrayColorRampEffect {}
unsafe impl ::std::marker::Sync for LampArrayColorRampEffect {}
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampArrayCustomEffect(::windows::runtime::IInspectable);
impl LampArrayCustomEffect {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn UpdateInterval(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetUpdateInterval<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn UpdateRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<LampArrayCustomEffect, LampArrayUpdateRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn RemoveUpdateRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn ZIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<LampArrayCustomEffect> {
        Self::ILampArrayCustomEffectFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::std::mem::transmute(lampindexes.as_ptr()), &mut result__).from_abi::<LampArrayCustomEffect>(result__)
        })
    }
    pub fn ILampArrayCustomEffectFactory<R, F: FnOnce(&ILampArrayCustomEffectFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LampArrayCustomEffect, ILampArrayCustomEffectFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayCustomEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayCustomEffect;{ec579170-3c34-4876-818b-5765f78b0ee4})");
}
unsafe impl ::windows::runtime::Interface for LampArrayCustomEffect {
    type Vtable = ILampArrayCustomEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3965161840, 15412, 18550, [129, 139, 87, 101, 247, 139, 14, 228]);
}
impl ::windows::runtime::RuntimeName for LampArrayCustomEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayCustomEffect";
}
impl ::std::convert::From<LampArrayCustomEffect> for ::windows::runtime::IUnknown {
    fn from(value: LampArrayCustomEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampArrayCustomEffect> for ::windows::runtime::IUnknown {
    fn from(value: &LampArrayCustomEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampArrayCustomEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampArrayCustomEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampArrayCustomEffect> for ::windows::runtime::IInspectable {
    fn from(value: LampArrayCustomEffect) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampArrayCustomEffect> for ::windows::runtime::IInspectable {
    fn from(value: &LampArrayCustomEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampArrayCustomEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampArrayCustomEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<LampArrayCustomEffect> for ILampArrayEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LampArrayCustomEffect) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LampArrayCustomEffect> for ILampArrayEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LampArrayCustomEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILampArrayEffect> for LampArrayCustomEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILampArrayEffect> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILampArrayEffect> for &LampArrayCustomEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILampArrayEffect> {
        ::std::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LampArrayCustomEffect {}
unsafe impl ::std::marker::Sync for LampArrayCustomEffect {}
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LampArrayEffectCompletionBehavior(pub i32);
impl LampArrayEffectCompletionBehavior {
    pub const ClearState: LampArrayEffectCompletionBehavior = LampArrayEffectCompletionBehavior(0i32);
    pub const KeepState: LampArrayEffectCompletionBehavior = LampArrayEffectCompletionBehavior(1i32);
}
impl ::std::convert::From<i32> for LampArrayEffectCompletionBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LampArrayEffectCompletionBehavior {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayEffectCompletionBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayEffectCompletionBehavior;i4)");
}
impl ::windows::runtime::DefaultType for LampArrayEffectCompletionBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampArrayEffectPlaylist(::windows::runtime::IInspectable);
impl LampArrayEffectPlaylist {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LampArrayEffectPlaylist, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<ILampArrayEffect>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<ILampArrayEffect>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<ILampArrayEffect> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<ILampArrayEffect>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, ILampArrayEffect>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ILampArrayEffect as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, ILampArrayEffect>>(&self, effect: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), effect.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn OverrideZIndex(&self, zindex: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), zindex).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn Pause(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn EffectStartMode(&self) -> ::windows::runtime::Result<LampArrayEffectStartMode> {
        let this = self;
        unsafe {
            let mut result__: LampArrayEffectStartMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayEffectStartMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetEffectStartMode(&self, value: LampArrayEffectStartMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn Occurrences(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetOccurrences(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn RepetitionMode(&self) -> ::windows::runtime::Result<LampArrayRepetitionMode> {
        let this = self;
        unsafe {
            let mut result__: LampArrayRepetitionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayRepetitionMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation_Collections`*"]
    pub fn StartAll<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation_Collections`*"]
    pub fn StopAll<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation_Collections`*"]
    pub fn PauseAll<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    pub fn ILampArrayEffectPlaylistStatics<R, F: FnOnce(&ILampArrayEffectPlaylistStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LampArrayEffectPlaylist, ILampArrayEffectPlaylistStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayEffectPlaylist {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayEffectPlaylist;{7de58bfe-6f61-4103-98c7-d6632f7b9169})");
}
unsafe impl ::windows::runtime::Interface for LampArrayEffectPlaylist {
    type Vtable = ILampArrayEffectPlaylist_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2112195582, 28513, 16643, [152, 199, 214, 99, 47, 123, 145, 105]);
}
impl ::windows::runtime::RuntimeName for LampArrayEffectPlaylist {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayEffectPlaylist";
}
impl ::std::convert::From<LampArrayEffectPlaylist> for ::windows::runtime::IUnknown {
    fn from(value: LampArrayEffectPlaylist) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampArrayEffectPlaylist> for ::windows::runtime::IUnknown {
    fn from(value: &LampArrayEffectPlaylist) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampArrayEffectPlaylist> for ::windows::runtime::IInspectable {
    fn from(value: LampArrayEffectPlaylist) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampArrayEffectPlaylist> for ::windows::runtime::IInspectable {
    fn from(value: &LampArrayEffectPlaylist) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<LampArrayEffectPlaylist> for super::super::super::Foundation::Collections::IIterable<ILampArrayEffect> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LampArrayEffectPlaylist) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&LampArrayEffectPlaylist> for super::super::super::Foundation::Collections::IIterable<ILampArrayEffect> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LampArrayEffectPlaylist) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>> for &LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<LampArrayEffectPlaylist> for super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LampArrayEffectPlaylist) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&LampArrayEffectPlaylist> for super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LampArrayEffectPlaylist) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>> for &LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LampArrayEffectPlaylist {}
unsafe impl ::std::marker::Sync for LampArrayEffectPlaylist {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for LampArrayEffectPlaylist {
    type Item = ILampArrayEffect;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &LampArrayEffectPlaylist {
    type Item = ILampArrayEffect;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LampArrayEffectStartMode(pub i32);
impl LampArrayEffectStartMode {
    pub const Sequential: LampArrayEffectStartMode = LampArrayEffectStartMode(0i32);
    pub const Simultaneous: LampArrayEffectStartMode = LampArrayEffectStartMode(1i32);
}
impl ::std::convert::From<i32> for LampArrayEffectStartMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LampArrayEffectStartMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayEffectStartMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayEffectStartMode;i4)");
}
impl ::windows::runtime::DefaultType for LampArrayEffectStartMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LampArrayRepetitionMode(pub i32);
impl LampArrayRepetitionMode {
    pub const Occurrences: LampArrayRepetitionMode = LampArrayRepetitionMode(0i32);
    pub const Forever: LampArrayRepetitionMode = LampArrayRepetitionMode(1i32);
}
impl ::std::convert::From<i32> for LampArrayRepetitionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LampArrayRepetitionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayRepetitionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayRepetitionMode;i4)");
}
impl ::windows::runtime::DefaultType for LampArrayRepetitionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampArraySolidEffect(::windows::runtime::IInspectable);
impl LampArraySolidEffect {
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `UI`*"]
    pub fn Color(&self) -> ::windows::runtime::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `UI`*"]
    pub fn SetColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn StartDelay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SetStartDelay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn CompletionBehavior(&self) -> ::windows::runtime::Result<LampArrayEffectCompletionBehavior> {
        let this = self;
        unsafe {
            let mut result__: LampArrayEffectCompletionBehavior = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LampArrayEffectCompletionBehavior>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn ZIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Lights_Effects`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<LampArraySolidEffect> {
        Self::ILampArraySolidEffectFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::std::mem::transmute(lampindexes.as_ptr()), &mut result__).from_abi::<LampArraySolidEffect>(result__)
        })
    }
    pub fn ILampArraySolidEffectFactory<R, F: FnOnce(&ILampArraySolidEffectFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LampArraySolidEffect, ILampArraySolidEffectFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampArraySolidEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArraySolidEffect;{441f8213-43cc-4b33-80eb-c6ddde7dc8ed})");
}
unsafe impl ::windows::runtime::Interface for LampArraySolidEffect {
    type Vtable = ILampArraySolidEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1142915603, 17356, 19251, [128, 235, 198, 221, 222, 125, 200, 237]);
}
impl ::windows::runtime::RuntimeName for LampArraySolidEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArraySolidEffect";
}
impl ::std::convert::From<LampArraySolidEffect> for ::windows::runtime::IUnknown {
    fn from(value: LampArraySolidEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampArraySolidEffect> for ::windows::runtime::IUnknown {
    fn from(value: &LampArraySolidEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampArraySolidEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampArraySolidEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampArraySolidEffect> for ::windows::runtime::IInspectable {
    fn from(value: LampArraySolidEffect) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampArraySolidEffect> for ::windows::runtime::IInspectable {
    fn from(value: &LampArraySolidEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampArraySolidEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampArraySolidEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<LampArraySolidEffect> for ILampArrayEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LampArraySolidEffect) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LampArraySolidEffect> for ILampArrayEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LampArraySolidEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILampArrayEffect> for LampArraySolidEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILampArrayEffect> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILampArrayEffect> for &LampArraySolidEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILampArrayEffect> {
        ::std::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LampArraySolidEffect {}
unsafe impl ::std::marker::Sync for LampArraySolidEffect {}
#[doc = "*Required features: `Devices_Lights_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LampArrayUpdateRequestedEventArgs(::windows::runtime::IInspectable);
impl LampArrayUpdateRequestedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `Foundation`*"]
    pub fn SinceStarted(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `UI`*"]
    pub fn SetColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::UI::Color>>(&self, desiredcolor: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), desiredcolor.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `UI`*"]
    pub fn SetColorForIndex<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::UI::Color>>(&self, lampindex: i32, desiredcolor: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), lampindex, desiredcolor.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `UI`*"]
    pub fn SetSingleColorForIndices<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::UI::Color>>(&self, desiredcolor: Param0, lampindexes: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), desiredcolor.into_param().abi(), lampindexes.len() as u32, ::std::mem::transmute(lampindexes.as_ptr())).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Lights_Effects`, `UI`*"]
    pub fn SetColorsForIndices(&self, desiredcolors: &[<super::super::super::UI::Color as ::windows::runtime::DefaultType>::DefaultType], lampindexes: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), desiredcolors.len() as u32, ::std::mem::transmute(desiredcolors.as_ptr()), lampindexes.len() as u32, ::std::mem::transmute(lampindexes.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LampArrayUpdateRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs;{73560d6a-576a-48af-8539-67ffa0ab3516})");
}
unsafe impl ::windows::runtime::Interface for LampArrayUpdateRequestedEventArgs {
    type Vtable = ILampArrayUpdateRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935019370, 22378, 18607, [133, 57, 103, 255, 160, 171, 53, 22]);
}
impl ::windows::runtime::RuntimeName for LampArrayUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs";
}
impl ::std::convert::From<LampArrayUpdateRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LampArrayUpdateRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LampArrayUpdateRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LampArrayUpdateRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LampArrayUpdateRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LampArrayUpdateRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LampArrayUpdateRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LampArrayUpdateRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LampArrayUpdateRequestedEventArgs {}
unsafe impl ::std::marker::Sync for LampArrayUpdateRequestedEventArgs {}
