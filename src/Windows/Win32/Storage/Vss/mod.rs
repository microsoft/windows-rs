#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[inline]
pub unsafe fn CreateVssExpressWriterInternal() -> ::windows::core::Result<IVssExpressWriter> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateVssExpressWriterInternal(ppwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IVssExpressWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateVssExpressWriterInternal(&mut result__).from_abi::<IVssExpressWriter>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssAdmin(pub ::windows::core::IUnknown);
impl IVssAdmin {
    pub unsafe fn RegisterProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param5: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, pproviderid: Param0, classid: Param1, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pproviderid.into_param().abi(), classid.into_param().abi(), ::core::mem::transmute(pwszprovidername), ::core::mem::transmute(eprovidertype), ::core::mem::transmute(pwszproviderversion), providerversionid.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, providerid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), providerid.into_param().abi()).ok()
    }
    pub unsafe fn QueryProviders(&self) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__: <IVssEnumObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IVssEnumObject>(result__)
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssAdmin {
    type Vtable = IVssAdmin_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77ed5996_2f63_11d3_8a39_00c04f72d8e3);
}
impl ::core::convert::From<IVssAdmin> for ::windows::core::IUnknown {
    fn from(value: IVssAdmin) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssAdmin> for ::windows::core::IUnknown {
    fn from(value: &IVssAdmin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssAdmin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssAdmin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAdmin_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssAdminEx(pub ::windows::core::IUnknown);
impl IVssAdminEx {
    pub unsafe fn RegisterProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param5: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, pproviderid: Param0, classid: Param1, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pproviderid.into_param().abi(), classid.into_param().abi(), ::core::mem::transmute(pwszprovidername), ::core::mem::transmute(eprovidertype), ::core::mem::transmute(pwszproviderversion), providerversionid.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, providerid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), providerid.into_param().abi()).ok()
    }
    pub unsafe fn QueryProviders(&self) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__: <IVssEnumObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IVssEnumObject>(result__)
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetProviderCapability<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, pproviderid: Param0) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pproviderid.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetProviderContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, providerid: Param0) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), providerid.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProviderContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, providerid: Param0, lcontext: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), providerid.into_param().abi(), ::core::mem::transmute(lcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssAdminEx {
    type Vtable = IVssAdminEx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7858a9f8_b1fa_41a6_964f_b9b36b8cd8d8);
}
impl ::core::convert::From<IVssAdminEx> for ::windows::core::IUnknown {
    fn from(value: IVssAdminEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssAdminEx> for ::windows::core::IUnknown {
    fn from(value: &IVssAdminEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssAdminEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssAdminEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IVssAdminEx> for IVssAdmin {
    fn from(value: IVssAdminEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssAdminEx> for IVssAdmin {
    fn from(value: &IVssAdminEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssAdmin> for IVssAdminEx {
    fn into_param(self) -> ::windows::core::Param<'a, IVssAdmin> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssAdmin> for &IVssAdminEx {
    fn into_param(self) -> ::windows::core::Param<'a, IVssAdmin> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAdminEx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproviderid: ::windows::core::GUID, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providerid: ::windows::core::GUID, plcontext: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providerid: ::windows::core::GUID, lcontext: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssAsync(pub ::windows::core::IUnknown);
impl IVssAsync {
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Wait(&self, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    pub unsafe fn QueryStatus(&self, phrresult: *mut ::windows::core::HRESULT, preserved: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(phrresult), ::core::mem::transmute(preserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssAsync {
    type Vtable = IVssAsync_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x507c37b4_cf5b_4e95_b0af_14eb9767467e);
}
impl ::core::convert::From<IVssAsync> for ::windows::core::IUnknown {
    fn from(value: IVssAsync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssAsync> for ::windows::core::IUnknown {
    fn from(value: &IVssAsync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phrresult: *mut ::windows::core::HRESULT, preserved: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssComponent(pub ::windows::core::IUnknown);
impl IVssComponent {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pct)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(&self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbsucceeded)).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcmappings)).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(imapping), &mut result__).from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupMetadata<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), wszdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPartialFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpath: Param0, wszfilename: Param1, wszranges: Param2, wszmetadata: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilename.into_param().abi(), wszranges.into_param().abi(), wszmetadata.into_param().abi()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcpartialfiles)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipartialfile), ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbselectedforrestore)).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbadditionalrestores)).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcnewtarget)).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(inewtarget), &mut result__).from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDirectedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        wszsourcepath: Param0,
        wszsourcefilename: Param1,
        wszsourcerangelist: Param2,
        wszdestinationpath: Param3,
        wszdestinationfilename: Param4,
        wszdestinationrangelist: Param5,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), wszsourcepath.into_param().abi(), wszsourcefilename.into_param().abi(), wszsourcerangelist.into_param().abi(), wszdestinationpath.into_param().abi(), wszdestinationfilename.into_param().abi(), wszdestinationrangelist.into_param().abi()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdirectedtarget)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(idirectedtarget),
            ::core::mem::transmute(pbstrsourcepath),
            ::core::mem::transmute(pbstrsourcefilename),
            ::core::mem::transmute(pbstrsourcerangelist),
            ::core::mem::transmute(pbstrdestinationpath),
            ::core::mem::transmute(pbstrdestinationfilename),
            ::core::mem::transmute(pbstrdestinationrangelist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestoreMetadata<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszrestoremetadata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), wszrestoremetadata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(target)).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptarget)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreRestoreFailureMsg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszprerestorefailuremsg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), wszprerestorefailuremsg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostRestoreFailureMsg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpostrestorefailuremsg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), wszpostrestorefailuremsg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupStamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszbackupstamp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), wszbackupstamp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcrestoresubcomponent)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(icomponent), ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), ::core::mem::transmute(pbrepair)).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatus)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(&self, wszpath: Param0, wszfilespec: Param1, brecursive: Param2, ftlastmodifytime: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), ftlastmodifytime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszpath: Param0, wszfilespec: Param1, brecursive: Param2, bstrlsnstring: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), bstrlsnstring.into_param().abi()).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdifferencedfiles)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(idifferencedfile), ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), ::core::mem::transmute(pbrecursive), ::core::mem::transmute(pbstrlsnstring), ::core::mem::transmute(pftlastmodifytime)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssComponent {
    type Vtable = IVssComponent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2c72c96_c121_4518_b627_e5a93d010ead);
}
impl ::core::convert::From<IVssComponent> for ::windows::core::IUnknown {
    fn from(value: IVssComponent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssComponent> for ::windows::core::IUnknown {
    fn from(value: &IVssComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsucceeded: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcmappings: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imapping: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszdata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilename: super::super::Foundation::PWSTR, wszranges: super::super::Foundation::PWSTR, wszmetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcpartialfiles: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipartialfile: u32, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrrange: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmetadata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbselectedforrestore: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbadditionalrestores: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcnewtarget: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inewtarget: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilename: super::super::Foundation::PWSTR, wszsourcerangelist: super::super::Foundation::PWSTR, wszdestinationpath: super::super::Foundation::PWSTR, wszdestinationfilename: super::super::Foundation::PWSTR, wszdestinationrangelist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdirectedtarget: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        idirectedtarget: u32,
        pbstrsourcepath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcefilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcerangelist: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationrangelist: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszrestoremetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrestoremetadata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: VSS_RESTORE_TARGET) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszprerestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrprerestorefailuremsg: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpostrestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpostrestorefailuremsg: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszbackupstamp: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrbackupstamp: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrbackupstamp: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrbackupoptions: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrestoreoptions: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcrestoresubcomponent: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icomponent: u32, pbstrlogicalpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrcomponentname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbrepair: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdifferencedfiles: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idifferencedfile: u32, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrfilespec: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssComponentEx(pub ::windows::core::IUnknown);
impl IVssComponentEx {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pct)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(&self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbsucceeded)).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcmappings)).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(imapping), &mut result__).from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupMetadata<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), wszdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPartialFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpath: Param0, wszfilename: Param1, wszranges: Param2, wszmetadata: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilename.into_param().abi(), wszranges.into_param().abi(), wszmetadata.into_param().abi()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcpartialfiles)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipartialfile), ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbselectedforrestore)).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbadditionalrestores)).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcnewtarget)).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(inewtarget), &mut result__).from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDirectedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        wszsourcepath: Param0,
        wszsourcefilename: Param1,
        wszsourcerangelist: Param2,
        wszdestinationpath: Param3,
        wszdestinationfilename: Param4,
        wszdestinationrangelist: Param5,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), wszsourcepath.into_param().abi(), wszsourcefilename.into_param().abi(), wszsourcerangelist.into_param().abi(), wszdestinationpath.into_param().abi(), wszdestinationfilename.into_param().abi(), wszdestinationrangelist.into_param().abi()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdirectedtarget)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(idirectedtarget),
            ::core::mem::transmute(pbstrsourcepath),
            ::core::mem::transmute(pbstrsourcefilename),
            ::core::mem::transmute(pbstrsourcerangelist),
            ::core::mem::transmute(pbstrdestinationpath),
            ::core::mem::transmute(pbstrdestinationfilename),
            ::core::mem::transmute(pbstrdestinationrangelist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestoreMetadata<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszrestoremetadata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), wszrestoremetadata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(target)).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptarget)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreRestoreFailureMsg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszprerestorefailuremsg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), wszprerestorefailuremsg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostRestoreFailureMsg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpostrestorefailuremsg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), wszpostrestorefailuremsg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupStamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszbackupstamp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), wszbackupstamp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcrestoresubcomponent)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(icomponent), ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), ::core::mem::transmute(pbrepair)).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatus)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(&self, wszpath: Param0, wszfilespec: Param1, brecursive: Param2, ftlastmodifytime: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), ftlastmodifytime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszpath: Param0, wszfilespec: Param1, brecursive: Param2, bstrlsnstring: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), bstrlsnstring.into_param().abi()).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdifferencedfiles)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(idifferencedfile), ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), ::core::mem::transmute(pbrecursive), ::core::mem::transmute(pbstrlsnstring), ::core::mem::transmute(pftlastmodifytime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPrepareForBackupFailureMsg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszfailuremsg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), wszfailuremsg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostSnapshotFailureMsg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszfailuremsg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), wszfailuremsg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrepareForBackupFailureMsg(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostSnapshotFailureMsg(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> ::windows::core::Result<bool> {
        let mut result__: <bool as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<bool>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(prolltype), ::core::mem::transmute(pbstrpoint)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IVssComponentEx {
    type Vtable = IVssComponentEx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x156c8b5e_f131_4bd7_9c97_d1923be7e1fa);
}
impl ::core::convert::From<IVssComponentEx> for ::windows::core::IUnknown {
    fn from(value: IVssComponentEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssComponentEx> for ::windows::core::IUnknown {
    fn from(value: &IVssComponentEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssComponentEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssComponentEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IVssComponentEx> for IVssComponent {
    fn from(value: IVssComponentEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssComponentEx> for IVssComponent {
    fn from(value: &IVssComponentEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssComponent> for IVssComponentEx {
    fn into_param(self) -> ::windows::core::Param<'a, IVssComponent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssComponent> for &IVssComponentEx {
    fn into_param(self) -> ::windows::core::Param<'a, IVssComponent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsucceeded: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcmappings: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imapping: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszdata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilename: super::super::Foundation::PWSTR, wszranges: super::super::Foundation::PWSTR, wszmetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcpartialfiles: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipartialfile: u32, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrrange: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmetadata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbselectedforrestore: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbadditionalrestores: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcnewtarget: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inewtarget: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilename: super::super::Foundation::PWSTR, wszsourcerangelist: super::super::Foundation::PWSTR, wszdestinationpath: super::super::Foundation::PWSTR, wszdestinationfilename: super::super::Foundation::PWSTR, wszdestinationrangelist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdirectedtarget: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        idirectedtarget: u32,
        pbstrsourcepath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcefilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcerangelist: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationrangelist: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszrestoremetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrestoremetadata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: VSS_RESTORE_TARGET) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszprerestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrprerestorefailuremsg: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpostrestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpostrestorefailuremsg: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszbackupstamp: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrbackupstamp: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrbackupstamp: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrbackupoptions: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrestoreoptions: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcrestoresubcomponent: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icomponent: u32, pbstrlogicalpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrcomponentname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbrepair: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdifferencedfiles: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idifferencedfile: u32, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrfilespec: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfailuremsg: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfailuremsg: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbauth: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssComponentEx2(pub ::windows::core::IUnknown);
impl IVssComponentEx2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pct)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(&self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbsucceeded)).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcmappings)).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(imapping), &mut result__).from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupMetadata<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), wszdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPartialFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpath: Param0, wszfilename: Param1, wszranges: Param2, wszmetadata: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilename.into_param().abi(), wszranges.into_param().abi(), wszmetadata.into_param().abi()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcpartialfiles)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipartialfile), ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbselectedforrestore)).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbadditionalrestores)).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcnewtarget)).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(inewtarget), &mut result__).from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDirectedTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        wszsourcepath: Param0,
        wszsourcefilename: Param1,
        wszsourcerangelist: Param2,
        wszdestinationpath: Param3,
        wszdestinationfilename: Param4,
        wszdestinationrangelist: Param5,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), wszsourcepath.into_param().abi(), wszsourcefilename.into_param().abi(), wszsourcerangelist.into_param().abi(), wszdestinationpath.into_param().abi(), wszdestinationfilename.into_param().abi(), wszdestinationrangelist.into_param().abi()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdirectedtarget)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(idirectedtarget),
            ::core::mem::transmute(pbstrsourcepath),
            ::core::mem::transmute(pbstrsourcefilename),
            ::core::mem::transmute(pbstrsourcerangelist),
            ::core::mem::transmute(pbstrdestinationpath),
            ::core::mem::transmute(pbstrdestinationfilename),
            ::core::mem::transmute(pbstrdestinationrangelist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestoreMetadata<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszrestoremetadata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), wszrestoremetadata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(target)).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptarget)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreRestoreFailureMsg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszprerestorefailuremsg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), wszprerestorefailuremsg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostRestoreFailureMsg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpostrestorefailuremsg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), wszpostrestorefailuremsg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupStamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszbackupstamp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), wszbackupstamp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcrestoresubcomponent)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(icomponent), ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), ::core::mem::transmute(pbrepair)).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatus)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(&self, wszpath: Param0, wszfilespec: Param1, brecursive: Param2, ftlastmodifytime: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), ftlastmodifytime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszpath: Param0, wszfilespec: Param1, brecursive: Param2, bstrlsnstring: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), bstrlsnstring.into_param().abi()).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdifferencedfiles)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(idifferencedfile), ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), ::core::mem::transmute(pbrecursive), ::core::mem::transmute(pbstrlsnstring), ::core::mem::transmute(pftlastmodifytime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPrepareForBackupFailureMsg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszfailuremsg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), wszfailuremsg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostSnapshotFailureMsg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszfailuremsg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), wszfailuremsg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrepareForBackupFailureMsg(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostSnapshotFailureMsg(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> ::windows::core::Result<bool> {
        let mut result__: <bool as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<bool>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(prolltype), ::core::mem::transmute(pbstrpoint)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFailure<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: Param2, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr), ::core::mem::transmute(hrapplication), wszapplicationmessage.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFailure(&self, phr: *mut ::windows::core::HRESULT, phrapplication: *mut ::windows::core::HRESULT, pbstrapplicationmessage: *mut super::super::Foundation::BSTR, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(phr), ::core::mem::transmute(phrapplication), ::core::mem::transmute(pbstrapplicationmessage), ::core::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssComponentEx2 {
    type Vtable = IVssComponentEx2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b5be0f2_07a9_4e4b_bdd3_cfdc8e2c0d2d);
}
impl ::core::convert::From<IVssComponentEx2> for ::windows::core::IUnknown {
    fn from(value: IVssComponentEx2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssComponentEx2> for ::windows::core::IUnknown {
    fn from(value: &IVssComponentEx2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssComponentEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssComponentEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IVssComponentEx2> for IVssComponentEx {
    fn from(value: IVssComponentEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssComponentEx2> for IVssComponentEx {
    fn from(value: &IVssComponentEx2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssComponentEx> for IVssComponentEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IVssComponentEx> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssComponentEx> for &IVssComponentEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IVssComponentEx> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVssComponentEx2> for IVssComponent {
    fn from(value: IVssComponentEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssComponentEx2> for IVssComponent {
    fn from(value: &IVssComponentEx2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssComponent> for IVssComponentEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IVssComponent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssComponent> for &IVssComponentEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IVssComponent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsucceeded: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcmappings: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imapping: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszdata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilename: super::super::Foundation::PWSTR, wszranges: super::super::Foundation::PWSTR, wszmetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcpartialfiles: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipartialfile: u32, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrrange: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmetadata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbselectedforrestore: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbadditionalrestores: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcnewtarget: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inewtarget: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilename: super::super::Foundation::PWSTR, wszsourcerangelist: super::super::Foundation::PWSTR, wszdestinationpath: super::super::Foundation::PWSTR, wszdestinationfilename: super::super::Foundation::PWSTR, wszdestinationrangelist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdirectedtarget: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        idirectedtarget: u32,
        pbstrsourcepath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcefilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcerangelist: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationrangelist: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszrestoremetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrestoremetadata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: VSS_RESTORE_TARGET) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszprerestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrprerestorefailuremsg: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpostrestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpostrestorefailuremsg: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszbackupstamp: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrbackupstamp: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrbackupstamp: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrbackupoptions: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrestoreoptions: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcrestoresubcomponent: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icomponent: u32, pbstrlogicalpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrcomponentname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbrepair: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdifferencedfiles: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idifferencedfile: u32, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrfilespec: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfailuremsg: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfailuremsg: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbauth: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phr: *mut ::windows::core::HRESULT, phrapplication: *mut ::windows::core::HRESULT, pbstrapplicationmessage: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssCreateExpressWriterMetadata(pub ::windows::core::IUnknown);
impl IVssCreateExpressWriterMetadata {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddExcludeFiles<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpath: Param0, wszfilespec: Param1, brecursive: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), ::core::mem::transmute(brecursive)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddComponent<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: Param1,
        wszcomponentname: Param2,
        wszcaption: Param3,
        pbicon: *const u8,
        cbicon: u32,
        brestoremetadata: u8,
        bnotifyonbackupcomplete: u8,
        bselectable: u8,
        bselectableforrestore: u8,
        dwcomponentflags: u32,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ct),
            wszlogicalpath.into_param().abi(),
            wszcomponentname.into_param().abi(),
            wszcaption.into_param().abi(),
            ::core::mem::transmute(pbicon),
            ::core::mem::transmute(cbicon),
            ::core::mem::transmute(brestoremetadata),
            ::core::mem::transmute(bnotifyonbackupcomplete),
            ::core::mem::transmute(bselectable),
            ::core::mem::transmute(bselectableforrestore),
            ::core::mem::transmute(dwcomponentflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFilesToFileGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        wszlogicalpath: Param0,
        wszgroupname: Param1,
        wszpath: Param2,
        wszfilespec: Param3,
        brecursive: u8,
        wszalternatelocation: Param5,
        dwbackuptypemask: u32,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wszlogicalpath.into_param().abi(), wszgroupname.into_param().abi(), wszpath.into_param().abi(), wszfilespec.into_param().abi(), ::core::mem::transmute(brecursive), wszalternatelocation.into_param().abi(), ::core::mem::transmute(dwbackuptypemask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestoreMethod<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: Param1, wszuserprocedure: Param2, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), wszservice.into_param().abi(), wszuserprocedure.into_param().abi(), ::core::mem::transmute(writerrestore), ::core::mem::transmute(brebootrequired)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddComponentDependency<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        wszforlogicalpath: Param0,
        wszforcomponentname: Param1,
        onwriterid: Param2,
        wszonlogicalpath: Param3,
        wszoncomponentname: Param4,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), wszforlogicalpath.into_param().abi(), wszforcomponentname.into_param().abi(), onwriterid.into_param().abi(), wszonlogicalpath.into_param().abi(), wszoncomponentname.into_param().abi()).ok()
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwschemamask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAsXML(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IVssCreateExpressWriterMetadata {
    type Vtable = IVssCreateExpressWriterMetadata_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c772e77_b26e_427f_92dd_c996f41ea5e3);
}
impl ::core::convert::From<IVssCreateExpressWriterMetadata> for ::windows::core::IUnknown {
    fn from(value: IVssCreateExpressWriterMetadata) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssCreateExpressWriterMetadata> for ::windows::core::IUnknown {
    fn from(value: &IVssCreateExpressWriterMetadata) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssCreateExpressWriterMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssCreateExpressWriterMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssCreateExpressWriterMetadata_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwschemamask: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrxml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssCreateWriterMetadata(pub ::windows::core::IUnknown);
impl IVssCreateWriterMetadata {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddIncludeFiles<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpath: Param0, wszfilespec: Param1, brecursive: u8, wszalternatelocation: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), ::core::mem::transmute(brecursive), wszalternatelocation.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddExcludeFiles<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpath: Param0, wszfilespec: Param1, brecursive: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), ::core::mem::transmute(brecursive)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddComponent<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: Param1,
        wszcomponentname: Param2,
        wszcaption: Param3,
        pbicon: *const u8,
        cbicon: u32,
        brestoremetadata: u8,
        bnotifyonbackupcomplete: u8,
        bselectable: u8,
        bselectableforrestore: u8,
        dwcomponentflags: u32,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ct),
            wszlogicalpath.into_param().abi(),
            wszcomponentname.into_param().abi(),
            wszcaption.into_param().abi(),
            ::core::mem::transmute(pbicon),
            ::core::mem::transmute(cbicon),
            ::core::mem::transmute(brestoremetadata),
            ::core::mem::transmute(bnotifyonbackupcomplete),
            ::core::mem::transmute(bselectable),
            ::core::mem::transmute(bselectableforrestore),
            ::core::mem::transmute(dwcomponentflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDatabaseFiles<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszlogicalpath: Param0, wszdatabasename: Param1, wszpath: Param2, wszfilespec: Param3, dwbackuptypemask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), wszlogicalpath.into_param().abi(), wszdatabasename.into_param().abi(), wszpath.into_param().abi(), wszfilespec.into_param().abi(), ::core::mem::transmute(dwbackuptypemask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDatabaseLogFiles<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszlogicalpath: Param0, wszdatabasename: Param1, wszpath: Param2, wszfilespec: Param3, dwbackuptypemask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), wszlogicalpath.into_param().abi(), wszdatabasename.into_param().abi(), wszpath.into_param().abi(), wszfilespec.into_param().abi(), ::core::mem::transmute(dwbackuptypemask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFilesToFileGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        wszlogicalpath: Param0,
        wszgroupname: Param1,
        wszpath: Param2,
        wszfilespec: Param3,
        brecursive: u8,
        wszalternatelocation: Param5,
        dwbackuptypemask: u32,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), wszlogicalpath.into_param().abi(), wszgroupname.into_param().abi(), wszpath.into_param().abi(), wszfilespec.into_param().abi(), ::core::mem::transmute(brecursive), wszalternatelocation.into_param().abi(), ::core::mem::transmute(dwbackuptypemask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestoreMethod<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: Param1, wszuserprocedure: Param2, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), wszservice.into_param().abi(), wszuserprocedure.into_param().abi(), ::core::mem::transmute(writerrestore), ::core::mem::transmute(brebootrequired)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddAlternateLocationMapping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszsourcepath: Param0, wszsourcefilespec: Param1, brecursive: u8, wszdestination: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), wszsourcepath.into_param().abi(), wszsourcefilespec.into_param().abi(), ::core::mem::transmute(brecursive), wszdestination.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddComponentDependency<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        wszforlogicalpath: Param0,
        wszforcomponentname: Param1,
        onwriterid: Param2,
        wszonlogicalpath: Param3,
        wszoncomponentname: Param4,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), wszforlogicalpath.into_param().abi(), wszforcomponentname.into_param().abi(), onwriterid.into_param().abi(), wszonlogicalpath.into_param().abi(), wszoncomponentname.into_param().abi()).ok()
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwschemamask)).ok()
    }
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument> {
        let mut result__: <super::super::Data::Xml::MsXml::IXMLDOMDocument as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Data::Xml::MsXml::IXMLDOMDocument>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAsXML(&self, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrxml)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssCreateWriterMetadata {
    type Vtable = IVssCreateWriterMetadata_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IVssCreateWriterMetadata> for ::windows::core::IUnknown {
    fn from(value: IVssCreateWriterMetadata) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssCreateWriterMetadata> for ::windows::core::IUnknown {
    fn from(value: &IVssCreateWriterMetadata) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssCreateWriterMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssCreateWriterMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssCreateWriterMetadata_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilespec: super::super::Foundation::PWSTR, brecursive: u8, wszdestination: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwschemamask: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Data_Xml_MsXml")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Data_Xml_MsXml"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrxml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssDifferentialSoftwareSnapshotMgmt(pub ::windows::core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(llmaximumdiffspace)).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(llmaximumdiffspace)).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszoriginalvolumename), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotid: Param0) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), snapshotid.into_param().abi(), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
}
unsafe impl ::windows::core::Interface for IVssDifferentialSoftwareSnapshotMgmt {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x214a0f28_b737_4026_b847_4f9e37d79529);
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt> for ::windows::core::IUnknown {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt> for ::windows::core::IUnknown {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssDifferentialSoftwareSnapshotMgmt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssDifferentialSoftwareSnapshotMgmt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszoriginalvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2(pub ::windows::core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt2 {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(llmaximumdiffspace)).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(llmaximumdiffspace)).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszoriginalvolumename), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotid: Param0) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), snapshotid.into_param().abi(), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDiffAreaMaximumSizeEx<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(llmaximumdiffspace), bvolatile.into_param().abi()).ok()
    }
    pub unsafe fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(pwsznewdiffareavolumename)).ok()
    }
    pub unsafe fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<IVssAsync> {
        let mut result__: <IVssAsync as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), &mut result__).from_abi::<IVssAsync>(result__)
    }
    pub unsafe fn SetSnapshotPriority<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, idsnapshot: Param0, priority: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), idsnapshot.into_param().abi(), ::core::mem::transmute(priority)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssDifferentialSoftwareSnapshotMgmt2 {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x949d7353_675f_4275_8969_f044c6277815);
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt2> for ::windows::core::IUnknown {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt2> for ::windows::core::IUnknown {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssDifferentialSoftwareSnapshotMgmt2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt2> for IVssDifferentialSoftwareSnapshotMgmt {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt2> for IVssDifferentialSoftwareSnapshotMgmt {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt> for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn into_param(self) -> ::windows::core::Param<'a, IVssDifferentialSoftwareSnapshotMgmt> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt> for &IVssDifferentialSoftwareSnapshotMgmt2 {
    fn into_param(self) -> ::windows::core::Param<'a, IVssDifferentialSoftwareSnapshotMgmt> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszoriginalvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3(pub ::windows::core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt3 {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(llmaximumdiffspace)).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(llmaximumdiffspace)).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszoriginalvolumename), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotid: Param0) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), snapshotid.into_param().abi(), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDiffAreaMaximumSizeEx<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(llmaximumdiffspace), bvolatile.into_param().abi()).ok()
    }
    pub unsafe fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(pwsznewdiffareavolumename)).ok()
    }
    pub unsafe fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<IVssAsync> {
        let mut result__: <IVssAsync as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), &mut result__).from_abi::<IVssAsync>(result__)
    }
    pub unsafe fn SetSnapshotPriority<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, idsnapshot: Param0, priority: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), idsnapshot.into_param().abi(), ::core::mem::transmute(priority)).ok()
    }
    pub unsafe fn SetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(protectionlevel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVolumeProtectLevel(&self, pwszvolumename: *const u16) -> ::windows::core::Result<VSS_VOLUME_PROTECTION_INFO> {
        let mut result__: <VSS_VOLUME_PROTECTION_INFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), &mut result__).from_abi::<VSS_VOLUME_PROTECTION_INFO>(result__)
    }
    pub unsafe fn ClearVolumeProtectFault(&self, pwszvolumename: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename)).ok()
    }
    pub unsafe fn DeleteUnusedDiffAreas(&self, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszdiffareavolumename)).ok()
    }
    pub unsafe fn QuerySnapshotDeltaBitmap<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, idsnapshotolder: Param0, idsnapshotyounger: Param1, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), idsnapshotolder.into_param().abi(), idsnapshotyounger.into_param().abi(), ::core::mem::transmute(pcblocksizeperbit), ::core::mem::transmute(pcbitmaplength), ::core::mem::transmute(ppbbitmap)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssDifferentialSoftwareSnapshotMgmt3 {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x383f7e71_a4c5_401f_b27f_f826289f8458);
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt3> for ::windows::core::IUnknown {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt3> for ::windows::core::IUnknown {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssDifferentialSoftwareSnapshotMgmt3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssDifferentialSoftwareSnapshotMgmt3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt3> for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt3> for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt2> for IVssDifferentialSoftwareSnapshotMgmt3 {
    fn into_param(self) -> ::windows::core::Param<'a, IVssDifferentialSoftwareSnapshotMgmt2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt2> for &IVssDifferentialSoftwareSnapshotMgmt3 {
    fn into_param(self) -> ::windows::core::Param<'a, IVssDifferentialSoftwareSnapshotMgmt2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt3> for IVssDifferentialSoftwareSnapshotMgmt {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt3> for IVssDifferentialSoftwareSnapshotMgmt {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt> for IVssDifferentialSoftwareSnapshotMgmt3 {
    fn into_param(self) -> ::windows::core::Param<'a, IVssDifferentialSoftwareSnapshotMgmt> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt> for &IVssDifferentialSoftwareSnapshotMgmt3 {
    fn into_param(self) -> ::windows::core::Param<'a, IVssDifferentialSoftwareSnapshotMgmt> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszoriginalvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszdiffareavolumename: *const u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idsnapshotolder: ::windows::core::GUID, idsnapshotyounger: ::windows::core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssEnumMgmtObject(pub ::windows::core::IUnknown);
impl IVssEnumMgmtObject {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: *mut ::core::option::Option<IVssEnumMgmtObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppenum)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssEnumMgmtObject {
    type Vtable = IVssEnumMgmtObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01954e6b_9254_4e6e_808c_c9e05d007696);
}
impl ::core::convert::From<IVssEnumMgmtObject> for ::windows::core::IUnknown {
    fn from(value: IVssEnumMgmtObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssEnumMgmtObject> for ::windows::core::IUnknown {
    fn from(value: &IVssEnumMgmtObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssEnumMgmtObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssEnumMgmtObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssEnumMgmtObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssEnumObject(pub ::windows::core::IUnknown);
impl IVssEnumObject {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: *mut ::core::option::Option<IVssEnumObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppenum)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssEnumObject {
    type Vtable = IVssEnumObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae1c7110_2f60_11d3_8a39_00c04f72d8e3);
}
impl ::core::convert::From<IVssEnumObject> for ::windows::core::IUnknown {
    fn from(value: IVssEnumObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssEnumObject> for ::windows::core::IUnknown {
    fn from(value: &IVssEnumObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssEnumObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssEnumObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssEnumObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IVssExamineWriterMetadata(pub u8);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssExpressWriter(pub ::windows::core::IUnknown);
impl IVssExpressWriter {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateMetadata<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, writerid: Param0, writername: Param1, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> ::windows::core::Result<IVssCreateExpressWriterMetadata> {
        let mut result__: <IVssCreateExpressWriterMetadata as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), writerid.into_param().abi(), writername.into_param().abi(), ::core::mem::transmute(usagetype), ::core::mem::transmute(versionmajor), ::core::mem::transmute(versionminor), ::core::mem::transmute(reserved), &mut result__).from_abi::<IVssCreateExpressWriterMetadata>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadMetadata<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, metadata: Param0, reserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), metadata.into_param().abi(), ::core::mem::transmute(reserved)).ok()
    }
    pub unsafe fn Register(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unregister<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, writerid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), writerid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssExpressWriter {
    type Vtable = IVssExpressWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe33affdc_59c7_47b1_97d5_4266598f6235);
}
impl ::core::convert::From<IVssExpressWriter> for ::windows::core::IUnknown {
    fn from(value: IVssExpressWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssExpressWriter> for ::windows::core::IUnknown {
    fn from(value: &IVssExpressWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssExpressWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssExpressWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssExpressWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, writerid: ::windows::core::GUID, writername: super::super::Foundation::PWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, metadata: super::super::Foundation::PWSTR, reserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, writerid: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssFileShareSnapshotProvider(pub ::windows::core::IUnknown);
impl IVssFileShareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcontext)).ok()
    }
    pub unsafe fn GetSnapshotProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotid: Param0) -> ::windows::core::Result<VSS_SNAPSHOT_PROP> {
        let mut result__: <VSS_SNAPSHOT_PROP as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), snapshotid.into_param().abi(), &mut result__).from_abi::<VSS_SNAPSHOT_PROP>(result__)
    }
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, queriedobjectid: Param0, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__: <IVssEnumObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), queriedobjectid.into_param().abi(), ::core::mem::transmute(equeriedobjecttype), ::core::mem::transmute(ereturnedobjectstype), &mut result__).from_abi::<IVssEnumObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSnapshots<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, sourceobjectid: Param0, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: Param2, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sourceobjectid.into_param().abi(), ::core::mem::transmute(esourceobjecttype), bforcedelete.into_param().abi(), ::core::mem::transmute(pldeletedsnapshots), ::core::mem::transmute(pnondeletedsnapshotid)).ok()
    }
    pub unsafe fn BeginPrepareSnapshot<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param4: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0, snapshotid: Param1, pwszsharepath: *const u16, lnewcontext: i32, providerid: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi(), snapshotid.into_param().abi(), ::core::mem::transmute(pwszsharepath), ::core::mem::transmute(lnewcontext), providerid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathSupported(&self, pwszsharepath: *const u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszsharepath), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathSnapshotted(&self, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszsharepath), ::core::mem::transmute(pbsnapshotspresent), ::core::mem::transmute(plsnapshotcompatibility)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSnapshotProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, snapshotid: Param0, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), snapshotid.into_param().abi(), ::core::mem::transmute(esnapshotpropertyid), vproperty.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssFileShareSnapshotProvider {
    type Vtable = IVssFileShareSnapshotProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8636060_7c2e_11df_8c4a_0800200c9a66);
}
impl ::core::convert::From<IVssFileShareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: IVssFileShareSnapshotProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssFileShareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: &IVssFileShareSnapshotProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssFileShareSnapshotProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssFileShareSnapshotProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssFileShareSnapshotProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcontext: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssHardwareSnapshotProvider(pub ::windows::core::IUnknown);
impl IVssHardwareSnapshotProvider {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lluncount), ::core::mem::transmute(lcontext), ::core::mem::transmute(rgwszdevices), ::core::mem::transmute(pluninformation), ::core::mem::transmute(pbissupported)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wszdevicename), ::core::mem::transmute(pluninfo), ::core::mem::transmute(pbissupported)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn BeginPrepareSnapshot<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0, snapshotid: Param1, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi(), snapshotid.into_param().abi(), ::core::mem::transmute(lcontext), ::core::mem::transmute(lluncount), ::core::mem::transmute(rgdevicenames), ::core::mem::transmute(rgluninformation)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lluncount), ::core::mem::transmute(rgdevicenames), ::core::mem::transmute(rgsourceluns), ::core::mem::transmute(rgdestinationluns)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn LocateLuns(&self, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lluncount), ::core::mem::transmute(rgsourceluns)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(wszdevicename), ::core::mem::transmute(pinformation)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssHardwareSnapshotProvider {
    type Vtable = IVssHardwareSnapshotProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9593a157_44e9_4344_bbeb_44fbf9b06b10);
}
impl ::core::convert::From<IVssHardwareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: IVssHardwareSnapshotProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssHardwareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: &IVssHardwareSnapshotProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssHardwareSnapshotProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssHardwareSnapshotProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssHardwareSnapshotProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssHardwareSnapshotProviderEx(pub ::windows::core::IUnknown);
impl IVssHardwareSnapshotProviderEx {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lluncount), ::core::mem::transmute(lcontext), ::core::mem::transmute(rgwszdevices), ::core::mem::transmute(pluninformation), ::core::mem::transmute(pbissupported)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wszdevicename), ::core::mem::transmute(pluninfo), ::core::mem::transmute(pbissupported)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn BeginPrepareSnapshot<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0, snapshotid: Param1, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi(), snapshotid.into_param().abi(), ::core::mem::transmute(lcontext), ::core::mem::transmute(lluncount), ::core::mem::transmute(rgdevicenames), ::core::mem::transmute(rgluninformation)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lluncount), ::core::mem::transmute(rgdevicenames), ::core::mem::transmute(rgsourceluns), ::core::mem::transmute(rgdestinationluns)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn LocateLuns(&self, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lluncount), ::core::mem::transmute(rgsourceluns)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(wszdevicename), ::core::mem::transmute(pinformation)).ok()
    }
    pub unsafe fn GetProviderCapabilities(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnLunStateChange(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(psnapshotluns), ::core::mem::transmute(poriginalluns), ::core::mem::transmute(dwcount), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn ResyncLuns(&self, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::Result<IVssAsync> {
        let mut result__: <IVssAsync as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(psourceluns), ::core::mem::transmute(ptargetluns), ::core::mem::transmute(dwcount), &mut result__).from_abi::<IVssAsync>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnReuseLuns(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(psnapshotluns), ::core::mem::transmute(poriginalluns), ::core::mem::transmute(dwcount)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssHardwareSnapshotProviderEx {
    type Vtable = IVssHardwareSnapshotProviderEx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f5ba925_cdb1_4d11_a71f_339eb7e709fd);
}
impl ::core::convert::From<IVssHardwareSnapshotProviderEx> for ::windows::core::IUnknown {
    fn from(value: IVssHardwareSnapshotProviderEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssHardwareSnapshotProviderEx> for ::windows::core::IUnknown {
    fn from(value: &IVssHardwareSnapshotProviderEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssHardwareSnapshotProviderEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssHardwareSnapshotProviderEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IVssHardwareSnapshotProviderEx> for IVssHardwareSnapshotProvider {
    fn from(value: IVssHardwareSnapshotProviderEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssHardwareSnapshotProviderEx> for IVssHardwareSnapshotProvider {
    fn from(value: &IVssHardwareSnapshotProviderEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssHardwareSnapshotProvider> for IVssHardwareSnapshotProviderEx {
    fn into_param(self) -> ::windows::core::Param<'a, IVssHardwareSnapshotProvider> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVssHardwareSnapshotProvider> for &IVssHardwareSnapshotProviderEx {
    fn into_param(self) -> ::windows::core::Param<'a, IVssHardwareSnapshotProvider> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssHardwareSnapshotProviderEx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssProviderCreateSnapshotSet(pub ::windows::core::IUnknown);
impl IVssProviderCreateSnapshotSet {
    pub unsafe fn EndPrepareSnapshots<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi()).ok()
    }
    pub unsafe fn PreCommitSnapshots<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi()).ok()
    }
    pub unsafe fn CommitSnapshots<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi()).ok()
    }
    pub unsafe fn PostCommitSnapshots<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0, lsnapshotscount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi(), ::core::mem::transmute(lsnapshotscount)).ok()
    }
    pub unsafe fn PreFinalCommitSnapshots<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi()).ok()
    }
    pub unsafe fn PostFinalCommitSnapshots<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi()).ok()
    }
    pub unsafe fn AbortSnapshots<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssProviderCreateSnapshotSet {
    type Vtable = IVssProviderCreateSnapshotSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f894e5b_1e39_4778_8e23_9abad9f0e08c);
}
impl ::core::convert::From<IVssProviderCreateSnapshotSet> for ::windows::core::IUnknown {
    fn from(value: IVssProviderCreateSnapshotSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssProviderCreateSnapshotSet> for ::windows::core::IUnknown {
    fn from(value: &IVssProviderCreateSnapshotSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssProviderCreateSnapshotSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssProviderCreateSnapshotSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssProviderCreateSnapshotSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID, lsnapshotscount: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssProviderNotifications(pub ::windows::core::IUnknown);
impl IVssProviderNotifications {
    pub unsafe fn OnLoad<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUnload<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bforceunload: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bforceunload.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssProviderNotifications {
    type Vtable = IVssProviderNotifications_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe561901f_03a5_4afe_86d0_72baeece7004);
}
impl ::core::convert::From<IVssProviderNotifications> for ::windows::core::IUnknown {
    fn from(value: IVssProviderNotifications) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssProviderNotifications> for ::windows::core::IUnknown {
    fn from(value: &IVssProviderNotifications) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssProviderNotifications {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssProviderNotifications {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssProviderNotifications_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssSnapshotMgmt(pub ::windows::core::IUnknown);
impl IVssSnapshotMgmt {
    pub unsafe fn GetProviderMgmtInterface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, providerid: Param0, interfaceid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), providerid.into_param().abi(), ::core::mem::transmute(interfaceid), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn QueryVolumesSupportedForSnapshots<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, providerid: Param0, lcontext: i32) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), providerid.into_param().abi(), ::core::mem::transmute(lcontext), &mut result__).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QuerySnapshotsByVolume<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, pwszvolumename: *const u16, providerid: Param1) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__: <IVssEnumObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), providerid.into_param().abi(), &mut result__).from_abi::<IVssEnumObject>(result__)
    }
}
unsafe impl ::windows::core::Interface for IVssSnapshotMgmt {
    type Vtable = IVssSnapshotMgmt_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa7df749_66e7_4986_a27f_e2f04ae53772);
}
impl ::core::convert::From<IVssSnapshotMgmt> for ::windows::core::IUnknown {
    fn from(value: IVssSnapshotMgmt) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssSnapshotMgmt> for ::windows::core::IUnknown {
    fn from(value: &IVssSnapshotMgmt) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssSnapshotMgmt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssSnapshotMgmt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSnapshotMgmt_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providerid: ::windows::core::GUID, interfaceid: *const ::windows::core::GUID, ppitf: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providerid: ::windows::core::GUID, lcontext: i32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, providerid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssSnapshotMgmt2(pub ::windows::core::IUnknown);
impl IVssSnapshotMgmt2 {
    pub unsafe fn GetMinDiffAreaSize(&self) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IVssSnapshotMgmt2 {
    type Vtable = IVssSnapshotMgmt2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f61ec39_fe82_45f2_a3f0_768b5d427102);
}
impl ::core::convert::From<IVssSnapshotMgmt2> for ::windows::core::IUnknown {
    fn from(value: IVssSnapshotMgmt2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssSnapshotMgmt2> for ::windows::core::IUnknown {
    fn from(value: &IVssSnapshotMgmt2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssSnapshotMgmt2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssSnapshotMgmt2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSnapshotMgmt2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pllmindiffareasize: *mut i64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssSoftwareSnapshotProvider(pub ::windows::core::IUnknown);
impl IVssSoftwareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcontext)).ok()
    }
    pub unsafe fn GetSnapshotProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotid: Param0) -> ::windows::core::Result<VSS_SNAPSHOT_PROP> {
        let mut result__: <VSS_SNAPSHOT_PROP as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), snapshotid.into_param().abi(), &mut result__).from_abi::<VSS_SNAPSHOT_PROP>(result__)
    }
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, queriedobjectid: Param0, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__: <IVssEnumObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), queriedobjectid.into_param().abi(), ::core::mem::transmute(equeriedobjecttype), ::core::mem::transmute(ereturnedobjectstype), &mut result__).from_abi::<IVssEnumObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSnapshots<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, sourceobjectid: Param0, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: Param2, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sourceobjectid.into_param().abi(), ::core::mem::transmute(esourceobjecttype), bforcedelete.into_param().abi(), ::core::mem::transmute(pldeletedsnapshots), ::core::mem::transmute(pnondeletedsnapshotid)).ok()
    }
    pub unsafe fn BeginPrepareSnapshot<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotsetid: Param0, snapshotid: Param1, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), snapshotsetid.into_param().abi(), snapshotid.into_param().abi(), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(lnewcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVolumeSupported(&self, pwszvolumename: *const u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVolumeSnapshotted(&self, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pbsnapshotspresent), ::core::mem::transmute(plsnapshotcompatibility)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSnapshotProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, snapshotid: Param0, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), snapshotid.into_param().abi(), ::core::mem::transmute(esnapshotpropertyid), vproperty.into_param().abi()).ok()
    }
    pub unsafe fn RevertToSnapshot<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, snapshotid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), snapshotid.into_param().abi()).ok()
    }
    pub unsafe fn QueryRevertStatus(&self, pwszvolume: *const u16) -> ::windows::core::Result<IVssAsync> {
        let mut result__: <IVssAsync as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszvolume), &mut result__).from_abi::<IVssAsync>(result__)
    }
}
unsafe impl ::windows::core::Interface for IVssSoftwareSnapshotProvider {
    type Vtable = IVssSoftwareSnapshotProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x609e123e_2c5a_44d3_8f01_0b1d9a47d1ff);
}
impl ::core::convert::From<IVssSoftwareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: IVssSoftwareSnapshotProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssSoftwareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: &IVssSoftwareSnapshotProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssSoftwareSnapshotProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssSoftwareSnapshotProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSoftwareSnapshotProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcontext: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, snapshotid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszvolume: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssWMDependency(pub ::windows::core::IUnknown);
impl IVssWMDependency {
    pub unsafe fn GetWriterId(&self, pwriterid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwriterid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(&self, pbstrlogicalpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrlogicalpath)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(&self, pbstrcomponentname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrcomponentname)).ok()
    }
}
unsafe impl ::windows::core::Interface for IVssWMDependency {
    type Vtable = IVssWMDependency_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IVssWMDependency> for ::windows::core::IUnknown {
    fn from(value: IVssWMDependency) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssWMDependency> for ::windows::core::IUnknown {
    fn from(value: &IVssWMDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssWMDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssWMDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwriterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrlogicalpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrcomponentname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssWMFiledesc(pub ::windows::core::IUnknown);
impl IVssWMFiledesc {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilespec(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetRecursive(&self) -> ::windows::core::Result<bool> {
        let mut result__: <bool as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<bool>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAlternateLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetBackupTypeMask(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IVssWMFiledesc {
    type Vtable = IVssWMFiledesc_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IVssWMFiledesc> for ::windows::core::IUnknown {
    fn from(value: IVssWMFiledesc) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssWMFiledesc> for ::windows::core::IUnknown {
    fn from(value: &IVssWMFiledesc) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssWMFiledesc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssWMFiledesc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMFiledesc_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfilespec: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbrecursive: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstralternatelocation: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwtypemask: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssWriterComponents(pub ::windows::core::IUnknown);
impl IVssWriterComponents {
    pub unsafe fn GetComponentCount(&self, pccomponents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pccomponents)).ok()
    }
    pub unsafe fn GetWriterInfo(&self, pidinstance: *mut ::windows::core::GUID, pidwriter: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pidinstance), ::core::mem::transmute(pidwriter)).ok()
    }
    pub unsafe fn GetComponent(&self, icomponent: u32) -> ::windows::core::Result<IVssComponent> {
        let mut result__: <IVssComponent as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(icomponent), &mut result__).from_abi::<IVssComponent>(result__)
    }
}
unsafe impl ::windows::core::Interface for IVssWriterComponents {
    type Vtable = IVssWriterComponents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IVssWriterComponents> for ::windows::core::IUnknown {
    fn from(value: IVssWriterComponents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssWriterComponents> for ::windows::core::IUnknown {
    fn from(value: &IVssWriterComponents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssWriterComponents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssWriterComponents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWriterComponents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pccomponents: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pidinstance: *mut ::windows::core::GUID, pidwriter: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icomponent: u32, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVssWriterImpl(pub ::windows::core::IUnknown);
impl IVssWriterImpl {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        writerid: Param0,
        wszwritername: Param1,
        wszwriterinstancename: Param2,
        dwmajorversion: u32,
        dwminorversion: u32,
        ut: VSS_USAGE_TYPE,
        st: VSS_SOURCE_TYPE,
        nlevel: VSS_APPLICATION_LEVEL,
        dwtimeout: u32,
        aws: VSS_ALTERNATE_WRITER_STATE,
        biothrottlingonly: u8,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            writerid.into_param().abi(),
            wszwritername.into_param().abi(),
            wszwriterinstancename.into_param().abi(),
            ::core::mem::transmute(dwmajorversion),
            ::core::mem::transmute(dwminorversion),
            ::core::mem::transmute(ut),
            ::core::mem::transmute(st),
            ::core::mem::transmute(nlevel),
            ::core::mem::transmute(dwtimeout),
            ::core::mem::transmute(aws),
            ::core::mem::transmute(biothrottlingonly),
        )
        .ok()
    }
    pub unsafe fn Subscribe(&self, dwsubscribetimeout: u32, dweventflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsubscribetimeout), ::core::mem::transmute(dweventflags)).ok()
    }
    pub unsafe fn Unsubscribe(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Uninitialize(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrentVolumeArray(&self) -> *mut super::super::Foundation::PWSTR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetCurrentVolumeCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapshotDeviceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszoriginalvolume: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), wszoriginalvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetCurrentSnapshotSetId(&self) -> ::windows::core::GUID {
        let mut result__: ::windows::core::GUID = ::core::default::Default::default();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
    pub unsafe fn GetContext(&self) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetCurrentLevel(&self) -> VSS_APPLICATION_LEVEL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathAffected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpath: Param0) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), wszpath.into_param().abi()))
    }
    pub unsafe fn IsBootableSystemStateBackedUp(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AreComponentsSelected(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetBackupType(&self) -> VSS_BACKUP_TYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetRestoreType(&self) -> VSS_RESTORE_TYPE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetWriterFailure(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn IsPartialFileSupportEnabled(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn InstallAlternateWriter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, idwriter: Param0, clsid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), idwriter.into_param().abi(), clsid.into_param().abi()).ok()
    }
    pub unsafe fn GetIdentityInformation(&self) -> *mut IVssExamineWriterMetadata {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriterFailureEx<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr), ::core::mem::transmute(hrapplication), wszapplicationmessage.into_param().abi()).ok()
    }
    pub unsafe fn GetSessionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsWriterShuttingDown(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IVssWriterImpl {
    type Vtable = IVssWriterImpl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IVssWriterImpl> for ::windows::core::IUnknown {
    fn from(value: IVssWriterImpl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVssWriterImpl> for ::windows::core::IUnknown {
    fn from(value: &IVssWriterImpl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVssWriterImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVssWriterImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWriterImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, writerid: ::windows::core::GUID, wszwritername: super::super::Foundation::PWSTR, wszwriterinstancename: super::super::Foundation::PWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwsubscribetimeout: u32, dweventflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> *mut super::super::Foundation::PWSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszoriginalvolume: super::super::Foundation::PWSTR, ppwszsnapshotdevice: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> i32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> VSS_APPLICATION_LEVEL,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR) -> bool,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> VSS_BACKUP_TYPE,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> VSS_RESTORE_TYPE,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idwriter: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> *mut IVssExamineWriterMetadata,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idsession: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> bool,
);
pub const VSSCoordinator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe579ab5f_1cc4_44b4_bed9_de0991ff0623);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_ALTERNATE_WRITER_STATE(pub i32);
pub const VSS_AWS_UNDEFINED: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(0i32);
pub const VSS_AWS_NO_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(1i32);
pub const VSS_AWS_ALTERNATE_WRITER_EXISTS: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(2i32);
pub const VSS_AWS_THIS_IS_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(3i32);
impl ::core::convert::From<i32> for VSS_ALTERNATE_WRITER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_ALTERNATE_WRITER_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_APPLICATION_LEVEL(pub i32);
pub const VSS_APP_UNKNOWN: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(0i32);
pub const VSS_APP_SYSTEM: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(1i32);
pub const VSS_APP_BACK_END: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(2i32);
pub const VSS_APP_FRONT_END: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(3i32);
pub const VSS_APP_SYSTEM_RM: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(4i32);
pub const VSS_APP_AUTO: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(-1i32);
impl ::core::convert::From<i32> for VSS_APPLICATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_APPLICATION_LEVEL {
    type Abi = Self;
}
pub const VSS_ASSOC_NO_MAX_SPACE: i32 = -1i32;
pub const VSS_ASSOC_REMOVE: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_BACKUP_SCHEMA(pub i32);
pub const VSS_BS_UNDEFINED: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(0i32);
pub const VSS_BS_DIFFERENTIAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(1i32);
pub const VSS_BS_INCREMENTAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(2i32);
pub const VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(4i32);
pub const VSS_BS_LOG: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(8i32);
pub const VSS_BS_COPY: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(16i32);
pub const VSS_BS_TIMESTAMPED: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(32i32);
pub const VSS_BS_LAST_MODIFY: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(64i32);
pub const VSS_BS_LSN: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(128i32);
pub const VSS_BS_WRITER_SUPPORTS_NEW_TARGET: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(256i32);
pub const VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(512i32);
pub const VSS_BS_INDEPENDENT_SYSTEM_STATE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(1024i32);
pub const VSS_BS_ROLLFORWARD_RESTORE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(4096i32);
pub const VSS_BS_RESTORE_RENAME: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(8192i32);
pub const VSS_BS_AUTHORITATIVE_RESTORE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(16384i32);
pub const VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(32768i32);
impl ::core::convert::From<i32> for VSS_BACKUP_SCHEMA {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_BACKUP_SCHEMA {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_BACKUP_TYPE(pub i32);
pub const VSS_BT_UNDEFINED: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(0i32);
pub const VSS_BT_FULL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(1i32);
pub const VSS_BT_INCREMENTAL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(2i32);
pub const VSS_BT_DIFFERENTIAL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(3i32);
pub const VSS_BT_LOG: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(4i32);
pub const VSS_BT_COPY: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(5i32);
pub const VSS_BT_OTHER: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(6i32);
impl ::core::convert::From<i32> for VSS_BACKUP_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_BACKUP_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_COMPONENT_FLAGS(pub i32);
pub const VSS_CF_BACKUP_RECOVERY: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(1i32);
pub const VSS_CF_APP_ROLLBACK_RECOVERY: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(2i32);
pub const VSS_CF_NOT_SYSTEM_STATE: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(4i32);
impl ::core::convert::From<i32> for VSS_COMPONENT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_COMPONENT_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_COMPONENT_TYPE(pub i32);
pub const VSS_CT_UNDEFINED: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(0i32);
pub const VSS_CT_DATABASE: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(1i32);
pub const VSS_CT_FILEGROUP: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(2i32);
impl ::core::convert::From<i32> for VSS_COMPONENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_COMPONENT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct VSS_DIFF_AREA_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszDiffAreaVolumeName: *mut u16,
    pub m_llMaximumDiffSpace: i64,
    pub m_llAllocatedDiffSpace: i64,
    pub m_llUsedDiffSpace: i64,
}
impl VSS_DIFF_AREA_PROP {}
impl ::core::default::Default for VSS_DIFF_AREA_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VSS_DIFF_AREA_PROP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VSS_DIFF_AREA_PROP")
            .field("m_pwszVolumeName", &self.m_pwszVolumeName)
            .field("m_pwszDiffAreaVolumeName", &self.m_pwszDiffAreaVolumeName)
            .field("m_llMaximumDiffSpace", &self.m_llMaximumDiffSpace)
            .field("m_llAllocatedDiffSpace", &self.m_llAllocatedDiffSpace)
            .field("m_llUsedDiffSpace", &self.m_llUsedDiffSpace)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VSS_DIFF_AREA_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName && self.m_pwszDiffAreaVolumeName == other.m_pwszDiffAreaVolumeName && self.m_llMaximumDiffSpace == other.m_llMaximumDiffSpace && self.m_llAllocatedDiffSpace == other.m_llAllocatedDiffSpace && self.m_llUsedDiffSpace == other.m_llUsedDiffSpace
    }
}
impl ::core::cmp::Eq for VSS_DIFF_AREA_PROP {}
unsafe impl ::windows::core::Abi for VSS_DIFF_AREA_PROP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct VSS_DIFF_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
    pub m_llVolumeFreeSpace: i64,
    pub m_llVolumeTotalSpace: i64,
}
impl VSS_DIFF_VOLUME_PROP {}
impl ::core::default::Default for VSS_DIFF_VOLUME_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VSS_DIFF_VOLUME_PROP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VSS_DIFF_VOLUME_PROP").field("m_pwszVolumeName", &self.m_pwszVolumeName).field("m_pwszVolumeDisplayName", &self.m_pwszVolumeDisplayName).field("m_llVolumeFreeSpace", &self.m_llVolumeFreeSpace).field("m_llVolumeTotalSpace", &self.m_llVolumeTotalSpace).finish()
    }
}
impl ::core::cmp::PartialEq for VSS_DIFF_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName && self.m_pwszVolumeDisplayName == other.m_pwszVolumeDisplayName && self.m_llVolumeFreeSpace == other.m_llVolumeFreeSpace && self.m_llVolumeTotalSpace == other.m_llVolumeTotalSpace
    }
}
impl ::core::cmp::Eq for VSS_DIFF_VOLUME_PROP {}
unsafe impl ::windows::core::Abi for VSS_DIFF_VOLUME_PROP {
    type Abi = Self;
}
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212280i32 as _);
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212267i32 as _);
pub const VSS_E_ASRERROR_DATADISK_RDISK0: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212282i32 as _);
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212287i32 as _);
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212286i32 as _);
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212278i32 as _);
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212268i32 as _);
pub const VSS_E_ASRERROR_MISSING_DYNDISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212284i32 as _);
pub const VSS_E_ASRERROR_NO_ARCPATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212285i32 as _);
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212269i32 as _);
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212281i32 as _);
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212270i32 as _);
pub const VSS_E_ASRERROR_SHARED_CRIDISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212283i32 as _);
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212266i32 as _);
pub const VSS_E_AUTORECOVERY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212293i32 as _);
pub const VSS_E_BAD_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212543i32 as _);
pub const VSS_E_BREAK_REVERT_ID_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212298i32 as _);
pub const VSS_E_CANNOT_REVERT_DISKID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212290i32 as _);
pub const VSS_E_CLUSTER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212288i32 as _);
pub const VSS_E_CLUSTER_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212498i32 as _);
pub const VSS_E_CORRUPT_XML_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212528i32 as _);
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212271i32 as _);
pub const VSS_E_DYNAMIC_DISK_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212292i32 as _);
pub const VSS_E_FLUSH_WRITES_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212525i32 as _);
pub const VSS_E_FSS_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212265i32 as _);
pub const VSS_E_HOLD_WRITES_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212524i32 as _);
pub const VSS_E_INSUFFICIENT_STORAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212513i32 as _);
pub const VSS_E_INVALID_XML_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212527i32 as _);
pub const VSS_E_LEGACY_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212297i32 as _);
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212514i32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212510i32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212521i32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212526i32 as _);
pub const VSS_E_MISSING_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212296i32 as _);
pub const VSS_E_MISSING_HIDDEN_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212295i32 as _);
pub const VSS_E_MISSING_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212294i32 as _);
pub const VSS_E_NESTED_VOLUME_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212500i32 as _);
pub const VSS_E_NONTRANSPORTABLE_BCD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212291i32 as _);
pub const VSS_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212497i32 as _);
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212512i32 as _);
pub const VSS_E_OBJECT_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212531i32 as _);
pub const VSS_E_OBJECT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212536i32 as _);
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212541i32 as _);
pub const VSS_E_PROVIDER_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212537i32 as _);
pub const VSS_E_PROVIDER_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212540i32 as _);
pub const VSS_E_PROVIDER_VETO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212538i32 as _);
pub const VSS_E_REBOOT_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212505i32 as _);
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212509i32 as _);
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212508i32 as _);
pub const VSS_E_RESYNC_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212289i32 as _);
pub const VSS_E_REVERT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212507i32 as _);
pub const VSS_E_REVERT_VOLUME_LOST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212506i32 as _);
pub const VSS_E_SNAPSHOT_NOT_IN_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212501i32 as _);
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212522i32 as _);
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212511i32 as _);
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212504i32 as _);
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212503i32 as _);
pub const VSS_E_UNEXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212542i32 as _);
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212529i32 as _);
pub const VSS_E_UNEXPECTED_WRITER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212523i32 as _);
pub const VSS_E_UNSELECTED_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212502i32 as _);
pub const VSS_E_UNSUPPORTED_CONTEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212517i32 as _);
pub const VSS_E_VOLUME_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212515i32 as _);
pub const VSS_E_VOLUME_NOT_LOCAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212499i32 as _);
pub const VSS_E_VOLUME_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212532i32 as _);
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212530i32 as _);
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212304i32 as _);
pub const VSS_E_WRITERERROR_NONRETRYABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212300i32 as _);
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212303i32 as _);
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212490i32 as _);
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212299i32 as _);
pub const VSS_E_WRITERERROR_RETRYABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212301i32 as _);
pub const VSS_E_WRITERERROR_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212302i32 as _);
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212518i32 as _);
pub const VSS_E_WRITER_INFRASTRUCTURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212520i32 as _);
pub const VSS_E_WRITER_NOT_RESPONDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212519i32 as _);
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212279i32 as _);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_FILE_RESTORE_STATUS(pub i32);
pub const VSS_RS_UNDEFINED: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(0i32);
pub const VSS_RS_NONE: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(1i32);
pub const VSS_RS_ALL: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(2i32);
pub const VSS_RS_FAILED: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(3i32);
impl ::core::convert::From<i32> for VSS_FILE_RESTORE_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_FILE_RESTORE_STATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_FILE_SPEC_BACKUP_TYPE(pub i32);
pub const VSS_FSBT_FULL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(1i32);
pub const VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(2i32);
pub const VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(4i32);
pub const VSS_FSBT_LOG_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(8i32);
pub const VSS_FSBT_FULL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(256i32);
pub const VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(512i32);
pub const VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(1024i32);
pub const VSS_FSBT_LOG_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(2048i32);
pub const VSS_FSBT_CREATED_DURING_BACKUP: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(65536i32);
pub const VSS_FSBT_ALL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(15i32);
pub const VSS_FSBT_ALL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(3840i32);
impl ::core::convert::From<i32> for VSS_FILE_SPEC_BACKUP_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_FILE_SPEC_BACKUP_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_HARDWARE_OPTIONS(pub i32);
pub const VSS_BREAKEX_FLAG_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(1i32);
pub const VSS_BREAKEX_FLAG_MAKE_READ_WRITE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2i32);
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(4i32);
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(8i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(256i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(512i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(1024i32);
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2048i32);
impl ::core::convert::From<i32> for VSS_HARDWARE_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_HARDWARE_OPTIONS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct VSS_MGMT_OBJECT_PROP {
    pub Type: VSS_MGMT_OBJECT_TYPE,
    pub Obj: VSS_MGMT_OBJECT_UNION,
}
impl VSS_MGMT_OBJECT_PROP {}
impl ::core::default::Default for VSS_MGMT_OBJECT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSS_MGMT_OBJECT_PROP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VSS_MGMT_OBJECT_PROP {}
unsafe impl ::windows::core::Abi for VSS_MGMT_OBJECT_PROP {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_MGMT_OBJECT_TYPE(pub i32);
pub const VSS_MGMT_OBJECT_UNKNOWN: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(0i32);
pub const VSS_MGMT_OBJECT_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(1i32);
pub const VSS_MGMT_OBJECT_DIFF_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(2i32);
pub const VSS_MGMT_OBJECT_DIFF_AREA: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(3i32);
impl ::core::convert::From<i32> for VSS_MGMT_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_MGMT_OBJECT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union VSS_MGMT_OBJECT_UNION {
    pub Vol: VSS_VOLUME_PROP,
    pub DiffVol: VSS_DIFF_VOLUME_PROP,
    pub DiffArea: VSS_DIFF_AREA_PROP,
}
impl VSS_MGMT_OBJECT_UNION {}
impl ::core::default::Default for VSS_MGMT_OBJECT_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSS_MGMT_OBJECT_UNION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VSS_MGMT_OBJECT_UNION {}
unsafe impl ::windows::core::Abi for VSS_MGMT_OBJECT_UNION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl VSS_OBJECT_PROP {}
impl ::core::default::Default for VSS_OBJECT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSS_OBJECT_PROP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VSS_OBJECT_PROP {}
unsafe impl ::windows::core::Abi for VSS_OBJECT_PROP {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_OBJECT_TYPE(pub i32);
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(0i32);
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(1i32);
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(2i32);
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(3i32);
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(4i32);
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(5i32);
impl ::core::convert::From<i32> for VSS_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_OBJECT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl VSS_OBJECT_UNION {}
impl ::core::default::Default for VSS_OBJECT_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSS_OBJECT_UNION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VSS_OBJECT_UNION {}
unsafe impl ::windows::core::Abi for VSS_OBJECT_UNION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_PROTECTION_FAULT(pub i32);
pub const VSS_PROTECTION_FAULT_NONE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(0i32);
pub const VSS_PROTECTION_FAULT_DIFF_AREA_MISSING: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(1i32);
pub const VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(2i32);
pub const VSS_PROTECTION_FAULT_META_DATA_CORRUPTION: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(3i32);
pub const VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(4i32);
pub const VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(5i32);
pub const VSS_PROTECTION_FAULT_COW_READ_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(6i32);
pub const VSS_PROTECTION_FAULT_COW_WRITE_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(7i32);
pub const VSS_PROTECTION_FAULT_DIFF_AREA_FULL: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(8i32);
pub const VSS_PROTECTION_FAULT_GROW_TOO_SLOW: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(9i32);
pub const VSS_PROTECTION_FAULT_GROW_FAILED: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(10i32);
pub const VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(11i32);
pub const VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(12i32);
pub const VSS_PROTECTION_FAULT_IO_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(13i32);
pub const VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(14i32);
pub const VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(15i32);
pub const VSS_PROTECTION_FAULT_MOUNT_DURING_CLUSTER_OFFLINE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(16i32);
impl ::core::convert::From<i32> for VSS_PROTECTION_FAULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_PROTECTION_FAULT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_PROTECTION_LEVEL(pub i32);
pub const VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(0i32);
pub const VSS_PROTECTION_LEVEL_SNAPSHOT: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(1i32);
impl ::core::convert::From<i32> for VSS_PROTECTION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_PROTECTION_LEVEL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_PROVIDER_CAPABILITIES(pub i32);
pub const VSS_PRV_CAPABILITY_LEGACY: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(1i32);
pub const VSS_PRV_CAPABILITY_COMPLIANT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(2i32);
pub const VSS_PRV_CAPABILITY_LUN_REPOINT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(4i32);
pub const VSS_PRV_CAPABILITY_LUN_RESYNC: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(8i32);
pub const VSS_PRV_CAPABILITY_OFFLINE_CREATION: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(16i32);
pub const VSS_PRV_CAPABILITY_MULTIPLE_IMPORT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(32i32);
pub const VSS_PRV_CAPABILITY_RECYCLING: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(64i32);
pub const VSS_PRV_CAPABILITY_PLEX: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(128i32);
pub const VSS_PRV_CAPABILITY_DIFFERENTIAL: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(256i32);
pub const VSS_PRV_CAPABILITY_CLUSTERED: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(512i32);
impl ::core::convert::From<i32> for VSS_PROVIDER_CAPABILITIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_PROVIDER_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: ::windows::core::GUID,
    pub m_pwszProviderName: *mut u16,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: *mut u16,
    pub m_ProviderVersionId: ::windows::core::GUID,
    pub m_ClassId: ::windows::core::GUID,
}
impl VSS_PROVIDER_PROP {}
impl ::core::default::Default for VSS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VSS_PROVIDER_PROP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VSS_PROVIDER_PROP")
            .field("m_ProviderId", &self.m_ProviderId)
            .field("m_pwszProviderName", &self.m_pwszProviderName)
            .field("m_eProviderType", &self.m_eProviderType)
            .field("m_pwszProviderVersion", &self.m_pwszProviderVersion)
            .field("m_ProviderVersionId", &self.m_ProviderVersionId)
            .field("m_ClassId", &self.m_ClassId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VSS_PROVIDER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_ProviderId == other.m_ProviderId && self.m_pwszProviderName == other.m_pwszProviderName && self.m_eProviderType == other.m_eProviderType && self.m_pwszProviderVersion == other.m_pwszProviderVersion && self.m_ProviderVersionId == other.m_ProviderVersionId && self.m_ClassId == other.m_ClassId
    }
}
impl ::core::cmp::Eq for VSS_PROVIDER_PROP {}
unsafe impl ::windows::core::Abi for VSS_PROVIDER_PROP {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_PROVIDER_TYPE(pub i32);
pub const VSS_PROV_UNKNOWN: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(0i32);
pub const VSS_PROV_SYSTEM: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(1i32);
pub const VSS_PROV_SOFTWARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(2i32);
pub const VSS_PROV_HARDWARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(3i32);
pub const VSS_PROV_FILESHARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(4i32);
impl ::core::convert::From<i32> for VSS_PROVIDER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_PROVIDER_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_RECOVERY_OPTIONS(pub i32);
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(256i32);
pub const VSS_RECOVERY_NO_VOLUME_CHECK: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(512i32);
impl ::core::convert::From<i32> for VSS_RECOVERY_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_RECOVERY_OPTIONS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_RESTOREMETHOD_ENUM(pub i32);
pub const VSS_RME_UNDEFINED: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(0i32);
pub const VSS_RME_RESTORE_IF_NOT_THERE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(1i32);
pub const VSS_RME_RESTORE_IF_CAN_REPLACE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(2i32);
pub const VSS_RME_STOP_RESTORE_START: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(3i32);
pub const VSS_RME_RESTORE_TO_ALTERNATE_LOCATION: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(4i32);
pub const VSS_RME_RESTORE_AT_REBOOT: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(5i32);
pub const VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(6i32);
pub const VSS_RME_CUSTOM: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(7i32);
pub const VSS_RME_RESTORE_STOP_START: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(8i32);
impl ::core::convert::From<i32> for VSS_RESTOREMETHOD_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_RESTOREMETHOD_ENUM {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_RESTORE_TARGET(pub i32);
pub const VSS_RT_UNDEFINED: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(0i32);
pub const VSS_RT_ORIGINAL: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(1i32);
pub const VSS_RT_ALTERNATE: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(2i32);
pub const VSS_RT_DIRECTED: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(3i32);
pub const VSS_RT_ORIGINAL_LOCATION: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(4i32);
impl ::core::convert::From<i32> for VSS_RESTORE_TARGET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_RESTORE_TARGET {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_RESTORE_TYPE(pub i32);
pub const VSS_RTYPE_UNDEFINED: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(0i32);
pub const VSS_RTYPE_BY_COPY: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(1i32);
pub const VSS_RTYPE_IMPORT: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(2i32);
pub const VSS_RTYPE_OTHER: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(3i32);
impl ::core::convert::From<i32> for VSS_RESTORE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_RESTORE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_ROLLFORWARD_TYPE(pub i32);
pub const VSS_RF_UNDEFINED: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(0i32);
pub const VSS_RF_NONE: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(1i32);
pub const VSS_RF_ALL: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(2i32);
pub const VSS_RF_PARTIAL: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(3i32);
impl ::core::convert::From<i32> for VSS_ROLLFORWARD_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_ROLLFORWARD_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_SNAPSHOT_COMPATIBILITY(pub i32);
pub const VSS_SC_DISABLE_DEFRAG: VSS_SNAPSHOT_COMPATIBILITY = VSS_SNAPSHOT_COMPATIBILITY(1i32);
pub const VSS_SC_DISABLE_CONTENTINDEX: VSS_SNAPSHOT_COMPATIBILITY = VSS_SNAPSHOT_COMPATIBILITY(2i32);
impl ::core::convert::From<i32> for VSS_SNAPSHOT_COMPATIBILITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_SNAPSHOT_COMPATIBILITY {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_SNAPSHOT_CONTEXT(pub i32);
pub const VSS_CTX_BACKUP: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(0i32);
pub const VSS_CTX_FILE_SHARE_BACKUP: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(16i32);
pub const VSS_CTX_NAS_ROLLBACK: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(25i32);
pub const VSS_CTX_APP_ROLLBACK: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(9i32);
pub const VSS_CTX_CLIENT_ACCESSIBLE: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(29i32);
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(13i32);
pub const VSS_CTX_ALL: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(-1i32);
impl ::core::convert::From<i32> for VSS_SNAPSHOT_CONTEXT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_SNAPSHOT_CONTEXT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: ::windows::core::GUID,
    pub m_SnapshotSetId: ::windows::core::GUID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: *mut u16,
    pub m_pwszOriginalVolumeName: *mut u16,
    pub m_pwszOriginatingMachine: *mut u16,
    pub m_pwszServiceMachine: *mut u16,
    pub m_pwszExposedName: *mut u16,
    pub m_pwszExposedPath: *mut u16,
    pub m_ProviderId: ::windows::core::GUID,
    pub m_lSnapshotAttributes: i32,
    pub m_tsCreationTimestamp: i64,
    pub m_eStatus: VSS_SNAPSHOT_STATE,
}
impl VSS_SNAPSHOT_PROP {}
impl ::core::default::Default for VSS_SNAPSHOT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_PROP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VSS_SNAPSHOT_PROP")
            .field("m_SnapshotId", &self.m_SnapshotId)
            .field("m_SnapshotSetId", &self.m_SnapshotSetId)
            .field("m_lSnapshotsCount", &self.m_lSnapshotsCount)
            .field("m_pwszSnapshotDeviceObject", &self.m_pwszSnapshotDeviceObject)
            .field("m_pwszOriginalVolumeName", &self.m_pwszOriginalVolumeName)
            .field("m_pwszOriginatingMachine", &self.m_pwszOriginatingMachine)
            .field("m_pwszServiceMachine", &self.m_pwszServiceMachine)
            .field("m_pwszExposedName", &self.m_pwszExposedName)
            .field("m_pwszExposedPath", &self.m_pwszExposedPath)
            .field("m_ProviderId", &self.m_ProviderId)
            .field("m_lSnapshotAttributes", &self.m_lSnapshotAttributes)
            .field("m_tsCreationTimestamp", &self.m_tsCreationTimestamp)
            .field("m_eStatus", &self.m_eStatus)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VSS_SNAPSHOT_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_SnapshotId == other.m_SnapshotId
            && self.m_SnapshotSetId == other.m_SnapshotSetId
            && self.m_lSnapshotsCount == other.m_lSnapshotsCount
            && self.m_pwszSnapshotDeviceObject == other.m_pwszSnapshotDeviceObject
            && self.m_pwszOriginalVolumeName == other.m_pwszOriginalVolumeName
            && self.m_pwszOriginatingMachine == other.m_pwszOriginatingMachine
            && self.m_pwszServiceMachine == other.m_pwszServiceMachine
            && self.m_pwszExposedName == other.m_pwszExposedName
            && self.m_pwszExposedPath == other.m_pwszExposedPath
            && self.m_ProviderId == other.m_ProviderId
            && self.m_lSnapshotAttributes == other.m_lSnapshotAttributes
            && self.m_tsCreationTimestamp == other.m_tsCreationTimestamp
            && self.m_eStatus == other.m_eStatus
    }
}
impl ::core::cmp::Eq for VSS_SNAPSHOT_PROP {}
unsafe impl ::windows::core::Abi for VSS_SNAPSHOT_PROP {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_SNAPSHOT_PROPERTY_ID(pub i32);
pub const VSS_SPROPID_UNKNOWN: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(0i32);
pub const VSS_SPROPID_SNAPSHOT_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(1i32);
pub const VSS_SPROPID_SNAPSHOT_SET_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(2i32);
pub const VSS_SPROPID_SNAPSHOTS_COUNT: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(3i32);
pub const VSS_SPROPID_SNAPSHOT_DEVICE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(4i32);
pub const VSS_SPROPID_ORIGINAL_VOLUME: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(5i32);
pub const VSS_SPROPID_ORIGINATING_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(6i32);
pub const VSS_SPROPID_SERVICE_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(7i32);
pub const VSS_SPROPID_EXPOSED_NAME: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(8i32);
pub const VSS_SPROPID_EXPOSED_PATH: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(9i32);
pub const VSS_SPROPID_PROVIDER_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(10i32);
pub const VSS_SPROPID_SNAPSHOT_ATTRIBUTES: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(11i32);
pub const VSS_SPROPID_CREATION_TIMESTAMP: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(12i32);
pub const VSS_SPROPID_STATUS: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(13i32);
impl ::core::convert::From<i32> for VSS_SNAPSHOT_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_SNAPSHOT_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_SNAPSHOT_STATE(pub i32);
pub const VSS_SS_UNKNOWN: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(0i32);
pub const VSS_SS_PREPARING: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(1i32);
pub const VSS_SS_PROCESSING_PREPARE: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(2i32);
pub const VSS_SS_PREPARED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(3i32);
pub const VSS_SS_PROCESSING_PRECOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(4i32);
pub const VSS_SS_PRECOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(5i32);
pub const VSS_SS_PROCESSING_COMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(6i32);
pub const VSS_SS_COMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(7i32);
pub const VSS_SS_PROCESSING_POSTCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(8i32);
pub const VSS_SS_PROCESSING_PREFINALCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(9i32);
pub const VSS_SS_PREFINALCOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(10i32);
pub const VSS_SS_PROCESSING_POSTFINALCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(11i32);
pub const VSS_SS_CREATED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(12i32);
pub const VSS_SS_ABORTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(13i32);
pub const VSS_SS_DELETED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(14i32);
pub const VSS_SS_POSTCOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(15i32);
pub const VSS_SS_COUNT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(16i32);
impl ::core::convert::From<i32> for VSS_SNAPSHOT_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_SNAPSHOT_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_SOURCE_TYPE(pub i32);
pub const VSS_ST_UNDEFINED: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(0i32);
pub const VSS_ST_TRANSACTEDDB: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(1i32);
pub const VSS_ST_NONTRANSACTEDDB: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(2i32);
pub const VSS_ST_OTHER: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(3i32);
impl ::core::convert::From<i32> for VSS_SOURCE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_SOURCE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_SUBSCRIBE_MASK(pub i32);
pub const VSS_SM_POST_SNAPSHOT_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(1i32);
pub const VSS_SM_BACKUP_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(2i32);
pub const VSS_SM_RESTORE_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(4i32);
pub const VSS_SM_IO_THROTTLING_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(8i32);
pub const VSS_SM_ALL_FLAGS: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(-1i32);
impl ::core::convert::From<i32> for VSS_SUBSCRIBE_MASK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_SUBSCRIBE_MASK {
    type Abi = Self;
}
pub const VSS_S_ASYNC_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(271115i32 as _);
pub const VSS_S_ASYNC_FINISHED: ::windows::core::HRESULT = ::windows::core::HRESULT(271114i32 as _);
pub const VSS_S_ASYNC_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(271113i32 as _);
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(271137i32 as _);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_USAGE_TYPE(pub i32);
pub const VSS_UT_UNDEFINED: VSS_USAGE_TYPE = VSS_USAGE_TYPE(0i32);
pub const VSS_UT_BOOTABLESYSTEMSTATE: VSS_USAGE_TYPE = VSS_USAGE_TYPE(1i32);
pub const VSS_UT_SYSTEMSERVICE: VSS_USAGE_TYPE = VSS_USAGE_TYPE(2i32);
pub const VSS_UT_USERDATA: VSS_USAGE_TYPE = VSS_USAGE_TYPE(3i32);
pub const VSS_UT_OTHER: VSS_USAGE_TYPE = VSS_USAGE_TYPE(4i32);
impl ::core::convert::From<i32> for VSS_USAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_USAGE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct VSS_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
}
impl VSS_VOLUME_PROP {}
impl ::core::default::Default for VSS_VOLUME_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VSS_VOLUME_PROP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VSS_VOLUME_PROP").field("m_pwszVolumeName", &self.m_pwszVolumeName).field("m_pwszVolumeDisplayName", &self.m_pwszVolumeDisplayName).finish()
    }
}
impl ::core::cmp::PartialEq for VSS_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName && self.m_pwszVolumeDisplayName == other.m_pwszVolumeDisplayName
    }
}
impl ::core::cmp::Eq for VSS_VOLUME_PROP {}
unsafe impl ::windows::core::Abi for VSS_VOLUME_PROP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VSS_VOLUME_PROTECTION_INFO {
    pub m_protectionLevel: VSS_PROTECTION_LEVEL,
    pub m_volumeIsOfflineForProtection: super::super::Foundation::BOOL,
    pub m_protectionFault: VSS_PROTECTION_FAULT,
    pub m_failureStatus: i32,
    pub m_volumeHasUnusedDiffArea: super::super::Foundation::BOOL,
    pub m_reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl VSS_VOLUME_PROTECTION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VSS_VOLUME_PROTECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VSS_VOLUME_PROTECTION_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VSS_VOLUME_PROTECTION_INFO")
            .field("m_protectionLevel", &self.m_protectionLevel)
            .field("m_volumeIsOfflineForProtection", &self.m_volumeIsOfflineForProtection)
            .field("m_protectionFault", &self.m_protectionFault)
            .field("m_failureStatus", &self.m_failureStatus)
            .field("m_volumeHasUnusedDiffArea", &self.m_volumeHasUnusedDiffArea)
            .field("m_reserved", &self.m_reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VSS_VOLUME_PROTECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.m_protectionLevel == other.m_protectionLevel && self.m_volumeIsOfflineForProtection == other.m_volumeIsOfflineForProtection && self.m_protectionFault == other.m_protectionFault && self.m_failureStatus == other.m_failureStatus && self.m_volumeHasUnusedDiffArea == other.m_volumeHasUnusedDiffArea && self.m_reserved == other.m_reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VSS_VOLUME_PROTECTION_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VSS_VOLUME_PROTECTION_INFO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_VOLUME_SNAPSHOT_ATTRIBUTES(pub i32);
pub const VSS_VOLSNAP_ATTR_PERSISTENT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(1i32);
pub const VSS_VOLSNAP_ATTR_NO_AUTORECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(2i32);
pub const VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(4i32);
pub const VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(8i32);
pub const VSS_VOLSNAP_ATTR_NO_WRITERS: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(16i32);
pub const VSS_VOLSNAP_ATTR_TRANSPORTABLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(32i32);
pub const VSS_VOLSNAP_ATTR_NOT_SURFACED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(64i32);
pub const VSS_VOLSNAP_ATTR_NOT_TRANSACTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(128i32);
pub const VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(65536i32);
pub const VSS_VOLSNAP_ATTR_DIFFERENTIAL: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(131072i32);
pub const VSS_VOLSNAP_ATTR_PLEX: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(262144i32);
pub const VSS_VOLSNAP_ATTR_IMPORTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(524288i32);
pub const VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(1048576i32);
pub const VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(2097152i32);
pub const VSS_VOLSNAP_ATTR_AUTORECOVER: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(4194304i32);
pub const VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(8388608i32);
pub const VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(16777216i32);
pub const VSS_VOLSNAP_ATTR_TXF_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(33554432i32);
pub const VSS_VOLSNAP_ATTR_FILE_SHARE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(67108864i32);
impl ::core::convert::From<i32> for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_WRITERRESTORE_ENUM(pub i32);
pub const VSS_WRE_UNDEFINED: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(0i32);
pub const VSS_WRE_NEVER: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(1i32);
pub const VSS_WRE_IF_REPLACE_FAILS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(2i32);
pub const VSS_WRE_ALWAYS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(3i32);
impl ::core::convert::From<i32> for VSS_WRITERRESTORE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_WRITERRESTORE_ENUM {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VSS_WRITER_STATE(pub i32);
pub const VSS_WS_UNKNOWN: VSS_WRITER_STATE = VSS_WRITER_STATE(0i32);
pub const VSS_WS_STABLE: VSS_WRITER_STATE = VSS_WRITER_STATE(1i32);
pub const VSS_WS_WAITING_FOR_FREEZE: VSS_WRITER_STATE = VSS_WRITER_STATE(2i32);
pub const VSS_WS_WAITING_FOR_THAW: VSS_WRITER_STATE = VSS_WRITER_STATE(3i32);
pub const VSS_WS_WAITING_FOR_POST_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(4i32);
pub const VSS_WS_WAITING_FOR_BACKUP_COMPLETE: VSS_WRITER_STATE = VSS_WRITER_STATE(5i32);
pub const VSS_WS_FAILED_AT_IDENTIFY: VSS_WRITER_STATE = VSS_WRITER_STATE(6i32);
pub const VSS_WS_FAILED_AT_PREPARE_BACKUP: VSS_WRITER_STATE = VSS_WRITER_STATE(7i32);
pub const VSS_WS_FAILED_AT_PREPARE_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(8i32);
pub const VSS_WS_FAILED_AT_FREEZE: VSS_WRITER_STATE = VSS_WRITER_STATE(9i32);
pub const VSS_WS_FAILED_AT_THAW: VSS_WRITER_STATE = VSS_WRITER_STATE(10i32);
pub const VSS_WS_FAILED_AT_POST_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(11i32);
pub const VSS_WS_FAILED_AT_BACKUP_COMPLETE: VSS_WRITER_STATE = VSS_WRITER_STATE(12i32);
pub const VSS_WS_FAILED_AT_PRE_RESTORE: VSS_WRITER_STATE = VSS_WRITER_STATE(13i32);
pub const VSS_WS_FAILED_AT_POST_RESTORE: VSS_WRITER_STATE = VSS_WRITER_STATE(14i32);
pub const VSS_WS_FAILED_AT_BACKUPSHUTDOWN: VSS_WRITER_STATE = VSS_WRITER_STATE(15i32);
pub const VSS_WS_COUNT: VSS_WRITER_STATE = VSS_WRITER_STATE(16i32);
impl ::core::convert::From<i32> for VSS_WRITER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VSS_WRITER_STATE {
    type Abi = Self;
}
pub const VssSnapshotMgmt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b5a2c52_3eb9_470a_96e2_6c6d4570e40f);
