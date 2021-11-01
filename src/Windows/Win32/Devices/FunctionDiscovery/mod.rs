#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_AUTHFAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193917i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_AUTHNOTALLOWED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193914i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_CONNECTTIMEOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193916i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_HWFAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193918i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_IPBUSDISABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193913i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_NOCONNECTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193919i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_NOPROFILES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193912i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const E_FDPAIRING_TOOMANYCONNECTIONS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1882193915i32 as _);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID: u32 = 1000u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_ASYNCTHREADEXIT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_IPADDRESSCHANGE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_PRIVATE: u32 = 100u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_QUERYREFRESH: u32 = 1004u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_SEARCHCOMPLETE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_EVENTID_SEARCHSTART: u32 = 1002u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_LONGHORN: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_Visibility_Default: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const FD_Visibility_Hidden: u32 = 1u32;
pub const FMTID_Device: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]);
pub const FMTID_DeviceInterface: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1400930312, 1979, 18017, [188, 60, 181, 149, 62, 112, 133, 96]);
pub const FMTID_FD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2420835234, 18205, 16956, [165, 132, 243, 72, 50, 56, 161, 70]);
pub const FMTID_PNPX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1701460915, 60608, 17405, [132, 119, 74, 224, 64, 74, 150, 205]);
pub const FMTID_PNPXDynamicProperty: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1338312574, 46726, 17598, [147, 227, 134, 202, 254, 54, 140, 205]);
pub const FMTID_Pairing: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2282212070, 32182, 20240, [142, 228, 67, 94, 170, 19, 146, 188]);
pub const FMTID_WSD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2454742161, 65429, 18212, [160, 90, 91, 129, 136, 90, 124, 146]);
pub const FunctionDiscovery: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3341542124, 36496, 17708, [178, 154, 171, 143, 241, 192, 113, 252]);
pub const FunctionInstanceCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3129052389, 46431, 17471, [173, 57, 47, 232, 155, 230, 25, 31]);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFunctionDiscovery(::windows::runtime::IUnknown);
impl IFunctionDiscovery {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn GetInstanceCollection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszcategory: Param0, pszsubcategory: Param1, fincludeallsubcategories: Param2) -> ::windows::runtime::Result<IFunctionInstanceCollection> {
        let mut result__: <IFunctionInstanceCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), fincludeallsubcategories.into_param().abi(), &mut result__).from_abi::<IFunctionInstanceCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn GetInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfunctioninstanceidentity: Param0) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszfunctioninstanceidentity.into_param().abi(), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstanceCollectionQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(
        &self,
        pszcategory: Param0,
        pszsubcategory: Param1,
        fincludeallsubcategories: Param2,
        pifunctiondiscoverynotification: Param3,
        pfdqcquerycontext: *mut u64,
        ppifunctioninstancecollectionquery: *mut ::std::option::Option<IFunctionInstanceCollectionQuery>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), fincludeallsubcategories.into_param().abi(), pifunctiondiscoverynotification.into_param().abi(), ::std::mem::transmute(pfdqcquerycontext), ::std::mem::transmute(ppifunctioninstancecollectionquery)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstanceQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszfunctioninstanceidentity: Param0, pifunctiondiscoverynotification: Param1, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::std::option::Option<IFunctionInstanceQuery>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pszfunctioninstanceidentity.into_param().abi(), pifunctiondiscoverynotification.into_param().abi(), ::std::mem::transmute(pfdqcquerycontext), ::std::mem::transmute(ppifunctioninstancequery)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn AddInstance<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: Param1, pszsubcategory: Param2, pszcategoryidentity: Param3) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(enumsystemvisibility), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), pszcategoryidentity.into_param().abi(), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn RemoveInstance<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: Param1, pszsubcategory: Param2, pszcategoryidentity: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(enumsystemvisibility), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), pszcategoryidentity.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscovery {
    type Vtable = IFunctionDiscovery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1308203888, 57672, 17458, [176, 4, 76, 158, 235, 83, 90, 94]);
}
impl ::std::convert::From<IFunctionDiscovery> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscovery) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFunctionDiscovery> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscovery) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscovery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFunctionDiscovery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscovery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, ppifunctioninstancecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: ::windows::runtime::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::runtime::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFunctionDiscoveryNotification(::windows::runtime::IUnknown);
impl IFunctionDiscoveryNotification {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn OnUpdate<'a, Param2: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(enumqueryupdateaction), ::std::mem::transmute(fdqcquerycontext), pifunctioninstance.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn OnError<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hr: ::windows::runtime::HRESULT, fdqcquerycontext: u64, pszprovider: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr), ::std::mem::transmute(fdqcquerycontext), pszprovider.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn OnEvent<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dweventid: u32, fdqcquerycontext: u64, pszprovider: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dweventid), ::std::mem::transmute(fdqcquerycontext), pszprovider.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscoveryNotification {
    type Vtable = IFunctionDiscoveryNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1600920488, 21296, 16942, [163, 104, 87, 43, 36, 77, 63, 135]);
}
impl ::std::convert::From<IFunctionDiscoveryNotification> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscoveryNotification) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFunctionDiscoveryNotification> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscoveryNotification) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscoveryNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFunctionDiscoveryNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dweventid: u32, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFunctionDiscoveryProvider(::windows::runtime::IUnknown);
impl IFunctionDiscoveryProvider {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryProviderFactory>, Param1: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pifunctiondiscoveryproviderfactory: Param0, pifunctiondiscoverynotification: Param1, lciduserdefault: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pifunctiondiscoveryproviderfactory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi(), ::std::mem::transmute(lciduserdefault), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Query<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryProviderQuery>>(&self, pifunctiondiscoveryproviderquery: Param0) -> ::windows::runtime::Result<IFunctionInstanceCollection> {
        let mut result__: <IFunctionInstanceCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pifunctiondiscoveryproviderquery.into_param().abi(), &mut result__).from_abi::<IFunctionInstanceCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn EndQuery(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn InstancePropertyStoreValidateAccess<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::std::mem::transmute(iproviderinstancecontext), ::std::mem::transmute(dwstgaccess)).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn InstancePropertyStoreOpen<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::runtime::Result<super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::std::mem::transmute(iproviderinstancecontext), ::std::mem::transmute(dwstgaccess), &mut result__).from_abi::<super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn InstancePropertyStoreFlush<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::std::mem::transmute(iproviderinstancecontext)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn InstanceQueryService<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::std::mem::transmute(iproviderinstancecontext), ::std::mem::transmute(guidservice), ::std::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn InstanceReleased<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::std::mem::transmute(iproviderinstancecontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscoveryProvider {
    type Vtable = IFunctionDiscoveryProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3705551183, 5240, 18451, [164, 2, 246, 251, 16, 101, 114, 34]);
}
impl ::std::convert::From<IFunctionDiscoveryProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscoveryProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFunctionDiscoveryProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscoveryProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscoveryProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFunctionDiscoveryProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctiondiscoveryproviderfactory: ::windows::runtime::RawPtr, pifunctiondiscoverynotification: ::windows::runtime::RawPtr, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctiondiscoveryproviderquery: ::windows::runtime::RawPtr, ppifunctioninstancecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppiunknown: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFunctionDiscoveryProviderFactory(::windows::runtime::IUnknown);
impl IFunctionDiscoveryProviderFactory {
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn CreatePropertyStore(&self) -> ::windows::runtime::Result<super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::System::PropertiesSystem::IPropertyStore>, Param4: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryProvider>>(
        &self,
        pszsubcategory: Param0,
        pszproviderinstanceidentity: Param1,
        iproviderinstancecontext: isize,
        pipropertystore: Param3,
        pifunctiondiscoveryprovider: Param4,
    ) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszsubcategory.into_param().abi(), pszproviderinstanceidentity.into_param().abi(), ::std::mem::transmute(iproviderinstancecontext), pipropertystore.into_param().abi(), pifunctiondiscoveryprovider.into_param().abi(), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn CreateFunctionInstanceCollection(&self) -> ::windows::runtime::Result<IFunctionInstanceCollection> {
        let mut result__: <IFunctionInstanceCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFunctionInstanceCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscoveryProviderFactory {
    type Vtable = IFunctionDiscoveryProviderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2252619760, 6869, 20072, [164, 90, 64, 194, 195, 41, 222, 59]);
}
impl ::std::convert::From<IFunctionDiscoveryProviderFactory> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscoveryProviderFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFunctionDiscoveryProviderFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscoveryProviderFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscoveryProviderFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFunctionDiscoveryProviderFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProviderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, iproviderinstancecontext: isize, pipropertystore: ::windows::runtime::RawPtr, pifunctiondiscoveryprovider: ::windows::runtime::RawPtr, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppifunctioninstancecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFunctionDiscoveryProviderQuery(::windows::runtime::IUnknown);
impl IFunctionDiscoveryProviderQuery {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn IsInstanceQuery(&self, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pisinstancequery), ::std::mem::transmute(ppszconstraintvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn IsSubcategoryQuery(&self, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pissubcategoryquery), ::std::mem::transmute(ppszconstraintvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetQueryConstraints(&self) -> ::windows::runtime::Result<IProviderQueryConstraintCollection> {
        let mut result__: <IProviderQueryConstraintCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IProviderQueryConstraintCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetPropertyConstraints(&self) -> ::windows::runtime::Result<IProviderPropertyConstraintCollection> {
        let mut result__: <IProviderPropertyConstraintCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IProviderPropertyConstraintCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscoveryProviderQuery {
    type Vtable = IFunctionDiscoveryProviderQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1752623768, 47852, 18139, [188, 32, 117, 167, 110, 38, 122, 58]);
}
impl ::std::convert::From<IFunctionDiscoveryProviderQuery> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscoveryProviderQuery) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFunctionDiscoveryProviderQuery> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscoveryProviderQuery) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscoveryProviderQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFunctionDiscoveryProviderQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProviderQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiproviderqueryconstraints: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiproviderpropertyconstraints: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFunctionDiscoveryServiceProvider(::windows::runtime::IUnknown);
impl IFunctionDiscoveryServiceProvider {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>, T: ::windows::runtime::Interface>(&self, pifunctioninstance: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionDiscoveryServiceProvider {
    type Vtable = IFunctionDiscoveryServiceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1283583234, 6916, 17394, [164, 81, 105, 150, 108, 188, 209, 194]);
}
impl ::std::convert::From<IFunctionDiscoveryServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionDiscoveryServiceProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFunctionDiscoveryServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionDiscoveryServiceProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionDiscoveryServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFunctionDiscoveryServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryServiceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFunctionInstance(::windows::runtime::IUnknown);
impl IFunctionInstance {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidservice), ::std::mem::transmute(riid), ::std::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetID(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetProviderInstanceID(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut u16>(result__)
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn OpenPropertyStore(&self, dwstgaccess: u32) -> ::windows::runtime::Result<super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstgaccess), &mut result__).from_abi::<super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppszcomemcategory), ::std::mem::transmute(ppszcomemsubcategory)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionInstance {
    type Vtable = IFunctionInstance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(861477904, 3053, 20226, [176, 171, 21, 48, 213, 83, 62, 233]);
}
impl ::std::convert::From<IFunctionInstance> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionInstance) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFunctionInstance> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionInstance) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFunctionInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IFunctionInstance> for super::super::System::Com::IServiceProvider {
    fn from(value: IFunctionInstance) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IFunctionInstance> for super::super::System::Com::IServiceProvider {
    fn from(value: &IFunctionInstance) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IServiceProvider> for IFunctionInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IServiceProvider> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Com::IServiceProvider>::into(self))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IServiceProvider> for &IFunctionInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IServiceProvider> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Com::IServiceProvider>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstance_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszcomemidentity: *mut *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszcomemproviderinstanceidentity: *mut *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstgaccess: u32, ppipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFunctionInstanceCollection(::windows::runtime::IUnknown);
impl IFunctionInstanceCollection {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Get<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszinstanceidentity: Param0, pdwindex: *mut u32, ppifunctioninstance: *mut ::std::option::Option<IFunctionInstance>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszinstanceidentity.into_param().abi(), ::std::mem::transmute(pdwindex), ::std::mem::transmute(ppifunctioninstance)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Item(&self, dwindex: u32) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Remove(&self, dwindex: u32) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Delete(&self, dwindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn DeleteAll(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionInstanceCollection {
    type Vtable = IFunctionInstanceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4037269653, 34140, 17058, [148, 141, 47, 151, 212, 80, 236, 177]);
}
impl ::std::convert::From<IFunctionInstanceCollection> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionInstanceCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFunctionInstanceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionInstanceCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionInstanceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFunctionInstanceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFunctionInstanceCollectionQuery(::windows::runtime::IUnknown);
impl IFunctionInstanceCollectionQuery {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn AddQueryConstraint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconstraintname: Param0, pszconstraintvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszconstraintname.into_param().abi(), pszconstraintvalue.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn AddPropertyConstraint(&self, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(pv), ::std::mem::transmute(enumpropertyconstraint)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Execute(&self) -> ::windows::runtime::Result<IFunctionInstanceCollection> {
        let mut result__: <IFunctionInstanceCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFunctionInstanceCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionInstanceCollectionQuery {
    type Vtable = IFunctionInstanceCollectionQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1473015762, 49306, 17033, [187, 114, 37, 240, 65, 66, 5, 142]);
}
impl ::std::convert::From<IFunctionInstanceCollectionQuery> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionInstanceCollectionQuery) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFunctionInstanceCollectionQuery> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionInstanceCollectionQuery) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionInstanceCollectionQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFunctionInstanceCollectionQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollectionQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconstraintname: super::super::Foundation::PWSTR, pszconstraintvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, pv: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, enumpropertyconstraint: PropertyConstraint) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppifunctioninstancecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFunctionInstanceQuery(::windows::runtime::IUnknown);
impl IFunctionInstanceQuery {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Execute(&self) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFunctionInstanceQuery {
    type Vtable = IFunctionInstanceQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1648540779, 37100, 19255, [187, 70, 226, 41, 253, 132, 237, 149]);
}
impl ::std::convert::From<IFunctionInstanceQuery> for ::windows::runtime::IUnknown {
    fn from(value: IFunctionInstanceQuery) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFunctionInstanceQuery> for ::windows::runtime::IUnknown {
    fn from(value: &IFunctionInstanceQuery) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFunctionInstanceQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFunctionInstanceQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPNPXAssociation(::windows::runtime::IUnknown);
impl IPNPXAssociation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Associate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubcategory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszsubcategory.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Unassociate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubcategory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszsubcategory.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubcategory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszsubcategory.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPNPXAssociation {
    type Vtable = IPNPXAssociation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(198698273, 19878, 17109, [129, 186, 25, 129, 182, 185, 64, 117]);
}
impl ::std::convert::From<IPNPXAssociation> for ::windows::runtime::IUnknown {
    fn from(value: IPNPXAssociation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPNPXAssociation> for ::windows::runtime::IUnknown {
    fn from(value: &IPNPXAssociation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPNPXAssociation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPNPXAssociation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPNPXAssociation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPNPXDeviceAssociation(::windows::runtime::IUnknown);
impl IPNPXDeviceAssociation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Associate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszsubcategory: Param0, pifunctiondiscoverynotification: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszsubcategory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Unassociate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszsubcategory: Param0, pifunctiondiscoverynotification: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszsubcategory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszsubcategory: Param0, pifunctiondiscoverynotification: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszsubcategory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPNPXDeviceAssociation {
    type Vtable = IPNPXDeviceAssociation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4006831824, 13752, 20421, [141, 32, 126, 91, 211, 31, 109, 237]);
}
impl ::std::convert::From<IPNPXDeviceAssociation> for ::windows::runtime::IUnknown {
    fn from(value: IPNPXDeviceAssociation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPNPXDeviceAssociation> for ::windows::runtime::IUnknown {
    fn from(value: &IPNPXDeviceAssociation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPNPXDeviceAssociation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPNPXDeviceAssociation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPNPXDeviceAssociation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyStoreCollection(::windows::runtime::IUnknown);
impl IPropertyStoreCollection {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Get<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszinstanceidentity: Param0, pdwindex: *mut u32, ppipropertystore: *mut ::std::option::Option<super::super::System::PropertiesSystem::IPropertyStore>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszinstanceidentity.into_param().abi(), ::std::mem::transmute(pdwindex), ::std::mem::transmute(ppipropertystore)).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Item(&self, dwindex: u32) -> ::windows::runtime::Result<super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), &mut result__).from_abi::<super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::PropertiesSystem::IPropertyStore>>(&self, pipropertystore: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pipropertystore.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Remove(&self, dwindex: u32) -> ::windows::runtime::Result<super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), &mut result__).from_abi::<super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Delete(&self, dwindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn DeleteAll(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyStoreCollection {
    type Vtable = IPropertyStoreCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3511524400, 4818, 17112, [188, 228, 198, 12, 43, 178, 38, 250]);
}
impl ::std::convert::From<IPropertyStoreCollection> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyStoreCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyStoreCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyStoreCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyStoreCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyStoreCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pipropertystore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pipropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProviderProperties(::windows::runtime::IUnknown);
impl IProviderProperties {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCount<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::std::mem::transmute(iproviderinstancecontext), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAt<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, dwindex: u32) -> ::windows::runtime::Result<super::super::System::PropertiesSystem::PROPERTYKEY> {
        let mut result__: <super::super::System::PropertiesSystem::PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::std::mem::transmute(iproviderinstancecontext), ::std::mem::transmute(dwindex), &mut result__).from_abi::<super::super::System::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, key: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::std::mem::transmute(iproviderinstancecontext), ::std::mem::transmute(key), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pifunctioninstance.into_param().abi(), ::std::mem::transmute(iproviderinstancecontext), ::std::mem::transmute(key), ::std::mem::transmute(ppropvar)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProviderProperties {
    type Vtable = IProviderProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3482873510, 15199, 19551, [184, 138, 47, 139, 32, 206, 239, 23]);
}
impl ::std::convert::From<IProviderProperties> for ::windows::runtime::IUnknown {
    fn from(value: IProviderProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProviderProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IProviderProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProviderProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IProviderProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::System::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifunctioninstance: ::windows::runtime::RawPtr, iproviderinstancecontext: isize, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, ppropvar: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProviderPropertyConstraintCollection(::windows::runtime::IUnknown);
impl IProviderPropertyConstraintCollection {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Get(&self, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(ppropvar), ::std::mem::transmute(pdwpropertyconstraint)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Item(&self, dwindex: u32, pkey: *mut super::super::System::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(pkey), ::std::mem::transmute(ppropvar), ::std::mem::transmute(pdwpropertyconstraint)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Next(&self, pkey: *mut super::super::System::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pkey), ::std::mem::transmute(ppropvar), ::std::mem::transmute(pdwpropertyconstraint)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Skip(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProviderPropertyConstraintCollection {
    type Vtable = IProviderPropertyConstraintCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4110083119, 22392, 18963, [133, 64, 181, 253, 140, 19, 152, 221]);
}
impl ::std::convert::From<IProviderPropertyConstraintCollection> for ::windows::runtime::IUnknown {
    fn from(value: IProviderPropertyConstraintCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProviderPropertyConstraintCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IProviderPropertyConstraintCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProviderPropertyConstraintCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IProviderPropertyConstraintCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderPropertyConstraintCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pkey: *mut super::super::System::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkey: *mut super::super::System::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProviderPublishing(::windows::runtime::IUnknown);
impl IProviderPublishing {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstance<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: Param1, pszproviderinstanceidentity: Param2) -> ::windows::runtime::Result<IFunctionInstance> {
        let mut result__: <IFunctionInstance as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(enumvisibilityflags), pszsubcategory.into_param().abi(), pszproviderinstanceidentity.into_param().abi(), &mut result__).from_abi::<IFunctionInstance>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn RemoveInstance<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: Param1, pszproviderinstanceidentity: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(enumvisibilityflags), pszsubcategory.into_param().abi(), pszproviderinstanceidentity.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProviderPublishing {
    type Vtable = IProviderPublishing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3441138180, 8300, 18949, [160, 200, 22, 53, 162, 26, 43, 124]);
}
impl ::std::convert::From<IProviderPublishing> for ::windows::runtime::IUnknown {
    fn from(value: IProviderPublishing) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProviderPublishing> for ::windows::runtime::IUnknown {
    fn from(value: &IProviderPublishing) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProviderPublishing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IProviderPublishing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderPublishing_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProviderQueryConstraintCollection(::windows::runtime::IUnknown);
impl IProviderQueryConstraintCollection {
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`, `Win32_Foundation`*"]
    pub unsafe fn Get<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszconstraintname: Param0) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszconstraintname.into_param().abi(), &mut result__).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Item(&self, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(ppszconstraintname), ::std::mem::transmute(ppszconstraintvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Next(&self, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppszconstraintname), ::std::mem::transmute(ppszconstraintvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Skip(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProviderQueryConstraintCollection {
    type Vtable = IProviderQueryConstraintCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2619620881, 12897, 19405, [185, 34, 132, 168, 115, 212, 96, 174]);
}
impl ::std::convert::From<IProviderQueryConstraintCollection> for ::windows::runtime::IUnknown {
    fn from(value: IProviderQueryConstraintCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProviderQueryConstraintCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IProviderQueryConstraintCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProviderQueryConstraintCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IProviderQueryConstraintCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderQueryConstraintCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszconstraintname: super::super::Foundation::PWSTR, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const MAX_FDCONSTRAINTNAME_LENGTH: u32 = 100u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const MAX_FDCONSTRAINTVALUE_LENGTH: u32 = 1000u32;
pub const PNPXAssociation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3471363273, 20331, 17513, [162, 53, 90, 34, 134, 158, 239, 3]);
pub const PNPXPairingHandler: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3097655618, 44519, 16517, [170, 110, 79, 173, 199, 173, 161, 239]);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const PNPX_INSTALLSTATE_FAILED: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const PNPX_INSTALLSTATE_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const PNPX_INSTALLSTATE_INSTALLING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
pub const PNPX_INSTALLSTATE_NOTINSTALLED: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PropertyConstraint(pub i32);
pub const QC_EQUALS: PropertyConstraint = PropertyConstraint(0i32);
pub const QC_NOTEQUAL: PropertyConstraint = PropertyConstraint(1i32);
pub const QC_LESSTHAN: PropertyConstraint = PropertyConstraint(2i32);
pub const QC_LESSTHANOREQUAL: PropertyConstraint = PropertyConstraint(3i32);
pub const QC_GREATERTHAN: PropertyConstraint = PropertyConstraint(4i32);
pub const QC_GREATERTHANOREQUAL: PropertyConstraint = PropertyConstraint(5i32);
pub const QC_STARTSWITH: PropertyConstraint = PropertyConstraint(6i32);
pub const QC_EXISTS: PropertyConstraint = PropertyConstraint(7i32);
pub const QC_DOESNOTEXIST: PropertyConstraint = PropertyConstraint(8i32);
pub const QC_CONTAINS: PropertyConstraint = PropertyConstraint(9i32);
impl ::std::convert::From<i32> for PropertyConstraint {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PropertyConstraint {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PropertyStore: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3833161040, 57185, 17547, [145, 147, 19, 252, 19, 65, 177, 99]);
pub const PropertyStoreCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3990052905, 55123, 18530, [170, 91, 91, 204, 173, 42, 77, 41]);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct QueryCategoryType(pub i32);
pub const QCT_PROVIDER: QueryCategoryType = QueryCategoryType(0i32);
pub const QCT_LAYERED: QueryCategoryType = QueryCategoryType(1i32);
impl ::std::convert::From<i32> for QueryCategoryType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QueryCategoryType {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct QueryUpdateAction(pub i32);
pub const QUA_ADD: QueryUpdateAction = QueryUpdateAction(0i32);
pub const QUA_REMOVE: QueryUpdateAction = QueryUpdateAction(1i32);
pub const QUA_CHANGE: QueryUpdateAction = QueryUpdateAction(2i32);
impl ::std::convert::From<i32> for QueryUpdateAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QueryUpdateAction {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SID_DeviceDisplayStatusManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4120552787, 33545, 18122, [151, 54, 26, 195, 198, 45, 96, 49]);
pub const SID_EnumDeviceFunction: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(333507042, 50170, 20028, [144, 110, 100, 80, 47, 164, 220, 149]);
pub const SID_EnumInterface: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1089122489, 19839, 19283, [163, 52, 21, 129, 221, 144, 65, 244]);
pub const SID_FDPairingHandler: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(943417850, 21638, 18906, [145, 245, 214, 60, 36, 200, 233, 208]);
pub const SID_FunctionDiscoveryProviderRefresh: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(726449609, 12740, 16596, [166, 45, 119, 42, 161, 116, 237, 82]);
pub const SID_PNPXAssociation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3471363273, 20331, 17513, [162, 53, 90, 34, 134, 158, 239, 3]);
pub const SID_PNPXPropertyStore: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2825203889, 21551, 17311, [183, 28, 176, 117, 107, 19, 103, 122]);
pub const SID_PNPXServiceCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1134461166, 41495, 18194, [159, 166, 222, 171, 217, 194, 167, 39]);
pub const SID_PnpProvider: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2164340366, 51899, 17446, [172, 255, 150, 196, 16, 129, 32, 0]);
pub const SID_UPnPActivator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(218982123, 53108, 16740, [181, 47, 8, 52, 70, 114, 221, 70]);
pub const SID_UninstallDeviceFunction: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3374339694, 22129, 17558, [128, 37, 191, 11, 137, 189, 68, 205]);
pub const SID_UnpairProvider: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2309292796, 34171, 18072, [160, 183, 2, 113, 146, 0, 47, 158]);
#[doc = "*Required features: `Win32_Devices_FunctionDiscovery`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemVisibilityFlags(pub i32);
pub const SVF_SYSTEM: SystemVisibilityFlags = SystemVisibilityFlags(0i32);
pub const SVF_USER: SystemVisibilityFlags = SystemVisibilityFlags(1i32);
impl ::std::convert::From<i32> for SystemVisibilityFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemVisibilityFlags {
    type Abi = Self;
    type DefaultType = Self;
}
