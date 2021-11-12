#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IScreenReaderPositionChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScreenReaderPositionChangedEventArgs {
    type Vtable = IScreenReaderPositionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x557eb5e5_54d0_5ccd_9fc5_ed33357f8a9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenReaderPositionChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScreenReaderService(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScreenReaderService {
    type Vtable = IScreenReaderService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19475427_eac0_50d3_bdd9_9b487a226256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenReaderService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScreenReaderPositionChangedEventArgs(pub ::windows::core::IInspectable);
impl ScreenReaderPositionChangedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn ScreenPositionInRawPixels(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    pub fn IsReadingText(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ScreenReaderPositionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs;{557eb5e5-54d0-5ccd-9fc5-ed33357f8a9f})");
}
unsafe impl ::windows::core::Interface for ScreenReaderPositionChangedEventArgs {
    type Vtable = IScreenReaderPositionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x557eb5e5_54d0_5ccd_9fc5_ed33357f8a9f);
}
impl ::windows::core::RuntimeName for ScreenReaderPositionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs";
}
impl ::core::convert::From<ScreenReaderPositionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ScreenReaderPositionChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScreenReaderPositionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ScreenReaderPositionChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScreenReaderPositionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScreenReaderPositionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScreenReaderPositionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ScreenReaderPositionChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScreenReaderPositionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ScreenReaderPositionChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScreenReaderPositionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScreenReaderPositionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ScreenReaderPositionChangedEventArgs {}
unsafe impl ::core::marker::Sync for ScreenReaderPositionChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScreenReaderService(pub ::windows::core::IInspectable);
impl ScreenReaderService {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScreenReaderService, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CurrentScreenReaderPosition(&self) -> ::windows::core::Result<ScreenReaderPositionChangedEventArgs> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ScreenReaderPositionChangedEventArgs>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ScreenReaderPositionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveScreenReaderPositionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ScreenReaderService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Accessibility.ScreenReaderService;{19475427-eac0-50d3-bdd9-9b487a226256})");
}
unsafe impl ::windows::core::Interface for ScreenReaderService {
    type Vtable = IScreenReaderService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19475427_eac0_50d3_bdd9_9b487a226256);
}
impl ::windows::core::RuntimeName for ScreenReaderService {
    const NAME: &'static str = "Windows.UI.Accessibility.ScreenReaderService";
}
impl ::core::convert::From<ScreenReaderService> for ::windows::core::IUnknown {
    fn from(value: ScreenReaderService) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScreenReaderService> for ::windows::core::IUnknown {
    fn from(value: &ScreenReaderService) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScreenReaderService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScreenReaderService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScreenReaderService> for ::windows::core::IInspectable {
    fn from(value: ScreenReaderService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScreenReaderService> for ::windows::core::IInspectable {
    fn from(value: &ScreenReaderService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScreenReaderService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScreenReaderService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ScreenReaderService {}
unsafe impl ::core::marker::Sync for ScreenReaderService {}
