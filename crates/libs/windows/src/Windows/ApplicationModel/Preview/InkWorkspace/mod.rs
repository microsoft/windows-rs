#[doc(hidden)]
#[repr(transparent)]
pub struct IInkWorkspaceHostedAppManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_Vtbl;
}
impl ::core::clone::Clone for IInkWorkspaceHostedAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkWorkspaceHostedAppManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe0a7990_5e59_4bb7_8a63_7d218cd96300);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetThumbnailAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkWorkspaceHostedAppManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkWorkspaceHostedAppManagerStatics {
    type Vtable = IInkWorkspaceHostedAppManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IInkWorkspaceHostedAppManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkWorkspaceHostedAppManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbfd8cc5_a162_4bc4_84ee_e8716d5233c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Preview_InkWorkspace\"`*"]
#[repr(transparent)]
pub struct InkWorkspaceHostedAppManager(::windows::core::IUnknown);
impl InkWorkspaceHostedAppManager {
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetThumbnailAsync(&self, bitmap: &super::super::super::Graphics::Imaging::SoftwareBitmap) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SetThumbnailAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(bitmap), &mut result__).from_abi(result__)
        }
    }
    pub fn GetForCurrentApp() -> ::windows::core::Result<InkWorkspaceHostedAppManager> {
        Self::IInkWorkspaceHostedAppManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<InkWorkspaceHostedAppManager>();
            (::windows::core::Interface::vtable(this).GetForCurrentApp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInkWorkspaceHostedAppManagerStatics<R, F: FnOnce(&IInkWorkspaceHostedAppManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<InkWorkspaceHostedAppManager, IInkWorkspaceHostedAppManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for InkWorkspaceHostedAppManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager;{fe0a7990-5e59-4bb7-8a63-7d218cd96300})");
}
impl ::core::clone::Clone for InkWorkspaceHostedAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkWorkspaceHostedAppManager {
    const IID: ::windows::core::GUID = <IInkWorkspaceHostedAppManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkWorkspaceHostedAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager";
}
::windows::imp::interface_hierarchy!(InkWorkspaceHostedAppManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkWorkspaceHostedAppManager {}
unsafe impl ::core::marker::Sync for InkWorkspaceHostedAppManager {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
