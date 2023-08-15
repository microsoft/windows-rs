#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
}
impl ::core::clone::Clone for IResourceLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IResourceLoader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08524908_16ef_45ad_a602_293637d7e61a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoader2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoader2 {
    type Vtable = IResourceLoader2_Vtbl;
}
impl ::core::clone::Clone for IResourceLoader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IResourceLoader2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10eb6ec6_8138_48c1_bc65_e1f14207367c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetStringForUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStringForUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderFactory {
    type Vtable = IResourceLoaderFactory_Vtbl;
}
impl ::core::clone::Clone for IResourceLoaderFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IResourceLoaderFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc33a3603_69dc_4285_a077_d5c0e47ccbe8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateResourceLoaderByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderStatics {
    type Vtable = IResourceLoaderStatics_Vtbl;
}
impl ::core::clone::Clone for IResourceLoaderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IResourceLoaderStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf777ce1_19c8_49c2_953c_47e9227b334e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetStringForReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStringForReference: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderStatics2 {
    type Vtable = IResourceLoaderStatics2_Vtbl;
}
impl ::core::clone::Clone for IResourceLoaderStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IResourceLoaderStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cc04141_6466_4989_9494_0b82dfc53f1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetForCurrentViewWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetForViewIndependentUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetForViewIndependentUseWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderStatics3 {
    type Vtable = IResourceLoaderStatics3_Vtbl;
}
impl ::core::clone::Clone for IResourceLoaderStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IResourceLoaderStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64609dfb_64ac_491b_8100_0e558d61c1d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub GetForUIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetForUIContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderStatics4 {
    type Vtable = IResourceLoaderStatics4_Vtbl;
}
impl ::core::clone::Clone for IResourceLoaderStatics4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IResourceLoaderStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fb36c32_6c8c_4316_962e_909539b5c259);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefaultPriPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceLoader(::windows_core::IUnknown);
impl ResourceLoader {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceLoader, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetString(&self, resource: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(resource), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStringForUri<P0>(&self, uri: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IResourceLoader2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStringForUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateResourceLoaderByName(name: &::windows_core::HSTRING) -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResourceLoaderByName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStringForReference<P0>(uri: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::IResourceLoaderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStringForReference)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForCurrentViewWithName(name: &::windows_core::HSTRING) -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentViewWithName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForViewIndependentUse() -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForViewIndependentUse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForViewIndependentUseWithName(name: &::windows_core::HSTRING) -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForViewIndependentUseWithName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn GetForUIContext<P0>(context: P0) -> ::windows_core::Result<ResourceLoader>
    where
        P0: ::windows_core::IntoParam<super::super::UI::UIContext>,
    {
        Self::IResourceLoaderStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUIContext)(::windows_core::Interface::as_raw(this), context.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefaultPriPath(packagefullname: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IResourceLoaderStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultPriPath)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IResourceLoaderFactory<R, F: FnOnce(&IResourceLoaderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceLoaderStatics<R, F: FnOnce(&IResourceLoaderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceLoaderStatics2<R, F: FnOnce(&IResourceLoaderStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceLoaderStatics3<R, F: FnOnce(&IResourceLoaderStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceLoaderStatics4<R, F: FnOnce(&IResourceLoaderStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ResourceLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceLoader {}
impl ::core::fmt::Debug for ResourceLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceLoader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ResourceLoader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.ResourceLoader;{08524908-16ef-45ad-a602-293637d7e61a})");
}
impl ::core::clone::Clone for ResourceLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceLoader {
    const IID: ::windows_core::GUID = <IResourceLoader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.ResourceLoader";
}
::windows_core::imp::interface_hierarchy!(ResourceLoader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ResourceLoader {}
unsafe impl ::core::marker::Sync for ResourceLoader {}
