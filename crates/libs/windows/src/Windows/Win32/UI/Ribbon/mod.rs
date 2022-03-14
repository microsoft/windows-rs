#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUIApplication(::windows::core::IUnknown);
impl IUIApplication {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn OnViewChanged<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, viewid: u32, typeid: UI_VIEWTYPE, view: Param2, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnViewChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewid), ::core::mem::transmute(typeid), view.into_param().abi(), ::core::mem::transmute(verb), ::core::mem::transmute(ureasoncode)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn OnCreateUICommand(&self, commandid: u32, typeid: UI_COMMANDTYPE) -> ::windows::core::Result<IUICommandHandler> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OnCreateUICommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(typeid), ::core::mem::transmute(&mut result__)).from_abi::<IUICommandHandler>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn OnDestroyUICommand<'a, Param2: ::windows::core::IntoParam<'a, IUICommandHandler>>(&self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDestroyUICommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(typeid), commandhandler.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIApplication> for ::windows::core::IUnknown {
    fn from(value: IUIApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIApplication> for ::windows::core::IUnknown {
    fn from(value: &IUIApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIApplication {}
impl ::core::fmt::Debug for IUIApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIApplication").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIApplication {
    type Vtable = IUIApplication_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd428903c_729a_491d_910d_682a08ff2522);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIApplication_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnViewChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: u32, typeid: UI_VIEWTYPE, view: *mut ::core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::core::HRESULT,
    pub OnCreateUICommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnDestroyUICommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUICollection(::windows::core::IUnknown);
impl IUICollection {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn GetItem(&self, index: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, item: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn Insert<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, item: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Insert)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), item.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn Replace<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, indexreplaced: u32, itemreplacewith: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Replace)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexreplaced), itemreplacewith.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IUICollection> for ::windows::core::IUnknown {
    fn from(value: IUICollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUICollection> for ::windows::core::IUnknown {
    fn from(value: &IUICollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUICollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUICollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUICollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUICollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUICollection {}
impl ::core::fmt::Debug for IUICollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUICollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUICollection {
    type Vtable = IUICollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf4f45bf_6f9d_4dd7_9d68_d8f9cd18c4db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Insert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub Replace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexreplaced: u32, itemreplacewith: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUICollectionChangedEvent(::windows::core::IUnknown);
impl IUICollectionChangedEvent {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn OnChanged<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: Param2, newindex: u32, newitem: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(action), ::core::mem::transmute(oldindex), olditem.into_param().abi(), ::core::mem::transmute(newindex), newitem.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUICollectionChangedEvent> for ::windows::core::IUnknown {
    fn from(value: IUICollectionChangedEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUICollectionChangedEvent> for ::windows::core::IUnknown {
    fn from(value: &IUICollectionChangedEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUICollectionChangedEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUICollectionChangedEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUICollectionChangedEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUICollectionChangedEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUICollectionChangedEvent {}
impl ::core::fmt::Debug for IUICollectionChangedEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUICollectionChangedEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUICollectionChangedEvent {
    type Vtable = IUICollectionChangedEvent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6502ae91_a14d_44b5_bbd0_62aacc581d52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICollectionChangedEvent_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut ::core::ffi::c_void, newindex: u32, newitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUICommandHandler(::windows::core::IUnknown);
impl IUICommandHandler {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Execute<'a, Param4: ::windows::core::IntoParam<'a, IUISimplePropertySet>>(&self, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Execute)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(verb), ::core::mem::transmute(key), ::core::mem::transmute(currentvalue), commandexecutionproperties.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn UpdateProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UpdateProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(key), ::core::mem::transmute(currentvalue), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
impl ::core::convert::From<IUICommandHandler> for ::windows::core::IUnknown {
    fn from(value: IUICommandHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUICommandHandler> for ::windows::core::IUnknown {
    fn from(value: &IUICommandHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUICommandHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUICommandHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUICommandHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUICommandHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUICommandHandler {}
impl ::core::fmt::Debug for IUICommandHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUICommandHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUICommandHandler {
    type Vtable = IUICommandHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75ae0a2d_dc03_4c9f_8883_069660d0beb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommandHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Execute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub UpdateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, newvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    UpdateProperty: usize,
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUIContextualUI(::windows::core::IUnknown);
impl IUIContextualUI {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn ShowAtLocation(&self, x: i32, y: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShowAtLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
}
impl ::core::convert::From<IUIContextualUI> for ::windows::core::IUnknown {
    fn from(value: IUIContextualUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIContextualUI> for ::windows::core::IUnknown {
    fn from(value: &IUIContextualUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIContextualUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIContextualUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIContextualUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIContextualUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIContextualUI {}
impl ::core::fmt::Debug for IUIContextualUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIContextualUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIContextualUI {
    type Vtable = IUIContextualUI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea11f37_7c46_437c_8e55_b52122b29293);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContextualUI_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ShowAtLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUIEventLogger(::windows::core::IUnknown);
impl IUIEventLogger {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn OnUIEvent(&self, peventparams: *const UI_EVENTPARAMS) {
        (::windows::core::Interface::vtable(self).OnUIEvent)(::core::mem::transmute_copy(self), ::core::mem::transmute(peventparams))
    }
}
impl ::core::convert::From<IUIEventLogger> for ::windows::core::IUnknown {
    fn from(value: IUIEventLogger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIEventLogger> for ::windows::core::IUnknown {
    fn from(value: &IUIEventLogger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIEventLogger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIEventLogger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIEventLogger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIEventLogger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIEventLogger {}
impl ::core::fmt::Debug for IUIEventLogger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIEventLogger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIEventLogger {
    type Vtable = IUIEventLogger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec3e1034_dbf4_41a1_95d5_03e0f1026e05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIEventLogger_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnUIEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventparams: *const UI_EVENTPARAMS),
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUIEventingManager(::windows::core::IUnknown);
impl IUIEventingManager {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn SetEventLogger<'a, Param0: ::windows::core::IntoParam<'a, IUIEventLogger>>(&self, eventlogger: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventLogger)(::core::mem::transmute_copy(self), eventlogger.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIEventingManager> for ::windows::core::IUnknown {
    fn from(value: IUIEventingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIEventingManager> for ::windows::core::IUnknown {
    fn from(value: &IUIEventingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIEventingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIEventingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIEventingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIEventingManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIEventingManager {}
impl ::core::fmt::Debug for IUIEventingManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIEventingManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIEventingManager {
    type Vtable = IUIEventingManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3be6ea7f_9a9b_4198_9368_9b0f923bd534);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIEventingManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetEventLogger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventlogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUIFramework(::windows::core::IUnknown);
impl IUIFramework {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, IUIApplication>>(&self, framewnd: Param0, application: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), framewnd.into_param().abi(), application.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Destroy)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadUI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, instance: Param0, resourcename: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadUI)(::core::mem::transmute_copy(self), instance.into_param().abi(), resourcename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn GetView(&self, viewid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetView)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewid), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUICommandProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUICommandProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn InvalidateUICommand(&self, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InvalidateUICommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(flags), ::core::mem::transmute(key)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn FlushPendingInvalidations(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlushPendingInvalidations)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn SetModes(&self, imodes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetModes)(::core::mem::transmute_copy(self), ::core::mem::transmute(imodes)).ok()
    }
}
impl ::core::convert::From<IUIFramework> for ::windows::core::IUnknown {
    fn from(value: IUIFramework) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIFramework> for ::windows::core::IUnknown {
    fn from(value: &IUIFramework) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIFramework {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIFramework {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIFramework {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIFramework {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIFramework {}
impl ::core::fmt::Debug for IUIFramework {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIFramework").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIFramework {
    type Vtable = IUIFramework_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4f0385d_6872_43a8_ad09_4c339cb3f5c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIFramework_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framewnd: super::super::Foundation::HWND, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instance: super::super::Foundation::HINSTANCE, resourcename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadUI: usize,
    pub GetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetUICommandProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetUICommandProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetUICommandProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetUICommandProperty: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub InvalidateUICommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    InvalidateUICommand: usize,
    pub FlushPendingInvalidations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imodes: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUIImage(::windows::core::IUnknown);
impl IUIImage {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetBitmap(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__: super::super::Graphics::Gdi::HBITMAP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBitmap)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Graphics::Gdi::HBITMAP>(result__)
    }
}
impl ::core::convert::From<IUIImage> for ::windows::core::IUnknown {
    fn from(value: IUIImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIImage> for ::windows::core::IUnknown {
    fn from(value: &IUIImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIImage {}
impl ::core::fmt::Debug for IUIImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIImage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIImage {
    type Vtable = IUIImage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23c8c838_4de6_436b_ab01_5554bb7c30dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIImage_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetBitmap: usize,
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUIImageFromBitmap(::windows::core::IUnknown);
impl IUIImageFromBitmap {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateImage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, bitmap: Param0, options: UI_OWNERSHIP) -> ::windows::core::Result<IUIImage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateImage)(::core::mem::transmute_copy(self), bitmap.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IUIImage>(result__)
    }
}
impl ::core::convert::From<IUIImageFromBitmap> for ::windows::core::IUnknown {
    fn from(value: IUIImageFromBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIImageFromBitmap> for ::windows::core::IUnknown {
    fn from(value: &IUIImageFromBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIImageFromBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIImageFromBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIImageFromBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIImageFromBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIImageFromBitmap {}
impl ::core::fmt::Debug for IUIImageFromBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIImageFromBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIImageFromBitmap {
    type Vtable = IUIImageFromBitmap_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18aba7f3_4c1c_4ba2_bf6c_f5c3326fa816);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIImageFromBitmap_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateImage: usize,
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUIRibbon(::windows::core::IUnknown);
impl IUIRibbon {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
    pub unsafe fn GetHeight(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHeight)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadSettingsFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadSettingsFromStream)(::core::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveSettingsToStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveSettingsToStream)(::core::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIRibbon> for ::windows::core::IUnknown {
    fn from(value: IUIRibbon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIRibbon> for ::windows::core::IUnknown {
    fn from(value: &IUIRibbon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIRibbon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIRibbon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIRibbon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIRibbon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIRibbon {}
impl ::core::fmt::Debug for IUIRibbon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIRibbon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIRibbon {
    type Vtable = IUIRibbon_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x803982ab_370a_4f7e_a9e7_8784036a6e26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIRibbon_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cy: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadSettingsFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadSettingsFromStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveSettingsToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveSettingsToStream: usize,
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
pub struct IUISimplePropertySet(::windows::core::IUnknown);
impl IUISimplePropertySet {
    #[doc = "*Required features: `\"Win32_UI_Ribbon\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
impl ::core::convert::From<IUISimplePropertySet> for ::windows::core::IUnknown {
    fn from(value: IUISimplePropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUISimplePropertySet> for ::windows::core::IUnknown {
    fn from(value: &IUISimplePropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUISimplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUISimplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUISimplePropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUISimplePropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUISimplePropertySet {}
impl ::core::fmt::Debug for IUISimplePropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUISimplePropertySet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUISimplePropertySet {
    type Vtable = IUISimplePropertySet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc205bb48_5b1c_4219_a106_15bd0a5f24e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISimplePropertySet_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetValue: usize,
}
pub const LIBID_UIRibbon: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x942f35c2_e83b_45ef_b085_ac295dd63d5b);
pub const UIRibbonFramework: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x926749fa_2615_4987_8845_c33e65f2b957);
pub const UIRibbonImageFromBitmapFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f7434b6_59b6_4250_999e_d168d6ae4293);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_ALL_COMMANDS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_COLLECTIONCHANGE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COLLECTIONCHANGE_INSERT: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COLLECTIONCHANGE_REMOVE: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COLLECTIONCHANGE_REPLACE: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(2i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COLLECTIONCHANGE_RESET: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(3i32);
impl ::core::marker::Copy for UI_COLLECTIONCHANGE {}
impl ::core::clone::Clone for UI_COLLECTIONCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_COLLECTIONCHANGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_COLLECTIONCHANGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_COLLECTIONCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_COLLECTIONCHANGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COLLECTION_INVALIDINDEX: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_COMMANDTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_UNKNOWN: UI_COMMANDTYPE = UI_COMMANDTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_GROUP: UI_COMMANDTYPE = UI_COMMANDTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_ACTION: UI_COMMANDTYPE = UI_COMMANDTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_ANCHOR: UI_COMMANDTYPE = UI_COMMANDTYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_CONTEXT: UI_COMMANDTYPE = UI_COMMANDTYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_COLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_COMMANDCOLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_DECIMAL: UI_COMMANDTYPE = UI_COMMANDTYPE(7i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_BOOLEAN: UI_COMMANDTYPE = UI_COMMANDTYPE(8i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_FONT: UI_COMMANDTYPE = UI_COMMANDTYPE(9i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_RECENTITEMS: UI_COMMANDTYPE = UI_COMMANDTYPE(10i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_COLORANCHOR: UI_COMMANDTYPE = UI_COMMANDTYPE(11i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_COLORCOLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(12i32);
impl ::core::marker::Copy for UI_COMMANDTYPE {}
impl ::core::clone::Clone for UI_COMMANDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_COMMANDTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_COMMANDTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_COMMANDTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_COMMANDTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_CONTEXTAVAILABILITY(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_CONTEXTAVAILABILITY_NOTAVAILABLE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_CONTEXTAVAILABILITY_AVAILABLE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_CONTEXTAVAILABILITY_ACTIVE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(2i32);
impl ::core::marker::Copy for UI_CONTEXTAVAILABILITY {}
impl ::core::clone::Clone for UI_CONTEXTAVAILABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_CONTEXTAVAILABILITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_CONTEXTAVAILABILITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_CONTEXTAVAILABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_CONTEXTAVAILABILITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_CONTROLDOCK(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_CONTROLDOCK_TOP: UI_CONTROLDOCK = UI_CONTROLDOCK(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_CONTROLDOCK_BOTTOM: UI_CONTROLDOCK = UI_CONTROLDOCK(3i32);
impl ::core::marker::Copy for UI_CONTROLDOCK {}
impl ::core::clone::Clone for UI_CONTROLDOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_CONTROLDOCK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_CONTROLDOCK {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_CONTROLDOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_CONTROLDOCK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_EVENTLOCATION(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTLOCATION_Ribbon: UI_EVENTLOCATION = UI_EVENTLOCATION(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTLOCATION_QAT: UI_EVENTLOCATION = UI_EVENTLOCATION(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTLOCATION_ApplicationMenu: UI_EVENTLOCATION = UI_EVENTLOCATION(2i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTLOCATION_ContextPopup: UI_EVENTLOCATION = UI_EVENTLOCATION(3i32);
impl ::core::marker::Copy for UI_EVENTLOCATION {}
impl ::core::clone::Clone for UI_EVENTLOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_EVENTLOCATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_EVENTLOCATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_EVENTLOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_EVENTLOCATION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub struct UI_EVENTPARAMS {
    pub EventType: UI_EVENTTYPE,
    pub Anonymous: UI_EVENTPARAMS_0,
}
impl ::core::marker::Copy for UI_EVENTPARAMS {}
impl ::core::clone::Clone for UI_EVENTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UI_EVENTPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UI_EVENTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UI_EVENTPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for UI_EVENTPARAMS {}
impl ::core::default::Default for UI_EVENTPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub union UI_EVENTPARAMS_0 {
    pub Modes: i32,
    pub Params: UI_EVENTPARAMS_COMMAND,
}
impl ::core::marker::Copy for UI_EVENTPARAMS_0 {}
impl ::core::clone::Clone for UI_EVENTPARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UI_EVENTPARAMS_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UI_EVENTPARAMS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UI_EVENTPARAMS_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for UI_EVENTPARAMS_0 {}
impl ::core::default::Default for UI_EVENTPARAMS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub struct UI_EVENTPARAMS_COMMAND {
    pub CommandID: u32,
    pub CommandName: ::windows::core::PCWSTR,
    pub ParentCommandID: u32,
    pub ParentCommandName: ::windows::core::PCWSTR,
    pub SelectionIndex: u32,
    pub Location: UI_EVENTLOCATION,
}
impl ::core::marker::Copy for UI_EVENTPARAMS_COMMAND {}
impl ::core::clone::Clone for UI_EVENTPARAMS_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UI_EVENTPARAMS_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UI_EVENTPARAMS_COMMAND").field("CommandID", &self.CommandID).field("CommandName", &self.CommandName).field("ParentCommandID", &self.ParentCommandID).field("ParentCommandName", &self.ParentCommandName).field("SelectionIndex", &self.SelectionIndex).field("Location", &self.Location).finish()
    }
}
unsafe impl ::windows::core::Abi for UI_EVENTPARAMS_COMMAND {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UI_EVENTPARAMS_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UI_EVENTPARAMS_COMMAND>()) == 0 }
    }
}
impl ::core::cmp::Eq for UI_EVENTPARAMS_COMMAND {}
impl ::core::default::Default for UI_EVENTPARAMS_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_EVENTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_ApplicationMenuOpened: UI_EVENTTYPE = UI_EVENTTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_RibbonMinimized: UI_EVENTTYPE = UI_EVENTTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_RibbonExpanded: UI_EVENTTYPE = UI_EVENTTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_ApplicationModeSwitched: UI_EVENTTYPE = UI_EVENTTYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_TabActivated: UI_EVENTTYPE = UI_EVENTTYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_MenuOpened: UI_EVENTTYPE = UI_EVENTTYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_CommandExecuted: UI_EVENTTYPE = UI_EVENTTYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_TooltipShown: UI_EVENTTYPE = UI_EVENTTYPE(7i32);
impl ::core::marker::Copy for UI_EVENTTYPE {}
impl ::core::clone::Clone for UI_EVENTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_EVENTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_EVENTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_EVENTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_EVENTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_EXECUTIONVERB(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EXECUTIONVERB_EXECUTE: UI_EXECUTIONVERB = UI_EXECUTIONVERB(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EXECUTIONVERB_PREVIEW: UI_EXECUTIONVERB = UI_EXECUTIONVERB(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EXECUTIONVERB_CANCELPREVIEW: UI_EXECUTIONVERB = UI_EXECUTIONVERB(2i32);
impl ::core::marker::Copy for UI_EXECUTIONVERB {}
impl ::core::clone::Clone for UI_EXECUTIONVERB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_EXECUTIONVERB {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_EXECUTIONVERB {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_EXECUTIONVERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_EXECUTIONVERB").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_FONTDELTASIZE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTDELTASIZE_GROW: UI_FONTDELTASIZE = UI_FONTDELTASIZE(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTDELTASIZE_SHRINK: UI_FONTDELTASIZE = UI_FONTDELTASIZE(1i32);
impl ::core::marker::Copy for UI_FONTDELTASIZE {}
impl ::core::clone::Clone for UI_FONTDELTASIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_FONTDELTASIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_FONTDELTASIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_FONTDELTASIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_FONTDELTASIZE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_FONTPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTPROPERTIES_NOTAVAILABLE: UI_FONTPROPERTIES = UI_FONTPROPERTIES(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTPROPERTIES_NOTSET: UI_FONTPROPERTIES = UI_FONTPROPERTIES(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTPROPERTIES_SET: UI_FONTPROPERTIES = UI_FONTPROPERTIES(2i32);
impl ::core::marker::Copy for UI_FONTPROPERTIES {}
impl ::core::clone::Clone for UI_FONTPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_FONTPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_FONTPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_FONTPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_FONTPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_FONTUNDERLINE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTUNDERLINE_NOTAVAILABLE: UI_FONTUNDERLINE = UI_FONTUNDERLINE(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTUNDERLINE_NOTSET: UI_FONTUNDERLINE = UI_FONTUNDERLINE(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTUNDERLINE_SET: UI_FONTUNDERLINE = UI_FONTUNDERLINE(2i32);
impl ::core::marker::Copy for UI_FONTUNDERLINE {}
impl ::core::clone::Clone for UI_FONTUNDERLINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_FONTUNDERLINE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_FONTUNDERLINE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_FONTUNDERLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_FONTUNDERLINE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_FONTVERTICALPOSITION(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTVERTICALPOSITION_NOTAVAILABLE: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTVERTICALPOSITION_NOTSET: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTVERTICALPOSITION_SUPERSCRIPT: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(2i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTVERTICALPOSITION_SUBSCRIPT: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(3i32);
impl ::core::marker::Copy for UI_FONTVERTICALPOSITION {}
impl ::core::clone::Clone for UI_FONTVERTICALPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_FONTVERTICALPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_FONTVERTICALPOSITION {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_FONTVERTICALPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_FONTVERTICALPOSITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_INVALIDATIONS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_INVALIDATIONS_STATE: UI_INVALIDATIONS = UI_INVALIDATIONS(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_INVALIDATIONS_VALUE: UI_INVALIDATIONS = UI_INVALIDATIONS(2i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_INVALIDATIONS_PROPERTY: UI_INVALIDATIONS = UI_INVALIDATIONS(4i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_INVALIDATIONS_ALLPROPERTIES: UI_INVALIDATIONS = UI_INVALIDATIONS(8i32);
impl ::core::marker::Copy for UI_INVALIDATIONS {}
impl ::core::clone::Clone for UI_INVALIDATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_INVALIDATIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_INVALIDATIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_INVALIDATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_INVALIDATIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_OWNERSHIP(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_OWNERSHIP_TRANSFER: UI_OWNERSHIP = UI_OWNERSHIP(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_OWNERSHIP_COPY: UI_OWNERSHIP = UI_OWNERSHIP(1i32);
impl ::core::marker::Copy for UI_OWNERSHIP {}
impl ::core::clone::Clone for UI_OWNERSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_OWNERSHIP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_OWNERSHIP {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_OWNERSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_OWNERSHIP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_SWATCHCOLORMODE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_SWATCHCOLORMODE_NORMAL: UI_SWATCHCOLORMODE = UI_SWATCHCOLORMODE(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_SWATCHCOLORMODE_MONOCHROME: UI_SWATCHCOLORMODE = UI_SWATCHCOLORMODE(1i32);
impl ::core::marker::Copy for UI_SWATCHCOLORMODE {}
impl ::core::clone::Clone for UI_SWATCHCOLORMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_SWATCHCOLORMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_SWATCHCOLORMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_SWATCHCOLORMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_SWATCHCOLORMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_SWATCHCOLORTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_SWATCHCOLORTYPE_NOCOLOR: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_SWATCHCOLORTYPE_AUTOMATIC: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_SWATCHCOLORTYPE_RGB: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(2i32);
impl ::core::marker::Copy for UI_SWATCHCOLORTYPE {}
impl ::core::clone::Clone for UI_SWATCHCOLORTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_SWATCHCOLORTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_SWATCHCOLORTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_SWATCHCOLORTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_SWATCHCOLORTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_VIEWTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_VIEWTYPE_RIBBON: UI_VIEWTYPE = UI_VIEWTYPE(1i32);
impl ::core::marker::Copy for UI_VIEWTYPE {}
impl ::core::clone::Clone for UI_VIEWTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_VIEWTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_VIEWTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_VIEWTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_VIEWTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_VIEWVERB(pub i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_VIEWVERB_CREATE: UI_VIEWVERB = UI_VIEWVERB(0i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_VIEWVERB_DESTROY: UI_VIEWVERB = UI_VIEWVERB(1i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_VIEWVERB_SIZE: UI_VIEWVERB = UI_VIEWVERB(2i32);
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_VIEWVERB_ERROR: UI_VIEWVERB = UI_VIEWVERB(3i32);
impl ::core::marker::Copy for UI_VIEWVERB {}
impl ::core::clone::Clone for UI_VIEWVERB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_VIEWVERB {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_VIEWVERB {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_VIEWVERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_VIEWVERB").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
