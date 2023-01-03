#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationAutoScrollBehavior(::windows::core::IUnknown);
impl IDirectManipulationAutoScrollBehavior {
    pub unsafe fn SetConfiguration(&self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetConfiguration)(::windows::core::Vtable::as_raw(self), motiontypes, scrollmotion).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationAutoScrollBehavior, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationAutoScrollBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationAutoScrollBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationAutoScrollBehavior {}
impl ::core::fmt::Debug for IDirectManipulationAutoScrollBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationAutoScrollBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationAutoScrollBehavior {
    type Vtable = IDirectManipulationAutoScrollBehavior_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationAutoScrollBehavior {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5954d4_2003_4356_9b31_d051c9ff0af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationAutoScrollBehavior_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationCompositor(::windows::core::IUnknown);
impl IDirectManipulationCompositor {
    pub unsafe fn AddContent<P0, P1, P2, P3>(&self, content: P0, device: P1, parentvisual: P2, childvisual: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationContent>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).AddContent)(::windows::core::Vtable::as_raw(self), content.into().abi(), device.into().abi(), parentvisual.into().abi(), childvisual.into().abi()).ok()
    }
    pub unsafe fn RemoveContent<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationContent>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveContent)(::windows::core::Vtable::as_raw(self), content.into().abi()).ok()
    }
    pub unsafe fn SetUpdateManager<P0>(&self, updatemanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationUpdateManager>>,
    {
        (::windows::core::Vtable::vtable(self).SetUpdateManager)(::windows::core::Vtable::as_raw(self), updatemanager.into().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Flush)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationCompositor, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationCompositor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationCompositor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationCompositor {}
impl ::core::fmt::Debug for IDirectManipulationCompositor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationCompositor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationCompositor {
    type Vtable = IDirectManipulationCompositor_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationCompositor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x537a0825_0387_4efa_b62f_71eb1f085a7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetUpdateManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatemanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationCompositor2(::windows::core::IUnknown);
impl IDirectManipulationCompositor2 {
    pub unsafe fn AddContent<P0, P1, P2, P3>(&self, content: P0, device: P1, parentvisual: P2, childvisual: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationContent>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddContent)(::windows::core::Vtable::as_raw(self), content.into().abi(), device.into().abi(), parentvisual.into().abi(), childvisual.into().abi()).ok()
    }
    pub unsafe fn RemoveContent<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationContent>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveContent)(::windows::core::Vtable::as_raw(self), content.into().abi()).ok()
    }
    pub unsafe fn SetUpdateManager<P0>(&self, updatemanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationUpdateManager>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUpdateManager)(::windows::core::Vtable::as_raw(self), updatemanager.into().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Flush)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AddContentWithCrossProcessChaining<P0, P1, P2, P3>(&self, content: P0, device: P1, parentvisual: P2, childvisual: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationPrimaryContent>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).AddContentWithCrossProcessChaining)(::windows::core::Vtable::as_raw(self), content.into().abi(), device.into().abi(), parentvisual.into().abi(), childvisual.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationCompositor2, ::windows::core::IUnknown, IDirectManipulationCompositor);
impl ::core::clone::Clone for IDirectManipulationCompositor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationCompositor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationCompositor2 {}
impl ::core::fmt::Debug for IDirectManipulationCompositor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationCompositor2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationCompositor2 {
    type Vtable = IDirectManipulationCompositor2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationCompositor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd38c7822_f1cb_43cb_b4b9_ac0c767a412e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor2_Vtbl {
    pub base__: IDirectManipulationCompositor_Vtbl,
    pub AddContentWithCrossProcessChaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationContent(::windows::core::IUnknown);
impl IDirectManipulationContent {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContentRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContentRect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentRect(&self, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetContentRect)(::windows::core::Vtable::as_raw(self), contentsize).ok()
    }
    pub unsafe fn GetViewport<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetViewport)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTag<T>(&self, id: ::core::option::Option<*mut u32>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).GetTag)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _, ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id).ok()
    }
    pub unsafe fn GetOutputTransform(&self, matrix: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetOutputTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(matrix.as_ptr()), matrix.len() as _).ok()
    }
    pub unsafe fn GetContentTransform(&self, matrix: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetContentTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(matrix.as_ptr()), matrix.len() as _).ok()
    }
    pub unsafe fn SyncContentTransform(&self, matrix: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SyncContentTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(matrix.as_ptr()), matrix.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationContent, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationContent {}
impl ::core::fmt::Debug for IDirectManipulationContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationContent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationContent {
    type Vtable = IDirectManipulationContent_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationContent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb89962cb_3d89_442b_bb58_5098fa0f9f16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationContent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContentRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentsize: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContentRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContentRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContentRect: usize,
    pub GetViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub GetOutputTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub GetContentTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub SyncContentTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationDeferContactService(::windows::core::IUnknown);
impl IDirectManipulationDeferContactService {
    pub unsafe fn DeferContact(&self, pointerid: u32, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeferContact)(::windows::core::Vtable::as_raw(self), pointerid, timeout).ok()
    }
    pub unsafe fn CancelContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelContact)(::windows::core::Vtable::as_raw(self), pointerid).ok()
    }
    pub unsafe fn CancelDeferral(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelDeferral)(::windows::core::Vtable::as_raw(self), pointerid).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationDeferContactService, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationDeferContactService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationDeferContactService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationDeferContactService {}
impl ::core::fmt::Debug for IDirectManipulationDeferContactService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationDeferContactService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationDeferContactService {
    type Vtable = IDirectManipulationDeferContactService_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationDeferContactService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x652d5c71_fe60_4a98_be70_e5f21291e7f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDeferContactService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DeferContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, timeout: u32) -> ::windows::core::HRESULT,
    pub CancelContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub CancelDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationDragDropBehavior(::windows::core::IUnknown);
impl IDirectManipulationDragDropBehavior {
    pub unsafe fn SetConfiguration(&self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetConfiguration)(::windows::core::Vtable::as_raw(self), configuration).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationDragDropBehavior, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationDragDropBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationDragDropBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationDragDropBehavior {}
impl ::core::fmt::Debug for IDirectManipulationDragDropBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationDragDropBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationDragDropBehavior {
    type Vtable = IDirectManipulationDragDropBehavior_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationDragDropBehavior {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x814b5af5_c2c8_4270_a9b7_a198ce8d02fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropBehavior_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationDragDropEventHandler(::windows::core::IUnknown);
impl IDirectManipulationDragDropEventHandler {
    pub unsafe fn OnDragDropStatusChange<P0>(&self, viewport: P0, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationViewport2>>,
    {
        (::windows::core::Vtable::vtable(self).OnDragDropStatusChange)(::windows::core::Vtable::as_raw(self), viewport.into().abi(), current, previous).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationDragDropEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationDragDropEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationDragDropEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationDragDropEventHandler {}
impl ::core::fmt::Debug for IDirectManipulationDragDropEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationDragDropEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationDragDropEventHandler {
    type Vtable = IDirectManipulationDragDropEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationDragDropEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fa11b10_701b_41ae_b5f2_49e36bd595aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnDragDropStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationFrameInfoProvider(::windows::core::IUnknown);
impl IDirectManipulationFrameInfoProvider {
    pub unsafe fn GetNextFrameInfo(&self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetNextFrameInfo)(::windows::core::Vtable::as_raw(self), time, processtime, compositiontime).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationFrameInfoProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationFrameInfoProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationFrameInfoProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationFrameInfoProvider {}
impl ::core::fmt::Debug for IDirectManipulationFrameInfoProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationFrameInfoProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationFrameInfoProvider {
    type Vtable = IDirectManipulationFrameInfoProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationFrameInfoProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb759dba_6f4c_4c01_874e_19c8a05907f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationFrameInfoProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNextFrameInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationInteractionEventHandler(::windows::core::IUnknown);
impl IDirectManipulationInteractionEventHandler {
    pub unsafe fn OnInteraction<P0>(&self, viewport: P0, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationViewport2>>,
    {
        (::windows::core::Vtable::vtable(self).OnInteraction)(::windows::core::Vtable::as_raw(self), viewport.into().abi(), interaction).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationInteractionEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationInteractionEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationInteractionEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationInteractionEventHandler {}
impl ::core::fmt::Debug for IDirectManipulationInteractionEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationInteractionEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationInteractionEventHandler {
    type Vtable = IDirectManipulationInteractionEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationInteractionEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe43f45b8_42b4_403e_b1f2_273b8f510830);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationInteractionEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationManager(::windows::core::IUnknown);
impl IDirectManipulationManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<P0>(&self, window: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).Activate)(::windows::core::Vtable::as_raw(self), window.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<P0>(&self, window: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).Deactivate)(::windows::core::Vtable::as_raw(self), window.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<P0, P1>(&self, window: P0, hittestwindow: P1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).RegisterHitTestTarget)(::windows::core::Vtable::as_raw(self), window.into(), hittestwindow.into(), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ProcessInput)(::windows::core::Vtable::as_raw(self), message, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUpdateManager<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUpdateManager)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<P0, P1, T>(&self, frameinfo: P0, window: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationFrameInfoProvider>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateViewport)(::windows::core::Vtable::as_raw(self), frameinfo.into().abi(), window.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateContent<P0, T>(&self, frameinfo: P0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationFrameInfoProvider>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateContent)(::windows::core::Vtable::as_raw(self), frameinfo.into().abi(), clsid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationManager {}
impl ::core::fmt::Debug for IDirectManipulationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationManager {
    type Vtable = IDirectManipulationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbf5d3b4_70c7_4163_9322_5a6f660d6fbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Deactivate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterHitTestTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterHitTestTarget: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub ProcessInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    ProcessInput: usize,
    pub GetUpdateManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateViewport: usize,
    pub CreateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationManager2(::windows::core::IUnknown);
impl IDirectManipulationManager2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<P0>(&self, window: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.Activate)(::windows::core::Vtable::as_raw(self), window.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<P0>(&self, window: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.Deactivate)(::windows::core::Vtable::as_raw(self), window.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<P0, P1>(&self, window: P0, hittestwindow: P1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterHitTestTarget)(::windows::core::Vtable::as_raw(self), window.into(), hittestwindow.into(), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProcessInput)(::windows::core::Vtable::as_raw(self), message, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUpdateManager<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUpdateManager)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<P0, P1, T>(&self, frameinfo: P0, window: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationFrameInfoProvider>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateViewport)(::windows::core::Vtable::as_raw(self), frameinfo.into().abi(), window.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateContent<P0, T>(&self, frameinfo: P0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationFrameInfoProvider>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateContent)(::windows::core::Vtable::as_raw(self), frameinfo.into().abi(), clsid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBehavior<T>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBehavior)(::windows::core::Vtable::as_raw(self), clsid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationManager2, ::windows::core::IUnknown, IDirectManipulationManager);
impl ::core::clone::Clone for IDirectManipulationManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationManager2 {}
impl ::core::fmt::Debug for IDirectManipulationManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationManager2 {
    type Vtable = IDirectManipulationManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa1005e9_3d16_484c_bfc9_62b61e56ec4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager2_Vtbl {
    pub base__: IDirectManipulationManager_Vtbl,
    pub CreateBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationManager3(::windows::core::IUnknown);
impl IDirectManipulationManager3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<P0>(&self, window: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Activate)(::windows::core::Vtable::as_raw(self), window.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<P0>(&self, window: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Deactivate)(::windows::core::Vtable::as_raw(self), window.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<P0, P1>(&self, window: P0, hittestwindow: P1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RegisterHitTestTarget)(::windows::core::Vtable::as_raw(self), window.into(), hittestwindow.into(), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ProcessInput)(::windows::core::Vtable::as_raw(self), message, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUpdateManager<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUpdateManager)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<P0, P1, T>(&self, frameinfo: P0, window: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationFrameInfoProvider>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateViewport)(::windows::core::Vtable::as_raw(self), frameinfo.into().abi(), window.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateContent<P0, T>(&self, frameinfo: P0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationFrameInfoProvider>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateContent)(::windows::core::Vtable::as_raw(self), frameinfo.into().abi(), clsid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBehavior<T>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBehavior)(::windows::core::Vtable::as_raw(self), clsid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetService<T>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetService)(::windows::core::Vtable::as_raw(self), clsid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationManager3, ::windows::core::IUnknown, IDirectManipulationManager, IDirectManipulationManager2);
impl ::core::clone::Clone for IDirectManipulationManager3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationManager3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationManager3 {}
impl ::core::fmt::Debug for IDirectManipulationManager3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationManager3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationManager3 {
    type Vtable = IDirectManipulationManager3_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationManager3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cb6b33d_ffe8_488c_b750_fbdfe88dca8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager3_Vtbl {
    pub base__: IDirectManipulationManager2_Vtbl,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationPrimaryContent(::windows::core::IUnknown);
impl IDirectManipulationPrimaryContent {
    pub unsafe fn SetSnapInterval(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSnapInterval)(::windows::core::Vtable::as_raw(self), motion, interval, offset).ok()
    }
    pub unsafe fn SetSnapPoints(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: ::core::option::Option<&[f32]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSnapPoints)(::windows::core::Vtable::as_raw(self), motion, ::core::mem::transmute(points.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), points.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn SetSnapType(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSnapType)(::windows::core::Vtable::as_raw(self), motion, r#type).ok()
    }
    pub unsafe fn SetSnapCoordinate(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSnapCoordinate)(::windows::core::Vtable::as_raw(self), motion, coordinate, origin).ok()
    }
    pub unsafe fn SetZoomBoundaries(&self, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetZoomBoundaries)(::windows::core::Vtable::as_raw(self), zoomminimum, zoommaximum).ok()
    }
    pub unsafe fn SetHorizontalAlignment(&self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHorizontalAlignment)(::windows::core::Vtable::as_raw(self), alignment).ok()
    }
    pub unsafe fn SetVerticalAlignment(&self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVerticalAlignment)(::windows::core::Vtable::as_raw(self), alignment).ok()
    }
    pub unsafe fn GetInertiaEndTransform(&self, matrix: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetInertiaEndTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(matrix.as_ptr()), matrix.len() as _).ok()
    }
    pub unsafe fn GetCenterPoint(&self, centerx: *mut f32, centery: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCenterPoint)(::windows::core::Vtable::as_raw(self), centerx, centery).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationPrimaryContent, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationPrimaryContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationPrimaryContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationPrimaryContent {}
impl ::core::fmt::Debug for IDirectManipulationPrimaryContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationPrimaryContent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationPrimaryContent {
    type Vtable = IDirectManipulationPrimaryContent_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationPrimaryContent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12851e4_1698_4625_b9b1_7ca3ec18630b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationPrimaryContent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetSnapInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::HRESULT,
    pub SetSnapPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub SetSnapType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::HRESULT,
    pub SetSnapCoordinate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::HRESULT,
    pub SetZoomBoundaries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::HRESULT,
    pub SetHorizontalAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::HRESULT,
    pub SetVerticalAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::HRESULT,
    pub GetInertiaEndTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub GetCenterPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationUpdateHandler(::windows::core::IUnknown);
impl IDirectManipulationUpdateHandler {
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Update)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationUpdateHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationUpdateHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationUpdateHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationUpdateHandler {}
impl ::core::fmt::Debug for IDirectManipulationUpdateHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationUpdateHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationUpdateHandler {
    type Vtable = IDirectManipulationUpdateHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationUpdateHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x790b6337_64f8_4ff5_a269_b32bc2af27a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationUpdateManager(::windows::core::IUnknown);
impl IDirectManipulationUpdateManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterWaitHandleCallback<P0, P1>(&self, handle: P0, eventhandler: P1) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<::windows::core::InParam<IDirectManipulationUpdateHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterWaitHandleCallback)(::windows::core::Vtable::as_raw(self), handle.into(), eventhandler.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterWaitHandleCallback(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterWaitHandleCallback)(::windows::core::Vtable::as_raw(self), cookie).ok()
    }
    pub unsafe fn Update<P0>(&self, frameinfo: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationFrameInfoProvider>>,
    {
        (::windows::core::Vtable::vtable(self).Update)(::windows::core::Vtable::as_raw(self), frameinfo.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationUpdateManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationUpdateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationUpdateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationUpdateManager {}
impl ::core::fmt::Debug for IDirectManipulationUpdateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationUpdateManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationUpdateManager {
    type Vtable = IDirectManipulationUpdateManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationUpdateManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0ae62fd_be34_46e7_9caa_d361facbb9cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterWaitHandleCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterWaitHandleCallback: usize,
    pub UnregisterWaitHandleCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationViewport(::windows::core::IUnknown);
impl IDirectManipulationViewport {
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Enable)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Disable)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetContact)(::windows::core::Vtable::as_raw(self), pointerid).ok()
    }
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReleaseContact)(::windows::core::Vtable::as_raw(self), pointerid).ok()
    }
    pub unsafe fn ReleaseAllContacts(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReleaseAllContacts)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTag<T>(&self, id: ::core::option::Option<*mut u32>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).GetTag)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _, ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewportRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetViewportRect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetViewportRect)(::windows::core::Vtable::as_raw(self), viewport).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZoomToRect<P0>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).ZoomToRect)(::windows::core::Vtable::as_raw(self), left, top, right, bottom, animate.into()).ok()
    }
    pub unsafe fn SetViewportTransform(&self, matrix: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetViewportTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(matrix.as_ptr()), matrix.len() as _).ok()
    }
    pub unsafe fn SyncDisplayTransform(&self, matrix: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SyncDisplayTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(matrix.as_ptr()), matrix.len() as _).ok()
    }
    pub unsafe fn GetPrimaryContent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrimaryContent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddContent<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationContent>>,
    {
        (::windows::core::Vtable::vtable(self).AddContent)(::windows::core::Vtable::as_raw(self), content.into().abi()).ok()
    }
    pub unsafe fn RemoveContent<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationContent>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveContent)(::windows::core::Vtable::as_raw(self), content.into().abi()).ok()
    }
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetViewportOptions)(::windows::core::Vtable::as_raw(self), options).ok()
    }
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddConfiguration)(::windows::core::Vtable::as_raw(self), configuration).ok()
    }
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveConfiguration)(::windows::core::Vtable::as_raw(self), configuration).ok()
    }
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ActivateConfiguration)(::windows::core::Vtable::as_raw(self), configuration).ok()
    }
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetManualGesture)(::windows::core::Vtable::as_raw(self), configuration).ok()
    }
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetChaining)(::windows::core::Vtable::as_raw(self), enabledtypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddEventHandler<P0, P1>(&self, window: P0, eventhandler: P1) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IDirectManipulationViewportEventHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddEventHandler)(::windows::core::Vtable::as_raw(self), window.into(), eventhandler.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveEventHandler)(::windows::core::Vtable::as_raw(self), cookie).ok()
    }
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInputMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUpdateMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Abandon)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationViewport, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationViewport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationViewport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationViewport {}
impl ::core::fmt::Debug for IDirectManipulationViewport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationViewport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationViewport {
    type Vtable = IDirectManipulationViewport_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationViewport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28b85a3d_60a0_48bd_9ba1_5ce8d9ea3a6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub ReleaseContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub ReleaseAllContacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetViewportRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetViewportRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetViewportRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetViewportRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ZoomToRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ZoomToRect: usize,
    pub SetViewportTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub SyncDisplayTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub GetPrimaryContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetViewportOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT,
    pub AddConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub RemoveConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub ActivateConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub SetManualGesture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT,
    pub SetChaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddEventHandler: usize,
    pub RemoveEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub SetInputMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub SetUpdateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationViewport2(::windows::core::IUnknown);
impl IDirectManipulationViewport2 {
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Enable)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Disable)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetContact)(::windows::core::Vtable::as_raw(self), pointerid).ok()
    }
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReleaseContact)(::windows::core::Vtable::as_raw(self), pointerid).ok()
    }
    pub unsafe fn ReleaseAllContacts(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReleaseAllContacts)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTag<T>(&self, id: ::core::option::Option<*mut u32>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.GetTag)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _, ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewportRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetViewportRect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetViewportRect)(::windows::core::Vtable::as_raw(self), viewport).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZoomToRect<P0>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ZoomToRect)(::windows::core::Vtable::as_raw(self), left, top, right, bottom, animate.into()).ok()
    }
    pub unsafe fn SetViewportTransform(&self, matrix: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetViewportTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(matrix.as_ptr()), matrix.len() as _).ok()
    }
    pub unsafe fn SyncDisplayTransform(&self, matrix: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SyncDisplayTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(matrix.as_ptr()), matrix.len() as _).ok()
    }
    pub unsafe fn GetPrimaryContent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPrimaryContent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddContent<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationContent>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddContent)(::windows::core::Vtable::as_raw(self), content.into().abi()).ok()
    }
    pub unsafe fn RemoveContent<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationContent>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveContent)(::windows::core::Vtable::as_raw(self), content.into().abi()).ok()
    }
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetViewportOptions)(::windows::core::Vtable::as_raw(self), options).ok()
    }
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddConfiguration)(::windows::core::Vtable::as_raw(self), configuration).ok()
    }
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveConfiguration)(::windows::core::Vtable::as_raw(self), configuration).ok()
    }
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ActivateConfiguration)(::windows::core::Vtable::as_raw(self), configuration).ok()
    }
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetManualGesture)(::windows::core::Vtable::as_raw(self), configuration).ok()
    }
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetChaining)(::windows::core::Vtable::as_raw(self), enabledtypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddEventHandler<P0, P1>(&self, window: P0, eventhandler: P1) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IDirectManipulationViewportEventHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AddEventHandler)(::windows::core::Vtable::as_raw(self), window.into(), eventhandler.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveEventHandler)(::windows::core::Vtable::as_raw(self), cookie).ok()
    }
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInputMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUpdateMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Abandon)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AddBehavior<P0>(&self, behavior: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddBehavior)(::windows::core::Vtable::as_raw(self), behavior.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveBehavior(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveBehavior)(::windows::core::Vtable::as_raw(self), cookie).ok()
    }
    pub unsafe fn RemoveAllBehaviors(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAllBehaviors)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationViewport2, ::windows::core::IUnknown, IDirectManipulationViewport);
impl ::core::clone::Clone for IDirectManipulationViewport2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationViewport2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationViewport2 {}
impl ::core::fmt::Debug for IDirectManipulationViewport2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationViewport2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationViewport2 {
    type Vtable = IDirectManipulationViewport2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationViewport2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x923ccaac_61e1_4385_b726_017af189882a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport2_Vtbl {
    pub base__: IDirectManipulationViewport_Vtbl,
    pub AddBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub RemoveAllBehaviors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationViewportEventHandler(::windows::core::IUnknown);
impl IDirectManipulationViewportEventHandler {
    pub unsafe fn OnViewportStatusChanged<P0>(&self, viewport: P0, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationViewport>>,
    {
        (::windows::core::Vtable::vtable(self).OnViewportStatusChanged)(::windows::core::Vtable::as_raw(self), viewport.into().abi(), current, previous).ok()
    }
    pub unsafe fn OnViewportUpdated<P0>(&self, viewport: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationViewport>>,
    {
        (::windows::core::Vtable::vtable(self).OnViewportUpdated)(::windows::core::Vtable::as_raw(self), viewport.into().abi()).ok()
    }
    pub unsafe fn OnContentUpdated<P0, P1>(&self, viewport: P0, content: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectManipulationViewport>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDirectManipulationContent>>,
    {
        (::windows::core::Vtable::vtable(self).OnContentUpdated)(::windows::core::Vtable::as_raw(self), viewport.into().abi(), content.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectManipulationViewportEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectManipulationViewportEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationViewportEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationViewportEventHandler {}
impl ::core::fmt::Debug for IDirectManipulationViewportEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationViewportEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDirectManipulationViewportEventHandler {
    type Vtable = IDirectManipulationViewportEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectManipulationViewportEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x952121da_d69f_45f9_b0f9_f23944321a6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewportEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnViewportStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT,
    pub OnViewportUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnContentUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const CLSID_AutoScrollBehavior: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26126a51_3c70_4c9a_aec2_948849eeb093);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const CLSID_DeferContactService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7b67cf4_84bb_434e_86ae_6592bbc9abd9);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const CLSID_DragDropConfigurationBehavior: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09b01b3e_ba6c_454d_82e8_95e352329f23);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const CLSID_HorizontalIndicatorContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7d18cf5_3ec7_44d5_a76b_3770f3cf903d);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const CLSID_VerticalIndicatorContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa10b5f17_afe0_4aa2_91e9_3e7001d2e6b4);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const CLSID_VirtualViewportContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3206a19a_86f0_4cb4_a7f3_16e3b7e2d852);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DCompManipulationCompositor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79dea627_a08a_43ac_8ef5_6900b9299126);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DirectManipulationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54e211b6_3650_4f75_8334_fa359598e1c5);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DirectManipulationPrimaryContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaa02661_d59e_41c7_8393_3ba3bacb6b57);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DirectManipulationSharedManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99793286_77cc_4b57_96db_3b354f6f9fb5);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DirectManipulationUpdateManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fc1bfd5_1835_441a_b3b1_b6cc74b727d0);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DirectManipulationViewport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34e211b6_3650_4f75_8334_fa359598e1c5);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(2i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {}
impl ::core::clone::Clone for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_CONFIGURATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_NONE: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_INTERACTION: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(16i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(32i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(128i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_X: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(256i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_Y: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(512i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_CONFIGURATION {}
impl ::core::clone::Clone for DIRECTMANIPULATION_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_CONFIGURATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_CONFIGURATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_CONFIGURATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(16i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(32i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(64i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {}
impl ::core::clone::Clone for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_DRAG_DROP_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_READY: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_PRESELECT: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_SELECTING: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_DRAGGING: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CANCELLED: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_COMMITTED: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(5i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_DRAG_DROP_STATUS {}
impl ::core::clone::Clone for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_DRAG_DROP_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_GESTURE_CONFIGURATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_NONE: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_DEFAULT: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(8i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(16i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(32i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_GESTURE_CONFIGURATION {}
impl ::core::clone::Clone for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_GESTURE_CONFIGURATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_HITTEST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(2i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_HITTEST_TYPE {}
impl ::core::clone::Clone for DIRECTMANIPULATION_HITTEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_HITTEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_HITTEST_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_HITTEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_HITTEST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_HORIZONTALALIGNMENT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(8i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_HORIZONTALALIGNMENT {}
impl ::core::clone::Clone for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_HORIZONTALALIGNMENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_INPUT_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC: DIRECTMANIPULATION_INPUT_MODE = DIRECTMANIPULATION_INPUT_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INPUT_MODE_MANUAL: DIRECTMANIPULATION_INPUT_MODE = DIRECTMANIPULATION_INPUT_MODE(1i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_INPUT_MODE {}
impl ::core::clone::Clone for DIRECTMANIPULATION_INPUT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_INPUT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_INPUT_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_INPUT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_INPUT_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_INTERACTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_BEGIN: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_END: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(100i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_INTERACTION_TYPE {}
impl ::core::clone::Clone for DIRECTMANIPULATION_INTERACTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_INTERACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_INTERACTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_INTERACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_INTERACTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_MOTION_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_NONE: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEX: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEY: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_ZOOM: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_CENTERX: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(16i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_CENTERY: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(32i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_ALL: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(55i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_MOTION_TYPES {}
impl ::core::clone::Clone for DIRECTMANIPULATION_MOTION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_MOTION_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_MOTION_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_MOTION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_MOTION_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_SNAPPOINT_COORDINATE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_BOUNDARY: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_ORIGIN: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_MIRRORED: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(16i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {}
impl ::core::clone::Clone for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_SNAPPOINT_COORDINATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_SNAPPOINT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(3i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_SNAPPOINT_TYPE {}
impl ::core::clone::Clone for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_SNAPPOINT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_BUILDING: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_ENABLED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DISABLED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_RUNNING: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INERTIA: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_READY: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SUSPENDED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(6i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_STATUS {}
impl ::core::clone::Clone for DIRECTMANIPULATION_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_VERTICALALIGNMENT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_NONE: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_TOP: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(8i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_VERTICALALIGNMENT {}
impl ::core::clone::Clone for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_VERTICALALIGNMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_VERTICALALIGNMENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTMANIPULATION_VIEWPORT_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(8i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(16i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_VIEWPORT_OPTIONS {}
impl ::core::clone::Clone for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_VIEWPORT_OPTIONS").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
