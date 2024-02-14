::windows_core::imp::com_interface!(IInkWorkspaceHostedAppManager, IInkWorkspaceHostedAppManager_Vtbl, 0xfe0a7990_5e59_4bb7_8a63_7d218cd96300);
#[repr(C)]
pub struct IInkWorkspaceHostedAppManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetThumbnailAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetThumbnailAsync: usize,
}
::windows_core::imp::com_interface!(IInkWorkspaceHostedAppManagerStatics, IInkWorkspaceHostedAppManagerStatics_Vtbl, 0xcbfd8cc5_a162_4bc4_84ee_e8716d5233c5);
#[repr(C)]
pub struct IInkWorkspaceHostedAppManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentApp: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InkWorkspaceHostedAppManager(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(InkWorkspaceHostedAppManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl InkWorkspaceHostedAppManager {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetThumbnailAsync<P0>(&self, bitmap: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetThumbnailAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetForCurrentApp() -> ::windows_core::Result<InkWorkspaceHostedAppManager> {
        Self::IInkWorkspaceHostedAppManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentApp)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IInkWorkspaceHostedAppManagerStatics<R, F: FnOnce(&IInkWorkspaceHostedAppManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InkWorkspaceHostedAppManager, IInkWorkspaceHostedAppManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InkWorkspaceHostedAppManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_Vtbl;
    const IID: ::windows_core::GUID = <IInkWorkspaceHostedAppManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkWorkspaceHostedAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager";
}
unsafe impl ::core::marker::Send for InkWorkspaceHostedAppManager {}
unsafe impl ::core::marker::Sync for InkWorkspaceHostedAppManager {}
