#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPreviewBuildsManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4194819425, 32335, 23031, [124, 159, 222, 249, 5, 28, 95, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPreviewBuildsManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPreviewBuildsManagerStatics {
    type Vtable = IPreviewBuildsManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1044523143, 45330, 23152, [125, 161, 151, 215, 141, 50, 170, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPreviewBuildsState(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPreviewBuildsState {
    type Vtable = IPreviewBuildsState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2733805630, 45603, 24419, [117, 70, 62, 142, 172, 7, 10, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsState_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `Management_Update`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PreviewBuildsManager(::windows::runtime::IInspectable);
impl PreviewBuildsManager {
    #[doc = "*Required features: `Management_Update`*"]
    pub fn ArePreviewBuildsAllowed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Update`*"]
    pub fn SetArePreviewBuildsAllowed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Update`*"]
    pub fn GetCurrentState(&self) -> ::windows::runtime::Result<PreviewBuildsState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PreviewBuildsState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Update`, `Foundation`*"]
    pub fn SyncAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Update`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<PreviewBuildsManager> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PreviewBuildsManager>(result__)
        })
    }
    #[doc = "*Required features: `Management_Update`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IPreviewBuildsManagerStatics<R, F: FnOnce(&IPreviewBuildsManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PreviewBuildsManager, IPreviewBuildsManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PreviewBuildsManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Update.PreviewBuildsManager;{fa07dd61-7e4f-59f7-7c9f-def9051c5f62})");
}
unsafe impl ::windows::runtime::Interface for PreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4194819425, 32335, 23031, [124, 159, 222, 249, 5, 28, 95, 98]);
}
impl ::windows::runtime::RuntimeName for PreviewBuildsManager {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsManager";
}
unsafe impl ::std::marker::Send for PreviewBuildsManager {}
unsafe impl ::std::marker::Sync for PreviewBuildsManager {}
#[doc = "*Required features: `Management_Update`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PreviewBuildsState(::windows::runtime::IInspectable);
impl PreviewBuildsState {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Update`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PreviewBuildsState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Update.PreviewBuildsState;{a2f2903e-b223-5f63-7546-3e8eac070a2e})");
}
unsafe impl ::windows::runtime::Interface for PreviewBuildsState {
    type Vtable = IPreviewBuildsState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2733805630, 45603, 24419, [117, 70, 62, 142, 172, 7, 10, 46]);
}
impl ::windows::runtime::RuntimeName for PreviewBuildsState {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsState";
}
unsafe impl ::std::marker::Send for PreviewBuildsState {}
unsafe impl ::std::marker::Sync for PreviewBuildsState {}
