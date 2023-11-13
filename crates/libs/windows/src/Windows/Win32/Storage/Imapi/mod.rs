#[inline]
pub unsafe fn CloseIMsgSession<P0>(lpmsgsess: P0)
where
    P0: ::windows_core::IntoParam<LPMSGSESS>,
{
    ::windows_targets::link!("mapi32.dll" "system" fn CloseIMsgSession(lpmsgsess : LPMSGSESS) -> ());
    CloseIMsgSession(lpmsgsess.into_param().abi())
}
#[doc = "Required features: `\"Win32_System_AddressBook\"`"]
#[cfg(feature = "Win32_System_AddressBook")]
#[inline]
pub unsafe fn GetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> ::windows_core::Result<()> {
    ::windows_targets::link!("mapi32.dll" "system" fn GetAttribIMsgOnIStg(lpobject : *mut ::core::ffi::c_void, lpproptagarray : *mut super::super::System::AddressBook:: SPropTagArray, lpppropattrarray : *mut *mut SPropAttrArray) -> ::windows_core::HRESULT);
    GetAttribIMsgOnIStg(lpobject, lpproptagarray, lpppropattrarray).ok()
}
#[inline]
pub unsafe fn MapStorageSCode(stgscode: i32) -> i32 {
    ::windows_targets::link!("mapi32.dll" "system" fn MapStorageSCode(stgscode : i32) -> i32);
    MapStorageSCode(stgscode)
}
#[doc = "Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OpenIMsgOnIStg<P0, P1, P2>(lpmsgsess: P0, lpallocatebuffer: super::super::System::AddressBook::LPALLOCATEBUFFER, lpallocatemore: super::super::System::AddressBook::LPALLOCATEMORE, lpfreebuffer: super::super::System::AddressBook::LPFREEBUFFER, lpmalloc: P1, lpmapisup: *mut ::core::ffi::c_void, lpstg: P2, lpfmsgcallrelease: *mut MSGCALLRELEASE, ulcallerdata: u32, ulflags: u32, lppmsg: *mut ::core::option::Option<super::super::System::AddressBook::IMessage>) -> i32
where
    P0: ::windows_core::IntoParam<LPMSGSESS>,
    P1: ::windows_core::IntoParam<super::super::System::Com::IMalloc>,
    P2: ::windows_core::IntoParam<super::super::System::Com::StructuredStorage::IStorage>,
{
    ::windows_targets::link!("mapi32.dll" "system" fn OpenIMsgOnIStg(lpmsgsess : LPMSGSESS, lpallocatebuffer : super::super::System::AddressBook:: LPALLOCATEBUFFER, lpallocatemore : super::super::System::AddressBook:: LPALLOCATEMORE, lpfreebuffer : super::super::System::AddressBook:: LPFREEBUFFER, lpmalloc : * mut::core::ffi::c_void, lpmapisup : *mut ::core::ffi::c_void, lpstg : * mut::core::ffi::c_void, lpfmsgcallrelease : *mut MSGCALLRELEASE, ulcallerdata : u32, ulflags : u32, lppmsg : *mut * mut::core::ffi::c_void) -> i32);
    OpenIMsgOnIStg(lpmsgsess.into_param().abi(), lpallocatebuffer, lpallocatemore, lpfreebuffer, lpmalloc.into_param().abi(), lpmapisup, lpstg.into_param().abi(), lpfmsgcallrelease, ulcallerdata, ulflags, ::core::mem::transmute(lppmsg))
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OpenIMsgSession<P0>(lpmalloc: P0, ulflags: u32, lppmsgsess: *mut LPMSGSESS) -> i32
where
    P0: ::windows_core::IntoParam<super::super::System::Com::IMalloc>,
{
    ::windows_targets::link!("mapi32.dll" "system" fn OpenIMsgSession(lpmalloc : * mut::core::ffi::c_void, ulflags : u32, lppmsgsess : *mut LPMSGSESS) -> i32);
    OpenIMsgSession(lpmalloc.into_param().abi(), ulflags, lppmsgsess)
}
#[doc = "Required features: `\"Win32_System_AddressBook\"`"]
#[cfg(feature = "Win32_System_AddressBook")]
#[inline]
pub unsafe fn SetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> ::windows_core::Result<()> {
    ::windows_targets::link!("mapi32.dll" "system" fn SetAttribIMsgOnIStg(lpobject : *mut ::core::ffi::c_void, lpproptags : *mut super::super::System::AddressBook:: SPropTagArray, lppropattrs : *mut SPropAttrArray, lpppropproblems : *mut *mut super::super::System::AddressBook:: SPropProblemArray) -> ::windows_core::HRESULT);
    SetAttribIMsgOnIStg(lpobject, lpproptags, lppropattrs, lpppropproblems).ok()
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DDiscFormat2DataEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2DataEvents {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self), object.into_param().abi(), progress.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(DDiscFormat2DataEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for DDiscFormat2DataEvents {
    type Vtable = DDiscFormat2DataEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for DDiscFormat2DataEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2735413c_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2DataEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DDiscFormat2EraseEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2EraseEvents {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0>(&self, object: P0, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self), object.into_param().abi(), elapsedseconds, estimatedtotalseconds).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(DDiscFormat2EraseEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for DDiscFormat2EraseEvents {
    type Vtable = DDiscFormat2EraseEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for DDiscFormat2EraseEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2735413a_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2EraseEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DDiscFormat2RawCDEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2RawCDEvents {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self), object.into_param().abi(), progress.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(DDiscFormat2RawCDEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for DDiscFormat2RawCDEvents {
    type Vtable = DDiscFormat2RawCDEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for DDiscFormat2RawCDEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354142_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2RawCDEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DDiscFormat2TrackAtOnceEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2TrackAtOnceEvents {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self), object.into_param().abi(), progress.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(DDiscFormat2TrackAtOnceEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for DDiscFormat2TrackAtOnceEvents {
    type Vtable = DDiscFormat2TrackAtOnceEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for DDiscFormat2TrackAtOnceEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2735413f_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DDiscMaster2Events(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DDiscMaster2Events {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NotifyDeviceAdded<P0, P1>(&self, object: P0, uniqueid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).NotifyDeviceAdded)(::windows_core::Interface::as_raw(self), object.into_param().abi(), uniqueid.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NotifyDeviceRemoved<P0, P1>(&self, object: P0, uniqueid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).NotifyDeviceRemoved)(::windows_core::Interface::as_raw(self), object.into_param().abi(), uniqueid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(DDiscMaster2Events, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for DDiscMaster2Events {
    type Vtable = DDiscMaster2Events_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for DDiscMaster2Events {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354131_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscMaster2Events_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub NotifyDeviceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, uniqueid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NotifyDeviceAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NotifyDeviceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, uniqueid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NotifyDeviceRemoved: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DFileSystemImageEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DFileSystemImageEvents {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0, P1>(&self, object: P0, currentfile: P1, copiedsectors: i32, totalsectors: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self), object.into_param().abi(), currentfile.into_param().abi(), copiedsectors, totalsectors).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(DFileSystemImageEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for DFileSystemImageEvents {
    type Vtable = DFileSystemImageEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for DFileSystemImageEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fdf_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, currentfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, copiedsectors: i32, totalsectors: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DFileSystemImageImportEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DFileSystemImageImportEvents {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateImport<P0, P1>(&self, object: P0, filesystem: FsiFileSystems, currentitem: P1, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).UpdateImport)(::windows_core::Interface::as_raw(self), object.into_param().abi(), filesystem, currentitem.into_param().abi(), importeddirectoryitems, totaldirectoryitems, importedfileitems, totalfileitems).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(DFileSystemImageImportEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for DFileSystemImageImportEvents {
    type Vtable = DFileSystemImageImportEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for DFileSystemImageImportEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd25c30f9_4087_4366_9e24_e55be286424b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageImportEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateImport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, currentitem: ::std::mem::MaybeUninit<::windows_core::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateImport: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DWriteEngine2Events(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DWriteEngine2Events {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self), object.into_param().abi(), progress.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(DWriteEngine2Events, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for DWriteEngine2Events {
    type Vtable = DWriteEngine2Events_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for DWriteEngine2Events {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354137_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DWriteEngine2Events_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBlockRange(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IBlockRange {
    pub unsafe fn StartLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EndLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IBlockRange, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IBlockRange {
    type Vtable = IBlockRange_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IBlockRange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb507ca25_2204_11dd_966a_001aa01bbc58);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRange_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub EndLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBlockRangeList(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IBlockRangeList {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockRanges(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BlockRanges)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IBlockRangeList, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IBlockRangeList {
    type Vtable = IBlockRangeList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IBlockRangeList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb507ca26_2204_11dd_966a_001aa01bbc58);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRangeList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BlockRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BlockRanges: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBootOptions(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IBootOptions {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImage(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BootImage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Manufacturer(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Manufacturer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetManufacturer<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetManufacturer)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn PlatformId(&self) -> ::windows_core::Result<PlatformId> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlatformId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlatformId(&self, newval: PlatformId) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlatformId)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn Emulation(&self) -> ::windows_core::Result<EmulationType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Emulation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEmulation(&self, newval: EmulationType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEmulation)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ImageSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ImageSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AssignBootImage<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).AssignBootImage)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IBootOptions, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IBootOptions {
    type Vtable = IBootOptions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IBootOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fd4_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBootOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImage: usize,
    pub Manufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PlatformId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut PlatformId) -> ::windows_core::HRESULT,
    pub SetPlatformId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: PlatformId) -> ::windows_core::HRESULT,
    pub Emulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut EmulationType) -> ::windows_core::HRESULT,
    pub SetEmulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: EmulationType) -> ::windows_core::HRESULT,
    pub ImageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AssignBootImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AssignBootImage: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBurnVerification(::windows_core::IUnknown);
impl IBurnVerification {
    pub unsafe fn SetBurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBurnVerificationLevel)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn BurnVerificationLevel(&self) -> ::windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BurnVerificationLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IBurnVerification, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBurnVerification {
    type Vtable = IBurnVerification_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBurnVerification {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2ffd834_958b_426d_8470_2a13879c6a91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBurnVerification_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetBurnVerificationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_core::HRESULT,
    pub BurnVerificationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscFormat2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2 {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsRecorderSupported<P0>(&self, recorder: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRecorderSupported)(::windows_core::Interface::as_raw(self), recorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsCurrentMediaSupported<P0>(&self, recorder: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsCurrentMediaSupported)(::windows_core::Interface::as_raw(self), recorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MediaPhysicallyBlank)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MediaHeuristicallyBlank)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedMediaTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDiscFormat2, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDiscFormat2 {
    type Vtable = IDiscFormat2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDiscFormat2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354152_8f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub IsRecorderSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recorder: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    IsRecorderSupported: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub IsCurrentMediaSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recorder: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    IsCurrentMediaSupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MediaPhysicallyBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MediaPhysicallyBlank: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MediaHeuristicallyBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MediaHeuristicallyBlank: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedMediaTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedMediaTypes: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscFormat2Data(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2Data {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsRecorderSupported<P0>(&self, recorder: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRecorderSupported)(::windows_core::Interface::as_raw(self), recorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsCurrentMediaSupported<P0>(&self, recorder: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsCurrentMediaSupported)(::windows_core::Interface::as_raw(self), recorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MediaPhysicallyBlank)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MediaHeuristicallyBlank)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupportedMediaTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        (::windows_core::Interface::vtable(self).SetRecorder)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> ::windows_core::Result<IDiscRecorder2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Recorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBufferUnderrunFreeDisabled<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostgapAlreadyInImage<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetPostgapAlreadyInImage)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostgapAlreadyInImage(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PostgapAlreadyInImage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentMediaStatus(&self) -> ::windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentMediaStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteProtectStatus(&self) -> ::windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).WriteProtectStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalSectorsOnMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FreeSectorsOnMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NextWritableAddress(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NextWritableAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartAddressOfPreviousSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastWrittenAddressOfPreviousSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetForceMediaToBeClosed<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetForceMediaToBeClosed)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ForceMediaToBeClosed(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ForceMediaToBeClosed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisableConsumerDvdCompatibilityMode<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetDisableConsumerDvdCompatibilityMode)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisableConsumerDvdCompatibilityMode(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DisableConsumerDvdCompatibilityMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClientName<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetClientName)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn ClientName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ClientName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RequestedWriteSpeed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentWriteSpeed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeeds(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedWriteSpeeds)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetForceOverwrite<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetForceOverwrite)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ForceOverwrite(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ForceOverwrite)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MultisessionInterfaces)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write<P0>(&self, data: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).Write)(::windows_core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn CancelWrite(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelWrite)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteSpeed<P0>(&self, requestedsectorspersecond: i32, rotationtypeispurecav: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetWriteSpeed)(::windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDiscFormat2Data, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDiscFormat2Data {
    type Vtable = IDiscFormat2Data_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDiscFormat2Data {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354153_9f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Data_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPostgapAlreadyInImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPostgapAlreadyInImage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostgapAlreadyInImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostgapAlreadyInImage: usize,
    pub CurrentMediaStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> ::windows_core::HRESULT,
    pub WriteProtectStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> ::windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetForceMediaToBeClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetForceMediaToBeClosed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ForceMediaToBeClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ForceMediaToBeClosed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisableConsumerDvdCompatibilityMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisableConsumerDvdCompatibilityMode: usize,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestedRotationTypeIsPureCAV: usize,
    pub CurrentWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentRotationTypeIsPureCAV: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetForceOverwrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetForceOverwrite: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ForceOverwrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ForceOverwrite: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MultisessionInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MultisessionInterfaces: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write: usize,
    pub CancelWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWriteSpeed: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscFormat2DataEventArgs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2DataEventArgs {
    pub unsafe fn StartLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.StartLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SectorCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SectorCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastReadLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastReadLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastWrittenLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastWrittenLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TotalSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UsedSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FreeSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ElapsedTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ElapsedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemainingTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemainingTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentAction(&self) -> ::windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentAction)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDiscFormat2DataEventArgs, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDiscFormat2DataEventArgs {
    type Vtable = IDiscFormat2DataEventArgs_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDiscFormat2DataEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2735413d_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2DataEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub ElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub TotalTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscFormat2Erase(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2Erase {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsRecorderSupported<P0>(&self, recorder: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRecorderSupported)(::windows_core::Interface::as_raw(self), recorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsCurrentMediaSupported<P0>(&self, recorder: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsCurrentMediaSupported)(::windows_core::Interface::as_raw(self), recorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MediaPhysicallyBlank)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MediaHeuristicallyBlank)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupportedMediaTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        (::windows_core::Interface::vtable(self).SetRecorder)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> ::windows_core::Result<IDiscRecorder2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Recorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullErase<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetFullErase)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FullErase(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FullErase)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClientName<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetClientName)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn ClientName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ClientName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EraseMedia(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EraseMedia)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDiscFormat2Erase, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDiscFormat2Erase {
    type Vtable = IDiscFormat2Erase_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDiscFormat2Erase {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354156_8f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Erase_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFullErase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFullErase: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FullErase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FullErase: usize,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EraseMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscFormat2RawCD(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2RawCD {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsRecorderSupported<P0>(&self, recorder: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRecorderSupported)(::windows_core::Interface::as_raw(self), recorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsCurrentMediaSupported<P0>(&self, recorder: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsCurrentMediaSupported)(::windows_core::Interface::as_raw(self), recorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MediaPhysicallyBlank)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MediaHeuristicallyBlank)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupportedMediaTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PrepareMedia(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrepareMedia)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteMedia<P0>(&self, data: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).WriteMedia)(::windows_core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteMedia2<P0>(&self, data: P0, streamleadinsectors: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).WriteMedia2)(::windows_core::Interface::as_raw(self), data.into_param().abi(), streamleadinsectors).ok()
    }
    pub unsafe fn CancelWrite(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelWrite)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseMedia(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseMedia)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteSpeed<P0>(&self, requestedsectorspersecond: i32, rotationtypeispurecav: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetWriteSpeed)(::windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        (::windows_core::Interface::vtable(self).SetRecorder)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> ::windows_core::Result<IDiscRecorder2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Recorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBufferUnderrunFreeDisabled<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartOfNextSession(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartOfNextSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastPossibleStartOfLeadout(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastPossibleStartOfLeadout)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedSectorTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedSectorTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRequestedSectorType)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn RequestedSectorType(&self) -> ::windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RequestedSectorType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClientName<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetClientName)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn ClientName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ClientName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RequestedWriteSpeed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentWriteSpeed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeeds(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedWriteSpeeds)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDiscFormat2RawCD, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDiscFormat2RawCD {
    type Vtable = IDiscFormat2RawCD_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDiscFormat2RawCD {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354155_8f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCD_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub PrepareMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteMedia: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteMedia2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, streamleadinsectors: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteMedia2: usize,
    pub CancelWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWriteSpeed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BufferUnderrunFreeDisabled: usize,
    pub StartOfNextSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub LastPossibleStartOfLeadout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedSectorTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedSectorTypes: usize,
    pub SetRequestedSectorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT,
    pub RequestedSectorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestedRotationTypeIsPureCAV: usize,
    pub CurrentWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentRotationTypeIsPureCAV: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscFormat2RawCDEventArgs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2RawCDEventArgs {
    pub unsafe fn StartLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.StartLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SectorCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SectorCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastReadLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastReadLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastWrittenLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastWrittenLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TotalSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UsedSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FreeSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentAction(&self) -> ::windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentAction)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ElapsedTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ElapsedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemainingTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemainingTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDiscFormat2RawCDEventArgs, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDiscFormat2RawCDEventArgs {
    type Vtable = IDiscFormat2RawCDEventArgs_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDiscFormat2RawCDEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354143_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCDEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub CurrentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> ::windows_core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscFormat2TrackAtOnce(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2TrackAtOnce {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsRecorderSupported<P0>(&self, recorder: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRecorderSupported)(::windows_core::Interface::as_raw(self), recorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsCurrentMediaSupported<P0>(&self, recorder: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsCurrentMediaSupported)(::windows_core::Interface::as_raw(self), recorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MediaPhysicallyBlank)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MediaHeuristicallyBlank)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupportedMediaTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PrepareMedia(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrepareMedia)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddAudioTrack<P0>(&self, data: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).AddAudioTrack)(::windows_core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn CancelAddTrack(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAddTrack)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseMedia(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseMedia)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteSpeed<P0>(&self, requestedsectorspersecond: i32, rotationtypeispurecav: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetWriteSpeed)(::windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        (::windows_core::Interface::vtable(self).SetRecorder)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> ::windows_core::Result<IDiscRecorder2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Recorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBufferUnderrunFreeDisabled<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NumberOfExistingTracks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NumberOfExistingTracks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalSectorsOnMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FreeSectorsOnMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UsedSectorsOnMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UsedSectorsOnMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDoNotFinalizeMedia<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetDoNotFinalizeMedia)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoNotFinalizeMedia(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DoNotFinalizeMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExpectedTableOfContents(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExpectedTableOfContents)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClientName<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetClientName)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn ClientName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ClientName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RequestedWriteSpeed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentWriteSpeed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeeds(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedWriteSpeeds)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDiscFormat2TrackAtOnce, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDiscFormat2TrackAtOnce {
    type Vtable = IDiscFormat2TrackAtOnce_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDiscFormat2TrackAtOnce {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354154_8f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnce_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub PrepareMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddAudioTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddAudioTrack: usize,
    pub CancelAddTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWriteSpeed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BufferUnderrunFreeDisabled: usize,
    pub NumberOfExistingTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub UsedSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDoNotFinalizeMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDoNotFinalizeMedia: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DoNotFinalizeMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoNotFinalizeMedia: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExpectedTableOfContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExpectedTableOfContents: usize,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestedRotationTypeIsPureCAV: usize,
    pub CurrentWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentRotationTypeIsPureCAV: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscFormat2TrackAtOnceEventArgs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2TrackAtOnceEventArgs {
    pub unsafe fn StartLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.StartLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SectorCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SectorCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastReadLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastReadLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastWrittenLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastWrittenLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TotalSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UsedSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FreeSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentTrackNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentTrackNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentAction(&self) -> ::windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentAction)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ElapsedTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ElapsedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemainingTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemainingTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDiscFormat2TrackAtOnceEventArgs, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDiscFormat2TrackAtOnceEventArgs {
    type Vtable = IDiscFormat2TrackAtOnceEventArgs_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDiscFormat2TrackAtOnceEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354140_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub CurrentTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> ::windows_core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscMaster(::windows_core::IUnknown);
impl IDiscMaster {
    pub unsafe fn Open(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumDiscMasterFormats(&self) -> ::windows_core::Result<IEnumDiscMasterFormats> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumDiscMasterFormats)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActiveDiscMasterFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetActiveDiscMasterFormat)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetActiveDiscMasterFormat(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActiveDiscMasterFormat)(::windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn EnumDiscRecorders(&self) -> ::windows_core::Result<IEnumDiscRecorders> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumDiscRecorders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActiveDiscRecorder(&self) -> ::windows_core::Result<IDiscRecorder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetActiveDiscRecorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetActiveDiscRecorder<P0>(&self, precorder: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder>,
    {
        (::windows_core::Interface::vtable(self).SetActiveDiscRecorder)(::windows_core::Interface::as_raw(self), precorder.into_param().abi()).ok()
    }
    pub unsafe fn ClearFormatContent(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClearFormatContent)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProgressAdvise<P0>(&self, pevents: P0) -> ::windows_core::Result<usize>
    where
        P0: ::windows_core::IntoParam<IDiscMasterProgressEvents>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProgressAdvise)(::windows_core::Interface::as_raw(self), pevents.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProgressUnadvise(&self, vcookie: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProgressUnadvise)(::windows_core::Interface::as_raw(self), vcookie).ok()
    }
    pub unsafe fn RecordDisc(&self, bsimulate: u8, bejectafterburn: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RecordDisc)(::windows_core::Interface::as_raw(self), bsimulate, bejectafterburn).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDiscMaster, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiscMaster {
    type Vtable = IDiscMaster_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDiscMaster {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x520cca62_51a5_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMaster_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumDiscMasterFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetActiveDiscMasterFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetActiveDiscMasterFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumDiscRecorders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetActiveDiscRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprecorder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetActiveDiscRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precorder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ClearFormatContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProgressAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void, pvcookie: *mut usize) -> ::windows_core::HRESULT,
    pub ProgressUnadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcookie: usize) -> ::windows_core::HRESULT,
    pub RecordDisc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsimulate: u8, bejectafterburn: u8) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscMaster2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscMaster2 {
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSupportedEnvironment(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsSupportedEnvironment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDiscMaster2, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDiscMaster2 {
    type Vtable = IDiscMaster2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDiscMaster2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354130_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMaster2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSupportedEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSupportedEnvironment: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscMasterProgressEvents(::windows_core::IUnknown);
impl IDiscMasterProgressEvents {
    pub unsafe fn QueryCancel(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryCancel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyPnPActivity(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyPnPActivity)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NotifyAddProgress(&self, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyAddProgress)(::windows_core::Interface::as_raw(self), ncompletedsteps, ntotalsteps).ok()
    }
    pub unsafe fn NotifyBlockProgress(&self, ncompleted: i32, ntotal: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyBlockProgress)(::windows_core::Interface::as_raw(self), ncompleted, ntotal).ok()
    }
    pub unsafe fn NotifyTrackProgress(&self, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyTrackProgress)(::windows_core::Interface::as_raw(self), ncurrenttrack, ntotaltracks).ok()
    }
    pub unsafe fn NotifyPreparingBurn(&self, nestimatedseconds: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyPreparingBurn)(::windows_core::Interface::as_raw(self), nestimatedseconds).ok()
    }
    pub unsafe fn NotifyClosingDisc(&self, nestimatedseconds: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyClosingDisc)(::windows_core::Interface::as_raw(self), nestimatedseconds).ok()
    }
    pub unsafe fn NotifyBurnComplete(&self, status: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyBurnComplete)(::windows_core::Interface::as_raw(self), status).ok()
    }
    pub unsafe fn NotifyEraseComplete(&self, status: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyEraseComplete)(::windows_core::Interface::as_raw(self), status).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDiscMasterProgressEvents, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiscMasterProgressEvents {
    type Vtable = IDiscMasterProgressEvents_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDiscMasterProgressEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec9e51c1_4e5d_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMasterProgressEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub QueryCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcancel: *mut u8) -> ::windows_core::HRESULT,
    pub NotifyPnPActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyAddProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows_core::HRESULT,
    pub NotifyBlockProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncompleted: i32, ntotal: i32) -> ::windows_core::HRESULT,
    pub NotifyTrackProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows_core::HRESULT,
    pub NotifyPreparingBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows_core::HRESULT,
    pub NotifyClosingDisc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows_core::HRESULT,
    pub NotifyBurnComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub NotifyEraseComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscRecorder(::windows_core::IUnknown);
impl IDiscRecorder {
    pub unsafe fn Init(&self, pbyuniqueid: &[u8], nuldrivenumber: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbyuniqueid.as_ptr()), pbyuniqueid.len().try_into().unwrap(), nuldrivenumber).ok()
    }
    pub unsafe fn GetRecorderGUID(&self, pbyuniqueid: ::core::option::Option<&mut [u8]>, pulreturnsizerequired: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRecorderGUID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbyuniqueid.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pbyuniqueid.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pulreturnsizerequired).ok()
    }
    pub unsafe fn GetRecorderType(&self) -> ::windows_core::Result<RECORDER_TYPES> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRecorderType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayNames(&self, pbstrvendorid: ::core::option::Option<*mut ::windows_core::BSTR>, pbstrproductid: ::core::option::Option<*mut ::windows_core::BSTR>, pbstrrevision: ::core::option::Option<*mut ::windows_core::BSTR>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDisplayNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrvendorid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbstrproductid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbstrrevision.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetBasePnPID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBasePnPID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetRecorderProperties(&self) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRecorderProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetRecorderProperties<P0>(&self, ppropstg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::StructuredStorage::IPropertyStorage>,
    {
        (::windows_core::Interface::vtable(self).SetRecorderProperties)(::windows_core::Interface::as_raw(self), ppropstg.into_param().abi()).ok()
    }
    pub unsafe fn GetRecorderState(&self) -> ::windows_core::Result<DISC_RECORDER_STATE_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRecorderState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenExclusive(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenExclusive)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryMediaType(&self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryMediaType)(::windows_core::Interface::as_raw(self), fmediatype, fmediaflags).ok()
    }
    pub unsafe fn QueryMediaInfo(&self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryMediaInfo)(::windows_core::Interface::as_raw(self), pbsessions, pblasttrack, ulstartaddress, ulnextwritable, ulfreeblocks).ok()
    }
    pub unsafe fn Eject(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Eject)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Erase(&self, bfullerase: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Erase)(::windows_core::Interface::as_raw(self), bfullerase).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDiscRecorder, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiscRecorder {
    type Vtable = IDiscRecorder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDiscRecorder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85ac9776_ca88_4cf2_894e_09598c078a41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows_core::HRESULT,
    pub GetRecorderGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows_core::HRESULT,
    pub GetRecorderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ftypecode: *mut RECORDER_TYPES) -> ::windows_core::HRESULT,
    pub GetDisplayNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrproductid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrrevision: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetBasePnPID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbasepnpid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetRecorderProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetRecorderProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetRecorderProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetRecorderProperties: usize,
    pub GetRecorderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> ::windows_core::HRESULT,
    pub OpenExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QueryMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows_core::HRESULT,
    pub QueryMediaInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows_core::HRESULT,
    pub Eject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Erase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfullerase: u8) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscRecorder2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscRecorder2 {
    pub unsafe fn EjectMedia(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EjectMedia)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CloseTray(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CloseTray)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AcquireExclusiveAccess<P0, P1>(&self, force: P0, __midl__idiscrecorder20000: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).AcquireExclusiveAccess)(::windows_core::Interface::as_raw(self), force.into_param().abi(), __midl__idiscrecorder20000.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseExclusiveAccess(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseExclusiveAccess)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableMcn(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableMcn)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableMcn(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableMcn)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InitializeDiscRecorder<P0>(&self, recorderuniqueid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).InitializeDiscRecorder)(::windows_core::Interface::as_raw(self), recorderuniqueid.into_param().abi()).ok()
    }
    pub unsafe fn ActiveDiscRecorder(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ActiveDiscRecorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VendorId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).VendorId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProductId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductRevision(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProductRevision)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).VolumeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn VolumePathNames(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).VolumePathNames)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceCanLoadMedia(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceCanLoadMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LegacyDeviceNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LegacyDeviceNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedFeaturePages(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedFeaturePages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentFeaturePages(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentFeaturePages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedProfiles(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedProfiles)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentProfiles(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentProfiles)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedModePages(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportedModePages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExclusiveAccessOwner(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExclusiveAccessOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDiscRecorder2, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDiscRecorder2 {
    type Vtable = IDiscRecorder2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDiscRecorder2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354133_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EjectMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CloseTray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AcquireExclusiveAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, force: super::super::Foundation::VARIANT_BOOL, __midl__idiscrecorder20000: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AcquireExclusiveAccess: usize,
    pub ReleaseExclusiveAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableMcn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableMcn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InitializeDiscRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recorderuniqueid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ActiveDiscRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub VendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ProductRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub VolumePathNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    VolumePathNames: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceCanLoadMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceCanLoadMedia: usize,
    pub LegacyDeviceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, legacydevicenumber: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedFeaturePages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedFeaturePages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentFeaturePages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentFeaturePages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedProfiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentProfiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedModePages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedModePages: usize,
    pub ExclusiveAccessOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDiscRecorder2Ex(::windows_core::IUnknown);
impl IDiscRecorder2Ex {
    pub unsafe fn SendCommandNoData(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendCommandNoData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cdb.as_ptr()), cdb.len().try_into().unwrap(), ::core::mem::transmute(sensebuffer.as_ptr()), timeout).ok()
    }
    pub unsafe fn SendCommandSendDataToDevice(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32, buffer: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendCommandSendDataToDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cdb.as_ptr()), cdb.len().try_into().unwrap(), ::core::mem::transmute(sensebuffer.as_ptr()), timeout, ::core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SendCommandGetDataFromDevice(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32, buffer: &mut [u8], bufferfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendCommandGetDataFromDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cdb.as_ptr()), cdb.len().try_into().unwrap(), ::core::mem::transmute(sensebuffer.as_ptr()), timeout, ::core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), bufferfetched).ok()
    }
    pub unsafe fn ReadDvdStructure(&self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReadDvdStructure)(::windows_core::Interface::as_raw(self), format, address, layer, agid, data, count).ok()
    }
    pub unsafe fn SendDvdStructure(&self, format: u32, data: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendDvdStructure)(::windows_core::Interface::as_raw(self), format, ::core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetAdapterDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAdapterDescriptor)(::windows_core::Interface::as_raw(self), data, bytesize).ok()
    }
    pub unsafe fn GetDeviceDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceDescriptor)(::windows_core::Interface::as_raw(self), data, bytesize).ok()
    }
    pub unsafe fn GetDiscInformation(&self, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDiscInformation)(::windows_core::Interface::as_raw(self), discinformation, bytesize).ok()
    }
    pub unsafe fn GetTrackInformation(&self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTrackInformation)(::windows_core::Interface::as_raw(self), address, addresstype, trackinformation, bytesize).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFeaturePage<P0>(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: P0, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOLEAN>,
    {
        (::windows_core::Interface::vtable(self).GetFeaturePage)(::windows_core::Interface::as_raw(self), requestedfeature, currentfeatureonly.into_param().abi(), featuredata, bytesize).ok()
    }
    pub unsafe fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetModePage)(::windows_core::Interface::as_raw(self), requestedmodepage, requesttype, modepagedata, bytesize).ok()
    }
    pub unsafe fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetModePage)(::windows_core::Interface::as_raw(self), requesttype, ::core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSupportedFeaturePages<P0>(&self, currentfeatureonly: P0, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOLEAN>,
    {
        (::windows_core::Interface::vtable(self).GetSupportedFeaturePages)(::windows_core::Interface::as_raw(self), currentfeatureonly.into_param().abi(), featuredata, bytesize).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSupportedProfiles<P0>(&self, currentonly: P0, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOLEAN>,
    {
        (::windows_core::Interface::vtable(self).GetSupportedProfiles)(::windows_core::Interface::as_raw(self), currentonly.into_param().abi(), profiletypes, validprofiles).ok()
    }
    pub unsafe fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSupportedModePages)(::windows_core::Interface::as_raw(self), requesttype, modepagetypes, validpages).ok()
    }
    pub unsafe fn GetByteAlignmentMask(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetByteAlignmentMask)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaximumNonPageAlignedTransferSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMaximumNonPageAlignedTransferSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaximumPageAlignedTransferSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMaximumPageAlignedTransferSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDiscRecorder2Ex, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiscRecorder2Ex {
    type Vtable = IDiscRecorder2Ex_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDiscRecorder2Ex {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354132_7f64_5b0f_8f00_5d77afbe261e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2Ex_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SendCommandNoData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows_core::HRESULT,
    pub SendCommandSendDataToDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows_core::HRESULT,
    pub SendCommandGetDataFromDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows_core::HRESULT,
    pub ReadDvdStructure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT,
    pub SendDvdStructure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: u32, data: *const u8, count: u32) -> ::windows_core::HRESULT,
    pub GetAdapterDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT,
    pub GetDeviceDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT,
    pub GetDiscInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT,
    pub GetTrackInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFeaturePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFeaturePage: usize,
    pub GetModePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT,
    pub SetModePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSupportedFeaturePages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSupportedFeaturePages: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSupportedProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSupportedProfiles: usize,
    pub GetSupportedModePages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows_core::HRESULT,
    pub GetByteAlignmentMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT,
    pub GetMaximumNonPageAlignedTransferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT,
    pub GetMaximumPageAlignedTransferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumDiscMasterFormats(::windows_core::IUnknown);
impl IEnumDiscMasterFormats {
    pub unsafe fn Next(&self, lpiidformatid: &mut [::windows_core::GUID], pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), lpiidformatid.len().try_into().unwrap(), ::core::mem::transmute(lpiidformatid.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cformats: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cformats).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumDiscMasterFormats> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumDiscMasterFormats, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumDiscMasterFormats {
    type Vtable = IEnumDiscMasterFormats_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumDiscMasterFormats {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddf445e1_54ba_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDiscMasterFormats_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cformats: u32, lpiidformatid: *mut ::windows_core::GUID, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cformats: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumDiscRecorders(::windows_core::IUnknown);
impl IEnumDiscRecorders {
    pub unsafe fn Next(&self, pprecorder: &mut [::core::option::Option<IDiscRecorder>], pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), pprecorder.len().try_into().unwrap(), ::core::mem::transmute(pprecorder.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, crecorders: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), crecorders).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumDiscRecorders> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumDiscRecorders, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumDiscRecorders {
    type Vtable = IEnumDiscRecorders_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumDiscRecorders {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b1921e1_54ac_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDiscRecorders_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crecorders: u32, pprecorder: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crecorders: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumFsiItems(::windows_core::IUnknown);
impl IEnumFsiItems {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IFsiItem>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumFsiItems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumFsiItems, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumFsiItems {
    type Vtable = IEnumFsiItems_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumFsiItems {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fda_975b_59be_a960_9a2a262853a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFsiItems_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumProgressItems(::windows_core::IUnknown);
impl IEnumProgressItems {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IProgressItem>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumProgressItems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumProgressItems, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumProgressItems {
    type Vtable = IEnumProgressItems_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumProgressItems {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fd6_975b_59be_a960_9a2a262853a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumProgressItems_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSystemImage(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Root(&self) -> ::windows_core::Result<IFsiDirectoryItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Root)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionStartBlock(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionStartBlock)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSessionStartBlock)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FreeMediaBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FreeMediaBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFreeMediaBlocks)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMaxMediaBlocksFromDevice<P0>(&self, discrecorder: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        (::windows_core::Interface::vtable(self).SetMaxMediaBlocksFromDevice)(::windows_core::Interface::as_raw(self), discrecorder.into_param().abi()).ok()
    }
    pub unsafe fn UsedBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UsedBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).VolumeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVolumeName<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetVolumeName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn ImportedVolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ImportedVolumeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptions(&self) -> ::windows_core::Result<IBootOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BootImageOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptions<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IBootOptions>,
    {
        (::windows_core::Interface::vtable(self).SetBootImageOptions)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FileCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DirectoryCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DirectoryCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WorkingDirectory(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).WorkingDirectory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWorkingDirectory<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetWorkingDirectory)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn ChangePoint(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ChangePoint)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StrictFileSystemCompliance(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StrictFileSystemCompliance)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrictFileSystemCompliance<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetStrictFileSystemCompliance)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseRestrictedCharacterSet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UseRestrictedCharacterSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseRestrictedCharacterSet<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseRestrictedCharacterSet)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileSystemsToCreate(&self) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FileSystemsToCreate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFileSystemsToCreate)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FileSystemsSupported(&self) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FileSystemsSupported)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUDFRevision)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn UDFRevision(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UDFRevision)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UDFRevisionsSupported(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UDFRevisionsSupported)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ChooseImageDefaults<P0>(&self, discrecorder: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        (::windows_core::Interface::vtable(self).ChooseImageDefaults)(::windows_core::Interface::as_raw(self), discrecorder.into_param().abi()).ok()
    }
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChooseImageDefaultsForMediaType)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetISO9660InterchangeLevel)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ISO9660InterchangeLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ISO9660InterchangeLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ISO9660InterchangeLevelsSupported)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultImage(&self) -> ::windows_core::Result<IFileSystemImageResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateResultImage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Exists<P0>(&self, fullpath: P0) -> ::windows_core::Result<FsiItemType>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Exists)(::windows_core::Interface::as_raw(self), fullpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CalculateDiscIdentifier(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CalculateDiscIdentifier)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IdentifyFileSystemsOnDisc<P0>(&self, discrecorder: P0) -> ::windows_core::Result<FsiFileSystems>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IdentifyFileSystemsOnDisc)(::windows_core::Interface::as_raw(self), discrecorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultFileSystemForImport)(::windows_core::Interface::as_raw(self), filesystems, &mut result__).from_abi(result__)
    }
    pub unsafe fn ImportFileSystem(&self) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ImportFileSystem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ImportSpecificFileSystem)(::windows_core::Interface::as_raw(self), filesystemtouse).ok()
    }
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RollbackToChangePoint)(::windows_core::Interface::as_raw(self), changepoint).ok()
    }
    pub unsafe fn LockInChangePoint(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockInChangePoint)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDirectoryItem<P0>(&self, name: P0) -> ::windows_core::Result<IFsiDirectoryItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDirectoryItem)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileItem<P0>(&self, name: P0) -> ::windows_core::Result<IFsiFileItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateFileItem)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeNameUDF(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).VolumeNameUDF)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeNameJoliet(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).VolumeNameJoliet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeNameISO9660(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).VolumeNameISO9660)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StageFiles(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StageFiles)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStageFiles<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetStageFiles)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MultisessionInterfaces)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMultisessionInterfaces)(::windows_core::Interface::as_raw(self), newval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFileSystemImage, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFileSystemImage {
    type Vtable = IFileSystemImage_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFileSystemImage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fe1_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Root: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Root: usize,
    pub SessionStartBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetSessionStartBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub FreeMediaBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetFreeMediaBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMaxMediaBlocksFromDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMaxMediaBlocksFromDevice: usize,
    pub UsedBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetVolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ImportedVolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImageOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImageOptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBootImageOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBootImageOptions: usize,
    pub FileCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub DirectoryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ChangePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StrictFileSystemCompliance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StrictFileSystemCompliance: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStrictFileSystemCompliance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStrictFileSystemCompliance: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UseRestrictedCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseRestrictedCharacterSet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseRestrictedCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseRestrictedCharacterSet: usize,
    pub FileSystemsToCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows_core::HRESULT,
    pub SetFileSystemsToCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsiFileSystems) -> ::windows_core::HRESULT,
    pub FileSystemsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows_core::HRESULT,
    pub SetUDFRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub UDFRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UDFRevisionsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UDFRevisionsSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ChooseImageDefaults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ChooseImageDefaults: usize,
    pub ChooseImageDefaultsForMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT,
    pub SetISO9660InterchangeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub ISO9660InterchangeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ISO9660InterchangeLevelsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ISO9660InterchangeLevelsSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResultImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResultImage: usize,
    pub Exists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fullpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, itemtype: *mut FsiItemType) -> ::windows_core::HRESULT,
    pub CalculateDiscIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IdentifyFileSystemsOnDisc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void, filesystems: *mut FsiFileSystems) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IdentifyFileSystemsOnDisc: usize,
    pub GetDefaultFileSystemForImport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows_core::HRESULT,
    pub ImportFileSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> ::windows_core::HRESULT,
    pub ImportSpecificFileSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesystemtouse: FsiFileSystems) -> ::windows_core::HRESULT,
    pub RollbackToChangePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changepoint: i32) -> ::windows_core::HRESULT,
    pub LockInChangePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDirectoryItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, newitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDirectoryItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, newitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileItem: usize,
    pub VolumeNameUDF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub VolumeNameJoliet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub VolumeNameISO9660: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StageFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StageFiles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStageFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStageFiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MultisessionInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MultisessionInterfaces: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMultisessionInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMultisessionInterfaces: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSystemImage2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage2 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Root(&self) -> ::windows_core::Result<IFsiDirectoryItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Root)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionStartBlock(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionStartBlock)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSessionStartBlock)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FreeMediaBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FreeMediaBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFreeMediaBlocks)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMaxMediaBlocksFromDevice<P0>(&self, discrecorder: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        (::windows_core::Interface::vtable(self).base__.SetMaxMediaBlocksFromDevice)(::windows_core::Interface::as_raw(self), discrecorder.into_param().abi()).ok()
    }
    pub unsafe fn UsedBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UsedBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.VolumeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVolumeName<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetVolumeName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn ImportedVolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ImportedVolumeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptions(&self) -> ::windows_core::Result<IBootOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.BootImageOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptions<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IBootOptions>,
    {
        (::windows_core::Interface::vtable(self).base__.SetBootImageOptions)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FileCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DirectoryCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DirectoryCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WorkingDirectory(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.WorkingDirectory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWorkingDirectory<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetWorkingDirectory)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn ChangePoint(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ChangePoint)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StrictFileSystemCompliance(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.StrictFileSystemCompliance)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrictFileSystemCompliance<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetStrictFileSystemCompliance)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseRestrictedCharacterSet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UseRestrictedCharacterSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseRestrictedCharacterSet<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetUseRestrictedCharacterSet)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileSystemsToCreate(&self) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FileSystemsToCreate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFileSystemsToCreate)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FileSystemsSupported(&self) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FileSystemsSupported)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetUDFRevision)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn UDFRevision(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UDFRevision)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UDFRevisionsSupported(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UDFRevisionsSupported)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ChooseImageDefaults<P0>(&self, discrecorder: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        (::windows_core::Interface::vtable(self).base__.ChooseImageDefaults)(::windows_core::Interface::as_raw(self), discrecorder.into_param().abi()).ok()
    }
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ChooseImageDefaultsForMediaType)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetISO9660InterchangeLevel)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ISO9660InterchangeLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ISO9660InterchangeLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ISO9660InterchangeLevelsSupported)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultImage(&self) -> ::windows_core::Result<IFileSystemImageResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateResultImage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Exists<P0>(&self, fullpath: P0) -> ::windows_core::Result<FsiItemType>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Exists)(::windows_core::Interface::as_raw(self), fullpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CalculateDiscIdentifier(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CalculateDiscIdentifier)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IdentifyFileSystemsOnDisc<P0>(&self, discrecorder: P0) -> ::windows_core::Result<FsiFileSystems>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IdentifyFileSystemsOnDisc)(::windows_core::Interface::as_raw(self), discrecorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultFileSystemForImport)(::windows_core::Interface::as_raw(self), filesystems, &mut result__).from_abi(result__)
    }
    pub unsafe fn ImportFileSystem(&self) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ImportFileSystem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ImportSpecificFileSystem)(::windows_core::Interface::as_raw(self), filesystemtouse).ok()
    }
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RollbackToChangePoint)(::windows_core::Interface::as_raw(self), changepoint).ok()
    }
    pub unsafe fn LockInChangePoint(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LockInChangePoint)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDirectoryItem<P0>(&self, name: P0) -> ::windows_core::Result<IFsiDirectoryItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateDirectoryItem)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileItem<P0>(&self, name: P0) -> ::windows_core::Result<IFsiFileItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateFileItem)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeNameUDF(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.VolumeNameUDF)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeNameJoliet(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.VolumeNameJoliet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeNameISO9660(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.VolumeNameISO9660)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StageFiles(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.StageFiles)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStageFiles<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetStageFiles)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MultisessionInterfaces)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMultisessionInterfaces)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptionsArray(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BootImageOptionsArray)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBootImageOptionsArray)(::windows_core::Interface::as_raw(self), newval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFileSystemImage2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFileSystemImage);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFileSystemImage2 {
    type Vtable = IFileSystemImage2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFileSystemImage2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7644b2c_1537_4767_b62f_f1387b02ddfd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage2_Vtbl {
    pub base__: IFileSystemImage_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImageOptionsArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImageOptionsArray: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBootImageOptionsArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBootImageOptionsArray: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSystemImage3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage3 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Root(&self) -> ::windows_core::Result<IFsiDirectoryItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Root)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionStartBlock(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionStartBlock)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetSessionStartBlock)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FreeMediaBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FreeMediaBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetFreeMediaBlocks)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMaxMediaBlocksFromDevice<P0>(&self, discrecorder: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetMaxMediaBlocksFromDevice)(::windows_core::Interface::as_raw(self), discrecorder.into_param().abi()).ok()
    }
    pub unsafe fn UsedBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UsedBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.VolumeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVolumeName<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetVolumeName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn ImportedVolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ImportedVolumeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptions(&self) -> ::windows_core::Result<IBootOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.BootImageOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptions<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IBootOptions>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetBootImageOptions)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FileCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DirectoryCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DirectoryCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WorkingDirectory(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.WorkingDirectory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWorkingDirectory<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetWorkingDirectory)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn ChangePoint(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ChangePoint)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StrictFileSystemCompliance(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.StrictFileSystemCompliance)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrictFileSystemCompliance<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetStrictFileSystemCompliance)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseRestrictedCharacterSet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UseRestrictedCharacterSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseRestrictedCharacterSet<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetUseRestrictedCharacterSet)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileSystemsToCreate(&self) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FileSystemsToCreate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetFileSystemsToCreate)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FileSystemsSupported(&self) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FileSystemsSupported)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetUDFRevision)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn UDFRevision(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UDFRevision)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UDFRevisionsSupported(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UDFRevisionsSupported)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ChooseImageDefaults<P0>(&self, discrecorder: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ChooseImageDefaults)(::windows_core::Interface::as_raw(self), discrecorder.into_param().abi()).ok()
    }
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ChooseImageDefaultsForMediaType)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetISO9660InterchangeLevel)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ISO9660InterchangeLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ISO9660InterchangeLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ISO9660InterchangeLevelsSupported)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultImage(&self) -> ::windows_core::Result<IFileSystemImageResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateResultImage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Exists<P0>(&self, fullpath: P0) -> ::windows_core::Result<FsiItemType>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Exists)(::windows_core::Interface::as_raw(self), fullpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CalculateDiscIdentifier(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CalculateDiscIdentifier)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IdentifyFileSystemsOnDisc<P0>(&self, discrecorder: P0) -> ::windows_core::Result<FsiFileSystems>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IdentifyFileSystemsOnDisc)(::windows_core::Interface::as_raw(self), discrecorder.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetDefaultFileSystemForImport)(::windows_core::Interface::as_raw(self), filesystems, &mut result__).from_abi(result__)
    }
    pub unsafe fn ImportFileSystem(&self) -> ::windows_core::Result<FsiFileSystems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ImportFileSystem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ImportSpecificFileSystem)(::windows_core::Interface::as_raw(self), filesystemtouse).ok()
    }
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RollbackToChangePoint)(::windows_core::Interface::as_raw(self), changepoint).ok()
    }
    pub unsafe fn LockInChangePoint(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.LockInChangePoint)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDirectoryItem<P0>(&self, name: P0) -> ::windows_core::Result<IFsiDirectoryItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateDirectoryItem)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileItem<P0>(&self, name: P0) -> ::windows_core::Result<IFsiFileItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateFileItem)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeNameUDF(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.VolumeNameUDF)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeNameJoliet(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.VolumeNameJoliet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VolumeNameISO9660(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.VolumeNameISO9660)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StageFiles(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.StageFiles)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStageFiles<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetStageFiles)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MultisessionInterfaces)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetMultisessionInterfaces)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptionsArray(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.BootImageOptionsArray)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBootImageOptionsArray)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRedundantUdfMetadataFiles(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateRedundantUdfMetadataFiles)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCreateRedundantUdfMetadataFiles<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetCreateRedundantUdfMetadataFiles)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProbeSpecificFileSystem)(::windows_core::Interface::as_raw(self), filesystemtoprobe, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFileSystemImage3, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFileSystemImage, IFileSystemImage2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFileSystemImage3 {
    type Vtable = IFileSystemImage3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFileSystemImage3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cff842c_7e97_4807_8304_910dd8f7c051);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage3_Vtbl {
    pub base__: IFileSystemImage2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateRedundantUdfMetadataFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateRedundantUdfMetadataFiles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCreateRedundantUdfMetadataFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCreateRedundantUdfMetadataFiles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProbeSpecificFileSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProbeSpecificFileSystem: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSystemImageResult(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageResult {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImageStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ImageStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProgressItems(&self) -> ::windows_core::Result<IProgressItems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProgressItems)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BlockSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BlockSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DiscId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DiscId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFileSystemImageResult, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFileSystemImageResult {
    type Vtable = IFileSystemImageResult_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFileSystemImageResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fd8_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ImageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImageStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ProgressItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProgressItems: usize,
    pub TotalBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub BlockSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub DiscId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSystemImageResult2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageResult2 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImageStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ImageStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProgressItems(&self) -> ::windows_core::Result<IProgressItems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProgressItems)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TotalBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BlockSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.BlockSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DiscId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DiscId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModifiedBlocks(&self) -> ::windows_core::Result<IBlockRangeList> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ModifiedBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFileSystemImageResult2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFileSystemImageResult);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFileSystemImageResult2 {
    type Vtable = IFileSystemImageResult2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFileSystemImageResult2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb507ca29_2204_11dd_966a_001aa01bbc58);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult2_Vtbl {
    pub base__: IFileSystemImageResult_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ModifiedBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModifiedBlocks: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFsiDirectoryItem(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiDirectoryItem {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FullPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FullPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCreationTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastAccessedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastAccessedTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastModifiedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastModifiedTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsHidden)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHidden<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetIsHidden)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FileSystemName)(::windows_core::Interface::as_raw(self), filesystem, &mut result__).from_abi(result__)
    }
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FileSystemPath)(::windows_core::Interface::as_raw(self), filesystem, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, path: P0) -> ::windows_core::Result<IFsiItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), path.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumFsiItems(&self) -> ::windows_core::Result<IEnumFsiItems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumFsiItems)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddDirectory<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).AddDirectory)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddFile<P0, P1>(&self, path: P0, filedata: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).AddFile)(::windows_core::Interface::as_raw(self), path.into_param().abi(), filedata.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddTree<P0, P1>(&self, sourcedirectory: P0, includebasedirectory: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).AddTree)(::windows_core::Interface::as_raw(self), sourcedirectory.into_param().abi(), includebasedirectory.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, item: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFsiItem>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), item.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn RemoveTree<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveTree)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFsiDirectoryItem, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFsiDirectoryItem {
    type Vtable = IFsiDirectoryItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFsiDirectoryItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fdc_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub EnumFsiItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, filedata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddTree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcedirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddTree: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RemoveTree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFsiDirectoryItem2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiDirectoryItem2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FullPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FullPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetCreationTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.LastAccessedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLastAccessedTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.LastModifiedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLastModifiedTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsHidden)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHidden<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetIsHidden)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FileSystemName)(::windows_core::Interface::as_raw(self), filesystem, &mut result__).from_abi(result__)
    }
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FileSystemPath)(::windows_core::Interface::as_raw(self), filesystem, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, path: P0) -> ::windows_core::Result<IFsiItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_Item)(::windows_core::Interface::as_raw(self), path.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumFsiItems(&self) -> ::windows_core::Result<IEnumFsiItems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumFsiItems)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddDirectory<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.AddDirectory)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddFile<P0, P1>(&self, path: P0, filedata: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).base__.AddFile)(::windows_core::Interface::as_raw(self), path.into_param().abi(), filedata.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddTree<P0, P1>(&self, sourcedirectory: P0, includebasedirectory: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.AddTree)(::windows_core::Interface::as_raw(self), sourcedirectory.into_param().abi(), includebasedirectory.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, item: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFsiItem>,
    {
        (::windows_core::Interface::vtable(self).base__.Add)(::windows_core::Interface::as_raw(self), item.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Remove)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn RemoveTree<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.RemoveTree)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddTreeWithNamedStreams<P0, P1>(&self, sourcedirectory: P0, includebasedirectory: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).AddTreeWithNamedStreams)(::windows_core::Interface::as_raw(self), sourcedirectory.into_param().abi(), includebasedirectory.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFsiDirectoryItem2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem, IFsiDirectoryItem);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFsiDirectoryItem2 {
    type Vtable = IFsiDirectoryItem2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFsiDirectoryItem2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7fb4b9b_6d96_4d7b_9115_201b144811ef);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem2_Vtbl {
    pub base__: IFsiDirectoryItem_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddTreeWithNamedStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcedirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddTreeWithNamedStreams: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFsiFileItem(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiFileItem {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FullPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FullPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCreationTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastAccessedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastAccessedTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastModifiedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastModifiedTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsHidden)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHidden<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetIsHidden)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FileSystemName)(::windows_core::Interface::as_raw(self), filesystem, &mut result__).from_abi(result__)
    }
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FileSystemPath)(::windows_core::Interface::as_raw(self), filesystem, &mut result__).from_abi(result__)
    }
    pub unsafe fn DataSize(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DataSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DataSize32BitLow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DataSize32BitLow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DataSize32BitHigh(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DataSize32BitHigh)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Data(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Data)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetData<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).SetData)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFsiFileItem, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFsiFileItem {
    type Vtable = IFsiFileItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFsiFileItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fdb_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    pub DataSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i64) -> ::windows_core::HRESULT,
    pub DataSize32BitLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub DataSize32BitHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Data: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetData: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFsiFileItem2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiFileItem2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FullPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FullPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetCreationTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.LastAccessedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLastAccessedTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.LastModifiedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLastModifiedTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsHidden)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHidden<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetIsHidden)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FileSystemName)(::windows_core::Interface::as_raw(self), filesystem, &mut result__).from_abi(result__)
    }
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FileSystemPath)(::windows_core::Interface::as_raw(self), filesystem, &mut result__).from_abi(result__)
    }
    pub unsafe fn DataSize(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DataSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DataSize32BitLow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DataSize32BitLow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DataSize32BitHigh(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DataSize32BitHigh)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Data(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Data)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetData<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).base__.SetData)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FsiNamedStreams(&self) -> ::windows_core::Result<IFsiNamedStreams> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FsiNamedStreams)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsNamedStream(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsNamedStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddStream<P0, P1>(&self, name: P0, streamdata: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).AddStream)(::windows_core::Interface::as_raw(self), name.into_param().abi(), streamdata.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStream<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveStream)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRealTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRealTime<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsRealTime)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFsiFileItem2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem, IFsiFileItem);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFsiFileItem2 {
    type Vtable = IFsiFileItem2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFsiFileItem2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x199d0c19_11e1_40eb_8ec2_c8c822a07792);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem2_Vtbl {
    pub base__: IFsiFileItem_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub FsiNamedStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FsiNamedStreams: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsNamedStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsNamedStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, streamdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddStream: usize,
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRealTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsRealTime: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFsiItem(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiItem {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FullPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FullPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCreationTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastAccessedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLastAccessedTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastModifiedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLastModifiedTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsHidden)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHidden<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsHidden)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FileSystemName)(::windows_core::Interface::as_raw(self), filesystem, &mut result__).from_abi(result__)
    }
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FileSystemPath)(::windows_core::Interface::as_raw(self), filesystem, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFsiItem, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFsiItem {
    type Vtable = IFsiItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFsiItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fd9_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FullPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub SetCreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows_core::HRESULT,
    pub LastAccessedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub SetLastAccessedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows_core::HRESULT,
    pub LastModifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub SetLastModifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsHidden: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsHidden: usize,
    pub FileSystemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FileSystemPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFsiNamedStreams(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiNamedStreams {
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IFsiFileItem2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumNamedStreams(&self) -> ::windows_core::Result<IEnumFsiItems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumNamedStreams)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFsiNamedStreams, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFsiNamedStreams {
    type Vtable = IFsiNamedStreams_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFsiNamedStreams {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed79ba56_5294_4250_8d46_f9aecee23459);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiNamedStreams_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub EnumNamedStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IIsoImageManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IIsoImageManager {
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Stream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Stream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPath<P0>(&self, val: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPath)(::windows_core::Interface::as_raw(self), val.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetStream<P0>(&self, data: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).SetStream)(::windows_core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn Validate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Validate)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IIsoImageManager, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IIsoImageManager {
    type Vtable = IIsoImageManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IIsoImageManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ca38be5_fbbb_4800_95a1_a438865eb0d4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIsoImageManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Stream: usize,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetStream: usize,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IJolietDiscMaster(::windows_core::IUnknown);
impl IJolietDiscMaster {
    pub unsafe fn GetTotalDataBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTotalDataBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUsedDataBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUsedDataBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataBlockSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDataBlockSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn AddData<P0>(&self, pstorage: P0, lfileoverwrite: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::StructuredStorage::IStorage>,
    {
        (::windows_core::Interface::vtable(self).AddData)(::windows_core::Interface::as_raw(self), pstorage.into_param().abi(), lfileoverwrite).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetJolietProperties(&self) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetJolietProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetJolietProperties<P0>(&self, ppropstg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::StructuredStorage::IPropertyStorage>,
    {
        (::windows_core::Interface::vtable(self).SetJolietProperties)(::windows_core::Interface::as_raw(self), ppropstg.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IJolietDiscMaster, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJolietDiscMaster {
    type Vtable = IJolietDiscMaster_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IJolietDiscMaster {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3bc42ce_4e5c_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJolietDiscMaster_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetTotalDataBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT,
    pub GetUsedDataBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT,
    pub GetDataBlockSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub AddData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstorage: *mut ::core::ffi::c_void, lfileoverwrite: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    AddData: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetJolietProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetJolietProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetJolietProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetJolietProperties: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMultisession(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMultisession {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsSupportedOnCurrentMediaState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInUse<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetInUse)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InUse(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InUse)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportRecorder(&self) -> ::windows_core::Result<IDiscRecorder2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ImportRecorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMultisession, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMultisession {
    type Vtable = IMultisession_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMultisession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354150_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisession_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSupportedOnCurrentMediaState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSupportedOnCurrentMediaState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInUse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InUse: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportRecorder: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMultisessionRandomWrite(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionRandomWrite {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsSupportedOnCurrentMediaState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInUse<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetInUse)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InUse(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.InUse)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportRecorder(&self) -> ::windows_core::Result<IDiscRecorder2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ImportRecorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteUnitSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).WriteUnitSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastWrittenAddress(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastWrittenAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalSectorsOnMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMultisessionRandomWrite, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IMultisession);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMultisessionRandomWrite {
    type Vtable = IMultisessionRandomWrite_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMultisessionRandomWrite {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb507ca23_2204_11dd_966a_001aa01bbc58);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionRandomWrite_Vtbl {
    pub base__: IMultisession_Vtbl,
    pub WriteUnitSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub LastWrittenAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMultisessionSequential(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionSequential {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsSupportedOnCurrentMediaState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInUse<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetInUse)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InUse(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.InUse)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportRecorder(&self) -> ::windows_core::Result<IDiscRecorder2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ImportRecorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFirstDataSession(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsFirstDataSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartAddressOfPreviousSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastWrittenAddressOfPreviousSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NextWritableAddress(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NextWritableAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FreeSectorsOnMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMultisessionSequential, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IMultisession);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMultisessionSequential {
    type Vtable = IMultisessionSequential_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMultisessionSequential {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354151_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential_Vtbl {
    pub base__: IMultisession_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFirstDataSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFirstDataSession: usize,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMultisessionSequential2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionSequential2 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsSupportedOnCurrentMediaState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInUse<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetInUse)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InUse(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.InUse)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportRecorder(&self) -> ::windows_core::Result<IDiscRecorder2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ImportRecorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFirstDataSession(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsFirstDataSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.StartAddressOfPreviousSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastWrittenAddressOfPreviousSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NextWritableAddress(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.NextWritableAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FreeSectorsOnMedia)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteUnitSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).WriteUnitSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMultisessionSequential2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IMultisession, IMultisessionSequential);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMultisessionSequential2 {
    type Vtable = IMultisessionSequential2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMultisessionSequential2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb507ca22_2204_11dd_966a_001aa01bbc58);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential2_Vtbl {
    pub base__: IMultisessionSequential_Vtbl,
    pub WriteUnitSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProgressItem(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IProgressItem {
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FirstBlock(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FirstBlock)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastBlock(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastBlock)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BlockCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BlockCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IProgressItem, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IProgressItem {
    type Vtable = IProgressItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IProgressItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fd5_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FirstBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows_core::HRESULT,
    pub LastBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows_core::HRESULT,
    pub BlockCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocks: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProgressItems(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IProgressItems {
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IProgressItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProgressItemFromBlock(&self, block: u32) -> ::windows_core::Result<IProgressItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProgressItemFromBlock)(::windows_core::Interface::as_raw(self), block, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProgressItemFromDescription<P0>(&self, description: P0) -> ::windows_core::Result<IProgressItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProgressItemFromDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumProgressItems(&self) -> ::windows_core::Result<IEnumProgressItems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumProgressItems)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IProgressItems, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IProgressItems {
    type Vtable = IProgressItems_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IProgressItems {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fd7_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItems_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ProgressItemFromBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, block: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProgressItemFromBlock: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ProgressItemFromDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::std::mem::MaybeUninit<::windows_core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProgressItemFromDescription: usize,
    pub EnumProgressItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRawCDImageCreator(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRawCDImageCreator {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultImage(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateResultImage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddTrack<P0>(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddTrack)(::windows_core::Interface::as_raw(self), datatype, data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddSpecialPregap<P0>(&self, data: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).AddSpecialPregap)(::windows_core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddSubcodeRWGenerator<P0>(&self, subcode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).AddSubcodeRWGenerator)(::windows_core::Interface::as_raw(self), subcode.into_param().abi()).ok()
    }
    pub unsafe fn SetResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetResultingImageType)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn ResultingImageType(&self) -> ::windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ResultingImageType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartOfLeadout(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartOfLeadout)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartOfLeadoutLimit(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartOfLeadoutLimit)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn StartOfLeadoutLimit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartOfLeadoutLimit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisableGaplessAudio<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetDisableGaplessAudio)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisableGaplessAudio(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DisableGaplessAudio)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMediaCatalogNumber<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetMediaCatalogNumber)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn MediaCatalogNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MediaCatalogNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartingTrackNumber(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartingTrackNumber)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn StartingTrackNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartingTrackNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_TrackInfo(&self, trackindex: i32) -> ::windows_core::Result<IRawCDImageTrackInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_TrackInfo)(::windows_core::Interface::as_raw(self), trackindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn NumberOfExistingTracks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NumberOfExistingTracks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastUsedUserSectorInImage(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastUsedUserSectorInImage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExpectedTableOfContents(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExpectedTableOfContents)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IRawCDImageCreator, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IRawCDImageCreator {
    type Vtable = IRawCDImageCreator_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IRawCDImageCreator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25983550_9d65_49ce_b335_40630d901227);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageCreator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResultImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResultImage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: *mut ::core::ffi::c_void, trackindex: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddTrack: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddSpecialPregap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddSpecialPregap: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddSubcodeRWGenerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subcode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddSubcodeRWGenerator: usize,
    pub SetResultingImageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT,
    pub ResultingImageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT,
    pub StartOfLeadout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub SetStartOfLeadoutLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub StartOfLeadoutLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisableGaplessAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisableGaplessAudio: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisableGaplessAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisableGaplessAudio: usize,
    pub SetMediaCatalogNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MediaCatalogNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetStartingTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub StartingTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_TrackInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trackindex: i32, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_TrackInfo: usize,
    pub NumberOfExistingTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub LastUsedUserSectorInImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExpectedTableOfContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExpectedTableOfContents: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRawCDImageTrackInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRawCDImageTrackInfo {
    pub unsafe fn StartingLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartingLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SectorCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SectorCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TrackNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TrackNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SectorType(&self) -> ::windows_core::Result<IMAPI_CD_SECTOR_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SectorType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ISRC(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ISRC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetISRC<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetISRC)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn DigitalAudioCopySetting(&self) -> ::windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DigitalAudioCopySetting)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDigitalAudioCopySetting)(::windows_core::Interface::as_raw(self), value).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AudioHasPreemphasis(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AudioHasPreemphasis)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAudioHasPreemphasis<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAudioHasPreemphasis)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TrackIndexes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TrackIndexes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddTrackIndex(&self, lbaoffset: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddTrackIndex)(::windows_core::Interface::as_raw(self), lbaoffset).ok()
    }
    pub unsafe fn ClearTrackIndex(&self, lbaoffset: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClearTrackIndex)(::windows_core::Interface::as_raw(self), lbaoffset).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IRawCDImageTrackInfo, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IRawCDImageTrackInfo {
    type Vtable = IRawCDImageTrackInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IRawCDImageTrackInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25983551_9d65_49ce_b335_40630d901227);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageTrackInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartingLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub SectorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> ::windows_core::HRESULT,
    pub ISRC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetISRC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DigitalAudioCopySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_core::HRESULT,
    pub SetDigitalAudioCopySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AudioHasPreemphasis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AudioHasPreemphasis: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAudioHasPreemphasis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAudioHasPreemphasis: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TrackIndexes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TrackIndexes: usize,
    pub AddTrackIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows_core::HRESULT,
    pub ClearTrackIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRedbookDiscMaster(::windows_core::IUnknown);
impl IRedbookDiscMaster {
    pub unsafe fn GetTotalAudioTracks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTotalAudioTracks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTotalAudioBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTotalAudioBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUsedAudioBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUsedAudioBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAvailableAudioTrackBlocks(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAvailableAudioTrackBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAudioBlockSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAudioBlockSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAudioTrack(&self, nblocks: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateAudioTrack)(::windows_core::Interface::as_raw(self), nblocks).ok()
    }
    pub unsafe fn AddAudioTrackBlocks(&self, pby: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddAudioTrackBlocks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pby.as_ptr()), pby.len().try_into().unwrap()).ok()
    }
    pub unsafe fn CloseAudioTrack(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CloseAudioTrack)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IRedbookDiscMaster, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRedbookDiscMaster {
    type Vtable = IRedbookDiscMaster_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRedbookDiscMaster {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3bc42cd_4e5c_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRedbookDiscMaster_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetTotalAudioTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pntracks: *mut i32) -> ::windows_core::HRESULT,
    pub GetTotalAudioBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT,
    pub GetUsedAudioBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT,
    pub GetAvailableAudioTrackBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT,
    pub GetAudioBlockSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows_core::HRESULT,
    pub CreateAudioTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nblocks: i32) -> ::windows_core::HRESULT,
    pub AddAudioTrackBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pby: *const u8, cb: i32) -> ::windows_core::HRESULT,
    pub CloseAudioTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStreamConcatenate(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IStreamConcatenate {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.Read)(::windows_core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.Write)(::windows_core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Seek)(::windows_core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSize)(::windows_core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).base__.CopyTo)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self), grfcommitflags.0 as _).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Revert)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LockRegion)(::windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype.0 as _).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnlockRegion)(::windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Stat)(::windows_core::Interface::as_raw(self), pstatstg, grfstatflag.0 as _).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1>(&self, stream1: P0, stream2: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), stream1.into_param().abi(), stream2.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize2(&self, streams: &[::core::option::Option<super::super::System::Com::IStream>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(streams.as_ptr()), streams.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Append<P0>(&self, stream: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), stream.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Append2(&self, streams: &[::core::option::Option<super::super::System::Com::IStream>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Append2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(streams.as_ptr()), streams.len().try_into().unwrap()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IStreamConcatenate, ::windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IStreamConcatenate {
    type Vtable = IStreamConcatenate_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IStreamConcatenate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354146_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamConcatenate_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream1: *mut ::core::ffi::c_void, stream2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, streamcount: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Append: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Append2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, streamcount: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Append2: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStreamInterleave(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IStreamInterleave {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.Read)(::windows_core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.Write)(::windows_core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Seek)(::windows_core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSize)(::windows_core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).base__.CopyTo)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self), grfcommitflags.0 as _).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Revert)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LockRegion)(::windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype.0 as _).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnlockRegion)(::windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Stat)(::windows_core::Interface::as_raw(self), pstatstg, grfstatflag.0 as _).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize(&self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(streams), interleavesizes, streamcount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IStreamInterleave, ::windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IStreamInterleave {
    type Vtable = IStreamInterleave_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IStreamInterleave {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354147_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamInterleave_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, interleavesizes: *const u32, streamcount: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStreamPseudoRandomBased(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IStreamPseudoRandomBased {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.Read)(::windows_core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.base__.Write)(::windows_core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Seek)(::windows_core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSize)(::windows_core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).base__.CopyTo)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self), grfcommitflags.0 as _).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Revert)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LockRegion)(::windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype.0 as _).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnlockRegion)(::windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Stat)(::windows_core::Interface::as_raw(self), pstatstg, grfstatflag.0 as _).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSeed(&self, value: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSeed)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn Seed(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Seed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn put_ExtendedSeed(&self, values: &[u32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_ExtendedSeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(values.as_ptr()), values.len().try_into().unwrap()).ok()
    }
    pub unsafe fn get_ExtendedSeed(&self, values: *mut *mut u32, ecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_ExtendedSeed)(::windows_core::Interface::as_raw(self), values, ecount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IStreamPseudoRandomBased, ::windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IStreamPseudoRandomBased {
    type Vtable = IStreamPseudoRandomBased_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IStreamPseudoRandomBased {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354145_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamPseudoRandomBased_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    pub SetSeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Seed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT,
    pub put_ExtendedSeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *const u32, ecount: u32) -> ::windows_core::HRESULT,
    pub get_ExtendedSeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWriteEngine2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWriteEngine2 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteSection<P0>(&self, data: P0, startingblockaddress: i32, numberofblocks: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).WriteSection)(::windows_core::Interface::as_raw(self), data.into_param().abi(), startingblockaddress, numberofblocks).ok()
    }
    pub unsafe fn CancelWrite(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelWrite)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDiscRecorder2Ex>,
    {
        (::windows_core::Interface::vtable(self).SetRecorder)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn Recorder(&self) -> ::windows_core::Result<IDiscRecorder2Ex> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Recorder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseStreamingWrite12<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseStreamingWrite12)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseStreamingWrite12(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UseStreamingWrite12)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartingSectorsPerSecond(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartingSectorsPerSecond)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn StartingSectorsPerSecond(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartingSectorsPerSecond)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEndingSectorsPerSecond(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEndingSectorsPerSecond)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn EndingSectorsPerSecond(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndingSectorsPerSecond)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBytesPerSector(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBytesPerSector)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn BytesPerSector(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BytesPerSector)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteInProgress(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).WriteInProgress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWriteEngine2, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWriteEngine2 {
    type Vtable = IWriteEngine2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWriteEngine2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354135_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, startingblockaddress: i32, numberofblocks: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteSection: usize,
    pub CancelWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseStreamingWrite12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseStreamingWrite12: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UseStreamingWrite12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseStreamingWrite12: usize,
    pub SetStartingSectorsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub StartingSectorsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub SetEndingSectorsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub EndingSectorsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub SetBytesPerSector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub BytesPerSector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteInProgress: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWriteEngine2EventArgs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWriteEngine2EventArgs {
    pub unsafe fn StartLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SectorCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SectorCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastReadLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastReadLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastWrittenLba(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastWrittenLba)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UsedSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FreeSystemBuffer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWriteEngine2EventArgs, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWriteEngine2EventArgs {
    type Vtable = IWriteEngine2EventArgs_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWriteEngine2EventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354136_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2EventArgs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub LastReadLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub LastWrittenLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub TotalSystemBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub UsedSystemBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub FreeSystemBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWriteSpeedDescriptor(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWriteSpeedDescriptor {
    pub unsafe fn MediaType(&self) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MediaType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RotationTypeIsPureCAV)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).WriteSpeed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWriteSpeedDescriptor, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWriteSpeedDescriptor {
    type Vtable = IWriteSpeedDescriptor_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWriteSpeedDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354144_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteSpeedDescriptor_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RotationTypeIsPureCAV: usize,
    pub WriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
}
pub const BlockRange: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb507ca27_2204_11dd_966a_001aa01bbc58);
pub const BlockRangeList: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb507ca28_2204_11dd_966a_001aa01bbc58);
pub const BootOptions: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fce_975b_59be_a960_9a2a262853a5);
pub const CATID_SMTP_DNSRESOLVERRECORDSINK: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd0b4366_8e03_11d2_94f6_00c04f79f1d6);
pub const CATID_SMTP_DSN: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22b55731_f5f8_4d23_bd8f_87b52371a73a);
pub const CATID_SMTP_GET_AUX_DOMAIN_INFO_FLAGS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84ff368a_fab3_43d7_bcdf_692c5b46e6b1);
pub const CATID_SMTP_LOG: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93d0a538_2c1e_4b68_a7c9_d73a8aa6ee97);
pub const CATID_SMTP_MAXMSGSIZE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xebf159de_a67e_11d2_94f7_00c04f79f1d6);
pub const CATID_SMTP_MSGTRACKLOG: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6df52aa_7db0_11d2_94f4_00c04f79f1d6);
pub const CATID_SMTP_ON_BEFORE_DATA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6628c92_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_INBOUND_COMMAND: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6628c8d_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_MESSAGE_START: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6628c90_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_PER_RECIPIENT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6628c91_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SERVER_RESPONSE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6628c8e_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SESSION_END: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6628c93_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SESSION_START: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6628c8f_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_STORE_DRIVER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59175850_e533_11d1_aa67_00c04fa345f6);
pub const CATID_SMTP_TRANSPORT_CATEGORIZE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x960252a3_0a3a_11d2_9e00_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_POSTCATEGORIZE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76719654_05a6_11d2_9dfd_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_PRECATEGORIZE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3acfb0d_83ff_11d2_9e14_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_ROUTER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x283430c9_1850_11d2_9e03_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_SUBMISSION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff3caa23_00b9_11d2_9dfb_00c04fa322ba);
pub const CLSID_SmtpCat: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb23c35b7_9219_11d2_9e17_00c04fa322ba);
pub const DISPID_DDISCFORMAT2DATAEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCFORMAT2TAOEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEADDED: u32 = 256u32;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED: u32 = 257u32;
pub const DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE: u32 = 256u32;
pub const DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT: u32 = 257u32;
pub const DISPID_DWRITEENGINE2EVENTS_UPDATE: u32 = 256u32;
pub const DISPID_IBLOCKRANGELIST_BLOCKRANGES: u32 = 256u32;
pub const DISPID_IBLOCKRANGE_ENDLBA: u32 = 257u32;
pub const DISPID_IBLOCKRANGE_STARTLBA: u32 = 256u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION: u32 = 771u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME: u32 = 768u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED: u32 = 257u32;
pub const DISPID_IDISCFORMAT2DATA_CANCELWRITE: u32 = 513u32;
pub const DISPID_IDISCFORMAT2DATA_CLIENTNAME: u32 = 272u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS: u32 = 262u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE: u32 = 271u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV: u32 = 276u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED: u32 = 275u32;
pub const DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE: u32 = 270u32;
pub const DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED: u32 = 269u32;
pub const DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE: u32 = 279u32;
pub const DISPID_IDISCFORMAT2DATA_FREESECTORS: u32 = 265u32;
pub const DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION: u32 = 268u32;
pub const DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES: u32 = 280u32;
pub const DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS: u32 = 266u32;
pub const DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE: u32 = 260u32;
pub const DISPID_IDISCFORMAT2DATA_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV: u32 = 274u32;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED: u32 = 273u32;
pub const DISPID_IDISCFORMAT2DATA_SETWRITESPEED: u32 = 514u32;
pub const DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION: u32 = 267u32;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 278u32;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS: u32 = 277u32;
pub const DISPID_IDISCFORMAT2DATA_TOTALSECTORS: u32 = 264u32;
pub const DISPID_IDISCFORMAT2DATA_WRITE: u32 = 512u32;
pub const DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS: u32 = 263u32;
pub const DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_IDISCFORMAT2ERASE_CLIENTNAME: u32 = 259u32;
pub const DISPID_IDISCFORMAT2ERASE_ERASEMEDIA: u32 = 513u32;
pub const DISPID_IDISCFORMAT2ERASE_FULLERASE: u32 = 257u32;
pub const DISPID_IDISCFORMAT2ERASE_MEDIATYPE: u32 = 258u32;
pub const DISPID_IDISCFORMAT2ERASE_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION: u32 = 769u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME: u32 = 768u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
pub const DISPID_IDISCFORMAT2RAWCD_CANCELWRITE: u32 = 515u32;
pub const DISPID_IDISCFORMAT2RAWCD_CLIENTNAME: u32 = 266u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE: u32 = 261u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV: u32 = 270u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED: u32 = 269u32;
pub const DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT: u32 = 260u32;
pub const DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA: u32 = 512u32;
pub const DISPID_IDISCFORMAT2RAWCD_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA: u32 = 516u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE: u32 = 265u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV: u32 = 268u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED: u32 = 267u32;
pub const DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED: u32 = 517u32;
pub const DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION: u32 = 259u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES: u32 = 264u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 272u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS: u32 = 271u32;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA: u32 = 513u32;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION: u32 = 514u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION: u32 = 769u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 771u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME: u32 = 772u32;
pub const DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK: u32 = 513u32;
pub const DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
pub const DISPID_IDISCFORMAT2TAO_CANCELADDTRACK: u32 = 514u32;
pub const DISPID_IDISCFORMAT2TAO_CLIENTNAME: u32 = 270u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE: u32 = 267u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV: u32 = 274u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED: u32 = 273u32;
pub const DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA: u32 = 263u32;
pub const DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS: u32 = 266u32;
pub const DISPID_IDISCFORMAT2TAO_FINISHMEDIA: u32 = 515u32;
pub const DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA: u32 = 261u32;
pub const DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS: u32 = 259u32;
pub const DISPID_IDISCFORMAT2TAO_PREPAREMEDIA: u32 = 512u32;
pub const DISPID_IDISCFORMAT2TAO_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV: u32 = 272u32;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED: u32 = 271u32;
pub const DISPID_IDISCFORMAT2TAO_SETWRITESPEED: u32 = 516u32;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 276u32;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS: u32 = 275u32;
pub const DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA: u32 = 260u32;
pub const DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA: u32 = 262u32;
pub const DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK: u32 = 1793u32;
pub const DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK: u32 = 1792u32;
pub const DISPID_IDISCFORMAT2_MEDIASUPPORTED: u32 = 2049u32;
pub const DISPID_IDISCFORMAT2_RECORDERSUPPORTED: u32 = 2048u32;
pub const DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES: u32 = 1794u32;
pub const DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS: u32 = 258u32;
pub const DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER: u32 = 0u32;
pub const DISPID_IDISCRECORDER2_CLOSETRAY: u32 = 257u32;
pub const DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES: u32 = 521u32;
pub const DISPID_IDISCRECORDER2_CURRENTPROFILES: u32 = 523u32;
pub const DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA: u32 = 518u32;
pub const DISPID_IDISCRECORDER2_DISABLEMCN: u32 = 260u32;
pub const DISPID_IDISCRECORDER2_EJECTMEDIA: u32 = 256u32;
pub const DISPID_IDISCRECORDER2_ENABLEMCN: u32 = 261u32;
pub const DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER: u32 = 525u32;
pub const DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER: u32 = 262u32;
pub const DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER: u32 = 519u32;
pub const DISPID_IDISCRECORDER2_PRODUCTID: u32 = 514u32;
pub const DISPID_IDISCRECORDER2_PRODUCTREVISION: u32 = 515u32;
pub const DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS: u32 = 259u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES: u32 = 520u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES: u32 = 524u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDPROFILES: u32 = 522u32;
pub const DISPID_IDISCRECORDER2_VENDORID: u32 = 513u32;
pub const DISPID_IDISCRECORDER2_VOLUMENAME: u32 = 516u32;
pub const DISPID_IDISCRECORDER2_VOLUMEPATHNAMES: u32 = 517u32;
pub const DISPID_IMULTISESSION_FIRSTDATASESSION: u32 = 512u32;
pub const DISPID_IMULTISESSION_FREESECTORS: u32 = 516u32;
pub const DISPID_IMULTISESSION_IMPORTRECORDER: u32 = 258u32;
pub const DISPID_IMULTISESSION_INUSE: u32 = 257u32;
pub const DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION: u32 = 514u32;
pub const DISPID_IMULTISESSION_LASTWRITTENADDRESS: u32 = 518u32;
pub const DISPID_IMULTISESSION_NEXTWRITABLEADDRESS: u32 = 515u32;
pub const DISPID_IMULTISESSION_SECTORSONMEDIA: u32 = 519u32;
pub const DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION: u32 = 513u32;
pub const DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA: u32 = 256u32;
pub const DISPID_IMULTISESSION_WRITEUNITSIZE: u32 = 517u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP: u32 = 514u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR: u32 = 515u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDTRACK: u32 = 513u32;
pub const DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE: u32 = 512u32;
pub const DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO: u32 = 259u32;
pub const DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS: u32 = 265u32;
pub const DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER: u32 = 260u32;
pub const DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS: u32 = 263u32;
pub const DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE: u32 = 256u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER: u32 = 261u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT: u32 = 257u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT: u32 = 258u32;
pub const DISPID_IRAWCDIMAGECREATOR_TRACKINFO: u32 = 262u32;
pub const DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC: u32 = 264u32;
pub const DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS: u32 = 262u32;
pub const DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING: u32 = 261u32;
pub const DISPID_IRAWCDTRACKINFO_ISRC: u32 = 260u32;
pub const DISPID_IRAWCDTRACKINFO_SECTORCOUNT: u32 = 257u32;
pub const DISPID_IRAWCDTRACKINFO_SECTORTYPE: u32 = 259u32;
pub const DISPID_IRAWCDTRACKINFO_STARTINGLBA: u32 = 256u32;
pub const DISPID_IRAWCDTRACKINFO_TRACKNUMBER: u32 = 258u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER: u32 = 264u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA: u32 = 258u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA: u32 = 259u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT: u32 = 257u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_STARTLBA: u32 = 256u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER: u32 = 260u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER: u32 = 262u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER: u32 = 261u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER: u32 = 263u32;
pub const DISPID_IWRITEENGINE2_BYTESPERSECTOR: u32 = 260u32;
pub const DISPID_IWRITEENGINE2_CANCELWRITE: u32 = 513u32;
pub const DISPID_IWRITEENGINE2_DISCRECORDER: u32 = 256u32;
pub const DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND: u32 = 259u32;
pub const DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND: u32 = 258u32;
pub const DISPID_IWRITEENGINE2_USESTREAMINGWRITE12: u32 = 257u32;
pub const DISPID_IWRITEENGINE2_WRITEINPROGRESS: u32 = 261u32;
pub const DISPID_IWRITEENGINE2_WRITESECTION: u32 = 512u32;
pub const Emulation12MFloppy: EmulationType = EmulationType(1i32);
pub const Emulation144MFloppy: EmulationType = EmulationType(2i32);
pub const Emulation288MFloppy: EmulationType = EmulationType(3i32);
pub const EmulationHardDisk: EmulationType = EmulationType(4i32);
pub const EmulationNone: EmulationType = EmulationType(0i32);
pub const EnumFsiItems: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fc6_975b_59be_a960_9a2a262853a5);
pub const EnumProgressItems: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fca_975b_59be_a960_9a2a262853a5);
pub const FileSystemImageResult: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fcc_975b_59be_a960_9a2a262853a5);
pub const FsiDirectoryItem: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fc8_975b_59be_a960_9a2a262853a5);
pub const FsiFileItem: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fc7_975b_59be_a960_9a2a262853a5);
pub const FsiFileSystemISO9660: FsiFileSystems = FsiFileSystems(1i32);
pub const FsiFileSystemJoliet: FsiFileSystems = FsiFileSystems(2i32);
pub const FsiFileSystemNone: FsiFileSystems = FsiFileSystems(0i32);
pub const FsiFileSystemUDF: FsiFileSystems = FsiFileSystems(4i32);
pub const FsiFileSystemUnknown: FsiFileSystems = FsiFileSystems(1073741824i32);
pub const FsiItemDirectory: FsiItemType = FsiItemType(1i32);
pub const FsiItemFile: FsiItemType = FsiItemType(2i32);
pub const FsiItemNotFound: FsiItemType = FsiItemType(0i32);
pub const FsiNamedStreams: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6b6f8ed_6d19_44b4_b539_b159b793a32d);
pub const FsiStream: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fcd_975b_59be_a960_9a2a262853a5);
pub const GUID_SMTPSVC_SOURCE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b3c0666_e470_11d1_aa67_00c04fa345f6);
pub const GUID_SMTP_SOURCE_TYPE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb65c4dc_e468_11d1_aa67_00c04fa345f6);
pub const IMAPI2FS_BOOT_ENTRY_COUNT_MAX: u32 = 32u32;
pub const IMAPI2FS_FullVersion_STR: ::windows_core::PCSTR = ::windows_core::s!("1.0");
pub const IMAPI2FS_FullVersion_WSTR: ::windows_core::PCWSTR = ::windows_core::w!("1.0");
pub const IMAPI2FS_MajorVersion: u32 = 1u32;
pub const IMAPI2FS_MinorVersion: u32 = 0u32;
pub const IMAPI2_DEFAULT_COMMAND_TIMEOUT: u32 = 10u32;
pub const IMAPILib2_MajorVersion: u32 = 1u32;
pub const IMAPILib2_MinorVersion: u32 = 0u32;
pub const IMAPI_BURN_VERIFICATION_FULL: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(2i32);
pub const IMAPI_BURN_VERIFICATION_NONE: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(0i32);
pub const IMAPI_BURN_VERIFICATION_QUICK: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(1i32);
pub const IMAPI_CD_SECTOR_AUDIO: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(0i32);
pub const IMAPI_CD_SECTOR_MODE1: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(2i32);
pub const IMAPI_CD_SECTOR_MODE1RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(6i32);
pub const IMAPI_CD_SECTOR_MODE2FORM0: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(3i32);
pub const IMAPI_CD_SECTOR_MODE2FORM0RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(7i32);
pub const IMAPI_CD_SECTOR_MODE2FORM1: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(4i32);
pub const IMAPI_CD_SECTOR_MODE2FORM1RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(8i32);
pub const IMAPI_CD_SECTOR_MODE2FORM2: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(5i32);
pub const IMAPI_CD_SECTOR_MODE2FORM2RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(9i32);
pub const IMAPI_CD_SECTOR_MODE_ZERO: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(1i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PERMITTED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(0i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PROHIBITED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(1i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_SCMS: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(2i32);
pub const IMAPI_E_ALREADYOPEN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220958i32);
pub const IMAPI_E_BADJOLIETNAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220963i32);
pub const IMAPI_E_BOOTIMAGE_AND_NONBLANK_DISC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220946i32);
pub const IMAPI_E_CANNOT_WRITE_TO_MEDIA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220948i32);
pub const IMAPI_E_COMPRESSEDSTASH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220952i32);
pub const IMAPI_E_DEVICE_INVALIDTYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220972i32);
pub const IMAPI_E_DEVICE_NOPROPERTIES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220975i32);
pub const IMAPI_E_DEVICE_NOTACCESSIBLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220974i32);
pub const IMAPI_E_DEVICE_NOTPRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220973i32);
pub const IMAPI_E_DEVICE_STILL_IN_USE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220954i32);
pub const IMAPI_E_DISCFULL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220964i32);
pub const IMAPI_E_DISCINFO: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220967i32);
pub const IMAPI_E_ENCRYPTEDSTASH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220951i32);
pub const IMAPI_E_FILEACCESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220968i32);
pub const IMAPI_E_FILEEXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220956i32);
pub const IMAPI_E_FILESYSTEM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220969i32);
pub const IMAPI_E_GENERIC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220978i32);
pub const IMAPI_E_INITIALIZE_ENDWRITE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220970i32);
pub const IMAPI_E_INITIALIZE_WRITE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220971i32);
pub const IMAPI_E_INVALIDIMAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220962i32);
pub const IMAPI_E_LOSS_OF_STREAMING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220953i32);
pub const IMAPI_E_MEDIUM_INVALIDTYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220976i32);
pub const IMAPI_E_MEDIUM_NOTPRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220977i32);
pub const IMAPI_E_NOACTIVEFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220961i32);
pub const IMAPI_E_NOACTIVERECORDER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220960i32);
pub const IMAPI_E_NOTENOUGHDISKFORSTASH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220950i32);
pub const IMAPI_E_NOTINITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220980i32);
pub const IMAPI_E_NOTOPENED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220981i32);
pub const IMAPI_E_REMOVABLESTASH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220949i32);
pub const IMAPI_E_STASHINUSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220955i32);
pub const IMAPI_E_TRACKNOTOPEN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220966i32);
pub const IMAPI_E_TRACKOPEN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220965i32);
pub const IMAPI_E_TRACK_NOT_BIG_ENOUGH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220947i32);
pub const IMAPI_E_USERABORT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220979i32);
pub const IMAPI_E_WRONGDISC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220957i32);
pub const IMAPI_E_WRONGFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220959i32);
pub const IMAPI_FEATURE_PAGE_TYPE_AACS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(269i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_PSEUDO_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(56i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(64i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(65i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CDRW_CAV_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(39i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_ANALOG_PLAY: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(259i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MASTERING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(46i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MULTIREAD: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(29i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(30i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_RW_MEDIA_WRITE_SUPPORT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(55i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_TRACK_AT_ONCE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(45i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CORE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(1i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DISC_CONTROL_BLOCKS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(266i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(48i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_RW_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(50i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_R_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(49i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CPRM: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(267i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CSS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(262i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_DASH_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(47i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(43i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_RW: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(42i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R_DUAL_LAYER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(59i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(31i32);
pub const IMAPI_FEATURE_PAGE_TYPE_EMBEDDED_CHANGER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(258i32);
pub const IMAPI_FEATURE_PAGE_TYPE_ENHANCED_DEFECT_REPORTING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(41i32);
pub const IMAPI_FEATURE_PAGE_TYPE_FIRMWARE_INFORMATION: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(268i32);
pub const IMAPI_FEATURE_PAGE_TYPE_FORMATTABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(35i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HARDWARE_DEFECT_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(36i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(80i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(81i32);
pub const IMAPI_FEATURE_PAGE_TYPE_INCREMENTAL_STREAMING_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(33i32);
pub const IMAPI_FEATURE_PAGE_TYPE_LAYER_JUMP_RECORDING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(51i32);
pub const IMAPI_FEATURE_PAGE_TYPE_LOGICAL_UNIT_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(264i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MEDIA_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(265i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MICROCODE_UPDATE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(260i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MORPHING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(2i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MRW: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(40i32);
pub const IMAPI_FEATURE_PAGE_TYPE_POWER_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(256i32);
pub const IMAPI_FEATURE_PAGE_TYPE_PROFILE_LIST: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(0i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_READABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(16i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(32i32);
pub const IMAPI_FEATURE_PAGE_TYPE_REAL_TIME_STREAMING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(263i32);
pub const IMAPI_FEATURE_PAGE_TYPE_REMOVABLE_MEDIUM: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(3i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(38i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RIGID_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(44i32);
pub const IMAPI_FEATURE_PAGE_TYPE_SECTOR_ERASABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(34i32);
pub const IMAPI_FEATURE_PAGE_TYPE_SMART: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(257i32);
pub const IMAPI_FEATURE_PAGE_TYPE_TIMEOUT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(261i32);
pub const IMAPI_FEATURE_PAGE_TYPE_VCPS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(272i32);
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_ONCE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(37i32);
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_PROTECT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(4i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_APPENDABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(4i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_BLANK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(2i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_DAMAGED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1024i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_ERASE_REQUIRED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(2048i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINALIZED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(16384i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINAL_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(8i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_INFORMATIONAL_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(15i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_NON_EMPTY_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(4096i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_OVERWRITE_ONLY: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_RANDOMLY_WRITABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNKNOWN: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(0i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(64512i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MEDIA: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(32768i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_WRITE_PROTECTED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(8192i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_CALIBRATING_POWER: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_COMPLETED: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(6i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FINALIZATION: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(5i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FORMATTING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_INITIALIZING_HARDWARE: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(2i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VALIDATING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(7i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_WRITING_DATA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(4i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_COOKED: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(2i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_RAW: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(3i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_PQ_ONLY: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(1i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_WRITING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(2i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(4i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_WRITING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(2i32);
pub const IMAPI_MEDIA_TYPE_BDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(18i32);
pub const IMAPI_MEDIA_TYPE_BDRE: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(19i32);
pub const IMAPI_MEDIA_TYPE_BDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(17i32);
pub const IMAPI_MEDIA_TYPE_CDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(2i32);
pub const IMAPI_MEDIA_TYPE_CDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(1i32);
pub const IMAPI_MEDIA_TYPE_CDRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(3i32);
pub const IMAPI_MEDIA_TYPE_DISK: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(12i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(9i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(10i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(11i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(6i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(7i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(13i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(8i32);
pub const IMAPI_MEDIA_TYPE_DVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(5i32);
pub const IMAPI_MEDIA_TYPE_DVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(4i32);
pub const IMAPI_MEDIA_TYPE_HDDVDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(15i32);
pub const IMAPI_MEDIA_TYPE_HDDVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(16i32);
pub const IMAPI_MEDIA_TYPE_HDDVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(14i32);
pub const IMAPI_MEDIA_TYPE_MAX: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(19i32);
pub const IMAPI_MEDIA_TYPE_UNKNOWN: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(0i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CHANGEABLE_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(1i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CURRENT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(0i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_DEFAULT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(2i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_SAVED_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(3i32);
pub const IMAPI_MODE_PAGE_TYPE_CACHING: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(8i32);
pub const IMAPI_MODE_PAGE_TYPE_INFORMATIONAL_EXCEPTIONS: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(28i32);
pub const IMAPI_MODE_PAGE_TYPE_LEGACY_CAPABILITIES: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(42i32);
pub const IMAPI_MODE_PAGE_TYPE_MRW: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(3i32);
pub const IMAPI_MODE_PAGE_TYPE_POWER_CONDITION: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(26i32);
pub const IMAPI_MODE_PAGE_TYPE_READ_WRITE_ERROR_RECOVERY: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(1i32);
pub const IMAPI_MODE_PAGE_TYPE_TIMEOUT_AND_PROTECT: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(29i32);
pub const IMAPI_MODE_PAGE_TYPE_WRITE_PARAMETERS: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(5i32);
pub const IMAPI_PROFILE_TYPE_AS_MO: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(5i32);
pub const IMAPI_PROFILE_TYPE_BD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(67i32);
pub const IMAPI_PROFILE_TYPE_BD_ROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(64i32);
pub const IMAPI_PROFILE_TYPE_BD_R_RANDOM_RECORDING: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(66i32);
pub const IMAPI_PROFILE_TYPE_BD_R_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(65i32);
pub const IMAPI_PROFILE_TYPE_CDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(8i32);
pub const IMAPI_PROFILE_TYPE_CD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(9i32);
pub const IMAPI_PROFILE_TYPE_CD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(10i32);
pub const IMAPI_PROFILE_TYPE_DDCDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(32i32);
pub const IMAPI_PROFILE_TYPE_DDCD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(33i32);
pub const IMAPI_PROFILE_TYPE_DDCD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(34i32);
pub const IMAPI_PROFILE_TYPE_DVDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(16i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(17i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(19i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RW_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(20i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_LAYER_JUMP: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(22i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(21i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(27i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(26i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW_DUAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(42i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R_DUAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(43i32);
pub const IMAPI_PROFILE_TYPE_DVD_RAM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(18i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_RAM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(82i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(81i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_ROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(80i32);
pub const IMAPI_PROFILE_TYPE_INVALID: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(0i32);
pub const IMAPI_PROFILE_TYPE_MO_ERASABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(3i32);
pub const IMAPI_PROFILE_TYPE_MO_WRITE_ONCE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(4i32);
pub const IMAPI_PROFILE_TYPE_NON_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(1i32);
pub const IMAPI_PROFILE_TYPE_NON_STANDARD: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(65535i32);
pub const IMAPI_PROFILE_TYPE_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(2i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_LBA: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(0i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_SESSION: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(2i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_TRACK: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(1i32);
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_BD: u32 = 2195u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_CD: u32 = 75u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_DVD: u32 = 680u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD: u32 = 4568u32;
pub const IMAPI_SECTOR_SIZE: u32 = 2048u32;
pub const IMAPI_S_BUFFER_TO_SMALL: ::windows_core::HRESULT = ::windows_core::HRESULT(262657i32);
pub const IMAPI_S_PROPERTIESIGNORED: ::windows_core::HRESULT = ::windows_core::HRESULT(262656i32);
pub const IMAPI_WRITEPROTECTED_BY_CARTRIDGE: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(2i32);
pub const IMAPI_WRITEPROTECTED_BY_DISC_CONTROL_BLOCK: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16i32);
pub const IMAPI_WRITEPROTECTED_BY_MEDIA_SPECIFIC_REASON: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(4i32);
pub const IMAPI_WRITEPROTECTED_BY_SOFTWARE_WRITE_PROTECT: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(8i32);
pub const IMAPI_WRITEPROTECTED_READ_ONLY_MEDIA: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16384i32);
pub const IMAPI_WRITEPROTECTED_UNTIL_POWERDOWN: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(1i32);
pub const IMMPID_CPV_AFTER__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32769i32);
pub const IMMPID_CPV_BEFORE__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32767i32);
pub const IMMPID_CP_START: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32768i32);
pub const IMMPID_MPV_AFTER__: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12294i32);
pub const IMMPID_MPV_BEFORE__: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12287i32);
pub const IMMPID_MPV_MESSAGE_CREATION_FLAGS: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12289i32);
pub const IMMPID_MPV_MESSAGE_OPEN_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12290i32);
pub const IMMPID_MPV_STORE_DRIVER_HANDLE: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12288i32);
pub const IMMPID_MPV_TOTAL_OPEN_CONTENT_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12293i32);
pub const IMMPID_MPV_TOTAL_OPEN_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12291i32);
pub const IMMPID_MPV_TOTAL_OPEN_PROPERTY_STREAM_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12292i32);
pub const IMMPID_MP_AFTER__: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4153i32);
pub const IMMPID_MP_ARRIVAL_FILETIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4121i32);
pub const IMMPID_MP_ARRIVAL_TIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4115i32);
pub const IMMPID_MP_AUTHENTICATED_USER_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4104i32);
pub const IMMPID_MP_BEFORE__: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4095i32);
pub const IMMPID_MP_BINARYMIME_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4109i32);
pub const IMMPID_MP_CHUNKING_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4108i32);
pub const IMMPID_MP_CLIENT_AUTH_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4149i32);
pub const IMMPID_MP_CLIENT_AUTH_USER: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4148i32);
pub const IMMPID_MP_CONNECTION_IP_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4105i32);
pub const IMMPID_MP_CONNECTION_SERVER_IP_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4134i32);
pub const IMMPID_MP_CONNECTION_SERVER_PORT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4147i32);
pub const IMMPID_MP_CONTENT_FILE_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4097i32);
pub const IMMPID_MP_CONTENT_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4145i32);
pub const IMMPID_MP_CRC_GLOBAL: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4150i32);
pub const IMMPID_MP_CRC_RECIPS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4151i32);
pub const IMMPID_MP_DEFERRED_DELIVERY_FILETIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4141i32);
pub const IMMPID_MP_DOMAIN_LIST: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4102i32);
pub const IMMPID_MP_DSN_ENVID_VALUE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4112i32);
pub const IMMPID_MP_DSN_RET_VALUE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4113i32);
pub const IMMPID_MP_EIGHTBIT_MIME_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4107i32);
pub const IMMPID_MP_ENCRYPTION_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4146i32);
pub const IMMPID_MP_ERROR_CODE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4111i32);
pub const IMMPID_MP_EXPIRE_DELAY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4117i32);
pub const IMMPID_MP_EXPIRE_NDR: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4118i32);
pub const IMMPID_MP_FOUND_EMBEDDED_CRLF_DOT_CRLF: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4126i32);
pub const IMMPID_MP_FROM_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4139i32);
pub const IMMPID_MP_HELO_DOMAIN: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4106i32);
pub const IMMPID_MP_HR_CAT_STATUS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4122i32);
pub const IMMPID_MP_INBOUND_MAIL_FROM_AUTH: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4152i32);
pub const IMMPID_MP_LOCAL_EXPIRE_DELAY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4119i32);
pub const IMMPID_MP_LOCAL_EXPIRE_NDR: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4120i32);
pub const IMMPID_MP_MESSAGE_STATUS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4116i32);
pub const IMMPID_MP_MSGCLASS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4144i32);
pub const IMMPID_MP_MSG_GUID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4123i32);
pub const IMMPID_MP_MSG_SIZE_HINT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4127i32);
pub const IMMPID_MP_NUM_RECIPIENTS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4137i32);
pub const IMMPID_MP_ORIGINAL_ARRIVAL_TIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4143i32);
pub const IMMPID_MP_PICKUP_FILE_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4103i32);
pub const IMMPID_MP_RECIPIENT_LIST: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4096i32);
pub const IMMPID_MP_REMOTE_AUTHENTICATION_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4110i32);
pub const IMMPID_MP_REMOTE_SERVER_DSN_CAPABLE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4114i32);
pub const IMMPID_MP_RFC822_BCC_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4133i32);
pub const IMMPID_MP_RFC822_CC_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4132i32);
pub const IMMPID_MP_RFC822_FROM_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4130i32);
pub const IMMPID_MP_RFC822_MSG_ID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4128i32);
pub const IMMPID_MP_RFC822_MSG_SUBJECT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4129i32);
pub const IMMPID_MP_RFC822_TO_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4131i32);
pub const IMMPID_MP_SCANNED_FOR_CRLF_DOT_CRLF: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4125i32);
pub const IMMPID_MP_SENDER_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4140i32);
pub const IMMPID_MP_SENDER_ADDRESS_LEGACY_EX_DN: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4101i32);
pub const IMMPID_MP_SENDER_ADDRESS_OTHER: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4142i32);
pub const IMMPID_MP_SENDER_ADDRESS_SMTP: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4098i32);
pub const IMMPID_MP_SENDER_ADDRESS_X400: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4100i32);
pub const IMMPID_MP_SENDER_ADDRESS_X500: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4099i32);
pub const IMMPID_MP_SERVER_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4135i32);
pub const IMMPID_MP_SERVER_VERSION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4136i32);
pub const IMMPID_MP_SUPERSEDES_MSG_GUID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4124i32);
pub const IMMPID_MP_X_PRIORITY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4138i32);
pub const IMMPID_NMP_AFTER__: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24585i32);
pub const IMMPID_NMP_BEFORE__: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24575i32);
pub const IMMPID_NMP_HEADERS: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24582i32);
pub const IMMPID_NMP_NEWSGROUP_LIST: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24581i32);
pub const IMMPID_NMP_NNTP_APPROVED_HEADER: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24584i32);
pub const IMMPID_NMP_NNTP_PROCESSING: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24583i32);
pub const IMMPID_NMP_POST_TOKEN: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24580i32);
pub const IMMPID_NMP_PRIMARY_ARTID: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24579i32);
pub const IMMPID_NMP_PRIMARY_GROUP: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24578i32);
pub const IMMPID_NMP_SECONDARY_ARTNUM: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24577i32);
pub const IMMPID_NMP_SECONDARY_GROUPS: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24576i32);
pub const IMMPID_RPV_AFTER__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16386i32);
pub const IMMPID_RPV_BEFORE__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16383i32);
pub const IMMPID_RPV_DONT_DELIVER: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16384i32);
pub const IMMPID_RPV_NO_NAME_COLLISIONS: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16385i32);
pub const IMMPID_RP_ADDRESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8195i32);
pub const IMMPID_RP_ADDRESS_OTHER: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8211i32);
pub const IMMPID_RP_ADDRESS_SMTP: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8201i32);
pub const IMMPID_RP_ADDRESS_TYPE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8194i32);
pub const IMMPID_RP_ADDRESS_TYPE_SMTP: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8196i32);
pub const IMMPID_RP_ADDRESS_X400: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8202i32);
pub const IMMPID_RP_ADDRESS_X500: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8203i32);
pub const IMMPID_RP_AFTER__: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8213i32);
pub const IMMPID_RP_BEFORE__: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8191i32);
pub const IMMPID_RP_DISPLAY_NAME: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8212i32);
pub const IMMPID_RP_DOMAIN: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8210i32);
pub const IMMPID_RP_DSN_NOTIFY_INVALID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8193i32);
pub const IMMPID_RP_DSN_NOTIFY_SUCCESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8192i32);
pub const IMMPID_RP_DSN_NOTIFY_VALUE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8199i32);
pub const IMMPID_RP_DSN_ORCPT_VALUE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8200i32);
pub const IMMPID_RP_DSN_PRE_CAT_ADDRESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8207i32);
pub const IMMPID_RP_ERROR_CODE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8197i32);
pub const IMMPID_RP_ERROR_STRING: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8198i32);
pub const IMMPID_RP_LEGACY_EX_DN: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8204i32);
pub const IMMPID_RP_MDB_GUID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8208i32);
pub const IMMPID_RP_RECIPIENT_FLAGS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8205i32);
pub const IMMPID_RP_SMTP_STATUS_STRING: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8206i32);
pub const IMMPID_RP_USER_GUID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8209i32);
pub const MEDIA_BLANK: MEDIA_FLAGS = MEDIA_FLAGS(1i32);
pub const MEDIA_CDDA_CDROM: MEDIA_TYPES = MEDIA_TYPES(1i32);
pub const MEDIA_CD_EXTRA: MEDIA_TYPES = MEDIA_TYPES(4i32);
pub const MEDIA_CD_I: MEDIA_TYPES = MEDIA_TYPES(3i32);
pub const MEDIA_CD_OTHER: MEDIA_TYPES = MEDIA_TYPES(5i32);
pub const MEDIA_CD_ROM_XA: MEDIA_TYPES = MEDIA_TYPES(2i32);
pub const MEDIA_FORMAT_UNUSABLE_BY_IMAPI: MEDIA_FLAGS = MEDIA_FLAGS(8i32);
pub const MEDIA_RW: MEDIA_FLAGS = MEDIA_FLAGS(2i32);
pub const MEDIA_SPECIAL: MEDIA_TYPES = MEDIA_TYPES(6i32);
pub const MEDIA_WRITABLE: MEDIA_FLAGS = MEDIA_FLAGS(4i32);
pub const MPV_INBOUND_CUTOFF_EXCEEDED: u32 = 1u32;
pub const MPV_WRITE_CONTENT: u32 = 2u32;
pub const MP_MSGCLASS_DELIVERY_REPORT: u32 = 3u32;
pub const MP_MSGCLASS_NONDELIVERY_REPORT: u32 = 4u32;
pub const MP_MSGCLASS_REPLICATION: u32 = 2u32;
pub const MP_MSGCLASS_SYSTEM: u32 = 1u32;
pub const MP_STATUS_ABANDON_DELIVERY: u32 = 6u32;
pub const MP_STATUS_ABORT_DELIVERY: u32 = 2u32;
pub const MP_STATUS_BAD_MAIL: u32 = 3u32;
pub const MP_STATUS_CATEGORIZED: u32 = 5u32;
pub const MP_STATUS_RETRY: u32 = 1u32;
pub const MP_STATUS_SUBMITTED: u32 = 4u32;
pub const MP_STATUS_SUCCESS: u32 = 0u32;
pub const MSDiscMasterObj: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x520cca63_51a5_11d3_9144_00104ba11c5e);
pub const MSDiscRecorderObj: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x520cca61_51a5_11d3_9144_00104ba11c5e);
pub const MSEnumDiscRecordersObj: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a03567a_63cb_4ba8_baf6_52119816d1ef);
pub const MsftDiscFormat2Data: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2735412a_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2Erase: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2735412b_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2RawCD: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354128_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2TrackAtOnce: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354129_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscMaster2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2735412e_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscRecorder2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2735412d_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftFileSystemImage: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fc5_975b_59be_a960_9a2a262853a5);
pub const MsftIsoImageManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xceee3b62_8f56_4056_869b_ef16917e3efc);
pub const MsftMultisessionRandomWrite: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb507ca24_2204_11dd_966a_001aa01bbc58);
pub const MsftMultisessionSequential: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354122_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftRawCDImageCreator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25983561_9d65_49ce_b335_40630d901227);
pub const MsftStreamConcatenate: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354125_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamInterleave: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354124_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamPrng001: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354126_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamZero: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354127_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftWriteEngine2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2735412c_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftWriteSpeedDescriptor: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27354123_7f64_5b0f_8f00_5d77afbe261e);
pub const NMP_PROCESS_CONTROL: u32 = 2u32;
pub const NMP_PROCESS_MODERATOR: u32 = 4u32;
pub const NMP_PROCESS_POST: u32 = 1u32;
pub const PlatformEFI: PlatformId = PlatformId(239i32);
pub const PlatformMac: PlatformId = PlatformId(2i32);
pub const PlatformPowerPC: PlatformId = PlatformId(1i32);
pub const PlatformX86: PlatformId = PlatformId(0i32);
pub const ProgressItem: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fcb_975b_59be_a960_9a2a262853a5);
pub const ProgressItems: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c941fc9_975b_59be_a960_9a2a262853a5);
pub const RECORDER_BURNING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(2u32);
pub const RECORDER_CDR: RECORDER_TYPES = RECORDER_TYPES(1i32);
pub const RECORDER_CDRW: RECORDER_TYPES = RECORDER_TYPES(2i32);
pub const RECORDER_DOING_NOTHING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(0u32);
pub const RECORDER_OPENED: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(1u32);
pub const RP_DELIVERED: u32 = 272u32;
pub const RP_DSN_HANDLED: u32 = 64u32;
pub const RP_DSN_NOTIFY_DELAY: u32 = 67108864u32;
pub const RP_DSN_NOTIFY_FAILURE: u32 = 33554432u32;
pub const RP_DSN_NOTIFY_INVALID: u32 = 0u32;
pub const RP_DSN_NOTIFY_MASK: u32 = 251658240u32;
pub const RP_DSN_NOTIFY_NEVER: u32 = 134217728u32;
pub const RP_DSN_NOTIFY_SUCCESS: u32 = 16777216u32;
pub const RP_DSN_SENT_DELAYED: u32 = 16384u32;
pub const RP_DSN_SENT_DELIVERED: u32 = 131136u32;
pub const RP_DSN_SENT_EXPANDED: u32 = 32832u32;
pub const RP_DSN_SENT_NDR: u32 = 1104u32;
pub const RP_DSN_SENT_RELAYED: u32 = 65600u32;
pub const RP_ENPANDED: u32 = 8208u32;
pub const RP_ERROR_CONTEXT_CAT: u32 = 2097152u32;
pub const RP_ERROR_CONTEXT_MTA: u32 = 4194304u32;
pub const RP_ERROR_CONTEXT_STORE: u32 = 1048576u32;
pub const RP_EXPANDED: u32 = 8208u32;
pub const RP_FAILED: u32 = 2096u32;
pub const RP_GENERAL_FAILURE: u32 = 32u32;
pub const RP_HANDLED: u32 = 16u32;
pub const RP_RECIP_FLAGS_RESERVED: u32 = 15u32;
pub const RP_REMOTE_MTA_NO_DSN: u32 = 524288u32;
pub const RP_UNRESOLVED: u32 = 4144u32;
pub const RP_VOLATILE_FLAGS_MASK: u32 = 4026531840u32;
pub const SZ_PROGID_SMTPCAT: ::windows_core::PCSTR = ::windows_core::s!("Smtp.Cat");
pub const tagIMMPID_CPV_STRUCT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2a76b2a_e52d_11d1_aa64_00c04fa35b82);
pub const tagIMMPID_MPV_STRUCT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbe69706_c9bd_11d1_9ff2_00c04fa37348);
pub const tagIMMPID_MP_STRUCT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13384cf0_b3c4_11d1_aa92_00aa006bc80b);
pub const tagIMMPID_NMP_STRUCT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7433a9aa_20e2_11d2_94d6_00c04fa379f1);
pub const tagIMMPID_RPV_STRUCT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79e82049_d320_11d1_9ff4_00c04fa37348);
pub const tagIMMPID_RP_STRUCT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79e82048_d320_11d1_9ff4_00c04fa37348);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISC_RECORDER_STATE_FLAGS(pub u32);
impl ::core::marker::Copy for DISC_RECORDER_STATE_FLAGS {}
impl ::core::clone::Clone for DISC_RECORDER_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISC_RECORDER_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DISC_RECORDER_STATE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DISC_RECORDER_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISC_RECORDER_STATE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmulationType(pub i32);
impl ::core::marker::Copy for EmulationType {}
impl ::core::clone::Clone for EmulationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmulationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EmulationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EmulationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmulationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsiFileSystems(pub i32);
impl ::core::marker::Copy for FsiFileSystems {}
impl ::core::clone::Clone for FsiFileSystems {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsiFileSystems {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FsiFileSystems {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FsiFileSystems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsiFileSystems").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsiItemType(pub i32);
impl ::core::marker::Copy for FsiItemType {}
impl ::core::clone::Clone for FsiItemType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsiItemType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FsiItemType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FsiItemType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsiItemType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_BURN_VERIFICATION_LEVEL(pub i32);
impl ::core::marker::Copy for IMAPI_BURN_VERIFICATION_LEVEL {}
impl ::core::clone::Clone for IMAPI_BURN_VERIFICATION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_BURN_VERIFICATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_BURN_VERIFICATION_LEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_BURN_VERIFICATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_BURN_VERIFICATION_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_CD_SECTOR_TYPE(pub i32);
impl ::core::marker::Copy for IMAPI_CD_SECTOR_TYPE {}
impl ::core::clone::Clone for IMAPI_CD_SECTOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_CD_SECTOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_CD_SECTOR_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_CD_SECTOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_CD_SECTOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(pub i32);
impl ::core::marker::Copy for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {}
impl ::core::clone::Clone for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_CD_TRACK_DIGITAL_COPY_SETTING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FEATURE_PAGE_TYPE(pub i32);
impl ::core::marker::Copy for IMAPI_FEATURE_PAGE_TYPE {}
impl ::core::clone::Clone for IMAPI_FEATURE_PAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FEATURE_PAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_FEATURE_PAGE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_FEATURE_PAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FEATURE_PAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FORMAT2_DATA_MEDIA_STATE(pub i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_DATA_MEDIA_STATE {}
impl ::core::clone::Clone for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_DATA_MEDIA_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FORMAT2_DATA_WRITE_ACTION(pub i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_DATA_WRITE_ACTION {}
impl ::core::clone::Clone for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_DATA_WRITE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(pub i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {}
impl ::core::clone::Clone for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(pub i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {}
impl ::core::clone::Clone for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_RAW_CD_WRITE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FORMAT2_TAO_WRITE_ACTION(pub i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_TAO_WRITE_ACTION {}
impl ::core::clone::Clone for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_TAO_WRITE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_MEDIA_PHYSICAL_TYPE(pub i32);
impl ::core::marker::Copy for IMAPI_MEDIA_PHYSICAL_TYPE {}
impl ::core::clone::Clone for IMAPI_MEDIA_PHYSICAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_MEDIA_PHYSICAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_MEDIA_PHYSICAL_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_MEDIA_PHYSICAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_MEDIA_PHYSICAL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_MEDIA_WRITE_PROTECT_STATE(pub i32);
impl ::core::marker::Copy for IMAPI_MEDIA_WRITE_PROTECT_STATE {}
impl ::core::clone::Clone for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_MEDIA_WRITE_PROTECT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_MODE_PAGE_REQUEST_TYPE(pub i32);
impl ::core::marker::Copy for IMAPI_MODE_PAGE_REQUEST_TYPE {}
impl ::core::clone::Clone for IMAPI_MODE_PAGE_REQUEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_MODE_PAGE_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_MODE_PAGE_REQUEST_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_MODE_PAGE_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_MODE_PAGE_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_MODE_PAGE_TYPE(pub i32);
impl ::core::marker::Copy for IMAPI_MODE_PAGE_TYPE {}
impl ::core::clone::Clone for IMAPI_MODE_PAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_MODE_PAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_MODE_PAGE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_MODE_PAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_MODE_PAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_PROFILE_TYPE(pub i32);
impl ::core::marker::Copy for IMAPI_PROFILE_TYPE {}
impl ::core::clone::Clone for IMAPI_PROFILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_PROFILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_PROFILE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_PROFILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_PROFILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_READ_TRACK_ADDRESS_TYPE(pub i32);
impl ::core::marker::Copy for IMAPI_READ_TRACK_ADDRESS_TYPE {}
impl ::core::clone::Clone for IMAPI_READ_TRACK_ADDRESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_READ_TRACK_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMAPI_READ_TRACK_ADDRESS_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMAPI_READ_TRACK_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_READ_TRACK_ADDRESS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_CPV_ENUM(pub i32);
impl ::core::marker::Copy for IMMPID_CPV_ENUM {}
impl ::core::clone::Clone for IMMPID_CPV_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_CPV_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMMPID_CPV_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMMPID_CPV_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_CPV_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_MPV_ENUM(pub i32);
impl ::core::marker::Copy for IMMPID_MPV_ENUM {}
impl ::core::clone::Clone for IMMPID_MPV_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_MPV_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMMPID_MPV_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMMPID_MPV_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_MPV_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_MP_ENUM(pub i32);
impl ::core::marker::Copy for IMMPID_MP_ENUM {}
impl ::core::clone::Clone for IMMPID_MP_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_MP_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMMPID_MP_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMMPID_MP_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_MP_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_NMP_ENUM(pub i32);
impl ::core::marker::Copy for IMMPID_NMP_ENUM {}
impl ::core::clone::Clone for IMMPID_NMP_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_NMP_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMMPID_NMP_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMMPID_NMP_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_NMP_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_RPV_ENUM(pub i32);
impl ::core::marker::Copy for IMMPID_RPV_ENUM {}
impl ::core::clone::Clone for IMMPID_RPV_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_RPV_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMMPID_RPV_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMMPID_RPV_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_RPV_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_RP_ENUM(pub i32);
impl ::core::marker::Copy for IMMPID_RP_ENUM {}
impl ::core::clone::Clone for IMMPID_RP_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_RP_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMMPID_RP_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMMPID_RP_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_RP_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEDIA_FLAGS(pub i32);
impl ::core::marker::Copy for MEDIA_FLAGS {}
impl ::core::clone::Clone for MEDIA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEDIA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MEDIA_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MEDIA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEDIA_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEDIA_TYPES(pub i32);
impl ::core::marker::Copy for MEDIA_TYPES {}
impl ::core::clone::Clone for MEDIA_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEDIA_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MEDIA_TYPES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MEDIA_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEDIA_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlatformId(pub i32);
impl ::core::marker::Copy for PlatformId {}
impl ::core::clone::Clone for PlatformId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformId {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PlatformId {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlatformId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformId").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RECORDER_TYPES(pub i32);
impl ::core::marker::Copy for RECORDER_TYPES {}
impl ::core::clone::Clone for RECORDER_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RECORDER_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RECORDER_TYPES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RECORDER_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RECORDER_TYPES").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct IMMP_MPV_STORE_DRIVER_HANDLE {
    pub guidSignature: ::windows_core::GUID,
}
impl ::core::marker::Copy for IMMP_MPV_STORE_DRIVER_HANDLE {}
impl ::core::clone::Clone for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMMP_MPV_STORE_DRIVER_HANDLE").field("guidSignature", &self.guidSignature).finish()
    }
}
impl ::windows_core::TypeKind for IMMP_MPV_STORE_DRIVER_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.guidSignature == other.guidSignature
    }
}
impl ::core::cmp::Eq for IMMP_MPV_STORE_DRIVER_HANDLE {}
impl ::core::default::Default for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LPMSGSESS(pub isize);
impl ::core::default::Default for LPMSGSESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for LPMSGSESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for LPMSGSESS {}
impl ::core::fmt::Debug for LPMSGSESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LPMSGSESS").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for LPMSGSESS {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct SPropAttrArray {
    pub cValues: u32,
    pub aPropAttr: [u32; 1],
}
impl ::core::marker::Copy for SPropAttrArray {}
impl ::core::clone::Clone for SPropAttrArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPropAttrArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPropAttrArray").field("cValues", &self.cValues).field("aPropAttr", &self.aPropAttr).finish()
    }
}
impl ::windows_core::TypeKind for SPropAttrArray {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SPropAttrArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.aPropAttr == other.aPropAttr
    }
}
impl ::core::cmp::Eq for SPropAttrArray {}
impl ::core::default::Default for SPropAttrArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct tagIMMPID_GUIDLIST_ITEM {
    pub pguid: *const ::windows_core::GUID,
    pub dwStart: u32,
    pub dwLast: u32,
}
impl ::core::marker::Copy for tagIMMPID_GUIDLIST_ITEM {}
impl ::core::clone::Clone for tagIMMPID_GUIDLIST_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tagIMMPID_GUIDLIST_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tagIMMPID_GUIDLIST_ITEM").field("pguid", &self.pguid).field("dwStart", &self.dwStart).field("dwLast", &self.dwLast).finish()
    }
}
impl ::windows_core::TypeKind for tagIMMPID_GUIDLIST_ITEM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for tagIMMPID_GUIDLIST_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.pguid == other.pguid && self.dwStart == other.dwStart && self.dwLast == other.dwLast
    }
}
impl ::core::cmp::Eq for tagIMMPID_GUIDLIST_ITEM {}
impl ::core::default::Default for tagIMMPID_GUIDLIST_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "Required features: `\"Win32_System_AddressBook\"`"]
#[cfg(feature = "Win32_System_AddressBook")]
pub type MSGCALLRELEASE = ::core::option::Option<unsafe extern "system" fn(ulcallerdata: u32, lpmessage: ::core::option::Option<super::super::System::AddressBook::IMessage>) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
