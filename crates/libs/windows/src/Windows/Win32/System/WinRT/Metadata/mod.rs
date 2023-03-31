#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[inline]
pub unsafe fn MetaDataGetDispenser(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "rometadata.dll""system" fn MetaDataGetDispenser ( rclsid : *const ::windows::core::GUID , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    MetaDataGetDispenser(rclsid, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[inline]
pub unsafe fn RoCreateNonAgilePropertySet() -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IPropertySet> {
    ::windows_targets::link ! ( "api-ms-win-ro-typeresolution-l1-1-1.dll""system" fn RoCreateNonAgilePropertySet ( pppropertyset : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IPropertySet>();
    RoCreateNonAgilePropertySet(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`, `\"Storage_Streams\"`*"]
#[cfg(feature = "Storage_Streams")]
#[inline]
pub unsafe fn RoCreatePropertySetSerializer() -> ::windows::core::Result<super::super::super::super::Storage::Streams::IPropertySetSerializer> {
    ::windows_targets::link ! ( "api-ms-win-ro-typeresolution-l1-1-1.dll""system" fn RoCreatePropertySetSerializer ( pppropertysetserializer : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::super::Storage::Streams::IPropertySetSerializer>();
    RoCreatePropertySetSerializer(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[inline]
pub unsafe fn RoFreeParameterizedTypeExtra<P0>(extra: P0)
where
    P0: ::windows::core::IntoParam<super::ROPARAMIIDHANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll""system" fn RoFreeParameterizedTypeExtra ( extra : super:: ROPARAMIIDHANDLE ) -> ( ) );
    RoFreeParameterizedTypeExtra(extra.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[inline]
pub unsafe fn RoGetMetaDataFile<P0>(name: &::windows::core::HSTRING, metadatadispenser: P0, metadatafilepath: ::core::option::Option<*mut ::windows::core::HSTRING>, metadataimport: ::core::option::Option<*mut ::core::option::Option<IMetaDataImport2>>, typedeftoken: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IMetaDataDispenserEx>,
{
    ::windows_targets::link ! ( "api-ms-win-ro-typeresolution-l1-1-0.dll""system" fn RoGetMetaDataFile ( name : ::std::mem::MaybeUninit <::windows::core::HSTRING > , metadatadispenser : * mut::core::ffi::c_void , metadatafilepath : *mut ::std::mem::MaybeUninit <::windows::core::HSTRING > , metadataimport : *mut * mut::core::ffi::c_void , typedeftoken : *mut u32 ) -> ::windows::core::HRESULT );
    RoGetMetaDataFile(::core::mem::transmute_copy(name), metadatadispenser.into_param().abi(), ::core::mem::transmute(metadatafilepath.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(metadataimport.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(typedeftoken.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[inline]
pub unsafe fn RoGetParameterizedTypeInstanceIID<P0>(nameelements: &[::windows::core::PCWSTR], metadatalocator: P0, iid: *mut ::windows::core::GUID, pextra: ::core::option::Option<*mut super::ROPARAMIIDHANDLE>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IRoMetaDataLocator>,
{
    ::windows_targets::link ! ( "api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll""system" fn RoGetParameterizedTypeInstanceIID ( nameelementcount : u32 , nameelements : *const ::windows::core::PCWSTR , metadatalocator : * mut::core::ffi::c_void , iid : *mut ::windows::core::GUID , pextra : *mut super:: ROPARAMIIDHANDLE ) -> ::windows::core::HRESULT );
    RoGetParameterizedTypeInstanceIID(nameelements.len() as _, ::core::mem::transmute(nameelements.as_ptr()), metadatalocator.into_param().abi(), iid, ::core::mem::transmute(pextra.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoIsApiContractMajorVersionPresent<P0>(name: P0, majorversion: u16) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-ro-typeresolution-l1-1-1.dll""system" fn RoIsApiContractMajorVersionPresent ( name : ::windows::core::PCWSTR , majorversion : u16 , present : *mut super::super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
    RoIsApiContractMajorVersionPresent(name.into_param().abi(), majorversion, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoIsApiContractPresent<P0>(name: P0, majorversion: u16, minorversion: u16) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-ro-typeresolution-l1-1-1.dll""system" fn RoIsApiContractPresent ( name : ::windows::core::PCWSTR , majorversion : u16 , minorversion : u16 , present : *mut super::super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
    RoIsApiContractPresent(name.into_param().abi(), majorversion, minorversion, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[inline]
pub unsafe fn RoParameterizedTypeExtraGetTypeSignature<P0>(extra: P0) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<super::ROPARAMIIDHANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll""system" fn RoParameterizedTypeExtraGetTypeSignature ( extra : super:: ROPARAMIIDHANDLE ) -> ::windows::core::PCSTR );
    RoParameterizedTypeExtraGetTypeSignature(extra.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[inline]
pub unsafe fn RoParseTypeName(typename: &::windows::core::HSTRING, partscount: *mut u32, typenameparts: *mut *mut ::windows::core::HSTRING) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-ro-typeresolution-l1-1-0.dll""system" fn RoParseTypeName ( typename : ::std::mem::MaybeUninit <::windows::core::HSTRING > , partscount : *mut u32 , typenameparts : *mut *mut ::windows::core::HSTRING ) -> ::windows::core::HRESULT );
    RoParseTypeName(::core::mem::transmute_copy(typename), partscount, typenameparts).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[inline]
pub unsafe fn RoResolveNamespace(name: &::windows::core::HSTRING, windowsmetadatadir: &::windows::core::HSTRING, packagegraphdirs: ::core::option::Option<&[::windows::core::HSTRING]>, metadatafilepathscount: ::core::option::Option<*mut u32>, metadatafilepaths: ::core::option::Option<*mut *mut ::windows::core::HSTRING>, subnamespacescount: ::core::option::Option<*mut u32>, subnamespaces: ::core::option::Option<*mut *mut ::windows::core::HSTRING>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-ro-typeresolution-l1-1-0.dll""system" fn RoResolveNamespace ( name : ::std::mem::MaybeUninit <::windows::core::HSTRING > , windowsmetadatadir : ::std::mem::MaybeUninit <::windows::core::HSTRING > , packagegraphdirscount : u32 , packagegraphdirs : *const ::std::mem::MaybeUninit <::windows::core::HSTRING > , metadatafilepathscount : *mut u32 , metadatafilepaths : *mut *mut ::windows::core::HSTRING , subnamespacescount : *mut u32 , subnamespaces : *mut *mut ::windows::core::HSTRING ) -> ::windows::core::HRESULT );
    RoResolveNamespace(
        ::core::mem::transmute_copy(name),
        ::core::mem::transmute_copy(windowsmetadatadir),
        packagegraphdirs.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(packagegraphdirs.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        ::core::mem::transmute(metadatafilepathscount.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(metadatafilepaths.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(subnamespacescount.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(subnamespaces.unwrap_or(::std::ptr::null_mut())),
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct ICeeGen(::windows::core::IUnknown);
impl ICeeGen {
    pub unsafe fn EmitString<P0>(&self, lpstring: P0, rva: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).EmitString)(::windows::core::Interface::as_raw(self), lpstring.into_param().abi(), rva).ok()
    }
    pub unsafe fn GetString(&self, rva: u32, lpstring: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetString)(::windows::core::Interface::as_raw(self), rva, ::core::mem::transmute(lpstring.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AllocateMethodBuffer(&self, cchbuffer: u32, lpbuffer: *mut *mut u8, rva: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateMethodBuffer)(::windows::core::Interface::as_raw(self), cchbuffer, lpbuffer, rva).ok()
    }
    pub unsafe fn GetMethodBuffer(&self, rva: u32, lpbuffer: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMethodBuffer)(::windows::core::Interface::as_raw(self), rva, lpbuffer).ok()
    }
    pub unsafe fn GetIMapTokenIface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetIMapTokenIface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GenerateCeeFile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateCeeFile)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetIlSection(&self, section: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIlSection)(::windows::core::Interface::as_raw(self), section).ok()
    }
    pub unsafe fn GetStringSection(&self, section: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStringSection)(::windows::core::Interface::as_raw(self), section).ok()
    }
    pub unsafe fn AddSectionReloc(&self, section: *mut ::core::ffi::c_void, offset: u32, relativeto: *mut ::core::ffi::c_void, reloctype: CeeSectionRelocType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddSectionReloc)(::windows::core::Interface::as_raw(self), section, offset, relativeto, reloctype).ok()
    }
    pub unsafe fn GetSectionCreate<P0>(&self, name: P0, flags: u32, section: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetSectionCreate)(::windows::core::Interface::as_raw(self), name.into_param().abi(), flags, section).ok()
    }
    pub unsafe fn GetSectionDataLen(&self, section: *mut ::core::ffi::c_void, datalen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSectionDataLen)(::windows::core::Interface::as_raw(self), section, datalen).ok()
    }
    pub unsafe fn GetSectionBlock(&self, section: *mut ::core::ffi::c_void, len: u32, align: u32, ppbytes: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSectionBlock)(::windows::core::Interface::as_raw(self), section, len, align, ppbytes).ok()
    }
    pub unsafe fn TruncateSection(&self, section: *mut ::core::ffi::c_void, len: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TruncateSection)(::windows::core::Interface::as_raw(self), section, len).ok()
    }
    pub unsafe fn GenerateCeeMemoryImage(&self, ppimage: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateCeeMemoryImage)(::windows::core::Interface::as_raw(self), ppimage).ok()
    }
    pub unsafe fn ComputePointer(&self, section: *mut ::core::ffi::c_void, rva: u32, lpbuffer: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ComputePointer)(::windows::core::Interface::as_raw(self), section, rva, lpbuffer).ok()
    }
}
::windows::imp::interface_hierarchy!(ICeeGen, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICeeGen {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICeeGen {}
impl ::core::fmt::Debug for ICeeGen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICeeGen").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICeeGen {
    type Vtable = ICeeGen_Vtbl;
}
impl ::core::clone::Clone for ICeeGen {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICeeGen {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed1bdff_8e36_11d2_9c56_00a0c9b7cc45);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICeeGen_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EmitString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpstring: ::windows::core::PCWSTR, rva: *mut u32) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rva: u32, lpstring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub AllocateMethodBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchbuffer: u32, lpbuffer: *mut *mut u8, rva: *mut u32) -> ::windows::core::HRESULT,
    pub GetMethodBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rva: u32, lpbuffer: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetIMapTokenIface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimaptoken: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GenerateCeeFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIlSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, section: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStringSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, section: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddSectionReloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, section: *mut ::core::ffi::c_void, offset: u32, relativeto: *mut ::core::ffi::c_void, reloctype: CeeSectionRelocType) -> ::windows::core::HRESULT,
    pub GetSectionCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, flags: u32, section: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSectionDataLen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, section: *mut ::core::ffi::c_void, datalen: *mut u32) -> ::windows::core::HRESULT,
    pub GetSectionBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, section: *mut ::core::ffi::c_void, len: u32, align: u32, ppbytes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TruncateSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, section: *mut ::core::ffi::c_void, len: u32) -> ::windows::core::HRESULT,
    pub GenerateCeeMemoryImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ComputePointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, section: *mut ::core::ffi::c_void, rva: u32, lpbuffer: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IHostFilter(::windows::core::IUnknown);
impl IHostFilter {
    pub unsafe fn MarkToken(&self, tk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MarkToken)(::windows::core::Interface::as_raw(self), tk).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostFilter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostFilter {}
impl ::core::fmt::Debug for IHostFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostFilter {
    type Vtable = IHostFilter_Vtbl;
}
impl ::core::clone::Clone for IHostFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0e80dd3_12d4_11d3_b39d_00c04ff81795);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostFilter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub MarkToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMapToken(::windows::core::IUnknown);
impl IMapToken {
    pub unsafe fn Map(&self, tkimp: u32, tkemit: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Map)(::windows::core::Interface::as_raw(self), tkimp, tkemit).ok()
    }
}
::windows::imp::interface_hierarchy!(IMapToken, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMapToken {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMapToken {}
impl ::core::fmt::Debug for IMapToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMapToken").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMapToken {
    type Vtable = IMapToken_Vtbl;
}
impl ::core::clone::Clone for IMapToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMapToken {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06a3ea8b_0225_11d1_bf72_00c04fc31e12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapToken_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tkimp: u32, tkemit: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataAssemblyEmit(::windows::core::IUnknown);
impl IMetaDataAssemblyEmit {
    pub unsafe fn DefineAssembly<P0>(&self, pbpublickey: *const ::core::ffi::c_void, cbpublickey: u32, ulhashalgid: u32, szname: P0, pmetadata: *const ASSEMBLYMETADATA, dwassemblyflags: u32, pma: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineAssembly)(::windows::core::Interface::as_raw(self), pbpublickey, cbpublickey, ulhashalgid, szname.into_param().abi(), pmetadata, dwassemblyflags, pma).ok()
    }
    pub unsafe fn DefineAssemblyRef<P0>(&self, pbpublickeyortoken: *const ::core::ffi::c_void, cbpublickeyortoken: u32, szname: P0, pmetadata: *const ASSEMBLYMETADATA, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, dwassemblyrefflags: u32, pmdar: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineAssemblyRef)(::windows::core::Interface::as_raw(self), pbpublickeyortoken, cbpublickeyortoken, szname.into_param().abi(), pmetadata, pbhashvalue, cbhashvalue, dwassemblyrefflags, pmdar).ok()
    }
    pub unsafe fn DefineFile<P0>(&self, szname: P0, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, dwfileflags: u32, pmdf: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineFile)(::windows::core::Interface::as_raw(self), szname.into_param().abi(), pbhashvalue, cbhashvalue, dwfileflags, pmdf).ok()
    }
    pub unsafe fn DefineExportedType<P0>(&self, szname: P0, tkimplementation: u32, tktypedef: u32, dwexportedtypeflags: u32, pmdct: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineExportedType)(::windows::core::Interface::as_raw(self), szname.into_param().abi(), tkimplementation, tktypedef, dwexportedtypeflags, pmdct).ok()
    }
    pub unsafe fn DefineManifestResource<P0>(&self, szname: P0, tkimplementation: u32, dwoffset: u32, dwresourceflags: u32, pmdmr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineManifestResource)(::windows::core::Interface::as_raw(self), szname.into_param().abi(), tkimplementation, dwoffset, dwresourceflags, pmdmr).ok()
    }
    pub unsafe fn SetAssemblyProps<P0>(&self, pma: u32, pbpublickey: *const ::core::ffi::c_void, cbpublickey: u32, ulhashalgid: u32, szname: P0, pmetadata: *const ASSEMBLYMETADATA, dwassemblyflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAssemblyProps)(::windows::core::Interface::as_raw(self), pma, pbpublickey, cbpublickey, ulhashalgid, szname.into_param().abi(), pmetadata, dwassemblyflags).ok()
    }
    pub unsafe fn SetAssemblyRefProps<P0>(&self, ar: u32, pbpublickeyortoken: *const ::core::ffi::c_void, cbpublickeyortoken: u32, szname: P0, pmetadata: *const ASSEMBLYMETADATA, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, dwassemblyrefflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAssemblyRefProps)(::windows::core::Interface::as_raw(self), ar, pbpublickeyortoken, cbpublickeyortoken, szname.into_param().abi(), pmetadata, pbhashvalue, cbhashvalue, dwassemblyrefflags).ok()
    }
    pub unsafe fn SetFileProps(&self, file: u32, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, dwfileflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileProps)(::windows::core::Interface::as_raw(self), file, pbhashvalue, cbhashvalue, dwfileflags).ok()
    }
    pub unsafe fn SetExportedTypeProps(&self, ct: u32, tkimplementation: u32, tktypedef: u32, dwexportedtypeflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExportedTypeProps)(::windows::core::Interface::as_raw(self), ct, tkimplementation, tktypedef, dwexportedtypeflags).ok()
    }
    pub unsafe fn SetManifestResourceProps(&self, mr: u32, tkimplementation: u32, dwoffset: u32, dwresourceflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetManifestResourceProps)(::windows::core::Interface::as_raw(self), mr, tkimplementation, dwoffset, dwresourceflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataAssemblyEmit, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataAssemblyEmit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataAssemblyEmit {}
impl ::core::fmt::Debug for IMetaDataAssemblyEmit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataAssemblyEmit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataAssemblyEmit {
    type Vtable = IMetaDataAssemblyEmit_Vtbl;
}
impl ::core::clone::Clone for IMetaDataAssemblyEmit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataAssemblyEmit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x211ef15b_5317_4438_b196_dec87b887693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataAssemblyEmit_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DefineAssembly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpublickey: *const ::core::ffi::c_void, cbpublickey: u32, ulhashalgid: u32, szname: ::windows::core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, dwassemblyflags: u32, pma: *mut u32) -> ::windows::core::HRESULT,
    pub DefineAssemblyRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpublickeyortoken: *const ::core::ffi::c_void, cbpublickeyortoken: u32, szname: ::windows::core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, dwassemblyrefflags: u32, pmdar: *mut u32) -> ::windows::core::HRESULT,
    pub DefineFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, dwfileflags: u32, pmdf: *mut u32) -> ::windows::core::HRESULT,
    pub DefineExportedType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, tkimplementation: u32, tktypedef: u32, dwexportedtypeflags: u32, pmdct: *mut u32) -> ::windows::core::HRESULT,
    pub DefineManifestResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, tkimplementation: u32, dwoffset: u32, dwresourceflags: u32, pmdmr: *mut u32) -> ::windows::core::HRESULT,
    pub SetAssemblyProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pma: u32, pbpublickey: *const ::core::ffi::c_void, cbpublickey: u32, ulhashalgid: u32, szname: ::windows::core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, dwassemblyflags: u32) -> ::windows::core::HRESULT,
    pub SetAssemblyRefProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ar: u32, pbpublickeyortoken: *const ::core::ffi::c_void, cbpublickeyortoken: u32, szname: ::windows::core::PCWSTR, pmetadata: *const ASSEMBLYMETADATA, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, dwassemblyrefflags: u32) -> ::windows::core::HRESULT,
    pub SetFileProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: u32, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, dwfileflags: u32) -> ::windows::core::HRESULT,
    pub SetExportedTypeProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ct: u32, tkimplementation: u32, tktypedef: u32, dwexportedtypeflags: u32) -> ::windows::core::HRESULT,
    pub SetManifestResourceProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mr: u32, tkimplementation: u32, dwoffset: u32, dwresourceflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataAssemblyImport(::windows::core::IUnknown);
impl IMetaDataAssemblyImport {
    pub unsafe fn GetAssemblyProps(&self, mda: u32, ppbpublickey: *const *const ::core::ffi::c_void, pcbpublickey: *mut u32, pulhashalgid: *mut u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32, pmetadata: *mut ASSEMBLYMETADATA, pdwassemblyflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAssemblyProps)(::windows::core::Interface::as_raw(self), mda, ppbpublickey, pcbpublickey, pulhashalgid, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname, pmetadata, pdwassemblyflags).ok()
    }
    pub unsafe fn GetAssemblyRefProps(&self, mdar: u32, ppbpublickeyortoken: *const *const ::core::ffi::c_void, pcbpublickeyortoken: *mut u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32, pmetadata: *mut ASSEMBLYMETADATA, ppbhashvalue: *const *const ::core::ffi::c_void, pcbhashvalue: *mut u32, pdwassemblyrefflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAssemblyRefProps)(::windows::core::Interface::as_raw(self), mdar, ppbpublickeyortoken, pcbpublickeyortoken, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname, pmetadata, ppbhashvalue, pcbhashvalue, pdwassemblyrefflags).ok()
    }
    pub unsafe fn GetFileProps(&self, mdf: u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32, ppbhashvalue: *const *const ::core::ffi::c_void, pcbhashvalue: *mut u32, pdwfileflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFileProps)(::windows::core::Interface::as_raw(self), mdf, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname, ppbhashvalue, pcbhashvalue, pdwfileflags).ok()
    }
    pub unsafe fn GetExportedTypeProps(&self, mdct: u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32, ptkimplementation: *mut u32, ptktypedef: *mut u32, pdwexportedtypeflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetExportedTypeProps)(::windows::core::Interface::as_raw(self), mdct, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname, ptkimplementation, ptktypedef, pdwexportedtypeflags).ok()
    }
    pub unsafe fn GetManifestResourceProps(&self, mdmr: u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32, ptkimplementation: *mut u32, pdwoffset: *mut u32, pdwresourceflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetManifestResourceProps)(::windows::core::Interface::as_raw(self), mdmr, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname, ptkimplementation, pdwoffset, pdwresourceflags).ok()
    }
    pub unsafe fn EnumAssemblyRefs(&self, phenum: *mut *mut ::core::ffi::c_void, rassemblyrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumAssemblyRefs)(::windows::core::Interface::as_raw(self), phenum, rassemblyrefs, cmax, pctokens).ok()
    }
    pub unsafe fn EnumFiles(&self, phenum: *mut *mut ::core::ffi::c_void, rfiles: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumFiles)(::windows::core::Interface::as_raw(self), phenum, rfiles, cmax, pctokens).ok()
    }
    pub unsafe fn EnumExportedTypes(&self, phenum: *mut *mut ::core::ffi::c_void, rexportedtypes: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumExportedTypes)(::windows::core::Interface::as_raw(self), phenum, rexportedtypes, cmax, pctokens).ok()
    }
    pub unsafe fn EnumManifestResources(&self, phenum: *mut *mut ::core::ffi::c_void, rmanifestresources: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumManifestResources)(::windows::core::Interface::as_raw(self), phenum, rmanifestresources, cmax, pctokens).ok()
    }
    pub unsafe fn GetAssemblyFromScope(&self, ptkassembly: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAssemblyFromScope)(::windows::core::Interface::as_raw(self), ptkassembly).ok()
    }
    pub unsafe fn FindExportedTypeByName<P0>(&self, szname: P0, mdtexportedtype: u32, ptkexportedtype: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindExportedTypeByName)(::windows::core::Interface::as_raw(self), szname.into_param().abi(), mdtexportedtype, ptkexportedtype).ok()
    }
    pub unsafe fn FindManifestResourceByName<P0>(&self, szname: P0, ptkmanifestresource: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindManifestResourceByName)(::windows::core::Interface::as_raw(self), szname.into_param().abi(), ptkmanifestresource).ok()
    }
    pub unsafe fn CloseEnum(&self, henum: *mut ::core::ffi::c_void) {
        (::windows::core::Interface::vtable(self).CloseEnum)(::windows::core::Interface::as_raw(self), henum)
    }
    pub unsafe fn FindAssembliesByName<P0, P1, P2>(&self, szappbase: P0, szprivatebin: P1, szassemblyname: P2, ppiunk: *mut ::core::option::Option<::windows::core::IUnknown>, cmax: u32, pcassemblies: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindAssembliesByName)(::windows::core::Interface::as_raw(self), szappbase.into_param().abi(), szprivatebin.into_param().abi(), szassemblyname.into_param().abi(), ::core::mem::transmute(ppiunk), cmax, pcassemblies).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataAssemblyImport, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataAssemblyImport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataAssemblyImport {}
impl ::core::fmt::Debug for IMetaDataAssemblyImport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataAssemblyImport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataAssemblyImport {
    type Vtable = IMetaDataAssemblyImport_Vtbl;
}
impl ::core::clone::Clone for IMetaDataAssemblyImport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataAssemblyImport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee62470b_e94b_424e_9b7c_2f00c9249f93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataAssemblyImport_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAssemblyProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mda: u32, ppbpublickey: *const *const ::core::ffi::c_void, pcbpublickey: *mut u32, pulhashalgid: *mut u32, szname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32, pmetadata: *mut ASSEMBLYMETADATA, pdwassemblyflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetAssemblyRefProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mdar: u32, ppbpublickeyortoken: *const *const ::core::ffi::c_void, pcbpublickeyortoken: *mut u32, szname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32, pmetadata: *mut ASSEMBLYMETADATA, ppbhashvalue: *const *const ::core::ffi::c_void, pcbhashvalue: *mut u32, pdwassemblyrefflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetFileProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mdf: u32, szname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32, ppbhashvalue: *const *const ::core::ffi::c_void, pcbhashvalue: *mut u32, pdwfileflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetExportedTypeProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mdct: u32, szname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32, ptkimplementation: *mut u32, ptktypedef: *mut u32, pdwexportedtypeflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetManifestResourceProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mdmr: u32, szname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32, ptkimplementation: *mut u32, pdwoffset: *mut u32, pdwresourceflags: *mut u32) -> ::windows::core::HRESULT,
    pub EnumAssemblyRefs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rassemblyrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rfiles: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumExportedTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rexportedtypes: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumManifestResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rmanifestresources: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub GetAssemblyFromScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptkassembly: *mut u32) -> ::windows::core::HRESULT,
    pub FindExportedTypeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, mdtexportedtype: u32, ptkexportedtype: *mut u32) -> ::windows::core::HRESULT,
    pub FindManifestResourceByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, ptkmanifestresource: *mut u32) -> ::windows::core::HRESULT,
    pub CloseEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, henum: *mut ::core::ffi::c_void),
    pub FindAssembliesByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szappbase: ::windows::core::PCWSTR, szprivatebin: ::windows::core::PCWSTR, szassemblyname: ::windows::core::PCWSTR, ppiunk: *mut *mut ::core::ffi::c_void, cmax: u32, pcassemblies: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataDispenser(::windows::core::IUnknown);
impl IMetaDataDispenser {
    pub unsafe fn DefineScope(&self, rclsid: *const ::windows::core::GUID, dwcreateflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).DefineScope)(::windows::core::Interface::as_raw(self), rclsid, dwcreateflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenScope<P0>(&self, szscope: P0, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).OpenScope)(::windows::core::Interface::as_raw(self), szscope.into_param().abi(), dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenScopeOnMemory(&self, pdata: *const ::core::ffi::c_void, cbdata: u32, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).OpenScopeOnMemory)(::windows::core::Interface::as_raw(self), pdata, cbdata, dwopenflags, riid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMetaDataDispenser, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataDispenser {}
impl ::core::fmt::Debug for IMetaDataDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataDispenser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataDispenser {
    type Vtable = IMetaDataDispenser_Vtbl;
}
impl ::core::clone::Clone for IMetaDataDispenser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataDispenser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x809c652e_7396_11d2_9771_00a0c9b4d50c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataDispenser_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DefineScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwcreateflags: u32, riid: *const ::windows::core::GUID, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szscope: ::windows::core::PCWSTR, dwopenflags: u32, riid: *const ::windows::core::GUID, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenScopeOnMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, cbdata: u32, dwopenflags: u32, riid: *const ::windows::core::GUID, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataDispenserEx(::windows::core::IUnknown);
impl IMetaDataDispenserEx {
    pub unsafe fn DefineScope(&self, rclsid: *const ::windows::core::GUID, dwcreateflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.DefineScope)(::windows::core::Interface::as_raw(self), rclsid, dwcreateflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenScope<P0>(&self, szscope: P0, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.OpenScope)(::windows::core::Interface::as_raw(self), szscope.into_param().abi(), dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenScopeOnMemory(&self, pdata: *const ::core::ffi::c_void, cbdata: u32, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.OpenScopeOnMemory)(::windows::core::Interface::as_raw(self), pdata, cbdata, dwopenflags, riid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetOption(&self, optionid: *const ::windows::core::GUID, value: *const super::super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOption)(::windows::core::Interface::as_raw(self), optionid, value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetOption(&self, optionid: *const ::windows::core::GUID, pvalue: *mut super::super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOption)(::windows::core::Interface::as_raw(self), optionid, pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenScopeOnITypeInfo<P0>(&self, piti: P0, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<super::super::Com::ITypeInfo>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).OpenScopeOnITypeInfo)(::windows::core::Interface::as_raw(self), piti.into_param().abi(), dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCORSystemDirectory(&self, szbuffer: ::core::option::Option<&mut [u16]>, pchbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCORSystemDirectory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(szbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szbuffer.as_deref().map_or(0, |slice| slice.len() as _), pchbuffer).ok()
    }
    pub unsafe fn FindAssembly<P0, P1, P2, P3, P4>(&self, szappbase: P0, szprivatebin: P1, szglobalbin: P2, szassemblyname: P3, szname: P4, cchname: u32, pcname: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindAssembly)(::windows::core::Interface::as_raw(self), szappbase.into_param().abi(), szprivatebin.into_param().abi(), szglobalbin.into_param().abi(), szassemblyname.into_param().abi(), szname.into_param().abi(), cchname, pcname).ok()
    }
    pub unsafe fn FindAssemblyModule<P0, P1, P2, P3, P4>(&self, szappbase: P0, szprivatebin: P1, szglobalbin: P2, szassemblyname: P3, szmodulename: P4, szname: ::core::option::Option<&mut [u16]>, pcname: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindAssemblyModule)(::windows::core::Interface::as_raw(self), szappbase.into_param().abi(), szprivatebin.into_param().abi(), szglobalbin.into_param().abi(), szassemblyname.into_param().abi(), szmodulename.into_param().abi(), ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pcname).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataDispenserEx, ::windows::core::IUnknown, IMetaDataDispenser);
impl ::core::cmp::PartialEq for IMetaDataDispenserEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataDispenserEx {}
impl ::core::fmt::Debug for IMetaDataDispenserEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataDispenserEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataDispenserEx {
    type Vtable = IMetaDataDispenserEx_Vtbl;
}
impl ::core::clone::Clone for IMetaDataDispenserEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataDispenserEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31bcfce2_dafb_11d2_9f81_00c04f79a0a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataDispenserEx_Vtbl {
    pub base__: IMetaDataDispenser_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionid: *const ::windows::core::GUID, value: *const super::super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetOption: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionid: *const ::windows::core::GUID, pvalue: *mut super::super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetOption: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenScopeOnITypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piti: *mut ::core::ffi::c_void, dwopenflags: u32, riid: *const ::windows::core::GUID, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenScopeOnITypeInfo: usize,
    pub GetCORSystemDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szbuffer: ::windows::core::PWSTR, cchbuffer: u32, pchbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub FindAssembly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szappbase: ::windows::core::PCWSTR, szprivatebin: ::windows::core::PCWSTR, szglobalbin: ::windows::core::PCWSTR, szassemblyname: ::windows::core::PCWSTR, szname: ::windows::core::PCWSTR, cchname: u32, pcname: *mut u32) -> ::windows::core::HRESULT,
    pub FindAssemblyModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szappbase: ::windows::core::PCWSTR, szprivatebin: ::windows::core::PCWSTR, szglobalbin: ::windows::core::PCWSTR, szassemblyname: ::windows::core::PCWSTR, szmodulename: ::windows::core::PCWSTR, szname: ::windows::core::PWSTR, cchname: u32, pcname: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataEmit(::windows::core::IUnknown);
impl IMetaDataEmit {
    pub unsafe fn SetModuleProps<P0>(&self, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetModuleProps)(::windows::core::Interface::as_raw(self), szname.into_param().abi()).ok()
    }
    pub unsafe fn Save<P0>(&self, szfile: P0, dwsaveflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Save)(::windows::core::Interface::as_raw(self), szfile.into_param().abi(), dwsaveflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveToStream<P0>(&self, pistream: P0, dwsaveflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).SaveToStream)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), dwsaveflags).ok()
    }
    pub unsafe fn GetSaveSize(&self, fsave: CorSaveSize, pdwsavesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSaveSize)(::windows::core::Interface::as_raw(self), fsave, pdwsavesize).ok()
    }
    pub unsafe fn DefineTypeDef<P0>(&self, sztypedef: P0, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, ptd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineTypeDef)(::windows::core::Interface::as_raw(self), sztypedef.into_param().abi(), dwtypedefflags, tkextends, rtkimplements, ptd).ok()
    }
    pub unsafe fn DefineNestedType<P0>(&self, sztypedef: P0, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, tdencloser: u32, ptd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineNestedType)(::windows::core::Interface::as_raw(self), sztypedef.into_param().abi(), dwtypedefflags, tkextends, rtkimplements, tdencloser, ptd).ok()
    }
    pub unsafe fn SetHandler<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetHandler)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn DefineMethod<P0>(&self, td: u32, szname: P0, dwmethodflags: u32, pvsigblob: *mut u8, cbsigblob: u32, ulcoderva: u32, dwimplflags: u32, pmd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineMethod)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), dwmethodflags, pvsigblob, cbsigblob, ulcoderva, dwimplflags, pmd).ok()
    }
    pub unsafe fn DefineMethodImpl(&self, td: u32, tkbody: u32, tkdecl: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DefineMethodImpl)(::windows::core::Interface::as_raw(self), td, tkbody, tkdecl).ok()
    }
    pub unsafe fn DefineTypeRefByName<P0>(&self, tkresolutionscope: u32, szname: P0, ptr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineTypeRefByName)(::windows::core::Interface::as_raw(self), tkresolutionscope, szname.into_param().abi(), ptr).ok()
    }
    pub unsafe fn DefineImportType<P0, P1, P2>(&self, passemimport: P0, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, pimport: P1, tdimport: u32, passememit: P2, ptr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMetaDataAssemblyImport>,
        P1: ::windows::core::IntoParam<IMetaDataImport>,
        P2: ::windows::core::IntoParam<IMetaDataAssemblyEmit>,
    {
        (::windows::core::Interface::vtable(self).DefineImportType)(::windows::core::Interface::as_raw(self), passemimport.into_param().abi(), pbhashvalue, cbhashvalue, pimport.into_param().abi(), tdimport, passememit.into_param().abi(), ptr).ok()
    }
    pub unsafe fn DefineMemberRef<P0>(&self, tkimport: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineMemberRef)(::windows::core::Interface::as_raw(self), tkimport, szname.into_param().abi(), pvsigblob, cbsigblob, pmr).ok()
    }
    pub unsafe fn DefineImportMember<P0, P1, P2>(&self, passemimport: P0, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, pimport: P1, mbmember: u32, passememit: P2, tkparent: u32, pmr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMetaDataAssemblyImport>,
        P1: ::windows::core::IntoParam<IMetaDataImport>,
        P2: ::windows::core::IntoParam<IMetaDataAssemblyEmit>,
    {
        (::windows::core::Interface::vtable(self).DefineImportMember)(::windows::core::Interface::as_raw(self), passemimport.into_param().abi(), pbhashvalue, cbhashvalue, pimport.into_param().abi(), mbmember, passememit.into_param().abi(), tkparent, pmr).ok()
    }
    pub unsafe fn DefineEvent<P0>(&self, td: u32, szevent: P0, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32, pmdevent: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineEvent)(::windows::core::Interface::as_raw(self), td, szevent.into_param().abi(), dweventflags, tkeventtype, mdaddon, mdremoveon, mdfire, rmdothermethods, pmdevent).ok()
    }
    pub unsafe fn SetClassLayout(&self, td: u32, dwpacksize: u32, rfieldoffsets: *mut COR_FIELD_OFFSET, ulclasssize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClassLayout)(::windows::core::Interface::as_raw(self), td, dwpacksize, rfieldoffsets, ulclasssize).ok()
    }
    pub unsafe fn DeleteClassLayout(&self, td: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteClassLayout)(::windows::core::Interface::as_raw(self), td).ok()
    }
    pub unsafe fn SetFieldMarshal(&self, tk: u32, pvnativetype: *mut u8, cbnativetype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFieldMarshal)(::windows::core::Interface::as_raw(self), tk, pvnativetype, cbnativetype).ok()
    }
    pub unsafe fn DeleteFieldMarshal(&self, tk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteFieldMarshal)(::windows::core::Interface::as_raw(self), tk).ok()
    }
    pub unsafe fn DefinePermissionSet(&self, tk: u32, dwaction: u32, pvpermission: *const ::core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DefinePermissionSet)(::windows::core::Interface::as_raw(self), tk, dwaction, pvpermission, cbpermission, ppm).ok()
    }
    pub unsafe fn SetRVA(&self, md: u32, ulrva: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRVA)(::windows::core::Interface::as_raw(self), md, ulrva).ok()
    }
    pub unsafe fn GetTokenFromSig(&self, pvsig: *mut u8, cbsig: u32, pmsig: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTokenFromSig)(::windows::core::Interface::as_raw(self), pvsig, cbsig, pmsig).ok()
    }
    pub unsafe fn DefineModuleRef<P0>(&self, szname: P0, pmur: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineModuleRef)(::windows::core::Interface::as_raw(self), szname.into_param().abi(), pmur).ok()
    }
    pub unsafe fn SetParent(&self, mr: u32, tk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParent)(::windows::core::Interface::as_raw(self), mr, tk).ok()
    }
    pub unsafe fn GetTokenFromTypeSpec(&self, pvsig: *mut u8, cbsig: u32, ptypespec: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTokenFromTypeSpec)(::windows::core::Interface::as_raw(self), pvsig, cbsig, ptypespec).ok()
    }
    pub unsafe fn SaveToMemory(&self, pbdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveToMemory)(::windows::core::Interface::as_raw(self), pbdata, cbdata).ok()
    }
    pub unsafe fn DefineUserString<P0>(&self, szstring: P0, cchstring: u32, pstk: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineUserString)(::windows::core::Interface::as_raw(self), szstring.into_param().abi(), cchstring, pstk).ok()
    }
    pub unsafe fn DeleteToken(&self, tkobj: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteToken)(::windows::core::Interface::as_raw(self), tkobj).ok()
    }
    pub unsafe fn SetMethodProps(&self, md: u32, dwmethodflags: u32, ulcoderva: u32, dwimplflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMethodProps)(::windows::core::Interface::as_raw(self), md, dwmethodflags, ulcoderva, dwimplflags).ok()
    }
    pub unsafe fn SetTypeDefProps(&self, td: u32, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTypeDefProps)(::windows::core::Interface::as_raw(self), td, dwtypedefflags, tkextends, rtkimplements).ok()
    }
    pub unsafe fn SetEventProps(&self, ev: u32, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventProps)(::windows::core::Interface::as_raw(self), ev, dweventflags, tkeventtype, mdaddon, mdremoveon, mdfire, rmdothermethods).ok()
    }
    pub unsafe fn SetPermissionSetProps(&self, tk: u32, dwaction: u32, pvpermission: *const ::core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPermissionSetProps)(::windows::core::Interface::as_raw(self), tk, dwaction, pvpermission, cbpermission, ppm).ok()
    }
    pub unsafe fn DefinePinvokeMap<P0>(&self, tk: u32, dwmappingflags: u32, szimportname: P0, mrimportdll: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefinePinvokeMap)(::windows::core::Interface::as_raw(self), tk, dwmappingflags, szimportname.into_param().abi(), mrimportdll).ok()
    }
    pub unsafe fn SetPinvokeMap<P0>(&self, tk: u32, dwmappingflags: u32, szimportname: P0, mrimportdll: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPinvokeMap)(::windows::core::Interface::as_raw(self), tk, dwmappingflags, szimportname.into_param().abi(), mrimportdll).ok()
    }
    pub unsafe fn DeletePinvokeMap(&self, tk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePinvokeMap)(::windows::core::Interface::as_raw(self), tk).ok()
    }
    pub unsafe fn DefineCustomAttribute(&self, tkowner: u32, tkctor: u32, pcustomattribute: *const ::core::ffi::c_void, cbcustomattribute: u32, pcv: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DefineCustomAttribute)(::windows::core::Interface::as_raw(self), tkowner, tkctor, pcustomattribute, cbcustomattribute, pcv).ok()
    }
    pub unsafe fn SetCustomAttributeValue(&self, pcv: u32, pcustomattribute: *const ::core::ffi::c_void, cbcustomattribute: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCustomAttributeValue)(::windows::core::Interface::as_raw(self), pcv, pcustomattribute, cbcustomattribute).ok()
    }
    pub unsafe fn DefineField<P0>(&self, td: u32, szname: P0, dwfieldflags: u32, pvsigblob: *mut u8, cbsigblob: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, pmd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineField)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), dwfieldflags, pvsigblob, cbsigblob, dwcplustypeflag, pvalue, cchvalue, pmd).ok()
    }
    pub unsafe fn DefineProperty<P0>(&self, td: u32, szproperty: P0, dwpropflags: u32, pvsig: *mut u8, cbsig: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32, pmdprop: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineProperty)(::windows::core::Interface::as_raw(self), td, szproperty.into_param().abi(), dwpropflags, pvsig, cbsig, dwcplustypeflag, pvalue, cchvalue, mdsetter, mdgetter, rmdothermethods, pmdprop).ok()
    }
    pub unsafe fn DefineParam<P0>(&self, md: u32, ulparamseq: u32, szname: P0, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, ppd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineParam)(::windows::core::Interface::as_raw(self), md, ulparamseq, szname.into_param().abi(), dwparamflags, dwcplustypeflag, pvalue, cchvalue, ppd).ok()
    }
    pub unsafe fn SetFieldProps(&self, fd: u32, dwfieldflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFieldProps)(::windows::core::Interface::as_raw(self), fd, dwfieldflags, dwcplustypeflag, pvalue, cchvalue).ok()
    }
    pub unsafe fn SetPropertyProps(&self, pr: u32, dwpropflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPropertyProps)(::windows::core::Interface::as_raw(self), pr, dwpropflags, dwcplustypeflag, pvalue, cchvalue, mdsetter, mdgetter, rmdothermethods).ok()
    }
    pub unsafe fn SetParamProps<P0>(&self, pd: u32, szname: P0, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetParamProps)(::windows::core::Interface::as_raw(self), pd, szname.into_param().abi(), dwparamflags, dwcplustypeflag, pvalue, cchvalue).ok()
    }
    pub unsafe fn DefineSecurityAttributeSet(&self, tkobj: u32, rsecattrs: *mut COR_SECATTR, csecattrs: u32, pulerrorattr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DefineSecurityAttributeSet)(::windows::core::Interface::as_raw(self), tkobj, rsecattrs, csecattrs, pulerrorattr).ok()
    }
    pub unsafe fn ApplyEditAndContinue<P0>(&self, pimport: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).ApplyEditAndContinue)(::windows::core::Interface::as_raw(self), pimport.into_param().abi()).ok()
    }
    pub unsafe fn TranslateSigWithScope<P0, P1, P2, P3>(&self, passemimport: P0, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, import: P1, pbsigblob: *mut u8, cbsigblob: u32, passememit: P2, emit: P3, pvtranslatedsig: *mut u8, cbtranslatedsigmax: u32, pcbtranslatedsig: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMetaDataAssemblyImport>,
        P1: ::windows::core::IntoParam<IMetaDataImport>,
        P2: ::windows::core::IntoParam<IMetaDataAssemblyEmit>,
        P3: ::windows::core::IntoParam<IMetaDataEmit>,
    {
        (::windows::core::Interface::vtable(self).TranslateSigWithScope)(::windows::core::Interface::as_raw(self), passemimport.into_param().abi(), pbhashvalue, cbhashvalue, import.into_param().abi(), pbsigblob, cbsigblob, passememit.into_param().abi(), emit.into_param().abi(), pvtranslatedsig, cbtranslatedsigmax, pcbtranslatedsig).ok()
    }
    pub unsafe fn SetMethodImplFlags(&self, md: u32, dwimplflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMethodImplFlags)(::windows::core::Interface::as_raw(self), md, dwimplflags).ok()
    }
    pub unsafe fn SetFieldRVA(&self, fd: u32, ulrva: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFieldRVA)(::windows::core::Interface::as_raw(self), fd, ulrva).ok()
    }
    pub unsafe fn Merge<P0, P1, P2>(&self, pimport: P0, phostmaptoken: P1, phandler: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMetaDataImport>,
        P1: ::windows::core::IntoParam<IMapToken>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Merge)(::windows::core::Interface::as_raw(self), pimport.into_param().abi(), phostmaptoken.into_param().abi(), phandler.into_param().abi()).ok()
    }
    pub unsafe fn MergeEnd(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MergeEnd)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataEmit, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataEmit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataEmit {}
impl ::core::fmt::Debug for IMetaDataEmit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataEmit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataEmit {
    type Vtable = IMetaDataEmit_Vtbl;
}
impl ::core::clone::Clone for IMetaDataEmit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataEmit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba3fee4c_ecb9_4e41_83b7_183fa41cd859);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataEmit_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetModuleProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szfile: ::windows::core::PCWSTR, dwsaveflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, dwsaveflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveToStream: usize,
    pub GetSaveSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsave: CorSaveSize, pdwsavesize: *mut u32) -> ::windows::core::HRESULT,
    pub DefineTypeDef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztypedef: ::windows::core::PCWSTR, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, ptd: *mut u32) -> ::windows::core::HRESULT,
    pub DefineNestedType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztypedef: ::windows::core::PCWSTR, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, tdencloser: u32, ptd: *mut u32) -> ::windows::core::HRESULT,
    pub SetHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DefineMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, szname: ::windows::core::PCWSTR, dwmethodflags: u32, pvsigblob: *mut u8, cbsigblob: u32, ulcoderva: u32, dwimplflags: u32, pmd: *mut u32) -> ::windows::core::HRESULT,
    pub DefineMethodImpl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, tkbody: u32, tkdecl: u32) -> ::windows::core::HRESULT,
    pub DefineTypeRefByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tkresolutionscope: u32, szname: ::windows::core::PCWSTR, ptr: *mut u32) -> ::windows::core::HRESULT,
    pub DefineImportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passemimport: *mut ::core::ffi::c_void, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, pimport: *mut ::core::ffi::c_void, tdimport: u32, passememit: *mut ::core::ffi::c_void, ptr: *mut u32) -> ::windows::core::HRESULT,
    pub DefineMemberRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tkimport: u32, szname: ::windows::core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> ::windows::core::HRESULT,
    pub DefineImportMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passemimport: *mut ::core::ffi::c_void, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, pimport: *mut ::core::ffi::c_void, mbmember: u32, passememit: *mut ::core::ffi::c_void, tkparent: u32, pmr: *mut u32) -> ::windows::core::HRESULT,
    pub DefineEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, szevent: ::windows::core::PCWSTR, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32, pmdevent: *mut u32) -> ::windows::core::HRESULT,
    pub SetClassLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, dwpacksize: u32, rfieldoffsets: *mut COR_FIELD_OFFSET, ulclasssize: u32) -> ::windows::core::HRESULT,
    pub DeleteClassLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32) -> ::windows::core::HRESULT,
    pub SetFieldMarshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, pvnativetype: *mut u8, cbnativetype: u32) -> ::windows::core::HRESULT,
    pub DeleteFieldMarshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32) -> ::windows::core::HRESULT,
    pub DefinePermissionSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, dwaction: u32, pvpermission: *const ::core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> ::windows::core::HRESULT,
    pub SetRVA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, md: u32, ulrva: u32) -> ::windows::core::HRESULT,
    pub GetTokenFromSig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvsig: *mut u8, cbsig: u32, pmsig: *mut u32) -> ::windows::core::HRESULT,
    pub DefineModuleRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, pmur: *mut u32) -> ::windows::core::HRESULT,
    pub SetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mr: u32, tk: u32) -> ::windows::core::HRESULT,
    pub GetTokenFromTypeSpec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvsig: *mut u8, cbsig: u32, ptypespec: *mut u32) -> ::windows::core::HRESULT,
    pub SaveToMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::HRESULT,
    pub DefineUserString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szstring: ::windows::core::PCWSTR, cchstring: u32, pstk: *mut u32) -> ::windows::core::HRESULT,
    pub DeleteToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tkobj: u32) -> ::windows::core::HRESULT,
    pub SetMethodProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, md: u32, dwmethodflags: u32, ulcoderva: u32, dwimplflags: u32) -> ::windows::core::HRESULT,
    pub SetTypeDefProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32) -> ::windows::core::HRESULT,
    pub SetEventProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ev: u32, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32) -> ::windows::core::HRESULT,
    pub SetPermissionSetProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, dwaction: u32, pvpermission: *const ::core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> ::windows::core::HRESULT,
    pub DefinePinvokeMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, dwmappingflags: u32, szimportname: ::windows::core::PCWSTR, mrimportdll: u32) -> ::windows::core::HRESULT,
    pub SetPinvokeMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, dwmappingflags: u32, szimportname: ::windows::core::PCWSTR, mrimportdll: u32) -> ::windows::core::HRESULT,
    pub DeletePinvokeMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32) -> ::windows::core::HRESULT,
    pub DefineCustomAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tkowner: u32, tkctor: u32, pcustomattribute: *const ::core::ffi::c_void, cbcustomattribute: u32, pcv: *mut u32) -> ::windows::core::HRESULT,
    pub SetCustomAttributeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcv: u32, pcustomattribute: *const ::core::ffi::c_void, cbcustomattribute: u32) -> ::windows::core::HRESULT,
    pub DefineField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, szname: ::windows::core::PCWSTR, dwfieldflags: u32, pvsigblob: *mut u8, cbsigblob: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, pmd: *mut u32) -> ::windows::core::HRESULT,
    pub DefineProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, szproperty: ::windows::core::PCWSTR, dwpropflags: u32, pvsig: *mut u8, cbsig: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32, pmdprop: *mut u32) -> ::windows::core::HRESULT,
    pub DefineParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, md: u32, ulparamseq: u32, szname: ::windows::core::PCWSTR, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, ppd: *mut u32) -> ::windows::core::HRESULT,
    pub SetFieldProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fd: u32, dwfieldflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32) -> ::windows::core::HRESULT,
    pub SetPropertyProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pr: u32, dwpropflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32) -> ::windows::core::HRESULT,
    pub SetParamProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd: u32, szname: ::windows::core::PCWSTR, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32) -> ::windows::core::HRESULT,
    pub DefineSecurityAttributeSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tkobj: u32, rsecattrs: *mut COR_SECATTR, csecattrs: u32, pulerrorattr: *mut u32) -> ::windows::core::HRESULT,
    pub ApplyEditAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TranslateSigWithScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passemimport: *mut ::core::ffi::c_void, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, import: *mut ::core::ffi::c_void, pbsigblob: *mut u8, cbsigblob: u32, passememit: *mut ::core::ffi::c_void, emit: *mut ::core::ffi::c_void, pvtranslatedsig: *mut u8, cbtranslatedsigmax: u32, pcbtranslatedsig: *mut u32) -> ::windows::core::HRESULT,
    pub SetMethodImplFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, md: u32, dwimplflags: u32) -> ::windows::core::HRESULT,
    pub SetFieldRVA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fd: u32, ulrva: u32) -> ::windows::core::HRESULT,
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimport: *mut ::core::ffi::c_void, phostmaptoken: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MergeEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataEmit2(::windows::core::IUnknown);
impl IMetaDataEmit2 {
    pub unsafe fn SetModuleProps<P0>(&self, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetModuleProps)(::windows::core::Interface::as_raw(self), szname.into_param().abi()).ok()
    }
    pub unsafe fn Save<P0>(&self, szfile: P0, dwsaveflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Save)(::windows::core::Interface::as_raw(self), szfile.into_param().abi(), dwsaveflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveToStream<P0>(&self, pistream: P0, dwsaveflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.SaveToStream)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), dwsaveflags).ok()
    }
    pub unsafe fn GetSaveSize(&self, fsave: CorSaveSize, pdwsavesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSaveSize)(::windows::core::Interface::as_raw(self), fsave, pdwsavesize).ok()
    }
    pub unsafe fn DefineTypeDef<P0>(&self, sztypedef: P0, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, ptd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineTypeDef)(::windows::core::Interface::as_raw(self), sztypedef.into_param().abi(), dwtypedefflags, tkextends, rtkimplements, ptd).ok()
    }
    pub unsafe fn DefineNestedType<P0>(&self, sztypedef: P0, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, tdencloser: u32, ptd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineNestedType)(::windows::core::Interface::as_raw(self), sztypedef.into_param().abi(), dwtypedefflags, tkextends, rtkimplements, tdencloser, ptd).ok()
    }
    pub unsafe fn SetHandler<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetHandler)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn DefineMethod<P0>(&self, td: u32, szname: P0, dwmethodflags: u32, pvsigblob: *mut u8, cbsigblob: u32, ulcoderva: u32, dwimplflags: u32, pmd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineMethod)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), dwmethodflags, pvsigblob, cbsigblob, ulcoderva, dwimplflags, pmd).ok()
    }
    pub unsafe fn DefineMethodImpl(&self, td: u32, tkbody: u32, tkdecl: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DefineMethodImpl)(::windows::core::Interface::as_raw(self), td, tkbody, tkdecl).ok()
    }
    pub unsafe fn DefineTypeRefByName<P0>(&self, tkresolutionscope: u32, szname: P0, ptr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineTypeRefByName)(::windows::core::Interface::as_raw(self), tkresolutionscope, szname.into_param().abi(), ptr).ok()
    }
    pub unsafe fn DefineImportType<P0, P1, P2>(&self, passemimport: P0, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, pimport: P1, tdimport: u32, passememit: P2, ptr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMetaDataAssemblyImport>,
        P1: ::windows::core::IntoParam<IMetaDataImport>,
        P2: ::windows::core::IntoParam<IMetaDataAssemblyEmit>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineImportType)(::windows::core::Interface::as_raw(self), passemimport.into_param().abi(), pbhashvalue, cbhashvalue, pimport.into_param().abi(), tdimport, passememit.into_param().abi(), ptr).ok()
    }
    pub unsafe fn DefineMemberRef<P0>(&self, tkimport: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineMemberRef)(::windows::core::Interface::as_raw(self), tkimport, szname.into_param().abi(), pvsigblob, cbsigblob, pmr).ok()
    }
    pub unsafe fn DefineImportMember<P0, P1, P2>(&self, passemimport: P0, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, pimport: P1, mbmember: u32, passememit: P2, tkparent: u32, pmr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMetaDataAssemblyImport>,
        P1: ::windows::core::IntoParam<IMetaDataImport>,
        P2: ::windows::core::IntoParam<IMetaDataAssemblyEmit>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineImportMember)(::windows::core::Interface::as_raw(self), passemimport.into_param().abi(), pbhashvalue, cbhashvalue, pimport.into_param().abi(), mbmember, passememit.into_param().abi(), tkparent, pmr).ok()
    }
    pub unsafe fn DefineEvent<P0>(&self, td: u32, szevent: P0, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32, pmdevent: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineEvent)(::windows::core::Interface::as_raw(self), td, szevent.into_param().abi(), dweventflags, tkeventtype, mdaddon, mdremoveon, mdfire, rmdothermethods, pmdevent).ok()
    }
    pub unsafe fn SetClassLayout(&self, td: u32, dwpacksize: u32, rfieldoffsets: *mut COR_FIELD_OFFSET, ulclasssize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetClassLayout)(::windows::core::Interface::as_raw(self), td, dwpacksize, rfieldoffsets, ulclasssize).ok()
    }
    pub unsafe fn DeleteClassLayout(&self, td: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteClassLayout)(::windows::core::Interface::as_raw(self), td).ok()
    }
    pub unsafe fn SetFieldMarshal(&self, tk: u32, pvnativetype: *mut u8, cbnativetype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFieldMarshal)(::windows::core::Interface::as_raw(self), tk, pvnativetype, cbnativetype).ok()
    }
    pub unsafe fn DeleteFieldMarshal(&self, tk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteFieldMarshal)(::windows::core::Interface::as_raw(self), tk).ok()
    }
    pub unsafe fn DefinePermissionSet(&self, tk: u32, dwaction: u32, pvpermission: *const ::core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DefinePermissionSet)(::windows::core::Interface::as_raw(self), tk, dwaction, pvpermission, cbpermission, ppm).ok()
    }
    pub unsafe fn SetRVA(&self, md: u32, ulrva: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRVA)(::windows::core::Interface::as_raw(self), md, ulrva).ok()
    }
    pub unsafe fn GetTokenFromSig(&self, pvsig: *mut u8, cbsig: u32, pmsig: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTokenFromSig)(::windows::core::Interface::as_raw(self), pvsig, cbsig, pmsig).ok()
    }
    pub unsafe fn DefineModuleRef<P0>(&self, szname: P0, pmur: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineModuleRef)(::windows::core::Interface::as_raw(self), szname.into_param().abi(), pmur).ok()
    }
    pub unsafe fn SetParent(&self, mr: u32, tk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetParent)(::windows::core::Interface::as_raw(self), mr, tk).ok()
    }
    pub unsafe fn GetTokenFromTypeSpec(&self, pvsig: *mut u8, cbsig: u32, ptypespec: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTokenFromTypeSpec)(::windows::core::Interface::as_raw(self), pvsig, cbsig, ptypespec).ok()
    }
    pub unsafe fn SaveToMemory(&self, pbdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveToMemory)(::windows::core::Interface::as_raw(self), pbdata, cbdata).ok()
    }
    pub unsafe fn DefineUserString<P0>(&self, szstring: P0, cchstring: u32, pstk: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineUserString)(::windows::core::Interface::as_raw(self), szstring.into_param().abi(), cchstring, pstk).ok()
    }
    pub unsafe fn DeleteToken(&self, tkobj: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteToken)(::windows::core::Interface::as_raw(self), tkobj).ok()
    }
    pub unsafe fn SetMethodProps(&self, md: u32, dwmethodflags: u32, ulcoderva: u32, dwimplflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMethodProps)(::windows::core::Interface::as_raw(self), md, dwmethodflags, ulcoderva, dwimplflags).ok()
    }
    pub unsafe fn SetTypeDefProps(&self, td: u32, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetTypeDefProps)(::windows::core::Interface::as_raw(self), td, dwtypedefflags, tkextends, rtkimplements).ok()
    }
    pub unsafe fn SetEventProps(&self, ev: u32, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEventProps)(::windows::core::Interface::as_raw(self), ev, dweventflags, tkeventtype, mdaddon, mdremoveon, mdfire, rmdothermethods).ok()
    }
    pub unsafe fn SetPermissionSetProps(&self, tk: u32, dwaction: u32, pvpermission: *const ::core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPermissionSetProps)(::windows::core::Interface::as_raw(self), tk, dwaction, pvpermission, cbpermission, ppm).ok()
    }
    pub unsafe fn DefinePinvokeMap<P0>(&self, tk: u32, dwmappingflags: u32, szimportname: P0, mrimportdll: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefinePinvokeMap)(::windows::core::Interface::as_raw(self), tk, dwmappingflags, szimportname.into_param().abi(), mrimportdll).ok()
    }
    pub unsafe fn SetPinvokeMap<P0>(&self, tk: u32, dwmappingflags: u32, szimportname: P0, mrimportdll: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPinvokeMap)(::windows::core::Interface::as_raw(self), tk, dwmappingflags, szimportname.into_param().abi(), mrimportdll).ok()
    }
    pub unsafe fn DeletePinvokeMap(&self, tk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeletePinvokeMap)(::windows::core::Interface::as_raw(self), tk).ok()
    }
    pub unsafe fn DefineCustomAttribute(&self, tkowner: u32, tkctor: u32, pcustomattribute: *const ::core::ffi::c_void, cbcustomattribute: u32, pcv: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DefineCustomAttribute)(::windows::core::Interface::as_raw(self), tkowner, tkctor, pcustomattribute, cbcustomattribute, pcv).ok()
    }
    pub unsafe fn SetCustomAttributeValue(&self, pcv: u32, pcustomattribute: *const ::core::ffi::c_void, cbcustomattribute: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetCustomAttributeValue)(::windows::core::Interface::as_raw(self), pcv, pcustomattribute, cbcustomattribute).ok()
    }
    pub unsafe fn DefineField<P0>(&self, td: u32, szname: P0, dwfieldflags: u32, pvsigblob: *mut u8, cbsigblob: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, pmd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineField)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), dwfieldflags, pvsigblob, cbsigblob, dwcplustypeflag, pvalue, cchvalue, pmd).ok()
    }
    pub unsafe fn DefineProperty<P0>(&self, td: u32, szproperty: P0, dwpropflags: u32, pvsig: *mut u8, cbsig: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32, pmdprop: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineProperty)(::windows::core::Interface::as_raw(self), td, szproperty.into_param().abi(), dwpropflags, pvsig, cbsig, dwcplustypeflag, pvalue, cchvalue, mdsetter, mdgetter, rmdothermethods, pmdprop).ok()
    }
    pub unsafe fn DefineParam<P0>(&self, md: u32, ulparamseq: u32, szname: P0, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, ppd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DefineParam)(::windows::core::Interface::as_raw(self), md, ulparamseq, szname.into_param().abi(), dwparamflags, dwcplustypeflag, pvalue, cchvalue, ppd).ok()
    }
    pub unsafe fn SetFieldProps(&self, fd: u32, dwfieldflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFieldProps)(::windows::core::Interface::as_raw(self), fd, dwfieldflags, dwcplustypeflag, pvalue, cchvalue).ok()
    }
    pub unsafe fn SetPropertyProps(&self, pr: u32, dwpropflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPropertyProps)(::windows::core::Interface::as_raw(self), pr, dwpropflags, dwcplustypeflag, pvalue, cchvalue, mdsetter, mdgetter, rmdothermethods).ok()
    }
    pub unsafe fn SetParamProps<P0>(&self, pd: u32, szname: P0, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const ::core::ffi::c_void, cchvalue: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetParamProps)(::windows::core::Interface::as_raw(self), pd, szname.into_param().abi(), dwparamflags, dwcplustypeflag, pvalue, cchvalue).ok()
    }
    pub unsafe fn DefineSecurityAttributeSet(&self, tkobj: u32, rsecattrs: *mut COR_SECATTR, csecattrs: u32, pulerrorattr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DefineSecurityAttributeSet)(::windows::core::Interface::as_raw(self), tkobj, rsecattrs, csecattrs, pulerrorattr).ok()
    }
    pub unsafe fn ApplyEditAndContinue<P0>(&self, pimport: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.ApplyEditAndContinue)(::windows::core::Interface::as_raw(self), pimport.into_param().abi()).ok()
    }
    pub unsafe fn TranslateSigWithScope<P0, P1, P2, P3>(&self, passemimport: P0, pbhashvalue: *const ::core::ffi::c_void, cbhashvalue: u32, import: P1, pbsigblob: *mut u8, cbsigblob: u32, passememit: P2, emit: P3, pvtranslatedsig: *mut u8, cbtranslatedsigmax: u32, pcbtranslatedsig: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMetaDataAssemblyImport>,
        P1: ::windows::core::IntoParam<IMetaDataImport>,
        P2: ::windows::core::IntoParam<IMetaDataAssemblyEmit>,
        P3: ::windows::core::IntoParam<IMetaDataEmit>,
    {
        (::windows::core::Interface::vtable(self).base__.TranslateSigWithScope)(::windows::core::Interface::as_raw(self), passemimport.into_param().abi(), pbhashvalue, cbhashvalue, import.into_param().abi(), pbsigblob, cbsigblob, passememit.into_param().abi(), emit.into_param().abi(), pvtranslatedsig, cbtranslatedsigmax, pcbtranslatedsig).ok()
    }
    pub unsafe fn SetMethodImplFlags(&self, md: u32, dwimplflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMethodImplFlags)(::windows::core::Interface::as_raw(self), md, dwimplflags).ok()
    }
    pub unsafe fn SetFieldRVA(&self, fd: u32, ulrva: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFieldRVA)(::windows::core::Interface::as_raw(self), fd, ulrva).ok()
    }
    pub unsafe fn Merge<P0, P1, P2>(&self, pimport: P0, phostmaptoken: P1, phandler: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMetaDataImport>,
        P1: ::windows::core::IntoParam<IMapToken>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.Merge)(::windows::core::Interface::as_raw(self), pimport.into_param().abi(), phostmaptoken.into_param().abi(), phandler.into_param().abi()).ok()
    }
    pub unsafe fn MergeEnd(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.MergeEnd)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DefineMethodSpec(&self, tkparent: u32, pvsigblob: *mut u8, cbsigblob: u32, pmi: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DefineMethodSpec)(::windows::core::Interface::as_raw(self), tkparent, pvsigblob, cbsigblob, pmi).ok()
    }
    pub unsafe fn GetDeltaSaveSize(&self, fsave: CorSaveSize, pdwsavesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeltaSaveSize)(::windows::core::Interface::as_raw(self), fsave, pdwsavesize).ok()
    }
    pub unsafe fn SaveDelta<P0>(&self, szfile: P0, dwsaveflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SaveDelta)(::windows::core::Interface::as_raw(self), szfile.into_param().abi(), dwsaveflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveDeltaToStream<P0>(&self, pistream: P0, dwsaveflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).SaveDeltaToStream)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), dwsaveflags).ok()
    }
    pub unsafe fn SaveDeltaToMemory(&self, pbdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveDeltaToMemory)(::windows::core::Interface::as_raw(self), pbdata, cbdata).ok()
    }
    pub unsafe fn DefineGenericParam<P0>(&self, tk: u32, ulparamseq: u32, dwparamflags: u32, szname: P0, reserved: u32, rtkconstraints: *mut u32, pgp: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DefineGenericParam)(::windows::core::Interface::as_raw(self), tk, ulparamseq, dwparamflags, szname.into_param().abi(), reserved, rtkconstraints, pgp).ok()
    }
    pub unsafe fn SetGenericParamProps<P0>(&self, gp: u32, dwparamflags: u32, szname: P0, reserved: u32, rtkconstraints: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetGenericParamProps)(::windows::core::Interface::as_raw(self), gp, dwparamflags, szname.into_param().abi(), reserved, rtkconstraints).ok()
    }
    pub unsafe fn ResetENCLog(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetENCLog)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataEmit2, ::windows::core::IUnknown, IMetaDataEmit);
impl ::core::cmp::PartialEq for IMetaDataEmit2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataEmit2 {}
impl ::core::fmt::Debug for IMetaDataEmit2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataEmit2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataEmit2 {
    type Vtable = IMetaDataEmit2_Vtbl;
}
impl ::core::clone::Clone for IMetaDataEmit2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataEmit2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5dd9950_f693_42e6_830e_7b833e8146a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataEmit2_Vtbl {
    pub base__: IMetaDataEmit_Vtbl,
    pub DefineMethodSpec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tkparent: u32, pvsigblob: *mut u8, cbsigblob: u32, pmi: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeltaSaveSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsave: CorSaveSize, pdwsavesize: *mut u32) -> ::windows::core::HRESULT,
    pub SaveDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szfile: ::windows::core::PCWSTR, dwsaveflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveDeltaToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, dwsaveflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveDeltaToStream: usize,
    pub SaveDeltaToMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::HRESULT,
    pub DefineGenericParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, ulparamseq: u32, dwparamflags: u32, szname: ::windows::core::PCWSTR, reserved: u32, rtkconstraints: *mut u32, pgp: *mut u32) -> ::windows::core::HRESULT,
    pub SetGenericParamProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gp: u32, dwparamflags: u32, szname: ::windows::core::PCWSTR, reserved: u32, rtkconstraints: *mut u32) -> ::windows::core::HRESULT,
    pub ResetENCLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataError(::windows::core::IUnknown);
impl IMetaDataError {
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT, token: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnError)(::windows::core::Interface::as_raw(self), hrerror, token).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataError, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataError {}
impl ::core::fmt::Debug for IMetaDataError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataError {
    type Vtable = IMetaDataError_Vtbl;
}
impl ::core::clone::Clone for IMetaDataError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb81ff171_20f3_11d2_8dcc_00a0c9b09c19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataError_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, token: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataFilter(::windows::core::IUnknown);
impl IMetaDataFilter {
    pub unsafe fn UnmarkAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnmarkAll)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MarkToken(&self, tk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MarkToken)(::windows::core::Interface::as_raw(self), tk).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTokenMarked(&self, tk: u32, pismarked: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsTokenMarked)(::windows::core::Interface::as_raw(self), tk, pismarked).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataFilter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataFilter {}
impl ::core::fmt::Debug for IMetaDataFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataFilter {
    type Vtable = IMetaDataFilter_Vtbl;
}
impl ::core::clone::Clone for IMetaDataFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0e80dd1_12d4_11d3_b39d_00c04ff81795);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataFilter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub UnmarkAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MarkToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTokenMarked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, pismarked: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTokenMarked: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataImport(::windows::core::IUnknown);
impl IMetaDataImport {
    pub unsafe fn CloseEnum(&self, henum: *mut ::core::ffi::c_void) {
        (::windows::core::Interface::vtable(self).CloseEnum)(::windows::core::Interface::as_raw(self), henum)
    }
    pub unsafe fn CountEnum(&self, henum: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CountEnum)(::windows::core::Interface::as_raw(self), henum, pulcount).ok()
    }
    pub unsafe fn ResetEnum(&self, henum: *mut ::core::ffi::c_void, ulpos: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetEnum)(::windows::core::Interface::as_raw(self), henum, ulpos).ok()
    }
    pub unsafe fn EnumTypeDefs(&self, phenum: *mut *mut ::core::ffi::c_void, rtypedefs: *mut u32, cmax: u32, pctypedefs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumTypeDefs)(::windows::core::Interface::as_raw(self), phenum, rtypedefs, cmax, pctypedefs).ok()
    }
    pub unsafe fn EnumInterfaceImpls(&self, phenum: *mut *mut ::core::ffi::c_void, td: u32, rimpls: *mut u32, cmax: u32, pcimpls: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumInterfaceImpls)(::windows::core::Interface::as_raw(self), phenum, td, rimpls, cmax, pcimpls).ok()
    }
    pub unsafe fn EnumTypeRefs(&self, phenum: *mut *mut ::core::ffi::c_void, rtyperefs: *mut u32, cmax: u32, pctyperefs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumTypeRefs)(::windows::core::Interface::as_raw(self), phenum, rtyperefs, cmax, pctyperefs).ok()
    }
    pub unsafe fn FindTypeDefByName<P0>(&self, sztypedef: P0, tkenclosingclass: u32, ptd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindTypeDefByName)(::windows::core::Interface::as_raw(self), sztypedef.into_param().abi(), tkenclosingclass, ptd).ok()
    }
    pub unsafe fn GetScopeProps(&self, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32, pmvid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScopeProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname, pmvid).ok()
    }
    pub unsafe fn GetModuleFromScope(&self, pmd: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetModuleFromScope)(::windows::core::Interface::as_raw(self), pmd).ok()
    }
    pub unsafe fn GetTypeDefProps(&self, td: u32, sztypedef: ::core::option::Option<&mut [u16]>, pchtypedef: *mut u32, pdwtypedefflags: *mut u32, ptkextends: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTypeDefProps)(::windows::core::Interface::as_raw(self), td, ::core::mem::transmute(sztypedef.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), sztypedef.as_deref().map_or(0, |slice| slice.len() as _), pchtypedef, pdwtypedefflags, ptkextends).ok()
    }
    pub unsafe fn GetInterfaceImplProps(&self, iiimpl: u32, pclass: *mut u32, ptkiface: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInterfaceImplProps)(::windows::core::Interface::as_raw(self), iiimpl, pclass, ptkiface).ok()
    }
    pub unsafe fn GetTypeRefProps(&self, tr: u32, ptkresolutionscope: *mut u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTypeRefProps)(::windows::core::Interface::as_raw(self), tr, ptkresolutionscope, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname).ok()
    }
    pub unsafe fn ResolveTypeRef(&self, tr: u32, riid: *const ::windows::core::GUID, ppiscope: *mut ::core::option::Option<::windows::core::IUnknown>, ptd: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResolveTypeRef)(::windows::core::Interface::as_raw(self), tr, riid, ::core::mem::transmute(ppiscope), ptd).ok()
    }
    pub unsafe fn EnumMembers(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumMembers)(::windows::core::Interface::as_raw(self), phenum, cl, rmembers, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMembersWithName<P0>(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, szname: P0, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).EnumMembersWithName)(::windows::core::Interface::as_raw(self), phenum, cl, szname.into_param().abi(), rmembers, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMethods(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumMethods)(::windows::core::Interface::as_raw(self), phenum, cl, rmethods, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMethodsWithName<P0>(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, szname: P0, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).EnumMethodsWithName)(::windows::core::Interface::as_raw(self), phenum, cl, szname.into_param().abi(), rmethods, cmax, pctokens).ok()
    }
    pub unsafe fn EnumFields(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumFields)(::windows::core::Interface::as_raw(self), phenum, cl, rfields, cmax, pctokens).ok()
    }
    pub unsafe fn EnumFieldsWithName<P0>(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, szname: P0, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).EnumFieldsWithName)(::windows::core::Interface::as_raw(self), phenum, cl, szname.into_param().abi(), rfields, cmax, pctokens).ok()
    }
    pub unsafe fn EnumParams(&self, phenum: *mut *mut ::core::ffi::c_void, mb: u32, rparams: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumParams)(::windows::core::Interface::as_raw(self), phenum, mb, rparams, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMemberRefs(&self, phenum: *mut *mut ::core::ffi::c_void, tkparent: u32, rmemberrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumMemberRefs)(::windows::core::Interface::as_raw(self), phenum, tkparent, rmemberrefs, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMethodImpls(&self, phenum: *mut *mut ::core::ffi::c_void, td: u32, rmethodbody: *mut u32, rmethoddecl: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumMethodImpls)(::windows::core::Interface::as_raw(self), phenum, td, rmethodbody, rmethoddecl, cmax, pctokens).ok()
    }
    pub unsafe fn EnumPermissionSets(&self, phenum: *mut *mut ::core::ffi::c_void, tk: u32, dwactions: u32, rpermission: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumPermissionSets)(::windows::core::Interface::as_raw(self), phenum, tk, dwactions, rpermission, cmax, pctokens).ok()
    }
    pub unsafe fn FindMember<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindMember)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), pvsigblob, cbsigblob, pmb).ok()
    }
    pub unsafe fn FindMethod<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindMethod)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), pvsigblob, cbsigblob, pmb).ok()
    }
    pub unsafe fn FindField<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindField)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), pvsigblob, cbsigblob, pmb).ok()
    }
    pub unsafe fn FindMemberRef<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindMemberRef)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), pvsigblob, cbsigblob, pmr).ok()
    }
    pub unsafe fn GetMethodProps(&self, mb: u32, pclass: *mut u32, szmethod: ::core::option::Option<&mut [u16]>, pchmethod: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMethodProps)(::windows::core::Interface::as_raw(self), mb, pclass, ::core::mem::transmute(szmethod.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szmethod.as_deref().map_or(0, |slice| slice.len() as _), pchmethod, pdwattr, ppvsigblob, pcbsigblob, pulcoderva, pdwimplflags).ok()
    }
    pub unsafe fn GetMemberRefProps(&self, mr: u32, ptk: *mut u32, szmember: ::core::option::Option<&mut [u16]>, pchmember: *mut u32, ppvsigblob: *mut *mut u8, pbsig: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMemberRefProps)(::windows::core::Interface::as_raw(self), mr, ptk, ::core::mem::transmute(szmember.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szmember.as_deref().map_or(0, |slice| slice.len() as _), pchmember, ppvsigblob, pbsig).ok()
    }
    pub unsafe fn EnumProperties(&self, phenum: *mut *mut ::core::ffi::c_void, td: u32, rproperties: *mut u32, cmax: u32, pcproperties: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumProperties)(::windows::core::Interface::as_raw(self), phenum, td, rproperties, cmax, pcproperties).ok()
    }
    pub unsafe fn EnumEvents(&self, phenum: *mut *mut ::core::ffi::c_void, td: u32, revents: *mut u32, cmax: u32, pcevents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumEvents)(::windows::core::Interface::as_raw(self), phenum, td, revents, cmax, pcevents).ok()
    }
    pub unsafe fn GetEventProps<P0>(&self, ev: u32, pclass: *mut u32, szevent: P0, cchevent: u32, pchevent: *mut u32, pdweventflags: *mut u32, ptkeventtype: *mut u32, pmdaddon: *mut u32, pmdremoveon: *mut u32, pmdfire: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetEventProps)(::windows::core::Interface::as_raw(self), ev, pclass, szevent.into_param().abi(), cchevent, pchevent, pdweventflags, ptkeventtype, pmdaddon, pmdremoveon, pmdfire, rmdothermethod, cmax, pcothermethod).ok()
    }
    pub unsafe fn EnumMethodSemantics(&self, phenum: *mut *mut ::core::ffi::c_void, mb: u32, reventprop: *mut u32, cmax: u32, pceventprop: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumMethodSemantics)(::windows::core::Interface::as_raw(self), phenum, mb, reventprop, cmax, pceventprop).ok()
    }
    pub unsafe fn GetMethodSemantics(&self, mb: u32, tkeventprop: u32, pdwsemanticsflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMethodSemantics)(::windows::core::Interface::as_raw(self), mb, tkeventprop, pdwsemanticsflags).ok()
    }
    pub unsafe fn GetClassLayout(&self, td: u32, pdwpacksize: *mut u32, rfieldoffset: *mut COR_FIELD_OFFSET, cmax: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClassLayout)(::windows::core::Interface::as_raw(self), td, pdwpacksize, rfieldoffset, cmax, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetFieldMarshal(&self, tk: u32, ppvnativetype: *mut *mut u8, pcbnativetype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFieldMarshal)(::windows::core::Interface::as_raw(self), tk, ppvnativetype, pcbnativetype).ok()
    }
    pub unsafe fn GetRVA(&self, tk: u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRVA)(::windows::core::Interface::as_raw(self), tk, pulcoderva, pdwimplflags).ok()
    }
    pub unsafe fn GetPermissionSetProps(&self, pm: u32, pdwaction: *mut u32, ppvpermission: *const *const ::core::ffi::c_void, pcbpermission: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPermissionSetProps)(::windows::core::Interface::as_raw(self), pm, pdwaction, ppvpermission, pcbpermission).ok()
    }
    pub unsafe fn GetSigFromToken(&self, mdsig: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSigFromToken)(::windows::core::Interface::as_raw(self), mdsig, ppvsig, pcbsig).ok()
    }
    pub unsafe fn GetModuleRefProps(&self, mur: u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetModuleRefProps)(::windows::core::Interface::as_raw(self), mur, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname).ok()
    }
    pub unsafe fn EnumModuleRefs(&self, phenum: *mut *mut ::core::ffi::c_void, rmodulerefs: *mut u32, cmax: u32, pcmodulerefs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumModuleRefs)(::windows::core::Interface::as_raw(self), phenum, rmodulerefs, cmax, pcmodulerefs).ok()
    }
    pub unsafe fn GetTypeSpecFromToken(&self, typespec: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTypeSpecFromToken)(::windows::core::Interface::as_raw(self), typespec, ppvsig, pcbsig).ok()
    }
    pub unsafe fn GetNameFromToken(&self, tk: u32, pszutf8nameptr: *mut *mut i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNameFromToken)(::windows::core::Interface::as_raw(self), tk, pszutf8nameptr).ok()
    }
    pub unsafe fn EnumUnresolvedMethods(&self, phenum: *mut *mut ::core::ffi::c_void, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumUnresolvedMethods)(::windows::core::Interface::as_raw(self), phenum, rmethods, cmax, pctokens).ok()
    }
    pub unsafe fn GetUserString(&self, stk: u32, szstring: ::core::option::Option<&mut [u16]>, pchstring: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUserString)(::windows::core::Interface::as_raw(self), stk, ::core::mem::transmute(szstring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szstring.as_deref().map_or(0, |slice| slice.len() as _), pchstring).ok()
    }
    pub unsafe fn GetPinvokeMap(&self, tk: u32, pdwmappingflags: *mut u32, szimportname: ::core::option::Option<&mut [u16]>, pchimportname: *mut u32, pmrimportdll: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPinvokeMap)(::windows::core::Interface::as_raw(self), tk, pdwmappingflags, ::core::mem::transmute(szimportname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szimportname.as_deref().map_or(0, |slice| slice.len() as _), pchimportname, pmrimportdll).ok()
    }
    pub unsafe fn EnumSignatures(&self, phenum: *mut *mut ::core::ffi::c_void, rsignatures: *mut u32, cmax: u32, pcsignatures: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumSignatures)(::windows::core::Interface::as_raw(self), phenum, rsignatures, cmax, pcsignatures).ok()
    }
    pub unsafe fn EnumTypeSpecs(&self, phenum: *mut *mut ::core::ffi::c_void, rtypespecs: *mut u32, cmax: u32, pctypespecs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumTypeSpecs)(::windows::core::Interface::as_raw(self), phenum, rtypespecs, cmax, pctypespecs).ok()
    }
    pub unsafe fn EnumUserStrings(&self, phenum: *mut *mut ::core::ffi::c_void, rstrings: *mut u32, cmax: u32, pcstrings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumUserStrings)(::windows::core::Interface::as_raw(self), phenum, rstrings, cmax, pcstrings).ok()
    }
    pub unsafe fn GetParamForMethodIndex(&self, md: u32, ulparamseq: u32, ppd: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetParamForMethodIndex)(::windows::core::Interface::as_raw(self), md, ulparamseq, ppd).ok()
    }
    pub unsafe fn EnumCustomAttributes(&self, phenum: *mut *mut ::core::ffi::c_void, tk: u32, tktype: u32, rcustomattributes: *mut u32, cmax: u32, pccustomattributes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumCustomAttributes)(::windows::core::Interface::as_raw(self), phenum, tk, tktype, rcustomattributes, cmax, pccustomattributes).ok()
    }
    pub unsafe fn GetCustomAttributeProps(&self, cv: u32, ptkobj: *mut u32, ptktype: *mut u32, ppblob: *const *const ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCustomAttributeProps)(::windows::core::Interface::as_raw(self), cv, ptkobj, ptktype, ppblob, pcbsize).ok()
    }
    pub unsafe fn FindTypeRef<P0>(&self, tkresolutionscope: u32, szname: P0, ptr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindTypeRef)(::windows::core::Interface::as_raw(self), tkresolutionscope, szname.into_param().abi(), ptr).ok()
    }
    pub unsafe fn GetMemberProps(&self, mb: u32, pclass: *mut u32, szmember: ::core::option::Option<&mut [u16]>, pchmember: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut ::core::ffi::c_void, pcchvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMemberProps)(::windows::core::Interface::as_raw(self), mb, pclass, ::core::mem::transmute(szmember.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szmember.as_deref().map_or(0, |slice| slice.len() as _), pchmember, pdwattr, ppvsigblob, pcbsigblob, pulcoderva, pdwimplflags, pdwcplustypeflag, ppvalue, pcchvalue).ok()
    }
    pub unsafe fn GetFieldProps(&self, mb: u32, pclass: *mut u32, szfield: ::core::option::Option<&mut [u16]>, pchfield: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut ::core::ffi::c_void, pcchvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFieldProps)(::windows::core::Interface::as_raw(self), mb, pclass, ::core::mem::transmute(szfield.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szfield.as_deref().map_or(0, |slice| slice.len() as _), pchfield, pdwattr, ppvsigblob, pcbsigblob, pdwcplustypeflag, ppvalue, pcchvalue).ok()
    }
    pub unsafe fn GetPropertyProps<P0>(&self, prop: u32, pclass: *mut u32, szproperty: P0, cchproperty: u32, pchproperty: *mut u32, pdwpropflags: *mut u32, ppvsig: *mut *mut u8, pbsig: *mut u32, pdwcplustypeflag: *mut u32, ppdefaultvalue: *mut *mut ::core::ffi::c_void, pcchdefaultvalue: *mut u32, pmdsetter: *mut u32, pmdgetter: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPropertyProps)(::windows::core::Interface::as_raw(self), prop, pclass, szproperty.into_param().abi(), cchproperty, pchproperty, pdwpropflags, ppvsig, pbsig, pdwcplustypeflag, ppdefaultvalue, pcchdefaultvalue, pmdsetter, pmdgetter, rmdothermethod, cmax, pcothermethod).ok()
    }
    pub unsafe fn GetParamProps(&self, tk: u32, pmd: *mut u32, pulsequence: *mut u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32, pdwattr: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut ::core::ffi::c_void, pcchvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetParamProps)(::windows::core::Interface::as_raw(self), tk, pmd, pulsequence, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname, pdwattr, pdwcplustypeflag, ppvalue, pcchvalue).ok()
    }
    pub unsafe fn GetCustomAttributeByName<P0>(&self, tkobj: u32, szname: P0, ppdata: *const *const ::core::ffi::c_void, pcbdata: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetCustomAttributeByName)(::windows::core::Interface::as_raw(self), tkobj, szname.into_param().abi(), ppdata, pcbdata).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValidToken(&self, tk: u32) -> super::super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsValidToken)(::windows::core::Interface::as_raw(self), tk)
    }
    pub unsafe fn GetNestedClassProps(&self, tdnestedclass: u32, ptdenclosingclass: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNestedClassProps)(::windows::core::Interface::as_raw(self), tdnestedclass, ptdenclosingclass).ok()
    }
    pub unsafe fn GetNativeCallConvFromSig(&self, pvsig: *const ::core::ffi::c_void, cbsig: u32, pcallconv: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNativeCallConvFromSig)(::windows::core::Interface::as_raw(self), pvsig, cbsig, pcallconv).ok()
    }
    pub unsafe fn IsGlobal(&self, pd: u32, pbglobal: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsGlobal)(::windows::core::Interface::as_raw(self), pd, pbglobal).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataImport, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataImport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataImport {}
impl ::core::fmt::Debug for IMetaDataImport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataImport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataImport {
    type Vtable = IMetaDataImport_Vtbl;
}
impl ::core::clone::Clone for IMetaDataImport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataImport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7dac8207_d3ae_4c75_9b67_92801a497d44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataImport_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CloseEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, henum: *mut ::core::ffi::c_void),
    pub CountEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, henum: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT,
    pub ResetEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, henum: *mut ::core::ffi::c_void, ulpos: u32) -> ::windows::core::HRESULT,
    pub EnumTypeDefs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rtypedefs: *mut u32, cmax: u32, pctypedefs: *mut u32) -> ::windows::core::HRESULT,
    pub EnumInterfaceImpls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, td: u32, rimpls: *mut u32, cmax: u32, pcimpls: *mut u32) -> ::windows::core::HRESULT,
    pub EnumTypeRefs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rtyperefs: *mut u32, cmax: u32, pctyperefs: *mut u32) -> ::windows::core::HRESULT,
    pub FindTypeDefByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztypedef: ::windows::core::PCWSTR, tkenclosingclass: u32, ptd: *mut u32) -> ::windows::core::HRESULT,
    pub GetScopeProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32, pmvid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetModuleFromScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmd: *mut u32) -> ::windows::core::HRESULT,
    pub GetTypeDefProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, sztypedef: ::windows::core::PWSTR, cchtypedef: u32, pchtypedef: *mut u32, pdwtypedefflags: *mut u32, ptkextends: *mut u32) -> ::windows::core::HRESULT,
    pub GetInterfaceImplProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iiimpl: u32, pclass: *mut u32, ptkiface: *mut u32) -> ::windows::core::HRESULT,
    pub GetTypeRefProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tr: u32, ptkresolutionscope: *mut u32, szname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32) -> ::windows::core::HRESULT,
    pub ResolveTypeRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tr: u32, riid: *const ::windows::core::GUID, ppiscope: *mut *mut ::core::ffi::c_void, ptd: *mut u32) -> ::windows::core::HRESULT,
    pub EnumMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, cl: u32, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumMembersWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, cl: u32, szname: ::windows::core::PCWSTR, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, cl: u32, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumMethodsWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, cl: u32, szname: ::windows::core::PCWSTR, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, cl: u32, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumFieldsWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, cl: u32, szname: ::windows::core::PCWSTR, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, mb: u32, rparams: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumMemberRefs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, tkparent: u32, rmemberrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumMethodImpls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, td: u32, rmethodbody: *mut u32, rmethoddecl: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub EnumPermissionSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, tk: u32, dwactions: u32, rpermission: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub FindMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, szname: ::windows::core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> ::windows::core::HRESULT,
    pub FindMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, szname: ::windows::core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> ::windows::core::HRESULT,
    pub FindField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, szname: ::windows::core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> ::windows::core::HRESULT,
    pub FindMemberRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, szname: ::windows::core::PCWSTR, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> ::windows::core::HRESULT,
    pub GetMethodProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mb: u32, pclass: *mut u32, szmethod: ::windows::core::PWSTR, cchmethod: u32, pchmethod: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetMemberRefProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mr: u32, ptk: *mut u32, szmember: ::windows::core::PWSTR, cchmember: u32, pchmember: *mut u32, ppvsigblob: *mut *mut u8, pbsig: *mut u32) -> ::windows::core::HRESULT,
    pub EnumProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, td: u32, rproperties: *mut u32, cmax: u32, pcproperties: *mut u32) -> ::windows::core::HRESULT,
    pub EnumEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, td: u32, revents: *mut u32, cmax: u32, pcevents: *mut u32) -> ::windows::core::HRESULT,
    pub GetEventProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ev: u32, pclass: *mut u32, szevent: ::windows::core::PCWSTR, cchevent: u32, pchevent: *mut u32, pdweventflags: *mut u32, ptkeventtype: *mut u32, pmdaddon: *mut u32, pmdremoveon: *mut u32, pmdfire: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> ::windows::core::HRESULT,
    pub EnumMethodSemantics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, mb: u32, reventprop: *mut u32, cmax: u32, pceventprop: *mut u32) -> ::windows::core::HRESULT,
    pub GetMethodSemantics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mb: u32, tkeventprop: u32, pdwsemanticsflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetClassLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, td: u32, pdwpacksize: *mut u32, rfieldoffset: *mut COR_FIELD_OFFSET, cmax: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::HRESULT,
    pub GetFieldMarshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, ppvnativetype: *mut *mut u8, pcbnativetype: *mut u32) -> ::windows::core::HRESULT,
    pub GetRVA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetPermissionSetProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pm: u32, pdwaction: *mut u32, ppvpermission: *const *const ::core::ffi::c_void, pcbpermission: *mut u32) -> ::windows::core::HRESULT,
    pub GetSigFromToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mdsig: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> ::windows::core::HRESULT,
    pub GetModuleRefProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mur: u32, szname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32) -> ::windows::core::HRESULT,
    pub EnumModuleRefs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rmodulerefs: *mut u32, cmax: u32, pcmodulerefs: *mut u32) -> ::windows::core::HRESULT,
    pub GetTypeSpecFromToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typespec: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> ::windows::core::HRESULT,
    pub GetNameFromToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, pszutf8nameptr: *mut *mut i8) -> ::windows::core::HRESULT,
    pub EnumUnresolvedMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::HRESULT,
    pub GetUserString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stk: u32, szstring: ::windows::core::PWSTR, cchstring: u32, pchstring: *mut u32) -> ::windows::core::HRESULT,
    pub GetPinvokeMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, pdwmappingflags: *mut u32, szimportname: ::windows::core::PWSTR, cchimportname: u32, pchimportname: *mut u32, pmrimportdll: *mut u32) -> ::windows::core::HRESULT,
    pub EnumSignatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rsignatures: *mut u32, cmax: u32, pcsignatures: *mut u32) -> ::windows::core::HRESULT,
    pub EnumTypeSpecs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rtypespecs: *mut u32, cmax: u32, pctypespecs: *mut u32) -> ::windows::core::HRESULT,
    pub EnumUserStrings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, rstrings: *mut u32, cmax: u32, pcstrings: *mut u32) -> ::windows::core::HRESULT,
    pub GetParamForMethodIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, md: u32, ulparamseq: u32, ppd: *mut u32) -> ::windows::core::HRESULT,
    pub EnumCustomAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, tk: u32, tktype: u32, rcustomattributes: *mut u32, cmax: u32, pccustomattributes: *mut u32) -> ::windows::core::HRESULT,
    pub GetCustomAttributeProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: u32, ptkobj: *mut u32, ptktype: *mut u32, ppblob: *const *const ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub FindTypeRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tkresolutionscope: u32, szname: ::windows::core::PCWSTR, ptr: *mut u32) -> ::windows::core::HRESULT,
    pub GetMemberProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mb: u32, pclass: *mut u32, szmember: ::windows::core::PWSTR, cchmember: u32, pchmember: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut ::core::ffi::c_void, pcchvalue: *mut u32) -> ::windows::core::HRESULT,
    pub GetFieldProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mb: u32, pclass: *mut u32, szfield: ::windows::core::PWSTR, cchfield: u32, pchfield: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut ::core::ffi::c_void, pcchvalue: *mut u32) -> ::windows::core::HRESULT,
    pub GetPropertyProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prop: u32, pclass: *mut u32, szproperty: ::windows::core::PCWSTR, cchproperty: u32, pchproperty: *mut u32, pdwpropflags: *mut u32, ppvsig: *mut *mut u8, pbsig: *mut u32, pdwcplustypeflag: *mut u32, ppdefaultvalue: *mut *mut ::core::ffi::c_void, pcchdefaultvalue: *mut u32, pmdsetter: *mut u32, pmdgetter: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> ::windows::core::HRESULT,
    pub GetParamProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32, pmd: *mut u32, pulsequence: *mut u32, szname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32, pdwattr: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut ::core::ffi::c_void, pcchvalue: *mut u32) -> ::windows::core::HRESULT,
    pub GetCustomAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tkobj: u32, szname: ::windows::core::PCWSTR, ppdata: *const *const ::core::ffi::c_void, pcbdata: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsValidToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tk: u32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsValidToken: usize,
    pub GetNestedClassProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tdnestedclass: u32, ptdenclosingclass: *mut u32) -> ::windows::core::HRESULT,
    pub GetNativeCallConvFromSig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvsig: *const ::core::ffi::c_void, cbsig: u32, pcallconv: *mut u32) -> ::windows::core::HRESULT,
    pub IsGlobal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd: u32, pbglobal: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataImport2(::windows::core::IUnknown);
impl IMetaDataImport2 {
    pub unsafe fn CloseEnum(&self, henum: *mut ::core::ffi::c_void) {
        (::windows::core::Interface::vtable(self).base__.CloseEnum)(::windows::core::Interface::as_raw(self), henum)
    }
    pub unsafe fn CountEnum(&self, henum: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CountEnum)(::windows::core::Interface::as_raw(self), henum, pulcount).ok()
    }
    pub unsafe fn ResetEnum(&self, henum: *mut ::core::ffi::c_void, ulpos: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ResetEnum)(::windows::core::Interface::as_raw(self), henum, ulpos).ok()
    }
    pub unsafe fn EnumTypeDefs(&self, phenum: *mut *mut ::core::ffi::c_void, rtypedefs: *mut u32, cmax: u32, pctypedefs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumTypeDefs)(::windows::core::Interface::as_raw(self), phenum, rtypedefs, cmax, pctypedefs).ok()
    }
    pub unsafe fn EnumInterfaceImpls(&self, phenum: *mut *mut ::core::ffi::c_void, td: u32, rimpls: *mut u32, cmax: u32, pcimpls: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumInterfaceImpls)(::windows::core::Interface::as_raw(self), phenum, td, rimpls, cmax, pcimpls).ok()
    }
    pub unsafe fn EnumTypeRefs(&self, phenum: *mut *mut ::core::ffi::c_void, rtyperefs: *mut u32, cmax: u32, pctyperefs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumTypeRefs)(::windows::core::Interface::as_raw(self), phenum, rtyperefs, cmax, pctyperefs).ok()
    }
    pub unsafe fn FindTypeDefByName<P0>(&self, sztypedef: P0, tkenclosingclass: u32, ptd: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.FindTypeDefByName)(::windows::core::Interface::as_raw(self), sztypedef.into_param().abi(), tkenclosingclass, ptd).ok()
    }
    pub unsafe fn GetScopeProps(&self, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32, pmvid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetScopeProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname, pmvid).ok()
    }
    pub unsafe fn GetModuleFromScope(&self, pmd: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetModuleFromScope)(::windows::core::Interface::as_raw(self), pmd).ok()
    }
    pub unsafe fn GetTypeDefProps(&self, td: u32, sztypedef: ::core::option::Option<&mut [u16]>, pchtypedef: *mut u32, pdwtypedefflags: *mut u32, ptkextends: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTypeDefProps)(::windows::core::Interface::as_raw(self), td, ::core::mem::transmute(sztypedef.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), sztypedef.as_deref().map_or(0, |slice| slice.len() as _), pchtypedef, pdwtypedefflags, ptkextends).ok()
    }
    pub unsafe fn GetInterfaceImplProps(&self, iiimpl: u32, pclass: *mut u32, ptkiface: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInterfaceImplProps)(::windows::core::Interface::as_raw(self), iiimpl, pclass, ptkiface).ok()
    }
    pub unsafe fn GetTypeRefProps(&self, tr: u32, ptkresolutionscope: *mut u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTypeRefProps)(::windows::core::Interface::as_raw(self), tr, ptkresolutionscope, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname).ok()
    }
    pub unsafe fn ResolveTypeRef(&self, tr: u32, riid: *const ::windows::core::GUID, ppiscope: *mut ::core::option::Option<::windows::core::IUnknown>, ptd: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ResolveTypeRef)(::windows::core::Interface::as_raw(self), tr, riid, ::core::mem::transmute(ppiscope), ptd).ok()
    }
    pub unsafe fn EnumMembers(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumMembers)(::windows::core::Interface::as_raw(self), phenum, cl, rmembers, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMembersWithName<P0>(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, szname: P0, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.EnumMembersWithName)(::windows::core::Interface::as_raw(self), phenum, cl, szname.into_param().abi(), rmembers, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMethods(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumMethods)(::windows::core::Interface::as_raw(self), phenum, cl, rmethods, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMethodsWithName<P0>(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, szname: P0, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.EnumMethodsWithName)(::windows::core::Interface::as_raw(self), phenum, cl, szname.into_param().abi(), rmethods, cmax, pctokens).ok()
    }
    pub unsafe fn EnumFields(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumFields)(::windows::core::Interface::as_raw(self), phenum, cl, rfields, cmax, pctokens).ok()
    }
    pub unsafe fn EnumFieldsWithName<P0>(&self, phenum: *mut *mut ::core::ffi::c_void, cl: u32, szname: P0, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.EnumFieldsWithName)(::windows::core::Interface::as_raw(self), phenum, cl, szname.into_param().abi(), rfields, cmax, pctokens).ok()
    }
    pub unsafe fn EnumParams(&self, phenum: *mut *mut ::core::ffi::c_void, mb: u32, rparams: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumParams)(::windows::core::Interface::as_raw(self), phenum, mb, rparams, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMemberRefs(&self, phenum: *mut *mut ::core::ffi::c_void, tkparent: u32, rmemberrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumMemberRefs)(::windows::core::Interface::as_raw(self), phenum, tkparent, rmemberrefs, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMethodImpls(&self, phenum: *mut *mut ::core::ffi::c_void, td: u32, rmethodbody: *mut u32, rmethoddecl: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumMethodImpls)(::windows::core::Interface::as_raw(self), phenum, td, rmethodbody, rmethoddecl, cmax, pctokens).ok()
    }
    pub unsafe fn EnumPermissionSets(&self, phenum: *mut *mut ::core::ffi::c_void, tk: u32, dwactions: u32, rpermission: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumPermissionSets)(::windows::core::Interface::as_raw(self), phenum, tk, dwactions, rpermission, cmax, pctokens).ok()
    }
    pub unsafe fn FindMember<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.FindMember)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), pvsigblob, cbsigblob, pmb).ok()
    }
    pub unsafe fn FindMethod<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.FindMethod)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), pvsigblob, cbsigblob, pmb).ok()
    }
    pub unsafe fn FindField<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.FindField)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), pvsigblob, cbsigblob, pmb).ok()
    }
    pub unsafe fn FindMemberRef<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.FindMemberRef)(::windows::core::Interface::as_raw(self), td, szname.into_param().abi(), pvsigblob, cbsigblob, pmr).ok()
    }
    pub unsafe fn GetMethodProps(&self, mb: u32, pclass: *mut u32, szmethod: ::core::option::Option<&mut [u16]>, pchmethod: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMethodProps)(::windows::core::Interface::as_raw(self), mb, pclass, ::core::mem::transmute(szmethod.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szmethod.as_deref().map_or(0, |slice| slice.len() as _), pchmethod, pdwattr, ppvsigblob, pcbsigblob, pulcoderva, pdwimplflags).ok()
    }
    pub unsafe fn GetMemberRefProps(&self, mr: u32, ptk: *mut u32, szmember: ::core::option::Option<&mut [u16]>, pchmember: *mut u32, ppvsigblob: *mut *mut u8, pbsig: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMemberRefProps)(::windows::core::Interface::as_raw(self), mr, ptk, ::core::mem::transmute(szmember.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szmember.as_deref().map_or(0, |slice| slice.len() as _), pchmember, ppvsigblob, pbsig).ok()
    }
    pub unsafe fn EnumProperties(&self, phenum: *mut *mut ::core::ffi::c_void, td: u32, rproperties: *mut u32, cmax: u32, pcproperties: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumProperties)(::windows::core::Interface::as_raw(self), phenum, td, rproperties, cmax, pcproperties).ok()
    }
    pub unsafe fn EnumEvents(&self, phenum: *mut *mut ::core::ffi::c_void, td: u32, revents: *mut u32, cmax: u32, pcevents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumEvents)(::windows::core::Interface::as_raw(self), phenum, td, revents, cmax, pcevents).ok()
    }
    pub unsafe fn GetEventProps<P0>(&self, ev: u32, pclass: *mut u32, szevent: P0, cchevent: u32, pchevent: *mut u32, pdweventflags: *mut u32, ptkeventtype: *mut u32, pmdaddon: *mut u32, pmdremoveon: *mut u32, pmdfire: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetEventProps)(::windows::core::Interface::as_raw(self), ev, pclass, szevent.into_param().abi(), cchevent, pchevent, pdweventflags, ptkeventtype, pmdaddon, pmdremoveon, pmdfire, rmdothermethod, cmax, pcothermethod).ok()
    }
    pub unsafe fn EnumMethodSemantics(&self, phenum: *mut *mut ::core::ffi::c_void, mb: u32, reventprop: *mut u32, cmax: u32, pceventprop: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumMethodSemantics)(::windows::core::Interface::as_raw(self), phenum, mb, reventprop, cmax, pceventprop).ok()
    }
    pub unsafe fn GetMethodSemantics(&self, mb: u32, tkeventprop: u32, pdwsemanticsflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMethodSemantics)(::windows::core::Interface::as_raw(self), mb, tkeventprop, pdwsemanticsflags).ok()
    }
    pub unsafe fn GetClassLayout(&self, td: u32, pdwpacksize: *mut u32, rfieldoffset: *mut COR_FIELD_OFFSET, cmax: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClassLayout)(::windows::core::Interface::as_raw(self), td, pdwpacksize, rfieldoffset, cmax, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetFieldMarshal(&self, tk: u32, ppvnativetype: *mut *mut u8, pcbnativetype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFieldMarshal)(::windows::core::Interface::as_raw(self), tk, ppvnativetype, pcbnativetype).ok()
    }
    pub unsafe fn GetRVA(&self, tk: u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRVA)(::windows::core::Interface::as_raw(self), tk, pulcoderva, pdwimplflags).ok()
    }
    pub unsafe fn GetPermissionSetProps(&self, pm: u32, pdwaction: *mut u32, ppvpermission: *const *const ::core::ffi::c_void, pcbpermission: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPermissionSetProps)(::windows::core::Interface::as_raw(self), pm, pdwaction, ppvpermission, pcbpermission).ok()
    }
    pub unsafe fn GetSigFromToken(&self, mdsig: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSigFromToken)(::windows::core::Interface::as_raw(self), mdsig, ppvsig, pcbsig).ok()
    }
    pub unsafe fn GetModuleRefProps(&self, mur: u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetModuleRefProps)(::windows::core::Interface::as_raw(self), mur, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname).ok()
    }
    pub unsafe fn EnumModuleRefs(&self, phenum: *mut *mut ::core::ffi::c_void, rmodulerefs: *mut u32, cmax: u32, pcmodulerefs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumModuleRefs)(::windows::core::Interface::as_raw(self), phenum, rmodulerefs, cmax, pcmodulerefs).ok()
    }
    pub unsafe fn GetTypeSpecFromToken(&self, typespec: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTypeSpecFromToken)(::windows::core::Interface::as_raw(self), typespec, ppvsig, pcbsig).ok()
    }
    pub unsafe fn GetNameFromToken(&self, tk: u32, pszutf8nameptr: *mut *mut i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNameFromToken)(::windows::core::Interface::as_raw(self), tk, pszutf8nameptr).ok()
    }
    pub unsafe fn EnumUnresolvedMethods(&self, phenum: *mut *mut ::core::ffi::c_void, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumUnresolvedMethods)(::windows::core::Interface::as_raw(self), phenum, rmethods, cmax, pctokens).ok()
    }
    pub unsafe fn GetUserString(&self, stk: u32, szstring: ::core::option::Option<&mut [u16]>, pchstring: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetUserString)(::windows::core::Interface::as_raw(self), stk, ::core::mem::transmute(szstring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szstring.as_deref().map_or(0, |slice| slice.len() as _), pchstring).ok()
    }
    pub unsafe fn GetPinvokeMap(&self, tk: u32, pdwmappingflags: *mut u32, szimportname: ::core::option::Option<&mut [u16]>, pchimportname: *mut u32, pmrimportdll: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPinvokeMap)(::windows::core::Interface::as_raw(self), tk, pdwmappingflags, ::core::mem::transmute(szimportname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szimportname.as_deref().map_or(0, |slice| slice.len() as _), pchimportname, pmrimportdll).ok()
    }
    pub unsafe fn EnumSignatures(&self, phenum: *mut *mut ::core::ffi::c_void, rsignatures: *mut u32, cmax: u32, pcsignatures: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumSignatures)(::windows::core::Interface::as_raw(self), phenum, rsignatures, cmax, pcsignatures).ok()
    }
    pub unsafe fn EnumTypeSpecs(&self, phenum: *mut *mut ::core::ffi::c_void, rtypespecs: *mut u32, cmax: u32, pctypespecs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumTypeSpecs)(::windows::core::Interface::as_raw(self), phenum, rtypespecs, cmax, pctypespecs).ok()
    }
    pub unsafe fn EnumUserStrings(&self, phenum: *mut *mut ::core::ffi::c_void, rstrings: *mut u32, cmax: u32, pcstrings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumUserStrings)(::windows::core::Interface::as_raw(self), phenum, rstrings, cmax, pcstrings).ok()
    }
    pub unsafe fn GetParamForMethodIndex(&self, md: u32, ulparamseq: u32, ppd: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetParamForMethodIndex)(::windows::core::Interface::as_raw(self), md, ulparamseq, ppd).ok()
    }
    pub unsafe fn EnumCustomAttributes(&self, phenum: *mut *mut ::core::ffi::c_void, tk: u32, tktype: u32, rcustomattributes: *mut u32, cmax: u32, pccustomattributes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumCustomAttributes)(::windows::core::Interface::as_raw(self), phenum, tk, tktype, rcustomattributes, cmax, pccustomattributes).ok()
    }
    pub unsafe fn GetCustomAttributeProps(&self, cv: u32, ptkobj: *mut u32, ptktype: *mut u32, ppblob: *const *const ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCustomAttributeProps)(::windows::core::Interface::as_raw(self), cv, ptkobj, ptktype, ppblob, pcbsize).ok()
    }
    pub unsafe fn FindTypeRef<P0>(&self, tkresolutionscope: u32, szname: P0, ptr: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.FindTypeRef)(::windows::core::Interface::as_raw(self), tkresolutionscope, szname.into_param().abi(), ptr).ok()
    }
    pub unsafe fn GetMemberProps(&self, mb: u32, pclass: *mut u32, szmember: ::core::option::Option<&mut [u16]>, pchmember: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut ::core::ffi::c_void, pcchvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMemberProps)(::windows::core::Interface::as_raw(self), mb, pclass, ::core::mem::transmute(szmember.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szmember.as_deref().map_or(0, |slice| slice.len() as _), pchmember, pdwattr, ppvsigblob, pcbsigblob, pulcoderva, pdwimplflags, pdwcplustypeflag, ppvalue, pcchvalue).ok()
    }
    pub unsafe fn GetFieldProps(&self, mb: u32, pclass: *mut u32, szfield: ::core::option::Option<&mut [u16]>, pchfield: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut ::core::ffi::c_void, pcchvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFieldProps)(::windows::core::Interface::as_raw(self), mb, pclass, ::core::mem::transmute(szfield.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szfield.as_deref().map_or(0, |slice| slice.len() as _), pchfield, pdwattr, ppvsigblob, pcbsigblob, pdwcplustypeflag, ppvalue, pcchvalue).ok()
    }
    pub unsafe fn GetPropertyProps<P0>(&self, prop: u32, pclass: *mut u32, szproperty: P0, cchproperty: u32, pchproperty: *mut u32, pdwpropflags: *mut u32, ppvsig: *mut *mut u8, pbsig: *mut u32, pdwcplustypeflag: *mut u32, ppdefaultvalue: *mut *mut ::core::ffi::c_void, pcchdefaultvalue: *mut u32, pmdsetter: *mut u32, pmdgetter: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetPropertyProps)(::windows::core::Interface::as_raw(self), prop, pclass, szproperty.into_param().abi(), cchproperty, pchproperty, pdwpropflags, ppvsig, pbsig, pdwcplustypeflag, ppdefaultvalue, pcchdefaultvalue, pmdsetter, pmdgetter, rmdothermethod, cmax, pcothermethod).ok()
    }
    pub unsafe fn GetParamProps(&self, tk: u32, pmd: *mut u32, pulsequence: *mut u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32, pdwattr: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut ::core::ffi::c_void, pcchvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetParamProps)(::windows::core::Interface::as_raw(self), tk, pmd, pulsequence, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname, pdwattr, pdwcplustypeflag, ppvalue, pcchvalue).ok()
    }
    pub unsafe fn GetCustomAttributeByName<P0>(&self, tkobj: u32, szname: P0, ppdata: *const *const ::core::ffi::c_void, pcbdata: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetCustomAttributeByName)(::windows::core::Interface::as_raw(self), tkobj, szname.into_param().abi(), ppdata, pcbdata).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValidToken(&self, tk: u32) -> super::super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValidToken)(::windows::core::Interface::as_raw(self), tk)
    }
    pub unsafe fn GetNestedClassProps(&self, tdnestedclass: u32, ptdenclosingclass: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNestedClassProps)(::windows::core::Interface::as_raw(self), tdnestedclass, ptdenclosingclass).ok()
    }
    pub unsafe fn GetNativeCallConvFromSig(&self, pvsig: *const ::core::ffi::c_void, cbsig: u32, pcallconv: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNativeCallConvFromSig)(::windows::core::Interface::as_raw(self), pvsig, cbsig, pcallconv).ok()
    }
    pub unsafe fn IsGlobal(&self, pd: u32, pbglobal: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsGlobal)(::windows::core::Interface::as_raw(self), pd, pbglobal).ok()
    }
    pub unsafe fn EnumGenericParams(&self, phenum: *mut *mut ::core::ffi::c_void, tk: u32, rgenericparams: *mut u32, cmax: u32, pcgenericparams: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumGenericParams)(::windows::core::Interface::as_raw(self), phenum, tk, rgenericparams, cmax, pcgenericparams).ok()
    }
    pub unsafe fn GetGenericParamProps(&self, gp: u32, pulparamseq: *mut u32, pdwparamflags: *mut u32, ptowner: *mut u32, reserved: *mut u32, wzname: ::core::option::Option<&mut [u16]>, pchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGenericParamProps)(::windows::core::Interface::as_raw(self), gp, pulparamseq, pdwparamflags, ptowner, reserved, ::core::mem::transmute(wzname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), wzname.as_deref().map_or(0, |slice| slice.len() as _), pchname).ok()
    }
    pub unsafe fn GetMethodSpecProps(&self, mi: u32, tkparent: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMethodSpecProps)(::windows::core::Interface::as_raw(self), mi, tkparent, ppvsigblob, pcbsigblob).ok()
    }
    pub unsafe fn EnumGenericParamConstraints(&self, phenum: *mut *mut ::core::ffi::c_void, tk: u32, rgenericparamconstraints: *mut u32, cmax: u32, pcgenericparamconstraints: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumGenericParamConstraints)(::windows::core::Interface::as_raw(self), phenum, tk, rgenericparamconstraints, cmax, pcgenericparamconstraints).ok()
    }
    pub unsafe fn GetGenericParamConstraintProps(&self, gpc: u32, ptgenericparam: *mut u32, ptkconstrainttype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGenericParamConstraintProps)(::windows::core::Interface::as_raw(self), gpc, ptgenericparam, ptkconstrainttype).ok()
    }
    pub unsafe fn GetPEKind(&self, pdwpekind: *mut u32, pdwmachine: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPEKind)(::windows::core::Interface::as_raw(self), pdwpekind, pdwmachine).ok()
    }
    pub unsafe fn GetVersionString(&self, pwzbuf: ::core::option::Option<&mut [u16]>, pccbufsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVersionString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwzbuf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pwzbuf.as_deref().map_or(0, |slice| slice.len() as _), pccbufsize).ok()
    }
    pub unsafe fn EnumMethodSpecs(&self, phenum: *mut *mut ::core::ffi::c_void, tk: u32, rmethodspecs: *mut u32, cmax: u32, pcmethodspecs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumMethodSpecs)(::windows::core::Interface::as_raw(self), phenum, tk, rmethodspecs, cmax, pcmethodspecs).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataImport2, ::windows::core::IUnknown, IMetaDataImport);
impl ::core::cmp::PartialEq for IMetaDataImport2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataImport2 {}
impl ::core::fmt::Debug for IMetaDataImport2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataImport2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataImport2 {
    type Vtable = IMetaDataImport2_Vtbl;
}
impl ::core::clone::Clone for IMetaDataImport2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataImport2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfce5efa0_8bba_4f8e_a036_8f2022b08466);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataImport2_Vtbl {
    pub base__: IMetaDataImport_Vtbl,
    pub EnumGenericParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, tk: u32, rgenericparams: *mut u32, cmax: u32, pcgenericparams: *mut u32) -> ::windows::core::HRESULT,
    pub GetGenericParamProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gp: u32, pulparamseq: *mut u32, pdwparamflags: *mut u32, ptowner: *mut u32, reserved: *mut u32, wzname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32) -> ::windows::core::HRESULT,
    pub GetMethodSpecProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mi: u32, tkparent: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32) -> ::windows::core::HRESULT,
    pub EnumGenericParamConstraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, tk: u32, rgenericparamconstraints: *mut u32, cmax: u32, pcgenericparamconstraints: *mut u32) -> ::windows::core::HRESULT,
    pub GetGenericParamConstraintProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpc: u32, ptgenericparam: *mut u32, ptkconstrainttype: *mut u32) -> ::windows::core::HRESULT,
    pub GetPEKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpekind: *mut u32, pdwmachine: *mut u32) -> ::windows::core::HRESULT,
    pub GetVersionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzbuf: ::windows::core::PWSTR, ccbufsize: u32, pccbufsize: *mut u32) -> ::windows::core::HRESULT,
    pub EnumMethodSpecs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phenum: *mut *mut ::core::ffi::c_void, tk: u32, rmethodspecs: *mut u32, cmax: u32, pcmethodspecs: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataInfo(::windows::core::IUnknown);
impl IMetaDataInfo {
    pub unsafe fn GetFileMapping(&self, ppvdata: *const *const ::core::ffi::c_void, pcbdata: *mut u64, pdwmappingtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFileMapping)(::windows::core::Interface::as_raw(self), ppvdata, pcbdata, pdwmappingtype).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataInfo {}
impl ::core::fmt::Debug for IMetaDataInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataInfo {
    type Vtable = IMetaDataInfo_Vtbl;
}
impl ::core::clone::Clone for IMetaDataInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7998ea64_7f95_48b8_86fc_17caf48bf5cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFileMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvdata: *const *const ::core::ffi::c_void, pcbdata: *mut u64, pdwmappingtype: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataTables(::windows::core::IUnknown);
impl IMetaDataTables {
    pub unsafe fn GetStringHeapSize(&self, pcbstrings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStringHeapSize)(::windows::core::Interface::as_raw(self), pcbstrings).ok()
    }
    pub unsafe fn GetBlobHeapSize(&self, pcbblobs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBlobHeapSize)(::windows::core::Interface::as_raw(self), pcbblobs).ok()
    }
    pub unsafe fn GetGuidHeapSize(&self, pcbguids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGuidHeapSize)(::windows::core::Interface::as_raw(self), pcbguids).ok()
    }
    pub unsafe fn GetUserStringHeapSize(&self, pcbblobs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUserStringHeapSize)(::windows::core::Interface::as_raw(self), pcbblobs).ok()
    }
    pub unsafe fn GetNumTables(&self, pctables: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNumTables)(::windows::core::Interface::as_raw(self), pctables).ok()
    }
    pub unsafe fn GetTableIndex(&self, token: u32, pixtbl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTableIndex)(::windows::core::Interface::as_raw(self), token, pixtbl).ok()
    }
    pub unsafe fn GetTableInfo(&self, ixtbl: u32, pcbrow: *mut u32, pcrows: *mut u32, pccols: *mut u32, pikey: *mut u32, ppname: *const *const i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTableInfo)(::windows::core::Interface::as_raw(self), ixtbl, pcbrow, pcrows, pccols, pikey, ppname).ok()
    }
    pub unsafe fn GetColumnInfo(&self, ixtbl: u32, ixcol: u32, pocol: *mut u32, pcbcol: *mut u32, ptype: *mut u32, ppname: *const *const i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColumnInfo)(::windows::core::Interface::as_raw(self), ixtbl, ixcol, pocol, pcbcol, ptype, ppname).ok()
    }
    pub unsafe fn GetCodedTokenInfo(&self, ixcdtkn: u32, pctokens: *mut u32, pptokens: *mut *mut u32, ppname: *const *const i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodedTokenInfo)(::windows::core::Interface::as_raw(self), ixcdtkn, pctokens, pptokens, ppname).ok()
    }
    pub unsafe fn GetRow(&self, ixtbl: u32, rid: u32, pprow: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRow)(::windows::core::Interface::as_raw(self), ixtbl, rid, pprow).ok()
    }
    pub unsafe fn GetColumn(&self, ixtbl: u32, ixcol: u32, rid: u32, pval: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColumn)(::windows::core::Interface::as_raw(self), ixtbl, ixcol, rid, pval).ok()
    }
    pub unsafe fn GetString(&self, ixstring: u32, ppstring: *const *const i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetString)(::windows::core::Interface::as_raw(self), ixstring, ppstring).ok()
    }
    pub unsafe fn GetBlob(&self, ixblob: u32, pcbdata: *mut u32, ppdata: *const *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBlob)(::windows::core::Interface::as_raw(self), ixblob, pcbdata, ppdata).ok()
    }
    pub unsafe fn GetGuid(&self, ixguid: u32, ppguid: *const *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGuid)(::windows::core::Interface::as_raw(self), ixguid, ppguid).ok()
    }
    pub unsafe fn GetUserString(&self, ixuserstring: u32, pcbdata: *mut u32, ppdata: *const *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUserString)(::windows::core::Interface::as_raw(self), ixuserstring, pcbdata, ppdata).ok()
    }
    pub unsafe fn GetNextString(&self, ixstring: u32, pnext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextString)(::windows::core::Interface::as_raw(self), ixstring, pnext).ok()
    }
    pub unsafe fn GetNextBlob(&self, ixblob: u32, pnext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextBlob)(::windows::core::Interface::as_raw(self), ixblob, pnext).ok()
    }
    pub unsafe fn GetNextGuid(&self, ixguid: u32, pnext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextGuid)(::windows::core::Interface::as_raw(self), ixguid, pnext).ok()
    }
    pub unsafe fn GetNextUserString(&self, ixuserstring: u32, pnext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextUserString)(::windows::core::Interface::as_raw(self), ixuserstring, pnext).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataTables, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataTables {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataTables {}
impl ::core::fmt::Debug for IMetaDataTables {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataTables").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataTables {
    type Vtable = IMetaDataTables_Vtbl;
}
impl ::core::clone::Clone for IMetaDataTables {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataTables {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f579ab_402d_4b8e_82d9_5d63b1065c68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataTables_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStringHeapSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbstrings: *mut u32) -> ::windows::core::HRESULT,
    pub GetBlobHeapSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbblobs: *mut u32) -> ::windows::core::HRESULT,
    pub GetGuidHeapSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbguids: *mut u32) -> ::windows::core::HRESULT,
    pub GetUserStringHeapSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbblobs: *mut u32) -> ::windows::core::HRESULT,
    pub GetNumTables: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctables: *mut u32) -> ::windows::core::HRESULT,
    pub GetTableIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: u32, pixtbl: *mut u32) -> ::windows::core::HRESULT,
    pub GetTableInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixtbl: u32, pcbrow: *mut u32, pcrows: *mut u32, pccols: *mut u32, pikey: *mut u32, ppname: *const *const i8) -> ::windows::core::HRESULT,
    pub GetColumnInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixtbl: u32, ixcol: u32, pocol: *mut u32, pcbcol: *mut u32, ptype: *mut u32, ppname: *const *const i8) -> ::windows::core::HRESULT,
    pub GetCodedTokenInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixcdtkn: u32, pctokens: *mut u32, pptokens: *mut *mut u32, ppname: *const *const i8) -> ::windows::core::HRESULT,
    pub GetRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixtbl: u32, rid: u32, pprow: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixtbl: u32, ixcol: u32, rid: u32, pval: *mut u32) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixstring: u32, ppstring: *const *const i8) -> ::windows::core::HRESULT,
    pub GetBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixblob: u32, pcbdata: *mut u32, ppdata: *const *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixguid: u32, ppguid: *const *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetUserString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixuserstring: u32, pcbdata: *mut u32, ppdata: *const *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNextString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixstring: u32, pnext: *mut u32) -> ::windows::core::HRESULT,
    pub GetNextBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixblob: u32, pnext: *mut u32) -> ::windows::core::HRESULT,
    pub GetNextGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixguid: u32, pnext: *mut u32) -> ::windows::core::HRESULT,
    pub GetNextUserString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ixuserstring: u32, pnext: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataTables2(::windows::core::IUnknown);
impl IMetaDataTables2 {
    pub unsafe fn GetStringHeapSize(&self, pcbstrings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStringHeapSize)(::windows::core::Interface::as_raw(self), pcbstrings).ok()
    }
    pub unsafe fn GetBlobHeapSize(&self, pcbblobs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBlobHeapSize)(::windows::core::Interface::as_raw(self), pcbblobs).ok()
    }
    pub unsafe fn GetGuidHeapSize(&self, pcbguids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetGuidHeapSize)(::windows::core::Interface::as_raw(self), pcbguids).ok()
    }
    pub unsafe fn GetUserStringHeapSize(&self, pcbblobs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetUserStringHeapSize)(::windows::core::Interface::as_raw(self), pcbblobs).ok()
    }
    pub unsafe fn GetNumTables(&self, pctables: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNumTables)(::windows::core::Interface::as_raw(self), pctables).ok()
    }
    pub unsafe fn GetTableIndex(&self, token: u32, pixtbl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTableIndex)(::windows::core::Interface::as_raw(self), token, pixtbl).ok()
    }
    pub unsafe fn GetTableInfo(&self, ixtbl: u32, pcbrow: *mut u32, pcrows: *mut u32, pccols: *mut u32, pikey: *mut u32, ppname: *const *const i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTableInfo)(::windows::core::Interface::as_raw(self), ixtbl, pcbrow, pcrows, pccols, pikey, ppname).ok()
    }
    pub unsafe fn GetColumnInfo(&self, ixtbl: u32, ixcol: u32, pocol: *mut u32, pcbcol: *mut u32, ptype: *mut u32, ppname: *const *const i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetColumnInfo)(::windows::core::Interface::as_raw(self), ixtbl, ixcol, pocol, pcbcol, ptype, ppname).ok()
    }
    pub unsafe fn GetCodedTokenInfo(&self, ixcdtkn: u32, pctokens: *mut u32, pptokens: *mut *mut u32, ppname: *const *const i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCodedTokenInfo)(::windows::core::Interface::as_raw(self), ixcdtkn, pctokens, pptokens, ppname).ok()
    }
    pub unsafe fn GetRow(&self, ixtbl: u32, rid: u32, pprow: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRow)(::windows::core::Interface::as_raw(self), ixtbl, rid, pprow).ok()
    }
    pub unsafe fn GetColumn(&self, ixtbl: u32, ixcol: u32, rid: u32, pval: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetColumn)(::windows::core::Interface::as_raw(self), ixtbl, ixcol, rid, pval).ok()
    }
    pub unsafe fn GetString(&self, ixstring: u32, ppstring: *const *const i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetString)(::windows::core::Interface::as_raw(self), ixstring, ppstring).ok()
    }
    pub unsafe fn GetBlob(&self, ixblob: u32, pcbdata: *mut u32, ppdata: *const *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBlob)(::windows::core::Interface::as_raw(self), ixblob, pcbdata, ppdata).ok()
    }
    pub unsafe fn GetGuid(&self, ixguid: u32, ppguid: *const *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetGuid)(::windows::core::Interface::as_raw(self), ixguid, ppguid).ok()
    }
    pub unsafe fn GetUserString(&self, ixuserstring: u32, pcbdata: *mut u32, ppdata: *const *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetUserString)(::windows::core::Interface::as_raw(self), ixuserstring, pcbdata, ppdata).ok()
    }
    pub unsafe fn GetNextString(&self, ixstring: u32, pnext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNextString)(::windows::core::Interface::as_raw(self), ixstring, pnext).ok()
    }
    pub unsafe fn GetNextBlob(&self, ixblob: u32, pnext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNextBlob)(::windows::core::Interface::as_raw(self), ixblob, pnext).ok()
    }
    pub unsafe fn GetNextGuid(&self, ixguid: u32, pnext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNextGuid)(::windows::core::Interface::as_raw(self), ixguid, pnext).ok()
    }
    pub unsafe fn GetNextUserString(&self, ixuserstring: u32, pnext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNextUserString)(::windows::core::Interface::as_raw(self), ixuserstring, pnext).ok()
    }
    pub unsafe fn GetMetaDataStorage(&self, ppvmd: *const *const ::core::ffi::c_void, pcbmd: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMetaDataStorage)(::windows::core::Interface::as_raw(self), ppvmd, pcbmd).ok()
    }
    pub unsafe fn GetMetaDataStreamInfo(&self, ix: u32, ppchname: *const *const i8, ppv: *const *const ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMetaDataStreamInfo)(::windows::core::Interface::as_raw(self), ix, ppchname, ppv, pcb).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataTables2, ::windows::core::IUnknown, IMetaDataTables);
impl ::core::cmp::PartialEq for IMetaDataTables2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataTables2 {}
impl ::core::fmt::Debug for IMetaDataTables2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataTables2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataTables2 {
    type Vtable = IMetaDataTables2_Vtbl;
}
impl ::core::clone::Clone for IMetaDataTables2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataTables2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbadb5f70_58da_43a9_a1c6_d74819f19b15);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataTables2_Vtbl {
    pub base__: IMetaDataTables_Vtbl,
    pub GetMetaDataStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvmd: *const *const ::core::ffi::c_void, pcbmd: *mut u32) -> ::windows::core::HRESULT,
    pub GetMetaDataStreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ix: u32, ppchname: *const *const i8, ppv: *const *const ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataValidate(::windows::core::IUnknown);
impl IMetaDataValidate {
    pub unsafe fn ValidatorInit<P0>(&self, dwmoduletype: u32, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).ValidatorInit)(::windows::core::Interface::as_raw(self), dwmoduletype, punk.into_param().abi()).ok()
    }
    pub unsafe fn ValidateMetaData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ValidateMetaData)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataValidate, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataValidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataValidate {}
impl ::core::fmt::Debug for IMetaDataValidate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataValidate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataValidate {
    type Vtable = IMetaDataValidate_Vtbl;
}
impl ::core::clone::Clone for IMetaDataValidate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataValidate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4709c9c6_81ff_11d3_9fc7_00c04f79a0a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataValidate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ValidatorInit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmoduletype: u32, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ValidateMetaData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IMetaDataWinMDImport(::windows::core::IUnknown);
impl IMetaDataWinMDImport {
    pub unsafe fn GetUntransformedTypeRefProps(&self, tr: u32, ptkresolutionscope: *mut u32, szname: ::core::option::Option<&mut [u16]>, pchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUntransformedTypeRefProps)(::windows::core::Interface::as_raw(self), tr, ptkresolutionscope, ::core::mem::transmute(szname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len() as _), pchname).ok()
    }
}
::windows::imp::interface_hierarchy!(IMetaDataWinMDImport, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMetaDataWinMDImport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaDataWinMDImport {}
impl ::core::fmt::Debug for IMetaDataWinMDImport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaDataWinMDImport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMetaDataWinMDImport {
    type Vtable = IMetaDataWinMDImport_Vtbl;
}
impl ::core::clone::Clone for IMetaDataWinMDImport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMetaDataWinMDImport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x969ea0c5_964e_411b_a807_b0f3c2dfcbd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaDataWinMDImport_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUntransformedTypeRefProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tr: u32, ptkresolutionscope: *mut u32, szname: ::windows::core::PWSTR, cchname: u32, pchname: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IRoMetaDataLocator(::std::ptr::NonNull<::std::ffi::c_void>);
impl IRoMetaDataLocator {
    pub unsafe fn Locate<P0, P1>(&self, nameelement: P0, metadatadestination: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IRoSimpleMetaDataBuilder>,
    {
        (::windows::core::Interface::vtable(self).Locate)(::windows::core::Interface::as_raw(self), nameelement.into_param().abi(), metadatadestination.into_param().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IRoMetaDataLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRoMetaDataLocator {}
impl ::core::fmt::Debug for IRoMetaDataLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRoMetaDataLocator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRoMetaDataLocator {
    type Vtable = IRoMetaDataLocator_Vtbl;
}
impl ::core::clone::Clone for IRoMetaDataLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoMetaDataLocator_Vtbl {
    pub Locate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nameelement: ::windows::core::PCWSTR, metadatadestination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
pub struct IRoSimpleMetaDataBuilder(::std::ptr::NonNull<::std::ffi::c_void>);
impl IRoSimpleMetaDataBuilder {
    pub unsafe fn SetWinRtInterface(&self, iid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWinRtInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid)).ok()
    }
    pub unsafe fn SetDelegate(&self, iid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDelegate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid)).ok()
    }
    pub unsafe fn SetInterfaceGroupSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetInterfaceGroupSimpleDefault)(::windows::core::Interface::as_raw(self), name.into_param().abi(), defaultinterfacename.into_param().abi(), ::core::mem::transmute(defaultinterfaceiid.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetInterfaceGroupParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[::windows::core::PCWSTR]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetInterfaceGroupParameterizedDefault)(::windows::core::Interface::as_raw(self), name.into_param().abi(), defaultinterfacenameelements.len() as _, ::core::mem::transmute(defaultinterfacenameelements.as_ptr())).ok()
    }
    pub unsafe fn SetRuntimeClassSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRuntimeClassSimpleDefault)(::windows::core::Interface::as_raw(self), name.into_param().abi(), defaultinterfacename.into_param().abi(), ::core::mem::transmute(defaultinterfaceiid.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetRuntimeClassParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[::windows::core::PCWSTR]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRuntimeClassParameterizedDefault)(::windows::core::Interface::as_raw(self), name.into_param().abi(), defaultinterfacenameelements.len() as _, ::core::mem::transmute(defaultinterfacenameelements.as_ptr())).ok()
    }
    pub unsafe fn SetStruct<P0>(&self, name: P0, fieldtypenames: &[::windows::core::PCWSTR]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetStruct)(::windows::core::Interface::as_raw(self), name.into_param().abi(), fieldtypenames.len() as _, ::core::mem::transmute(fieldtypenames.as_ptr())).ok()
    }
    pub unsafe fn SetEnum<P0, P1>(&self, name: P0, basetype: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetEnum)(::windows::core::Interface::as_raw(self), name.into_param().abi(), basetype.into_param().abi()).ok()
    }
    pub unsafe fn SetParameterizedInterface(&self, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParameterizedInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(piid), numargs).ok()
    }
    pub unsafe fn SetParameterizedDelegate(&self, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParameterizedDelegate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(piid), numargs).ok()
    }
}
impl ::core::cmp::PartialEq for IRoSimpleMetaDataBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRoSimpleMetaDataBuilder {}
impl ::core::fmt::Debug for IRoSimpleMetaDataBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRoSimpleMetaDataBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRoSimpleMetaDataBuilder {
    type Vtable = IRoSimpleMetaDataBuilder_Vtbl;
}
impl ::core::clone::Clone for IRoSimpleMetaDataBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoSimpleMetaDataBuilder_Vtbl {
    pub SetWinRtInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetDelegate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetInterfaceGroupSimpleDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, defaultinterfacename: ::windows::core::PCWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetInterfaceGroupParameterizedDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetRuntimeClassSimpleDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, defaultinterfacename: ::windows::core::PCWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetRuntimeClassParameterizedDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, numfields: u32, fieldtypenames: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, basetype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetParameterizedInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::HRESULT,
    pub SetParameterizedDelegate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ASSEMBLY_METADATA_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Reflection.AssemblyMetadataAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ASSEMBLY_METADATA_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Reflection.AssemblyMetadataAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CLSID_CLR_v1_MetaData: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x005023ca_72b1_11d3_9fc4_00c04f79a0a3);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CLSID_CLR_v2_MetaData: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefea471a_44fd_4862_9292_0c58d46e1f3a);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CLSID_Cor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbee00010_ee77_11d0_a015_00c04fbbb884);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CLSID_CorMetaDataDispenser: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5cb7a31_7512_11d2_89ce_0080c792e5d8);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CLSID_CorMetaDataDispenserReg: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x435755ff_7397_11d2_9771_00a0c9b4d50c);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CLSID_CorMetaDataDispenserRuntime: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ec2de53_75cc_11d2_9775_00a0c9b4d50c);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CLSID_CorMetaDataReg: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87f3a1f5_7397_11d2_9771_00a0c9b4d50c);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CMOD_CALLCONV_NAMESPACE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CMOD_CALLCONV_NAMESPACE_OLD: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CMOD_CALLCONV_NAME_CDECL: ::windows::core::PCSTR = ::windows::core::s!("CallConvCdecl");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CMOD_CALLCONV_NAME_FASTCALL: ::windows::core::PCSTR = ::windows::core::s!("CallConvFastcall");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CMOD_CALLCONV_NAME_STDCALL: ::windows::core::PCSTR = ::windows::core::s!("CallConvStdcall");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CMOD_CALLCONV_NAME_THISCALL: ::windows::core::PCSTR = ::windows::core::s!("CallConvThiscall");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COMPILATIONRELAXATIONS_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.CompilationRelaxationsAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COMPILATIONRELAXATIONS_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.CompilationRelaxationsAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_BASE_SECURITY_ATTRIBUTE_CLASS: ::windows::core::PCWSTR = ::windows::core::w!("System.Security.Permissions.SecurityAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_BASE_SECURITY_ATTRIBUTE_CLASS_ANSI: ::windows::core::PCSTR = ::windows::core::s!("System.Security.Permissions.SecurityAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_CCTOR_METHOD_NAME: ::windows::core::PCSTR = ::windows::core::s!(".cctor");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_CCTOR_METHOD_NAME_W: ::windows::core::PCWSTR = ::windows::core::w!(".cctor");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_COMPILERSERVICE_DISCARDABLEATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.DiscardableAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_COMPILERSERVICE_DISCARDABLEATTRIBUTE_ASNI: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.DiscardableAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_CTOR_METHOD_NAME: ::windows::core::PCSTR = ::windows::core::s!(".ctor");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_CTOR_METHOD_NAME_W: ::windows::core::PCWSTR = ::windows::core::w!(".ctor");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_DELETED_NAME_A: ::windows::core::PCSTR = ::windows::core::s!("_Deleted");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_DELETED_NAME_W: ::windows::core::PCWSTR = ::windows::core::w!("_Deleted");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_ENUM_FIELD_NAME: ::windows::core::PCSTR = ::windows::core::s!("value__");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_ENUM_FIELD_NAME_W: ::windows::core::PCWSTR = ::windows::core::w!("value__");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_E_AMBIGUOUSMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147475171i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_E_ARGUMENT: i32 = -2147024809i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_E_BADIMAGEFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024885i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_E_DIVIDEBYZERO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147352558i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_E_INVALIDCAST: i32 = -2147467262i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_E_NULLREFERENCE: i32 = -2147467261i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_E_OUTOFMEMORY: i32 = -2147024882i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_E_TARGETPARAMCOUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147352562i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_E_UNAUTHORIZEDACCESS: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_NATIVE_LINK_CUSTOM_VALUE: ::windows::core::PCWSTR = ::windows::core::w!("COMPLUS_NativeLink");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_NATIVE_LINK_CUSTOM_VALUE_ANSI: ::windows::core::PCSTR = ::windows::core::s!("COMPLUS_NativeLink");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_NATIVE_LINK_CUSTOM_VALUE_CC: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_REQUIRES_SECOBJ_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("System.Security.DynamicSecurityMethodAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_REQUIRES_SECOBJ_ATTRIBUTE_ANSI: ::windows::core::PCSTR = ::windows::core::s!("System.Security.DynamicSecurityMethodAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_SUPPRESS_UNMANAGED_CODE_CHECK_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("System.Security.SuppressUnmanagedCodeSecurityAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_SUPPRESS_UNMANAGED_CODE_CHECK_ATTRIBUTE_ANSI: ::windows::core::PCSTR = ::windows::core::s!("System.Security.SuppressUnmanagedCodeSecurityAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_UNVER_CODE_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("System.Security.UnverifiableCodeAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_UNVER_CODE_ATTRIBUTE_ANSI: ::windows::core::PCSTR = ::windows::core::s!("System.Security.UnverifiableCodeAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_VTABLEGAP_NAME_A: ::windows::core::PCSTR = ::windows::core::s!("_VtblGap");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_VTABLEGAP_NAME_W: ::windows::core::PCWSTR = ::windows::core::w!("_VtblGap");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DEFAULTDEPENDENCY_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.DefaultDependencyAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DEFAULTDEPENDENCY_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.DefaultDependencyAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DEFAULTDOMAIN_LOADEROPTIMIZATION_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.LoaderOptimizationAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DEFAULTDOMAIN_LOADEROPTIMIZATION_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.LoaderOptimizationAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DEFAULTDOMAIN_MTA_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.MTAThreadAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DEFAULTDOMAIN_MTA_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.MTAThreadAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DEFAULTDOMAIN_STA_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.STAThreadAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DEFAULTDOMAIN_STA_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.STAThreadAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DEPENDENCY_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.DependencyAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DEPENDENCY_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.DependencyAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DESCR_GROUP_METHODDEF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DESCR_GROUP_METHODIMPL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DISABLED_PRIVATE_REFLECTION_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.DisablePrivateReflectionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DISABLED_PRIVATE_REFLECTION_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.DisablePrivateReflectionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const FORWARD_INTEROP_STUB_METHOD_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ManagedToNativeComInteropStubAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const FORWARD_INTEROP_STUB_METHOD_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ManagedToNativeComInteropStubAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const FRAMEWORK_REGISTRY_KEY: ::windows::core::PCSTR = ::windows::core::s!("Software\\Microsoft\\.NETFramework");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const FRAMEWORK_REGISTRY_KEY_W: ::windows::core::PCWSTR = ::windows::core::w!("Software\\Microsoft\\.NETFramework");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const FRIEND_ACCESS_ALLOWED_ATTRIBUTE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.FriendAccessAllowedAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const FRIEND_ACCESS_ALLOWED_ATTRIBUTE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.FriendAccessAllowedAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const FRIEND_ASSEMBLY_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.InternalsVisibleToAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const FRIEND_ASSEMBLY_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.InternalsVisibleToAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const GUID_DispIdOverride: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd2bc5c9_f452_4326_b714_f9c539d4da58);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const GUID_ExportedFromComPlus: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90883f05_3d28_11d2_8f17_00a0c9a6186d);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const GUID_ForceIEnumerable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb64784eb_d8d4_4d9b_9acd_0e30806426f7);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const GUID_Function2Getter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54fc8f55_38de_4703_9c4e_250351302b1c);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const GUID_ManagedName: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f21f359_ab84_41e8_9a78_36d110e6d2f9);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const GUID_PropGetCA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2941ff83_88d8_4f73_b6a9_bdf8712d000d);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const GUID_PropPutCA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29533527_3683_4364_abc0_db1add822fa2);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_AUTOPROXY_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.AutomationProxyAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_AUTOPROXY_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.AutomationProxyAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_BESTFITMAPPING_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.BestFitMappingAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_BESTFITMAPPING_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.BestFitMappingAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_CLASSINTERFACE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ClassInterfaceAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_CLASSINTERFACE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ClassInterfaceAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COCLASS_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.CoClassAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COCLASS_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.CoClassAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMALIASNAME_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComAliasNameAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMALIASNAME_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComAliasNameAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMCOMPATIBLEVERSION_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComCompatibleVersionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMCOMPATIBLEVERSION_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComCompatibleVersionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMCONVERSIONLOSS_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComConversionLossAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMCONVERSIONLOSS_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComConversionLossAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMDEFAULTINTERFACE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComDefaultInterfaceAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMDEFAULTINTERFACE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComDefaultInterfaceAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMEMULATE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComEmulateAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMEMULATE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComEmulateAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMEVENTINTERFACE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComEventInterfaceAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMEVENTINTERFACE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComEventInterfaceAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMIMPORT_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComImportAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMIMPORT_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComImportAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMREGISTERFUNCTION_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComRegisterFunctionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMREGISTERFUNCTION_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComRegisterFunctionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMSOURCEINTERFACES_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComSourceInterfacesAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMSOURCEINTERFACES_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComSourceInterfacesAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMSUBSTITUTABLEINTERFACE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComSubstitutableInterfaceAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMSUBSTITUTABLEINTERFACE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComSubstitutableInterfaceAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMUNREGISTERFUNCTION_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComUnregisterFunctionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMUNREGISTERFUNCTION_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComUnregisterFunctionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMVISIBLE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ComVisibleAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_COMVISIBLE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ComVisibleAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_DATETIMEVALUE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.DateTimeConstantAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_DATETIMEVALUE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.DateTimeConstantAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_DECIMALVALUE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.DecimalConstantAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_DECIMALVALUE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.DecimalConstantAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_DEFAULTMEMBER_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Reflection.DefaultMemberAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_DEFAULTMEMBER_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Reflection.DefaultMemberAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_DISPID_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.DispIdAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_DISPID_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.DispIdAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_GUID_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.GuidAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_GUID_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.GuidAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_IDISPATCHIMPL_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.IDispatchImplAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_IDISPATCHIMPL_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.IDispatchImplAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_IDISPATCHVALUE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.IDispatchConstantAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_IDISPATCHVALUE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.IDispatchConstantAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_IMPORTEDFROMTYPELIB_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.ImportedFromTypeLibAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_IMPORTEDFROMTYPELIB_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.ImportedFromTypeLibAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_INTERFACETYPE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.InterfaceTypeAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_INTERFACETYPE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.InterfaceTypeAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_IN_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.InAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_IN_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.InAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_IUNKNOWNVALUE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.IUnknownConstantAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_IUNKNOWNVALUE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.IUnknownConstantAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_LCIDCONVERSION_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.LCIDConversionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_LCIDCONVERSION_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.LCIDConversionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_MARSHALAS_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.MarshalAsAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_MARSHALAS_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.MarshalAsAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_OUT_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.OutAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_OUT_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.OutAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_PARAMARRAY_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.ParamArrayAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_PARAMARRAY_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.ParamArrayAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_PRESERVESIG_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.PreserveSigAttribure");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_PRESERVESIG_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.PreserveSigAttribure");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_PRIMARYINTEROPASSEMBLY_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.PrimaryInteropAssemblyAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_PRIMARYINTEROPASSEMBLY_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.PrimaryInteropAssemblyAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_SERIALIZABLE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.SerializableAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_SERIALIZABLE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.SerializableAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_SETWIN32CONTEXTINIDISPATCHATTRIBUTE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.SetWin32ContextInIDispatchAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_SETWIN32CONTEXTINIDISPATCHATTRIBUTE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.SetWin32ContextInIDispatchAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_TYPELIBFUNC_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.TypeLibFuncAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_TYPELIBFUNC_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.TypeLibFuncAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_TYPELIBIMPORTCLASS_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.TypeLibImportClassAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_TYPELIBIMPORTCLASS_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.TypeLibImportClassAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_TYPELIBTYPE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.TypeLibTypeAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_TYPELIBTYPE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.TypeLibTypeAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_TYPELIBVAR_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.TypeLibVarAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_TYPELIBVAR_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.TypeLibVarAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_TYPELIBVERSION_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.InteropServices.TypeLibVersionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INTEROP_TYPELIBVERSION_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.InteropServices.TypeLibVersionAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INVALID_CONNECTION_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const INVALID_TASK_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const LIBID_ComPlusRuntime: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbed7f4ea_1a96_11d2_8f08_00a0c9a6186d);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MAIN_CLR_MODULE_NAME_A: ::windows::core::PCSTR = ::windows::core::s!("coreclr");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MAIN_CLR_MODULE_NAME_W: ::windows::core::PCWSTR = ::windows::core::w!("coreclr");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MAX_CONNECTION_NAME: u32 = 260u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MSCOREE_SHIM_A: ::windows::core::PCSTR = ::windows::core::s!("mscoree.dll");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MSCOREE_SHIM_W: ::windows::core::PCWSTR = ::windows::core::w!("mscoree.dll");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataCheckDuplicatesFor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30fe7be8_d7d9_11d2_9f80_00c04f79a0a3);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataErrorIfEmitOutOfOrder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1547872d_dc03_11d2_9420_0000f8083460);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataGenerateTCEAdapters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcc9de90_4151_11d3_88d6_00902754c43a);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataImportOption: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79700f36_4aac_11d3_84c3_009027868cb1);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataLinkerOptions: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47e099b6_ae7c_4797_8317_b48aa645b8f9);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataMergerOptions: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x132d3a6e_b35d_464e_951a_42efb9fb6601);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataNotificationForTokenMovement: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5d71a4c_d7da_11d2_9f80_00c04f79a0a3);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataPreserveLocalRefs: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa55c0354_e91b_468b_8648_7cc31035d533);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataRefToDefCheck: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde3856f8_d7d9_11d2_9f80_00c04f79a0a3);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataRuntimeVersion: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47e099b7_ae7c_4797_8317_b48aa645b8f9);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataSetUpdate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2eee315c_d7db_11d2_9f80_00c04f79a0a3);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataThreadSafetyOptions: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7559806_f266_42ea_8c63_0adb45e8b234);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MetaDataTypeLibImportNamespace: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf17ff889_5a63_11d3_9ff2_00c04ff7431a);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NONVERSIONABLE_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.Versioning.NonVersionableAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NONVERSIONABLE_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.Versioning.NonVersionableAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const RUNTIMECOMPATIBILITY_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.RuntimeCompatibilityAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const RUNTIMECOMPATIBILITY_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.RuntimeCompatibilityAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SIGN_MASK_FOURBYTE: i32 = -268435456i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SIGN_MASK_ONEBYTE: i32 = -64i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SIGN_MASK_TWOBYTE: i32 = -8192i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SUBJECT_ASSEMBLY_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.CompilerServices.IgnoresAccessChecksToAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SUBJECT_ASSEMBLY_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.CompilerServices.IgnoresAccessChecksToAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const TARGET_FRAMEWORK_TYPE: ::windows::core::PCSTR = ::windows::core::s!("System.Runtime.Versioning.TargetFrameworkAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const TARGET_FRAMEWORK_TYPE_W: ::windows::core::PCWSTR = ::windows::core::w!("System.Runtime.Versioning.TargetFrameworkAttribute");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const USER_FRAMEWORK_REGISTRY_KEY: ::windows::core::PCSTR = ::windows::core::s!("Software\\Microsoft\\.NETFramework64");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const USER_FRAMEWORK_REGISTRY_KEY_W: ::windows::core::PCWSTR = ::windows::core::w!("Software\\Microsoft\\.NETFramework64");
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COINITICOR(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COINITCOR_DEFAULT: COINITICOR = COINITICOR(0i32);
impl ::core::marker::Copy for COINITICOR {}
impl ::core::clone::Clone for COINITICOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COINITICOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COINITICOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COINITICOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COINITICOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COINITIEE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COINITEE_DEFAULT: COINITIEE = COINITIEE(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COINITEE_DLL: COINITIEE = COINITIEE(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COINITEE_MAIN: COINITIEE = COINITIEE(2i32);
impl ::core::marker::Copy for COINITIEE {}
impl ::core::clone::Clone for COINITIEE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COINITIEE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COINITIEE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COINITIEE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COINITIEE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COUNINITIEE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COUNINITEE_DEFAULT: COUNINITIEE = COUNINITIEE(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COUNINITEE_DLL: COUNINITIEE = COUNINITIEE(1i32);
impl ::core::marker::Copy for COUNINITIEE {}
impl ::core::clone::Clone for COUNINITIEE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COUNINITIEE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COUNINITIEE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COUNINITIEE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COUNINITIEE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CeeSectionAttr(pub i64);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const sdNone: CeeSectionAttr = CeeSectionAttr(0i64);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const sdReadOnly: CeeSectionAttr = CeeSectionAttr(1073741888i64);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const sdReadWrite: CeeSectionAttr = CeeSectionAttr(3221225536i64);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const sdExecute: CeeSectionAttr = CeeSectionAttr(1610612768i64);
impl ::core::marker::Copy for CeeSectionAttr {}
impl ::core::clone::Clone for CeeSectionAttr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CeeSectionAttr {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CeeSectionAttr {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CeeSectionAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CeeSectionAttr").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CeeSectionRelocType(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocAbsolute: CeeSectionRelocType = CeeSectionRelocType(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocHighLow: CeeSectionRelocType = CeeSectionRelocType(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocHighAdj: CeeSectionRelocType = CeeSectionRelocType(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocMapToken: CeeSectionRelocType = CeeSectionRelocType(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocRelative: CeeSectionRelocType = CeeSectionRelocType(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocFilePos: CeeSectionRelocType = CeeSectionRelocType(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocCodeRelative: CeeSectionRelocType = CeeSectionRelocType(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocIA64Imm64: CeeSectionRelocType = CeeSectionRelocType(9i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocDir64: CeeSectionRelocType = CeeSectionRelocType(10i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocIA64PcRel25: CeeSectionRelocType = CeeSectionRelocType(11i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocIA64PcRel64: CeeSectionRelocType = CeeSectionRelocType(12i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocAbsoluteTagged: CeeSectionRelocType = CeeSectionRelocType(13i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocSentinel: CeeSectionRelocType = CeeSectionRelocType(14i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srNoBaseReloc: CeeSectionRelocType = CeeSectionRelocType(16384i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocPtr: CeeSectionRelocType = CeeSectionRelocType(32768i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocAbsolutePtr: CeeSectionRelocType = CeeSectionRelocType(32768i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocHighLowPtr: CeeSectionRelocType = CeeSectionRelocType(32771i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocRelativePtr: CeeSectionRelocType = CeeSectionRelocType(32774i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocIA64Imm64Ptr: CeeSectionRelocType = CeeSectionRelocType(32777i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const srRelocDir64Ptr: CeeSectionRelocType = CeeSectionRelocType(32778i32);
impl ::core::marker::Copy for CeeSectionRelocType {}
impl ::core::clone::Clone for CeeSectionRelocType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CeeSectionRelocType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CeeSectionRelocType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CeeSectionRelocType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CeeSectionRelocType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CompilationRelaxationsEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CompilationRelaxations_NoStringInterning: CompilationRelaxationsEnum = CompilationRelaxationsEnum(8i32);
impl ::core::marker::Copy for CompilationRelaxationsEnum {}
impl ::core::clone::Clone for CompilationRelaxationsEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CompilationRelaxationsEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CompilationRelaxationsEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CompilationRelaxationsEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompilationRelaxationsEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorArgType(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_END: CorArgType = CorArgType(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_VOID: CorArgType = CorArgType(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_I4: CorArgType = CorArgType(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_I8: CorArgType = CorArgType(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_R4: CorArgType = CorArgType(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_R8: CorArgType = CorArgType(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_PTR: CorArgType = CorArgType(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_OBJECT: CorArgType = CorArgType(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_STRUCT4: CorArgType = CorArgType(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_STRUCT32: CorArgType = CorArgType(9i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_BYVALUE: CorArgType = CorArgType(10i32);
impl ::core::marker::Copy for CorArgType {}
impl ::core::clone::Clone for CorArgType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorArgType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorArgType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorArgType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorArgType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorAssemblyFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPublicKey: CorAssemblyFlags = CorAssemblyFlags(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_None: CorAssemblyFlags = CorAssemblyFlags(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_MSIL: CorAssemblyFlags = CorAssemblyFlags(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_x86: CorAssemblyFlags = CorAssemblyFlags(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_IA64: CorAssemblyFlags = CorAssemblyFlags(48i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_AMD64: CorAssemblyFlags = CorAssemblyFlags(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_ARM: CorAssemblyFlags = CorAssemblyFlags(80i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_NoPlatform: CorAssemblyFlags = CorAssemblyFlags(112i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_Specified: CorAssemblyFlags = CorAssemblyFlags(128i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_Mask: CorAssemblyFlags = CorAssemblyFlags(112i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_FullMask: CorAssemblyFlags = CorAssemblyFlags(240i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afPA_Shift: CorAssemblyFlags = CorAssemblyFlags(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afEnableJITcompileTracking: CorAssemblyFlags = CorAssemblyFlags(32768i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afDisableJITcompileOptimizer: CorAssemblyFlags = CorAssemblyFlags(16384i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afRetargetable: CorAssemblyFlags = CorAssemblyFlags(256i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afContentType_Default: CorAssemblyFlags = CorAssemblyFlags(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afContentType_WindowsRuntime: CorAssemblyFlags = CorAssemblyFlags(512i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const afContentType_Mask: CorAssemblyFlags = CorAssemblyFlags(3584i32);
impl ::core::marker::Copy for CorAssemblyFlags {}
impl ::core::clone::Clone for CorAssemblyFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorAssemblyFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorAssemblyFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorAssemblyFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorAssemblyFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorAttributeTargets(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catAssembly: CorAttributeTargets = CorAttributeTargets(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catModule: CorAttributeTargets = CorAttributeTargets(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catClass: CorAttributeTargets = CorAttributeTargets(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catStruct: CorAttributeTargets = CorAttributeTargets(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catEnum: CorAttributeTargets = CorAttributeTargets(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catConstructor: CorAttributeTargets = CorAttributeTargets(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catMethod: CorAttributeTargets = CorAttributeTargets(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catProperty: CorAttributeTargets = CorAttributeTargets(128i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catField: CorAttributeTargets = CorAttributeTargets(256i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catEvent: CorAttributeTargets = CorAttributeTargets(512i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catInterface: CorAttributeTargets = CorAttributeTargets(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catParameter: CorAttributeTargets = CorAttributeTargets(2048i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catDelegate: CorAttributeTargets = CorAttributeTargets(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catGenericParameter: CorAttributeTargets = CorAttributeTargets(16384i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catAll: CorAttributeTargets = CorAttributeTargets(24575i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const catClassMembers: CorAttributeTargets = CorAttributeTargets(6140i32);
impl ::core::marker::Copy for CorAttributeTargets {}
impl ::core::clone::Clone for CorAttributeTargets {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorAttributeTargets {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorAttributeTargets {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorAttributeTargets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorAttributeTargets").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorCallingConvention(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_DEFAULT: CorCallingConvention = CorCallingConvention(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_VARARG: CorCallingConvention = CorCallingConvention(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_FIELD: CorCallingConvention = CorCallingConvention(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_LOCAL_SIG: CorCallingConvention = CorCallingConvention(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_PROPERTY: CorCallingConvention = CorCallingConvention(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_UNMGD: CorCallingConvention = CorCallingConvention(9i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_GENERICINST: CorCallingConvention = CorCallingConvention(10i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_NATIVEVARARG: CorCallingConvention = CorCallingConvention(11i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_MAX: CorCallingConvention = CorCallingConvention(12i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_MASK: CorCallingConvention = CorCallingConvention(15i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_HASTHIS: CorCallingConvention = CorCallingConvention(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_EXPLICITTHIS: CorCallingConvention = CorCallingConvention(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_GENERIC: CorCallingConvention = CorCallingConvention(16i32);
impl ::core::marker::Copy for CorCallingConvention {}
impl ::core::clone::Clone for CorCallingConvention {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorCallingConvention {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorCallingConvention {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorCallingConvention {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorCallingConvention").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorCheckDuplicatesFor(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupAll: CorCheckDuplicatesFor = CorCheckDuplicatesFor(-1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupENC: CorCheckDuplicatesFor = CorCheckDuplicatesFor(-1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNoDupChecks: CorCheckDuplicatesFor = CorCheckDuplicatesFor(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupTypeDef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupInterfaceImpl: CorCheckDuplicatesFor = CorCheckDuplicatesFor(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupMethodDef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupTypeRef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupMemberRef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupCustomAttribute: CorCheckDuplicatesFor = CorCheckDuplicatesFor(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupParamDef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupPermission: CorCheckDuplicatesFor = CorCheckDuplicatesFor(128i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupProperty: CorCheckDuplicatesFor = CorCheckDuplicatesFor(256i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupEvent: CorCheckDuplicatesFor = CorCheckDuplicatesFor(512i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupFieldDef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupSignature: CorCheckDuplicatesFor = CorCheckDuplicatesFor(2048i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupModuleRef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupTypeSpec: CorCheckDuplicatesFor = CorCheckDuplicatesFor(8192i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupImplMap: CorCheckDuplicatesFor = CorCheckDuplicatesFor(16384i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupAssemblyRef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(32768i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupFile: CorCheckDuplicatesFor = CorCheckDuplicatesFor(65536i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupExportedType: CorCheckDuplicatesFor = CorCheckDuplicatesFor(131072i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupManifestResource: CorCheckDuplicatesFor = CorCheckDuplicatesFor(262144i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupGenericParam: CorCheckDuplicatesFor = CorCheckDuplicatesFor(524288i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupMethodSpec: CorCheckDuplicatesFor = CorCheckDuplicatesFor(1048576i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupGenericParamConstraint: CorCheckDuplicatesFor = CorCheckDuplicatesFor(2097152i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupAssembly: CorCheckDuplicatesFor = CorCheckDuplicatesFor(268435456i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDDupDefault: CorCheckDuplicatesFor = CorCheckDuplicatesFor(1058840i32);
impl ::core::marker::Copy for CorCheckDuplicatesFor {}
impl ::core::clone::Clone for CorCheckDuplicatesFor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorCheckDuplicatesFor {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorCheckDuplicatesFor {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorCheckDuplicatesFor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorCheckDuplicatesFor").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorDeclSecurity(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclActionMask: CorDeclSecurity = CorDeclSecurity(31i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclActionNil: CorDeclSecurity = CorDeclSecurity(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclRequest: CorDeclSecurity = CorDeclSecurity(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclDemand: CorDeclSecurity = CorDeclSecurity(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclAssert: CorDeclSecurity = CorDeclSecurity(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclDeny: CorDeclSecurity = CorDeclSecurity(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclPermitOnly: CorDeclSecurity = CorDeclSecurity(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclLinktimeCheck: CorDeclSecurity = CorDeclSecurity(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclInheritanceCheck: CorDeclSecurity = CorDeclSecurity(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclRequestMinimum: CorDeclSecurity = CorDeclSecurity(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclRequestOptional: CorDeclSecurity = CorDeclSecurity(9i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclRequestRefuse: CorDeclSecurity = CorDeclSecurity(10i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclPrejitGrant: CorDeclSecurity = CorDeclSecurity(11i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclPrejitDenied: CorDeclSecurity = CorDeclSecurity(12i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclNonCasDemand: CorDeclSecurity = CorDeclSecurity(13i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclNonCasLinkDemand: CorDeclSecurity = CorDeclSecurity(14i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclNonCasInheritance: CorDeclSecurity = CorDeclSecurity(15i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const dclMaximumValue: CorDeclSecurity = CorDeclSecurity(15i32);
impl ::core::marker::Copy for CorDeclSecurity {}
impl ::core::clone::Clone for CorDeclSecurity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorDeclSecurity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorDeclSecurity {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorDeclSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorDeclSecurity").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorElementType(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_END: CorElementType = CorElementType(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_VOID: CorElementType = CorElementType(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_BOOLEAN: CorElementType = CorElementType(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_CHAR: CorElementType = CorElementType(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_I1: CorElementType = CorElementType(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_U1: CorElementType = CorElementType(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_I2: CorElementType = CorElementType(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_U2: CorElementType = CorElementType(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_I4: CorElementType = CorElementType(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_U4: CorElementType = CorElementType(9i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_I8: CorElementType = CorElementType(10i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_U8: CorElementType = CorElementType(11i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_R4: CorElementType = CorElementType(12i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_R8: CorElementType = CorElementType(13i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_STRING: CorElementType = CorElementType(14i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_PTR: CorElementType = CorElementType(15i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_BYREF: CorElementType = CorElementType(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_VALUETYPE: CorElementType = CorElementType(17i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_CLASS: CorElementType = CorElementType(18i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_VAR: CorElementType = CorElementType(19i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_ARRAY: CorElementType = CorElementType(20i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_GENERICINST: CorElementType = CorElementType(21i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_TYPEDBYREF: CorElementType = CorElementType(22i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_I: CorElementType = CorElementType(24i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_U: CorElementType = CorElementType(25i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_FNPTR: CorElementType = CorElementType(27i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_OBJECT: CorElementType = CorElementType(28i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_SZARRAY: CorElementType = CorElementType(29i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_MVAR: CorElementType = CorElementType(30i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_CMOD_REQD: CorElementType = CorElementType(31i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_CMOD_OPT: CorElementType = CorElementType(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_INTERNAL: CorElementType = CorElementType(33i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_MAX: CorElementType = CorElementType(34i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_MODIFIER: CorElementType = CorElementType(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_SENTINEL: CorElementType = CorElementType(65i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ELEMENT_TYPE_PINNED: CorElementType = CorElementType(69i32);
impl ::core::marker::Copy for CorElementType {}
impl ::core::clone::Clone for CorElementType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorElementType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorElementType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorElementType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorElementType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorErrorIfEmitOutOfOrder(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDErrorOutOfOrderDefault: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDErrorOutOfOrderNone: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDErrorOutOfOrderAll: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(-1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDMethodOutOfOrder: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDFieldOutOfOrder: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDParamOutOfOrder: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDPropertyOutOfOrder: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDEventOutOfOrder: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(16i32);
impl ::core::marker::Copy for CorErrorIfEmitOutOfOrder {}
impl ::core::clone::Clone for CorErrorIfEmitOutOfOrder {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorErrorIfEmitOutOfOrder {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorErrorIfEmitOutOfOrder {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorErrorIfEmitOutOfOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorErrorIfEmitOutOfOrder").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorEventAttr(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const evSpecialName: CorEventAttr = CorEventAttr(512i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const evReservedMask: CorEventAttr = CorEventAttr(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const evRTSpecialName: CorEventAttr = CorEventAttr(1024i32);
impl ::core::marker::Copy for CorEventAttr {}
impl ::core::clone::Clone for CorEventAttr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorEventAttr {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorEventAttr {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorEventAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorEventAttr").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorExceptionFlag(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_ILEXCEPTION_CLAUSE_NONE: CorExceptionFlag = CorExceptionFlag(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_ILEXCEPTION_CLAUSE_OFFSETLEN: CorExceptionFlag = CorExceptionFlag(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_ILEXCEPTION_CLAUSE_DEPRECATED: CorExceptionFlag = CorExceptionFlag(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_ILEXCEPTION_CLAUSE_FILTER: CorExceptionFlag = CorExceptionFlag(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_ILEXCEPTION_CLAUSE_FINALLY: CorExceptionFlag = CorExceptionFlag(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_ILEXCEPTION_CLAUSE_FAULT: CorExceptionFlag = CorExceptionFlag(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const COR_ILEXCEPTION_CLAUSE_DUPLICATED: CorExceptionFlag = CorExceptionFlag(8i32);
impl ::core::marker::Copy for CorExceptionFlag {}
impl ::core::clone::Clone for CorExceptionFlag {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorExceptionFlag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorExceptionFlag {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorExceptionFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorExceptionFlag").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorFieldAttr(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdFieldAccessMask: CorFieldAttr = CorFieldAttr(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdPrivateScope: CorFieldAttr = CorFieldAttr(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdPrivate: CorFieldAttr = CorFieldAttr(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdFamANDAssem: CorFieldAttr = CorFieldAttr(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdAssembly: CorFieldAttr = CorFieldAttr(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdFamily: CorFieldAttr = CorFieldAttr(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdFamORAssem: CorFieldAttr = CorFieldAttr(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdPublic: CorFieldAttr = CorFieldAttr(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdStatic: CorFieldAttr = CorFieldAttr(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdInitOnly: CorFieldAttr = CorFieldAttr(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdLiteral: CorFieldAttr = CorFieldAttr(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdNotSerialized: CorFieldAttr = CorFieldAttr(128i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdSpecialName: CorFieldAttr = CorFieldAttr(512i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdPinvokeImpl: CorFieldAttr = CorFieldAttr(8192i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdReservedMask: CorFieldAttr = CorFieldAttr(38144i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdRTSpecialName: CorFieldAttr = CorFieldAttr(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdHasFieldMarshal: CorFieldAttr = CorFieldAttr(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdHasDefault: CorFieldAttr = CorFieldAttr(32768i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fdHasFieldRVA: CorFieldAttr = CorFieldAttr(256i32);
impl ::core::marker::Copy for CorFieldAttr {}
impl ::core::clone::Clone for CorFieldAttr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorFieldAttr {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorFieldAttr {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorFieldAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorFieldAttr").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorFileFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ffContainsMetaData: CorFileFlags = CorFileFlags(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ffContainsNoMetaData: CorFileFlags = CorFileFlags(1i32);
impl ::core::marker::Copy for CorFileFlags {}
impl ::core::clone::Clone for CorFileFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorFileFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorFileFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorFileFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorFileFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorFileMapping(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fmFlat: CorFileMapping = CorFileMapping(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const fmExecutableImage: CorFileMapping = CorFileMapping(1i32);
impl ::core::marker::Copy for CorFileMapping {}
impl ::core::clone::Clone for CorFileMapping {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorFileMapping {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorFileMapping {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorFileMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorFileMapping").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorGenericParamAttr(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const gpVarianceMask: CorGenericParamAttr = CorGenericParamAttr(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const gpNonVariant: CorGenericParamAttr = CorGenericParamAttr(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const gpCovariant: CorGenericParamAttr = CorGenericParamAttr(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const gpContravariant: CorGenericParamAttr = CorGenericParamAttr(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const gpSpecialConstraintMask: CorGenericParamAttr = CorGenericParamAttr(28i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const gpNoSpecialConstraint: CorGenericParamAttr = CorGenericParamAttr(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const gpReferenceTypeConstraint: CorGenericParamAttr = CorGenericParamAttr(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const gpNotNullableValueTypeConstraint: CorGenericParamAttr = CorGenericParamAttr(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const gpDefaultConstructorConstraint: CorGenericParamAttr = CorGenericParamAttr(16i32);
impl ::core::marker::Copy for CorGenericParamAttr {}
impl ::core::clone::Clone for CorGenericParamAttr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorGenericParamAttr {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorGenericParamAttr {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorGenericParamAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorGenericParamAttr").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorILMethodFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_InitLocals: CorILMethodFlags = CorILMethodFlags(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_MoreSects: CorILMethodFlags = CorILMethodFlags(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_CompressedIL: CorILMethodFlags = CorILMethodFlags(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_FormatShift: CorILMethodFlags = CorILMethodFlags(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_FormatMask: CorILMethodFlags = CorILMethodFlags(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_TinyFormat: CorILMethodFlags = CorILMethodFlags(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_SmallFormat: CorILMethodFlags = CorILMethodFlags(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_FatFormat: CorILMethodFlags = CorILMethodFlags(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_TinyFormat1: CorILMethodFlags = CorILMethodFlags(6i32);
impl ::core::marker::Copy for CorILMethodFlags {}
impl ::core::clone::Clone for CorILMethodFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorILMethodFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorILMethodFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorILMethodFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorILMethodFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorILMethodSect(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_Sect_Reserved: CorILMethodSect = CorILMethodSect(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_Sect_EHTable: CorILMethodSect = CorILMethodSect(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_Sect_OptILTable: CorILMethodSect = CorILMethodSect(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_Sect_KindMask: CorILMethodSect = CorILMethodSect(63i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_Sect_FatFormat: CorILMethodSect = CorILMethodSect(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const CorILMethod_Sect_MoreSects: CorILMethodSect = CorILMethodSect(128i32);
impl ::core::marker::Copy for CorILMethodSect {}
impl ::core::clone::Clone for CorILMethodSect {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorILMethodSect {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorILMethodSect {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorILMethodSect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorILMethodSect").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorImportOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDImportOptionDefault: CorImportOptions = CorImportOptions(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDImportOptionAll: CorImportOptions = CorImportOptions(-1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDImportOptionAllTypeDefs: CorImportOptions = CorImportOptions(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDImportOptionAllMethodDefs: CorImportOptions = CorImportOptions(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDImportOptionAllFieldDefs: CorImportOptions = CorImportOptions(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDImportOptionAllProperties: CorImportOptions = CorImportOptions(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDImportOptionAllEvents: CorImportOptions = CorImportOptions(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDImportOptionAllCustomAttributes: CorImportOptions = CorImportOptions(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDImportOptionAllExportedTypes: CorImportOptions = CorImportOptions(64i32);
impl ::core::marker::Copy for CorImportOptions {}
impl ::core::clone::Clone for CorImportOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorImportOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorImportOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorImportOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorImportOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorLinkerOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDAssembly: CorLinkerOptions = CorLinkerOptions(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNetModule: CorLinkerOptions = CorLinkerOptions(1i32);
impl ::core::marker::Copy for CorLinkerOptions {}
impl ::core::clone::Clone for CorLinkerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorLinkerOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorLinkerOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorLinkerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorLinkerOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorLocalRefPreservation(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDPreserveLocalRefsNone: CorLocalRefPreservation = CorLocalRefPreservation(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDPreserveLocalTypeRef: CorLocalRefPreservation = CorLocalRefPreservation(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDPreserveLocalMemberRef: CorLocalRefPreservation = CorLocalRefPreservation(2i32);
impl ::core::marker::Copy for CorLocalRefPreservation {}
impl ::core::clone::Clone for CorLocalRefPreservation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorLocalRefPreservation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorLocalRefPreservation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorLocalRefPreservation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorLocalRefPreservation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorManifestResourceFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mrVisibilityMask: CorManifestResourceFlags = CorManifestResourceFlags(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mrPublic: CorManifestResourceFlags = CorManifestResourceFlags(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mrPrivate: CorManifestResourceFlags = CorManifestResourceFlags(2i32);
impl ::core::marker::Copy for CorManifestResourceFlags {}
impl ::core::clone::Clone for CorManifestResourceFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorManifestResourceFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorManifestResourceFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorManifestResourceFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorManifestResourceFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorMethodAttr(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdMemberAccessMask: CorMethodAttr = CorMethodAttr(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdPrivateScope: CorMethodAttr = CorMethodAttr(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdPrivate: CorMethodAttr = CorMethodAttr(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdFamANDAssem: CorMethodAttr = CorMethodAttr(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdAssem: CorMethodAttr = CorMethodAttr(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdFamily: CorMethodAttr = CorMethodAttr(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdFamORAssem: CorMethodAttr = CorMethodAttr(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdPublic: CorMethodAttr = CorMethodAttr(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdStatic: CorMethodAttr = CorMethodAttr(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdFinal: CorMethodAttr = CorMethodAttr(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdVirtual: CorMethodAttr = CorMethodAttr(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdHideBySig: CorMethodAttr = CorMethodAttr(128i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdVtableLayoutMask: CorMethodAttr = CorMethodAttr(256i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdReuseSlot: CorMethodAttr = CorMethodAttr(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdNewSlot: CorMethodAttr = CorMethodAttr(256i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdCheckAccessOnOverride: CorMethodAttr = CorMethodAttr(512i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdAbstract: CorMethodAttr = CorMethodAttr(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdSpecialName: CorMethodAttr = CorMethodAttr(2048i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdPinvokeImpl: CorMethodAttr = CorMethodAttr(8192i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdUnmanagedExport: CorMethodAttr = CorMethodAttr(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdReservedMask: CorMethodAttr = CorMethodAttr(53248i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdRTSpecialName: CorMethodAttr = CorMethodAttr(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdHasSecurity: CorMethodAttr = CorMethodAttr(16384i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdRequireSecObject: CorMethodAttr = CorMethodAttr(32768i32);
impl ::core::marker::Copy for CorMethodAttr {}
impl ::core::clone::Clone for CorMethodAttr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorMethodAttr {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorMethodAttr {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorMethodAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorMethodAttr").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorMethodImpl(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miCodeTypeMask: CorMethodImpl = CorMethodImpl(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miIL: CorMethodImpl = CorMethodImpl(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miNative: CorMethodImpl = CorMethodImpl(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miOPTIL: CorMethodImpl = CorMethodImpl(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miRuntime: CorMethodImpl = CorMethodImpl(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miManagedMask: CorMethodImpl = CorMethodImpl(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miUnmanaged: CorMethodImpl = CorMethodImpl(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miManaged: CorMethodImpl = CorMethodImpl(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miForwardRef: CorMethodImpl = CorMethodImpl(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miPreserveSig: CorMethodImpl = CorMethodImpl(128i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miInternalCall: CorMethodImpl = CorMethodImpl(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miSynchronized: CorMethodImpl = CorMethodImpl(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miNoInlining: CorMethodImpl = CorMethodImpl(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miAggressiveInlining: CorMethodImpl = CorMethodImpl(256i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miNoOptimization: CorMethodImpl = CorMethodImpl(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miSecurityMitigations: CorMethodImpl = CorMethodImpl(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miUserMask: CorMethodImpl = CorMethodImpl(5628i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const miMaxMethodImplVal: CorMethodImpl = CorMethodImpl(65535i32);
impl ::core::marker::Copy for CorMethodImpl {}
impl ::core::clone::Clone for CorMethodImpl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorMethodImpl {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorMethodImpl {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorMethodImpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorMethodImpl").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorMethodSemanticsAttr(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const msSetter: CorMethodSemanticsAttr = CorMethodSemanticsAttr(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const msGetter: CorMethodSemanticsAttr = CorMethodSemanticsAttr(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const msOther: CorMethodSemanticsAttr = CorMethodSemanticsAttr(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const msAddOn: CorMethodSemanticsAttr = CorMethodSemanticsAttr(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const msRemoveOn: CorMethodSemanticsAttr = CorMethodSemanticsAttr(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const msFire: CorMethodSemanticsAttr = CorMethodSemanticsAttr(32i32);
impl ::core::marker::Copy for CorMethodSemanticsAttr {}
impl ::core::clone::Clone for CorMethodSemanticsAttr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorMethodSemanticsAttr {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorMethodSemanticsAttr {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorMethodSemanticsAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorMethodSemanticsAttr").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorNativeLinkFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const nlfNone: CorNativeLinkFlags = CorNativeLinkFlags(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const nlfLastError: CorNativeLinkFlags = CorNativeLinkFlags(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const nlfNoMangle: CorNativeLinkFlags = CorNativeLinkFlags(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const nlfMaxValue: CorNativeLinkFlags = CorNativeLinkFlags(3i32);
impl ::core::marker::Copy for CorNativeLinkFlags {}
impl ::core::clone::Clone for CorNativeLinkFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorNativeLinkFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorNativeLinkFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorNativeLinkFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorNativeLinkFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorNativeLinkType(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const nltNone: CorNativeLinkType = CorNativeLinkType(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const nltAnsi: CorNativeLinkType = CorNativeLinkType(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const nltUnicode: CorNativeLinkType = CorNativeLinkType(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const nltAuto: CorNativeLinkType = CorNativeLinkType(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const nltOle: CorNativeLinkType = CorNativeLinkType(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const nltMaxValue: CorNativeLinkType = CorNativeLinkType(7i32);
impl ::core::marker::Copy for CorNativeLinkType {}
impl ::core::clone::Clone for CorNativeLinkType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorNativeLinkType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorNativeLinkType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorNativeLinkType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorNativeLinkType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorNativeType(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_END: CorNativeType = CorNativeType(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_VOID: CorNativeType = CorNativeType(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_BOOLEAN: CorNativeType = CorNativeType(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_I1: CorNativeType = CorNativeType(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_U1: CorNativeType = CorNativeType(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_I2: CorNativeType = CorNativeType(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_U2: CorNativeType = CorNativeType(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_I4: CorNativeType = CorNativeType(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_U4: CorNativeType = CorNativeType(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_I8: CorNativeType = CorNativeType(9i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_U8: CorNativeType = CorNativeType(10i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_R4: CorNativeType = CorNativeType(11i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_R8: CorNativeType = CorNativeType(12i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_SYSCHAR: CorNativeType = CorNativeType(13i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_VARIANT: CorNativeType = CorNativeType(14i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_CURRENCY: CorNativeType = CorNativeType(15i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_PTR: CorNativeType = CorNativeType(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_DECIMAL: CorNativeType = CorNativeType(17i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_DATE: CorNativeType = CorNativeType(18i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_BSTR: CorNativeType = CorNativeType(19i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_LPSTR: CorNativeType = CorNativeType(20i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_LPWSTR: CorNativeType = CorNativeType(21i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_LPTSTR: CorNativeType = CorNativeType(22i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_FIXEDSYSSTRING: CorNativeType = CorNativeType(23i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_OBJECTREF: CorNativeType = CorNativeType(24i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_IUNKNOWN: CorNativeType = CorNativeType(25i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_IDISPATCH: CorNativeType = CorNativeType(26i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_STRUCT: CorNativeType = CorNativeType(27i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_INTF: CorNativeType = CorNativeType(28i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_SAFEARRAY: CorNativeType = CorNativeType(29i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_FIXEDARRAY: CorNativeType = CorNativeType(30i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_INT: CorNativeType = CorNativeType(31i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_UINT: CorNativeType = CorNativeType(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_NESTEDSTRUCT: CorNativeType = CorNativeType(33i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_BYVALSTR: CorNativeType = CorNativeType(34i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_ANSIBSTR: CorNativeType = CorNativeType(35i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_TBSTR: CorNativeType = CorNativeType(36i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_VARIANTBOOL: CorNativeType = CorNativeType(37i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_FUNC: CorNativeType = CorNativeType(38i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_ASANY: CorNativeType = CorNativeType(40i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_ARRAY: CorNativeType = CorNativeType(42i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_LPSTRUCT: CorNativeType = CorNativeType(43i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_CUSTOMMARSHALER: CorNativeType = CorNativeType(44i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_ERROR: CorNativeType = CorNativeType(45i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_IINSPECTABLE: CorNativeType = CorNativeType(46i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_HSTRING: CorNativeType = CorNativeType(47i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_LPUTF8STR: CorNativeType = CorNativeType(48i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NATIVE_TYPE_MAX: CorNativeType = CorNativeType(80i32);
impl ::core::marker::Copy for CorNativeType {}
impl ::core::clone::Clone for CorNativeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorNativeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorNativeType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorNativeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorNativeType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorNotificationForTokenMovement(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyDefault: CorNotificationForTokenMovement = CorNotificationForTokenMovement(15i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyAll: CorNotificationForTokenMovement = CorNotificationForTokenMovement(-1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyNone: CorNotificationForTokenMovement = CorNotificationForTokenMovement(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyMethodDef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyMemberRef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyFieldDef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyTypeRef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyTypeDef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyParamDef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyInterfaceImpl: CorNotificationForTokenMovement = CorNotificationForTokenMovement(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyProperty: CorNotificationForTokenMovement = CorNotificationForTokenMovement(128i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyEvent: CorNotificationForTokenMovement = CorNotificationForTokenMovement(256i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifySignature: CorNotificationForTokenMovement = CorNotificationForTokenMovement(512i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyTypeSpec: CorNotificationForTokenMovement = CorNotificationForTokenMovement(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyCustomAttribute: CorNotificationForTokenMovement = CorNotificationForTokenMovement(2048i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifySecurityValue: CorNotificationForTokenMovement = CorNotificationForTokenMovement(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyPermission: CorNotificationForTokenMovement = CorNotificationForTokenMovement(8192i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyModuleRef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(16384i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyNameSpace: CorNotificationForTokenMovement = CorNotificationForTokenMovement(32768i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyAssemblyRef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(16777216i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyFile: CorNotificationForTokenMovement = CorNotificationForTokenMovement(33554432i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyExportedType: CorNotificationForTokenMovement = CorNotificationForTokenMovement(67108864i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDNotifyResource: CorNotificationForTokenMovement = CorNotificationForTokenMovement(134217728i32);
impl ::core::marker::Copy for CorNotificationForTokenMovement {}
impl ::core::clone::Clone for CorNotificationForTokenMovement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorNotificationForTokenMovement {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorNotificationForTokenMovement {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorNotificationForTokenMovement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorNotificationForTokenMovement").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorOpenFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofRead: CorOpenFlags = CorOpenFlags(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofWrite: CorOpenFlags = CorOpenFlags(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofReadWriteMask: CorOpenFlags = CorOpenFlags(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofCopyMemory: CorOpenFlags = CorOpenFlags(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofReadOnly: CorOpenFlags = CorOpenFlags(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofTakeOwnership: CorOpenFlags = CorOpenFlags(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofNoTypeLib: CorOpenFlags = CorOpenFlags(128i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofNoTransform: CorOpenFlags = CorOpenFlags(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofCheckIntegrity: CorOpenFlags = CorOpenFlags(2048i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofReserved1: CorOpenFlags = CorOpenFlags(256i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofReserved2: CorOpenFlags = CorOpenFlags(512i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofReserved3: CorOpenFlags = CorOpenFlags(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ofReserved: CorOpenFlags = CorOpenFlags(-6336i32);
impl ::core::marker::Copy for CorOpenFlags {}
impl ::core::clone::Clone for CorOpenFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorOpenFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorOpenFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorOpenFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorOpenFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorPEKind(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const peNot: CorPEKind = CorPEKind(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const peILonly: CorPEKind = CorPEKind(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pe32BitRequired: CorPEKind = CorPEKind(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pe32Plus: CorPEKind = CorPEKind(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pe32Unmanaged: CorPEKind = CorPEKind(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pe32BitPreferred: CorPEKind = CorPEKind(16i32);
impl ::core::marker::Copy for CorPEKind {}
impl ::core::clone::Clone for CorPEKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorPEKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorPEKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorPEKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorPEKind").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorParamAttr(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pdIn: CorParamAttr = CorParamAttr(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pdOut: CorParamAttr = CorParamAttr(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pdOptional: CorParamAttr = CorParamAttr(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pdReservedMask: CorParamAttr = CorParamAttr(61440i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pdHasDefault: CorParamAttr = CorParamAttr(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pdHasFieldMarshal: CorParamAttr = CorParamAttr(8192i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pdUnused: CorParamAttr = CorParamAttr(53216i32);
impl ::core::marker::Copy for CorParamAttr {}
impl ::core::clone::Clone for CorParamAttr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorParamAttr {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorParamAttr {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorParamAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorParamAttr").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorPinvokeMap(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmNoMangle: CorPinvokeMap = CorPinvokeMap(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCharSetMask: CorPinvokeMap = CorPinvokeMap(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCharSetNotSpec: CorPinvokeMap = CorPinvokeMap(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCharSetAnsi: CorPinvokeMap = CorPinvokeMap(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCharSetUnicode: CorPinvokeMap = CorPinvokeMap(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCharSetAuto: CorPinvokeMap = CorPinvokeMap(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmBestFitUseAssem: CorPinvokeMap = CorPinvokeMap(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmBestFitEnabled: CorPinvokeMap = CorPinvokeMap(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmBestFitDisabled: CorPinvokeMap = CorPinvokeMap(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmBestFitMask: CorPinvokeMap = CorPinvokeMap(48i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmThrowOnUnmappableCharUseAssem: CorPinvokeMap = CorPinvokeMap(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmThrowOnUnmappableCharEnabled: CorPinvokeMap = CorPinvokeMap(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmThrowOnUnmappableCharDisabled: CorPinvokeMap = CorPinvokeMap(8192i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmThrowOnUnmappableCharMask: CorPinvokeMap = CorPinvokeMap(12288i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmSupportsLastError: CorPinvokeMap = CorPinvokeMap(64i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCallConvMask: CorPinvokeMap = CorPinvokeMap(1792i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCallConvWinapi: CorPinvokeMap = CorPinvokeMap(256i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCallConvCdecl: CorPinvokeMap = CorPinvokeMap(512i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCallConvStdcall: CorPinvokeMap = CorPinvokeMap(768i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCallConvThiscall: CorPinvokeMap = CorPinvokeMap(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmCallConvFastcall: CorPinvokeMap = CorPinvokeMap(1280i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const pmMaxValue: CorPinvokeMap = CorPinvokeMap(65535i32);
impl ::core::marker::Copy for CorPinvokeMap {}
impl ::core::clone::Clone for CorPinvokeMap {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorPinvokeMap {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorPinvokeMap {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorPinvokeMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorPinvokeMap").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorPropertyAttr(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const prSpecialName: CorPropertyAttr = CorPropertyAttr(512i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const prReservedMask: CorPropertyAttr = CorPropertyAttr(62464i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const prRTSpecialName: CorPropertyAttr = CorPropertyAttr(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const prHasDefault: CorPropertyAttr = CorPropertyAttr(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const prUnused: CorPropertyAttr = CorPropertyAttr(59903i32);
impl ::core::marker::Copy for CorPropertyAttr {}
impl ::core::clone::Clone for CorPropertyAttr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorPropertyAttr {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorPropertyAttr {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorPropertyAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorPropertyAttr").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorRefToDefCheck(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDRefToDefDefault: CorRefToDefCheck = CorRefToDefCheck(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDRefToDefAll: CorRefToDefCheck = CorRefToDefCheck(-1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDRefToDefNone: CorRefToDefCheck = CorRefToDefCheck(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDTypeRefToDef: CorRefToDefCheck = CorRefToDefCheck(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDMemberRefToDef: CorRefToDefCheck = CorRefToDefCheck(2i32);
impl ::core::marker::Copy for CorRefToDefCheck {}
impl ::core::clone::Clone for CorRefToDefCheck {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorRefToDefCheck {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorRefToDefCheck {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorRefToDefCheck {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorRefToDefCheck").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorRegFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const regNoCopy: CorRegFlags = CorRegFlags(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const regConfig: CorRegFlags = CorRegFlags(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const regHasRefs: CorRegFlags = CorRegFlags(4i32);
impl ::core::marker::Copy for CorRegFlags {}
impl ::core::clone::Clone for CorRegFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorRegFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorRegFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorRegFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorRegFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorSaveSize(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const cssAccurate: CorSaveSize = CorSaveSize(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const cssQuick: CorSaveSize = CorSaveSize(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const cssDiscardTransientCAs: CorSaveSize = CorSaveSize(2i32);
impl ::core::marker::Copy for CorSaveSize {}
impl ::core::clone::Clone for CorSaveSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorSaveSize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorSaveSize {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorSaveSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorSaveSize").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorSerializationType(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_UNDEFINED: CorSerializationType = CorSerializationType(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_BOOLEAN: CorSerializationType = CorSerializationType(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_CHAR: CorSerializationType = CorSerializationType(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_I1: CorSerializationType = CorSerializationType(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_U1: CorSerializationType = CorSerializationType(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_I2: CorSerializationType = CorSerializationType(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_U2: CorSerializationType = CorSerializationType(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_I4: CorSerializationType = CorSerializationType(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_U4: CorSerializationType = CorSerializationType(9i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_I8: CorSerializationType = CorSerializationType(10i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_U8: CorSerializationType = CorSerializationType(11i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_R4: CorSerializationType = CorSerializationType(12i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_R8: CorSerializationType = CorSerializationType(13i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_STRING: CorSerializationType = CorSerializationType(14i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_SZARRAY: CorSerializationType = CorSerializationType(29i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_TYPE: CorSerializationType = CorSerializationType(80i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_TAGGED_OBJECT: CorSerializationType = CorSerializationType(81i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_FIELD: CorSerializationType = CorSerializationType(83i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_PROPERTY: CorSerializationType = CorSerializationType(84i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const SERIALIZATION_TYPE_ENUM: CorSerializationType = CorSerializationType(85i32);
impl ::core::marker::Copy for CorSerializationType {}
impl ::core::clone::Clone for CorSerializationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorSerializationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorSerializationType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorSerializationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorSerializationType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorSetENC(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDSetENCOn: CorSetENC = CorSetENC(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDSetENCOff: CorSetENC = CorSetENC(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDUpdateENC: CorSetENC = CorSetENC(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDUpdateFull: CorSetENC = CorSetENC(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDUpdateExtension: CorSetENC = CorSetENC(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDUpdateIncremental: CorSetENC = CorSetENC(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDUpdateDelta: CorSetENC = CorSetENC(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDUpdateMask: CorSetENC = CorSetENC(7i32);
impl ::core::marker::Copy for CorSetENC {}
impl ::core::clone::Clone for CorSetENC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorSetENC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorSetENC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorSetENC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorSetENC").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorThreadSafetyOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDThreadSafetyDefault: CorThreadSafetyOptions = CorThreadSafetyOptions(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDThreadSafetyOff: CorThreadSafetyOptions = CorThreadSafetyOptions(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MDThreadSafetyOn: CorThreadSafetyOptions = CorThreadSafetyOptions(1i32);
impl ::core::marker::Copy for CorThreadSafetyOptions {}
impl ::core::clone::Clone for CorThreadSafetyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorThreadSafetyOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorThreadSafetyOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorThreadSafetyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorThreadSafetyOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorTokenType(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtModule: CorTokenType = CorTokenType(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtTypeRef: CorTokenType = CorTokenType(16777216i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtTypeDef: CorTokenType = CorTokenType(33554432i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtFieldDef: CorTokenType = CorTokenType(67108864i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtMethodDef: CorTokenType = CorTokenType(100663296i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtParamDef: CorTokenType = CorTokenType(134217728i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtInterfaceImpl: CorTokenType = CorTokenType(150994944i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtMemberRef: CorTokenType = CorTokenType(167772160i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtCustomAttribute: CorTokenType = CorTokenType(201326592i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtPermission: CorTokenType = CorTokenType(234881024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtSignature: CorTokenType = CorTokenType(285212672i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtEvent: CorTokenType = CorTokenType(335544320i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtProperty: CorTokenType = CorTokenType(385875968i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtMethodImpl: CorTokenType = CorTokenType(419430400i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtModuleRef: CorTokenType = CorTokenType(436207616i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtTypeSpec: CorTokenType = CorTokenType(452984832i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtAssembly: CorTokenType = CorTokenType(536870912i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtAssemblyRef: CorTokenType = CorTokenType(587202560i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtFile: CorTokenType = CorTokenType(637534208i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtExportedType: CorTokenType = CorTokenType(654311424i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtManifestResource: CorTokenType = CorTokenType(671088640i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtGenericParam: CorTokenType = CorTokenType(704643072i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtMethodSpec: CorTokenType = CorTokenType(721420288i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtGenericParamConstraint: CorTokenType = CorTokenType(738197504i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtString: CorTokenType = CorTokenType(1879048192i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtName: CorTokenType = CorTokenType(1895825408i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const mdtBaseType: CorTokenType = CorTokenType(1912602624i32);
impl ::core::marker::Copy for CorTokenType {}
impl ::core::clone::Clone for CorTokenType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorTokenType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorTokenType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorTokenType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorTokenType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorTypeAttr(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdVisibilityMask: CorTypeAttr = CorTypeAttr(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdNotPublic: CorTypeAttr = CorTypeAttr(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdPublic: CorTypeAttr = CorTypeAttr(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdNestedPublic: CorTypeAttr = CorTypeAttr(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdNestedPrivate: CorTypeAttr = CorTypeAttr(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdNestedFamily: CorTypeAttr = CorTypeAttr(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdNestedAssembly: CorTypeAttr = CorTypeAttr(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdNestedFamANDAssem: CorTypeAttr = CorTypeAttr(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdNestedFamORAssem: CorTypeAttr = CorTypeAttr(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdLayoutMask: CorTypeAttr = CorTypeAttr(24i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdAutoLayout: CorTypeAttr = CorTypeAttr(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdSequentialLayout: CorTypeAttr = CorTypeAttr(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdExplicitLayout: CorTypeAttr = CorTypeAttr(16i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdClassSemanticsMask: CorTypeAttr = CorTypeAttr(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdClass: CorTypeAttr = CorTypeAttr(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdInterface: CorTypeAttr = CorTypeAttr(32i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdAbstract: CorTypeAttr = CorTypeAttr(128i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdSealed: CorTypeAttr = CorTypeAttr(256i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdSpecialName: CorTypeAttr = CorTypeAttr(1024i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdImport: CorTypeAttr = CorTypeAttr(4096i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdSerializable: CorTypeAttr = CorTypeAttr(8192i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdWindowsRuntime: CorTypeAttr = CorTypeAttr(16384i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdStringFormatMask: CorTypeAttr = CorTypeAttr(196608i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdAnsiClass: CorTypeAttr = CorTypeAttr(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdUnicodeClass: CorTypeAttr = CorTypeAttr(65536i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdAutoClass: CorTypeAttr = CorTypeAttr(131072i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdCustomFormatClass: CorTypeAttr = CorTypeAttr(196608i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdCustomFormatMask: CorTypeAttr = CorTypeAttr(12582912i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdBeforeFieldInit: CorTypeAttr = CorTypeAttr(1048576i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdForwarder: CorTypeAttr = CorTypeAttr(2097152i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdReservedMask: CorTypeAttr = CorTypeAttr(264192i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdRTSpecialName: CorTypeAttr = CorTypeAttr(2048i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const tdHasSecurity: CorTypeAttr = CorTypeAttr(262144i32);
impl ::core::marker::Copy for CorTypeAttr {}
impl ::core::clone::Clone for CorTypeAttr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorTypeAttr {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorTypeAttr {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorTypeAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorTypeAttr").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorUnmanagedCallingConvention(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_UNMANAGED_CALLCONV_C: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_UNMANAGED_CALLCONV_STDCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_UNMANAGED_CALLCONV_THISCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_UNMANAGED_CALLCONV_FASTCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_C: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_STDCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_THISCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_CEE_CS_CALLCONV_FASTCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(4i32);
impl ::core::marker::Copy for CorUnmanagedCallingConvention {}
impl ::core::clone::Clone for CorUnmanagedCallingConvention {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorUnmanagedCallingConvention {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorUnmanagedCallingConvention {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorUnmanagedCallingConvention {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorUnmanagedCallingConvention").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorValidatorModuleType(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ValidatorModuleTypeInvalid: CorValidatorModuleType = CorValidatorModuleType(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ValidatorModuleTypeMin: CorValidatorModuleType = CorValidatorModuleType(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ValidatorModuleTypePE: CorValidatorModuleType = CorValidatorModuleType(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ValidatorModuleTypeObj: CorValidatorModuleType = CorValidatorModuleType(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ValidatorModuleTypeEnc: CorValidatorModuleType = CorValidatorModuleType(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ValidatorModuleTypeIncr: CorValidatorModuleType = CorValidatorModuleType(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ValidatorModuleTypeMax: CorValidatorModuleType = CorValidatorModuleType(4i32);
impl ::core::marker::Copy for CorValidatorModuleType {}
impl ::core::clone::Clone for CorValidatorModuleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorValidatorModuleType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorValidatorModuleType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorValidatorModuleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorValidatorModuleType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LoadHintEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const LoadDefault: LoadHintEnum = LoadHintEnum(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const LoadAlways: LoadHintEnum = LoadHintEnum(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const LoadSometimes: LoadHintEnum = LoadHintEnum(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const LoadNever: LoadHintEnum = LoadHintEnum(3i32);
impl ::core::marker::Copy for LoadHintEnum {}
impl ::core::clone::Clone for LoadHintEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LoadHintEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LoadHintEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LoadHintEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoadHintEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MergeFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MergeFlagsNone: MergeFlags = MergeFlags(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MergeManifest: MergeFlags = MergeFlags(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const DropMemberRefCAs: MergeFlags = MergeFlags(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NoDupCheck: MergeFlags = MergeFlags(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const MergeExportedTypes: MergeFlags = MergeFlags(8i32);
impl ::core::marker::Copy for MergeFlags {}
impl ::core::clone::Clone for MergeFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MergeFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MergeFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MergeFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MergeFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NGenHintEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NGenDefault: NGenHintEnum = NGenHintEnum(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NGenEager: NGenHintEnum = NGenHintEnum(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NGenLazy: NGenHintEnum = NGenHintEnum(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const NGenNever: NGenHintEnum = NGenHintEnum(3i32);
impl ::core::marker::Copy for NGenHintEnum {}
impl ::core::clone::Clone for NGenHintEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NGenHintEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NGenHintEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NGenHintEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NGenHintEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NativeTypeArrayFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ntaSizeParamIndexSpecified: NativeTypeArrayFlags = NativeTypeArrayFlags(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const ntaReserved: NativeTypeArrayFlags = NativeTypeArrayFlags(65534i32);
impl ::core::marker::Copy for NativeTypeArrayFlags {}
impl ::core::clone::Clone for NativeTypeArrayFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NativeTypeArrayFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NativeTypeArrayFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NativeTypeArrayFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NativeTypeArrayFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ReplacesGeneralNumericDefines(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_COMHEADER: ReplacesGeneralNumericDefines = ReplacesGeneralNumericDefines(14i32);
impl ::core::marker::Copy for ReplacesGeneralNumericDefines {}
impl ::core::clone::Clone for ReplacesGeneralNumericDefines {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ReplacesGeneralNumericDefines {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ReplacesGeneralNumericDefines {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ReplacesGeneralNumericDefines {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReplacesGeneralNumericDefines").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct ASSEMBLYMETADATA {
    pub usMajorVersion: u16,
    pub usMinorVersion: u16,
    pub usBuildNumber: u16,
    pub usRevisionNumber: u16,
    pub szLocale: ::windows::core::PWSTR,
    pub cbLocale: u32,
    pub rProcessor: *mut u32,
    pub ulProcessor: u32,
    pub rOS: *mut OSINFO,
    pub ulOS: u32,
}
impl ::core::marker::Copy for ASSEMBLYMETADATA {}
impl ::core::clone::Clone for ASSEMBLYMETADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ASSEMBLYMETADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASSEMBLYMETADATA").field("usMajorVersion", &self.usMajorVersion).field("usMinorVersion", &self.usMinorVersion).field("usBuildNumber", &self.usBuildNumber).field("usRevisionNumber", &self.usRevisionNumber).field("szLocale", &self.szLocale).field("cbLocale", &self.cbLocale).field("rProcessor", &self.rProcessor).field("ulProcessor", &self.ulProcessor).field("rOS", &self.rOS).field("ulOS", &self.ulOS).finish()
    }
}
impl ::windows::core::TypeKind for ASSEMBLYMETADATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ASSEMBLYMETADATA {
    fn eq(&self, other: &Self) -> bool {
        self.usMajorVersion == other.usMajorVersion && self.usMinorVersion == other.usMinorVersion && self.usBuildNumber == other.usBuildNumber && self.usRevisionNumber == other.usRevisionNumber && self.szLocale == other.szLocale && self.cbLocale == other.cbLocale && self.rProcessor == other.rProcessor && self.ulProcessor == other.ulProcessor && self.rOS == other.rOS && self.ulOS == other.ulOS
    }
}
impl ::core::cmp::Eq for ASSEMBLYMETADATA {}
impl ::core::default::Default for ASSEMBLYMETADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct COR_FIELD_OFFSET {
    pub ridOfField: u32,
    pub ulOffset: u32,
}
impl ::core::marker::Copy for COR_FIELD_OFFSET {}
impl ::core::clone::Clone for COR_FIELD_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_FIELD_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_FIELD_OFFSET").field("ridOfField", &self.ridOfField).field("ulOffset", &self.ulOffset).finish()
    }
}
impl ::windows::core::TypeKind for COR_FIELD_OFFSET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_FIELD_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.ridOfField == other.ridOfField && self.ulOffset == other.ulOffset
    }
}
impl ::core::cmp::Eq for COR_FIELD_OFFSET {}
impl ::core::default::Default for COR_FIELD_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct COR_NATIVE_LINK {
    pub m_linkType: u8,
    pub m_flags: u8,
    pub m_entryPoint: u32,
}
impl ::core::marker::Copy for COR_NATIVE_LINK {}
impl ::core::clone::Clone for COR_NATIVE_LINK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for COR_NATIVE_LINK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for COR_NATIVE_LINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct COR_SECATTR {
    pub tkCtor: u32,
    pub pCustomAttribute: *const ::core::ffi::c_void,
    pub cbCustomAttribute: u32,
}
impl ::core::marker::Copy for COR_SECATTR {}
impl ::core::clone::Clone for COR_SECATTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_SECATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_SECATTR").field("tkCtor", &self.tkCtor).field("pCustomAttribute", &self.pCustomAttribute).field("cbCustomAttribute", &self.cbCustomAttribute).finish()
    }
}
impl ::windows::core::TypeKind for COR_SECATTR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_SECATTR {
    fn eq(&self, other: &Self) -> bool {
        self.tkCtor == other.tkCtor && self.pCustomAttribute == other.pCustomAttribute && self.cbCustomAttribute == other.cbCustomAttribute
    }
}
impl ::core::cmp::Eq for COR_SECATTR {}
impl ::core::default::Default for COR_SECATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct CVStruct {
    pub Major: i16,
    pub Minor: i16,
    pub Sub: i16,
    pub Build: i16,
}
impl ::core::marker::Copy for CVStruct {}
impl ::core::clone::Clone for CVStruct {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CVStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CVStruct").field("Major", &self.Major).field("Minor", &self.Minor).field("Sub", &self.Sub).field("Build", &self.Build).finish()
    }
}
impl ::windows::core::TypeKind for CVStruct {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CVStruct {
    fn eq(&self, other: &Self) -> bool {
        self.Major == other.Major && self.Minor == other.Minor && self.Sub == other.Sub && self.Build == other.Build
    }
}
impl ::core::cmp::Eq for CVStruct {}
impl ::core::default::Default for CVStruct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub union CeeSectionRelocExtra {
    pub highAdj: u16,
}
impl ::core::marker::Copy for CeeSectionRelocExtra {}
impl ::core::clone::Clone for CeeSectionRelocExtra {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CeeSectionRelocExtra {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CeeSectionRelocExtra {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub union IMAGE_COR_ILMETHOD {
    pub Tiny: IMAGE_COR_ILMETHOD_TINY,
    pub Fat: IMAGE_COR_ILMETHOD_FAT,
}
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD {}
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_COR_ILMETHOD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct IMAGE_COR_ILMETHOD_FAT {
    pub _bitfield: u32,
    pub CodeSize: u32,
    pub LocalVarSigTok: u32,
}
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_FAT {}
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_FAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_COR_ILMETHOD_FAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_COR_ILMETHOD_FAT").field("_bitfield", &self._bitfield).field("CodeSize", &self.CodeSize).field("LocalVarSigTok", &self.LocalVarSigTok).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_FAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_COR_ILMETHOD_FAT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.CodeSize == other.CodeSize && self.LocalVarSigTok == other.LocalVarSigTok
    }
}
impl ::core::cmp::Eq for IMAGE_COR_ILMETHOD_FAT {}
impl ::core::default::Default for IMAGE_COR_ILMETHOD_FAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub union IMAGE_COR_ILMETHOD_SECT_EH {
    pub Small: IMAGE_COR_ILMETHOD_SECT_EH_SMALL,
    pub Fat: IMAGE_COR_ILMETHOD_SECT_EH_FAT,
}
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_EH {}
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_EH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_EH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {
    pub Flags: CorExceptionFlag,
    pub TryOffset: u32,
    pub TryLength: u32,
    pub HandlerOffset: u32,
    pub HandlerLength: u32,
    pub Anonymous: IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0,
}
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {}
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub union IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {
    pub ClassToken: u32,
    pub FilterOffset: u32,
}
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {}
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub Anonymous: IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub union IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    pub ClassToken: u32,
    pub FilterOffset: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[cfg(target_arch = "x86")]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    pub _bitfield1: i32,
    pub _bitfield2: u32,
    pub Anonymous: IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
#[cfg(target_arch = "x86")]
pub union IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    pub ClassToken: u32,
    pub FilterOffset: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_FAT {
    pub SectFat: IMAGE_COR_ILMETHOD_SECT_FAT,
    pub Clauses: [IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT; 1],
}
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_EH_FAT {}
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_EH_FAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_FAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_EH_FAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_SMALL {
    pub SectSmall: IMAGE_COR_ILMETHOD_SECT_SMALL,
    pub Reserved: u16,
    pub Clauses: [IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL; 1],
}
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_EH_SMALL {}
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_EH_SMALL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_SMALL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_EH_SMALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct IMAGE_COR_ILMETHOD_SECT_FAT {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_FAT {}
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_FAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_COR_ILMETHOD_SECT_FAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_COR_ILMETHOD_SECT_FAT").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_FAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_COR_ILMETHOD_SECT_FAT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for IMAGE_COR_ILMETHOD_SECT_FAT {}
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_FAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct IMAGE_COR_ILMETHOD_SECT_SMALL {
    pub Kind: u8,
    pub DataSize: u8,
}
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_SECT_SMALL {}
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_SECT_SMALL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_COR_ILMETHOD_SECT_SMALL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_COR_ILMETHOD_SECT_SMALL").field("Kind", &self.Kind).field("DataSize", &self.DataSize).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_SECT_SMALL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_COR_ILMETHOD_SECT_SMALL {
    fn eq(&self, other: &Self) -> bool {
        self.Kind == other.Kind && self.DataSize == other.DataSize
    }
}
impl ::core::cmp::Eq for IMAGE_COR_ILMETHOD_SECT_SMALL {}
impl ::core::default::Default for IMAGE_COR_ILMETHOD_SECT_SMALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct IMAGE_COR_ILMETHOD_TINY {
    pub Flags_CodeSize: u8,
}
impl ::core::marker::Copy for IMAGE_COR_ILMETHOD_TINY {}
impl ::core::clone::Clone for IMAGE_COR_ILMETHOD_TINY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_COR_ILMETHOD_TINY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_COR_ILMETHOD_TINY").field("Flags_CodeSize", &self.Flags_CodeSize).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_ILMETHOD_TINY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_COR_ILMETHOD_TINY {
    fn eq(&self, other: &Self) -> bool {
        self.Flags_CodeSize == other.Flags_CodeSize
    }
}
impl ::core::cmp::Eq for IMAGE_COR_ILMETHOD_TINY {}
impl ::core::default::Default for IMAGE_COR_ILMETHOD_TINY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct IMAGE_COR_VTABLEFIXUP {
    pub RVA: u32,
    pub Count: u16,
    pub Type: u16,
}
impl ::core::marker::Copy for IMAGE_COR_VTABLEFIXUP {}
impl ::core::clone::Clone for IMAGE_COR_VTABLEFIXUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_COR_VTABLEFIXUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_COR_VTABLEFIXUP").field("RVA", &self.RVA).field("Count", &self.Count).field("Type", &self.Type).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_COR_VTABLEFIXUP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_COR_VTABLEFIXUP {
    fn eq(&self, other: &Self) -> bool {
        self.RVA == other.RVA && self.Count == other.Count && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for IMAGE_COR_VTABLEFIXUP {}
impl ::core::default::Default for IMAGE_COR_VTABLEFIXUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
pub struct OSINFO {
    pub dwOSPlatformId: u32,
    pub dwOSMajorVersion: u32,
    pub dwOSMinorVersion: u32,
}
impl ::core::marker::Copy for OSINFO {}
impl ::core::clone::Clone for OSINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OSINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSINFO").field("dwOSPlatformId", &self.dwOSPlatformId).field("dwOSMajorVersion", &self.dwOSMajorVersion).field("dwOSMinorVersion", &self.dwOSMinorVersion).finish()
    }
}
impl ::windows::core::TypeKind for OSINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OSINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSPlatformId == other.dwOSPlatformId && self.dwOSMajorVersion == other.dwOSMajorVersion && self.dwOSMinorVersion == other.dwOSMinorVersion
    }
}
impl ::core::cmp::Eq for OSINFO {}
impl ::core::default::Default for OSINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
