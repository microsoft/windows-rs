#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub unsafe fn CreateVssExpressWriterInternal() -> ::windows::runtime::Result<IVssExpressWriter> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateVssExpressWriterInternal(
                ppwriter: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IVssExpressWriter as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        CreateVssExpressWriterInternal(&mut result__).from_abi::<IVssExpressWriter>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssAdmin(::windows::runtime::IUnknown);
impl IVssAdmin {
    pub unsafe fn RegisterProvider<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        pproviderid: Param0,
        classid: Param1,
        pwszprovidername: *const u16,
        eprovidertype: VSS_PROVIDER_TYPE,
        pwszproviderversion: *const u16,
        providerversionid: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pproviderid.into_param().abi(),
            classid.into_param().abi(),
            ::std::mem::transmute(pwszprovidername),
            ::std::mem::transmute(eprovidertype),
            ::std::mem::transmute(pwszproviderversion),
            providerversionid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn UnregisterProvider<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        providerid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            providerid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn QueryProviders(&self) -> ::windows::runtime::Result<IVssEnumObject> {
        let mut result__: <IVssEnumObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IVssEnumObject>(result__)
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssAdmin {
    type Vtable = IVssAdmin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2012043670,
        12131,
        4563,
        [138, 57, 0, 192, 79, 114, 216, 227],
    );
}
impl ::std::convert::From<IVssAdmin> for ::windows::runtime::IUnknown {
    fn from(value: IVssAdmin) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssAdmin> for ::windows::runtime::IUnknown {
    fn from(value: &IVssAdmin) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssAdmin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssAdmin {
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
pub struct IVssAdmin_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pproviderid: ::windows::runtime::GUID,
        classid: ::windows::runtime::GUID,
        pwszprovidername: *const u16,
        eprovidertype: VSS_PROVIDER_TYPE,
        pwszproviderversion: *const u16,
        providerversionid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        providerid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
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
pub struct IVssAdminEx(::windows::runtime::IUnknown);
impl IVssAdminEx {
    pub unsafe fn RegisterProvider<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        pproviderid: Param0,
        classid: Param1,
        pwszprovidername: *const u16,
        eprovidertype: VSS_PROVIDER_TYPE,
        pwszproviderversion: *const u16,
        providerversionid: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pproviderid.into_param().abi(),
            classid.into_param().abi(),
            ::std::mem::transmute(pwszprovidername),
            ::std::mem::transmute(eprovidertype),
            ::std::mem::transmute(pwszproviderversion),
            providerversionid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn UnregisterProvider<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        providerid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            providerid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn QueryProviders(&self) -> ::windows::runtime::Result<IVssEnumObject> {
        let mut result__: <IVssEnumObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IVssEnumObject>(result__)
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetProviderCapability<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        pproviderid: Param0,
    ) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            pproviderid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u64>(result__)
    }
    pub unsafe fn GetProviderContext<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        providerid: Param0,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            providerid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetProviderContext<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        providerid: Param0,
        lcontext: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            providerid.into_param().abi(),
            ::std::mem::transmute(lcontext),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssAdminEx {
    type Vtable = IVssAdminEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2019076600,
        45562,
        16806,
        [150, 79, 185, 179, 107, 140, 216, 216],
    );
}
impl ::std::convert::From<IVssAdminEx> for ::windows::runtime::IUnknown {
    fn from(value: IVssAdminEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssAdminEx> for ::windows::runtime::IUnknown {
    fn from(value: &IVssAdminEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssAdminEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssAdminEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IVssAdminEx> for IVssAdmin {
    fn from(value: IVssAdminEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssAdminEx> for IVssAdmin {
    fn from(value: &IVssAdminEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssAdmin> for IVssAdminEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssAdmin> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVssAdmin>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssAdmin> for &IVssAdminEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssAdmin> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVssAdmin>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAdminEx_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pproviderid: ::windows::runtime::GUID,
        classid: ::windows::runtime::GUID,
        pwszprovidername: *const u16,
        eprovidertype: VSS_PROVIDER_TYPE,
        pwszproviderversion: *const u16,
        providerversionid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        providerid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pproviderid: ::windows::runtime::GUID,
        plloriginalcapabilitymask: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        providerid: ::windows::runtime::GUID,
        plcontext: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        providerid: ::windows::runtime::GUID,
        lcontext: i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssAsync(::windows::runtime::IUnknown);
impl IVssAsync {
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Wait(&self, dwmilliseconds: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwmilliseconds),
        )
        .ok()
    }
    pub unsafe fn QueryStatus(
        &self,
        phrresult: *mut ::windows::runtime::HRESULT,
        preserved: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(phrresult),
            ::std::mem::transmute(preserved),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssAsync {
    type Vtable = IVssAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1350318004,
        53083,
        20117,
        [176, 175, 20, 235, 151, 103, 70, 126],
    );
}
impl ::std::convert::From<IVssAsync> for ::windows::runtime::IUnknown {
    fn from(value: IVssAsync) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssAsync> for ::windows::runtime::IUnknown {
    fn from(value: &IVssAsync) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssAsync {
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
pub struct IVssAsync_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwmilliseconds: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phrresult: *mut ::windows::runtime::HRESULT,
        preserved: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssComponent(::windows::runtime::IUnknown);
impl IVssComponent {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(
        &self,
        pbstrpath: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrpath),
        )
        .ok()
    }
    pub unsafe fn GetComponentType(
        &self,
        pct: *mut VSS_COMPONENT_TYPE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pct),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(
        &self,
        pbstrname: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrname),
        )
        .ok()
    }
    pub unsafe fn GetBackupSucceeded(
        &self,
        pbsucceeded: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbsucceeded),
        )
        .ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(
        &self,
        pcmappings: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcmappings),
        )
        .ok()
    }
    pub unsafe fn GetAlternateLocationMapping(
        &self,
        imapping: u32,
    ) -> ::windows::runtime::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(imapping),
            &mut result__,
        )
        .from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupMetadata<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszdata: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            wszdata.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupMetadata(
        &self,
        pbstrdata: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrdata),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPartialFile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszpath: Param0,
        wszfilename: Param1,
        wszranges: Param2,
        wszmetadata: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilename.into_param().abi(),
            wszranges.into_param().abi(),
            wszmetadata.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPartialFileCount(
        &self,
        pcpartialfiles: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcpartialfiles),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartialFile(
        &self,
        ipartialfile: u32,
        pbstrpath: *mut super::super::Foundation::BSTR,
        pbstrfilename: *mut super::super::Foundation::BSTR,
        pbstrrange: *mut super::super::Foundation::BSTR,
        pbstrmetadata: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ipartialfile),
            ::std::mem::transmute(pbstrpath),
            ::std::mem::transmute(pbstrfilename),
            ::std::mem::transmute(pbstrrange),
            ::std::mem::transmute(pbstrmetadata),
        )
        .ok()
    }
    pub unsafe fn IsSelectedForRestore(
        &self,
        pbselectedforrestore: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbselectedforrestore),
        )
        .ok()
    }
    pub unsafe fn GetAdditionalRestores(
        &self,
        pbadditionalrestores: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbadditionalrestores),
        )
        .ok()
    }
    pub unsafe fn GetNewTargetCount(
        &self,
        pcnewtarget: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcnewtarget),
        )
        .ok()
    }
    pub unsafe fn GetNewTarget(
        &self,
        inewtarget: u32,
    ) -> ::windows::runtime::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(inewtarget),
            &mut result__,
        )
        .from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDirectedTarget<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszsourcepath: Param0,
        wszsourcefilename: Param1,
        wszsourcerangelist: Param2,
        wszdestinationpath: Param3,
        wszdestinationfilename: Param4,
        wszdestinationrangelist: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            wszsourcepath.into_param().abi(),
            wszsourcefilename.into_param().abi(),
            wszsourcerangelist.into_param().abi(),
            wszdestinationpath.into_param().abi(),
            wszdestinationfilename.into_param().abi(),
            wszdestinationrangelist.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDirectedTargetCount(
        &self,
        pcdirectedtarget: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcdirectedtarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectedTarget(
        &self,
        idirectedtarget: u32,
        pbstrsourcepath: *mut super::super::Foundation::BSTR,
        pbstrsourcefilename: *mut super::super::Foundation::BSTR,
        pbstrsourcerangelist: *mut super::super::Foundation::BSTR,
        pbstrdestinationpath: *mut super::super::Foundation::BSTR,
        pbstrdestinationfilename: *mut super::super::Foundation::BSTR,
        pbstrdestinationrangelist: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idirectedtarget),
            ::std::mem::transmute(pbstrsourcepath),
            ::std::mem::transmute(pbstrsourcefilename),
            ::std::mem::transmute(pbstrsourcerangelist),
            ::std::mem::transmute(pbstrdestinationpath),
            ::std::mem::transmute(pbstrdestinationfilename),
            ::std::mem::transmute(pbstrdestinationrangelist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestoreMetadata<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszrestoremetadata: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            wszrestoremetadata.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreMetadata(
        &self,
        pbstrrestoremetadata: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrrestoremetadata),
        )
        .ok()
    }
    pub unsafe fn SetRestoreTarget(
        &self,
        target: VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(target),
        )
        .ok()
    }
    pub unsafe fn GetRestoreTarget(
        &self,
        ptarget: *mut VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreRestoreFailureMsg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszprerestorefailuremsg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            wszprerestorefailuremsg.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreRestoreFailureMsg(
        &self,
        pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrprerestorefailuremsg),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostRestoreFailureMsg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszpostrestorefailuremsg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            wszpostrestorefailuremsg.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostRestoreFailureMsg(
        &self,
        pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrpostrestorefailuremsg),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupStamp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszbackupstamp: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            wszbackupstamp.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupStamp(
        &self,
        pbstrbackupstamp: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrbackupstamp),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreviousBackupStamp(
        &self,
        pbstrbackupstamp: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrbackupstamp),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupOptions(
        &self,
        pbstrbackupoptions: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrbackupoptions),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreOptions(
        &self,
        pbstrrestoreoptions: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrrestoreoptions),
        )
        .ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(
        &self,
        pcrestoresubcomponent: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcrestoresubcomponent),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreSubcomponent(
        &self,
        icomponent: u32,
        pbstrlogicalpath: *mut super::super::Foundation::BSTR,
        pbstrcomponentname: *mut super::super::Foundation::BSTR,
        pbrepair: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(icomponent),
            ::std::mem::transmute(pbstrlogicalpath),
            ::std::mem::transmute(pbstrcomponentname),
            ::std::mem::transmute(pbrepair),
        )
        .ok()
    }
    pub unsafe fn GetFileRestoreStatus(
        &self,
        pstatus: *mut VSS_FILE_RESTORE_STATUS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pstatus),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>,
    >(
        &self,
        wszpath: Param0,
        wszfilespec: Param1,
        brecursive: Param2,
        ftlastmodifytime: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            brecursive.into_param().abi(),
            ftlastmodifytime.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        wszpath: Param0,
        wszfilespec: Param1,
        brecursive: Param2,
        bstrlsnstring: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            brecursive.into_param().abi(),
            bstrlsnstring.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDifferencedFilesCount(
        &self,
        pcdifferencedfiles: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcdifferencedfiles),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(
        &self,
        idifferencedfile: u32,
        pbstrpath: *mut super::super::Foundation::BSTR,
        pbstrfilespec: *mut super::super::Foundation::BSTR,
        pbrecursive: *mut super::super::Foundation::BOOL,
        pbstrlsnstring: *mut super::super::Foundation::BSTR,
        pftlastmodifytime: *mut super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idifferencedfile),
            ::std::mem::transmute(pbstrpath),
            ::std::mem::transmute(pbstrfilespec),
            ::std::mem::transmute(pbrecursive),
            ::std::mem::transmute(pbstrlsnstring),
            ::std::mem::transmute(pftlastmodifytime),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssComponent {
    type Vtable = IVssComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3536268438,
        49441,
        17688,
        [182, 39, 229, 169, 61, 1, 14, 173],
    );
}
impl ::std::convert::From<IVssComponent> for ::windows::runtime::IUnknown {
    fn from(value: IVssComponent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssComponent> for ::windows::runtime::IUnknown {
    fn from(value: &IVssComponent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssComponent {
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
pub struct IVssComponent_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pct: *mut VSS_COMPONENT_TYPE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbsucceeded: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcmappings: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        imapping: u32,
        ppfiledesc: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszdata: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrdata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilename: super::super::Foundation::PWSTR,
        wszranges: super::super::Foundation::PWSTR,
        wszmetadata: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcpartialfiles: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ipartialfile: u32,
        pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrfilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrrange: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrmetadata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbselectedforrestore: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbadditionalrestores: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcnewtarget: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inewtarget: u32,
        ppfiledesc: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszsourcepath: super::super::Foundation::PWSTR,
        wszsourcefilename: super::super::Foundation::PWSTR,
        wszsourcerangelist: super::super::Foundation::PWSTR,
        wszdestinationpath: super::super::Foundation::PWSTR,
        wszdestinationfilename: super::super::Foundation::PWSTR,
        wszdestinationrangelist: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcdirectedtarget: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idirectedtarget: u32,
        pbstrsourcepath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcefilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcerangelist: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationfilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationrangelist: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszrestoremetadata: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrrestoremetadata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptarget: *mut VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszprerestorefailuremsg: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrprerestorefailuremsg: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpostrestorefailuremsg: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpostrestorefailuremsg: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszbackupstamp: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrbackupstamp: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrbackupstamp: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrbackupoptions: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrrestoreoptions: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcrestoresubcomponent: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        icomponent: u32,
        pbstrlogicalpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrcomponentname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbrepair: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstatus: *mut VSS_FILE_RESTORE_STATUS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: super::super::Foundation::BOOL,
        ftlastmodifytime: super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: super::super::Foundation::BOOL,
        bstrlsnstring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcdifferencedfiles: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idifferencedfile: u32,
        pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrfilespec: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbrecursive: *mut super::super::Foundation::BOOL,
        pbstrlsnstring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pftlastmodifytime: *mut super::super::Foundation::FILETIME,
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
pub struct IVssComponentEx(::windows::runtime::IUnknown);
impl IVssComponentEx {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(
        &self,
        pbstrpath: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrpath),
        )
        .ok()
    }
    pub unsafe fn GetComponentType(
        &self,
        pct: *mut VSS_COMPONENT_TYPE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pct),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(
        &self,
        pbstrname: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrname),
        )
        .ok()
    }
    pub unsafe fn GetBackupSucceeded(
        &self,
        pbsucceeded: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbsucceeded),
        )
        .ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(
        &self,
        pcmappings: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcmappings),
        )
        .ok()
    }
    pub unsafe fn GetAlternateLocationMapping(
        &self,
        imapping: u32,
    ) -> ::windows::runtime::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(imapping),
            &mut result__,
        )
        .from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupMetadata<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszdata: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            wszdata.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupMetadata(
        &self,
        pbstrdata: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrdata),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPartialFile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszpath: Param0,
        wszfilename: Param1,
        wszranges: Param2,
        wszmetadata: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilename.into_param().abi(),
            wszranges.into_param().abi(),
            wszmetadata.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPartialFileCount(
        &self,
        pcpartialfiles: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcpartialfiles),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartialFile(
        &self,
        ipartialfile: u32,
        pbstrpath: *mut super::super::Foundation::BSTR,
        pbstrfilename: *mut super::super::Foundation::BSTR,
        pbstrrange: *mut super::super::Foundation::BSTR,
        pbstrmetadata: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ipartialfile),
            ::std::mem::transmute(pbstrpath),
            ::std::mem::transmute(pbstrfilename),
            ::std::mem::transmute(pbstrrange),
            ::std::mem::transmute(pbstrmetadata),
        )
        .ok()
    }
    pub unsafe fn IsSelectedForRestore(
        &self,
        pbselectedforrestore: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbselectedforrestore),
        )
        .ok()
    }
    pub unsafe fn GetAdditionalRestores(
        &self,
        pbadditionalrestores: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbadditionalrestores),
        )
        .ok()
    }
    pub unsafe fn GetNewTargetCount(
        &self,
        pcnewtarget: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcnewtarget),
        )
        .ok()
    }
    pub unsafe fn GetNewTarget(
        &self,
        inewtarget: u32,
    ) -> ::windows::runtime::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(inewtarget),
            &mut result__,
        )
        .from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDirectedTarget<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszsourcepath: Param0,
        wszsourcefilename: Param1,
        wszsourcerangelist: Param2,
        wszdestinationpath: Param3,
        wszdestinationfilename: Param4,
        wszdestinationrangelist: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            wszsourcepath.into_param().abi(),
            wszsourcefilename.into_param().abi(),
            wszsourcerangelist.into_param().abi(),
            wszdestinationpath.into_param().abi(),
            wszdestinationfilename.into_param().abi(),
            wszdestinationrangelist.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDirectedTargetCount(
        &self,
        pcdirectedtarget: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcdirectedtarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectedTarget(
        &self,
        idirectedtarget: u32,
        pbstrsourcepath: *mut super::super::Foundation::BSTR,
        pbstrsourcefilename: *mut super::super::Foundation::BSTR,
        pbstrsourcerangelist: *mut super::super::Foundation::BSTR,
        pbstrdestinationpath: *mut super::super::Foundation::BSTR,
        pbstrdestinationfilename: *mut super::super::Foundation::BSTR,
        pbstrdestinationrangelist: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idirectedtarget),
            ::std::mem::transmute(pbstrsourcepath),
            ::std::mem::transmute(pbstrsourcefilename),
            ::std::mem::transmute(pbstrsourcerangelist),
            ::std::mem::transmute(pbstrdestinationpath),
            ::std::mem::transmute(pbstrdestinationfilename),
            ::std::mem::transmute(pbstrdestinationrangelist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestoreMetadata<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszrestoremetadata: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            wszrestoremetadata.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreMetadata(
        &self,
        pbstrrestoremetadata: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrrestoremetadata),
        )
        .ok()
    }
    pub unsafe fn SetRestoreTarget(
        &self,
        target: VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(target),
        )
        .ok()
    }
    pub unsafe fn GetRestoreTarget(
        &self,
        ptarget: *mut VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreRestoreFailureMsg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszprerestorefailuremsg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            wszprerestorefailuremsg.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreRestoreFailureMsg(
        &self,
        pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrprerestorefailuremsg),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostRestoreFailureMsg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszpostrestorefailuremsg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            wszpostrestorefailuremsg.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostRestoreFailureMsg(
        &self,
        pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrpostrestorefailuremsg),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupStamp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszbackupstamp: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            wszbackupstamp.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupStamp(
        &self,
        pbstrbackupstamp: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrbackupstamp),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreviousBackupStamp(
        &self,
        pbstrbackupstamp: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrbackupstamp),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupOptions(
        &self,
        pbstrbackupoptions: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrbackupoptions),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreOptions(
        &self,
        pbstrrestoreoptions: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrrestoreoptions),
        )
        .ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(
        &self,
        pcrestoresubcomponent: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcrestoresubcomponent),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreSubcomponent(
        &self,
        icomponent: u32,
        pbstrlogicalpath: *mut super::super::Foundation::BSTR,
        pbstrcomponentname: *mut super::super::Foundation::BSTR,
        pbrepair: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(icomponent),
            ::std::mem::transmute(pbstrlogicalpath),
            ::std::mem::transmute(pbstrcomponentname),
            ::std::mem::transmute(pbrepair),
        )
        .ok()
    }
    pub unsafe fn GetFileRestoreStatus(
        &self,
        pstatus: *mut VSS_FILE_RESTORE_STATUS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pstatus),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>,
    >(
        &self,
        wszpath: Param0,
        wszfilespec: Param1,
        brecursive: Param2,
        ftlastmodifytime: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            brecursive.into_param().abi(),
            ftlastmodifytime.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        wszpath: Param0,
        wszfilespec: Param1,
        brecursive: Param2,
        bstrlsnstring: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            brecursive.into_param().abi(),
            bstrlsnstring.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDifferencedFilesCount(
        &self,
        pcdifferencedfiles: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcdifferencedfiles),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(
        &self,
        idifferencedfile: u32,
        pbstrpath: *mut super::super::Foundation::BSTR,
        pbstrfilespec: *mut super::super::Foundation::BSTR,
        pbrecursive: *mut super::super::Foundation::BOOL,
        pbstrlsnstring: *mut super::super::Foundation::BSTR,
        pftlastmodifytime: *mut super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idifferencedfile),
            ::std::mem::transmute(pbstrpath),
            ::std::mem::transmute(pbstrfilespec),
            ::std::mem::transmute(pbrecursive),
            ::std::mem::transmute(pbstrlsnstring),
            ::std::mem::transmute(pftlastmodifytime),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPrepareForBackupFailureMsg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszfailuremsg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            wszfailuremsg.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostSnapshotFailureMsg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszfailuremsg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            wszfailuremsg.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrepareForBackupFailureMsg(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostSnapshotFailureMsg(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> ::windows::runtime::Result<bool> {
        let mut result__: <bool as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<bool>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRollForward(
        &self,
        prolltype: *mut VSS_ROLLFORWARD_TYPE,
        pbstrpoint: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(prolltype),
            ::std::mem::transmute(pbstrpoint),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreName(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVssComponentEx {
    type Vtable = IVssComponentEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        359435102,
        61745,
        19415,
        [156, 151, 209, 146, 59, 231, 225, 250],
    );
}
impl ::std::convert::From<IVssComponentEx> for ::windows::runtime::IUnknown {
    fn from(value: IVssComponentEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssComponentEx> for ::windows::runtime::IUnknown {
    fn from(value: &IVssComponentEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssComponentEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssComponentEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IVssComponentEx> for IVssComponent {
    fn from(value: IVssComponentEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssComponentEx> for IVssComponent {
    fn from(value: &IVssComponentEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssComponent> for IVssComponentEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVssComponent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssComponent> for &IVssComponentEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVssComponent>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pct: *mut VSS_COMPONENT_TYPE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbsucceeded: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcmappings: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        imapping: u32,
        ppfiledesc: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszdata: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrdata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilename: super::super::Foundation::PWSTR,
        wszranges: super::super::Foundation::PWSTR,
        wszmetadata: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcpartialfiles: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ipartialfile: u32,
        pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrfilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrrange: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrmetadata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbselectedforrestore: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbadditionalrestores: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcnewtarget: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inewtarget: u32,
        ppfiledesc: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszsourcepath: super::super::Foundation::PWSTR,
        wszsourcefilename: super::super::Foundation::PWSTR,
        wszsourcerangelist: super::super::Foundation::PWSTR,
        wszdestinationpath: super::super::Foundation::PWSTR,
        wszdestinationfilename: super::super::Foundation::PWSTR,
        wszdestinationrangelist: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcdirectedtarget: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idirectedtarget: u32,
        pbstrsourcepath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcefilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcerangelist: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationfilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationrangelist: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszrestoremetadata: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrrestoremetadata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptarget: *mut VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszprerestorefailuremsg: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrprerestorefailuremsg: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpostrestorefailuremsg: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpostrestorefailuremsg: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszbackupstamp: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrbackupstamp: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrbackupstamp: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrbackupoptions: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrrestoreoptions: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcrestoresubcomponent: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        icomponent: u32,
        pbstrlogicalpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrcomponentname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbrepair: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstatus: *mut VSS_FILE_RESTORE_STATUS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: super::super::Foundation::BOOL,
        ftlastmodifytime: super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: super::super::Foundation::BOOL,
        bstrlsnstring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcdifferencedfiles: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idifferencedfile: u32,
        pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrfilespec: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbrecursive: *mut super::super::Foundation::BOOL,
        pbstrlsnstring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pftlastmodifytime: *mut super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszfailuremsg: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszfailuremsg: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrfailuremsg: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrfailuremsg: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbauth: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        prolltype: *mut VSS_ROLLFORWARD_TYPE,
        pbstrpoint: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IVssComponentEx2(::windows::runtime::IUnknown);
impl IVssComponentEx2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(
        &self,
        pbstrpath: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrpath),
        )
        .ok()
    }
    pub unsafe fn GetComponentType(
        &self,
        pct: *mut VSS_COMPONENT_TYPE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pct),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(
        &self,
        pbstrname: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrname),
        )
        .ok()
    }
    pub unsafe fn GetBackupSucceeded(
        &self,
        pbsucceeded: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbsucceeded),
        )
        .ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(
        &self,
        pcmappings: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcmappings),
        )
        .ok()
    }
    pub unsafe fn GetAlternateLocationMapping(
        &self,
        imapping: u32,
    ) -> ::windows::runtime::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(imapping),
            &mut result__,
        )
        .from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupMetadata<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszdata: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            wszdata.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupMetadata(
        &self,
        pbstrdata: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrdata),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPartialFile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszpath: Param0,
        wszfilename: Param1,
        wszranges: Param2,
        wszmetadata: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilename.into_param().abi(),
            wszranges.into_param().abi(),
            wszmetadata.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPartialFileCount(
        &self,
        pcpartialfiles: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcpartialfiles),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartialFile(
        &self,
        ipartialfile: u32,
        pbstrpath: *mut super::super::Foundation::BSTR,
        pbstrfilename: *mut super::super::Foundation::BSTR,
        pbstrrange: *mut super::super::Foundation::BSTR,
        pbstrmetadata: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ipartialfile),
            ::std::mem::transmute(pbstrpath),
            ::std::mem::transmute(pbstrfilename),
            ::std::mem::transmute(pbstrrange),
            ::std::mem::transmute(pbstrmetadata),
        )
        .ok()
    }
    pub unsafe fn IsSelectedForRestore(
        &self,
        pbselectedforrestore: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbselectedforrestore),
        )
        .ok()
    }
    pub unsafe fn GetAdditionalRestores(
        &self,
        pbadditionalrestores: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbadditionalrestores),
        )
        .ok()
    }
    pub unsafe fn GetNewTargetCount(
        &self,
        pcnewtarget: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcnewtarget),
        )
        .ok()
    }
    pub unsafe fn GetNewTarget(
        &self,
        inewtarget: u32,
    ) -> ::windows::runtime::Result<IVssWMFiledesc> {
        let mut result__: <IVssWMFiledesc as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(inewtarget),
            &mut result__,
        )
        .from_abi::<IVssWMFiledesc>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDirectedTarget<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszsourcepath: Param0,
        wszsourcefilename: Param1,
        wszsourcerangelist: Param2,
        wszdestinationpath: Param3,
        wszdestinationfilename: Param4,
        wszdestinationrangelist: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            wszsourcepath.into_param().abi(),
            wszsourcefilename.into_param().abi(),
            wszsourcerangelist.into_param().abi(),
            wszdestinationpath.into_param().abi(),
            wszdestinationfilename.into_param().abi(),
            wszdestinationrangelist.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDirectedTargetCount(
        &self,
        pcdirectedtarget: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcdirectedtarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectedTarget(
        &self,
        idirectedtarget: u32,
        pbstrsourcepath: *mut super::super::Foundation::BSTR,
        pbstrsourcefilename: *mut super::super::Foundation::BSTR,
        pbstrsourcerangelist: *mut super::super::Foundation::BSTR,
        pbstrdestinationpath: *mut super::super::Foundation::BSTR,
        pbstrdestinationfilename: *mut super::super::Foundation::BSTR,
        pbstrdestinationrangelist: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idirectedtarget),
            ::std::mem::transmute(pbstrsourcepath),
            ::std::mem::transmute(pbstrsourcefilename),
            ::std::mem::transmute(pbstrsourcerangelist),
            ::std::mem::transmute(pbstrdestinationpath),
            ::std::mem::transmute(pbstrdestinationfilename),
            ::std::mem::transmute(pbstrdestinationrangelist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestoreMetadata<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszrestoremetadata: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            wszrestoremetadata.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreMetadata(
        &self,
        pbstrrestoremetadata: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrrestoremetadata),
        )
        .ok()
    }
    pub unsafe fn SetRestoreTarget(
        &self,
        target: VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(target),
        )
        .ok()
    }
    pub unsafe fn GetRestoreTarget(
        &self,
        ptarget: *mut VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreRestoreFailureMsg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszprerestorefailuremsg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            wszprerestorefailuremsg.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreRestoreFailureMsg(
        &self,
        pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrprerestorefailuremsg),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostRestoreFailureMsg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszpostrestorefailuremsg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            wszpostrestorefailuremsg.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostRestoreFailureMsg(
        &self,
        pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrpostrestorefailuremsg),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackupStamp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszbackupstamp: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            wszbackupstamp.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupStamp(
        &self,
        pbstrbackupstamp: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrbackupstamp),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreviousBackupStamp(
        &self,
        pbstrbackupstamp: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrbackupstamp),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupOptions(
        &self,
        pbstrbackupoptions: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrbackupoptions),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreOptions(
        &self,
        pbstrrestoreoptions: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrrestoreoptions),
        )
        .ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(
        &self,
        pcrestoresubcomponent: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcrestoresubcomponent),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreSubcomponent(
        &self,
        icomponent: u32,
        pbstrlogicalpath: *mut super::super::Foundation::BSTR,
        pbstrcomponentname: *mut super::super::Foundation::BSTR,
        pbrepair: *mut bool,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(icomponent),
            ::std::mem::transmute(pbstrlogicalpath),
            ::std::mem::transmute(pbstrcomponentname),
            ::std::mem::transmute(pbrepair),
        )
        .ok()
    }
    pub unsafe fn GetFileRestoreStatus(
        &self,
        pstatus: *mut VSS_FILE_RESTORE_STATUS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pstatus),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>,
    >(
        &self,
        wszpath: Param0,
        wszfilespec: Param1,
        brecursive: Param2,
        ftlastmodifytime: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            brecursive.into_param().abi(),
            ftlastmodifytime.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        wszpath: Param0,
        wszfilespec: Param1,
        brecursive: Param2,
        bstrlsnstring: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            brecursive.into_param().abi(),
            bstrlsnstring.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDifferencedFilesCount(
        &self,
        pcdifferencedfiles: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcdifferencedfiles),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(
        &self,
        idifferencedfile: u32,
        pbstrpath: *mut super::super::Foundation::BSTR,
        pbstrfilespec: *mut super::super::Foundation::BSTR,
        pbrecursive: *mut super::super::Foundation::BOOL,
        pbstrlsnstring: *mut super::super::Foundation::BSTR,
        pftlastmodifytime: *mut super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idifferencedfile),
            ::std::mem::transmute(pbstrpath),
            ::std::mem::transmute(pbstrfilespec),
            ::std::mem::transmute(pbrecursive),
            ::std::mem::transmute(pbstrlsnstring),
            ::std::mem::transmute(pftlastmodifytime),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPrepareForBackupFailureMsg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszfailuremsg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            wszfailuremsg.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPostSnapshotFailureMsg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszfailuremsg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            wszfailuremsg.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrepareForBackupFailureMsg(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostSnapshotFailureMsg(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> ::windows::runtime::Result<bool> {
        let mut result__: <bool as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<bool>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRollForward(
        &self,
        prolltype: *mut VSS_ROLLFORWARD_TYPE,
        pbstrpoint: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(prolltype),
            ::std::mem::transmute(pbstrpoint),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreName(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFailure<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hr: ::windows::runtime::HRESULT,
        hrapplication: ::windows::runtime::HRESULT,
        wszapplicationmessage: Param2,
        dwreserved: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hr),
            ::std::mem::transmute(hrapplication),
            wszapplicationmessage.into_param().abi(),
            ::std::mem::transmute(dwreserved),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFailure(
        &self,
        phr: *mut ::windows::runtime::HRESULT,
        phrapplication: *mut ::windows::runtime::HRESULT,
        pbstrapplicationmessage: *mut super::super::Foundation::BSTR,
        pdwreserved: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(phr),
            ::std::mem::transmute(phrapplication),
            ::std::mem::transmute(pbstrapplicationmessage),
            ::std::mem::transmute(pdwreserved),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssComponentEx2 {
    type Vtable = IVssComponentEx2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        995877106,
        1961,
        20043,
        [189, 211, 207, 220, 142, 44, 13, 45],
    );
}
impl ::std::convert::From<IVssComponentEx2> for ::windows::runtime::IUnknown {
    fn from(value: IVssComponentEx2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssComponentEx2> for ::windows::runtime::IUnknown {
    fn from(value: &IVssComponentEx2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssComponentEx2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssComponentEx2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IVssComponentEx2> for IVssComponentEx {
    fn from(value: IVssComponentEx2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssComponentEx2> for IVssComponentEx {
    fn from(value: &IVssComponentEx2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssComponentEx> for IVssComponentEx2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssComponentEx> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVssComponentEx>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssComponentEx> for &IVssComponentEx2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssComponentEx> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVssComponentEx>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IVssComponentEx2> for IVssComponent {
    fn from(value: IVssComponentEx2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssComponentEx2> for IVssComponent {
    fn from(value: &IVssComponentEx2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssComponent> for IVssComponentEx2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVssComponent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssComponent> for &IVssComponentEx2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVssComponent>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pct: *mut VSS_COMPONENT_TYPE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbsucceeded: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcmappings: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        imapping: u32,
        ppfiledesc: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszdata: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrdata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilename: super::super::Foundation::PWSTR,
        wszranges: super::super::Foundation::PWSTR,
        wszmetadata: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcpartialfiles: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ipartialfile: u32,
        pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrfilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrrange: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrmetadata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbselectedforrestore: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbadditionalrestores: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcnewtarget: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inewtarget: u32,
        ppfiledesc: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszsourcepath: super::super::Foundation::PWSTR,
        wszsourcefilename: super::super::Foundation::PWSTR,
        wszsourcerangelist: super::super::Foundation::PWSTR,
        wszdestinationpath: super::super::Foundation::PWSTR,
        wszdestinationfilename: super::super::Foundation::PWSTR,
        wszdestinationrangelist: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcdirectedtarget: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idirectedtarget: u32,
        pbstrsourcepath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcefilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrsourcerangelist: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationfilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdestinationrangelist: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszrestoremetadata: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrrestoremetadata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptarget: *mut VSS_RESTORE_TARGET,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszprerestorefailuremsg: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrprerestorefailuremsg: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpostrestorefailuremsg: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpostrestorefailuremsg: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszbackupstamp: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrbackupstamp: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrbackupstamp: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrbackupoptions: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrrestoreoptions: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcrestoresubcomponent: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        icomponent: u32,
        pbstrlogicalpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrcomponentname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbrepair: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstatus: *mut VSS_FILE_RESTORE_STATUS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: super::super::Foundation::BOOL,
        ftlastmodifytime: super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: super::super::Foundation::BOOL,
        bstrlsnstring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcdifferencedfiles: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idifferencedfile: u32,
        pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrfilespec: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbrecursive: *mut super::super::Foundation::BOOL,
        pbstrlsnstring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pftlastmodifytime: *mut super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszfailuremsg: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszfailuremsg: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrfailuremsg: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrfailuremsg: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbauth: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        prolltype: *mut VSS_ROLLFORWARD_TYPE,
        pbstrpoint: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hr: ::windows::runtime::HRESULT,
        hrapplication: ::windows::runtime::HRESULT,
        wszapplicationmessage: super::super::Foundation::PWSTR,
        dwreserved: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phr: *mut ::windows::runtime::HRESULT,
        phrapplication: *mut ::windows::runtime::HRESULT,
        pbstrapplicationmessage: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pdwreserved: *mut u32,
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
pub struct IVssCreateExpressWriterMetadata(::windows::runtime::IUnknown);
impl IVssCreateExpressWriterMetadata {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddExcludeFiles<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszpath: Param0,
        wszfilespec: Param1,
        brecursive: u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            ::std::mem::transmute(brecursive),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddComponent<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ct),
            wszlogicalpath.into_param().abi(),
            wszcomponentname.into_param().abi(),
            wszcaption.into_param().abi(),
            ::std::mem::transmute(pbicon),
            ::std::mem::transmute(cbicon),
            ::std::mem::transmute(brestoremetadata),
            ::std::mem::transmute(bnotifyonbackupcomplete),
            ::std::mem::transmute(bselectable),
            ::std::mem::transmute(bselectableforrestore),
            ::std::mem::transmute(dwcomponentflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFilesToFileGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszlogicalpath: Param0,
        wszgroupname: Param1,
        wszpath: Param2,
        wszfilespec: Param3,
        brecursive: u8,
        wszalternatelocation: Param5,
        dwbackuptypemask: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            wszlogicalpath.into_param().abi(),
            wszgroupname.into_param().abi(),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            ::std::mem::transmute(brecursive),
            wszalternatelocation.into_param().abi(),
            ::std::mem::transmute(dwbackuptypemask),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestoreMethod<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        method: VSS_RESTOREMETHOD_ENUM,
        wszservice: Param1,
        wszuserprocedure: Param2,
        writerrestore: VSS_WRITERRESTORE_ENUM,
        brebootrequired: u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(method),
            wszservice.into_param().abi(),
            wszuserprocedure.into_param().abi(),
            ::std::mem::transmute(writerrestore),
            ::std::mem::transmute(brebootrequired),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddComponentDependency<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszforlogicalpath: Param0,
        wszforcomponentname: Param1,
        onwriterid: Param2,
        wszonlogicalpath: Param3,
        wszoncomponentname: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            wszforlogicalpath.into_param().abi(),
            wszforcomponentname.into_param().abi(),
            onwriterid.into_param().abi(),
            wszonlogicalpath.into_param().abi(),
            wszoncomponentname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwschemamask),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAsXML(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVssCreateExpressWriterMetadata {
    type Vtable = IVssCreateExpressWriterMetadata_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2625056375,
        45678,
        17023,
        [146, 221, 201, 150, 244, 30, 165, 227],
    );
}
impl ::std::convert::From<IVssCreateExpressWriterMetadata> for ::windows::runtime::IUnknown {
    fn from(value: IVssCreateExpressWriterMetadata) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssCreateExpressWriterMetadata> for ::windows::runtime::IUnknown {
    fn from(value: &IVssCreateExpressWriterMetadata) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssCreateExpressWriterMetadata
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssCreateExpressWriterMetadata
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
pub struct IVssCreateExpressWriterMetadata_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: super::super::Foundation::PWSTR,
        wszcomponentname: super::super::Foundation::PWSTR,
        wszcaption: super::super::Foundation::PWSTR,
        pbicon: *const u8,
        cbicon: u32,
        brestoremetadata: u8,
        bnotifyonbackupcomplete: u8,
        bselectable: u8,
        bselectableforrestore: u8,
        dwcomponentflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszlogicalpath: super::super::Foundation::PWSTR,
        wszgroupname: super::super::Foundation::PWSTR,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: u8,
        wszalternatelocation: super::super::Foundation::PWSTR,
        dwbackuptypemask: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        method: VSS_RESTOREMETHOD_ENUM,
        wszservice: super::super::Foundation::PWSTR,
        wszuserprocedure: super::super::Foundation::PWSTR,
        writerrestore: VSS_WRITERRESTORE_ENUM,
        brebootrequired: u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszforlogicalpath: super::super::Foundation::PWSTR,
        wszforcomponentname: super::super::Foundation::PWSTR,
        onwriterid: ::windows::runtime::GUID,
        wszonlogicalpath: super::super::Foundation::PWSTR,
        wszoncomponentname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwschemamask: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrxml: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IVssCreateWriterMetadata(::windows::runtime::IUnknown);
impl IVssCreateWriterMetadata {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddIncludeFiles<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszpath: Param0,
        wszfilespec: Param1,
        brecursive: u8,
        wszalternatelocation: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            ::std::mem::transmute(brecursive),
            wszalternatelocation.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddExcludeFiles<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszpath: Param0,
        wszfilespec: Param1,
        brecursive: u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            ::std::mem::transmute(brecursive),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddComponent<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ct),
            wszlogicalpath.into_param().abi(),
            wszcomponentname.into_param().abi(),
            wszcaption.into_param().abi(),
            ::std::mem::transmute(pbicon),
            ::std::mem::transmute(cbicon),
            ::std::mem::transmute(brestoremetadata),
            ::std::mem::transmute(bnotifyonbackupcomplete),
            ::std::mem::transmute(bselectable),
            ::std::mem::transmute(bselectableforrestore),
            ::std::mem::transmute(dwcomponentflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDatabaseFiles<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszlogicalpath: Param0,
        wszdatabasename: Param1,
        wszpath: Param2,
        wszfilespec: Param3,
        dwbackuptypemask: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            wszlogicalpath.into_param().abi(),
            wszdatabasename.into_param().abi(),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            ::std::mem::transmute(dwbackuptypemask),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDatabaseLogFiles<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszlogicalpath: Param0,
        wszdatabasename: Param1,
        wszpath: Param2,
        wszfilespec: Param3,
        dwbackuptypemask: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            wszlogicalpath.into_param().abi(),
            wszdatabasename.into_param().abi(),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            ::std::mem::transmute(dwbackuptypemask),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFilesToFileGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszlogicalpath: Param0,
        wszgroupname: Param1,
        wszpath: Param2,
        wszfilespec: Param3,
        brecursive: u8,
        wszalternatelocation: Param5,
        dwbackuptypemask: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            wszlogicalpath.into_param().abi(),
            wszgroupname.into_param().abi(),
            wszpath.into_param().abi(),
            wszfilespec.into_param().abi(),
            ::std::mem::transmute(brecursive),
            wszalternatelocation.into_param().abi(),
            ::std::mem::transmute(dwbackuptypemask),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestoreMethod<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        method: VSS_RESTOREMETHOD_ENUM,
        wszservice: Param1,
        wszuserprocedure: Param2,
        writerrestore: VSS_WRITERRESTORE_ENUM,
        brebootrequired: u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(method),
            wszservice.into_param().abi(),
            wszuserprocedure.into_param().abi(),
            ::std::mem::transmute(writerrestore),
            ::std::mem::transmute(brebootrequired),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddAlternateLocationMapping<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszsourcepath: Param0,
        wszsourcefilespec: Param1,
        brecursive: u8,
        wszdestination: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            wszsourcepath.into_param().abi(),
            wszsourcefilespec.into_param().abi(),
            ::std::mem::transmute(brecursive),
            wszdestination.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddComponentDependency<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszforlogicalpath: Param0,
        wszforcomponentname: Param1,
        onwriterid: Param2,
        wszonlogicalpath: Param3,
        wszoncomponentname: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            wszforlogicalpath.into_param().abi(),
            wszforcomponentname.into_param().abi(),
            onwriterid.into_param().abi(),
            wszonlogicalpath.into_param().abi(),
            wszoncomponentname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwschemamask),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub unsafe fn GetDocument(
        &self,
    ) -> ::windows::runtime::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument> {
        let mut result__ : < super::super::Data::Xml::MsXml:: IXMLDOMDocument as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Data::Xml::MsXml::IXMLDOMDocument>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAsXML(
        &self,
        pbstrxml: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrxml),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssCreateWriterMetadata {
    type Vtable = IVssCreateWriterMetadata_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IVssCreateWriterMetadata> for ::windows::runtime::IUnknown {
    fn from(value: IVssCreateWriterMetadata) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssCreateWriterMetadata> for ::windows::runtime::IUnknown {
    fn from(value: &IVssCreateWriterMetadata) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssCreateWriterMetadata
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssCreateWriterMetadata
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
pub struct IVssCreateWriterMetadata_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: u8,
        wszalternatelocation: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: super::super::Foundation::PWSTR,
        wszcomponentname: super::super::Foundation::PWSTR,
        wszcaption: super::super::Foundation::PWSTR,
        pbicon: *const u8,
        cbicon: u32,
        brestoremetadata: u8,
        bnotifyonbackupcomplete: u8,
        bselectable: u8,
        bselectableforrestore: u8,
        dwcomponentflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszlogicalpath: super::super::Foundation::PWSTR,
        wszdatabasename: super::super::Foundation::PWSTR,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        dwbackuptypemask: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszlogicalpath: super::super::Foundation::PWSTR,
        wszdatabasename: super::super::Foundation::PWSTR,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        dwbackuptypemask: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszlogicalpath: super::super::Foundation::PWSTR,
        wszgroupname: super::super::Foundation::PWSTR,
        wszpath: super::super::Foundation::PWSTR,
        wszfilespec: super::super::Foundation::PWSTR,
        brecursive: u8,
        wszalternatelocation: super::super::Foundation::PWSTR,
        dwbackuptypemask: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        method: VSS_RESTOREMETHOD_ENUM,
        wszservice: super::super::Foundation::PWSTR,
        wszuserprocedure: super::super::Foundation::PWSTR,
        writerrestore: VSS_WRITERRESTORE_ENUM,
        brebootrequired: u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszsourcepath: super::super::Foundation::PWSTR,
        wszsourcefilespec: super::super::Foundation::PWSTR,
        brecursive: u8,
        wszdestination: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszforlogicalpath: super::super::Foundation::PWSTR,
        wszforcomponentname: super::super::Foundation::PWSTR,
        onwriterid: ::windows::runtime::GUID,
        wszonlogicalpath: super::super::Foundation::PWSTR,
        wszoncomponentname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwschemamask: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdoc: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Data_Xml_MsXml"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrxml: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IVssDifferentialSoftwareSnapshotMgmt(::windows::runtime::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt {
    pub unsafe fn AddDiffArea(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            ::std::mem::transmute(llmaximumdiffspace),
        )
        .ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            ::std::mem::transmute(llmaximumdiffspace),
        )
        .ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(
        &self,
        pwszoriginalvolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszoriginalvolumename),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(
        &self,
        pwszvolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(
        &self,
        pwszvolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotid: Param0,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            snapshotid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVssDifferentialSoftwareSnapshotMgmt {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        558501672,
        46903,
        16422,
        [184, 71, 79, 158, 55, 215, 149, 41],
    );
}
impl ::std::convert::From<IVssDifferentialSoftwareSnapshotMgmt> for ::windows::runtime::IUnknown {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssDifferentialSoftwareSnapshotMgmt> for ::windows::runtime::IUnknown {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssDifferentialSoftwareSnapshotMgmt
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssDifferentialSoftwareSnapshotMgmt
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
pub struct IVssDifferentialSoftwareSnapshotMgmt_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszoriginalvolumename: *const u16,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotid: ::windows::runtime::GUID,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2(::windows::runtime::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt2 {
    pub unsafe fn AddDiffArea(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            ::std::mem::transmute(llmaximumdiffspace),
        )
        .ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            ::std::mem::transmute(llmaximumdiffspace),
        )
        .ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(
        &self,
        pwszoriginalvolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszoriginalvolumename),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(
        &self,
        pwszvolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(
        &self,
        pwszvolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotid: Param0,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            snapshotid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDiffAreaMaximumSizeEx<
        'a,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
        bvolatile: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            ::std::mem::transmute(llmaximumdiffspace),
            bvolatile.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn MigrateDiffAreas(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        pwsznewdiffareavolumename: *const u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            ::std::mem::transmute(pwsznewdiffareavolumename),
        )
        .ok()
    }
    pub unsafe fn QueryMigrationStatus(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssAsync> {
        let mut result__: <IVssAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            &mut result__,
        )
        .from_abi::<IVssAsync>(result__)
    }
    pub unsafe fn SetSnapshotPriority<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        idsnapshot: Param0,
        priority: u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            idsnapshot.into_param().abi(),
            ::std::mem::transmute(priority),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssDifferentialSoftwareSnapshotMgmt2 {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2493346643,
        26463,
        17013,
        [137, 105, 240, 68, 198, 39, 120, 21],
    );
}
impl ::std::convert::From<IVssDifferentialSoftwareSnapshotMgmt2> for ::windows::runtime::IUnknown {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssDifferentialSoftwareSnapshotMgmt2> for ::windows::runtime::IUnknown {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssDifferentialSoftwareSnapshotMgmt2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssDifferentialSoftwareSnapshotMgmt2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IVssDifferentialSoftwareSnapshotMgmt2>
    for IVssDifferentialSoftwareSnapshotMgmt
{
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssDifferentialSoftwareSnapshotMgmt2>
    for IVssDifferentialSoftwareSnapshotMgmt
{
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt>
    for IVssDifferentialSoftwareSnapshotMgmt2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssDifferentialSoftwareSnapshotMgmt> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<IVssDifferentialSoftwareSnapshotMgmt>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt>
    for &IVssDifferentialSoftwareSnapshotMgmt2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssDifferentialSoftwareSnapshotMgmt> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<IVssDifferentialSoftwareSnapshotMgmt>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszoriginalvolumename: *const u16,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotid: ::windows::runtime::GUID,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
        bvolatile: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        pwsznewdiffareavolumename: *const u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        ppasync: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idsnapshot: ::windows::runtime::GUID,
        priority: u8,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3(::windows::runtime::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt3 {
    pub unsafe fn AddDiffArea(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            ::std::mem::transmute(llmaximumdiffspace),
        )
        .ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            ::std::mem::transmute(llmaximumdiffspace),
        )
        .ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(
        &self,
        pwszoriginalvolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszoriginalvolumename),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(
        &self,
        pwszvolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(
        &self,
        pwszvolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotid: Param0,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            snapshotid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDiffAreaMaximumSizeEx<
        'a,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
        bvolatile: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            ::std::mem::transmute(llmaximumdiffspace),
            bvolatile.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn MigrateDiffAreas(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        pwsznewdiffareavolumename: *const u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            ::std::mem::transmute(pwsznewdiffareavolumename),
        )
        .ok()
    }
    pub unsafe fn QueryMigrationStatus(
        &self,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
    ) -> ::windows::runtime::Result<IVssAsync> {
        let mut result__: <IVssAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pwszdiffareavolumename),
            &mut result__,
        )
        .from_abi::<IVssAsync>(result__)
    }
    pub unsafe fn SetSnapshotPriority<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        idsnapshot: Param0,
        priority: u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            idsnapshot.into_param().abi(),
            ::std::mem::transmute(priority),
        )
        .ok()
    }
    pub unsafe fn SetVolumeProtectLevel(
        &self,
        pwszvolumename: *const u16,
        protectionlevel: VSS_PROTECTION_LEVEL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(protectionlevel),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVolumeProtectLevel(
        &self,
        pwszvolumename: *const u16,
    ) -> ::windows::runtime::Result<VSS_VOLUME_PROTECTION_INFO> {
        let mut result__: <VSS_VOLUME_PROTECTION_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            &mut result__,
        )
        .from_abi::<VSS_VOLUME_PROTECTION_INFO>(result__)
    }
    pub unsafe fn ClearVolumeProtectFault(
        &self,
        pwszvolumename: *const u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
        )
        .ok()
    }
    pub unsafe fn DeleteUnusedDiffAreas(
        &self,
        pwszdiffareavolumename: *const u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszdiffareavolumename),
        )
        .ok()
    }
    pub unsafe fn QuerySnapshotDeltaBitmap<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        idsnapshotolder: Param0,
        idsnapshotyounger: Param1,
        pcblocksizeperbit: *mut u32,
        pcbitmaplength: *mut u32,
        ppbbitmap: *mut *mut u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            idsnapshotolder.into_param().abi(),
            idsnapshotyounger.into_param().abi(),
            ::std::mem::transmute(pcblocksizeperbit),
            ::std::mem::transmute(pcbitmaplength),
            ::std::mem::transmute(ppbbitmap),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssDifferentialSoftwareSnapshotMgmt3 {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        943685233,
        42181,
        16415,
        [178, 127, 248, 38, 40, 159, 132, 88],
    );
}
impl ::std::convert::From<IVssDifferentialSoftwareSnapshotMgmt3> for ::windows::runtime::IUnknown {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssDifferentialSoftwareSnapshotMgmt3> for ::windows::runtime::IUnknown {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssDifferentialSoftwareSnapshotMgmt3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssDifferentialSoftwareSnapshotMgmt3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IVssDifferentialSoftwareSnapshotMgmt3>
    for IVssDifferentialSoftwareSnapshotMgmt2
{
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssDifferentialSoftwareSnapshotMgmt3>
    for IVssDifferentialSoftwareSnapshotMgmt2
{
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt2>
    for IVssDifferentialSoftwareSnapshotMgmt3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssDifferentialSoftwareSnapshotMgmt2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            IVssDifferentialSoftwareSnapshotMgmt2,
        >::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt2>
    for &IVssDifferentialSoftwareSnapshotMgmt3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssDifferentialSoftwareSnapshotMgmt2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            IVssDifferentialSoftwareSnapshotMgmt2,
        >::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IVssDifferentialSoftwareSnapshotMgmt3>
    for IVssDifferentialSoftwareSnapshotMgmt
{
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssDifferentialSoftwareSnapshotMgmt3>
    for IVssDifferentialSoftwareSnapshotMgmt
{
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt>
    for IVssDifferentialSoftwareSnapshotMgmt3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssDifferentialSoftwareSnapshotMgmt> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<IVssDifferentialSoftwareSnapshotMgmt>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssDifferentialSoftwareSnapshotMgmt>
    for &IVssDifferentialSoftwareSnapshotMgmt3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssDifferentialSoftwareSnapshotMgmt> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<IVssDifferentialSoftwareSnapshotMgmt>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszoriginalvolumename: *const u16,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotid: ::windows::runtime::GUID,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        llmaximumdiffspace: i64,
        bvolatile: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        pwsznewdiffareavolumename: *const u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pwszdiffareavolumename: *const u16,
        ppasync: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idsnapshot: ::windows::runtime::GUID,
        priority: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        protectionlevel: VSS_PROTECTION_LEVEL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszdiffareavolumename: *const u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idsnapshotolder: ::windows::runtime::GUID,
        idsnapshotyounger: ::windows::runtime::GUID,
        pcblocksizeperbit: *mut u32,
        pcbitmaplength: *mut u32,
        ppbbitmap: *mut *mut u8,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssEnumMgmtObject(::windows::runtime::IUnknown);
impl IVssEnumMgmtObject {
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut VSS_MGMT_OBJECT_PROP,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(rgelt),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(
        &self,
        ppenum: *mut ::std::option::Option<IVssEnumMgmtObject>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppenum),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssEnumMgmtObject {
    type Vtable = IVssEnumMgmtObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        26562155,
        37460,
        20078,
        [128, 140, 201, 224, 93, 0, 118, 150],
    );
}
impl ::std::convert::From<IVssEnumMgmtObject> for ::windows::runtime::IUnknown {
    fn from(value: IVssEnumMgmtObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssEnumMgmtObject> for ::windows::runtime::IUnknown {
    fn from(value: &IVssEnumMgmtObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssEnumMgmtObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssEnumMgmtObject {
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
pub struct IVssEnumMgmtObject_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        rgelt: *mut VSS_MGMT_OBJECT_PROP,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssEnumObject(::windows::runtime::IUnknown);
impl IVssEnumObject {
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut VSS_OBJECT_PROP,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(rgelt),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(
        &self,
        ppenum: *mut ::std::option::Option<IVssEnumObject>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppenum),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssEnumObject {
    type Vtable = IVssEnumObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2921099536,
        12128,
        4563,
        [138, 57, 0, 192, 79, 114, 216, 227],
    );
}
impl ::std::convert::From<IVssEnumObject> for ::windows::runtime::IUnknown {
    fn from(value: IVssEnumObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssEnumObject> for ::windows::runtime::IUnknown {
    fn from(value: &IVssEnumObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssEnumObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssEnumObject {
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
pub struct IVssEnumObject_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        rgelt: *mut VSS_OBJECT_PROP,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct IVssExamineWriterMetadata(pub u8);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssExpressWriter(::windows::runtime::IUnknown);
impl IVssExpressWriter {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateMetadata<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        writerid: Param0,
        writername: Param1,
        usagetype: VSS_USAGE_TYPE,
        versionmajor: u32,
        versionminor: u32,
        reserved: u32,
    ) -> ::windows::runtime::Result<IVssCreateExpressWriterMetadata> {
        let mut result__: <IVssCreateExpressWriterMetadata as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            writerid.into_param().abi(),
            writername.into_param().abi(),
            ::std::mem::transmute(usagetype),
            ::std::mem::transmute(versionmajor),
            ::std::mem::transmute(versionminor),
            ::std::mem::transmute(reserved),
            &mut result__,
        )
        .from_abi::<IVssCreateExpressWriterMetadata>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadMetadata<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        metadata: Param0,
        reserved: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            metadata.into_param().abi(),
            ::std::mem::transmute(reserved),
        )
        .ok()
    }
    pub unsafe fn Register(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unregister<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        writerid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            writerid.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssExpressWriter {
    type Vtable = IVssExpressWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3812294620,
        22983,
        18353,
        [151, 213, 66, 102, 89, 143, 98, 53],
    );
}
impl ::std::convert::From<IVssExpressWriter> for ::windows::runtime::IUnknown {
    fn from(value: IVssExpressWriter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssExpressWriter> for ::windows::runtime::IUnknown {
    fn from(value: &IVssExpressWriter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssExpressWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssExpressWriter {
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
pub struct IVssExpressWriter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        writerid: ::windows::runtime::GUID,
        writername: super::super::Foundation::PWSTR,
        usagetype: VSS_USAGE_TYPE,
        versionmajor: u32,
        versionminor: u32,
        reserved: u32,
        ppmetadata: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        metadata: super::super::Foundation::PWSTR,
        reserved: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        writerid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssFileShareSnapshotProvider(::windows::runtime::IUnknown);
impl IVssFileShareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lcontext),
        )
        .ok()
    }
    pub unsafe fn GetSnapshotProperties<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotid: Param0,
    ) -> ::windows::runtime::Result<VSS_SNAPSHOT_PROP> {
        let mut result__: <VSS_SNAPSHOT_PROP as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            snapshotid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<VSS_SNAPSHOT_PROP>(result__)
    }
    pub unsafe fn Query<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(
        &self,
        queriedobjectid: Param0,
        equeriedobjecttype: VSS_OBJECT_TYPE,
        ereturnedobjectstype: VSS_OBJECT_TYPE,
    ) -> ::windows::runtime::Result<IVssEnumObject> {
        let mut result__: <IVssEnumObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            queriedobjectid.into_param().abi(),
            ::std::mem::transmute(equeriedobjecttype),
            ::std::mem::transmute(ereturnedobjectstype),
            &mut result__,
        )
        .from_abi::<IVssEnumObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSnapshots<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        sourceobjectid: Param0,
        esourceobjecttype: VSS_OBJECT_TYPE,
        bforcedelete: Param2,
        pldeletedsnapshots: *mut i32,
        pnondeletedsnapshotid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            sourceobjectid.into_param().abi(),
            ::std::mem::transmute(esourceobjecttype),
            bforcedelete.into_param().abi(),
            ::std::mem::transmute(pldeletedsnapshots),
            ::std::mem::transmute(pnondeletedsnapshotid),
        )
        .ok()
    }
    pub unsafe fn BeginPrepareSnapshot<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
        snapshotid: Param1,
        pwszsharepath: *const u16,
        lnewcontext: i32,
        providerid: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
            snapshotid.into_param().abi(),
            ::std::mem::transmute(pwszsharepath),
            ::std::mem::transmute(lnewcontext),
            providerid.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathSupported(
        &self,
        pwszsharepath: *const u16,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszsharepath),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathSnapshotted(
        &self,
        pwszsharepath: *const u16,
        pbsnapshotspresent: *mut super::super::Foundation::BOOL,
        plsnapshotcompatibility: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszsharepath),
            ::std::mem::transmute(pbsnapshotspresent),
            ::std::mem::transmute(plsnapshotcompatibility),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn SetSnapshotProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::System::OleAutomation::VARIANT>,
    >(
        &self,
        snapshotid: Param0,
        esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID,
        vproperty: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            snapshotid.into_param().abi(),
            ::std::mem::transmute(esnapshotpropertyid),
            vproperty.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssFileShareSnapshotProvider {
    type Vtable = IVssFileShareSnapshotProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3361955936,
        31790,
        4575,
        [140, 74, 8, 0, 32, 12, 154, 102],
    );
}
impl ::std::convert::From<IVssFileShareSnapshotProvider> for ::windows::runtime::IUnknown {
    fn from(value: IVssFileShareSnapshotProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssFileShareSnapshotProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IVssFileShareSnapshotProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssFileShareSnapshotProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssFileShareSnapshotProvider
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
pub struct IVssFileShareSnapshotProvider_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lcontext: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotid: ::windows::runtime::GUID,
        pprop: *mut VSS_SNAPSHOT_PROP,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        queriedobjectid: ::windows::runtime::GUID,
        equeriedobjecttype: VSS_OBJECT_TYPE,
        ereturnedobjectstype: VSS_OBJECT_TYPE,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceobjectid: ::windows::runtime::GUID,
        esourceobjecttype: VSS_OBJECT_TYPE,
        bforcedelete: super::super::Foundation::BOOL,
        pldeletedsnapshots: *mut i32,
        pnondeletedsnapshotid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
        snapshotid: ::windows::runtime::GUID,
        pwszsharepath: *const u16,
        lnewcontext: i32,
        providerid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszsharepath: *const u16,
        pbsupportedbythisprovider: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszsharepath: *const u16,
        pbsnapshotspresent: *mut super::super::Foundation::BOOL,
        plsnapshotcompatibility: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotid: ::windows::runtime::GUID,
        esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID,
        vproperty: ::std::mem::ManuallyDrop<super::super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
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
pub struct IVssHardwareSnapshotProvider(::windows::runtime::IUnknown);
impl IVssHardwareSnapshotProvider {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn AreLunsSupported(
        &self,
        lluncount: i32,
        lcontext: i32,
        rgwszdevices: *const *const u16,
        pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
        pbissupported: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lluncount),
            ::std::mem::transmute(lcontext),
            ::std::mem::transmute(rgwszdevices),
            ::std::mem::transmute(pluninformation),
            ::std::mem::transmute(pbissupported),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn FillInLunInfo(
        &self,
        wszdevicename: *const u16,
        pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
        pbissupported: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wszdevicename),
            ::std::mem::transmute(pluninfo),
            ::std::mem::transmute(pbissupported),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn BeginPrepareSnapshot<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
        snapshotid: Param1,
        lcontext: i32,
        lluncount: i32,
        rgdevicenames: *const *const u16,
        rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
            snapshotid.into_param().abi(),
            ::std::mem::transmute(lcontext),
            ::std::mem::transmute(lluncount),
            ::std::mem::transmute(rgdevicenames),
            ::std::mem::transmute(rgluninformation),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn GetTargetLuns(
        &self,
        lluncount: i32,
        rgdevicenames: *const *const u16,
        rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lluncount),
            ::std::mem::transmute(rgdevicenames),
            ::std::mem::transmute(rgsourceluns),
            ::std::mem::transmute(rgdestinationluns),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn LocateLuns(
        &self,
        lluncount: i32,
        rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lluncount),
            ::std::mem::transmute(rgsourceluns),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn OnLunEmpty(
        &self,
        wszdevicename: *const u16,
        pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wszdevicename),
            ::std::mem::transmute(pinformation),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssHardwareSnapshotProvider {
    type Vtable = IVssHardwareSnapshotProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2509480279,
        17641,
        17220,
        [187, 235, 68, 251, 249, 176, 107, 16],
    );
}
impl ::std::convert::From<IVssHardwareSnapshotProvider> for ::windows::runtime::IUnknown {
    fn from(value: IVssHardwareSnapshotProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssHardwareSnapshotProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IVssHardwareSnapshotProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssHardwareSnapshotProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssHardwareSnapshotProvider
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
pub struct IVssHardwareSnapshotProvider_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lluncount: i32,
        lcontext: i32,
        rgwszdevices: *const *const u16,
        pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
        pbissupported: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszdevicename: *const u16,
        pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
        pbissupported: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
        snapshotid: ::windows::runtime::GUID,
        lcontext: i32,
        lluncount: i32,
        rgdevicenames: *const *const u16,
        rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lluncount: i32,
        rgdevicenames: *const *const u16,
        rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lluncount: i32,
        rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszdevicename: *const u16,
        pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
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
pub struct IVssHardwareSnapshotProviderEx(::windows::runtime::IUnknown);
impl IVssHardwareSnapshotProviderEx {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn AreLunsSupported(
        &self,
        lluncount: i32,
        lcontext: i32,
        rgwszdevices: *const *const u16,
        pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
        pbissupported: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lluncount),
            ::std::mem::transmute(lcontext),
            ::std::mem::transmute(rgwszdevices),
            ::std::mem::transmute(pluninformation),
            ::std::mem::transmute(pbissupported),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn FillInLunInfo(
        &self,
        wszdevicename: *const u16,
        pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
        pbissupported: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wszdevicename),
            ::std::mem::transmute(pluninfo),
            ::std::mem::transmute(pbissupported),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn BeginPrepareSnapshot<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
        snapshotid: Param1,
        lcontext: i32,
        lluncount: i32,
        rgdevicenames: *const *const u16,
        rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
            snapshotid.into_param().abi(),
            ::std::mem::transmute(lcontext),
            ::std::mem::transmute(lluncount),
            ::std::mem::transmute(rgdevicenames),
            ::std::mem::transmute(rgluninformation),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn GetTargetLuns(
        &self,
        lluncount: i32,
        rgdevicenames: *const *const u16,
        rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lluncount),
            ::std::mem::transmute(rgdevicenames),
            ::std::mem::transmute(rgsourceluns),
            ::std::mem::transmute(rgdestinationluns),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn LocateLuns(
        &self,
        lluncount: i32,
        rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lluncount),
            ::std::mem::transmute(rgsourceluns),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn OnLunEmpty(
        &self,
        wszdevicename: *const u16,
        pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wszdevicename),
            ::std::mem::transmute(pinformation),
        )
        .ok()
    }
    pub unsafe fn GetProviderCapabilities(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u64>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn OnLunStateChange(
        &self,
        psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        dwcount: u32,
        dwflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(psnapshotluns),
            ::std::mem::transmute(poriginalluns),
            ::std::mem::transmute(dwcount),
            ::std::mem::transmute(dwflags),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn ResyncLuns(
        &self,
        psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        dwcount: u32,
    ) -> ::windows::runtime::Result<IVssAsync> {
        let mut result__: <IVssAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(psourceluns),
            ::std::mem::transmute(ptargetluns),
            ::std::mem::transmute(dwcount),
            &mut result__,
        )
        .from_abi::<IVssAsync>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub unsafe fn OnReuseLuns(
        &self,
        psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        dwcount: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(psnapshotluns),
            ::std::mem::transmute(poriginalluns),
            ::std::mem::transmute(dwcount),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssHardwareSnapshotProviderEx {
    type Vtable = IVssHardwareSnapshotProviderEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2136713509,
        52657,
        19729,
        [167, 31, 51, 158, 183, 231, 9, 253],
    );
}
impl ::std::convert::From<IVssHardwareSnapshotProviderEx> for ::windows::runtime::IUnknown {
    fn from(value: IVssHardwareSnapshotProviderEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssHardwareSnapshotProviderEx> for ::windows::runtime::IUnknown {
    fn from(value: &IVssHardwareSnapshotProviderEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssHardwareSnapshotProviderEx
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssHardwareSnapshotProviderEx
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IVssHardwareSnapshotProviderEx> for IVssHardwareSnapshotProvider {
    fn from(value: IVssHardwareSnapshotProviderEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssHardwareSnapshotProviderEx> for IVssHardwareSnapshotProvider {
    fn from(value: &IVssHardwareSnapshotProviderEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssHardwareSnapshotProvider>
    for IVssHardwareSnapshotProviderEx
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssHardwareSnapshotProvider> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<IVssHardwareSnapshotProvider>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVssHardwareSnapshotProvider>
    for &IVssHardwareSnapshotProviderEx
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IVssHardwareSnapshotProvider> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<IVssHardwareSnapshotProvider>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssHardwareSnapshotProviderEx_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lluncount: i32,
        lcontext: i32,
        rgwszdevices: *const *const u16,
        pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
        pbissupported: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszdevicename: *const u16,
        pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
        pbissupported: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
        snapshotid: ::windows::runtime::GUID,
        lcontext: i32,
        lluncount: i32,
        rgdevicenames: *const *const u16,
        rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lluncount: i32,
        rgdevicenames: *const *const u16,
        rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lluncount: i32,
        rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszdevicename: *const u16,
        pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plloriginalcapabilitymask: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        dwcount: u32,
        dwflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        dwcount: u32,
        ppasync: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION,
        dwcount: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Storage_VirtualDiskService"
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
pub struct IVssProviderCreateSnapshotSet(::windows::runtime::IUnknown);
impl IVssProviderCreateSnapshotSet {
    pub unsafe fn EndPrepareSnapshots<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn PreCommitSnapshots<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CommitSnapshots<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn PostCommitSnapshots<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
        lsnapshotscount: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
            ::std::mem::transmute(lsnapshotscount),
        )
        .ok()
    }
    pub unsafe fn PreFinalCommitSnapshots<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn PostFinalCommitSnapshots<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AbortSnapshots<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssProviderCreateSnapshotSet {
    type Vtable = IVssProviderCreateSnapshotSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1602834011,
        7737,
        18296,
        [142, 35, 154, 186, 217, 240, 224, 140],
    );
}
impl ::std::convert::From<IVssProviderCreateSnapshotSet> for ::windows::runtime::IUnknown {
    fn from(value: IVssProviderCreateSnapshotSet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssProviderCreateSnapshotSet> for ::windows::runtime::IUnknown {
    fn from(value: &IVssProviderCreateSnapshotSet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssProviderCreateSnapshotSet
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssProviderCreateSnapshotSet
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
pub struct IVssProviderCreateSnapshotSet_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
        lsnapshotscount: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssProviderNotifications(::windows::runtime::IUnknown);
impl IVssProviderNotifications {
    pub unsafe fn OnLoad<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pcallback: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pcallback.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUnload<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bforceunload: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            bforceunload.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssProviderNotifications {
    type Vtable = IVssProviderNotifications_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3848376351,
        933,
        19198,
        [134, 208, 114, 186, 238, 206, 112, 4],
    );
}
impl ::std::convert::From<IVssProviderNotifications> for ::windows::runtime::IUnknown {
    fn from(value: IVssProviderNotifications) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssProviderNotifications> for ::windows::runtime::IUnknown {
    fn from(value: &IVssProviderNotifications) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssProviderNotifications
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssProviderNotifications
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
pub struct IVssProviderNotifications_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bforceunload: super::super::Foundation::BOOL,
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
pub struct IVssSnapshotMgmt(::windows::runtime::IUnknown);
impl IVssSnapshotMgmt {
    pub unsafe fn GetProviderMgmtInterface<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        providerid: Param0,
        interfaceid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            providerid.into_param().abi(),
            ::std::mem::transmute(interfaceid),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn QueryVolumesSupportedForSnapshots<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        providerid: Param0,
        lcontext: i32,
    ) -> ::windows::runtime::Result<IVssEnumMgmtObject> {
        let mut result__: <IVssEnumMgmtObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            providerid.into_param().abi(),
            ::std::mem::transmute(lcontext),
            &mut result__,
        )
        .from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QuerySnapshotsByVolume<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        pwszvolumename: *const u16,
        providerid: Param1,
    ) -> ::windows::runtime::Result<IVssEnumObject> {
        let mut result__: <IVssEnumObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            providerid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IVssEnumObject>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVssSnapshotMgmt {
    type Vtable = IVssSnapshotMgmt_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4202559305,
        26343,
        18822,
        [162, 127, 226, 240, 74, 229, 55, 114],
    );
}
impl ::std::convert::From<IVssSnapshotMgmt> for ::windows::runtime::IUnknown {
    fn from(value: IVssSnapshotMgmt) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssSnapshotMgmt> for ::windows::runtime::IUnknown {
    fn from(value: &IVssSnapshotMgmt) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssSnapshotMgmt {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssSnapshotMgmt {
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
pub struct IVssSnapshotMgmt_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        providerid: ::windows::runtime::GUID,
        interfaceid: *const ::windows::runtime::GUID,
        ppitf: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        providerid: ::windows::runtime::GUID,
        lcontext: i32,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        providerid: ::windows::runtime::GUID,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssSnapshotMgmt2(::windows::runtime::IUnknown);
impl IVssSnapshotMgmt2 {
    pub unsafe fn GetMinDiffAreaSize(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVssSnapshotMgmt2 {
    type Vtable = IVssSnapshotMgmt2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        258075705,
        65154,
        17906,
        [163, 240, 118, 139, 93, 66, 113, 2],
    );
}
impl ::std::convert::From<IVssSnapshotMgmt2> for ::windows::runtime::IUnknown {
    fn from(value: IVssSnapshotMgmt2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssSnapshotMgmt2> for ::windows::runtime::IUnknown {
    fn from(value: &IVssSnapshotMgmt2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssSnapshotMgmt2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssSnapshotMgmt2 {
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
pub struct IVssSnapshotMgmt2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pllmindiffareasize: *mut i64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssSoftwareSnapshotProvider(::windows::runtime::IUnknown);
impl IVssSoftwareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lcontext),
        )
        .ok()
    }
    pub unsafe fn GetSnapshotProperties<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotid: Param0,
    ) -> ::windows::runtime::Result<VSS_SNAPSHOT_PROP> {
        let mut result__: <VSS_SNAPSHOT_PROP as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            snapshotid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<VSS_SNAPSHOT_PROP>(result__)
    }
    pub unsafe fn Query<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(
        &self,
        queriedobjectid: Param0,
        equeriedobjecttype: VSS_OBJECT_TYPE,
        ereturnedobjectstype: VSS_OBJECT_TYPE,
    ) -> ::windows::runtime::Result<IVssEnumObject> {
        let mut result__: <IVssEnumObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            queriedobjectid.into_param().abi(),
            ::std::mem::transmute(equeriedobjecttype),
            ::std::mem::transmute(ereturnedobjectstype),
            &mut result__,
        )
        .from_abi::<IVssEnumObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSnapshots<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        sourceobjectid: Param0,
        esourceobjecttype: VSS_OBJECT_TYPE,
        bforcedelete: Param2,
        pldeletedsnapshots: *mut i32,
        pnondeletedsnapshotid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            sourceobjectid.into_param().abi(),
            ::std::mem::transmute(esourceobjecttype),
            bforcedelete.into_param().abi(),
            ::std::mem::transmute(pldeletedsnapshots),
            ::std::mem::transmute(pnondeletedsnapshotid),
        )
        .ok()
    }
    pub unsafe fn BeginPrepareSnapshot<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotsetid: Param0,
        snapshotid: Param1,
        pwszvolumename: *const u16,
        lnewcontext: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            snapshotsetid.into_param().abi(),
            snapshotid.into_param().abi(),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(lnewcontext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVolumeSupported(
        &self,
        pwszvolumename: *const u16,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVolumeSnapshotted(
        &self,
        pwszvolumename: *const u16,
        pbsnapshotspresent: *mut super::super::Foundation::BOOL,
        plsnapshotcompatibility: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolumename),
            ::std::mem::transmute(pbsnapshotspresent),
            ::std::mem::transmute(plsnapshotcompatibility),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn SetSnapshotProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::System::OleAutomation::VARIANT>,
    >(
        &self,
        snapshotid: Param0,
        esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID,
        vproperty: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            snapshotid.into_param().abi(),
            ::std::mem::transmute(esnapshotpropertyid),
            vproperty.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RevertToSnapshot<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        snapshotid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            snapshotid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn QueryRevertStatus(
        &self,
        pwszvolume: *const u16,
    ) -> ::windows::runtime::Result<IVssAsync> {
        let mut result__: <IVssAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwszvolume),
            &mut result__,
        )
        .from_abi::<IVssAsync>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVssSoftwareSnapshotProvider {
    type Vtable = IVssSoftwareSnapshotProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1620972094,
        11354,
        17619,
        [143, 1, 11, 29, 154, 71, 209, 255],
    );
}
impl ::std::convert::From<IVssSoftwareSnapshotProvider> for ::windows::runtime::IUnknown {
    fn from(value: IVssSoftwareSnapshotProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssSoftwareSnapshotProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IVssSoftwareSnapshotProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVssSoftwareSnapshotProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVssSoftwareSnapshotProvider
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
pub struct IVssSoftwareSnapshotProvider_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lcontext: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotid: ::windows::runtime::GUID,
        pprop: *mut VSS_SNAPSHOT_PROP,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        queriedobjectid: ::windows::runtime::GUID,
        equeriedobjecttype: VSS_OBJECT_TYPE,
        ereturnedobjectstype: VSS_OBJECT_TYPE,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceobjectid: ::windows::runtime::GUID,
        esourceobjecttype: VSS_OBJECT_TYPE,
        bforcedelete: super::super::Foundation::BOOL,
        pldeletedsnapshots: *mut i32,
        pnondeletedsnapshotid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotsetid: ::windows::runtime::GUID,
        snapshotid: ::windows::runtime::GUID,
        pwszvolumename: *const u16,
        lnewcontext: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pbsupportedbythisprovider: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolumename: *const u16,
        pbsnapshotspresent: *mut super::super::Foundation::BOOL,
        plsnapshotcompatibility: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotid: ::windows::runtime::GUID,
        esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID,
        vproperty: ::std::mem::ManuallyDrop<super::super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapshotid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwszvolume: *const u16,
        ppasync: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssWMDependency(::windows::runtime::IUnknown);
impl IVssWMDependency {
    pub unsafe fn GetWriterId(
        &self,
        pwriterid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwriterid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(
        &self,
        pbstrlogicalpath: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrlogicalpath),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(
        &self,
        pbstrcomponentname: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrcomponentname),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVssWMDependency {
    type Vtable = IVssWMDependency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IVssWMDependency> for ::windows::runtime::IUnknown {
    fn from(value: IVssWMDependency) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssWMDependency> for ::windows::runtime::IUnknown {
    fn from(value: &IVssWMDependency) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssWMDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssWMDependency {
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
pub struct IVssWMDependency_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwriterid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrlogicalpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrcomponentname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IVssWMFiledesc(::windows::runtime::IUnknown);
impl IVssWMFiledesc {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilespec(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetRecursive(&self) -> ::windows::runtime::Result<bool> {
        let mut result__: <bool as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<bool>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAlternateLocation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetBackupTypeMask(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVssWMFiledesc {
    type Vtable = IVssWMFiledesc_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IVssWMFiledesc> for ::windows::runtime::IUnknown {
    fn from(value: IVssWMFiledesc) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssWMFiledesc> for ::windows::runtime::IUnknown {
    fn from(value: &IVssWMFiledesc) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssWMFiledesc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssWMFiledesc {
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
pub struct IVssWMFiledesc_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrfilespec: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbrecursive: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstralternatelocation: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwtypemask: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssWriterComponents(::windows::runtime::IUnknown);
impl IVssWriterComponents {
    pub unsafe fn GetComponentCount(
        &self,
        pccomponents: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pccomponents),
        )
        .ok()
    }
    pub unsafe fn GetWriterInfo(
        &self,
        pidinstance: *mut ::windows::runtime::GUID,
        pidwriter: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pidinstance),
            ::std::mem::transmute(pidwriter),
        )
        .ok()
    }
    pub unsafe fn GetComponent(
        &self,
        icomponent: u32,
    ) -> ::windows::runtime::Result<IVssComponent> {
        let mut result__: <IVssComponent as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(icomponent),
            &mut result__,
        )
        .from_abi::<IVssComponent>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVssWriterComponents {
    type Vtable = IVssWriterComponents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IVssWriterComponents> for ::windows::runtime::IUnknown {
    fn from(value: IVssWriterComponents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssWriterComponents> for ::windows::runtime::IUnknown {
    fn from(value: &IVssWriterComponents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssWriterComponents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssWriterComponents {
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
pub struct IVssWriterComponents_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pccomponents: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pidinstance: *mut ::windows::runtime::GUID,
        pidwriter: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        icomponent: u32,
        ppcomponent: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVssWriterImpl(::windows::runtime::IUnknown);
impl IVssWriterImpl {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            writerid.into_param().abi(),
            wszwritername.into_param().abi(),
            wszwriterinstancename.into_param().abi(),
            ::std::mem::transmute(dwmajorversion),
            ::std::mem::transmute(dwminorversion),
            ::std::mem::transmute(ut),
            ::std::mem::transmute(st),
            ::std::mem::transmute(nlevel),
            ::std::mem::transmute(dwtimeout),
            ::std::mem::transmute(aws),
            ::std::mem::transmute(biothrottlingonly),
        )
        .ok()
    }
    pub unsafe fn Subscribe(
        &self,
        dwsubscribetimeout: u32,
        dweventflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwsubscribetimeout),
            ::std::mem::transmute(dweventflags),
        )
        .ok()
    }
    pub unsafe fn Unsubscribe(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Uninitialize(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrentVolumeArray(&self) -> *mut super::super::Foundation::PWSTR {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetCurrentVolumeCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapshotDeviceName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszoriginalvolume: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            wszoriginalvolume.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetCurrentSnapshotSetId(&self) -> ::windows::runtime::GUID {
        let mut result__: ::windows::runtime::GUID = ::std::default::Default::default();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        );
        result__
    }
    pub unsafe fn GetContext(&self) -> i32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetCurrentLevel(&self) -> VSS_APPLICATION_LEVEL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathAffected<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wszpath: Param0,
    ) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            wszpath.into_param().abi(),
        ))
    }
    pub unsafe fn IsBootableSystemStateBackedUp(&self) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn AreComponentsSelected(&self) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetBackupType(&self) -> VSS_BACKUP_TYPE {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetRestoreType(&self) -> VSS_RESTORE_TYPE {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn SetWriterFailure(
        &self,
        hr: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hr),
        )
        .ok()
    }
    pub unsafe fn IsPartialFileSupportEnabled(&self) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn InstallAlternateWriter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        idwriter: Param0,
        clsid: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            idwriter.into_param().abi(),
            clsid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetIdentityInformation(&self) -> *mut IVssExamineWriterMetadata {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriterFailureEx<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hr: ::windows::runtime::HRESULT,
        hrapplication: ::windows::runtime::HRESULT,
        wszapplicationmessage: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hr),
            ::std::mem::transmute(hrapplication),
            wszapplicationmessage.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetSessionId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn IsWriterShuttingDown(&self) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IVssWriterImpl {
    type Vtable = IVssWriterImpl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IVssWriterImpl> for ::windows::runtime::IUnknown {
    fn from(value: IVssWriterImpl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVssWriterImpl> for ::windows::runtime::IUnknown {
    fn from(value: &IVssWriterImpl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVssWriterImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVssWriterImpl {
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
pub struct IVssWriterImpl_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        writerid: ::windows::runtime::GUID,
        wszwritername: super::super::Foundation::PWSTR,
        wszwriterinstancename: super::super::Foundation::PWSTR,
        dwmajorversion: u32,
        dwminorversion: u32,
        ut: VSS_USAGE_TYPE,
        st: VSS_SOURCE_TYPE,
        nlevel: VSS_APPLICATION_LEVEL,
        dwtimeout: u32,
        aws: VSS_ALTERNATE_WRITER_STATE,
        biothrottlingonly: u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwsubscribetimeout: u32,
        dweventflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> *mut super::super::Foundation::PWSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszoriginalvolume: super::super::Foundation::PWSTR,
        ppwszsnapshotdevice: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::GUID,
    ),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> i32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> VSS_APPLICATION_LEVEL,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wszpath: super::super::Foundation::PWSTR,
    ) -> bool,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> VSS_BACKUP_TYPE,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> VSS_RESTORE_TYPE,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hr: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> bool,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idwriter: ::windows::runtime::GUID,
        clsid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> *mut IVssExamineWriterMetadata,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hr: ::windows::runtime::HRESULT,
        hrapplication: ::windows::runtime::HRESULT,
        wszapplicationmessage: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idsession: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> bool,
);
pub const VSSCoordinator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3849956191,
    7364,
    17588,
    [190, 217, 222, 9, 145, 255, 6, 35],
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
pub struct VSS_ALTERNATE_WRITER_STATE(pub i32);
pub const VSS_AWS_UNDEFINED: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(0i32);
pub const VSS_AWS_NO_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE =
    VSS_ALTERNATE_WRITER_STATE(1i32);
pub const VSS_AWS_ALTERNATE_WRITER_EXISTS: VSS_ALTERNATE_WRITER_STATE =
    VSS_ALTERNATE_WRITER_STATE(2i32);
pub const VSS_AWS_THIS_IS_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE =
    VSS_ALTERNATE_WRITER_STATE(3i32);
impl ::std::convert::From<i32> for VSS_ALTERNATE_WRITER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_ALTERNATE_WRITER_STATE {
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
pub struct VSS_APPLICATION_LEVEL(pub i32);
pub const VSS_APP_UNKNOWN: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(0i32);
pub const VSS_APP_SYSTEM: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(1i32);
pub const VSS_APP_BACK_END: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(2i32);
pub const VSS_APP_FRONT_END: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(3i32);
pub const VSS_APP_SYSTEM_RM: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(4i32);
pub const VSS_APP_AUTO: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(-1i32);
impl ::std::convert::From<i32> for VSS_APPLICATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_APPLICATION_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VSS_ASSOC_NO_MAX_SPACE: i32 = -1i32;
pub const VSS_ASSOC_REMOVE: u32 = 0u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for VSS_BACKUP_SCHEMA {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_BACKUP_SCHEMA {
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
pub struct VSS_BACKUP_TYPE(pub i32);
pub const VSS_BT_UNDEFINED: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(0i32);
pub const VSS_BT_FULL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(1i32);
pub const VSS_BT_INCREMENTAL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(2i32);
pub const VSS_BT_DIFFERENTIAL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(3i32);
pub const VSS_BT_LOG: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(4i32);
pub const VSS_BT_COPY: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(5i32);
pub const VSS_BT_OTHER: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(6i32);
impl ::std::convert::From<i32> for VSS_BACKUP_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_BACKUP_TYPE {
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
pub struct VSS_COMPONENT_FLAGS(pub i32);
pub const VSS_CF_BACKUP_RECOVERY: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(1i32);
pub const VSS_CF_APP_ROLLBACK_RECOVERY: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(2i32);
pub const VSS_CF_NOT_SYSTEM_STATE: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(4i32);
impl ::std::convert::From<i32> for VSS_COMPONENT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_COMPONENT_FLAGS {
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
pub struct VSS_COMPONENT_TYPE(pub i32);
pub const VSS_CT_UNDEFINED: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(0i32);
pub const VSS_CT_DATABASE: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(1i32);
pub const VSS_CT_FILEGROUP: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(2i32);
impl ::std::convert::From<i32> for VSS_COMPONENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_COMPONENT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VSS_DIFF_AREA_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszDiffAreaVolumeName: *mut u16,
    pub m_llMaximumDiffSpace: i64,
    pub m_llAllocatedDiffSpace: i64,
    pub m_llUsedDiffSpace: i64,
}
impl VSS_DIFF_AREA_PROP {}
impl ::std::default::Default for VSS_DIFF_AREA_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VSS_DIFF_AREA_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VSS_DIFF_AREA_PROP")
            .field("m_pwszVolumeName", &self.m_pwszVolumeName)
            .field("m_pwszDiffAreaVolumeName", &self.m_pwszDiffAreaVolumeName)
            .field("m_llMaximumDiffSpace", &self.m_llMaximumDiffSpace)
            .field("m_llAllocatedDiffSpace", &self.m_llAllocatedDiffSpace)
            .field("m_llUsedDiffSpace", &self.m_llUsedDiffSpace)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VSS_DIFF_AREA_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName
            && self.m_pwszDiffAreaVolumeName == other.m_pwszDiffAreaVolumeName
            && self.m_llMaximumDiffSpace == other.m_llMaximumDiffSpace
            && self.m_llAllocatedDiffSpace == other.m_llAllocatedDiffSpace
            && self.m_llUsedDiffSpace == other.m_llUsedDiffSpace
    }
}
impl ::std::cmp::Eq for VSS_DIFF_AREA_PROP {}
unsafe impl ::windows::runtime::Abi for VSS_DIFF_AREA_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VSS_DIFF_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
    pub m_llVolumeFreeSpace: i64,
    pub m_llVolumeTotalSpace: i64,
}
impl VSS_DIFF_VOLUME_PROP {}
impl ::std::default::Default for VSS_DIFF_VOLUME_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VSS_DIFF_VOLUME_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VSS_DIFF_VOLUME_PROP")
            .field("m_pwszVolumeName", &self.m_pwszVolumeName)
            .field("m_pwszVolumeDisplayName", &self.m_pwszVolumeDisplayName)
            .field("m_llVolumeFreeSpace", &self.m_llVolumeFreeSpace)
            .field("m_llVolumeTotalSpace", &self.m_llVolumeTotalSpace)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VSS_DIFF_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName
            && self.m_pwszVolumeDisplayName == other.m_pwszVolumeDisplayName
            && self.m_llVolumeFreeSpace == other.m_llVolumeFreeSpace
            && self.m_llVolumeTotalSpace == other.m_llVolumeTotalSpace
    }
}
impl ::std::cmp::Eq for VSS_DIFF_VOLUME_PROP {}
unsafe impl ::windows::runtime::Abi for VSS_DIFF_VOLUME_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212280i32 as _);
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212267i32 as _);
pub const VSS_E_ASRERROR_DATADISK_RDISK0: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212282i32 as _);
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212287i32 as _);
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212286i32 as _);
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212278i32 as _);
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION:
    ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212268i32 as _);
pub const VSS_E_ASRERROR_MISSING_DYNDISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212284i32 as _);
pub const VSS_E_ASRERROR_NO_ARCPATH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212285i32 as _);
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212269i32 as _);
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212281i32 as _);
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212270i32 as _);
pub const VSS_E_ASRERROR_SHARED_CRIDISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212283i32 as _);
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212266i32 as _);
pub const VSS_E_AUTORECOVERY_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212293i32 as _);
pub const VSS_E_BAD_STATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212543i32 as _);
pub const VSS_E_BREAK_REVERT_ID_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212298i32 as _);
pub const VSS_E_CANNOT_REVERT_DISKID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212290i32 as _);
pub const VSS_E_CLUSTER_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212288i32 as _);
pub const VSS_E_CLUSTER_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212498i32 as _);
pub const VSS_E_CORRUPT_XML_DOCUMENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212528i32 as _);
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212271i32 as _);
pub const VSS_E_DYNAMIC_DISK_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212292i32 as _);
pub const VSS_E_FLUSH_WRITES_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212525i32 as _);
pub const VSS_E_FSS_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212265i32 as _);
pub const VSS_E_HOLD_WRITES_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212524i32 as _);
pub const VSS_E_INSUFFICIENT_STORAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212513i32 as _);
pub const VSS_E_INVALID_XML_DOCUMENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212527i32 as _);
pub const VSS_E_LEGACY_PROVIDER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212297i32 as _);
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212514i32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212510i32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212521i32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212526i32 as _);
pub const VSS_E_MISSING_DISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212296i32 as _);
pub const VSS_E_MISSING_HIDDEN_VOLUME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212295i32 as _);
pub const VSS_E_MISSING_VOLUME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212294i32 as _);
pub const VSS_E_NESTED_VOLUME_LIMIT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212500i32 as _);
pub const VSS_E_NONTRANSPORTABLE_BCD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212291i32 as _);
pub const VSS_E_NOT_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212497i32 as _);
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212512i32 as _);
pub const VSS_E_OBJECT_ALREADY_EXISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212531i32 as _);
pub const VSS_E_OBJECT_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212536i32 as _);
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212541i32 as _);
pub const VSS_E_PROVIDER_IN_USE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212537i32 as _);
pub const VSS_E_PROVIDER_NOT_REGISTERED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212540i32 as _);
pub const VSS_E_PROVIDER_VETO: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212538i32 as _);
pub const VSS_E_REBOOT_REQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212505i32 as _);
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212509i32 as _);
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212508i32 as _);
pub const VSS_E_RESYNC_IN_PROGRESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212289i32 as _);
pub const VSS_E_REVERT_IN_PROGRESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212507i32 as _);
pub const VSS_E_REVERT_VOLUME_LOST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212506i32 as _);
pub const VSS_E_SNAPSHOT_NOT_IN_SET: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212501i32 as _);
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212522i32 as _);
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212511i32 as _);
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212504i32 as _);
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212503i32 as _);
pub const VSS_E_UNEXPECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212542i32 as _);
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212529i32 as _);
pub const VSS_E_UNEXPECTED_WRITER_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212523i32 as _);
pub const VSS_E_UNSELECTED_VOLUME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212502i32 as _);
pub const VSS_E_UNSUPPORTED_CONTEXT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212517i32 as _);
pub const VSS_E_VOLUME_IN_USE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212515i32 as _);
pub const VSS_E_VOLUME_NOT_LOCAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212499i32 as _);
pub const VSS_E_VOLUME_NOT_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212532i32 as _);
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212530i32 as _);
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212304i32 as _);
pub const VSS_E_WRITERERROR_NONRETRYABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212300i32 as _);
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212303i32 as _);
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212490i32 as _);
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212299i32 as _);
pub const VSS_E_WRITERERROR_RETRYABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212301i32 as _);
pub const VSS_E_WRITERERROR_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212302i32 as _);
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212518i32 as _);
pub const VSS_E_WRITER_INFRASTRUCTURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212520i32 as _);
pub const VSS_E_WRITER_NOT_RESPONDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212519i32 as _);
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2147212279i32 as _);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VSS_FILE_RESTORE_STATUS(pub i32);
pub const VSS_RS_UNDEFINED: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(0i32);
pub const VSS_RS_NONE: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(1i32);
pub const VSS_RS_ALL: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(2i32);
pub const VSS_RS_FAILED: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(3i32);
impl ::std::convert::From<i32> for VSS_FILE_RESTORE_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_FILE_RESTORE_STATUS {
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
pub struct VSS_FILE_SPEC_BACKUP_TYPE(pub i32);
pub const VSS_FSBT_FULL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE =
    VSS_FILE_SPEC_BACKUP_TYPE(1i32);
pub const VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE =
    VSS_FILE_SPEC_BACKUP_TYPE(2i32);
pub const VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE =
    VSS_FILE_SPEC_BACKUP_TYPE(4i32);
pub const VSS_FSBT_LOG_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(8i32);
pub const VSS_FSBT_FULL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE =
    VSS_FILE_SPEC_BACKUP_TYPE(256i32);
pub const VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE =
    VSS_FILE_SPEC_BACKUP_TYPE(512i32);
pub const VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE =
    VSS_FILE_SPEC_BACKUP_TYPE(1024i32);
pub const VSS_FSBT_LOG_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE =
    VSS_FILE_SPEC_BACKUP_TYPE(2048i32);
pub const VSS_FSBT_CREATED_DURING_BACKUP: VSS_FILE_SPEC_BACKUP_TYPE =
    VSS_FILE_SPEC_BACKUP_TYPE(65536i32);
pub const VSS_FSBT_ALL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE =
    VSS_FILE_SPEC_BACKUP_TYPE(15i32);
pub const VSS_FSBT_ALL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE =
    VSS_FILE_SPEC_BACKUP_TYPE(3840i32);
impl ::std::convert::From<i32> for VSS_FILE_SPEC_BACKUP_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_FILE_SPEC_BACKUP_TYPE {
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
pub struct VSS_HARDWARE_OPTIONS(pub i32);
pub const VSS_BREAKEX_FLAG_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(1i32);
pub const VSS_BREAKEX_FLAG_MAKE_READ_WRITE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2i32);
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(4i32);
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(8i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: VSS_HARDWARE_OPTIONS =
    VSS_HARDWARE_OPTIONS(256i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: VSS_HARDWARE_OPTIONS =
    VSS_HARDWARE_OPTIONS(512i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: VSS_HARDWARE_OPTIONS =
    VSS_HARDWARE_OPTIONS(1024i32);
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2048i32);
impl ::std::convert::From<i32> for VSS_HARDWARE_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_HARDWARE_OPTIONS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VSS_MGMT_OBJECT_PROP {
    pub Type: VSS_MGMT_OBJECT_TYPE,
    pub Obj: VSS_MGMT_OBJECT_UNION,
}
impl VSS_MGMT_OBJECT_PROP {}
impl ::std::default::Default for VSS_MGMT_OBJECT_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VSS_MGMT_OBJECT_PROP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VSS_MGMT_OBJECT_PROP {}
unsafe impl ::windows::runtime::Abi for VSS_MGMT_OBJECT_PROP {
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
pub struct VSS_MGMT_OBJECT_TYPE(pub i32);
pub const VSS_MGMT_OBJECT_UNKNOWN: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(0i32);
pub const VSS_MGMT_OBJECT_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(1i32);
pub const VSS_MGMT_OBJECT_DIFF_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(2i32);
pub const VSS_MGMT_OBJECT_DIFF_AREA: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(3i32);
impl ::std::convert::From<i32> for VSS_MGMT_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_MGMT_OBJECT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VSS_MGMT_OBJECT_UNION {
    pub Vol: VSS_VOLUME_PROP,
    pub DiffVol: VSS_DIFF_VOLUME_PROP,
    pub DiffArea: VSS_DIFF_AREA_PROP,
}
impl VSS_MGMT_OBJECT_UNION {}
impl ::std::default::Default for VSS_MGMT_OBJECT_UNION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VSS_MGMT_OBJECT_UNION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VSS_MGMT_OBJECT_UNION {}
unsafe impl ::windows::runtime::Abi for VSS_MGMT_OBJECT_UNION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl VSS_OBJECT_PROP {}
impl ::std::default::Default for VSS_OBJECT_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VSS_OBJECT_PROP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VSS_OBJECT_PROP {}
unsafe impl ::windows::runtime::Abi for VSS_OBJECT_PROP {
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
pub struct VSS_OBJECT_TYPE(pub i32);
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(0i32);
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(1i32);
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(2i32);
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(3i32);
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(4i32);
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(5i32);
impl ::std::convert::From<i32> for VSS_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_OBJECT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl VSS_OBJECT_UNION {}
impl ::std::default::Default for VSS_OBJECT_UNION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VSS_OBJECT_UNION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VSS_OBJECT_UNION {}
unsafe impl ::windows::runtime::Abi for VSS_OBJECT_UNION {
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
pub struct VSS_PROTECTION_FAULT(pub i32);
pub const VSS_PROTECTION_FAULT_NONE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(0i32);
pub const VSS_PROTECTION_FAULT_DIFF_AREA_MISSING: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(1i32);
pub const VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE: VSS_PROTECTION_FAULT =
    VSS_PROTECTION_FAULT(2i32);
pub const VSS_PROTECTION_FAULT_META_DATA_CORRUPTION: VSS_PROTECTION_FAULT =
    VSS_PROTECTION_FAULT(3i32);
pub const VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE: VSS_PROTECTION_FAULT =
    VSS_PROTECTION_FAULT(4i32);
pub const VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE: VSS_PROTECTION_FAULT =
    VSS_PROTECTION_FAULT(5i32);
pub const VSS_PROTECTION_FAULT_COW_READ_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(6i32);
pub const VSS_PROTECTION_FAULT_COW_WRITE_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(7i32);
pub const VSS_PROTECTION_FAULT_DIFF_AREA_FULL: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(8i32);
pub const VSS_PROTECTION_FAULT_GROW_TOO_SLOW: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(9i32);
pub const VSS_PROTECTION_FAULT_GROW_FAILED: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(10i32);
pub const VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS: VSS_PROTECTION_FAULT =
    VSS_PROTECTION_FAULT(11i32);
pub const VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE: VSS_PROTECTION_FAULT =
    VSS_PROTECTION_FAULT(12i32);
pub const VSS_PROTECTION_FAULT_IO_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(13i32);
pub const VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED: VSS_PROTECTION_FAULT =
    VSS_PROTECTION_FAULT(14i32);
pub const VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA: VSS_PROTECTION_FAULT =
    VSS_PROTECTION_FAULT(15i32);
pub const VSS_PROTECTION_FAULT_MOUNT_DURING_CLUSTER_OFFLINE: VSS_PROTECTION_FAULT =
    VSS_PROTECTION_FAULT(16i32);
impl ::std::convert::From<i32> for VSS_PROTECTION_FAULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_PROTECTION_FAULT {
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
pub struct VSS_PROTECTION_LEVEL(pub i32);
pub const VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(0i32);
pub const VSS_PROTECTION_LEVEL_SNAPSHOT: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(1i32);
impl ::std::convert::From<i32> for VSS_PROTECTION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_PROTECTION_LEVEL {
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
pub struct VSS_PROVIDER_CAPABILITIES(pub i32);
pub const VSS_PRV_CAPABILITY_LEGACY: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(1i32);
pub const VSS_PRV_CAPABILITY_COMPLIANT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(2i32);
pub const VSS_PRV_CAPABILITY_LUN_REPOINT: VSS_PROVIDER_CAPABILITIES =
    VSS_PROVIDER_CAPABILITIES(4i32);
pub const VSS_PRV_CAPABILITY_LUN_RESYNC: VSS_PROVIDER_CAPABILITIES =
    VSS_PROVIDER_CAPABILITIES(8i32);
pub const VSS_PRV_CAPABILITY_OFFLINE_CREATION: VSS_PROVIDER_CAPABILITIES =
    VSS_PROVIDER_CAPABILITIES(16i32);
pub const VSS_PRV_CAPABILITY_MULTIPLE_IMPORT: VSS_PROVIDER_CAPABILITIES =
    VSS_PROVIDER_CAPABILITIES(32i32);
pub const VSS_PRV_CAPABILITY_RECYCLING: VSS_PROVIDER_CAPABILITIES =
    VSS_PROVIDER_CAPABILITIES(64i32);
pub const VSS_PRV_CAPABILITY_PLEX: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(128i32);
pub const VSS_PRV_CAPABILITY_DIFFERENTIAL: VSS_PROVIDER_CAPABILITIES =
    VSS_PROVIDER_CAPABILITIES(256i32);
pub const VSS_PRV_CAPABILITY_CLUSTERED: VSS_PROVIDER_CAPABILITIES =
    VSS_PROVIDER_CAPABILITIES(512i32);
impl ::std::convert::From<i32> for VSS_PROVIDER_CAPABILITIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_PROVIDER_CAPABILITIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: ::windows::runtime::GUID,
    pub m_pwszProviderName: *mut u16,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: *mut u16,
    pub m_ProviderVersionId: ::windows::runtime::GUID,
    pub m_ClassId: ::windows::runtime::GUID,
}
impl VSS_PROVIDER_PROP {}
impl ::std::default::Default for VSS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VSS_PROVIDER_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for VSS_PROVIDER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_ProviderId == other.m_ProviderId
            && self.m_pwszProviderName == other.m_pwszProviderName
            && self.m_eProviderType == other.m_eProviderType
            && self.m_pwszProviderVersion == other.m_pwszProviderVersion
            && self.m_ProviderVersionId == other.m_ProviderVersionId
            && self.m_ClassId == other.m_ClassId
    }
}
impl ::std::cmp::Eq for VSS_PROVIDER_PROP {}
unsafe impl ::windows::runtime::Abi for VSS_PROVIDER_PROP {
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
pub struct VSS_PROVIDER_TYPE(pub i32);
pub const VSS_PROV_UNKNOWN: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(0i32);
pub const VSS_PROV_SYSTEM: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(1i32);
pub const VSS_PROV_SOFTWARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(2i32);
pub const VSS_PROV_HARDWARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(3i32);
pub const VSS_PROV_FILESHARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(4i32);
impl ::std::convert::From<i32> for VSS_PROVIDER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_PROVIDER_TYPE {
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
pub struct VSS_RECOVERY_OPTIONS(pub i32);
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(256i32);
pub const VSS_RECOVERY_NO_VOLUME_CHECK: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(512i32);
impl ::std::convert::From<i32> for VSS_RECOVERY_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_RECOVERY_OPTIONS {
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
pub struct VSS_RESTOREMETHOD_ENUM(pub i32);
pub const VSS_RME_UNDEFINED: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(0i32);
pub const VSS_RME_RESTORE_IF_NOT_THERE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(1i32);
pub const VSS_RME_RESTORE_IF_CAN_REPLACE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(2i32);
pub const VSS_RME_STOP_RESTORE_START: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(3i32);
pub const VSS_RME_RESTORE_TO_ALTERNATE_LOCATION: VSS_RESTOREMETHOD_ENUM =
    VSS_RESTOREMETHOD_ENUM(4i32);
pub const VSS_RME_RESTORE_AT_REBOOT: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(5i32);
pub const VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE: VSS_RESTOREMETHOD_ENUM =
    VSS_RESTOREMETHOD_ENUM(6i32);
pub const VSS_RME_CUSTOM: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(7i32);
pub const VSS_RME_RESTORE_STOP_START: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(8i32);
impl ::std::convert::From<i32> for VSS_RESTOREMETHOD_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_RESTOREMETHOD_ENUM {
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
pub struct VSS_RESTORE_TARGET(pub i32);
pub const VSS_RT_UNDEFINED: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(0i32);
pub const VSS_RT_ORIGINAL: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(1i32);
pub const VSS_RT_ALTERNATE: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(2i32);
pub const VSS_RT_DIRECTED: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(3i32);
pub const VSS_RT_ORIGINAL_LOCATION: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(4i32);
impl ::std::convert::From<i32> for VSS_RESTORE_TARGET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_RESTORE_TARGET {
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
pub struct VSS_RESTORE_TYPE(pub i32);
pub const VSS_RTYPE_UNDEFINED: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(0i32);
pub const VSS_RTYPE_BY_COPY: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(1i32);
pub const VSS_RTYPE_IMPORT: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(2i32);
pub const VSS_RTYPE_OTHER: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(3i32);
impl ::std::convert::From<i32> for VSS_RESTORE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_RESTORE_TYPE {
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
pub struct VSS_ROLLFORWARD_TYPE(pub i32);
pub const VSS_RF_UNDEFINED: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(0i32);
pub const VSS_RF_NONE: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(1i32);
pub const VSS_RF_ALL: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(2i32);
pub const VSS_RF_PARTIAL: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(3i32);
impl ::std::convert::From<i32> for VSS_ROLLFORWARD_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_ROLLFORWARD_TYPE {
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
pub struct VSS_SNAPSHOT_COMPATIBILITY(pub i32);
pub const VSS_SC_DISABLE_DEFRAG: VSS_SNAPSHOT_COMPATIBILITY = VSS_SNAPSHOT_COMPATIBILITY(1i32);
pub const VSS_SC_DISABLE_CONTENTINDEX: VSS_SNAPSHOT_COMPATIBILITY =
    VSS_SNAPSHOT_COMPATIBILITY(2i32);
impl ::std::convert::From<i32> for VSS_SNAPSHOT_COMPATIBILITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_SNAPSHOT_COMPATIBILITY {
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
pub struct VSS_SNAPSHOT_CONTEXT(pub i32);
pub const VSS_CTX_BACKUP: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(0i32);
pub const VSS_CTX_FILE_SHARE_BACKUP: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(16i32);
pub const VSS_CTX_NAS_ROLLBACK: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(25i32);
pub const VSS_CTX_APP_ROLLBACK: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(9i32);
pub const VSS_CTX_CLIENT_ACCESSIBLE: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(29i32);
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(13i32);
pub const VSS_CTX_ALL: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(-1i32);
impl ::std::convert::From<i32> for VSS_SNAPSHOT_CONTEXT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_SNAPSHOT_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: ::windows::runtime::GUID,
    pub m_SnapshotSetId: ::windows::runtime::GUID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: *mut u16,
    pub m_pwszOriginalVolumeName: *mut u16,
    pub m_pwszOriginatingMachine: *mut u16,
    pub m_pwszServiceMachine: *mut u16,
    pub m_pwszExposedName: *mut u16,
    pub m_pwszExposedPath: *mut u16,
    pub m_ProviderId: ::windows::runtime::GUID,
    pub m_lSnapshotAttributes: i32,
    pub m_tsCreationTimestamp: i64,
    pub m_eStatus: VSS_SNAPSHOT_STATE,
}
impl VSS_SNAPSHOT_PROP {}
impl ::std::default::Default for VSS_SNAPSHOT_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VSS_SNAPSHOT_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VSS_SNAPSHOT_PROP")
            .field("m_SnapshotId", &self.m_SnapshotId)
            .field("m_SnapshotSetId", &self.m_SnapshotSetId)
            .field("m_lSnapshotsCount", &self.m_lSnapshotsCount)
            .field(
                "m_pwszSnapshotDeviceObject",
                &self.m_pwszSnapshotDeviceObject,
            )
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
impl ::std::cmp::PartialEq for VSS_SNAPSHOT_PROP {
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
impl ::std::cmp::Eq for VSS_SNAPSHOT_PROP {}
unsafe impl ::windows::runtime::Abi for VSS_SNAPSHOT_PROP {
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
pub struct VSS_SNAPSHOT_PROPERTY_ID(pub i32);
pub const VSS_SPROPID_UNKNOWN: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(0i32);
pub const VSS_SPROPID_SNAPSHOT_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(1i32);
pub const VSS_SPROPID_SNAPSHOT_SET_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(2i32);
pub const VSS_SPROPID_SNAPSHOTS_COUNT: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(3i32);
pub const VSS_SPROPID_SNAPSHOT_DEVICE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(4i32);
pub const VSS_SPROPID_ORIGINAL_VOLUME: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(5i32);
pub const VSS_SPROPID_ORIGINATING_MACHINE: VSS_SNAPSHOT_PROPERTY_ID =
    VSS_SNAPSHOT_PROPERTY_ID(6i32);
pub const VSS_SPROPID_SERVICE_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(7i32);
pub const VSS_SPROPID_EXPOSED_NAME: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(8i32);
pub const VSS_SPROPID_EXPOSED_PATH: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(9i32);
pub const VSS_SPROPID_PROVIDER_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(10i32);
pub const VSS_SPROPID_SNAPSHOT_ATTRIBUTES: VSS_SNAPSHOT_PROPERTY_ID =
    VSS_SNAPSHOT_PROPERTY_ID(11i32);
pub const VSS_SPROPID_CREATION_TIMESTAMP: VSS_SNAPSHOT_PROPERTY_ID =
    VSS_SNAPSHOT_PROPERTY_ID(12i32);
pub const VSS_SPROPID_STATUS: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(13i32);
impl ::std::convert::From<i32> for VSS_SNAPSHOT_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_SNAPSHOT_PROPERTY_ID {
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
impl ::std::convert::From<i32> for VSS_SNAPSHOT_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_SNAPSHOT_STATE {
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
pub struct VSS_SOURCE_TYPE(pub i32);
pub const VSS_ST_UNDEFINED: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(0i32);
pub const VSS_ST_TRANSACTEDDB: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(1i32);
pub const VSS_ST_NONTRANSACTEDDB: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(2i32);
pub const VSS_ST_OTHER: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(3i32);
impl ::std::convert::From<i32> for VSS_SOURCE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_SOURCE_TYPE {
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
pub struct VSS_SUBSCRIBE_MASK(pub i32);
pub const VSS_SM_POST_SNAPSHOT_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(1i32);
pub const VSS_SM_BACKUP_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(2i32);
pub const VSS_SM_RESTORE_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(4i32);
pub const VSS_SM_IO_THROTTLING_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(8i32);
pub const VSS_SM_ALL_FLAGS: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(-1i32);
impl ::std::convert::From<i32> for VSS_SUBSCRIBE_MASK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_SUBSCRIBE_MASK {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VSS_S_ASYNC_CANCELLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(271115i32 as _);
pub const VSS_S_ASYNC_FINISHED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(271114i32 as _);
pub const VSS_S_ASYNC_PENDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(271113i32 as _);
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(271137i32 as _);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VSS_USAGE_TYPE(pub i32);
pub const VSS_UT_UNDEFINED: VSS_USAGE_TYPE = VSS_USAGE_TYPE(0i32);
pub const VSS_UT_BOOTABLESYSTEMSTATE: VSS_USAGE_TYPE = VSS_USAGE_TYPE(1i32);
pub const VSS_UT_SYSTEMSERVICE: VSS_USAGE_TYPE = VSS_USAGE_TYPE(2i32);
pub const VSS_UT_USERDATA: VSS_USAGE_TYPE = VSS_USAGE_TYPE(3i32);
pub const VSS_UT_OTHER: VSS_USAGE_TYPE = VSS_USAGE_TYPE(4i32);
impl ::std::convert::From<i32> for VSS_USAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_USAGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VSS_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
}
impl VSS_VOLUME_PROP {}
impl ::std::default::Default for VSS_VOLUME_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VSS_VOLUME_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VSS_VOLUME_PROP")
            .field("m_pwszVolumeName", &self.m_pwszVolumeName)
            .field("m_pwszVolumeDisplayName", &self.m_pwszVolumeDisplayName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VSS_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName
            && self.m_pwszVolumeDisplayName == other.m_pwszVolumeDisplayName
    }
}
impl ::std::cmp::Eq for VSS_VOLUME_PROP {}
unsafe impl ::windows::runtime::Abi for VSS_VOLUME_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for VSS_VOLUME_PROTECTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VSS_VOLUME_PROTECTION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VSS_VOLUME_PROTECTION_INFO")
            .field("m_protectionLevel", &self.m_protectionLevel)
            .field(
                "m_volumeIsOfflineForProtection",
                &self.m_volumeIsOfflineForProtection,
            )
            .field("m_protectionFault", &self.m_protectionFault)
            .field("m_failureStatus", &self.m_failureStatus)
            .field("m_volumeHasUnusedDiffArea", &self.m_volumeHasUnusedDiffArea)
            .field("m_reserved", &self.m_reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VSS_VOLUME_PROTECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.m_protectionLevel == other.m_protectionLevel
            && self.m_volumeIsOfflineForProtection == other.m_volumeIsOfflineForProtection
            && self.m_protectionFault == other.m_protectionFault
            && self.m_failureStatus == other.m_failureStatus
            && self.m_volumeHasUnusedDiffArea == other.m_volumeHasUnusedDiffArea
            && self.m_reserved == other.m_reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VSS_VOLUME_PROTECTION_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VSS_VOLUME_PROTECTION_INFO {
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
pub struct VSS_VOLUME_SNAPSHOT_ATTRIBUTES(pub i32);
pub const VSS_VOLSNAP_ATTR_PERSISTENT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(1i32);
pub const VSS_VOLSNAP_ATTR_NO_AUTORECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(2i32);
pub const VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(4i32);
pub const VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(8i32);
pub const VSS_VOLSNAP_ATTR_NO_WRITERS: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(16i32);
pub const VSS_VOLSNAP_ATTR_TRANSPORTABLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(32i32);
pub const VSS_VOLSNAP_ATTR_NOT_SURFACED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(64i32);
pub const VSS_VOLSNAP_ATTR_NOT_TRANSACTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(128i32);
pub const VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(65536i32);
pub const VSS_VOLSNAP_ATTR_DIFFERENTIAL: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(131072i32);
pub const VSS_VOLSNAP_ATTR_PLEX: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(262144i32);
pub const VSS_VOLSNAP_ATTR_IMPORTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(524288i32);
pub const VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(1048576i32);
pub const VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(2097152i32);
pub const VSS_VOLSNAP_ATTR_AUTORECOVER: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(4194304i32);
pub const VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(8388608i32);
pub const VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(16777216i32);
pub const VSS_VOLSNAP_ATTR_TXF_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(33554432i32);
pub const VSS_VOLSNAP_ATTR_FILE_SHARE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES =
    VSS_VOLUME_SNAPSHOT_ATTRIBUTES(67108864i32);
impl ::std::convert::From<i32> for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
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
pub struct VSS_WRITERRESTORE_ENUM(pub i32);
pub const VSS_WRE_UNDEFINED: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(0i32);
pub const VSS_WRE_NEVER: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(1i32);
pub const VSS_WRE_IF_REPLACE_FAILS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(2i32);
pub const VSS_WRE_ALWAYS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(3i32);
impl ::std::convert::From<i32> for VSS_WRITERRESTORE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_WRITERRESTORE_ENUM {
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
impl ::std::convert::From<i32> for VSS_WRITER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VSS_WRITER_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VssSnapshotMgmt: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    190458962,
    16057,
    18186,
    [150, 226, 108, 109, 69, 112, 228, 15],
);
