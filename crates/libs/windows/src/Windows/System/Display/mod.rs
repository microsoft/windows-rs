#[doc = "*Required features: `\"System_Display\"`*"]
#[repr(transparent)]
pub struct DisplayRequest(::windows::core::IUnknown);
impl DisplayRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DisplayRequest, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RequestActive(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestActive)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn RequestRelease(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestRelease)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for DisplayRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayRequest {}
impl ::core::fmt::Debug for DisplayRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Display.DisplayRequest;{e5732044-f49f-4b60-8dd4-5e7e3a632ac0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayRequest {
    type Vtable = IDisplayRequest_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayRequest {
    const NAME: &'static str = "Windows.System.Display.DisplayRequest";
}
impl ::core::convert::From<DisplayRequest> for ::windows::core::IUnknown {
    fn from(value: DisplayRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayRequest> for ::windows::core::IUnknown {
    fn from(value: &DisplayRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DisplayRequest> for &::windows::core::IUnknown {
    fn from(value: &DisplayRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DisplayRequest> for ::windows::core::IInspectable {
    fn from(value: DisplayRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayRequest> for ::windows::core::IInspectable {
    fn from(value: &DisplayRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DisplayRequest> for &::windows::core::IInspectable {
    fn from(value: &DisplayRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayRequest {
    type Vtable = IDisplayRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5732044_f49f_4b60_8dd4_5e7e3a632ac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayRequest_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RequestActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
