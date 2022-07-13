#[doc(hidden)]
#[repr(transparent)]
pub struct IPreviewBuildsManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa07dd61_7e4f_59f7_7c9f_def9051c5f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ArePreviewBuildsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetArePreviewBuildsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPreviewBuildsManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPreviewBuildsManagerStatics {
    type Vtable = IPreviewBuildsManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e422887_b112_5a70_7da1_97d78d32aa29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPreviewBuildsState(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPreviewBuildsState {
    type Vtable = IPreviewBuildsState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2f2903e_b223_5f63_7546_3e8eac070a2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsState_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct PreviewBuildsManager(::windows::core::IUnknown);
impl PreviewBuildsManager {
    pub fn ArePreviewBuildsAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ArePreviewBuildsAllowed)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetArePreviewBuildsAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetArePreviewBuildsAllowed)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetCurrentState(&self) -> ::windows::core::Result<PreviewBuildsState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PreviewBuildsState>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SyncAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<PreviewBuildsManager> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PreviewBuildsManager>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPreviewBuildsManagerStatics<R, F: FnOnce(&IPreviewBuildsManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PreviewBuildsManager, IPreviewBuildsManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PreviewBuildsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PreviewBuildsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PreviewBuildsManager {}
impl ::core::fmt::Debug for PreviewBuildsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreviewBuildsManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PreviewBuildsManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.PreviewBuildsManager;{fa07dd61-7e4f-59f7-7c9f-def9051c5f62})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_Vtbl;
    const IID: ::windows::core::GUID = <IPreviewBuildsManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PreviewBuildsManager {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsManager";
}
impl ::core::convert::From<PreviewBuildsManager> for ::windows::core::IUnknown {
    fn from(value: PreviewBuildsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PreviewBuildsManager> for ::windows::core::IUnknown {
    fn from(value: &PreviewBuildsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PreviewBuildsManager> for &::windows::core::IUnknown {
    fn from(value: &PreviewBuildsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PreviewBuildsManager> for ::windows::core::IInspectable {
    fn from(value: PreviewBuildsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PreviewBuildsManager> for ::windows::core::IInspectable {
    fn from(value: &PreviewBuildsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PreviewBuildsManager> for &::windows::core::IInspectable {
    fn from(value: &PreviewBuildsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PreviewBuildsManager {}
unsafe impl ::core::marker::Sync for PreviewBuildsManager {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct PreviewBuildsState(::windows::core::IUnknown);
impl PreviewBuildsState {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for PreviewBuildsState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PreviewBuildsState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PreviewBuildsState {}
impl ::core::fmt::Debug for PreviewBuildsState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreviewBuildsState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PreviewBuildsState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.PreviewBuildsState;{a2f2903e-b223-5f63-7546-3e8eac070a2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PreviewBuildsState {
    type Vtable = IPreviewBuildsState_Vtbl;
    const IID: ::windows::core::GUID = <IPreviewBuildsState as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PreviewBuildsState {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsState";
}
impl ::core::convert::From<PreviewBuildsState> for ::windows::core::IUnknown {
    fn from(value: PreviewBuildsState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PreviewBuildsState> for ::windows::core::IUnknown {
    fn from(value: &PreviewBuildsState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PreviewBuildsState> for &::windows::core::IUnknown {
    fn from(value: &PreviewBuildsState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PreviewBuildsState> for ::windows::core::IInspectable {
    fn from(value: PreviewBuildsState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PreviewBuildsState> for ::windows::core::IInspectable {
    fn from(value: &PreviewBuildsState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PreviewBuildsState> for &::windows::core::IInspectable {
    fn from(value: &PreviewBuildsState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PreviewBuildsState {}
unsafe impl ::core::marker::Sync for PreviewBuildsState {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
