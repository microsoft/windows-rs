#[doc(hidden)]
#[repr(transparent)]
pub struct IScreenReaderPositionChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IScreenReaderPositionChangedEventArgs {
    type Vtable = IScreenReaderPositionChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x557eb5e5_54d0_5ccd_9fc5_ed33357f8a9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenReaderPositionChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ScreenPositionInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenPositionInRawPixels: usize,
    pub IsReadingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScreenReaderService(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IScreenReaderService {
    type Vtable = IScreenReaderService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19475427_eac0_50d3_bdd9_9b487a226256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenReaderService_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CurrentScreenReaderPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ScreenReaderPositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenReaderPositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScreenReaderPositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScreenReaderPositionChanged: usize,
}
#[doc = "*Required features: `\"UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ScreenReaderPositionChangedEventArgs(::windows::core::IUnknown);
impl ScreenReaderPositionChangedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenPositionInRawPixels(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ScreenPositionInRawPixels)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    pub fn IsReadingText(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsReadingText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ScreenReaderPositionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScreenReaderPositionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScreenReaderPositionChangedEventArgs {}
impl ::core::fmt::Debug for ScreenReaderPositionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScreenReaderPositionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ScreenReaderPositionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs;{557eb5e5-54d0-5ccd-9fc5-ed33357f8a9f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ScreenReaderPositionChangedEventArgs {
    type Vtable = IScreenReaderPositionChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IScreenReaderPositionChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ScreenReaderPositionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs";
}
impl ::core::convert::From<ScreenReaderPositionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ScreenReaderPositionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenReaderPositionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ScreenReaderPositionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ScreenReaderPositionChangedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ScreenReaderPositionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ScreenReaderPositionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ScreenReaderPositionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenReaderPositionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ScreenReaderPositionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ScreenReaderPositionChangedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ScreenReaderPositionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ScreenReaderPositionChangedEventArgs {}
unsafe impl ::core::marker::Sync for ScreenReaderPositionChangedEventArgs {}
#[doc = "*Required features: `\"UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ScreenReaderService(::windows::core::IUnknown);
impl ScreenReaderService {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ScreenReaderService, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CurrentScreenReaderPosition(&self) -> ::windows::core::Result<ScreenReaderPositionChangedEventArgs> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentScreenReaderPosition)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ScreenReaderPositionChangedEventArgs>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenReaderPositionChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ScreenReaderPositionChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScreenReaderPositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveScreenReaderPositionChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for ScreenReaderService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScreenReaderService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScreenReaderService {}
impl ::core::fmt::Debug for ScreenReaderService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScreenReaderService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ScreenReaderService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Accessibility.ScreenReaderService;{19475427-eac0-50d3-bdd9-9b487a226256})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ScreenReaderService {
    type Vtable = IScreenReaderService_Vtbl;
    const IID: ::windows::core::GUID = <IScreenReaderService as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ScreenReaderService {
    const NAME: &'static str = "Windows.UI.Accessibility.ScreenReaderService";
}
impl ::core::convert::From<ScreenReaderService> for ::windows::core::IUnknown {
    fn from(value: ScreenReaderService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenReaderService> for ::windows::core::IUnknown {
    fn from(value: &ScreenReaderService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ScreenReaderService> for &::windows::core::IUnknown {
    fn from(value: &ScreenReaderService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ScreenReaderService> for ::windows::core::IInspectable {
    fn from(value: ScreenReaderService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenReaderService> for ::windows::core::IInspectable {
    fn from(value: &ScreenReaderService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ScreenReaderService> for &::windows::core::IInspectable {
    fn from(value: &ScreenReaderService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ScreenReaderService {}
unsafe impl ::core::marker::Sync for ScreenReaderService {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
