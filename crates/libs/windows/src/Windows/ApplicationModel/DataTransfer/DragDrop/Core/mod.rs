#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragDropManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragDropManager {
    type Vtable = ICoreDragDropManager_Vtbl;
}
impl ::core::clone::Clone for ICoreDragDropManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreDragDropManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d56d344_8464_4faf_aa49_37ea6e2d7bd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragDropManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TargetRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetRequested: usize,
    pub AreConcurrentOperationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAreConcurrentOperationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragDropManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragDropManagerStatics {
    type Vtable = ICoreDragDropManagerStatics_Vtbl;
}
impl ::core::clone::Clone for ICoreDragDropManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreDragDropManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9542fdca_da12_4c1c_8d06_041db29733c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragDropManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragInfo {
    type Vtable = ICoreDragInfo_Vtbl;
}
impl ::core::clone::Clone for ICoreDragInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreDragInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48353a8b_cb50_464e_9575_cd4e3a7ab028);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Modifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DragDropModifiers) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragInfo2 {
    type Vtable = ICoreDragInfo2_Vtbl;
}
impl ::core::clone::Clone for ICoreDragInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreDragInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc54691e5_e6fb_4d74_b4b1_8a3c17f25e9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragInfo2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::DataPackageOperation) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragOperation {
    type Vtable = ICoreDragOperation_Vtbl;
}
impl ::core::clone::Clone for ICoreDragOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreDragOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc06de4f_6db0_4e62_ab1b_a74a02dc6d85);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetDragUIContentFromSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetDragUIContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetDragUIContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: *mut ::core::ffi::c_void, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetDragUIContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub DragUIContentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreDragUIContentMode) -> ::windows_core::HRESULT,
    pub SetDragUIContentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreDragUIContentMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragOperation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragOperation2 {
    type Vtable = ICoreDragOperation2_Vtbl;
}
impl ::core::clone::Clone for ICoreDragOperation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreDragOperation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x824b1e2c_d99a_4fc3_8507_6c182f33b46a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragOperation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::DataPackageOperation) -> ::windows_core::HRESULT,
    pub SetAllowedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::DataPackageOperation) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragUIOverride(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragUIOverride {
    type Vtable = ICoreDragUIOverride_Vtbl;
}
impl ::core::clone::Clone for ICoreDragUIOverride {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreDragUIOverride {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89a85064_3389_4f4f_8897_7e8a3ffb3c93);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragUIOverride_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetContentFromSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: *mut ::core::ffi::c_void, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub IsContentVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsContentVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsCaptionVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCaptionVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsGlyphVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsGlyphVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct ICoreDropOperationTarget(::windows_core::IUnknown);
impl ICoreDropOperationTarget {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnterAsync<P0, P1>(&self, draginfo: P0, draguioverride: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>
    where
        P0: ::windows_core::IntoParam<CoreDragInfo>,
        P1: ::windows_core::IntoParam<CoreDragUIOverride>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnterAsync)(::windows_core::Interface::as_raw(this), draginfo.into_param().abi(), draguioverride.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OverAsync<P0, P1>(&self, draginfo: P0, draguioverride: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>
    where
        P0: ::windows_core::IntoParam<CoreDragInfo>,
        P1: ::windows_core::IntoParam<CoreDragUIOverride>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OverAsync)(::windows_core::Interface::as_raw(this), draginfo.into_param().abi(), draguioverride.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LeaveAsync<P0>(&self, draginfo: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<CoreDragInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LeaveAsync)(::windows_core::Interface::as_raw(this), draginfo.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DropAsync<P0>(&self, draginfo: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>
    where
        P0: ::windows_core::IntoParam<CoreDragInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DropAsync)(::windows_core::Interface::as_raw(this), draginfo.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ICoreDropOperationTarget, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for ICoreDropOperationTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreDropOperationTarget {}
impl ::core::fmt::Debug for ICoreDropOperationTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreDropOperationTarget").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ICoreDropOperationTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{d9126196-4c5b-417d-bb37-76381def8db4}");
}
unsafe impl ::windows_core::Interface for ICoreDropOperationTarget {
    type Vtable = ICoreDropOperationTarget_Vtbl;
}
impl ::core::clone::Clone for ICoreDropOperationTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreDropOperationTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9126196_4c5b_417d_bb37_76381def8db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDropOperationTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub EnterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, draguioverride: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, draguioverride: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OverAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LeaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DropAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DropAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDropOperationTargetRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDropOperationTargetRequestedEventArgs {
    type Vtable = ICoreDropOperationTargetRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICoreDropOperationTargetRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreDropOperationTargetRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2aca929a_5e28_4ea6_829e_29134e665d6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDropOperationTargetRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragDropManager(::windows_core::IUnknown);
impl CoreDragDropManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TargetRequested<P0>(&self, value: P0) -> ::windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::TypedEventHandler<CoreDragDropManager, CoreDropOperationTargetRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetRequested)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTargetRequested(&self, value: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTargetRequested)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AreConcurrentOperationsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AreConcurrentOperationsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAreConcurrentOperationsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAreConcurrentOperationsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<CoreDragDropManager> {
        Self::ICoreDragDropManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreDragDropManagerStatics<R, F: FnOnce(&ICoreDragDropManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CoreDragDropManager, ICoreDragDropManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CoreDragDropManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDragDropManager {}
impl ::core::fmt::Debug for CoreDragDropManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragDropManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreDragDropManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager;{7d56d344-8464-4faf-aa49-37ea6e2d7bd1})");
}
impl ::core::clone::Clone for CoreDragDropManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CoreDragDropManager {
    type Vtable = ICoreDragDropManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreDragDropManager {
    const IID: ::windows_core::GUID = <ICoreDragDropManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreDragDropManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager";
}
::windows_core::imp::interface_hierarchy!(CoreDragDropManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CoreDragDropManager {}
unsafe impl ::core::marker::Sync for CoreDragDropManager {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragInfo(::windows_core::IUnknown);
impl CoreDragInfo {
    pub fn Data(&self) -> ::windows_core::Result<super::super::DataPackageView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Modifiers(&self) -> ::windows_core::Result<super::DragDropModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Modifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AllowedOperations(&self) -> ::windows_core::Result<super::super::DataPackageOperation> {
        let this = &::windows_core::ComInterface::cast::<ICoreDragInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowedOperations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CoreDragInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDragInfo {}
impl ::core::fmt::Debug for CoreDragInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreDragInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo;{48353a8b-cb50-464e-9575-cd4e3a7ab028})");
}
impl ::core::clone::Clone for CoreDragInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CoreDragInfo {
    type Vtable = ICoreDragInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreDragInfo {
    const IID: ::windows_core::GUID = <ICoreDragInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreDragInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo";
}
::windows_core::imp::interface_hierarchy!(CoreDragInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CoreDragInfo {}
unsafe impl ::core::marker::Sync for CoreDragInfo {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragOperation(::windows_core::IUnknown);
impl CoreDragOperation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CoreDragOperation, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Data(&self) -> ::windows_core::Result<super::super::DataPackage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPointerId(&self, pointerid: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerId)(::windows_core::Interface::as_raw(this), pointerid).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetDragUIContentFromSoftwareBitmap<P0>(&self, softwarebitmap: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragUIContentFromSoftwareBitmap)(::windows_core::Interface::as_raw(this), softwarebitmap.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetDragUIContentFromSoftwareBitmapWithAnchorPoint<P0>(&self, softwarebitmap: P0, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragUIContentFromSoftwareBitmapWithAnchorPoint)(::windows_core::Interface::as_raw(this), softwarebitmap.into_param().abi(), anchorpoint).ok() }
    }
    pub fn DragUIContentMode(&self) -> ::windows_core::Result<CoreDragUIContentMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DragUIContentMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDragUIContentMode(&self, value: CoreDragUIContentMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragUIContentMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AllowedOperations(&self) -> ::windows_core::Result<super::super::DataPackageOperation> {
        let this = &::windows_core::ComInterface::cast::<ICoreDragOperation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowedOperations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowedOperations(&self, value: super::super::DataPackageOperation) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreDragOperation2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowedOperations)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for CoreDragOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDragOperation {}
impl ::core::fmt::Debug for CoreDragOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreDragOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation;{cc06de4f-6db0-4e62-ab1b-a74a02dc6d85})");
}
impl ::core::clone::Clone for CoreDragOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CoreDragOperation {
    type Vtable = ICoreDragOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreDragOperation {
    const IID: ::windows_core::GUID = <ICoreDragOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreDragOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation";
}
::windows_core::imp::interface_hierarchy!(CoreDragOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CoreDragOperation {}
unsafe impl ::core::marker::Sync for CoreDragOperation {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragUIOverride(::windows_core::IUnknown);
impl CoreDragUIOverride {
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetContentFromSoftwareBitmap<P0>(&self, softwarebitmap: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentFromSoftwareBitmap)(::windows_core::Interface::as_raw(this), softwarebitmap.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetContentFromSoftwareBitmapWithAnchorPoint<P0>(&self, softwarebitmap: P0, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentFromSoftwareBitmapWithAnchorPoint)(::windows_core::Interface::as_raw(this), softwarebitmap.into_param().abi(), anchorpoint).ok() }
    }
    pub fn IsContentVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsContentVisible)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsContentVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsContentVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Caption(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Caption)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCaption(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCaption)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsCaptionVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCaptionVisible)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsCaptionVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCaptionVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGlyphVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGlyphVisible)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsGlyphVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGlyphVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for CoreDragUIOverride {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDragUIOverride {}
impl ::core::fmt::Debug for CoreDragUIOverride {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragUIOverride").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreDragUIOverride {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride;{89a85064-3389-4f4f-8897-7e8a3ffb3c93})");
}
impl ::core::clone::Clone for CoreDragUIOverride {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CoreDragUIOverride {
    type Vtable = ICoreDragUIOverride_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreDragUIOverride {
    const IID: ::windows_core::GUID = <ICoreDragUIOverride as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreDragUIOverride {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride";
}
::windows_core::imp::interface_hierarchy!(CoreDragUIOverride, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CoreDragUIOverride {}
unsafe impl ::core::marker::Sync for CoreDragUIOverride {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDropOperationTargetRequestedEventArgs(::windows_core::IUnknown);
impl CoreDropOperationTargetRequestedEventArgs {
    pub fn SetTarget<P0>(&self, target: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ICoreDropOperationTarget>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTarget)(::windows_core::Interface::as_raw(this), target.try_into_param()?.abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for CoreDropOperationTargetRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDropOperationTargetRequestedEventArgs {}
impl ::core::fmt::Debug for CoreDropOperationTargetRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDropOperationTargetRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreDropOperationTargetRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs;{2aca929a-5e28-4ea6-829e-29134e665d6d})");
}
impl ::core::clone::Clone for CoreDropOperationTargetRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CoreDropOperationTargetRequestedEventArgs {
    type Vtable = ICoreDropOperationTargetRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreDropOperationTargetRequestedEventArgs {
    const IID: ::windows_core::GUID = <ICoreDropOperationTargetRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreDropOperationTargetRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CoreDropOperationTargetRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CoreDropOperationTargetRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreDropOperationTargetRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreDragUIContentMode(pub u32);
impl CoreDragUIContentMode {
    pub const Auto: Self = Self(0u32);
    pub const Deferred: Self = Self(1u32);
}
impl ::core::marker::Copy for CoreDragUIContentMode {}
impl ::core::clone::Clone for CoreDragUIContentMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreDragUIContentMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreDragUIContentMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreDragUIContentMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragUIContentMode").field(&self.0).finish()
    }
}
impl CoreDragUIContentMode {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CoreDragUIContentMode {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreDragUIContentMode {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreDragUIContentMode {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreDragUIContentMode {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreDragUIContentMode {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for CoreDragUIContentMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIContentMode;u4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
