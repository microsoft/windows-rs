#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct APPLICATION_EVENT_DATA {
    pub cbApplicationEventData: u32,
    pub ApplicationId: ::windows::runtime::GUID,
    pub EndpointId: ::windows::runtime::GUID,
    pub dwEventId: u32,
    pub cbEventData: u32,
    pub bEventData: [u8; 1],
}
impl APPLICATION_EVENT_DATA {}
impl ::std::default::Default for APPLICATION_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for APPLICATION_EVENT_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for APPLICATION_EVENT_DATA {}
unsafe impl ::windows::runtime::Abi for APPLICATION_EVENT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CONTENT_ID_GLANCE: u32 = 0u32;
pub const CONTENT_ID_HOME: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct CONTENT_MISSING_EVENT_DATA {
    pub cbContentMissingEventData: u32,
    pub ApplicationId: ::windows::runtime::GUID,
    pub EndpointId: ::windows::runtime::GUID,
    pub ContentId: u32,
}
impl CONTENT_MISSING_EVENT_DATA {}
impl ::std::default::Default for CONTENT_MISSING_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CONTENT_MISSING_EVENT_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CONTENT_MISSING_EVENT_DATA {}
unsafe impl ::windows::runtime::Abi for CONTENT_MISSING_EVENT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct DEVICE_USER_CHANGE_EVENT_DATA {
    pub cbDeviceUserChangeEventData: u32,
    pub wszUser: u16,
}
impl DEVICE_USER_CHANGE_EVENT_DATA {}
impl ::std::default::Default for DEVICE_USER_CHANGE_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DEVICE_USER_CHANGE_EVENT_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DEVICE_USER_CHANGE_EVENT_DATA {}
unsafe impl ::windows::runtime::Abi for DEVICE_USER_CHANGE_EVENT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct EVENT_DATA_HEADER {
    pub cbEventDataHeader: u32,
    pub guidEventType: ::windows::runtime::GUID,
    pub dwVersion: u32,
    pub cbEventDataSid: u32,
}
impl EVENT_DATA_HEADER {}
impl ::std::default::Default for EVENT_DATA_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_DATA_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_DATA_HEADER {}
unsafe impl ::windows::runtime::Abi for EVENT_DATA_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const GUID_DEVINTERFACE_SIDESHOW: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        355358737,
        65209,
        19200,
        [144, 244, 211, 41, 71, 174, 22, 129],
    );
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowBulkCapabilities(::windows::runtime::IUnknown);
impl ISideShowBulkCapabilities {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_PropertiesSystem",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetCapability(
        &self,
        in_keycapability: *const super::PropertiesSystem::PROPERTYKEY,
        inout_pvalue: *mut super::super::Storage::StructuredStorage::PROPVARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(in_keycapability),
            ::std::mem::transmute(inout_pvalue),
        )
        .ok()
    }
    pub unsafe fn GetCapabilities<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ISideShowKeyCollection>,
    >(
        &self,
        in_keycollection: Param0,
        inout_pvalues: *mut ::std::option::Option<ISideShowPropVariantCollection>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            in_keycollection.into_param().abi(),
            ::std::mem::transmute(inout_pvalues),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowBulkCapabilities {
    type Vtable = ISideShowBulkCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        975929276,
        15061,
        18621,
        [187, 241, 14, 108, 251, 209, 8, 7],
    );
}
impl ::std::convert::From<ISideShowBulkCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowBulkCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowBulkCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowBulkCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISideShowBulkCapabilities
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISideShowBulkCapabilities
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ISideShowBulkCapabilities> for ISideShowCapabilities {
    fn from(value: ISideShowBulkCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowBulkCapabilities> for ISideShowCapabilities {
    fn from(value: &ISideShowBulkCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISideShowCapabilities> for ISideShowBulkCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISideShowCapabilities> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISideShowCapabilities>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISideShowCapabilities> for &ISideShowBulkCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISideShowCapabilities> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISideShowCapabilities>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowBulkCapabilities_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_PropertiesSystem",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_keycapability: *const super::PropertiesSystem::PROPERTYKEY,
        inout_pvalue: *mut ::std::mem::ManuallyDrop<
            super::super::Storage::StructuredStorage::PROPVARIANT,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_PropertiesSystem",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_keycollection: ::windows::runtime::RawPtr,
        inout_pvalues: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowCapabilities(::windows::runtime::IUnknown);
impl ISideShowCapabilities {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_PropertiesSystem",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetCapability(
        &self,
        in_keycapability: *const super::PropertiesSystem::PROPERTYKEY,
        inout_pvalue: *mut super::super::Storage::StructuredStorage::PROPVARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(in_keycapability),
            ::std::mem::transmute(inout_pvalue),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowCapabilities {
    type Vtable = ISideShowCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1398674297,
        49310,
        19028,
        [165, 17, 89, 123, 171, 58, 114, 184],
    );
}
impl ::std::convert::From<ISideShowCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISideShowCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISideShowCapabilities
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowCapabilities_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_PropertiesSystem",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_keycapability: *const super::PropertiesSystem::PROPERTYKEY,
        inout_pvalue: *mut ::std::mem::ManuallyDrop<
            super::super::Storage::StructuredStorage::PROPVARIANT,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_PropertiesSystem",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowCapabilitiesCollection(::windows::runtime::IUnknown);
impl ISideShowCapabilitiesCollection {
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(
        &self,
        in_dwindex: u32,
    ) -> ::windows::runtime::Result<ISideShowCapabilities> {
        let mut result__: <ISideShowCapabilities as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(in_dwindex),
            &mut result__,
        )
        .from_abi::<ISideShowCapabilities>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowCapabilitiesCollection {
    type Vtable = ISideShowCapabilitiesCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1345344919,
        24077,
        20471,
        [179, 175, 51, 208, 217, 189, 82, 221],
    );
}
impl ::std::convert::From<ISideShowCapabilitiesCollection> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowCapabilitiesCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowCapabilitiesCollection> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowCapabilitiesCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISideShowCapabilitiesCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISideShowCapabilitiesCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowCapabilitiesCollection_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        out_pdwcount: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_dwindex: u32,
        out_ppcapabilities: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowContent(::windows::runtime::IUnknown);
impl ISideShowContent {
    pub unsafe fn GetContent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ISideShowCapabilities>,
    >(
        &self,
        in_picapabilities: Param0,
        out_pdwsize: *mut u32,
        out_ppbdata: *mut *mut u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            in_picapabilities.into_param().abi(),
            ::std::mem::transmute(out_pdwsize),
            ::std::mem::transmute(out_ppbdata),
        )
        .ok()
    }
    pub unsafe fn ContentId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DifferentiateContent(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowContent {
    type Vtable = ISideShowContent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3246740205,
        29951,
        20460,
        [190, 7, 76, 254, 210, 157, 72, 135],
    );
}
impl ::std::convert::From<ISideShowContent> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowContent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowContent> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowContent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISideShowContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISideShowContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowContent_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_picapabilities: ::windows::runtime::RawPtr,
        out_pdwsize: *mut u32,
        out_ppbdata: *mut *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        out_pcontentid: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        out_pfdifferentiatecontent: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowContentManager(::windows::runtime::IUnknown);
impl ISideShowContentManager {
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, ISideShowContent>>(
        &self,
        in_picontent: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            in_picontent.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Remove(&self, in_contentid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(in_contentid),
        )
        .ok()
    }
    pub unsafe fn RemoveAll(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetEventSink<'a, Param0: ::windows::runtime::IntoParam<'a, ISideShowEvents>>(
        &self,
        in_pievents: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            in_pievents.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDeviceCapabilities(
        &self,
    ) -> ::windows::runtime::Result<ISideShowCapabilitiesCollection> {
        let mut result__: <ISideShowCapabilitiesCollection as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<ISideShowCapabilitiesCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowContentManager {
    type Vtable = ISideShowContentManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2782246507,
        61177,
        16859,
        [141, 126, 225, 124, 51, 171, 16, 176],
    );
}
impl ::std::convert::From<ISideShowContentManager> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowContentManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowContentManager> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowContentManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISideShowContentManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISideShowContentManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowContentManager_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_picontent: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_contentid: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_pievents: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        out_ppcollection: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowEvents(::windows::runtime::IUnknown);
impl ISideShowEvents {
    pub unsafe fn ContentMissing(
        &self,
        in_contentid: u32,
    ) -> ::windows::runtime::Result<ISideShowContent> {
        let mut result__: <ISideShowContent as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(in_contentid),
            &mut result__,
        )
        .from_abi::<ISideShowContent>(result__)
    }
    pub unsafe fn ApplicationEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ISideShowCapabilities>,
    >(
        &self,
        in_picapabilities: Param0,
        in_dweventid: u32,
        in_dweventsize: u32,
        in_pbeventdata: *const u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            in_picapabilities.into_param().abi(),
            ::std::mem::transmute(in_dweventid),
            ::std::mem::transmute(in_dweventsize),
            ::std::mem::transmute(in_pbeventdata),
        )
        .ok()
    }
    pub unsafe fn DeviceAdded<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ISideShowCapabilities>,
    >(
        &self,
        in_pidevice: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            in_pidevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DeviceRemoved<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ISideShowCapabilities>,
    >(
        &self,
        in_pidevice: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            in_pidevice.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowEvents {
    type Vtable = ISideShowEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1644087884,
        57012,
        19070,
        [141, 117, 81, 241, 19, 45, 97, 91],
    );
}
impl ::std::convert::From<ISideShowEvents> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowEvents> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISideShowEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISideShowEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowEvents_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_contentid: u32,
        out_ppicontent: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_picapabilities: ::windows::runtime::RawPtr,
        in_dweventid: u32,
        in_dweventsize: u32,
        in_pbeventdata: *const u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_pidevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_pidevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowKeyCollection(::windows::runtime::IUnknown);
impl ISideShowKeyCollection {
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    pub unsafe fn Add(
        &self,
        key: *const super::PropertiesSystem::PROPERTYKEY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(key),
        )
        .ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    pub unsafe fn GetAt(
        &self,
        dwindex: u32,
        pkey: *mut super::PropertiesSystem::PROPERTYKEY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(pkey),
        )
        .ok()
    }
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcelems),
        )
        .ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwindex),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowKeyCollection {
    type Vtable = ISideShowKeyCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        72643516,
        41851,
        18775,
        [177, 68, 104, 16, 84, 17, 237, 142],
    );
}
impl ::std::convert::From<ISideShowKeyCollection> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowKeyCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowKeyCollection> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowKeyCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISideShowKeyCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISideShowKeyCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowKeyCollection_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        key: *const super::PropertiesSystem::PROPERTYKEY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwindex: u32,
        pkey: *mut super::PropertiesSystem::PROPERTYKEY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcelems: *const u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwindex: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowNotification(::windows::runtime::IUnknown);
impl ISideShowNotification {
    pub unsafe fn NotificationId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotificationId(
        &self,
        in_notificationid: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(in_notificationid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTitle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        in_pwsztitle: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            in_pwsztitle.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Message(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        in_pwszmessage: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            in_pwszmessage.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn Image(
        &self,
    ) -> ::windows::runtime::Result<super::super::UI::WindowsAndMessaging::HICON> {
        let mut result__ : < super::super::UI::WindowsAndMessaging:: HICON as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::UI::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetImage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HICON>,
    >(
        &self,
        in_hicon: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            in_hicon.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpirationTime(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: <super::super::Foundation::SYSTEMTIME as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExpirationTime(
        &self,
        in_ptime: *const super::super::Foundation::SYSTEMTIME,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(in_ptime),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowNotification {
    type Vtable = ISideShowNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        63517440,
        35506,
        16837,
        [155, 121, 70, 18, 122, 48, 225, 72],
    );
}
impl ::std::convert::From<ISideShowNotification> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowNotification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowNotification> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowNotification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISideShowNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISideShowNotification
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowNotification_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        out_pnotificationid: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_notificationid: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        out_ppwsztitle: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_pwsztitle: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        out_ppwszmessage: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_pwszmessage: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_hicon: super::super::UI::WindowsAndMessaging::HICON,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        out_ptime: *mut super::super::Foundation::SYSTEMTIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_ptime: *const super::super::Foundation::SYSTEMTIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowNotificationManager(::windows::runtime::IUnknown);
impl ISideShowNotificationManager {
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, ISideShowNotification>>(
        &self,
        in_pinotification: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            in_pinotification.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Revoke(&self, in_notificationid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(in_notificationid),
        )
        .ok()
    }
    pub unsafe fn RevokeAll(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowNotificationManager {
    type Vtable = ISideShowNotificationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1674488073,
        62137,
        17154,
        [181, 225, 198, 142, 109, 154, 184, 51],
    );
}
impl ::std::convert::From<ISideShowNotificationManager> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowNotificationManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowNotificationManager> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowNotificationManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISideShowNotificationManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISideShowNotificationManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowNotificationManager_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_pinotification: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_notificationid: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowPropVariantCollection(::windows::runtime::IUnknown);
impl ISideShowPropVariantCollection {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Add(
        &self,
        pvalue: *const super::super::Storage::StructuredStorage::PROPVARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvalue),
        )
        .ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetAt(
        &self,
        dwindex: u32,
        pvalue: *mut super::super::Storage::StructuredStorage::PROPVARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(pvalue),
        )
        .ok()
    }
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcelems),
        )
        .ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwindex),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowPropVariantCollection {
    type Vtable = ISideShowPropVariantCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        782738761,
        31743,
        19118,
        [186, 176, 34, 212, 49, 17, 222, 73],
    );
}
impl ::std::convert::From<ISideShowPropVariantCollection> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowPropVariantCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowPropVariantCollection> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowPropVariantCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISideShowPropVariantCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISideShowPropVariantCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowPropVariantCollection_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvalue: *const ::std::mem::ManuallyDrop<
            super::super::Storage::StructuredStorage::PROPVARIANT,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwindex: u32,
        pvalue: *mut ::std::mem::ManuallyDrop<super::super::Storage::StructuredStorage::PROPVARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_StructuredStorage",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcelems: *const u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwindex: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISideShowSession(::windows::runtime::IUnknown);
impl ISideShowSession {
    pub unsafe fn RegisterContent(
        &self,
        in_applicationid: *const ::windows::runtime::GUID,
        in_endpointid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<ISideShowContentManager> {
        let mut result__: <ISideShowContentManager as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(in_applicationid),
            ::std::mem::transmute(in_endpointid),
            &mut result__,
        )
        .from_abi::<ISideShowContentManager>(result__)
    }
    pub unsafe fn RegisterNotifications(
        &self,
        in_applicationid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<ISideShowNotificationManager> {
        let mut result__: <ISideShowNotificationManager as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(in_applicationid),
            &mut result__,
        )
        .from_abi::<ISideShowNotificationManager>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISideShowSession {
    type Vtable = ISideShowSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3793957358,
        40573,
        18722,
        [159, 194, 171, 122, 164, 28, 228, 145],
    );
}
impl ::std::convert::From<ISideShowSession> for ::windows::runtime::IUnknown {
    fn from(value: ISideShowSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISideShowSession> for ::windows::runtime::IUnknown {
    fn from(value: &ISideShowSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISideShowSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISideShowSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowSession_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_applicationid: *const ::windows::runtime::GUID,
        in_endpointid: *const ::windows::runtime::GUID,
        out_ppicontent: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        in_applicationid: *const ::windows::runtime::GUID,
        out_ppinotification: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct NEW_EVENT_DATA_AVAILABLE {
    pub cbNewEventDataAvailable: u32,
    pub dwVersion: u32,
}
impl NEW_EVENT_DATA_AVAILABLE {}
impl ::std::default::Default for NEW_EVENT_DATA_AVAILABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NEW_EVENT_DATA_AVAILABLE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NEW_EVENT_DATA_AVAILABLE {}
unsafe impl ::windows::runtime::Abi for NEW_EVENT_DATA_AVAILABLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SCF_BUTTON_IDS(pub i32);
pub const SCF_BUTTON_MENU: SCF_BUTTON_IDS = SCF_BUTTON_IDS(1i32);
pub const SCF_BUTTON_SELECT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(2i32);
pub const SCF_BUTTON_UP: SCF_BUTTON_IDS = SCF_BUTTON_IDS(3i32);
pub const SCF_BUTTON_DOWN: SCF_BUTTON_IDS = SCF_BUTTON_IDS(4i32);
pub const SCF_BUTTON_LEFT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(5i32);
pub const SCF_BUTTON_RIGHT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(6i32);
pub const SCF_BUTTON_PLAY: SCF_BUTTON_IDS = SCF_BUTTON_IDS(7i32);
pub const SCF_BUTTON_PAUSE: SCF_BUTTON_IDS = SCF_BUTTON_IDS(8i32);
pub const SCF_BUTTON_FASTFORWARD: SCF_BUTTON_IDS = SCF_BUTTON_IDS(9i32);
pub const SCF_BUTTON_REWIND: SCF_BUTTON_IDS = SCF_BUTTON_IDS(10i32);
pub const SCF_BUTTON_STOP: SCF_BUTTON_IDS = SCF_BUTTON_IDS(11i32);
pub const SCF_BUTTON_BACK: SCF_BUTTON_IDS = SCF_BUTTON_IDS(65280i32);
impl ::std::convert::From<i32> for SCF_BUTTON_IDS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCF_BUTTON_IDS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCF_CONTEXTMENU_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub PreviousItemId: u32,
    pub MenuPage: u32,
    pub MenuItemId: u32,
}
impl SCF_CONTEXTMENU_EVENT {}
impl ::std::default::Default for SCF_CONTEXTMENU_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCF_CONTEXTMENU_EVENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCF_CONTEXTMENU_EVENT")
            .field("PreviousPage", &self.PreviousPage)
            .field("TargetPage", &self.TargetPage)
            .field("PreviousItemId", &self.PreviousItemId)
            .field("MenuPage", &self.MenuPage)
            .field("MenuItemId", &self.MenuItemId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCF_CONTEXTMENU_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage
            && self.TargetPage == other.TargetPage
            && self.PreviousItemId == other.PreviousItemId
            && self.MenuPage == other.MenuPage
            && self.MenuItemId == other.MenuItemId
    }
}
impl ::std::cmp::Eq for SCF_CONTEXTMENU_EVENT {}
unsafe impl ::windows::runtime::Abi for SCF_CONTEXTMENU_EVENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCF_EVENT_HEADER {
    pub PreviousPage: u32,
    pub TargetPage: u32,
}
impl SCF_EVENT_HEADER {}
impl ::std::default::Default for SCF_EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCF_EVENT_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCF_EVENT_HEADER")
            .field("PreviousPage", &self.PreviousPage)
            .field("TargetPage", &self.TargetPage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCF_EVENT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage && self.TargetPage == other.TargetPage
    }
}
impl ::std::cmp::Eq for SCF_EVENT_HEADER {}
unsafe impl ::windows::runtime::Abi for SCF_EVENT_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SCF_EVENT_IDS(pub i32);
pub const SCF_EVENT_NAVIGATION: SCF_EVENT_IDS = SCF_EVENT_IDS(1i32);
pub const SCF_EVENT_MENUACTION: SCF_EVENT_IDS = SCF_EVENT_IDS(2i32);
pub const SCF_EVENT_CONTEXTMENU: SCF_EVENT_IDS = SCF_EVENT_IDS(3i32);
impl ::std::convert::From<i32> for SCF_EVENT_IDS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCF_EVENT_IDS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCF_MENUACTION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
    pub ItemId: u32,
}
impl SCF_MENUACTION_EVENT {}
impl ::std::default::Default for SCF_MENUACTION_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCF_MENUACTION_EVENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCF_MENUACTION_EVENT")
            .field("PreviousPage", &self.PreviousPage)
            .field("TargetPage", &self.TargetPage)
            .field("Button", &self.Button)
            .field("ItemId", &self.ItemId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCF_MENUACTION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage
            && self.TargetPage == other.TargetPage
            && self.Button == other.Button
            && self.ItemId == other.ItemId
    }
}
impl ::std::cmp::Eq for SCF_MENUACTION_EVENT {}
unsafe impl ::windows::runtime::Abi for SCF_MENUACTION_EVENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCF_NAVIGATION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
}
impl SCF_NAVIGATION_EVENT {}
impl ::std::default::Default for SCF_NAVIGATION_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCF_NAVIGATION_EVENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCF_NAVIGATION_EVENT")
            .field("PreviousPage", &self.PreviousPage)
            .field("TargetPage", &self.TargetPage)
            .field("Button", &self.Button)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCF_NAVIGATION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage
            && self.TargetPage == other.TargetPage
            && self.Button == other.Button
    }
}
impl ::std::cmp::Eq for SCF_NAVIGATION_EVENT {}
unsafe impl ::windows::runtime::Abi for SCF_NAVIGATION_EVENT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SIDESHOW_APPLICATION_EVENT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1286959866,
        7483,
        18867,
        [161, 122, 46, 107, 255, 5, 40, 84],
    );
pub const SIDESHOW_CAPABILITY_DEVICE_PROPERTIES: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2327611560,
        34171,
        19159,
        [163, 90, 181, 148, 47, 73, 43, 153],
    );
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SIDESHOW_COLOR_TYPE(pub i32);
pub const SIDESHOW_COLOR_TYPE_COLOR: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(0i32);
pub const SIDESHOW_COLOR_TYPE_GREYSCALE: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(1i32);
pub const SIDESHOW_COLOR_TYPE_BLACK_AND_WHITE: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(2i32);
impl ::std::convert::From<i32> for SIDESHOW_COLOR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SIDESHOW_COLOR_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SIDESHOW_CONTENT_MISSING_EVENT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1342700456,
        54035,
        17311,
        [190, 162, 165, 2, 1, 211, 233, 168],
    );
pub const SIDESHOW_ENDPOINT_ICAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1308571317,
    40414,
    20342,
    [154, 42, 150, 67, 80, 71, 6, 61],
);
pub const SIDESHOW_ENDPOINT_SIMPLE_CONTENT_FORMAT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2846176575,
        11595,
        18382,
        [147, 238, 117, 159, 58, 125, 218, 79],
    );
pub const SIDESHOW_EVENTID_APPLICATION_ENTER: u32 = 4294901760u32;
pub const SIDESHOW_EVENTID_APPLICATION_EXIT: u32 = 4294901761u32;
pub const SIDESHOW_NEW_EVENT_DATA_AVAILABLE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1468086356,
        12225,
        16668,
        [165, 159, 242, 73, 39, 96, 136, 4],
    );
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SIDESHOW_SCREEN_TYPE(pub i32);
pub const SIDESHOW_SCREEN_TYPE_BITMAP: SIDESHOW_SCREEN_TYPE = SIDESHOW_SCREEN_TYPE(0i32);
pub const SIDESHOW_SCREEN_TYPE_TEXT: SIDESHOW_SCREEN_TYPE = SIDESHOW_SCREEN_TYPE(1i32);
impl ::std::convert::From<i32> for SIDESHOW_SCREEN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SIDESHOW_SCREEN_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SIDESHOW_USER_CHANGE_REQUEST_EVENT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1342793532,
        16253,
        19582,
        [153, 113, 234, 162, 233, 31, 21, 117],
    );
pub const SideShowKeyCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3753630712,
    6366,
    18872,
    [131, 220, 235, 199, 39, 198, 45, 148],
);
pub const SideShowNotification: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    216262767,
    54733,
    17701,
    [167, 102, 26, 186, 177, 167, 82, 245],
);
pub const SideShowPropVariantCollection: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3863016469,
        21406,
        18723,
        [150, 205, 95, 9, 59, 194, 80, 205],
    );
pub const SideShowSession: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3791995833,
    63365,
    20130,
    [152, 30, 196, 255, 167, 107, 188, 124],
);
pub const VERSION_1_WINDOWS_7: u32 = 0u32;
