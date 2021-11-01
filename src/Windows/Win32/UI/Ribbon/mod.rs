#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUIApplication(::windows::runtime::IUnknown);
impl IUIApplication {
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn OnViewChanged<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, viewid: u32, typeid: UI_VIEWTYPE, view: Param2, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(viewid), ::std::mem::transmute(typeid), view.into_param().abi(), ::std::mem::transmute(verb), ::std::mem::transmute(ureasoncode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn OnCreateUICommand(&self, commandid: u32, typeid: UI_COMMANDTYPE) -> ::windows::runtime::Result<IUICommandHandler> {
        let mut result__: <IUICommandHandler as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(commandid), ::std::mem::transmute(typeid), &mut result__).from_abi::<IUICommandHandler>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn OnDestroyUICommand<'a, Param2: ::windows::runtime::IntoParam<'a, IUICommandHandler>>(&self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(commandid), ::std::mem::transmute(typeid), commandhandler.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIApplication {
    type Vtable = IUIApplication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3559428156, 29338, 18717, [145, 13, 104, 42, 8, 255, 37, 34]);
}
impl ::std::convert::From<IUIApplication> for ::windows::runtime::IUnknown {
    fn from(value: IUIApplication) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUIApplication> for ::windows::runtime::IUnknown {
    fn from(value: &IUIApplication) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUIApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewid: u32, typeid: UI_VIEWTYPE, view: ::windows::runtime::RawPtr, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUICollection(::windows::runtime::IUnknown);
impl IUICollection {
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn GetItem(&self, index: u32) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, item: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn Insert<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, index: u32, item: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), item.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn Replace<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, indexreplaced: u32, itemreplacewith: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(indexreplaced), itemreplacewith.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUICollection {
    type Vtable = IUICollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3746514367, 28573, 19927, [157, 104, 216, 249, 205, 24, 196, 219]);
}
impl ::std::convert::From<IUICollection> for ::windows::runtime::IUnknown {
    fn from(value: IUICollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUICollection> for ::windows::runtime::IUnknown {
    fn from(value: &IUICollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUICollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUICollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, item: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, item: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, indexreplaced: u32, itemreplacewith: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUICollectionChangedEvent(::windows::runtime::IUnknown);
impl IUICollectionChangedEvent {
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn OnChanged<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: Param2, newindex: u32, newitem: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(action), ::std::mem::transmute(oldindex), olditem.into_param().abi(), ::std::mem::transmute(newindex), newitem.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUICollectionChangedEvent {
    type Vtable = IUICollectionChangedEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1694674577, 41293, 17589, [187, 208, 98, 170, 204, 88, 29, 82]);
}
impl ::std::convert::From<IUICollectionChangedEvent> for ::windows::runtime::IUnknown {
    fn from(value: IUICollectionChangedEvent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUICollectionChangedEvent> for ::windows::runtime::IUnknown {
    fn from(value: &IUICollectionChangedEvent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUICollectionChangedEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUICollectionChangedEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICollectionChangedEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: ::windows::runtime::RawPtr, newindex: u32, newitem: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUICommandHandler(::windows::runtime::IUnknown);
impl IUICommandHandler {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Execute<'a, Param4: ::windows::runtime::IntoParam<'a, IUISimplePropertySet>>(&self, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(commandid), ::std::mem::transmute(verb), ::std::mem::transmute(key), ::std::mem::transmute(currentvalue), commandexecutionproperties.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn UpdateProperty(&self, commandid: u32, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(commandid), ::std::mem::transmute(key), ::std::mem::transmute(currentvalue), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUICommandHandler {
    type Vtable = IUICommandHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1974340141, 56323, 19615, [136, 131, 6, 150, 96, 208, 190, 182]);
}
impl ::std::convert::From<IUICommandHandler> for ::windows::runtime::IUnknown {
    fn from(value: IUICommandHandler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUICommandHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IUICommandHandler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUICommandHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUICommandHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommandHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, currentvalue: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, commandexecutionproperties: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u32, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, currentvalue: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, newvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUIContextualUI(::windows::runtime::IUnknown);
impl IUIContextualUI {
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn ShowAtLocation(&self, x: i32, y: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIContextualUI {
    type Vtable = IUIContextualUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4003536695, 31814, 17276, [142, 85, 181, 33, 34, 178, 146, 147]);
}
impl ::std::convert::From<IUIContextualUI> for ::windows::runtime::IUnknown {
    fn from(value: IUIContextualUI) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUIContextualUI> for ::windows::runtime::IUnknown {
    fn from(value: &IUIContextualUI) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIContextualUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUIContextualUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContextualUI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUIEventLogger(::windows::runtime::IUnknown);
impl IUIEventLogger {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`*"]
    pub unsafe fn OnUIEvent(&self, peventparams: *const UI_EVENTPARAMS) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(peventparams)))
    }
}
unsafe impl ::windows::runtime::Interface for IUIEventLogger {
    type Vtable = IUIEventLogger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3963490356, 56308, 16801, [149, 213, 3, 224, 241, 2, 110, 5]);
}
impl ::std::convert::From<IUIEventLogger> for ::windows::runtime::IUnknown {
    fn from(value: IUIEventLogger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUIEventLogger> for ::windows::runtime::IUnknown {
    fn from(value: &IUIEventLogger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIEventLogger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUIEventLogger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIEventLogger_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peventparams: *const UI_EVENTPARAMS),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUIEventingManager(::windows::runtime::IUnknown);
impl IUIEventingManager {
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn SetEventLogger<'a, Param0: ::windows::runtime::IntoParam<'a, IUIEventLogger>>(&self, eventlogger: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), eventlogger.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIEventingManager {
    type Vtable = IUIEventingManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1004989055, 39579, 16792, [147, 104, 155, 15, 146, 59, 213, 52]);
}
impl ::std::convert::From<IUIEventingManager> for ::windows::runtime::IUnknown {
    fn from(value: IUIEventingManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUIEventingManager> for ::windows::runtime::IUnknown {
    fn from(value: &IUIEventingManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIEventingManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUIEventingManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIEventingManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventlogger: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUIFramework(::windows::runtime::IUnknown);
impl IUIFramework {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, IUIApplication>>(&self, framewnd: Param0, application: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), framewnd.into_param().abi(), application.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn Destroy(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`*"]
    pub unsafe fn LoadUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, instance: Param0, resourcename: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), instance.into_param().abi(), resourcename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn GetView(&self, viewid: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(viewid), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetUICommandProperty(&self, commandid: u32, key: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(commandid), ::std::mem::transmute(key), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetUICommandProperty(&self, commandid: u32, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(commandid), ::std::mem::transmute(key), ::std::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn InvalidateUICommand(&self, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(commandid), ::std::mem::transmute(flags), ::std::mem::transmute(key)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn FlushPendingInvalidations(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn SetModes(&self, imodes: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(imodes)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIFramework {
    type Vtable = IUIFramework_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4109383773, 26738, 17320, [173, 9, 76, 51, 156, 179, 245, 197]);
}
impl ::std::convert::From<IUIFramework> for ::windows::runtime::IUnknown {
    fn from(value: IUIFramework) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUIFramework> for ::windows::runtime::IUnknown {
    fn from(value: &IUIFramework) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIFramework {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUIFramework {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIFramework_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, framewnd: super::super::Foundation::HWND, application: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instance: super::super::Foundation::HINSTANCE, resourcename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewid: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u32, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, value: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u32, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, value: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imodes: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUIImage(::windows::runtime::IUnknown);
impl IUIImage {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetBitmap(&self) -> ::windows::runtime::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__: <super::super::Graphics::Gdi::HBITMAP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Gdi::HBITMAP>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUIImage {
    type Vtable = IUIImage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(600361016, 19942, 17259, [171, 1, 85, 84, 187, 124, 48, 221]);
}
impl ::std::convert::From<IUIImage> for ::windows::runtime::IUnknown {
    fn from(value: IUIImage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUIImage> for ::windows::runtime::IUnknown {
    fn from(value: &IUIImage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUIImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIImage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUIImageFromBitmap(::windows::runtime::IUnknown);
impl IUIImageFromBitmap {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CreateImage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, bitmap: Param0, options: UI_OWNERSHIP) -> ::windows::runtime::Result<IUIImage> {
        let mut result__: <IUIImage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bitmap.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IUIImage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUIImageFromBitmap {
    type Vtable = IUIImageFromBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(413902835, 19484, 19362, [191, 108, 245, 195, 50, 111, 168, 22]);
}
impl ::std::convert::From<IUIImageFromBitmap> for ::windows::runtime::IUnknown {
    fn from(value: IUIImageFromBitmap) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUIImageFromBitmap> for ::windows::runtime::IUnknown {
    fn from(value: &IUIImageFromBitmap) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIImageFromBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUIImageFromBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIImageFromBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUIRibbon(::windows::runtime::IUnknown);
impl IUIRibbon {
    #[doc = "*Required features: `Win32_UI_Ribbon`*"]
    pub unsafe fn GetHeight(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_System_Com`*"]
    pub unsafe fn LoadSettingsFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_System_Com`*"]
    pub unsafe fn SaveSettingsToStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIRibbon {
    type Vtable = IUIRibbon_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2151252651, 14090, 20350, [169, 231, 135, 132, 3, 106, 110, 38]);
}
impl ::std::convert::From<IUIRibbon> for ::windows::runtime::IUnknown {
    fn from(value: IUIRibbon) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUIRibbon> for ::windows::runtime::IUnknown {
    fn from(value: &IUIRibbon) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIRibbon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUIRibbon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIRibbon_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cy: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUISimplePropertySet(::windows::runtime::IUnknown);
impl IUISimplePropertySet {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetValue(&self, key: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUISimplePropertySet {
    type Vtable = IUISimplePropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3255155528, 23324, 16921, [161, 6, 21, 189, 10, 95, 36, 226]);
}
impl ::std::convert::From<IUISimplePropertySet> for ::windows::runtime::IUnknown {
    fn from(value: IUISimplePropertySet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUISimplePropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &IUISimplePropertySet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUISimplePropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUISimplePropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISimplePropertySet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, value: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
);
pub const LIBID_UIRibbon: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2486121922, 59451, 17903, [176, 133, 172, 41, 93, 214, 61, 91]);
pub const UIRibbonFramework: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2456242682, 9749, 18823, [136, 69, 195, 62, 101, 242, 185, 87]);
pub const UIRibbonImageFromBitmapFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(259273910, 22966, 16976, [153, 158, 209, 104, 214, 174, 66, 147]);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
pub const UI_ALL_COMMANDS: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_COLLECTIONCHANGE(pub i32);
pub const UI_COLLECTIONCHANGE_INSERT: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(0i32);
pub const UI_COLLECTIONCHANGE_REMOVE: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(1i32);
pub const UI_COLLECTIONCHANGE_REPLACE: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(2i32);
pub const UI_COLLECTIONCHANGE_RESET: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(3i32);
impl ::std::convert::From<i32> for UI_COLLECTIONCHANGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_COLLECTIONCHANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
pub const UI_COLLECTION_INVALIDINDEX: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_COMMANDTYPE(pub i32);
pub const UI_COMMANDTYPE_UNKNOWN: UI_COMMANDTYPE = UI_COMMANDTYPE(0i32);
pub const UI_COMMANDTYPE_GROUP: UI_COMMANDTYPE = UI_COMMANDTYPE(1i32);
pub const UI_COMMANDTYPE_ACTION: UI_COMMANDTYPE = UI_COMMANDTYPE(2i32);
pub const UI_COMMANDTYPE_ANCHOR: UI_COMMANDTYPE = UI_COMMANDTYPE(3i32);
pub const UI_COMMANDTYPE_CONTEXT: UI_COMMANDTYPE = UI_COMMANDTYPE(4i32);
pub const UI_COMMANDTYPE_COLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(5i32);
pub const UI_COMMANDTYPE_COMMANDCOLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(6i32);
pub const UI_COMMANDTYPE_DECIMAL: UI_COMMANDTYPE = UI_COMMANDTYPE(7i32);
pub const UI_COMMANDTYPE_BOOLEAN: UI_COMMANDTYPE = UI_COMMANDTYPE(8i32);
pub const UI_COMMANDTYPE_FONT: UI_COMMANDTYPE = UI_COMMANDTYPE(9i32);
pub const UI_COMMANDTYPE_RECENTITEMS: UI_COMMANDTYPE = UI_COMMANDTYPE(10i32);
pub const UI_COMMANDTYPE_COLORANCHOR: UI_COMMANDTYPE = UI_COMMANDTYPE(11i32);
pub const UI_COMMANDTYPE_COLORCOLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(12i32);
impl ::std::convert::From<i32> for UI_COMMANDTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_COMMANDTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_CONTEXTAVAILABILITY(pub i32);
pub const UI_CONTEXTAVAILABILITY_NOTAVAILABLE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(0i32);
pub const UI_CONTEXTAVAILABILITY_AVAILABLE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(1i32);
pub const UI_CONTEXTAVAILABILITY_ACTIVE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(2i32);
impl ::std::convert::From<i32> for UI_CONTEXTAVAILABILITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_CONTEXTAVAILABILITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_CONTROLDOCK(pub i32);
pub const UI_CONTROLDOCK_TOP: UI_CONTROLDOCK = UI_CONTROLDOCK(1i32);
pub const UI_CONTROLDOCK_BOTTOM: UI_CONTROLDOCK = UI_CONTROLDOCK(3i32);
impl ::std::convert::From<i32> for UI_CONTROLDOCK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_CONTROLDOCK {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_EVENTLOCATION(pub i32);
pub const UI_EVENTLOCATION_Ribbon: UI_EVENTLOCATION = UI_EVENTLOCATION(0i32);
pub const UI_EVENTLOCATION_QAT: UI_EVENTLOCATION = UI_EVENTLOCATION(1i32);
pub const UI_EVENTLOCATION_ApplicationMenu: UI_EVENTLOCATION = UI_EVENTLOCATION(2i32);
pub const UI_EVENTLOCATION_ContextPopup: UI_EVENTLOCATION = UI_EVENTLOCATION(3i32);
impl ::std::convert::From<i32> for UI_EVENTLOCATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_EVENTLOCATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`*"]
pub struct UI_EVENTPARAMS {
    pub EventType: UI_EVENTTYPE,
    pub Anonymous: UI_EVENTPARAMS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl UI_EVENTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for UI_EVENTPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for UI_EVENTPARAMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for UI_EVENTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for UI_EVENTPARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`*"]
pub union UI_EVENTPARAMS_0 {
    pub Modes: i32,
    pub Params: UI_EVENTPARAMS_COMMAND,
}
#[cfg(feature = "Win32_Foundation")]
impl UI_EVENTPARAMS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for UI_EVENTPARAMS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for UI_EVENTPARAMS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for UI_EVENTPARAMS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for UI_EVENTPARAMS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Ribbon`, `Win32_Foundation`*"]
pub struct UI_EVENTPARAMS_COMMAND {
    pub CommandID: u32,
    pub CommandName: super::super::Foundation::PWSTR,
    pub ParentCommandID: u32,
    pub ParentCommandName: super::super::Foundation::PWSTR,
    pub SelectionIndex: u32,
    pub Location: UI_EVENTLOCATION,
}
#[cfg(feature = "Win32_Foundation")]
impl UI_EVENTPARAMS_COMMAND {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for UI_EVENTPARAMS_COMMAND {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for UI_EVENTPARAMS_COMMAND {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UI_EVENTPARAMS_COMMAND")
            .field("CommandID", &self.CommandID)
            .field("CommandName", &self.CommandName)
            .field("ParentCommandID", &self.ParentCommandID)
            .field("ParentCommandName", &self.ParentCommandName)
            .field("SelectionIndex", &self.SelectionIndex)
            .field("Location", &self.Location)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for UI_EVENTPARAMS_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.CommandID == other.CommandID && self.CommandName == other.CommandName && self.ParentCommandID == other.ParentCommandID && self.ParentCommandName == other.ParentCommandName && self.SelectionIndex == other.SelectionIndex && self.Location == other.Location
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for UI_EVENTPARAMS_COMMAND {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for UI_EVENTPARAMS_COMMAND {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_EVENTTYPE(pub i32);
pub const UI_EVENTTYPE_ApplicationMenuOpened: UI_EVENTTYPE = UI_EVENTTYPE(0i32);
pub const UI_EVENTTYPE_RibbonMinimized: UI_EVENTTYPE = UI_EVENTTYPE(1i32);
pub const UI_EVENTTYPE_RibbonExpanded: UI_EVENTTYPE = UI_EVENTTYPE(2i32);
pub const UI_EVENTTYPE_ApplicationModeSwitched: UI_EVENTTYPE = UI_EVENTTYPE(3i32);
pub const UI_EVENTTYPE_TabActivated: UI_EVENTTYPE = UI_EVENTTYPE(4i32);
pub const UI_EVENTTYPE_MenuOpened: UI_EVENTTYPE = UI_EVENTTYPE(5i32);
pub const UI_EVENTTYPE_CommandExecuted: UI_EVENTTYPE = UI_EVENTTYPE(6i32);
pub const UI_EVENTTYPE_TooltipShown: UI_EVENTTYPE = UI_EVENTTYPE(7i32);
impl ::std::convert::From<i32> for UI_EVENTTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_EVENTTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_EXECUTIONVERB(pub i32);
pub const UI_EXECUTIONVERB_EXECUTE: UI_EXECUTIONVERB = UI_EXECUTIONVERB(0i32);
pub const UI_EXECUTIONVERB_PREVIEW: UI_EXECUTIONVERB = UI_EXECUTIONVERB(1i32);
pub const UI_EXECUTIONVERB_CANCELPREVIEW: UI_EXECUTIONVERB = UI_EXECUTIONVERB(2i32);
impl ::std::convert::From<i32> for UI_EXECUTIONVERB {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_EXECUTIONVERB {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_FONTDELTASIZE(pub i32);
pub const UI_FONTDELTASIZE_GROW: UI_FONTDELTASIZE = UI_FONTDELTASIZE(0i32);
pub const UI_FONTDELTASIZE_SHRINK: UI_FONTDELTASIZE = UI_FONTDELTASIZE(1i32);
impl ::std::convert::From<i32> for UI_FONTDELTASIZE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_FONTDELTASIZE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_FONTPROPERTIES(pub i32);
pub const UI_FONTPROPERTIES_NOTAVAILABLE: UI_FONTPROPERTIES = UI_FONTPROPERTIES(0i32);
pub const UI_FONTPROPERTIES_NOTSET: UI_FONTPROPERTIES = UI_FONTPROPERTIES(1i32);
pub const UI_FONTPROPERTIES_SET: UI_FONTPROPERTIES = UI_FONTPROPERTIES(2i32);
impl ::std::convert::From<i32> for UI_FONTPROPERTIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_FONTPROPERTIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_FONTUNDERLINE(pub i32);
pub const UI_FONTUNDERLINE_NOTAVAILABLE: UI_FONTUNDERLINE = UI_FONTUNDERLINE(0i32);
pub const UI_FONTUNDERLINE_NOTSET: UI_FONTUNDERLINE = UI_FONTUNDERLINE(1i32);
pub const UI_FONTUNDERLINE_SET: UI_FONTUNDERLINE = UI_FONTUNDERLINE(2i32);
impl ::std::convert::From<i32> for UI_FONTUNDERLINE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_FONTUNDERLINE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_FONTVERTICALPOSITION(pub i32);
pub const UI_FONTVERTICALPOSITION_NOTAVAILABLE: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(0i32);
pub const UI_FONTVERTICALPOSITION_NOTSET: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(1i32);
pub const UI_FONTVERTICALPOSITION_SUPERSCRIPT: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(2i32);
pub const UI_FONTVERTICALPOSITION_SUBSCRIPT: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(3i32);
impl ::std::convert::From<i32> for UI_FONTVERTICALPOSITION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_FONTVERTICALPOSITION {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_INVALIDATIONS(pub i32);
pub const UI_INVALIDATIONS_STATE: UI_INVALIDATIONS = UI_INVALIDATIONS(1i32);
pub const UI_INVALIDATIONS_VALUE: UI_INVALIDATIONS = UI_INVALIDATIONS(2i32);
pub const UI_INVALIDATIONS_PROPERTY: UI_INVALIDATIONS = UI_INVALIDATIONS(4i32);
pub const UI_INVALIDATIONS_ALLPROPERTIES: UI_INVALIDATIONS = UI_INVALIDATIONS(8i32);
impl ::std::convert::From<i32> for UI_INVALIDATIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_INVALIDATIONS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_OWNERSHIP(pub i32);
pub const UI_OWNERSHIP_TRANSFER: UI_OWNERSHIP = UI_OWNERSHIP(0i32);
pub const UI_OWNERSHIP_COPY: UI_OWNERSHIP = UI_OWNERSHIP(1i32);
impl ::std::convert::From<i32> for UI_OWNERSHIP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_OWNERSHIP {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_SWATCHCOLORMODE(pub i32);
pub const UI_SWATCHCOLORMODE_NORMAL: UI_SWATCHCOLORMODE = UI_SWATCHCOLORMODE(0i32);
pub const UI_SWATCHCOLORMODE_MONOCHROME: UI_SWATCHCOLORMODE = UI_SWATCHCOLORMODE(1i32);
impl ::std::convert::From<i32> for UI_SWATCHCOLORMODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_SWATCHCOLORMODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_SWATCHCOLORTYPE(pub i32);
pub const UI_SWATCHCOLORTYPE_NOCOLOR: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(0i32);
pub const UI_SWATCHCOLORTYPE_AUTOMATIC: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(1i32);
pub const UI_SWATCHCOLORTYPE_RGB: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(2i32);
impl ::std::convert::From<i32> for UI_SWATCHCOLORTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_SWATCHCOLORTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_VIEWTYPE(pub i32);
pub const UI_VIEWTYPE_RIBBON: UI_VIEWTYPE = UI_VIEWTYPE(1i32);
impl ::std::convert::From<i32> for UI_VIEWTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_VIEWTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_VIEWVERB(pub i32);
pub const UI_VIEWVERB_CREATE: UI_VIEWVERB = UI_VIEWVERB(0i32);
pub const UI_VIEWVERB_DESTROY: UI_VIEWVERB = UI_VIEWVERB(1i32);
pub const UI_VIEWVERB_SIZE: UI_VIEWVERB = UI_VIEWVERB(2i32);
pub const UI_VIEWVERB_ERROR: UI_VIEWVERB = UI_VIEWVERB(3i32);
impl ::std::convert::From<i32> for UI_VIEWVERB {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_VIEWVERB {
    type Abi = Self;
    type DefaultType = Self;
}
