#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkWorkspaceHostedAppManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe0a7990_5e59_4bb7_8a63_7d218cd96300);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetThumbnailAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkWorkspaceHostedAppManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkWorkspaceHostedAppManagerStatics {
    type Vtable = IInkWorkspaceHostedAppManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbfd8cc5_a162_4bc4_84ee_e8716d5233c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Preview_InkWorkspace\"`*"]
#[repr(transparent)]
pub struct InkWorkspaceHostedAppManager(::windows::core::IUnknown);
impl InkWorkspaceHostedAppManager {
    #[doc = "*Required features: `\"ApplicationModel_Preview_InkWorkspace\"`, `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetThumbnailAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetThumbnailAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_InkWorkspace\"`*"]
    pub fn GetForCurrentApp() -> ::windows::core::Result<InkWorkspaceHostedAppManager> {
        Self::IInkWorkspaceHostedAppManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentApp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkWorkspaceHostedAppManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInkWorkspaceHostedAppManagerStatics<R, F: FnOnce(&IInkWorkspaceHostedAppManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkWorkspaceHostedAppManager, IInkWorkspaceHostedAppManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InkWorkspaceHostedAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkWorkspaceHostedAppManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkWorkspaceHostedAppManager {}
impl ::core::fmt::Debug for InkWorkspaceHostedAppManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkWorkspaceHostedAppManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkWorkspaceHostedAppManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager;{fe0a7990-5e59-4bb7-8a63-7d218cd96300})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_Vtbl;
    const IID: ::windows::core::GUID = <IInkWorkspaceHostedAppManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InkWorkspaceHostedAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager";
}
impl ::core::convert::From<InkWorkspaceHostedAppManager> for ::windows::core::IUnknown {
    fn from(value: InkWorkspaceHostedAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkWorkspaceHostedAppManager> for ::windows::core::IUnknown {
    fn from(value: &InkWorkspaceHostedAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkWorkspaceHostedAppManager> for ::windows::core::IInspectable {
    fn from(value: InkWorkspaceHostedAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkWorkspaceHostedAppManager> for ::windows::core::IInspectable {
    fn from(value: &InkWorkspaceHostedAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkWorkspaceHostedAppManager {}
unsafe impl ::core::marker::Sync for InkWorkspaceHostedAppManager {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
