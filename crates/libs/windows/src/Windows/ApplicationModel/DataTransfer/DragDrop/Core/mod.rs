#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragDropManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreDragDropManager {
    type Vtable = ICoreDragDropManager_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreDragDropManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d56d344_8464_4faf_aa49_37ea6e2d7bd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragDropManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TargetRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetRequested: usize,
    pub AreConcurrentOperationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAreConcurrentOperationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragDropManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreDragDropManagerStatics {
    type Vtable = ICoreDragDropManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreDragDropManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9542fdca_da12_4c1c_8d06_041db29733c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragDropManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreDragInfo {
    type Vtable = ICoreDragInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreDragInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48353a8b_cb50_464e_9575_cd4e3a7ab028);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Modifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DragDropModifiers) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreDragInfo2 {
    type Vtable = ICoreDragInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreDragInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc54691e5_e6fb_4d74_b4b1_8a3c17f25e9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragInfo2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::DataPackageOperation) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreDragOperation {
    type Vtable = ICoreDragOperation_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreDragOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc06de4f_6db0_4e62_ab1b_a74a02dc6d85);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragOperation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetDragUIContentFromSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetDragUIContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetDragUIContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: *mut ::core::ffi::c_void, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetDragUIContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub DragUIContentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreDragUIContentMode) -> ::windows::core::HRESULT,
    pub SetDragUIContentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreDragUIContentMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragOperation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreDragOperation2 {
    type Vtable = ICoreDragOperation2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreDragOperation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x824b1e2c_d99a_4fc3_8507_6c182f33b46a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragOperation2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::DataPackageOperation) -> ::windows::core::HRESULT,
    pub SetAllowedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::DataPackageOperation) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragUIOverride(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreDragUIOverride {
    type Vtable = ICoreDragUIOverride_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreDragUIOverride {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89a85064_3389_4f4f_8897_7e8a3ffb3c93);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragUIOverride_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetContentFromSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: *mut ::core::ffi::c_void, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub IsContentVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsContentVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsCaptionVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCaptionVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsGlyphVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsGlyphVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct ICoreDropOperationTarget(::windows::core::IUnknown);
impl ICoreDropOperationTarget {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnterAsync(&self, draginfo: &CoreDragInfo, draguioverride: &CoreDragUIOverride) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnterAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(draginfo), ::core::mem::transmute_copy(draguioverride), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OverAsync(&self, draginfo: &CoreDragInfo, draguioverride: &CoreDragUIOverride) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OverAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(draginfo), ::core::mem::transmute_copy(draguioverride), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LeaveAsync(&self, draginfo: &CoreDragInfo) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LeaveAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(draginfo), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DropAsync(&self, draginfo: &CoreDragInfo) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DropAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(draginfo), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ICoreDropOperationTarget, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ICoreDropOperationTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for ICoreDropOperationTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d9126196-4c5b-417d-bb37-76381def8db4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICoreDropOperationTarget {
    type Vtable = ICoreDropOperationTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreDropOperationTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9126196_4c5b_417d_bb37_76381def8db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDropOperationTarget_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub EnterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, draguioverride: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, draguioverride: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OverAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LeaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DropAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DropAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDropOperationTargetRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreDropOperationTargetRequestedEventArgs {
    type Vtable = ICoreDropOperationTargetRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreDropOperationTargetRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2aca929a_5e28_4ea6_829e_29134e665d6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDropOperationTargetRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragDropManager(::windows::core::IUnknown);
impl CoreDragDropManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TargetRequested(&self, value: &super::super::super::super::Foundation::TypedEventHandler<CoreDragDropManager, CoreDropOperationTargetRequestedEventArgs>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTargetRequested(&self, value: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveTargetRequested)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AreConcurrentOperationsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AreConcurrentOperationsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAreConcurrentOperationsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAreConcurrentOperationsEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreDragDropManager> {
        Self::ICoreDragDropManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreDragDropManagerStatics<R, F: FnOnce(&ICoreDragDropManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreDragDropManager, ICoreDragDropManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CoreDragDropManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CoreDragDropManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager;{7d56d344-8464-4faf-aa49-37ea6e2d7bd1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreDragDropManager {
    type Vtable = ICoreDragDropManager_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreDragDropManager {
    const IID: ::windows::core::GUID = <ICoreDragDropManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreDragDropManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager";
}
::windows::core::interface_hierarchy!(CoreDragDropManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreDragDropManager {}
unsafe impl ::core::marker::Sync for CoreDragDropManager {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragInfo(::windows::core::IUnknown);
impl CoreDragInfo {
    pub fn Data(&self) -> ::windows::core::Result<super::super::DataPackageView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Modifiers(&self) -> ::windows::core::Result<super::DragDropModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Modifiers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AllowedOperations(&self) -> ::windows::core::Result<super::super::DataPackageOperation> {
        let this = &::windows::core::Interface::cast::<ICoreDragInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowedOperations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreDragInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CoreDragInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo;{48353a8b-cb50-464e-9575-cd4e3a7ab028})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreDragInfo {
    type Vtable = ICoreDragInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreDragInfo {
    const IID: ::windows::core::GUID = <ICoreDragInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreDragInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo";
}
::windows::core::interface_hierarchy!(CoreDragInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreDragInfo {}
unsafe impl ::core::marker::Sync for CoreDragInfo {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragOperation(::windows::core::IUnknown);
impl CoreDragOperation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreDragOperation, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Data(&self) -> ::windows::core::Result<super::super::DataPackage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPointerId(&self, pointerid: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPointerId)(::windows::core::Vtable::as_raw(this), pointerid).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetDragUIContentFromSoftwareBitmap(&self, softwarebitmap: &super::super::super::super::Graphics::Imaging::SoftwareBitmap) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDragUIContentFromSoftwareBitmap)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(softwarebitmap)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetDragUIContentFromSoftwareBitmapWithAnchorPoint(&self, softwarebitmap: &super::super::super::super::Graphics::Imaging::SoftwareBitmap, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDragUIContentFromSoftwareBitmapWithAnchorPoint)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(softwarebitmap), anchorpoint).ok() }
    }
    pub fn DragUIContentMode(&self) -> ::windows::core::Result<CoreDragUIContentMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DragUIContentMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDragUIContentMode(&self, value: CoreDragUIContentMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDragUIContentMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AllowedOperations(&self) -> ::windows::core::Result<super::super::DataPackageOperation> {
        let this = &::windows::core::Interface::cast::<ICoreDragOperation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowedOperations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowedOperations(&self, value: super::super::DataPackageOperation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreDragOperation2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowedOperations)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CoreDragOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CoreDragOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation;{cc06de4f-6db0-4e62-ab1b-a74a02dc6d85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreDragOperation {
    type Vtable = ICoreDragOperation_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreDragOperation {
    const IID: ::windows::core::GUID = <ICoreDragOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreDragOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation";
}
::windows::core::interface_hierarchy!(CoreDragOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreDragOperation {}
unsafe impl ::core::marker::Sync for CoreDragOperation {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragUIOverride(::windows::core::IUnknown);
impl CoreDragUIOverride {
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetContentFromSoftwareBitmap(&self, softwarebitmap: &super::super::super::super::Graphics::Imaging::SoftwareBitmap) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentFromSoftwareBitmap)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(softwarebitmap)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetContentFromSoftwareBitmapWithAnchorPoint(&self, softwarebitmap: &super::super::super::super::Graphics::Imaging::SoftwareBitmap, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentFromSoftwareBitmapWithAnchorPoint)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(softwarebitmap), anchorpoint).ok() }
    }
    pub fn IsContentVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsContentVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsContentVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsContentVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Caption)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCaption(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCaption)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsCaptionVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCaptionVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsCaptionVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsCaptionVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsGlyphVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsGlyphVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsGlyphVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsGlyphVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for CoreDragUIOverride {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CoreDragUIOverride {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride;{89a85064-3389-4f4f-8897-7e8a3ffb3c93})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreDragUIOverride {
    type Vtable = ICoreDragUIOverride_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreDragUIOverride {
    const IID: ::windows::core::GUID = <ICoreDragUIOverride as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreDragUIOverride {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride";
}
::windows::core::interface_hierarchy!(CoreDragUIOverride, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreDragUIOverride {}
unsafe impl ::core::marker::Sync for CoreDragUIOverride {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDropOperationTargetRequestedEventArgs(::windows::core::IUnknown);
impl CoreDropOperationTargetRequestedEventArgs {
    pub fn SetTarget<P0, E0>(&self, target: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICoreDropOperationTarget>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTarget)(::windows::core::Vtable::as_raw(this), target.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreDropOperationTargetRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CoreDropOperationTargetRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs;{2aca929a-5e28-4ea6-829e-29134e665d6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreDropOperationTargetRequestedEventArgs {
    type Vtable = ICoreDropOperationTargetRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreDropOperationTargetRequestedEventArgs {
    const IID: ::windows::core::GUID = <ICoreDropOperationTargetRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreDropOperationTargetRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs";
}
::windows::core::interface_hierarchy!(CoreDropOperationTargetRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Abi for CoreDragUIContentMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreDragUIContentMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragUIContentMode").field(&self.0).finish()
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
unsafe impl ::windows::core::RuntimeType for CoreDragUIContentMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIContentMode;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
