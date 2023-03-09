#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[inline]
pub unsafe fn WMCreateBackupRestorer<P0>(pcallback: P0) -> ::windows::core::Result<IWMLicenseBackup>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMCreateBackupRestorer ( pcallback : * mut::core::ffi::c_void , ppbackup : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWMLicenseBackup>();
    WMCreateBackupRestorer(pcallback.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[inline]
pub unsafe fn WMCreateEditor() -> ::windows::core::Result<IWMMetadataEditor> {
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMCreateEditor ( ppeditor : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWMMetadataEditor>();
    WMCreateEditor(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[inline]
pub unsafe fn WMCreateIndexer() -> ::windows::core::Result<IWMIndexer> {
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMCreateIndexer ( ppindexer : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWMIndexer>();
    WMCreateIndexer(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[inline]
pub unsafe fn WMCreateProfileManager() -> ::windows::core::Result<IWMProfileManager> {
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMCreateProfileManager ( ppprofilemanager : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWMProfileManager>();
    WMCreateProfileManager(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[inline]
pub unsafe fn WMCreateReader<P0>(punkcert: P0, dwrights: u32) -> ::windows::core::Result<IWMReader>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMCreateReader ( punkcert : * mut::core::ffi::c_void , dwrights : u32 , ppreader : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWMReader>();
    WMCreateReader(punkcert.into_param().abi(), dwrights, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[inline]
pub unsafe fn WMCreateSyncReader<P0>(punkcert: P0, dwrights: u32) -> ::windows::core::Result<IWMSyncReader>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMCreateSyncReader ( punkcert : * mut::core::ffi::c_void , dwrights : u32 , ppsyncreader : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWMSyncReader>();
    WMCreateSyncReader(punkcert.into_param().abi(), dwrights, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[inline]
pub unsafe fn WMCreateWriter<P0>(punkcert: P0) -> ::windows::core::Result<IWMWriter>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMCreateWriter ( punkcert : * mut::core::ffi::c_void , ppwriter : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWMWriter>();
    WMCreateWriter(punkcert.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[inline]
pub unsafe fn WMCreateWriterFileSink() -> ::windows::core::Result<IWMWriterFileSink> {
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMCreateWriterFileSink ( ppsink : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWMWriterFileSink>();
    WMCreateWriterFileSink(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[inline]
pub unsafe fn WMCreateWriterNetworkSink() -> ::windows::core::Result<IWMWriterNetworkSink> {
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMCreateWriterNetworkSink ( ppsink : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWMWriterNetworkSink>();
    WMCreateWriterNetworkSink(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[inline]
pub unsafe fn WMCreateWriterPushSink() -> ::windows::core::Result<IWMWriterPushSink> {
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMCreateWriterPushSink ( ppsink : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWMWriterPushSink>();
    WMCreateWriterPushSink(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WMIsContentProtected<P0>(pwszfilename: P0, pfisprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wmvcore.dll""system" fn WMIsContentProtected ( pwszfilename : :: windows::core::PCWSTR , pfisprotected : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    WMIsContentProtected(pwszfilename.into_param().abi(), pfisprotected).ok()
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct INSNetSourceCreator(::windows::core::IUnknown);
impl INSNetSourceCreator {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateNetSource<P0, P1, P2, P3>(&self, pszstreamname: P0, pmonitor: P1, pdata: *const u8, pusercontext: P2, pcallback: P3, qwcontext: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P3: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateNetSource)(::windows::core::Interface::as_raw(self), pszstreamname.into_param().abi(), pmonitor.into_param().abi(), pdata, pusercontext.into_param().abi(), pcallback.into_param().abi(), qwcontext).ok()
    }
    pub unsafe fn GetNetSourceProperties<P0>(&self, pszstreamname: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetNetSourceProperties)(::windows::core::Interface::as_raw(self), pszstreamname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNetSourceSharedNamespace(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetNetSourceSharedNamespace)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetNetSourceAdminInterface<P0>(&self, pszstreamname: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetNetSourceAdminInterface)(::windows::core::Interface::as_raw(self), pszstreamname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetNumProtocolsSupported)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProtocolName)(::windows::core::Interface::as_raw(self), dwprotocolnum, ::core::mem::transmute(pwszprotocolname), pcchprotocolname).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(INSNetSourceCreator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INSNetSourceCreator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSNetSourceCreator {}
impl ::core::fmt::Debug for INSNetSourceCreator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSNetSourceCreator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INSNetSourceCreator {
    type Vtable = INSNetSourceCreator_Vtbl;
}
impl ::core::clone::Clone for INSNetSourceCreator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INSNetSourceCreator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c0e4080_9081_11d2_beec_0060082f2054);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSNetSourceCreator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: ::windows::core::PCWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows::core::HRESULT,
    pub GetNetSourceProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: ::windows::core::PCWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNetSourceSharedNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetNetSourceAdminInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: ::windows::core::PCWSTR, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetNetSourceAdminInterface: usize,
    pub GetNumProtocolsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT,
    pub GetProtocolName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct INSSBuffer(::windows::core::IUnknown);
impl INSSBuffer {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLength)(::windows::core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::windows::core::zeroed::<*mut u8>();
        (::windows::core::Interface::vtable(self).GetBuffer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferAndLength)(::windows::core::Interface::as_raw(self), ppdwbuffer, pdwlength).ok()
    }
}
::windows::imp::interface_hierarchy!(INSSBuffer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INSSBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer {}
impl ::core::fmt::Debug for INSSBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INSSBuffer {
    type Vtable = INSSBuffer_Vtbl;
}
impl ::core::clone::Clone for INSSBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INSSBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1cd3524_03d7_11d2_9eed_006097d2d7cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct INSSBuffer2(::windows::core::IUnknown);
impl INSSBuffer2 {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLength)(::windows::core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMaxLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::windows::core::zeroed::<*mut u8>();
        (::windows::core::Interface::vtable(self).base__.GetBuffer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBufferAndLength)(::windows::core::Interface::as_raw(self), ppdwbuffer, pdwlength).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__ = ::windows::core::zeroed::<u8>();
        (::windows::core::Interface::vtable(self).GetSampleProperties)(::windows::core::Interface::as_raw(self), cbproperties, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSampleProperties)(::windows::core::Interface::as_raw(self), cbproperties, pbproperties).ok()
    }
}
::windows::imp::interface_hierarchy!(INSSBuffer2, ::windows::core::IUnknown, INSSBuffer);
impl ::core::cmp::PartialEq for INSSBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer2 {}
impl ::core::fmt::Debug for INSSBuffer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INSSBuffer2 {
    type Vtable = INSSBuffer2_Vtbl;
}
impl ::core::clone::Clone for INSSBuffer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INSSBuffer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f528693_1035_43fe_b428_757561ad3a68);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer2_Vtbl {
    pub base__: INSSBuffer_Vtbl,
    pub GetSampleProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows::core::HRESULT,
    pub SetSampleProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct INSSBuffer3(::windows::core::IUnknown);
impl INSSBuffer3 {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetLength)(::windows::core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMaxLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::windows::core::zeroed::<*mut u8>();
        (::windows::core::Interface::vtable(self).base__.base__.GetBuffer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetBufferAndLength)(::windows::core::Interface::as_raw(self), ppdwbuffer, pdwlength).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__ = ::windows::core::zeroed::<u8>();
        (::windows::core::Interface::vtable(self).base__.GetSampleProperties)(::windows::core::Interface::as_raw(self), cbproperties, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSampleProperties)(::windows::core::Interface::as_raw(self), cbproperties, pbproperties).ok()
    }
    pub unsafe fn SetProperty(&self, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidbufferproperty), pvbufferproperty, dwbufferpropertysize).ok()
    }
    pub unsafe fn GetProperty(&self, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidbufferproperty), pvbufferproperty, pdwbufferpropertysize).ok()
    }
}
::windows::imp::interface_hierarchy!(INSSBuffer3, ::windows::core::IUnknown, INSSBuffer, INSSBuffer2);
impl ::core::cmp::PartialEq for INSSBuffer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer3 {}
impl ::core::fmt::Debug for INSSBuffer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INSSBuffer3 {
    type Vtable = INSSBuffer3_Vtbl;
}
impl ::core::clone::Clone for INSSBuffer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INSSBuffer3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc87ceaaf_75be_4bc4_84eb_ac2798507672);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer3_Vtbl {
    pub base__: INSSBuffer2_Vtbl,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct INSSBuffer4(::windows::core::IUnknown);
impl INSSBuffer4 {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetLength)(::windows::core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetMaxLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::windows::core::zeroed::<*mut u8>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetBuffer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetBufferAndLength)(::windows::core::Interface::as_raw(self), ppdwbuffer, pdwlength).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__ = ::windows::core::zeroed::<u8>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSampleProperties)(::windows::core::Interface::as_raw(self), cbproperties, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetSampleProperties)(::windows::core::Interface::as_raw(self), cbproperties, pbproperties).ok()
    }
    pub unsafe fn SetProperty(&self, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidbufferproperty), pvbufferproperty, dwbufferpropertysize).ok()
    }
    pub unsafe fn GetProperty(&self, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidbufferproperty), pvbufferproperty, pdwbufferpropertysize).ok()
    }
    pub unsafe fn GetPropertyCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPropertyCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyByIndex(&self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyByIndex)(::windows::core::Interface::as_raw(self), dwbufferpropertyindex, pguidbufferproperty, pvbufferproperty, pdwbufferpropertysize).ok()
    }
}
::windows::imp::interface_hierarchy!(INSSBuffer4, ::windows::core::IUnknown, INSSBuffer, INSSBuffer2, INSSBuffer3);
impl ::core::cmp::PartialEq for INSSBuffer4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer4 {}
impl ::core::fmt::Debug for INSSBuffer4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INSSBuffer4 {
    type Vtable = INSSBuffer4_Vtbl;
}
impl ::core::clone::Clone for INSSBuffer4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INSSBuffer4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b8fd5a_32e2_49d4_a910_c26cc85465ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer4_Vtbl {
    pub base__: INSSBuffer3_Vtbl,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbufferproperties: *mut u32) -> ::windows::core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMAddressAccess(::windows::core::IUnknown);
impl IWMAddressAccess {
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetAccessEntryCount)(::windows::core::Interface::as_raw(self), aetype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__ = ::windows::core::zeroed::<WM_ADDRESS_ACCESSENTRY>();
        (::windows::core::Interface::vtable(self).GetAccessEntry)(::windows::core::Interface::as_raw(self), aetype, dwentrynum, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddAccessEntry)(::windows::core::Interface::as_raw(self), aetype, paddraccessentry).ok()
    }
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAccessEntry)(::windows::core::Interface::as_raw(self), aetype, dwentrynum).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMAddressAccess, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMAddressAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAddressAccess {}
impl ::core::fmt::Debug for IWMAddressAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAddressAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMAddressAccess {
    type Vtable = IWMAddressAccess_Vtbl;
}
impl ::core::clone::Clone for IWMAddressAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMAddressAccess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb3c6389_1633_4e92_af14_9f3173ba39d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAccessEntryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::core::HRESULT,
    pub GetAccessEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT,
    pub AddAccessEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT,
    pub RemoveAccessEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMAddressAccess2(::windows::core::IUnknown);
impl IWMAddressAccess2 {
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetAccessEntryCount)(::windows::core::Interface::as_raw(self), aetype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__ = ::windows::core::zeroed::<WM_ADDRESS_ACCESSENTRY>();
        (::windows::core::Interface::vtable(self).base__.GetAccessEntry)(::windows::core::Interface::as_raw(self), aetype, dwentrynum, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddAccessEntry)(::windows::core::Interface::as_raw(self), aetype, paddraccessentry).ok()
    }
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveAccessEntry)(::windows::core::Interface::as_raw(self), aetype, dwentrynum).ok()
    }
    pub unsafe fn GetAccessEntryEx(&self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut ::windows::core::BSTR, pbstrmask: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAccessEntryEx)(::windows::core::Interface::as_raw(self), aetype, dwentrynum, ::core::mem::transmute(pbstraddress), ::core::mem::transmute(pbstrmask)).ok()
    }
    pub unsafe fn AddAccessEntryEx<P0, P1>(&self, aetype: WM_AETYPE, bstraddress: P0, bstrmask: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddAccessEntryEx)(::windows::core::Interface::as_raw(self), aetype, bstraddress.into_param().abi(), bstrmask.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMAddressAccess2, ::windows::core::IUnknown, IWMAddressAccess);
impl ::core::cmp::PartialEq for IWMAddressAccess2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAddressAccess2 {}
impl ::core::fmt::Debug for IWMAddressAccess2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAddressAccess2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMAddressAccess2 {
    type Vtable = IWMAddressAccess2_Vtbl;
}
impl ::core::clone::Clone for IWMAddressAccess2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMAddressAccess2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65a83fc2_3e98_4d4d_81b5_2a742886b33d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess2_Vtbl {
    pub base__: IWMAddressAccess_Vtbl,
    pub GetAccessEntryEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrmask: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub AddAccessEntryEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, bstraddress: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrmask: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMAuthorizer(::windows::core::IUnknown);
impl IWMAuthorizer {
    pub unsafe fn GetCertCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCertCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::windows::core::zeroed::<*mut u8>();
        (::windows::core::Interface::vtable(self).GetCert)(::windows::core::Interface::as_raw(self), dwindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::windows::core::zeroed::<*mut u8>();
        (::windows::core::Interface::vtable(self).GetSharedData)(::windows::core::Interface::as_raw(self), dwcertindex, pbshareddata, pbcert, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMAuthorizer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMAuthorizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAuthorizer {}
impl ::core::fmt::Debug for IWMAuthorizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAuthorizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMAuthorizer {
    type Vtable = IWMAuthorizer_Vtbl;
}
impl ::core::clone::Clone for IWMAuthorizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMAuthorizer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9b67d36_a9ad_4eb4_baef_db284ef5504c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAuthorizer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCertCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows::core::HRESULT,
    pub GetCert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetSharedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMBackupRestoreProps(::windows::core::IUnknown);
impl IWMBackupRestoreProps {
    pub unsafe fn GetPropCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetPropCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropByIndex(&self, windex: u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropByIndex)(::windows::core::Interface::as_raw(self), windex, ::core::mem::transmute(pwszname), pcchnamelen, ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn GetPropByName<P0>(&self, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPropByName)(::windows::core::Interface::as_raw(self), pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetProp<P0>(&self, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProp)(::windows::core::Interface::as_raw(self), pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn RemoveProp<P0>(&self, pcwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RemoveProp)(::windows::core::Interface::as_raw(self), pcwszname.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllProps(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAllProps)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMBackupRestoreProps, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMBackupRestoreProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMBackupRestoreProps {}
impl ::core::fmt::Debug for IWMBackupRestoreProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMBackupRestoreProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMBackupRestoreProps {
    type Vtable = IWMBackupRestoreProps_Vtbl;
}
impl ::core::clone::Clone for IWMBackupRestoreProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMBackupRestoreProps {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c8e0da6_996f_4ff3_a1af_4838f9377e2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBackupRestoreProps_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPropCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprops: *mut u16) -> ::windows::core::HRESULT,
    pub GetPropByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub GetPropByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub SetProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    pub RemoveProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RemoveAllProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMBandwidthSharing(::windows::core::IUnknown);
impl IWMBandwidthSharing {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStreams)(::windows::core::Interface::as_raw(self), pwstreamnumarray, pcstreams).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddStream)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveStream)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetType)(::windows::core::Interface::as_raw(self), guidtype).ok()
    }
    pub unsafe fn GetBandwidth(&self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBandwidth)(::windows::core::Interface::as_raw(self), pdwbitrate, pmsbufferwindow).ok()
    }
    pub unsafe fn SetBandwidth(&self, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBandwidth)(::windows::core::Interface::as_raw(self), dwbitrate, msbufferwindow).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMBandwidthSharing, ::windows::core::IUnknown, IWMStreamList);
impl ::core::cmp::PartialEq for IWMBandwidthSharing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMBandwidthSharing {}
impl ::core::fmt::Debug for IWMBandwidthSharing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMBandwidthSharing").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMBandwidthSharing {
    type Vtable = IWMBandwidthSharing_Vtbl;
}
impl ::core::clone::Clone for IWMBandwidthSharing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMBandwidthSharing {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad694af1_f8d9_42f8_bc47_70311b0c4f9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBandwidthSharing_Vtbl {
    pub base__: IWMStreamList_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT,
    pub SetBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMClientConnections(::windows::core::IUnknown);
impl IWMClientConnections {
    pub unsafe fn GetClientCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetClientCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows::core::Result<WM_CLIENT_PROPERTIES> {
        let mut result__ = ::windows::core::zeroed::<WM_CLIENT_PROPERTIES>();
        (::windows::core::Interface::vtable(self).GetClientProperties)(::windows::core::Interface::as_raw(self), dwclientnum, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMClientConnections, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMClientConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMClientConnections {}
impl ::core::fmt::Debug for IWMClientConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMClientConnections").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMClientConnections {
    type Vtable = IWMClientConnections_Vtbl;
}
impl ::core::clone::Clone for IWMClientConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMClientConnections {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73c66010_a299_41df_b1f0_ccf03b09c1c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetClientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows::core::HRESULT,
    pub GetClientProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMClientConnections2(::windows::core::IUnknown);
impl IWMClientConnections2 {
    pub unsafe fn GetClientCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetClientCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows::core::Result<WM_CLIENT_PROPERTIES> {
        let mut result__ = ::windows::core::zeroed::<WM_CLIENT_PROPERTIES>();
        (::windows::core::Interface::vtable(self).base__.GetClientProperties)(::windows::core::Interface::as_raw(self), dwclientnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetClientInfo(&self, dwclientnum: u32, pwsznetworkaddress: ::windows::core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows::core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows::core::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClientInfo)(::windows::core::Interface::as_raw(self), dwclientnum, ::core::mem::transmute(pwsznetworkaddress), pcchnetworkaddress, ::core::mem::transmute(pwszport), pcchport, ::core::mem::transmute(pwszdnsname), pcchdnsname).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMClientConnections2, ::windows::core::IUnknown, IWMClientConnections);
impl ::core::cmp::PartialEq for IWMClientConnections2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMClientConnections2 {}
impl ::core::fmt::Debug for IWMClientConnections2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMClientConnections2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMClientConnections2 {
    type Vtable = IWMClientConnections2_Vtbl;
}
impl ::core::clone::Clone for IWMClientConnections2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMClientConnections2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4091571e_4701_4593_bb3d_d5f5f0c74246);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections2_Vtbl {
    pub base__: IWMClientConnections_Vtbl,
    pub GetClientInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: ::windows::core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows::core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows::core::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMCodecInfo(::windows::core::IUnknown);
impl IWMCodecInfo {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCodecInfoCount)(::windows::core::Interface::as_raw(self), guidtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCodecFormatCount)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).GetCodecFormat)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMCodecInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMCodecInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo {}
impl ::core::fmt::Debug for IWMCodecInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMCodecInfo {
    type Vtable = IWMCodecInfo_Vtbl;
}
impl ::core::clone::Clone for IWMCodecInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMCodecInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa970f41e_34de_4a98_b3ba_e4b3ca7528f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCodecInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, pccodecs: *mut u32) -> ::windows::core::HRESULT,
    pub GetCodecFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::core::HRESULT,
    pub GetCodecFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMCodecInfo2(::windows::core::IUnknown);
impl IWMCodecInfo2 {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCodecInfoCount)(::windows::core::Interface::as_raw(self), guidtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCodecFormatCount)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).base__.GetCodecFormat)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecName)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, ::core::mem::transmute(wszname), pcchname).ok()
    }
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: ::windows::core::PWSTR, pcchdesc: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecFormatDesc)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, ::core::mem::transmute(ppistreamconfig), ::core::mem::transmute(wszdesc), pcchdesc).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMCodecInfo2, ::windows::core::IUnknown, IWMCodecInfo);
impl ::core::cmp::PartialEq for IWMCodecInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo2 {}
impl ::core::fmt::Debug for IWMCodecInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMCodecInfo2 {
    type Vtable = IWMCodecInfo2_Vtbl;
}
impl ::core::clone::Clone for IWMCodecInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMCodecInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa65e273_b686_4056_91ec_dd768d4df710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo2_Vtbl {
    pub base__: IWMCodecInfo_Vtbl,
    pub GetCodecName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT,
    pub GetCodecFormatDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut *mut ::core::ffi::c_void, wszdesc: ::windows::core::PWSTR, pcchdesc: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMCodecInfo3(::windows::core::IUnknown);
impl IWMCodecInfo3 {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetCodecInfoCount)(::windows::core::Interface::as_raw(self), guidtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetCodecFormatCount)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).base__.base__.GetCodecFormat)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCodecName)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, ::core::mem::transmute(wszname), pcchname).ok()
    }
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: ::windows::core::PWSTR, pcchdesc: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCodecFormatDesc)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, ::core::mem::transmute(ppistreamconfig), ::core::mem::transmute(wszdesc), pcchdesc).ok()
    }
    pub unsafe fn GetCodecFormatProp<P0>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetCodecFormatProp)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, dwformatindex, pszname.into_param().abi(), ptype, pvalue, pdwsize).ok()
    }
    pub unsafe fn GetCodecProp<P0>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetCodecProp)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, pszname.into_param().abi(), ptype, pvalue, pdwsize).ok()
    }
    pub unsafe fn SetCodecEnumerationSetting<P0>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetCodecEnumerationSetting)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetCodecEnumerationSetting<P0>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetCodecEnumerationSetting)(::windows::core::Interface::as_raw(self), guidtype, dwcodecindex, pszname.into_param().abi(), ptype, pvalue, pdwsize).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMCodecInfo3, ::windows::core::IUnknown, IWMCodecInfo, IWMCodecInfo2);
impl ::core::cmp::PartialEq for IWMCodecInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo3 {}
impl ::core::fmt::Debug for IWMCodecInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMCodecInfo3 {
    type Vtable = IWMCodecInfo3_Vtbl;
}
impl ::core::clone::Clone for IWMCodecInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMCodecInfo3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e51f487_4d93_4f98_8ab4_27d0565adc51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo3_Vtbl {
    pub base__: IWMCodecInfo2_Vtbl,
    pub GetCodecFormatProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetCodecProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetCodecEnumerationSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT,
    pub GetCodecEnumerationSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMCredentialCallback(::windows::core::IUnknown);
impl IWMCredentialCallback {
    pub unsafe fn AcquireCredentials<P0, P1>(&self, pwszrealm: P0, pwszsite: P1, pwszuser: &mut [u16], pwszpassword: &mut [u16], hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AcquireCredentials)(::windows::core::Interface::as_raw(self), pwszrealm.into_param().abi(), pwszsite.into_param().abi(), ::core::mem::transmute(pwszuser.as_ptr()), pwszuser.len() as _, ::core::mem::transmute(pwszpassword.as_ptr()), pwszpassword.len() as _, hrstatus, pdwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMCredentialCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMCredentialCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCredentialCallback {}
impl ::core::fmt::Debug for IWMCredentialCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCredentialCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMCredentialCallback {
    type Vtable = IWMCredentialCallback_Vtbl;
}
impl ::core::clone::Clone for IWMCredentialCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMCredentialCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x342e0eb7_e651_450c_975b_2ace2c90c48e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCredentialCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AcquireCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszrealm: ::windows::core::PCWSTR, pwszsite: ::windows::core::PCWSTR, pwszuser: ::windows::core::PWSTR, cchuser: u32, pwszpassword: ::windows::core::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMEditor(::windows::core::IUnknown);
impl IWMDRMEditor {
    pub unsafe fn GetDRMProperty<P0>(&self, pwstrname: P0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDRMProperty)(::windows::core::Interface::as_raw(self), pwstrname.into_param().abi(), pdwtype, pvalue, pcblength).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDRMEditor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDRMEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMEditor {}
impl ::core::fmt::Debug for IWMDRMEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMEditor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMEditor {
    type Vtable = IWMDRMEditor_Vtbl;
}
impl ::core::clone::Clone for IWMDRMEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMEditor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff130ebc_a6c3_42a6_b401_c3382c3e08b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMEditor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDRMProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: ::windows::core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMMessageParser(::windows::core::IUnknown);
impl IWMDRMMessageParser {
    pub unsafe fn ParseRegistrationReqMsg(&self, pbregistrationreqmsg: &[u8], ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ParseRegistrationReqMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbregistrationreqmsg.as_ptr()), pbregistrationreqmsg.len() as _, ::core::mem::transmute(ppdevicecert), pdeviceserialnumber).ok()
    }
    pub unsafe fn ParseLicenseRequestMsg(&self, pblicenserequestmsg: &[u8], ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ParseLicenseRequestMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblicenserequestmsg.as_ptr()), pblicenserequestmsg.len() as _, ::core::mem::transmute(ppdevicecert), pdeviceserialnumber, ::core::mem::transmute(pbstraction)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDRMMessageParser, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDRMMessageParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMMessageParser {}
impl ::core::fmt::Debug for IWMDRMMessageParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMMessageParser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMMessageParser {
    type Vtable = IWMDRMMessageParser_Vtbl;
}
impl ::core::clone::Clone for IWMDRMMessageParser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMMessageParser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa73a0072_25a0_4c99_b4a5_ede8101a6c39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMMessageParser_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ParseRegistrationReqMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut *mut ::core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT,
    pub ParseLicenseRequestMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut *mut ::core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMReader(::windows::core::IUnknown);
impl IWMDRMReader {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcquireLicense)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelLicenseAcquisition)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Individualize)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelIndividualization)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MonitorLicenseAcquisition)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelMonitorLicenseAcquisition)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDRMProperty<P0>(&self, pwstrname: P0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDRMProperty)(::windows::core::Interface::as_raw(self), pwstrname.into_param().abi(), dwtype, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetDRMProperty<P0>(&self, pwstrname: P0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDRMProperty)(::windows::core::Interface::as_raw(self), pwstrname.into_param().abi(), pdwtype, pvalue, pcblength).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDRMReader, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDRMReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader {}
impl ::core::fmt::Debug for IWMDRMReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMReader {
    type Vtable = IWMDRMReader_Vtbl;
}
impl ::core::clone::Clone for IWMDRMReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2827540_3ee7_432c_b14c_dc17f085d3b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AcquireLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub CancelLicenseAcquisition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Individualize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub CancelIndividualization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MonitorLicenseAcquisition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelMonitorLicenseAcquisition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDRMProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: ::windows::core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    pub GetDRMProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: ::windows::core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMReader2(::windows::core::IUnknown);
impl IWMDRMReader2 {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AcquireLicense)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CancelLicenseAcquisition)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Individualize)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CancelIndividualization)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.MonitorLicenseAcquisition)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CancelMonitorLicenseAcquisition)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDRMProperty<P0>(&self, pwstrname: P0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDRMProperty)(::windows::core::Interface::as_raw(self), pwstrname.into_param().abi(), dwtype, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetDRMProperty<P0>(&self, pwstrname: P0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetDRMProperty)(::windows::core::Interface::as_raw(self), pwstrname.into_param().abi(), pdwtype, pvalue, pcblength).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEvaluateOutputLevelLicenses<P0>(&self, fevaluate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEvaluateOutputLevelLicenses)(::windows::core::Interface::as_raw(self), fevaluate.into_param().abi()).ok()
    }
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPlayOutputLevels)(::windows::core::Interface::as_raw(self), pplayopl, pcblength, pdwminappcompliancelevel).ok()
    }
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCopyOutputLevels)(::windows::core::Interface::as_raw(self), pcopyopl, pcblength, pdwminappcompliancelevel).ok()
    }
    pub unsafe fn TryNextLicense(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TryNextLicense)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDRMReader2, ::windows::core::IUnknown, IWMDRMReader);
impl ::core::cmp::PartialEq for IWMDRMReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader2 {}
impl ::core::fmt::Debug for IWMDRMReader2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMReader2 {
    type Vtable = IWMDRMReader2_Vtbl;
}
impl ::core::clone::Clone for IWMDRMReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMReader2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbefe7a75_9f1d_4075_b9d9_a3c37bda49a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader2_Vtbl {
    pub base__: IWMDRMReader_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEvaluateOutputLevelLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEvaluateOutputLevelLicenses: usize,
    pub GetPlayOutputLevels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT,
    pub GetCopyOutputLevels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT,
    pub TryNextLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMReader3(::windows::core::IUnknown);
impl IWMDRMReader3 {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AcquireLicense)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CancelLicenseAcquisition)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Individualize)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CancelIndividualization)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.MonitorLicenseAcquisition)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CancelMonitorLicenseAcquisition)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDRMProperty<P0>(&self, pwstrname: P0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetDRMProperty)(::windows::core::Interface::as_raw(self), pwstrname.into_param().abi(), dwtype, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetDRMProperty<P0>(&self, pwstrname: P0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetDRMProperty)(::windows::core::Interface::as_raw(self), pwstrname.into_param().abi(), pdwtype, pvalue, pcblength).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEvaluateOutputLevelLicenses<P0>(&self, fevaluate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEvaluateOutputLevelLicenses)(::windows::core::Interface::as_raw(self), fevaluate.into_param().abi()).ok()
    }
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPlayOutputLevels)(::windows::core::Interface::as_raw(self), pplayopl, pcblength, pdwminappcompliancelevel).ok()
    }
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCopyOutputLevels)(::windows::core::Interface::as_raw(self), pcopyopl, pcblength, pdwminappcompliancelevel).ok()
    }
    pub unsafe fn TryNextLicense(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.TryNextLicense)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetInclusionList(&self, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInclusionList)(::windows::core::Interface::as_raw(self), ppguids, pcguids).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDRMReader3, ::windows::core::IUnknown, IWMDRMReader, IWMDRMReader2);
impl ::core::cmp::PartialEq for IWMDRMReader3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader3 {}
impl ::core::fmt::Debug for IWMDRMReader3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMReader3 {
    type Vtable = IWMDRMReader3_Vtbl;
}
impl ::core::clone::Clone for IWMDRMReader3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMReader3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe08672de_f1e7_4ff4_a0a3_fc4b08e4caf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader3_Vtbl {
    pub base__: IWMDRMReader2_Vtbl,
    pub GetInclusionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMTranscryptionManager(::windows::core::IUnknown);
impl IWMDRMTranscryptionManager {
    pub unsafe fn CreateTranscryptor(&self) -> ::windows::core::Result<IWMDRMTranscryptor> {
        let mut result__ = ::windows::core::zeroed::<IWMDRMTranscryptor>();
        (::windows::core::Interface::vtable(self).CreateTranscryptor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMDRMTranscryptionManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDRMTranscryptionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptionManager {}
impl ::core::fmt::Debug for IWMDRMTranscryptionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMTranscryptionManager {
    type Vtable = IWMDRMTranscryptionManager_Vtbl;
}
impl ::core::clone::Clone for IWMDRMTranscryptionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMTranscryptionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1a887b2_a4f0_407a_b02e_efbd23bbecdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptionManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateTranscryptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptranscryptor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMTranscryptor(::windows::core::IUnknown);
impl IWMDRMTranscryptor {
    pub unsafe fn Initialize<P0, P1>(&self, bstrfilename: P0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWMStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), bstrfilename.into_param().abi(), pblicenserequestmsg, cblicenserequestmsg, ::core::mem::transmute(pplicenseresponsemsg), pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Seek)(::windows::core::Interface::as_raw(self), hnstime).ok()
    }
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), pbdata, pcbdata).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDRMTranscryptor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDRMTranscryptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptor {}
impl ::core::fmt::Debug for IWMDRMTranscryptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMTranscryptor {
    type Vtable = IWMDRMTranscryptor_Vtbl;
}
impl ::core::clone::Clone for IWMDRMTranscryptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMTranscryptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69059850_6e6f_4bb2_806f_71863ddfc471);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMTranscryptor2(::windows::core::IUnknown);
impl IWMDRMTranscryptor2 {
    pub unsafe fn Initialize<P0, P1>(&self, bstrfilename: P0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWMStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), bstrfilename.into_param().abi(), pblicenserequestmsg, cblicenserequestmsg, ::core::mem::transmute(pplicenseresponsemsg), pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), hnstime).ok()
    }
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Read)(::windows::core::Interface::as_raw(self), pbdata, pcbdata).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SeekEx<P0>(&self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SeekEx)(::windows::core::Interface::as_raw(self), cnsstarttime, cnsduration, flrate, fincludefileheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZeroAdjustTimestamps<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).ZeroAdjustTimestamps)(::windows::core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn GetSeekStartTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetSeekStartTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetDuration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMDRMTranscryptor2, ::windows::core::IUnknown, IWMDRMTranscryptor);
impl ::core::cmp::PartialEq for IWMDRMTranscryptor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptor2 {}
impl ::core::fmt::Debug for IWMDRMTranscryptor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptor2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMTranscryptor2 {
    type Vtable = IWMDRMTranscryptor2_Vtbl;
}
impl ::core::clone::Clone for IWMDRMTranscryptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMTranscryptor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0da439f_d331_496a_bece_18e5bac5dd23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor2_Vtbl {
    pub base__: IWMDRMTranscryptor_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SeekEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SeekEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ZeroAdjustTimestamps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ZeroAdjustTimestamps: usize,
    pub GetSeekStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnstime: *mut u64) -> ::windows::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMWriter(::windows::core::IUnknown);
impl IWMDRMWriter {
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateKeySeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyseed), pcwchlength).ok()
    }
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateKeyID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyid), pcwchlength).ok()
    }
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateSigningKeyPair)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszprivkey), pcwchprivkeylength, ::core::mem::transmute(pwszpubkey), pcwchpubkeylength).ok()
    }
    pub unsafe fn SetDRMAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDRMAttribute)(::windows::core::Interface::as_raw(self), wstreamnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDRMWriter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDRMWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter {}
impl ::core::fmt::Debug for IWMDRMWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMWriter {
    type Vtable = IWMDRMWriter_Vtbl;
}
impl ::core::clone::Clone for IWMDRMWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6ea5dd0_12a0_43f4_90ab_a3fd451e6a07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GenerateKeySeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT,
    pub GenerateKeyID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT,
    pub GenerateSigningKeyPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::HRESULT,
    pub SetDRMAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMWriter2(::windows::core::IUnknown);
impl IWMDRMWriter2 {
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GenerateKeySeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyseed), pcwchlength).ok()
    }
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GenerateKeyID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyid), pcwchlength).ok()
    }
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GenerateSigningKeyPair)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszprivkey), pcwchprivkeylength, ::core::mem::transmute(pwszpubkey), pcwchpubkeylength).ok()
    }
    pub unsafe fn SetDRMAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDRMAttribute)(::windows::core::Interface::as_raw(self), wstreamnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWMDRMNetEncryption<P0>(&self, fsamplesencrypted: P0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetWMDRMNetEncryption)(::windows::core::Interface::as_raw(self), fsamplesencrypted.into_param().abi(), pbkeyid, cbkeyid).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDRMWriter2, ::windows::core::IUnknown, IWMDRMWriter);
impl ::core::cmp::PartialEq for IWMDRMWriter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter2 {}
impl ::core::fmt::Debug for IWMDRMWriter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMWriter2 {
    type Vtable = IWMDRMWriter2_Vtbl;
}
impl ::core::clone::Clone for IWMDRMWriter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMWriter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38ee7a94_40e2_4e10_aa3f_33fd3210ed5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter2_Vtbl {
    pub base__: IWMDRMWriter_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWMDRMNetEncryption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWMDRMNetEncryption: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDRMWriter3(::windows::core::IUnknown);
impl IWMDRMWriter3 {
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GenerateKeySeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyseed), pcwchlength).ok()
    }
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GenerateKeyID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyid), pcwchlength).ok()
    }
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GenerateSigningKeyPair)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszprivkey), pcwchprivkeylength, ::core::mem::transmute(pwszpubkey), pcwchpubkeylength).ok()
    }
    pub unsafe fn SetDRMAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetDRMAttribute)(::windows::core::Interface::as_raw(self), wstreamnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWMDRMNetEncryption<P0>(&self, fsamplesencrypted: P0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetWMDRMNetEncryption)(::windows::core::Interface::as_raw(self), fsamplesencrypted.into_param().abi(), pbkeyid, cbkeyid).ok()
    }
    pub unsafe fn SetProtectStreamSamples(&self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProtectStreamSamples)(::windows::core::Interface::as_raw(self), pimportinitstruct).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDRMWriter3, ::windows::core::IUnknown, IWMDRMWriter, IWMDRMWriter2);
impl ::core::cmp::PartialEq for IWMDRMWriter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter3 {}
impl ::core::fmt::Debug for IWMDRMWriter3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMWriter3 {
    type Vtable = IWMDRMWriter3_Vtbl;
}
impl ::core::clone::Clone for IWMDRMWriter3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDRMWriter3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7184082_a4aa_4dde_ac9c_e75dbd1117ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter3_Vtbl {
    pub base__: IWMDRMWriter2_Vtbl,
    pub SetProtectStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMDeviceRegistration(::windows::core::IUnknown);
impl IWMDeviceRegistration {
    pub unsafe fn RegisterDevice(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: DRM_VAL16) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__ = ::windows::core::zeroed::<IWMRegisteredDevice>();
        (::windows::core::Interface::vtable(self).RegisterDevice)(::windows::core::Interface::as_raw(self), dwregistertype, ::core::mem::transmute(pbcertificate.as_ptr()), pbcertificate.len() as _, ::core::mem::transmute(serialnumber), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterDevice(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: DRM_VAL16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterDevice)(::windows::core::Interface::as_raw(self), dwregistertype, ::core::mem::transmute(pbcertificate.as_ptr()), pbcertificate.len() as _, ::core::mem::transmute(serialnumber)).ok()
    }
    pub unsafe fn GetRegistrationStats(&self, dwregistertype: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetRegistrationStats)(::windows::core::Interface::as_raw(self), dwregistertype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFirstRegisteredDevice(&self, dwregistertype: u32) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__ = ::windows::core::zeroed::<IWMRegisteredDevice>();
        (::windows::core::Interface::vtable(self).GetFirstRegisteredDevice)(::windows::core::Interface::as_raw(self), dwregistertype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNextRegisteredDevice(&self) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__ = ::windows::core::zeroed::<IWMRegisteredDevice>();
        (::windows::core::Interface::vtable(self).GetNextRegisteredDevice)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRegisteredDeviceByID(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: DRM_VAL16) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__ = ::windows::core::zeroed::<IWMRegisteredDevice>();
        (::windows::core::Interface::vtable(self).GetRegisteredDeviceByID)(::windows::core::Interface::as_raw(self), dwregistertype, ::core::mem::transmute(pbcertificate.as_ptr()), pbcertificate.len() as _, ::core::mem::transmute(serialnumber), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMDeviceRegistration, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDeviceRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDeviceRegistration {}
impl ::core::fmt::Debug for IWMDeviceRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDeviceRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDeviceRegistration {
    type Vtable = IWMDeviceRegistration_Vtbl;
}
impl ::core::clone::Clone for IWMDeviceRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDeviceRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6211f03_8d21_4e94_93e6_8510805f2d99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceRegistration_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::HRESULT,
    pub GetRegistrationStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows::core::HRESULT,
    pub GetFirstRegisteredDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNextRegisteredDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRegisteredDeviceByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMGetSecureChannel(::windows::core::IUnknown);
impl IWMGetSecureChannel {
    pub unsafe fn GetPeerSecureChannelInterface(&self) -> ::windows::core::Result<IWMSecureChannel> {
        let mut result__ = ::windows::core::zeroed::<IWMSecureChannel>();
        (::windows::core::Interface::vtable(self).GetPeerSecureChannelInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMGetSecureChannel, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMGetSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMGetSecureChannel {}
impl ::core::fmt::Debug for IWMGetSecureChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMGetSecureChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMGetSecureChannel {
    type Vtable = IWMGetSecureChannel_Vtbl;
}
impl ::core::clone::Clone for IWMGetSecureChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMGetSecureChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94bc0598_c3d2_11d3_bedf_00c04f612986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMGetSecureChannel_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPeerSecureChannelInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppeer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMHeaderInfo(::windows::core::IUnknown);
impl IWMHeaderInfo {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetAttributeCount)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributeByIndex)(::windows::core::Interface::as_raw(self), windex, pwstreamnum, ::core::mem::transmute(pwszname), pcchnamelen, ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn GetAttributeByName<P0>(&self, pwstreamnum: *mut u16, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetAttributeByName)(::windows::core::Interface::as_raw(self), pwstreamnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAttribute)(::windows::core::Interface::as_raw(self), wstreamnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetMarkerCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMarker)(::windows::core::Interface::as_raw(self), windex, ::core::mem::transmute(pwszmarkername), pcchmarkernamelen, pcnsmarkertime).ok()
    }
    pub unsafe fn AddMarker<P0>(&self, pwszmarkername: P0, cnsmarkertime: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddMarker)(::windows::core::Interface::as_raw(self), pwszmarkername.into_param().abi(), cnsmarkertime).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveMarker)(::windows::core::Interface::as_raw(self), windex).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetScriptCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScript)(::windows::core::Interface::as_raw(self), windex, ::core::mem::transmute(pwsztype), pcchtypelen, ::core::mem::transmute(pwszcommand), pcchcommandlen, pcnsscripttime).ok()
    }
    pub unsafe fn AddScript<P0, P1>(&self, pwsztype: P0, pwszcommand: P1, cnsscripttime: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddScript)(::windows::core::Interface::as_raw(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), cnsscripttime).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveScript)(::windows::core::Interface::as_raw(self), windex).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMHeaderInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMHeaderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo {}
impl ::core::fmt::Debug for IWMHeaderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMHeaderInfo {
    type Vtable = IWMHeaderInfo_Vtbl;
}
impl ::core::clone::Clone for IWMHeaderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMHeaderInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bda_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT,
    pub GetAttributeByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub GetAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    pub GetMarkerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows::core::HRESULT,
    pub GetMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::HRESULT,
    pub AddMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmarkername: ::windows::core::PCWSTR, cnsmarkertime: u64) -> ::windows::core::HRESULT,
    pub RemoveMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT,
    pub GetScriptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows::core::HRESULT,
    pub GetScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::HRESULT,
    pub AddScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztype: ::windows::core::PCWSTR, pwszcommand: ::windows::core::PCWSTR, cnsscripttime: u64) -> ::windows::core::HRESULT,
    pub RemoveScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMHeaderInfo2(::windows::core::IUnknown);
impl IWMHeaderInfo2 {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetAttributeCount)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAttributeByIndex)(::windows::core::Interface::as_raw(self), windex, pwstreamnum, ::core::mem::transmute(pwszname), pcchnamelen, ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn GetAttributeByName<P0>(&self, pwstreamnum: *mut u16, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAttributeByName)(::windows::core::Interface::as_raw(self), pwstreamnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetAttribute)(::windows::core::Interface::as_raw(self), wstreamnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetMarkerCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMarker)(::windows::core::Interface::as_raw(self), windex, ::core::mem::transmute(pwszmarkername), pcchmarkernamelen, pcnsmarkertime).ok()
    }
    pub unsafe fn AddMarker<P0>(&self, pwszmarkername: P0, cnsmarkertime: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddMarker)(::windows::core::Interface::as_raw(self), pwszmarkername.into_param().abi(), cnsmarkertime).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveMarker)(::windows::core::Interface::as_raw(self), windex).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetScriptCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetScript)(::windows::core::Interface::as_raw(self), windex, ::core::mem::transmute(pwsztype), pcchtypelen, ::core::mem::transmute(pwszcommand), pcchcommandlen, pcnsscripttime).ok()
    }
    pub unsafe fn AddScript<P0, P1>(&self, pwsztype: P0, pwszcommand: P1, cnsscripttime: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddScript)(::windows::core::Interface::as_raw(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), cnsscripttime).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveScript)(::windows::core::Interface::as_raw(self), windex).ok()
    }
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCodecInfoCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecInfo)(::windows::core::Interface::as_raw(self), windex, pcchname, ::core::mem::transmute(pwszname), pcchdescription, ::core::mem::transmute(pwszdescription), pcodectype, pcbcodecinfo, pbcodecinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMHeaderInfo2, ::windows::core::IUnknown, IWMHeaderInfo);
impl ::core::cmp::PartialEq for IWMHeaderInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo2 {}
impl ::core::fmt::Debug for IWMHeaderInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMHeaderInfo2 {
    type Vtable = IWMHeaderInfo2_Vtbl;
}
impl ::core::clone::Clone for IWMHeaderInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMHeaderInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15cf9781_454e_482e_b393_85fae487a810);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo2_Vtbl {
    pub base__: IWMHeaderInfo_Vtbl,
    pub GetCodecInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows::core::HRESULT,
    pub GetCodecInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMHeaderInfo3(::windows::core::IUnknown);
impl IWMHeaderInfo3 {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.base__.GetAttributeCount)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAttributeByIndex)(::windows::core::Interface::as_raw(self), windex, pwstreamnum, ::core::mem::transmute(pwszname), pcchnamelen, ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn GetAttributeByName<P0>(&self, pwstreamnum: *mut u16, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetAttributeByName)(::windows::core::Interface::as_raw(self), pwstreamnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetAttribute)(::windows::core::Interface::as_raw(self), wstreamnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMarkerCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetMarker)(::windows::core::Interface::as_raw(self), windex, ::core::mem::transmute(pwszmarkername), pcchmarkernamelen, pcnsmarkertime).ok()
    }
    pub unsafe fn AddMarker<P0>(&self, pwszmarkername: P0, cnsmarkertime: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddMarker)(::windows::core::Interface::as_raw(self), pwszmarkername.into_param().abi(), cnsmarkertime).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveMarker)(::windows::core::Interface::as_raw(self), windex).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.base__.GetScriptCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetScript)(::windows::core::Interface::as_raw(self), windex, ::core::mem::transmute(pwsztype), pcchtypelen, ::core::mem::transmute(pwszcommand), pcchcommandlen, pcnsscripttime).ok()
    }
    pub unsafe fn AddScript<P0, P1>(&self, pwsztype: P0, pwszcommand: P1, cnsscripttime: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddScript)(::windows::core::Interface::as_raw(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), cnsscripttime).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveScript)(::windows::core::Interface::as_raw(self), windex).ok()
    }
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCodecInfoCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCodecInfo)(::windows::core::Interface::as_raw(self), windex, pcchname, ::core::mem::transmute(pwszname), pcchdescription, ::core::mem::transmute(pwszdescription), pcodectype, pcbcodecinfo, pbcodecinfo).ok()
    }
    pub unsafe fn GetAttributeCountEx(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetAttributeCountEx)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAttributeIndices<P0>(&self, wstreamnum: u16, pwszname: P0, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetAttributeIndices)(::windows::core::Interface::as_raw(self), wstreamnum, pwszname.into_param().abi(), pwlangindex, pwindices, pwcount).ok()
    }
    pub unsafe fn GetAttributeByIndexEx(&self, wstreamnum: u16, windex: u16, pwszname: ::windows::core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributeByIndexEx)(::windows::core::Interface::as_raw(self), wstreamnum, windex, ::core::mem::transmute(pwszname), pwnamelen, ptype, pwlangindex, pvalue, pdwdatalength).ok()
    }
    pub unsafe fn ModifyAttribute(&self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModifyAttribute)(::windows::core::Interface::as_raw(self), wstreamnum, windex, r#type, wlangindex, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn AddAttribute<P0>(&self, wstreamnum: u16, pszname: P0, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddAttribute)(::windows::core::Interface::as_raw(self), wstreamnum, pszname.into_param().abi(), pwindex, r#type, wlangindex, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn DeleteAttribute(&self, wstreamnum: u16, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteAttribute)(::windows::core::Interface::as_raw(self), wstreamnum, windex).ok()
    }
    pub unsafe fn AddCodecInfo<P0, P1>(&self, pwszname: P0, pwszdescription: P1, codectype: WMT_CODEC_INFO_TYPE, pbcodecinfo: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddCodecInfo)(::windows::core::Interface::as_raw(self), pwszname.into_param().abi(), pwszdescription.into_param().abi(), codectype, pbcodecinfo.len() as _, ::core::mem::transmute(pbcodecinfo.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMHeaderInfo3, ::windows::core::IUnknown, IWMHeaderInfo, IWMHeaderInfo2);
impl ::core::cmp::PartialEq for IWMHeaderInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo3 {}
impl ::core::fmt::Debug for IWMHeaderInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMHeaderInfo3 {
    type Vtable = IWMHeaderInfo3_Vtbl;
}
impl ::core::clone::Clone for IWMHeaderInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMHeaderInfo3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15cc68e3_27cc_4ecd_b222_3f5d02d80bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo3_Vtbl {
    pub base__: IWMHeaderInfo2_Vtbl,
    pub GetAttributeCountEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT,
    pub GetAttributeIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwszname: ::windows::core::PCWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::HRESULT,
    pub GetAttributeByIndexEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: ::windows::core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::HRESULT,
    pub ModifyAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT,
    pub AddAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows::core::PCWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT,
    pub DeleteAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16) -> ::windows::core::HRESULT,
    pub AddCodecInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwszdescription: ::windows::core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMIStreamProps(::windows::core::IUnknown);
impl IWMIStreamProps {
    pub unsafe fn GetProperty<P0>(&self, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), pszname.into_param().abi(), ptype, pvalue, pdwsize).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMIStreamProps, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMIStreamProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIStreamProps {}
impl ::core::fmt::Debug for IWMIStreamProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIStreamProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMIStreamProps {
    type Vtable = IWMIStreamProps_Vtbl;
}
impl ::core::clone::Clone for IWMIStreamProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMIStreamProps {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6816dad3_2b4b_4c8e_8149_874c3483a753);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIStreamProps_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMImageInfo(::windows::core::IUnknown);
impl IWMImageInfo {
    pub unsafe fn GetImageCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetImageCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetImage(&self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetImage)(::windows::core::Interface::as_raw(self), windex, pcchmimetype, ::core::mem::transmute(pwszmimetype), pcchdescription, ::core::mem::transmute(pwszdescription), pimagetype, pcbimagedata, pbimagedata).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMImageInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMImageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMImageInfo {}
impl ::core::fmt::Debug for IWMImageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMImageInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMImageInfo {
    type Vtable = IWMImageInfo_Vtbl;
}
impl ::core::clone::Clone for IWMImageInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMImageInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f0aa3b6_7267_4d89_88f2_ba915aa5c4c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMImageInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetImageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcimages: *mut u32) -> ::windows::core::HRESULT,
    pub GetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMIndexer(::windows::core::IUnknown);
impl IWMIndexer {
    pub unsafe fn StartIndexing<P0, P1>(&self, pwszurl: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).StartIndexing)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMIndexer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMIndexer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIndexer {}
impl ::core::fmt::Debug for IWMIndexer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIndexer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMIndexer {
    type Vtable = IWMIndexer_Vtbl;
}
impl ::core::clone::Clone for IWMIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMIndexer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d7cdc71_9888_11d3_8edc_00c04f6109cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMIndexer2(::windows::core::IUnknown);
impl IWMIndexer2 {
    pub unsafe fn StartIndexing<P0, P1>(&self, pwszurl: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).base__.StartIndexing)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Configure(&self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Configure)(::windows::core::Interface::as_raw(self), wstreamnum, nindexertype, pvinterval, pvindextype).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMIndexer2, ::windows::core::IUnknown, IWMIndexer);
impl ::core::cmp::PartialEq for IWMIndexer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIndexer2 {}
impl ::core::fmt::Debug for IWMIndexer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIndexer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMIndexer2 {
    type Vtable = IWMIndexer2_Vtbl;
}
impl ::core::clone::Clone for IWMIndexer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMIndexer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb70f1e42_6255_4df0_a6b9_02b212d9e2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer2_Vtbl {
    pub base__: IWMIndexer_Vtbl,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMInputMediaProps(::windows::core::IUnknown);
impl IWMInputMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMediaType)(::windows::core::Interface::as_raw(self), ptype, pcbtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMediaType)(::windows::core::Interface::as_raw(self), ptype).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConnectionName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn GetGroupName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGroupName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname), pcchname).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMInputMediaProps, ::windows::core::IUnknown, IWMMediaProps);
impl ::core::cmp::PartialEq for IWMInputMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMInputMediaProps {}
impl ::core::fmt::Debug for IWMInputMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMInputMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMInputMediaProps {
    type Vtable = IWMInputMediaProps_Vtbl;
}
impl ::core::clone::Clone for IWMInputMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMInputMediaProps {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd5_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMInputMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    pub GetGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMLanguageList(::windows::core::IUnknown);
impl IWMLanguageList {
    pub unsafe fn GetLanguageCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetLanguageCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLanguageDetails(&self, windex: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLanguageDetails)(::windows::core::Interface::as_raw(self), windex, ::core::mem::transmute(pwszlanguagestring), pcchlanguagestringlength).ok()
    }
    pub unsafe fn AddLanguageByRFC1766String<P0>(&self, pwszlanguagestring: P0) -> ::windows::core::Result<u16>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).AddLanguageByRFC1766String)(::windows::core::Interface::as_raw(self), pwszlanguagestring.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMLanguageList, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMLanguageList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLanguageList {}
impl ::core::fmt::Debug for IWMLanguageList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLanguageList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMLanguageList {
    type Vtable = IWMLanguageList_Vtbl;
}
impl ::core::clone::Clone for IWMLanguageList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMLanguageList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf683f00_2d49_4d8e_92b7_fb19f6a0dc57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLanguageList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLanguageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT,
    pub GetLanguageDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    pub AddLanguageByRFC1766String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows::core::PCWSTR, pwindex: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMLicenseBackup(::windows::core::IUnknown);
impl IWMLicenseBackup {
    pub unsafe fn BackupLicenses<P0>(&self, dwflags: u32, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).BackupLicenses)(::windows::core::Interface::as_raw(self), dwflags, pcallback.into_param().abi()).ok()
    }
    pub unsafe fn CancelLicenseBackup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelLicenseBackup)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMLicenseBackup, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMLicenseBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseBackup {}
impl ::core::fmt::Debug for IWMLicenseBackup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseBackup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMLicenseBackup {
    type Vtable = IWMLicenseBackup_Vtbl;
}
impl ::core::clone::Clone for IWMLicenseBackup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMLicenseBackup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05e5ac9f_3fb6_4508_bb43_a4067ba1ebe8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseBackup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BackupLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelLicenseBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMLicenseRestore(::windows::core::IUnknown);
impl IWMLicenseRestore {
    pub unsafe fn RestoreLicenses<P0>(&self, dwflags: u32, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).RestoreLicenses)(::windows::core::Interface::as_raw(self), dwflags, pcallback.into_param().abi()).ok()
    }
    pub unsafe fn CancelLicenseRestore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelLicenseRestore)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMLicenseRestore, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMLicenseRestore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseRestore {}
impl ::core::fmt::Debug for IWMLicenseRestore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseRestore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMLicenseRestore {
    type Vtable = IWMLicenseRestore_Vtbl;
}
impl ::core::clone::Clone for IWMLicenseRestore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMLicenseRestore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc70b6334_a22e_4efb_a245_15e65a004a13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRestore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RestoreLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelLicenseRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMLicenseRevocationAgent(::windows::core::IUnknown);
impl IWMLicenseRevocationAgent {
    pub unsafe fn GetLRBChallenge(&self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLRBChallenge)(::windows::core::Interface::as_raw(self), pmachineid, dwmachineidlength, pchallenge, dwchallengelength, pchallengeoutput, pdwchallengeoutputlength).ok()
    }
    pub unsafe fn ProcessLRB(&self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessLRB)(::windows::core::Interface::as_raw(self), psignedlrb, dwsignedlrblength, psignedack, pdwsignedacklength).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMLicenseRevocationAgent, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMLicenseRevocationAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseRevocationAgent {}
impl ::core::fmt::Debug for IWMLicenseRevocationAgent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseRevocationAgent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMLicenseRevocationAgent {
    type Vtable = IWMLicenseRevocationAgent_Vtbl;
}
impl ::core::clone::Clone for IWMLicenseRevocationAgent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMLicenseRevocationAgent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6967f2c9_4e26_4b57_8894_799880f7ac7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRevocationAgent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLRBChallenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::HRESULT,
    pub ProcessLRB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMMediaProps(::windows::core::IUnknown);
impl IWMMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMediaType)(::windows::core::Interface::as_raw(self), ptype, pcbtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMediaType)(::windows::core::Interface::as_raw(self), ptype).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMMediaProps, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMediaProps {}
impl ::core::fmt::Debug for IWMMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMMediaProps {
    type Vtable = IWMMediaProps_Vtbl;
}
impl ::core::clone::Clone for IWMMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMMediaProps {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bce_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMediaProps_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMediaType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMediaType: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMMetadataEditor(::windows::core::IUnknown);
impl IWMMetadataEditor {
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMMetadataEditor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMMetadataEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMetadataEditor {}
impl ::core::fmt::Debug for IWMMetadataEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMetadataEditor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMMetadataEditor {
    type Vtable = IWMMetadataEditor_Vtbl;
}
impl ::core::clone::Clone for IWMMetadataEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMMetadataEditor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd9_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMMetadataEditor2(::windows::core::IUnknown);
impl IWMMetadataEditor2 {
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Open)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Flush)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OpenEx<P0>(&self, pwszfilename: P0, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OpenEx)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi(), dwdesiredaccess, dwsharemode).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMMetadataEditor2, ::windows::core::IUnknown, IWMMetadataEditor);
impl ::core::cmp::PartialEq for IWMMetadataEditor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMetadataEditor2 {}
impl ::core::fmt::Debug for IWMMetadataEditor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMetadataEditor2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMMetadataEditor2 {
    type Vtable = IWMMetadataEditor2_Vtbl;
}
impl ::core::clone::Clone for IWMMetadataEditor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMMetadataEditor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x203cffe3_2e18_4fdf_b59d_6e71530534cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor2_Vtbl {
    pub base__: IWMMetadataEditor_Vtbl,
    pub OpenEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMMutualExclusion(::windows::core::IUnknown);
impl IWMMutualExclusion {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStreams)(::windows::core::Interface::as_raw(self), pwstreamnumarray, pcstreams).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddStream)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveStream)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetType)(::windows::core::Interface::as_raw(self), guidtype).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMMutualExclusion, ::windows::core::IUnknown, IWMStreamList);
impl ::core::cmp::PartialEq for IWMMutualExclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMutualExclusion {}
impl ::core::fmt::Debug for IWMMutualExclusion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMutualExclusion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMMutualExclusion {
    type Vtable = IWMMutualExclusion_Vtbl;
}
impl ::core::clone::Clone for IWMMutualExclusion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMMutualExclusion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bde_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion_Vtbl {
    pub base__: IWMStreamList_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMMutualExclusion2(::windows::core::IUnknown);
impl IWMMutualExclusion2 {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetStreams)(::windows::core::Interface::as_raw(self), pwstreamnumarray, pcstreams).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AddStream)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveStream)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetType)(::windows::core::Interface::as_raw(self), guidtype).ok()
    }
    pub unsafe fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn SetName<P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn GetRecordCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetRecordCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddRecord(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRecord)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveRecord(&self, wrecordnumber: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveRecord)(::windows::core::Interface::as_raw(self), wrecordnumber).ok()
    }
    pub unsafe fn GetRecordName(&self, wrecordnumber: u16, pwszrecordname: ::windows::core::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRecordName)(::windows::core::Interface::as_raw(self), wrecordnumber, ::core::mem::transmute(pwszrecordname), pcchrecordname).ok()
    }
    pub unsafe fn SetRecordName<P0>(&self, wrecordnumber: u16, pwszrecordname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRecordName)(::windows::core::Interface::as_raw(self), wrecordnumber, pwszrecordname.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamsForRecord(&self, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamsForRecord)(::windows::core::Interface::as_raw(self), wrecordnumber, pwstreamnumarray, pcstreams).ok()
    }
    pub unsafe fn AddStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddStreamForRecord)(::windows::core::Interface::as_raw(self), wrecordnumber, wstreamnumber).ok()
    }
    pub unsafe fn RemoveStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStreamForRecord)(::windows::core::Interface::as_raw(self), wrecordnumber, wstreamnumber).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMMutualExclusion2, ::windows::core::IUnknown, IWMStreamList, IWMMutualExclusion);
impl ::core::cmp::PartialEq for IWMMutualExclusion2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMutualExclusion2 {}
impl ::core::fmt::Debug for IWMMutualExclusion2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMutualExclusion2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMMutualExclusion2 {
    type Vtable = IWMMutualExclusion2_Vtbl;
}
impl ::core::clone::Clone for IWMMutualExclusion2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMMutualExclusion2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0302b57d_89d1_4ba2_85c9_166f2c53eb91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion2_Vtbl {
    pub base__: IWMMutualExclusion_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrecordcount: *mut u16) -> ::windows::core::HRESULT,
    pub AddRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16) -> ::windows::core::HRESULT,
    pub GetRecordName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: ::windows::core::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::HRESULT,
    pub SetRecordName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetStreamsForRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT,
    pub AddStreamForRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT,
    pub RemoveStreamForRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMOutputMediaProps(::windows::core::IUnknown);
impl IWMOutputMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMediaType)(::windows::core::Interface::as_raw(self), ptype, pcbtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMediaType)(::windows::core::Interface::as_raw(self), ptype).ok()
    }
    pub unsafe fn GetStreamGroupName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamGroupName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConnectionName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname), pcchname).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMOutputMediaProps, ::windows::core::IUnknown, IWMMediaProps);
impl ::core::cmp::PartialEq for IWMOutputMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMOutputMediaProps {}
impl ::core::fmt::Debug for IWMOutputMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMOutputMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMOutputMediaProps {
    type Vtable = IWMOutputMediaProps_Vtbl;
}
impl ::core::clone::Clone for IWMOutputMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMOutputMediaProps {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd7_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMOutputMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    pub GetStreamGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMPacketSize(::windows::core::IUnknown);
impl IWMPacketSize {
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxPacketSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxPacketSize)(::windows::core::Interface::as_raw(self), dwmaxpacketsize).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMPacketSize, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMPacketSize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPacketSize {}
impl ::core::fmt::Debug for IWMPacketSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPacketSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMPacketSize {
    type Vtable = IWMPacketSize_Vtbl;
}
impl ::core::clone::Clone for IWMPacketSize {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMPacketSize {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdfb97ab_188f_40b3_b643_5b7903975c59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetMaxPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMPacketSize2(::windows::core::IUnknown);
impl IWMPacketSize2 {
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMaxPacketSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMaxPacketSize)(::windows::core::Interface::as_raw(self), dwmaxpacketsize).ok()
    }
    pub unsafe fn GetMinPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMinPacketSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMinPacketSize(&self, dwminpacketsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinPacketSize)(::windows::core::Interface::as_raw(self), dwminpacketsize).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMPacketSize2, ::windows::core::IUnknown, IWMPacketSize);
impl ::core::cmp::PartialEq for IWMPacketSize2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPacketSize2 {}
impl ::core::fmt::Debug for IWMPacketSize2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPacketSize2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMPacketSize2 {
    type Vtable = IWMPacketSize2_Vtbl;
}
impl ::core::clone::Clone for IWMPacketSize2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMPacketSize2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bfc2b9e_b646_4233_a877_1c6a079669dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize2_Vtbl {
    pub base__: IWMPacketSize_Vtbl,
    pub GetMinPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwminpacketsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetMinPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwminpacketsize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMPlayerHook(::windows::core::IUnknown);
impl IWMPlayerHook {
    pub unsafe fn PreDecode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PreDecode)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMPlayerHook, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMPlayerHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPlayerHook {}
impl ::core::fmt::Debug for IWMPlayerHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPlayerHook").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMPlayerHook {
    type Vtable = IWMPlayerHook_Vtbl;
}
impl ::core::clone::Clone for IWMPlayerHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMPlayerHook {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5b7ca9a_0f1c_4f66_9002_74ec50d8b304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerHook_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub PreDecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMPlayerTimestampHook(::windows::core::IUnknown);
impl IWMPlayerTimestampHook {
    pub unsafe fn MapTimestamp(&self, rtin: i64) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).MapTimestamp)(::windows::core::Interface::as_raw(self), rtin, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMPlayerTimestampHook, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMPlayerTimestampHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPlayerTimestampHook {}
impl ::core::fmt::Debug for IWMPlayerTimestampHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPlayerTimestampHook").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMPlayerTimestampHook {
    type Vtable = IWMPlayerTimestampHook_Vtbl;
}
impl ::core::clone::Clone for IWMPlayerTimestampHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMPlayerTimestampHook {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28580dda_d98e_48d0_b7ae_69e473a02825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerTimestampHook_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub MapTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtin: i64, prtout: *mut i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMProfile(::windows::core::IUnknown);
impl IWMProfile {
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__ = ::windows::core::zeroed::<WMT_VERSION>();
        (::windows::core::Interface::vtable(self).GetVersion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn SetName<P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszdescription), pcchdescription).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, pwszdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), pwszdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStreamCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).GetStream)(::windows::core::Interface::as_raw(self), dwstreamindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).GetStreamByNumber)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStreamConfig>,
    {
        (::windows::core::Interface::vtable(self).RemoveStream)(::windows::core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStreamByNumber)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn AddStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStreamConfig>,
    {
        (::windows::core::Interface::vtable(self).AddStream)(::windows::core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn ReconfigStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStreamConfig>,
    {
        (::windows::core::Interface::vtable(self).ReconfigStream)(::windows::core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).CreateNewStream)(::windows::core::Interface::as_raw(self), guidstreamtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMutualExclusionCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__ = ::windows::core::zeroed::<IWMMutualExclusion>();
        (::windows::core::Interface::vtable(self).GetMutualExclusion)(::windows::core::Interface::as_raw(self), dwmeindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveMutualExclusion<P0>(&self, pme: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMMutualExclusion>,
    {
        (::windows::core::Interface::vtable(self).RemoveMutualExclusion)(::windows::core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<P0>(&self, pme: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMMutualExclusion>,
    {
        (::windows::core::Interface::vtable(self).AddMutualExclusion)(::windows::core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__ = ::windows::core::zeroed::<IWMMutualExclusion>();
        (::windows::core::Interface::vtable(self).CreateNewMutualExclusion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMProfile, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile {}
impl ::core::fmt::Debug for IWMProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfile {
    type Vtable = IWMProfile_Vtbl;
}
impl ::core::clone::Clone for IWMProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMProfile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bdb_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetStreamCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT,
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStreamByNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveStreamByNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReconfigStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNewStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows::core::GUID, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMutualExclusionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows::core::HRESULT,
    pub GetMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNewMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppme: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMProfile2(::windows::core::IUnknown);
impl IWMProfile2 {
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__ = ::windows::core::zeroed::<WMT_VERSION>();
        (::windows::core::Interface::vtable(self).base__.GetVersion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn SetName<P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszdescription), pcchdescription).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, pwszdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), pwszdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStreamCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).base__.GetStream)(::windows::core::Interface::as_raw(self), dwstreamindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).base__.GetStreamByNumber)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStreamConfig>,
    {
        (::windows::core::Interface::vtable(self).base__.RemoveStream)(::windows::core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveStreamByNumber)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn AddStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStreamConfig>,
    {
        (::windows::core::Interface::vtable(self).base__.AddStream)(::windows::core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn ReconfigStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStreamConfig>,
    {
        (::windows::core::Interface::vtable(self).base__.ReconfigStream)(::windows::core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).base__.CreateNewStream)(::windows::core::Interface::as_raw(self), guidstreamtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMutualExclusionCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__ = ::windows::core::zeroed::<IWMMutualExclusion>();
        (::windows::core::Interface::vtable(self).base__.GetMutualExclusion)(::windows::core::Interface::as_raw(self), dwmeindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveMutualExclusion<P0>(&self, pme: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMMutualExclusion>,
    {
        (::windows::core::Interface::vtable(self).base__.RemoveMutualExclusion)(::windows::core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<P0>(&self, pme: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMMutualExclusion>,
    {
        (::windows::core::Interface::vtable(self).base__.AddMutualExclusion)(::windows::core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__ = ::windows::core::zeroed::<IWMMutualExclusion>();
        (::windows::core::Interface::vtable(self).base__.CreateNewMutualExclusion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProfileID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetProfileID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMProfile2, ::windows::core::IUnknown, IWMProfile);
impl ::core::cmp::PartialEq for IWMProfile2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile2 {}
impl ::core::fmt::Debug for IWMProfile2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfile2 {
    type Vtable = IWMProfile2_Vtbl;
}
impl ::core::clone::Clone for IWMProfile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMProfile2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07e72d33_d94e_4be7_8843_60ae5ff7e5f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile2_Vtbl {
    pub base__: IWMProfile_Vtbl,
    pub GetProfileID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMProfile3(::windows::core::IUnknown);
impl IWMProfile3 {
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__ = ::windows::core::zeroed::<WMT_VERSION>();
        (::windows::core::Interface::vtable(self).base__.base__.GetVersion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn SetName<P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetName)(::windows::core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszdescription), pcchdescription).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, pwszdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetDescription)(::windows::core::Interface::as_raw(self), pwszdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStreamCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStream)(::windows::core::Interface::as_raw(self), dwstreamindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStreamByNumber)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStreamConfig>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveStream)(::windows::core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveStreamByNumber)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn AddStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStreamConfig>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddStream)(::windows::core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn ReconfigStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStreamConfig>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.ReconfigStream)(::windows::core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamConfig>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateNewStream)(::windows::core::Interface::as_raw(self), guidstreamtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMutualExclusionCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__ = ::windows::core::zeroed::<IWMMutualExclusion>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMutualExclusion)(::windows::core::Interface::as_raw(self), dwmeindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveMutualExclusion<P0>(&self, pme: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMMutualExclusion>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveMutualExclusion)(::windows::core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<P0>(&self, pme: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMMutualExclusion>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddMutualExclusion)(::windows::core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__ = ::windows::core::zeroed::<IWMMutualExclusion>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateNewMutualExclusion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProfileID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetProfileID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStorageFormat(&self) -> ::windows::core::Result<WMT_STORAGE_FORMAT> {
        let mut result__ = ::windows::core::zeroed::<WMT_STORAGE_FORMAT>();
        (::windows::core::Interface::vtable(self).GetStorageFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStorageFormat(&self, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStorageFormat)(::windows::core::Interface::as_raw(self), nstorageformat).ok()
    }
    pub unsafe fn GetBandwidthSharingCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetBandwidthSharingCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBandwidthSharing(&self, dwbsindex: u32) -> ::windows::core::Result<IWMBandwidthSharing> {
        let mut result__ = ::windows::core::zeroed::<IWMBandwidthSharing>();
        (::windows::core::Interface::vtable(self).GetBandwidthSharing)(::windows::core::Interface::as_raw(self), dwbsindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveBandwidthSharing<P0>(&self, pbs: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMBandwidthSharing>,
    {
        (::windows::core::Interface::vtable(self).RemoveBandwidthSharing)(::windows::core::Interface::as_raw(self), pbs.into_param().abi()).ok()
    }
    pub unsafe fn AddBandwidthSharing<P0>(&self, pbs: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMBandwidthSharing>,
    {
        (::windows::core::Interface::vtable(self).AddBandwidthSharing)(::windows::core::Interface::as_raw(self), pbs.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewBandwidthSharing(&self) -> ::windows::core::Result<IWMBandwidthSharing> {
        let mut result__ = ::windows::core::zeroed::<IWMBandwidthSharing>();
        (::windows::core::Interface::vtable(self).CreateNewBandwidthSharing)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamPrioritization(&self) -> ::windows::core::Result<IWMStreamPrioritization> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamPrioritization>();
        (::windows::core::Interface::vtable(self).GetStreamPrioritization)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamPrioritization<P0>(&self, psp: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStreamPrioritization>,
    {
        (::windows::core::Interface::vtable(self).SetStreamPrioritization)(::windows::core::Interface::as_raw(self), psp.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamPrioritization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStreamPrioritization)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateNewStreamPrioritization(&self) -> ::windows::core::Result<IWMStreamPrioritization> {
        let mut result__ = ::windows::core::zeroed::<IWMStreamPrioritization>();
        (::windows::core::Interface::vtable(self).CreateNewStreamPrioritization)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetExpectedPacketCount(&self, msduration: u64) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetExpectedPacketCount)(::windows::core::Interface::as_raw(self), msduration, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMProfile3, ::windows::core::IUnknown, IWMProfile, IWMProfile2);
impl ::core::cmp::PartialEq for IWMProfile3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile3 {}
impl ::core::fmt::Debug for IWMProfile3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfile3 {
    type Vtable = IWMProfile3_Vtbl;
}
impl ::core::clone::Clone for IWMProfile3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMProfile3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00ef96cc_a461_4546_8bcd_c9a28f0e06f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile3_Vtbl {
    pub base__: IWMProfile2_Vtbl,
    pub GetStorageFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT,
    pub SetStorageFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT,
    pub GetBandwidthSharingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbs: *mut u32) -> ::windows::core::HRESULT,
    pub GetBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbsindex: u32, ppbs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNewBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psp: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNewStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetExpectedPacketCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMProfileManager(::windows::core::IUnknown);
impl IWMProfileManager {
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<IWMProfile> {
        let mut result__ = ::windows::core::zeroed::<IWMProfile>();
        (::windows::core::Interface::vtable(self).CreateEmptyProfile)(::windows::core::Interface::as_raw(self), dwversion, &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<IWMProfile> {
        let mut result__ = ::windows::core::zeroed::<IWMProfile>();
        (::windows::core::Interface::vtable(self).LoadProfileByID)(::windows::core::Interface::as_raw(self), guidprofile, &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadProfileByData<P0>(&self, pwszprofile: P0) -> ::windows::core::Result<IWMProfile>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMProfile>();
        (::windows::core::Interface::vtable(self).LoadProfileByData)(::windows::core::Interface::as_raw(self), pwszprofile.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SaveProfile<P0, P1>(&self, piwmprofile: P0, pwszprofile: P1, pdwlength: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMProfile>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SaveProfile)(::windows::core::Interface::as_raw(self), piwmprofile.into_param().abi(), pwszprofile.into_param().abi(), pdwlength).ok()
    }
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSystemProfileCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows::core::Result<IWMProfile> {
        let mut result__ = ::windows::core::zeroed::<IWMProfile>();
        (::windows::core::Interface::vtable(self).LoadSystemProfile)(::windows::core::Interface::as_raw(self), dwprofileindex, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMProfileManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMProfileManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManager {}
impl ::core::fmt::Debug for IWMProfileManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfileManager {
    type Vtable = IWMProfileManager_Vtbl;
}
impl ::core::clone::Clone for IWMProfileManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMProfileManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd16679f2_6ca0_472d_8d31_2f5d55aee155);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateEmptyProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadProfileByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadProfileByData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprofile: ::windows::core::PCWSTR, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SaveProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmprofile: *mut ::core::ffi::c_void, pwszprofile: ::windows::core::PCWSTR, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetSystemProfileCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows::core::HRESULT,
    pub LoadSystemProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMProfileManager2(::windows::core::IUnknown);
impl IWMProfileManager2 {
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<IWMProfile> {
        let mut result__ = ::windows::core::zeroed::<IWMProfile>();
        (::windows::core::Interface::vtable(self).base__.CreateEmptyProfile)(::windows::core::Interface::as_raw(self), dwversion, &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<IWMProfile> {
        let mut result__ = ::windows::core::zeroed::<IWMProfile>();
        (::windows::core::Interface::vtable(self).base__.LoadProfileByID)(::windows::core::Interface::as_raw(self), guidprofile, &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadProfileByData<P0>(&self, pwszprofile: P0) -> ::windows::core::Result<IWMProfile>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMProfile>();
        (::windows::core::Interface::vtable(self).base__.LoadProfileByData)(::windows::core::Interface::as_raw(self), pwszprofile.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SaveProfile<P0, P1>(&self, piwmprofile: P0, pwszprofile: P1, pdwlength: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMProfile>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SaveProfile)(::windows::core::Interface::as_raw(self), piwmprofile.into_param().abi(), pwszprofile.into_param().abi(), pdwlength).ok()
    }
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSystemProfileCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows::core::Result<IWMProfile> {
        let mut result__ = ::windows::core::zeroed::<IWMProfile>();
        (::windows::core::Interface::vtable(self).base__.LoadSystemProfile)(::windows::core::Interface::as_raw(self), dwprofileindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSystemProfileVersion(&self, pdwversion: *mut WMT_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSystemProfileVersion)(::windows::core::Interface::as_raw(self), pdwversion).ok()
    }
    pub unsafe fn SetSystemProfileVersion(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSystemProfileVersion)(::windows::core::Interface::as_raw(self), dwversion).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMProfileManager2, ::windows::core::IUnknown, IWMProfileManager);
impl ::core::cmp::PartialEq for IWMProfileManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManager2 {}
impl ::core::fmt::Debug for IWMProfileManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfileManager2 {
    type Vtable = IWMProfileManager2_Vtbl;
}
impl ::core::clone::Clone for IWMProfileManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMProfileManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a924e51_73c1_494d_8019_23d37ed9b89a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager2_Vtbl {
    pub base__: IWMProfileManager_Vtbl,
    pub GetSystemProfileVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT,
    pub SetSystemProfileVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMProfileManagerLanguage(::windows::core::IUnknown);
impl IWMProfileManagerLanguage {
    pub unsafe fn GetUserLanguageID(&self, wlangid: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUserLanguageID)(::windows::core::Interface::as_raw(self), wlangid).ok()
    }
    pub unsafe fn SetUserLanguageID(&self, wlangid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUserLanguageID)(::windows::core::Interface::as_raw(self), wlangid).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMProfileManagerLanguage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMProfileManagerLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManagerLanguage {}
impl ::core::fmt::Debug for IWMProfileManagerLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManagerLanguage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfileManagerLanguage {
    type Vtable = IWMProfileManagerLanguage_Vtbl;
}
impl ::core::clone::Clone for IWMProfileManagerLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMProfileManagerLanguage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba4dcc78_7ee0_4ab8_b27a_dbce8bc51454);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManagerLanguage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUserLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wlangid: *mut u16) -> ::windows::core::HRESULT,
    pub SetUserLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wlangid: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMPropertyVault(::windows::core::IUnknown);
impl IWMPropertyVault {
    pub unsafe fn GetPropertyCount(&self, pdwcount: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyCount)(::windows::core::Interface::as_raw(self), pdwcount).ok()
    }
    pub unsafe fn GetPropertyByName<P0>(&self, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPropertyByName)(::windows::core::Interface::as_raw(self), pszname.into_param().abi(), ptype, pvalue, pdwsize).ok()
    }
    pub unsafe fn SetProperty<P0>(&self, pszname: P0, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), pszname.into_param().abi(), ptype, pvalue, dwsize).ok()
    }
    pub unsafe fn GetPropertyByIndex(&self, dwindex: u32, pszname: ::windows::core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyByIndex)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(pszname), pdwnamelen, ptype, pvalue, pdwsize).ok()
    }
    pub unsafe fn CopyPropertiesFrom<P0>(&self, piwmpropertyvault: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMPropertyVault>,
    {
        (::windows::core::Interface::vtable(self).CopyPropertiesFrom)(::windows::core::Interface::as_raw(self), piwmpropertyvault.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMPropertyVault, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMPropertyVault {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPropertyVault {}
impl ::core::fmt::Debug for IWMPropertyVault {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPropertyVault").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMPropertyVault {
    type Vtable = IWMPropertyVault_Vtbl;
}
impl ::core::clone::Clone for IWMPropertyVault {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMPropertyVault {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72995a79_5090_42a4_9c8c_d9d0b6d34be5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPropertyVault_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *const u32) -> ::windows::core::HRESULT,
    pub GetPropertyByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pszname: ::windows::core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub CopyPropertiesFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpropertyvault: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMProximityDetection(::windows::core::IUnknown);
impl IWMProximityDetection {
    pub unsafe fn StartDetection<P0>(&self, pbregistrationmsg: &[u8], pblocaladdress: &[u8], dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: P0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).StartDetection)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbregistrationmsg.as_ptr()), pbregistrationmsg.len() as _, ::core::mem::transmute(pblocaladdress.as_ptr()), pblocaladdress.len() as _, dwextraportsallowed, ::core::mem::transmute(ppregistrationresponsemsg), pcallback.into_param().abi(), pvcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMProximityDetection, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMProximityDetection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProximityDetection {}
impl ::core::fmt::Debug for IWMProximityDetection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProximityDetection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProximityDetection {
    type Vtable = IWMProximityDetection_Vtbl;
}
impl ::core::clone::Clone for IWMProximityDetection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMProximityDetection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a9fd8ee_b651_4bf0_b849_7d4ece79a2b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProximityDetection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartDetection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReader(::windows::core::IUnknown);
impl IWMReader {
    pub unsafe fn Open<P0, P1>(&self, pwszurl: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMReaderCallback>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetOutputCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__ = ::windows::core::zeroed::<IWMOutputMediaProps>();
        (::windows::core::Interface::vtable(self).GetOutputProps)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOutputProps<P0>(&self, dwoutputnum: u32, poutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMOutputMediaProps>,
    {
        (::windows::core::Interface::vtable(self).SetOutputProps)(::windows::core::Interface::as_raw(self), dwoutputnum, poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnumber: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetOutputFormatCount)(::windows::core::Interface::as_raw(self), dwoutputnumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__ = ::windows::core::zeroed::<IWMOutputMediaProps>();
        (::windows::core::Interface::vtable(self).GetOutputFormat)(::windows::core::Interface::as_raw(self), dwoutputnumber, dwformatnumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn Start(&self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self), cnsstart, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReader, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReader {}
impl ::core::fmt::Debug for IWMReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReader {
    type Vtable = IWMReader_Vtbl;
}
impl ::core::clone::Clone for IWMReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd6_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderAccelerator(::windows::core::IUnknown);
impl IWMReaderAccelerator {
    pub unsafe fn GetCodecInterface(&self, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecInterface)(::windows::core::Interface::as_raw(self), dwoutputnum, riid, ppvcodecinterface).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Notify(&self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Notify)(::windows::core::Interface::as_raw(self), dwoutputnum, psubtype).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderAccelerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMReaderAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAccelerator {}
impl ::core::fmt::Debug for IWMReaderAccelerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAccelerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAccelerator {
    type Vtable = IWMReaderAccelerator_Vtbl;
}
impl ::core::clone::Clone for IWMReaderAccelerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderAccelerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbddc4d08_944d_4d52_a612_46c3fda07dd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAccelerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCodecInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Notify: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced(::windows::core::IUnknown);
impl IWMReaderAdvanced {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetUserProvidedClock)(::windows::core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetUserProvidedClock)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeliverTime)(::windows::core::Interface::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetManualStreamSelection)(::windows::core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetManualStreamSelection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamsSelected)(::windows::core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::windows::core::zeroed::<WMT_STREAM_SELECTION>();
        (::windows::core::Interface::vtable(self).GetStreamSelected)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAllocateForStream)(::windows::core::Interface::as_raw(self), wstreamnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetAllocateForStream)(::windows::core::Interface::as_raw(self), dwsreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatistics)(::windows::core::Interface::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClientInfo)(::windows::core::Interface::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxOutputSampleSize)(::windows::core::Interface::as_raw(self), dwoutput, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxStreamSampleSize)(::windows::core::Interface::as_raw(self), wstream, &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyLateDelivery)(::windows::core::Interface::as_raw(self), cnslateness).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderAdvanced, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMReaderAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced {}
impl ::core::fmt::Debug for IWMReaderAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced {
    type Vtable = IWMReaderAdvanced_Vtbl;
}
impl ::core::clone::Clone for IWMReaderAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderAdvanced {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bea_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserProvidedClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserProvidedClock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserProvidedClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserProvidedClock: usize,
    pub DeliverTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetManualStreamSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetManualStreamSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetManualStreamSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetManualStreamSelection: usize,
    pub SetStreamsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub GetStreamSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceiveSelectionCallbacks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceiveSelectionCallbacks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReceiveSelectionCallbacks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReceiveSelectionCallbacks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceiveStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceiveStreamSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReceiveStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReceiveStreamSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateForOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateForOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateForStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateForStream: usize,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientInfo: usize,
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub NotifyLateDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced2(::windows::core::IUnknown);
impl IWMReaderAdvanced2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetUserProvidedClock)(::windows::core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetUserProvidedClock)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeliverTime)(::windows::core::Interface::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetManualStreamSelection)(::windows::core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetManualStreamSelection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetStreamsSelected)(::windows::core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::windows::core::zeroed::<WMT_STREAM_SELECTION>();
        (::windows::core::Interface::vtable(self).base__.GetStreamSelected)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetAllocateForStream)(::windows::core::Interface::as_raw(self), wstreamnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetAllocateForStream)(::windows::core::Interface::as_raw(self), dwsreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStatistics)(::windows::core::Interface::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetClientInfo)(::windows::core::Interface::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMaxOutputSampleSize)(::windows::core::Interface::as_raw(self), dwoutput, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMaxStreamSampleSize)(::windows::core::Interface::as_raw(self), wstream, &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.NotifyLateDelivery)(::windows::core::Interface::as_raw(self), cnslateness).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::windows::core::zeroed::<WMT_PLAY_MODE>();
        (::windows::core::Interface::vtable(self).GetPlayMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferProgress)(::windows::core::Interface::as_raw(self), pdwpercent, pcnsbuffering).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDownloadProgress)(::windows::core::Interface::as_raw(self), pdwpercent, pqwbytesdownloaded, pcnsdownload).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSaveAsProgress)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SaveFileAs<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SaveFileAs)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProtocolName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszprotocol), pcchprotocol).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartAtMarker)(::windows::core::Interface::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Preroll)(::windows::core::Interface::as_raw(self), cnsstart, cnsduration, frate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<P0>(&self, flogclientid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetLogClientID)(::windows::core::Interface::as_raw(self), flogclientid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetLogClientID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopBuffering)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows::core::IntoParam<IWMReaderCallback>,
    {
        (::windows::core::Interface::vtable(self).OpenStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), pcallback.into_param().abi(), pvcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderAdvanced2, ::windows::core::IUnknown, IWMReaderAdvanced);
impl ::core::cmp::PartialEq for IWMReaderAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced2 {}
impl ::core::fmt::Debug for IWMReaderAdvanced2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced2 {
    type Vtable = IWMReaderAdvanced2_Vtbl;
}
impl ::core::clone::Clone for IWMReaderAdvanced2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderAdvanced2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae14a945_b90c_4d0d_9127_80d665f7d73e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced2_Vtbl {
    pub base__: IWMReaderAdvanced_Vtbl,
    pub SetPlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub GetPlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub GetBufferProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT,
    pub GetDownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT,
    pub GetSaveAsProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT,
    pub SaveFileAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetProtocolName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT,
    pub StartAtMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub SetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    pub Preroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogClientID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogClientID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLogClientID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLogClientID: usize,
    pub StopBuffering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenStream: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced3(::windows::core::IUnknown);
impl IWMReaderAdvanced3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetUserProvidedClock)(::windows::core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.GetUserProvidedClock)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeliverTime)(::windows::core::Interface::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetManualStreamSelection)(::windows::core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.GetManualStreamSelection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetStreamsSelected)(::windows::core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::windows::core::zeroed::<WMT_STREAM_SELECTION>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStreamSelected)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.GetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.GetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.GetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetAllocateForStream)(::windows::core::Interface::as_raw(self), wstreamnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.GetAllocateForStream)(::windows::core::Interface::as_raw(self), dwsreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetStatistics)(::windows::core::Interface::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetClientInfo)(::windows::core::Interface::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMaxOutputSampleSize)(::windows::core::Interface::as_raw(self), dwoutput, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMaxStreamSampleSize)(::windows::core::Interface::as_raw(self), wstream, &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.NotifyLateDelivery)(::windows::core::Interface::as_raw(self), cnslateness).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPlayMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::windows::core::zeroed::<WMT_PLAY_MODE>();
        (::windows::core::Interface::vtable(self).base__.GetPlayMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBufferProgress)(::windows::core::Interface::as_raw(self), pdwpercent, pcnsbuffering).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDownloadProgress)(::windows::core::Interface::as_raw(self), pdwpercent, pqwbytesdownloaded, pcnsdownload).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSaveAsProgress)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SaveFileAs<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SaveFileAs)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProtocolName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszprotocol), pcchprotocol).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartAtMarker)(::windows::core::Interface::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Preroll)(::windows::core::Interface::as_raw(self), cnsstart, cnsduration, frate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<P0>(&self, flogclientid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetLogClientID)(::windows::core::Interface::as_raw(self), flogclientid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetLogClientID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StopBuffering)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows::core::IntoParam<IWMReaderCallback>,
    {
        (::windows::core::Interface::vtable(self).base__.OpenStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopNetStreaming)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartAtPosition)(::windows::core::Interface::as_raw(self), wstreamnum, pvoffsetstart, pvduration, dwoffsetformat, frate, pvcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderAdvanced3, ::windows::core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2);
impl ::core::cmp::PartialEq for IWMReaderAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced3 {}
impl ::core::fmt::Debug for IWMReaderAdvanced3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced3 {
    type Vtable = IWMReaderAdvanced3_Vtbl;
}
impl ::core::clone::Clone for IWMReaderAdvanced3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderAdvanced3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5dc0674b_f04b_4a4e_9f2a_b1afde2c8100);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced3_Vtbl {
    pub base__: IWMReaderAdvanced2_Vtbl,
    pub StopNetStreaming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartAtPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced4(::windows::core::IUnknown);
impl IWMReaderAdvanced4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetUserProvidedClock)(::windows::core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetUserProvidedClock)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.DeliverTime)(::windows::core::Interface::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetManualStreamSelection)(::windows::core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetManualStreamSelection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetStreamsSelected)(::windows::core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::windows::core::zeroed::<WMT_STREAM_SELECTION>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetStreamSelected)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetAllocateForStream)(::windows::core::Interface::as_raw(self), wstreamnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetAllocateForStream)(::windows::core::Interface::as_raw(self), dwsreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetStatistics)(::windows::core::Interface::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetClientInfo)(::windows::core::Interface::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetMaxOutputSampleSize)(::windows::core::Interface::as_raw(self), dwoutput, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetMaxStreamSampleSize)(::windows::core::Interface::as_raw(self), wstream, &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.NotifyLateDelivery)(::windows::core::Interface::as_raw(self), cnslateness).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPlayMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::windows::core::zeroed::<WMT_PLAY_MODE>();
        (::windows::core::Interface::vtable(self).base__.base__.GetPlayMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetBufferProgress)(::windows::core::Interface::as_raw(self), pdwpercent, pcnsbuffering).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDownloadProgress)(::windows::core::Interface::as_raw(self), pdwpercent, pqwbytesdownloaded, pcnsdownload).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSaveAsProgress)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SaveFileAs<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SaveFileAs)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetProtocolName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszprotocol), pcchprotocol).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.StartAtMarker)(::windows::core::Interface::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Preroll)(::windows::core::Interface::as_raw(self), cnsstart, cnsduration, frate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<P0>(&self, flogclientid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetLogClientID)(::windows::core::Interface::as_raw(self), flogclientid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.GetLogClientID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.StopBuffering)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows::core::IntoParam<IWMReaderCallback>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.OpenStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StopNetStreaming)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartAtPosition)(::windows::core::Interface::as_raw(self), wstreamnum, pvoffsetstart, pvduration, dwoffsetformat, frate, pvcontext).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetLanguageCount)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLanguage)(::windows::core::Interface::as_raw(self), dwoutputnum, wlanguage, ::core::mem::transmute(pwszlanguagestring), pcchlanguagestringlength).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).GetMaxSpeedFactor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsUsingFastCache)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddLogParam<P0, P1, P2>(&self, wsznamespace: P0, wszname: P1, wszvalue: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddLogParam)(::windows::core::Interface::as_raw(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendLogParams)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanSaveFileAs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelSaveFileAs)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetURL)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszurl), pcchurl).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderAdvanced4, ::windows::core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2, IWMReaderAdvanced3);
impl ::core::cmp::PartialEq for IWMReaderAdvanced4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced4 {}
impl ::core::fmt::Debug for IWMReaderAdvanced4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced4 {
    type Vtable = IWMReaderAdvanced4_Vtbl;
}
impl ::core::clone::Clone for IWMReaderAdvanced4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderAdvanced4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x945a76a2_12ae_4d48_bd3c_cd1d90399b85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced4_Vtbl {
    pub base__: IWMReaderAdvanced3_Vtbl,
    pub GetLanguageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    pub GetMaxSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUsingFastCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUsingFastCache: usize,
    pub AddLogParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznamespace: ::windows::core::PCWSTR, wszname: ::windows::core::PCWSTR, wszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SendLogParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanSaveFileAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanSaveFileAs: usize,
    pub CancelSaveFileAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced5(::windows::core::IUnknown);
impl IWMReaderAdvanced5 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetUserProvidedClock)(::windows::core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetUserProvidedClock)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.DeliverTime)(::windows::core::Interface::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetManualStreamSelection)(::windows::core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetManualStreamSelection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetStreamsSelected)(::windows::core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::windows::core::zeroed::<WMT_STREAM_SELECTION>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetStreamSelected)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetAllocateForStream)(::windows::core::Interface::as_raw(self), wstreamnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetAllocateForStream)(::windows::core::Interface::as_raw(self), dwsreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetStatistics)(::windows::core::Interface::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetClientInfo)(::windows::core::Interface::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetMaxOutputSampleSize)(::windows::core::Interface::as_raw(self), dwoutput, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetMaxStreamSampleSize)(::windows::core::Interface::as_raw(self), wstream, &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.NotifyLateDelivery)(::windows::core::Interface::as_raw(self), cnslateness).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPlayMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::windows::core::zeroed::<WMT_PLAY_MODE>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPlayMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetBufferProgress)(::windows::core::Interface::as_raw(self), pdwpercent, pcnsbuffering).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDownloadProgress)(::windows::core::Interface::as_raw(self), pdwpercent, pqwbytesdownloaded, pcnsdownload).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetSaveAsProgress)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SaveFileAs<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SaveFileAs)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetProtocolName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszprotocol), pcchprotocol).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.StartAtMarker)(::windows::core::Interface::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Preroll)(::windows::core::Interface::as_raw(self), cnsstart, cnsduration, frate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<P0>(&self, flogclientid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetLogClientID)(::windows::core::Interface::as_raw(self), flogclientid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetLogClientID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.StopBuffering)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows::core::IntoParam<IWMReaderCallback>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.OpenStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.StopNetStreaming)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.StartAtPosition)(::windows::core::Interface::as_raw(self), wstreamnum, pvoffsetstart, pvduration, dwoffsetformat, frate, pvcontext).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetLanguageCount)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLanguage)(::windows::core::Interface::as_raw(self), dwoutputnum, wlanguage, ::core::mem::transmute(pwszlanguagestring), pcchlanguagestringlength).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).base__.GetMaxSpeedFactor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsUsingFastCache)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddLogParam<P0, P1, P2>(&self, wsznamespace: P0, wszname: P1, wszvalue: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddLogParam)(::windows::core::Interface::as_raw(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SendLogParams)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.CanSaveFileAs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CancelSaveFileAs)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetURL)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszurl), pcchurl).ok()
    }
    pub unsafe fn SetPlayerHook<P0>(&self, dwoutputnum: u32, phook: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMPlayerHook>,
    {
        (::windows::core::Interface::vtable(self).SetPlayerHook)(::windows::core::Interface::as_raw(self), dwoutputnum, phook.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderAdvanced5, ::windows::core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2, IWMReaderAdvanced3, IWMReaderAdvanced4);
impl ::core::cmp::PartialEq for IWMReaderAdvanced5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced5 {}
impl ::core::fmt::Debug for IWMReaderAdvanced5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced5 {
    type Vtable = IWMReaderAdvanced5_Vtbl;
}
impl ::core::clone::Clone for IWMReaderAdvanced5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderAdvanced5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24c44db0_55d1_49ae_a5cc_f13815e36363);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced5_Vtbl {
    pub base__: IWMReaderAdvanced4_Vtbl,
    pub SetPlayerHook: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced6(::windows::core::IUnknown);
impl IWMReaderAdvanced6 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetUserProvidedClock)(::windows::core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetUserProvidedClock)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.DeliverTime)(::windows::core::Interface::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetManualStreamSelection)(::windows::core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetManualStreamSelection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetStreamsSelected)(::windows::core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::windows::core::zeroed::<WMT_STREAM_SELECTION>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetStreamSelected)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetReceiveSelectionCallbacks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetReceiveStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetAllocateForStream)(::windows::core::Interface::as_raw(self), wstreamnum, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetAllocateForStream)(::windows::core::Interface::as_raw(self), dwsreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetStatistics)(::windows::core::Interface::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetClientInfo)(::windows::core::Interface::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetMaxOutputSampleSize)(::windows::core::Interface::as_raw(self), dwoutput, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetMaxStreamSampleSize)(::windows::core::Interface::as_raw(self), wstream, &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.NotifyLateDelivery)(::windows::core::Interface::as_raw(self), cnslateness).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPlayMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::windows::core::zeroed::<WMT_PLAY_MODE>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetPlayMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetBufferProgress)(::windows::core::Interface::as_raw(self), pdwpercent, pcnsbuffering).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDownloadProgress)(::windows::core::Interface::as_raw(self), pdwpercent, pqwbytesdownloaded, pcnsdownload).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetSaveAsProgress)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SaveFileAs<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SaveFileAs)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetProtocolName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszprotocol), pcchprotocol).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.StartAtMarker)(::windows::core::Interface::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.Preroll)(::windows::core::Interface::as_raw(self), cnsstart, cnsduration, frate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<P0>(&self, flogclientid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetLogClientID)(::windows::core::Interface::as_raw(self), flogclientid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetLogClientID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.StopBuffering)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows::core::IntoParam<IWMReaderCallback>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.OpenStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.StopNetStreaming)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.StartAtPosition)(::windows::core::Interface::as_raw(self), wstreamnum, pvoffsetstart, pvduration, dwoffsetformat, frate, pvcontext).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.base__.GetLanguageCount)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetLanguage)(::windows::core::Interface::as_raw(self), dwoutputnum, wlanguage, ::core::mem::transmute(pwszlanguagestring), pcchlanguagestringlength).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMaxSpeedFactor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.IsUsingFastCache)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddLogParam<P0, P1, P2>(&self, wsznamespace: P0, wszname: P1, wszvalue: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddLogParam)(::windows::core::Interface::as_raw(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SendLogParams)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.CanSaveFileAs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CancelSaveFileAs)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetURL)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszurl), pcchurl).ok()
    }
    pub unsafe fn SetPlayerHook<P0>(&self, dwoutputnum: u32, phook: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMPlayerHook>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPlayerHook)(::windows::core::Interface::as_raw(self), dwoutputnum, phook.into_param().abi()).ok()
    }
    pub unsafe fn SetProtectStreamSamples(&self, pbcertificate: &[u8], dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProtectStreamSamples)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbcertificate.as_ptr()), pbcertificate.len() as _, dwcertificatetype, dwflags, pbinitializationvector, pcbinitializationvector).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderAdvanced6, ::windows::core::IUnknown, IWMReaderAdvanced, IWMReaderAdvanced2, IWMReaderAdvanced3, IWMReaderAdvanced4, IWMReaderAdvanced5);
impl ::core::cmp::PartialEq for IWMReaderAdvanced6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced6 {}
impl ::core::fmt::Debug for IWMReaderAdvanced6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced6").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced6 {
    type Vtable = IWMReaderAdvanced6_Vtbl;
}
impl ::core::clone::Clone for IWMReaderAdvanced6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderAdvanced6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18a2e7f8_428f_4acd_8a00_e64639bc93de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced6_Vtbl {
    pub base__: IWMReaderAdvanced5_Vtbl,
    pub SetProtectStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderAllocatorEx(::windows::core::IUnknown);
impl IWMReaderAllocatorEx {
    pub unsafe fn AllocateForStreamEx(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateForStreamEx)(::windows::core::Interface::as_raw(self), wstreamnum, cbbuffer, ::core::mem::transmute(ppbuffer), dwflags, cnssampletime, cnssampleduration, pvcontext).ok()
    }
    pub unsafe fn AllocateForOutputEx(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateForOutputEx)(::windows::core::Interface::as_raw(self), dwoutputnum, cbbuffer, ::core::mem::transmute(ppbuffer), dwflags, cnssampletime, cnssampleduration, pvcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderAllocatorEx, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMReaderAllocatorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAllocatorEx {}
impl ::core::fmt::Debug for IWMReaderAllocatorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAllocatorEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAllocatorEx {
    type Vtable = IWMReaderAllocatorEx_Vtbl;
}
impl ::core::clone::Clone for IWMReaderAllocatorEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderAllocatorEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f762fa7_a22e_428d_93c9_ac82f3aafe5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAllocatorEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AllocateForStreamEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateForOutputEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderCallback(::windows::core::IUnknown);
impl IWMReaderCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnStatus)(::windows::core::Interface::as_raw(self), status, hr, dwtype, pvalue, pvcontext).ok()
    }
    pub unsafe fn OnSample<P0>(&self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: P0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).OnSample)(::windows::core::Interface::as_raw(self), dwoutputnum, cnssampletime, cnssampleduration, dwflags, psample.into_param().abi(), pvcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderCallback, ::windows::core::IUnknown, IWMStatusCallback);
impl ::core::cmp::PartialEq for IWMReaderCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderCallback {}
impl ::core::fmt::Debug for IWMReaderCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderCallback {
    type Vtable = IWMReaderCallback_Vtbl;
}
impl ::core::clone::Clone for IWMReaderCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd8_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallback_Vtbl {
    pub base__: IWMStatusCallback_Vtbl,
    pub OnSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderCallbackAdvanced(::windows::core::IUnknown);
impl IWMReaderCallbackAdvanced {
    pub unsafe fn OnStreamSample<P0>(&self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: P0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).OnStreamSample)(::windows::core::Interface::as_raw(self), wstreamnum, cnssampletime, cnssampleduration, dwflags, psample.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn OnTime(&self, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTime)(::windows::core::Interface::as_raw(self), cnscurrenttime, pvcontext).ok()
    }
    pub unsafe fn OnStreamSelection(&self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStreamSelection)(::windows::core::Interface::as_raw(self), wstreamcount, pstreamnumbers, pselections, pvcontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutputPropsChanged(&self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutputPropsChanged)(::windows::core::Interface::as_raw(self), dwoutputnum, pmediatype, pvcontext).ok()
    }
    pub unsafe fn AllocateForStream(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateForStream)(::windows::core::Interface::as_raw(self), wstreamnum, cbbuffer, ::core::mem::transmute(ppbuffer), pvcontext).ok()
    }
    pub unsafe fn AllocateForOutput(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, cbbuffer, ::core::mem::transmute(ppbuffer), pvcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderCallbackAdvanced, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMReaderCallbackAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderCallbackAdvanced {}
impl ::core::fmt::Debug for IWMReaderCallbackAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderCallbackAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderCallbackAdvanced {
    type Vtable = IWMReaderCallbackAdvanced_Vtbl;
}
impl ::core::clone::Clone for IWMReaderCallbackAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderCallbackAdvanced {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406beb_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallbackAdvanced_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStreamSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnStreamSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnOutputPropsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnOutputPropsChanged: usize,
    pub AllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderNetworkConfig(::windows::core::IUnknown);
impl IWMReaderNetworkConfig {
    pub unsafe fn GetBufferingTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetBufferingTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferingTime)(::windows::core::Interface::as_raw(self), cnsbufferingtime).ok()
    }
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUDPPortRanges)(::windows::core::Interface::as_raw(self), prangearray, pcranges).ok()
    }
    pub unsafe fn SetUDPPortRanges(&self, prangearray: &[WM_PORT_NUMBER_RANGE]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUDPPortRanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prangearray.as_ptr()), prangearray.len() as _).ok()
    }
    pub unsafe fn GetProxySettings<P0>(&self, pwszprotocol: P0) -> ::windows::core::Result<WMT_PROXY_SETTINGS>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<WMT_PROXY_SETTINGS>();
        (::windows::core::Interface::vtable(self).GetProxySettings)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProxySettings<P0>(&self, pwszprotocol: P0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProxySettings)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), proxysetting).ok()
    }
    pub unsafe fn GetProxyHostName<P0>(&self, pwszprotocol: P0, pwszhostname: ::windows::core::PWSTR, pcchhostname: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetProxyHostName)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszhostname), pcchhostname).ok()
    }
    pub unsafe fn SetProxyHostName<P0, P1>(&self, pwszprotocol: P0, pwszhostname: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProxyHostName)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), pwszhostname.into_param().abi()).ok()
    }
    pub unsafe fn GetProxyPort<P0>(&self, pwszprotocol: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetProxyPort)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProxyPort<P0>(&self, pwszprotocol: P0, dwport: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProxyPort)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), dwport).ok()
    }
    pub unsafe fn GetProxyExceptionList<P0>(&self, pwszprotocol: P0, pwszexceptionlist: ::windows::core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetProxyExceptionList)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszexceptionlist), pcchexceptionlist).ok()
    }
    pub unsafe fn SetProxyExceptionList<P0, P1>(&self, pwszprotocol: P0, pwszexceptionlist: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProxyExceptionList)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), pwszexceptionlist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyBypassForLocal<P0>(&self, pwszprotocol: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetProxyBypassForLocal)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyBypassForLocal<P0, P1>(&self, pwszprotocol: P0, fbypassforlocal: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetProxyBypassForLocal)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetForceRerunAutoProxyDetection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetForceRerunAutoProxyDetection<P0>(&self, fforcererundetection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetForceRerunAutoProxyDetection)(::windows::core::Interface::as_raw(self), fforcererundetection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableMulticast(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetEnableMulticast)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableMulticast<P0>(&self, fenablemulticast: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnableMulticast)(::windows::core::Interface::as_raw(self), fenablemulticast.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableHTTP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetEnableHTTP)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableHTTP<P0>(&self, fenablehttp: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnableHTTP)(::windows::core::Interface::as_raw(self), fenablehttp.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableUDP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetEnableUDP)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableUDP<P0>(&self, fenableudp: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnableUDP)(::windows::core::Interface::as_raw(self), fenableudp.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableTCP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetEnableTCP)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableTCP<P0>(&self, fenabletcp: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnableTCP)(::windows::core::Interface::as_raw(self), fenabletcp.into_param().abi()).ok()
    }
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetProtocolRollover)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetConnectionBandwidth)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConnectionBandwidth)(::windows::core::Interface::as_raw(self), dwconnectionbandwidth).ok()
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetNumProtocolsSupported)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSupportedProtocolName)(::windows::core::Interface::as_raw(self), dwprotocolnum, ::core::mem::transmute(pwszprotocolname), pcchprotocolname).ok()
    }
    pub unsafe fn AddLoggingUrl<P0>(&self, pwszurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddLoggingUrl)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi()).ok()
    }
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLoggingUrl)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(pwszurl), pcchurl).ok()
    }
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLoggingUrlCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetLoggingUrlList)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderNetworkConfig, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMReaderNetworkConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderNetworkConfig {}
impl ::core::fmt::Debug for IWMReaderNetworkConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderNetworkConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderNetworkConfig {
    type Vtable = IWMReaderNetworkConfig_Vtbl;
}
impl ::core::clone::Clone for IWMReaderNetworkConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderNetworkConfig {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bec_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBufferingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows::core::HRESULT,
    pub SetBufferingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows::core::HRESULT,
    pub GetUDPPortRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::HRESULT,
    pub SetUDPPortRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::HRESULT,
    pub GetProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT,
    pub SetProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT,
    pub GetProxyHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszhostname: ::windows::core::PWSTR, pcchhostname: *mut u32) -> ::windows::core::HRESULT,
    pub SetProxyHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszhostname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetProxyPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pdwport: *mut u32) -> ::windows::core::HRESULT,
    pub SetProxyPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, dwport: u32) -> ::windows::core::HRESULT,
    pub GetProxyExceptionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszexceptionlist: ::windows::core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::HRESULT,
    pub SetProxyExceptionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszexceptionlist: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProxyBypassForLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProxyBypassForLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProxyBypassForLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProxyBypassForLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForceRerunAutoProxyDetection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForceRerunAutoProxyDetection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetForceRerunAutoProxyDetection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetForceRerunAutoProxyDetection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableMulticast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableMulticast: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableMulticast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableMulticast: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableHTTP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableHTTP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableHTTP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableHTTP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableUDP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableUDP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableUDP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableUDP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableTCP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableTCP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableTCP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableTCP: usize,
    pub ResetProtocolRollover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConnectionBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows::core::HRESULT,
    pub SetConnectionBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows::core::HRESULT,
    pub GetNumProtocolsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT,
    pub GetSupportedProtocolName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::HRESULT,
    pub AddLoggingUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetLoggingUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
    pub GetLoggingUrlCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows::core::HRESULT,
    pub ResetLoggingUrlList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderNetworkConfig2(::windows::core::IUnknown);
impl IWMReaderNetworkConfig2 {
    pub unsafe fn GetBufferingTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).base__.GetBufferingTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBufferingTime)(::windows::core::Interface::as_raw(self), cnsbufferingtime).ok()
    }
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetUDPPortRanges)(::windows::core::Interface::as_raw(self), prangearray, pcranges).ok()
    }
    pub unsafe fn SetUDPPortRanges(&self, prangearray: &[WM_PORT_NUMBER_RANGE]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetUDPPortRanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prangearray.as_ptr()), prangearray.len() as _).ok()
    }
    pub unsafe fn GetProxySettings<P0>(&self, pwszprotocol: P0) -> ::windows::core::Result<WMT_PROXY_SETTINGS>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<WMT_PROXY_SETTINGS>();
        (::windows::core::Interface::vtable(self).base__.GetProxySettings)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProxySettings<P0>(&self, pwszprotocol: P0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetProxySettings)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), proxysetting).ok()
    }
    pub unsafe fn GetProxyHostName<P0>(&self, pwszprotocol: P0, pwszhostname: ::windows::core::PWSTR, pcchhostname: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetProxyHostName)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszhostname), pcchhostname).ok()
    }
    pub unsafe fn SetProxyHostName<P0, P1>(&self, pwszprotocol: P0, pwszhostname: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetProxyHostName)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), pwszhostname.into_param().abi()).ok()
    }
    pub unsafe fn GetProxyPort<P0>(&self, pwszprotocol: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetProxyPort)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProxyPort<P0>(&self, pwszprotocol: P0, dwport: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetProxyPort)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), dwport).ok()
    }
    pub unsafe fn GetProxyExceptionList<P0>(&self, pwszprotocol: P0, pwszexceptionlist: ::windows::core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetProxyExceptionList)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszexceptionlist), pcchexceptionlist).ok()
    }
    pub unsafe fn SetProxyExceptionList<P0, P1>(&self, pwszprotocol: P0, pwszexceptionlist: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetProxyExceptionList)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), pwszexceptionlist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyBypassForLocal<P0>(&self, pwszprotocol: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetProxyBypassForLocal)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyBypassForLocal<P0, P1>(&self, pwszprotocol: P0, fbypassforlocal: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetProxyBypassForLocal)(::windows::core::Interface::as_raw(self), pwszprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetForceRerunAutoProxyDetection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetForceRerunAutoProxyDetection<P0>(&self, fforcererundetection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetForceRerunAutoProxyDetection)(::windows::core::Interface::as_raw(self), fforcererundetection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableMulticast(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetEnableMulticast)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableMulticast<P0>(&self, fenablemulticast: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEnableMulticast)(::windows::core::Interface::as_raw(self), fenablemulticast.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableHTTP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetEnableHTTP)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableHTTP<P0>(&self, fenablehttp: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEnableHTTP)(::windows::core::Interface::as_raw(self), fenablehttp.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableUDP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetEnableUDP)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableUDP<P0>(&self, fenableudp: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEnableUDP)(::windows::core::Interface::as_raw(self), fenableudp.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableTCP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetEnableTCP)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableTCP<P0>(&self, fenabletcp: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEnableTCP)(::windows::core::Interface::as_raw(self), fenabletcp.into_param().abi()).ok()
    }
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ResetProtocolRollover)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetConnectionBandwidth)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetConnectionBandwidth)(::windows::core::Interface::as_raw(self), dwconnectionbandwidth).ok()
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetNumProtocolsSupported)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSupportedProtocolName)(::windows::core::Interface::as_raw(self), dwprotocolnum, ::core::mem::transmute(pwszprotocolname), pcchprotocolname).ok()
    }
    pub unsafe fn AddLoggingUrl<P0>(&self, pwszurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddLoggingUrl)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi()).ok()
    }
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLoggingUrl)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(pwszurl), pcchurl).ok()
    }
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetLoggingUrlCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ResetLoggingUrlList)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableContentCaching(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetEnableContentCaching)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableContentCaching<P0>(&self, fenablecontentcaching: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnableContentCaching)(::windows::core::Interface::as_raw(self), fenablecontentcaching.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetEnableFastCache)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableFastCache<P0>(&self, fenablefastcache: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnableFastCache)(::windows::core::Interface::as_raw(self), fenablefastcache.into_param().abi()).ok()
    }
    pub unsafe fn GetAcceleratedStreamingDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetAcceleratedStreamingDuration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAcceleratedStreamingDuration(&self, cnsaccelduration: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAcceleratedStreamingDuration)(::windows::core::Interface::as_raw(self), cnsaccelduration).ok()
    }
    pub unsafe fn GetAutoReconnectLimit(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetAutoReconnectLimit)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAutoReconnectLimit(&self, dwautoreconnectlimit: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAutoReconnectLimit)(::windows::core::Interface::as_raw(self), dwautoreconnectlimit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableResends(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetEnableResends)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableResends<P0>(&self, fenableresends: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnableResends)(::windows::core::Interface::as_raw(self), fenableresends.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableThinning(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetEnableThinning)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableThinning<P0>(&self, fenablethinning: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnableThinning)(::windows::core::Interface::as_raw(self), fenablethinning.into_param().abi()).ok()
    }
    pub unsafe fn GetMaxNetPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxNetPacketSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMReaderNetworkConfig2, ::windows::core::IUnknown, IWMReaderNetworkConfig);
impl ::core::cmp::PartialEq for IWMReaderNetworkConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderNetworkConfig2 {}
impl ::core::fmt::Debug for IWMReaderNetworkConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderNetworkConfig2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderNetworkConfig2 {
    type Vtable = IWMReaderNetworkConfig2_Vtbl;
}
impl ::core::clone::Clone for IWMReaderNetworkConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderNetworkConfig2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd979a853_042b_4050_8387_c939db22013f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig2_Vtbl {
    pub base__: IWMReaderNetworkConfig_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableContentCaching: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableContentCaching: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableContentCaching: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableContentCaching: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableFastCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableFastCache: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableFastCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableFastCache: usize,
    pub GetAcceleratedStreamingDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsaccelduration: *mut u64) -> ::windows::core::HRESULT,
    pub SetAcceleratedStreamingDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsaccelduration: u64) -> ::windows::core::HRESULT,
    pub GetAutoReconnectLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> ::windows::core::HRESULT,
    pub SetAutoReconnectLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwautoreconnectlimit: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableResends: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableResends: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableResends: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableresends: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableResends: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableThinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableThinning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableThinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablethinning: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableThinning: usize,
    pub GetMaxNetPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderPlaylistBurn(::windows::core::IUnknown);
impl IWMReaderPlaylistBurn {
    pub unsafe fn InitPlaylistBurn<P0>(&self, cfiles: u32, ppwszfilenames: *const ::windows::core::PCWSTR, pcallback: P0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).InitPlaylistBurn)(::windows::core::Interface::as_raw(self), cfiles, ppwszfilenames, pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn GetInitResults(&self, cfiles: u32) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
        (::windows::core::Interface::vtable(self).GetInitResults)(::windows::core::Interface::as_raw(self), cfiles, &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndPlaylistBurn(&self, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndPlaylistBurn)(::windows::core::Interface::as_raw(self), hrburnresult).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderPlaylistBurn, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMReaderPlaylistBurn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderPlaylistBurn {}
impl ::core::fmt::Debug for IWMReaderPlaylistBurn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderPlaylistBurn").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderPlaylistBurn {
    type Vtable = IWMReaderPlaylistBurn_Vtbl;
}
impl ::core::clone::Clone for IWMReaderPlaylistBurn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderPlaylistBurn {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf28c0300_9baa_4477_a846_1744d9cbf533);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderPlaylistBurn_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InitPlaylistBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfiles: u32, ppwszfilenames: *const ::windows::core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInitResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfiles: u32, phrstati: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndPlaylistBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderStreamClock(::windows::core::IUnknown);
impl IWMReaderStreamClock {
    pub unsafe fn GetTime(&self, pcnsnow: *const u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTime)(::windows::core::Interface::as_raw(self), pcnsnow).ok()
    }
    pub unsafe fn SetTimer(&self, cnswhen: u64, pvparam: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SetTimer)(::windows::core::Interface::as_raw(self), cnswhen, pvparam, &mut result__).from_abi(result__)
    }
    pub unsafe fn KillTimer(&self, dwtimerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KillTimer)(::windows::core::Interface::as_raw(self), dwtimerid).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderStreamClock, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMReaderStreamClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderStreamClock {}
impl ::core::fmt::Debug for IWMReaderStreamClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderStreamClock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderStreamClock {
    type Vtable = IWMReaderStreamClock_Vtbl;
}
impl ::core::clone::Clone for IWMReaderStreamClock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderStreamClock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bed_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderStreamClock_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsnow: *const u64) -> ::windows::core::HRESULT,
    pub SetTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows::core::HRESULT,
    pub KillTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimerid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderTimecode(::windows::core::IUnknown);
impl IWMReaderTimecode {
    pub unsafe fn GetTimecodeRangeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetTimecodeRangeCount)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTimecodeRangeBounds(&self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTimecodeRangeBounds)(::windows::core::Interface::as_raw(self), wstreamnum, wrangenum, pstarttimecode, pendtimecode).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderTimecode, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMReaderTimecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderTimecode {}
impl ::core::fmt::Debug for IWMReaderTimecode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderTimecode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderTimecode {
    type Vtable = IWMReaderTimecode_Vtbl;
}
impl ::core::clone::Clone for IWMReaderTimecode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderTimecode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf369e2f0_e081_4fe6_8450_b810b2f410d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTimecode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTimecodeRangeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows::core::HRESULT,
    pub GetTimecodeRangeBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMReaderTypeNegotiation(::windows::core::IUnknown);
impl IWMReaderTypeNegotiation {
    pub unsafe fn TryOutputProps<P0>(&self, dwoutputnum: u32, poutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMOutputMediaProps>,
    {
        (::windows::core::Interface::vtable(self).TryOutputProps)(::windows::core::Interface::as_raw(self), dwoutputnum, poutput.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMReaderTypeNegotiation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMReaderTypeNegotiation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderTypeNegotiation {}
impl ::core::fmt::Debug for IWMReaderTypeNegotiation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderTypeNegotiation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderTypeNegotiation {
    type Vtable = IWMReaderTypeNegotiation_Vtbl;
}
impl ::core::clone::Clone for IWMReaderTypeNegotiation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMReaderTypeNegotiation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdbe5592_81a1_41ea_93bd_735cad1adc05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTypeNegotiation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub TryOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMRegisterCallback(::windows::core::IUnknown);
impl IWMRegisterCallback {
    pub unsafe fn Advise<P0>(&self, pcallback: P0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).Advise)(::windows::core::Interface::as_raw(self), pcallback.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn Unadvise<P0>(&self, pcallback: P0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).Unadvise)(::windows::core::Interface::as_raw(self), pcallback.into_param().abi(), pvcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMRegisterCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMRegisterCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMRegisterCallback {}
impl ::core::fmt::Debug for IWMRegisterCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMRegisterCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMRegisterCallback {
    type Vtable = IWMRegisterCallback_Vtbl;
}
impl ::core::clone::Clone for IWMRegisterCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMRegisterCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf4b1f99_4de2_4e49_a363_252740d99bc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisterCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMRegisteredDevice(::windows::core::IUnknown);
impl IWMRegisteredDevice {
    pub unsafe fn GetDeviceSerialNumber(&self) -> ::windows::core::Result<DRM_VAL16> {
        let mut result__ = ::windows::core::zeroed::<DRM_VAL16>();
        (::windows::core::Interface::vtable(self).GetDeviceSerialNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceCertificate(&self) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::windows::core::zeroed::<INSSBuffer>();
        (::windows::core::Interface::vtable(self).GetDeviceCertificate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDeviceType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAttributeCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetAttributeCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, dwindex: u32, pbstrname: *mut ::windows::core::BSTR, pbstrvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributeByIndex)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrvalue)).ok()
    }
    pub unsafe fn GetAttributeByName<P0>(&self, bstrname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetAttributeByName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAttributeByName<P0, P1>(&self, bstrname: P0, bstrvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAttributeByName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Approve<P0>(&self, fapprove: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Approve)(::windows::core::Interface::as_raw(self), fapprove.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsValid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsApproved(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsApproved)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWmdrmCompliant(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsWmdrmCompliant)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOpened(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsOpened)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMRegisteredDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMRegisteredDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMRegisteredDevice {}
impl ::core::fmt::Debug for IWMRegisteredDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMRegisteredDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMRegisteredDevice {
    type Vtable = IWMRegisteredDevice_Vtbl;
}
impl ::core::clone::Clone for IWMRegisteredDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMRegisteredDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4503bec_5508_4148_97ac_bfa75760a70d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisteredDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDeviceSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT,
    pub GetDeviceCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcertificate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcattributes: *mut u32) -> ::windows::core::HRESULT,
    pub GetAttributeByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Approve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fapprove: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Approve: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsValid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsApproved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsApproved: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWmdrmCompliant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWmdrmCompliant: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfopened: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOpened: usize,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMSBufferAllocator(::windows::core::IUnknown);
impl IWMSBufferAllocator {
    pub unsafe fn AllocateBuffer(&self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::windows::core::zeroed::<INSSBuffer>();
        (::windows::core::Interface::vtable(self).AllocateBuffer)(::windows::core::Interface::as_raw(self), dwmaxbuffersize, &mut result__).from_abi(result__)
    }
    pub unsafe fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::windows::core::zeroed::<INSSBuffer>();
        (::windows::core::Interface::vtable(self).AllocatePageSizeBuffer)(::windows::core::Interface::as_raw(self), dwmaxbuffersize, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMSBufferAllocator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMSBufferAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSBufferAllocator {}
impl ::core::fmt::Debug for IWMSBufferAllocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSBufferAllocator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSBufferAllocator {
    type Vtable = IWMSBufferAllocator_Vtbl;
}
impl ::core::clone::Clone for IWMSBufferAllocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMSBufferAllocator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61103ca4_2033_11d2_9ef1_006097d2d7cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSBufferAllocator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AllocateBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocatePageSizeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource(::windows::core::IUnknown);
impl IWMSInternalAdminNetSource {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0, P1, P2, P3>(&self, psharednamespace: P0, pnamespacenode: P1, pnetsourcecreator: P2, fembeddedinserver: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P2: ::windows::core::IntoParam<INSNetSourceCreator>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), psharednamespace.into_param().abi(), pnamespacenode.into_param().abi(), pnetsourcecreator.into_param().abi(), fembeddedinserver.into_param().abi()).ok()
    }
    pub unsafe fn GetNetSourceCreator(&self) -> ::windows::core::Result<INSNetSourceCreator> {
        let mut result__ = ::windows::core::zeroed::<INSNetSourceCreator>();
        (::windows::core::Interface::vtable(self).GetNetSourceCreator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials<P0, P1, P2, P3, P4>(&self, bstrrealm: P0, bstrname: P1, bstrpassword: P2, fpersist: P3, fconfirmedgood: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetCredentials)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentials<P0>(&self, bstrrealm: P0, pbstrname: *mut ::windows::core::BSTR, pbstrpassword: *mut ::windows::core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).GetCredentials)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi(), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), pfconfirmedgood).ok()
    }
    pub unsafe fn DeleteCredentials<P0>(&self, bstrrealm: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteCredentials)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi()).ok()
    }
    pub unsafe fn GetCredentialFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCredentialFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCredentialFlags(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCredentialFlags)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURL<P0, P1>(&self, bstrprotocol: P0, bstrhost: P1, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows::core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).FindProxyForURL)(::windows::core::Interface::as_raw(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), pfproxyenabled, ::core::mem::transmute(pbstrproxyserver), pdwproxyport, pdwproxycontext).ok()
    }
    pub unsafe fn RegisterProxyFailure(&self, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterProxyFailure)(::windows::core::Interface::as_raw(self), hrparam, dwproxycontext).ok()
    }
    pub unsafe fn ShutdownProxyContext(&self, dwproxycontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShutdownProxyContext)(::windows::core::Interface::as_raw(self), dwproxycontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingIE(&self, dwproxycontext: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsUsingIE)(::windows::core::Interface::as_raw(self), dwproxycontext, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMSInternalAdminNetSource, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSInternalAdminNetSource {
    type Vtable = IWMSInternalAdminNetSource_Vtbl;
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMSInternalAdminNetSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bb23e5f_d127_4afb_8d02_ae5b66d54c78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: *mut ::core::ffi::c_void, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetNetSourceCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentials: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrpassword: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCredentials: usize,
    pub DeleteCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetCredentialFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub SetCredentialFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindProxyForURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrhost: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindProxyForURL: usize,
    pub RegisterProxyFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::HRESULT,
    pub ShutdownProxyContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproxycontext: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUsingIE: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUsingIE: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource2(::windows::core::IUnknown);
impl IWMSInternalAdminNetSource2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentialsEx<P0, P1, P2, P3, P4, P5, P6>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2, bstrname: P3, bstrpassword: P4, fpersist: P5, fconfirmedgood: P6) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P6: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetCredentialsEx)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentialsEx<P0, P1, P2>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::windows::core::BSTR, pbstrpassword: *mut ::windows::core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetCredentialsEx)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), pdwurlpolicy, ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), pfconfirmedgood).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteCredentialsEx<P0, P1, P2>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).DeleteCredentialsEx)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURLEx<P0, P1, P2>(&self, bstrprotocol: P0, bstrhost: P1, bstrurl: P2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows::core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).FindProxyForURLEx)(::windows::core::Interface::as_raw(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), pfproxyenabled, ::core::mem::transmute(pbstrproxyserver), pdwproxyport, pdwproxycontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMSInternalAdminNetSource2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource2 {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSInternalAdminNetSource2 {
    type Vtable = IWMSInternalAdminNetSource2_Vtbl;
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMSInternalAdminNetSource2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe74d58c3_cf77_4b51_af17_744687c43eae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentialsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentialsEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCredentialsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrpassword: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCredentialsEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteCredentialsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteCredentialsEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FindProxyForURLEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrhost: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindProxyForURLEx: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource3(::windows::core::IUnknown);
impl IWMSInternalAdminNetSource3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentialsEx<P0, P1, P2, P3, P4, P5, P6>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2, bstrname: P3, bstrpassword: P4, fpersist: P5, fconfirmedgood: P6) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P6: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetCredentialsEx)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentialsEx<P0, P1, P2>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::windows::core::BSTR, pbstrpassword: *mut ::windows::core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.GetCredentialsEx)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), pdwurlpolicy, ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), pfconfirmedgood).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteCredentialsEx<P0, P1, P2>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteCredentialsEx)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURLEx<P0, P1, P2>(&self, bstrprotocol: P0, bstrhost: P1, bstrurl: P2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows::core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.FindProxyForURLEx)(::windows::core::Interface::as_raw(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), pfproxyenabled, ::core::mem::transmute(pbstrproxyserver), pdwproxyport, pdwproxycontext).ok()
    }
    pub unsafe fn GetNetSourceCreator2(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetNetSourceCreator2)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURLEx2<P0, P1, P2>(&self, bstrprotocol: P0, bstrhost: P1, bstrurl: P2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows::core::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).FindProxyForURLEx2)(::windows::core::Interface::as_raw(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), pfproxyenabled, ::core::mem::transmute(pbstrproxyserver), pdwproxyport, pqwproxycontext).ok()
    }
    pub unsafe fn RegisterProxyFailure2(&self, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterProxyFailure2)(::windows::core::Interface::as_raw(self), hrparam, qwproxycontext).ok()
    }
    pub unsafe fn ShutdownProxyContext2(&self, qwproxycontext: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShutdownProxyContext2)(::windows::core::Interface::as_raw(self), qwproxycontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingIE2(&self, qwproxycontext: u64) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsUsingIE2)(::windows::core::Interface::as_raw(self), qwproxycontext, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentialsEx2<P0, P1, P2, P3, P4, P5, P6, P7>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2, bstrname: P3, bstrpassword: P4, fpersist: P5, fconfirmedgood: P6, fcleartextauthentication: P7) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P6: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P7: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetCredentialsEx2)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi(), fcleartextauthentication.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentialsEx2<P0, P1, P2, P3>(&self, bstrrealm: P0, bstrurl: P1, fproxy: P2, fcleartextauthentication: P3, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::windows::core::BSTR, pbstrpassword: *mut ::windows::core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetCredentialsEx2)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), fcleartextauthentication.into_param().abi(), pdwurlpolicy, ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), pfconfirmedgood).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMSInternalAdminNetSource3, ::windows::core::IUnknown, IWMSInternalAdminNetSource2);
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource3 {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSInternalAdminNetSource3 {
    type Vtable = IWMSInternalAdminNetSource3_Vtbl;
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMSInternalAdminNetSource3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b63d08e_4590_44af_9eb3_57ff1e73bf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource3_Vtbl {
    pub base__: IWMSInternalAdminNetSource2_Vtbl,
    pub GetNetSourceCreator2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindProxyForURLEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrhost: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindProxyForURLEx2: usize,
    pub RegisterProxyFailure2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::HRESULT,
    pub ShutdownProxyContext2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qwproxycontext: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUsingIE2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUsingIE2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentialsEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentialsEx2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCredentialsEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrpassword: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCredentialsEx2: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMSecureChannel(::windows::core::IUnknown);
impl IWMSecureChannel {
    pub unsafe fn GetCertCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCertCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::windows::core::zeroed::<*mut u8>();
        (::windows::core::Interface::vtable(self).base__.GetCert)(::windows::core::Interface::as_raw(self), dwindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::windows::core::zeroed::<*mut u8>();
        (::windows::core::Interface::vtable(self).base__.GetSharedData)(::windows::core::Interface::as_raw(self), dwcertindex, pbshareddata, pbcert, &mut result__).from_abi(result__)
    }
    pub unsafe fn WMSC_AddCertificate<P0>(&self, pcert: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMAuthorizer>,
    {
        (::windows::core::Interface::vtable(self).WMSC_AddCertificate)(::windows::core::Interface::as_raw(self), pcert.into_param().abi()).ok()
    }
    pub unsafe fn WMSC_AddSignature(&self, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_AddSignature)(::windows::core::Interface::as_raw(self), pbcertsig, cbcertsig).ok()
    }
    pub unsafe fn WMSC_Connect<P0>(&self, potherside: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMSecureChannel>,
    {
        (::windows::core::Interface::vtable(self).WMSC_Connect)(::windows::core::Interface::as_raw(self), potherside.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WMSC_IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).WMSC_IsConnected)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WMSC_Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Disconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WMSC_GetValidCertificate(&self, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_GetValidCertificate)(::windows::core::Interface::as_raw(self), ppbcertificate, pdwsignature).ok()
    }
    pub unsafe fn WMSC_Encrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Encrypt)(::windows::core::Interface::as_raw(self), pbdata, cbdata).ok()
    }
    pub unsafe fn WMSC_Decrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Decrypt)(::windows::core::Interface::as_raw(self), pbdata, cbdata).ok()
    }
    pub unsafe fn WMSC_Lock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Lock)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WMSC_Unlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Unlock)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WMSC_SetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_SetSharedData)(::windows::core::Interface::as_raw(self), dwcertindex, pbshareddata).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMSecureChannel, ::windows::core::IUnknown, IWMAuthorizer);
impl ::core::cmp::PartialEq for IWMSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSecureChannel {}
impl ::core::fmt::Debug for IWMSecureChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSecureChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSecureChannel {
    type Vtable = IWMSecureChannel_Vtbl;
}
impl ::core::clone::Clone for IWMSecureChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMSecureChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2720598a_d0f2_4189_bd10_91c46ef0936f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSecureChannel_Vtbl {
    pub base__: IWMAuthorizer_Vtbl,
    pub WMSC_AddCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcert: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WMSC_AddSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::HRESULT,
    pub WMSC_Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, potherside: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WMSC_IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WMSC_IsConnected: usize,
    pub WMSC_Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WMSC_GetValidCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::HRESULT,
    pub WMSC_Encrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT,
    pub WMSC_Decrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT,
    pub WMSC_Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WMSC_Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WMSC_SetSharedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMStatusCallback(::windows::core::IUnknown);
impl IWMStatusCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStatus)(::windows::core::Interface::as_raw(self), status, hr, dwtype, pvalue, pvcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMStatusCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStatusCallback {}
impl ::core::fmt::Debug for IWMStatusCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStatusCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStatusCallback {
    type Vtable = IWMStatusCallback_Vtbl;
}
impl ::core::clone::Clone for IWMStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMStatusCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d7cdc70_9888_11d3_8edc_00c04f6109cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStatusCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMStreamConfig(::windows::core::IUnknown);
impl IWMStreamConfig {
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetStreamType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetStreamNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamNumber)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszstreamname), pcchstreamname).ok()
    }
    pub unsafe fn SetStreamName<P0>(&self, pwszstreamname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetStreamName)(::windows::core::Interface::as_raw(self), pwszstreamname.into_param().abi()).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConnectionName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszinputname), pcchinputname).ok()
    }
    pub unsafe fn SetConnectionName<P0>(&self, pwszinputname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetConnectionName)(::windows::core::Interface::as_raw(self), pwszinputname.into_param().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetBitrate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBitrate)(::windows::core::Interface::as_raw(self), pdwbitrate).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetBufferWindow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferWindow)(::windows::core::Interface::as_raw(self), msbufferwindow).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMStreamConfig, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMStreamConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig {}
impl ::core::fmt::Debug for IWMStreamConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStreamConfig {
    type Vtable = IWMStreamConfig_Vtbl;
}
impl ::core::clone::Clone for IWMStreamConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMStreamConfig {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bdc_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStreamType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetStreamNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub SetStreamNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub GetStreamName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::HRESULT,
    pub SetStreamName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::HRESULT,
    pub SetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows::core::HRESULT,
    pub SetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows::core::HRESULT,
    pub GetBufferWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT,
    pub SetBufferWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMStreamConfig2(::windows::core::IUnknown);
impl IWMStreamConfig2 {
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetStreamType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetStreamNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetStreamNumber)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStreamName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszstreamname), pcchstreamname).ok()
    }
    pub unsafe fn SetStreamName<P0>(&self, pwszstreamname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStreamName)(::windows::core::Interface::as_raw(self), pwszstreamname.into_param().abi()).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetConnectionName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszinputname), pcchinputname).ok()
    }
    pub unsafe fn SetConnectionName<P0>(&self, pwszinputname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetConnectionName)(::windows::core::Interface::as_raw(self), pwszinputname.into_param().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetBitrate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBitrate)(::windows::core::Interface::as_raw(self), pdwbitrate).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetBufferWindow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBufferWindow)(::windows::core::Interface::as_raw(self), msbufferwindow).ok()
    }
    pub unsafe fn GetTransportType(&self) -> ::windows::core::Result<WMT_TRANSPORT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<WMT_TRANSPORT_TYPE>();
        (::windows::core::Interface::vtable(self).GetTransportType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransportType)(::windows::core::Interface::as_raw(self), ntransporttype).ok()
    }
    pub unsafe fn AddDataUnitExtension(&self, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDataUnitExtension)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidextensionsystemid), cbextensiondatasize, ::core::mem::transmute(pbextensionsysteminfo.as_ptr()), pbextensionsysteminfo.len() as _).ok()
    }
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetDataUnitExtensionCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataUnitExtension)(::windows::core::Interface::as_raw(self), wdataunitextensionnumber, pguidextensionsystemid, pcbextensiondatasize, pbextensionsysteminfo, pcbextensionsysteminfo).ok()
    }
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAllDataUnitExtensions)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMStreamConfig2, ::windows::core::IUnknown, IWMStreamConfig);
impl ::core::cmp::PartialEq for IWMStreamConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig2 {}
impl ::core::fmt::Debug for IWMStreamConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStreamConfig2 {
    type Vtable = IWMStreamConfig2_Vtbl;
}
impl ::core::clone::Clone for IWMStreamConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMStreamConfig2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7688d8cb_fc0d_43bd_9459_5a8dec200cfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig2_Vtbl {
    pub base__: IWMStreamConfig_Vtbl,
    pub GetTransportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT,
    pub SetTransportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT,
    pub AddDataUnitExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::HRESULT,
    pub GetDataUnitExtensionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows::core::HRESULT,
    pub GetDataUnitExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveAllDataUnitExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMStreamConfig3(::windows::core::IUnknown);
impl IWMStreamConfig3 {
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStreamType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStreamNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetStreamNumber)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetStreamName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszstreamname), pcchstreamname).ok()
    }
    pub unsafe fn SetStreamName<P0>(&self, pwszstreamname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetStreamName)(::windows::core::Interface::as_raw(self), pwszstreamname.into_param().abi()).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetConnectionName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszinputname), pcchinputname).ok()
    }
    pub unsafe fn SetConnectionName<P0>(&self, pwszinputname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetConnectionName)(::windows::core::Interface::as_raw(self), pwszinputname.into_param().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetBitrate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetBitrate)(::windows::core::Interface::as_raw(self), pdwbitrate).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetBufferWindow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetBufferWindow)(::windows::core::Interface::as_raw(self), msbufferwindow).ok()
    }
    pub unsafe fn GetTransportType(&self) -> ::windows::core::Result<WMT_TRANSPORT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<WMT_TRANSPORT_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetTransportType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetTransportType)(::windows::core::Interface::as_raw(self), ntransporttype).ok()
    }
    pub unsafe fn AddDataUnitExtension(&self, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddDataUnitExtension)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidextensionsystemid), cbextensiondatasize, ::core::mem::transmute(pbextensionsysteminfo.as_ptr()), pbextensionsysteminfo.len() as _).ok()
    }
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetDataUnitExtensionCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDataUnitExtension)(::windows::core::Interface::as_raw(self), wdataunitextensionnumber, pguidextensionsystemid, pcbextensiondatasize, pbextensionsysteminfo, pcbextensionsysteminfo).ok()
    }
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveAllDataUnitExtensions)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetLanguage(&self, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLanguage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszlanguagestring), pcchlanguagestringlength).ok()
    }
    pub unsafe fn SetLanguage<P0>(&self, pwszlanguagestring: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLanguage)(::windows::core::Interface::as_raw(self), pwszlanguagestring.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMStreamConfig3, ::windows::core::IUnknown, IWMStreamConfig, IWMStreamConfig2);
impl ::core::cmp::PartialEq for IWMStreamConfig3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig3 {}
impl ::core::fmt::Debug for IWMStreamConfig3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStreamConfig3 {
    type Vtable = IWMStreamConfig3_Vtbl;
}
impl ::core::clone::Clone for IWMStreamConfig3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMStreamConfig3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb164104_3aa9_45a7_9ac9_4daee131d6e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig3_Vtbl {
    pub base__: IWMStreamConfig2_Vtbl,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMStreamList(::windows::core::IUnknown);
impl IWMStreamList {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreams)(::windows::core::Interface::as_raw(self), pwstreamnumarray, pcstreams).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddStream)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStream)(::windows::core::Interface::as_raw(self), wstreamnum).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMStreamList, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMStreamList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamList {}
impl ::core::fmt::Debug for IWMStreamList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStreamList {
    type Vtable = IWMStreamList_Vtbl;
}
impl ::core::clone::Clone for IWMStreamList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMStreamList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bdd_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMStreamPrioritization(::windows::core::IUnknown);
impl IWMStreamPrioritization {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPriorityRecords(&self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPriorityRecords)(::windows::core::Interface::as_raw(self), precordarray, pcrecords).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPriorityRecords(&self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriorityRecords)(::windows::core::Interface::as_raw(self), precordarray, crecords).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMStreamPrioritization, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMStreamPrioritization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamPrioritization {}
impl ::core::fmt::Debug for IWMStreamPrioritization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamPrioritization").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStreamPrioritization {
    type Vtable = IWMStreamPrioritization_Vtbl;
}
impl ::core::clone::Clone for IWMStreamPrioritization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMStreamPrioritization {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c1c6090_f9a8_4748_8ec3_dd1108ba1e77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamPrioritization_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPriorityRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPriorityRecords: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPriorityRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPriorityRecords: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMSyncReader(::windows::core::IUnknown);
impl IWMSyncReader {
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRange)(::windows::core::Interface::as_raw(self), cnsstarttime, cnsduration).ok()
    }
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRangeByFrame)(::windows::core::Interface::as_raw(self), wstreamnum, qwframenumber, cframestoread).ok()
    }
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextSample)(::windows::core::Interface::as_raw(self), wstreamnum, ::core::mem::transmute(ppsample), pcnssampletime, pcnsduration, pdwflags, pdwoutputnum, pwstreamnum).ok()
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamsSelected)(::windows::core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::windows::core::zeroed::<WMT_STREAM_SELECTION>();
        (::windows::core::Interface::vtable(self).GetStreamSelected)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReadStreamSamples<P0>(&self, wstreamnum: u16, fcompressed: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetReadStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, fcompressed.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetReadStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetOutputCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__ = ::windows::core::zeroed::<IWMOutputMediaProps>();
        (::windows::core::Interface::vtable(self).GetOutputProps)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOutputProps<P0>(&self, dwoutputnum: u32, poutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMOutputMediaProps>,
    {
        (::windows::core::Interface::vtable(self).SetOutputProps)(::windows::core::Interface::as_raw(self), dwoutputnum, poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetOutputFormatCount)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__ = ::windows::core::zeroed::<IWMOutputMediaProps>();
        (::windows::core::Interface::vtable(self).GetOutputFormat)(::windows::core::Interface::as_raw(self), dwoutputnum, dwformatnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetOutputNumberForStream)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetStreamNumberForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxOutputSampleSize)(::windows::core::Interface::as_raw(self), dwoutput, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxStreamSampleSize)(::windows::core::Interface::as_raw(self), wstream, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0>(&self, pstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).OpenStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMSyncReader, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMSyncReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSyncReader {}
impl ::core::fmt::Debug for IWMSyncReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSyncReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSyncReader {
    type Vtable = IWMSyncReader_Vtbl;
}
impl ::core::clone::Clone for IWMSyncReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMSyncReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9397f121_7705_4dc9_b049_98b698188414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::HRESULT,
    pub SetRangeByFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::HRESULT,
    pub GetNextSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut *mut ::core::ffi::c_void, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub SetStreamsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub GetStreamSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReadStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReadStreamSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReadStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReadStreamSamples: usize,
    pub GetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub SetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputNumberForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::core::HRESULT,
    pub GetStreamNumberForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenStream: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMSyncReader2(::windows::core::IUnknown);
impl IWMSyncReader2 {
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Open)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRange)(::windows::core::Interface::as_raw(self), cnsstarttime, cnsduration).ok()
    }
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRangeByFrame)(::windows::core::Interface::as_raw(self), wstreamnum, qwframenumber, cframestoread).ok()
    }
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNextSample)(::windows::core::Interface::as_raw(self), wstreamnum, ::core::mem::transmute(ppsample), pcnssampletime, pcnsduration, pdwflags, pdwoutputnum, pwstreamnum).ok()
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetStreamsSelected)(::windows::core::Interface::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::windows::core::zeroed::<WMT_STREAM_SELECTION>();
        (::windows::core::Interface::vtable(self).base__.GetStreamSelected)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReadStreamSamples<P0>(&self, wstreamnum: u16, fcompressed: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetReadStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, fcompressed.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GetReadStreamSamples)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetOutputSetting)(::windows::core::Interface::as_raw(self), dwoutputnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetOutputCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__ = ::windows::core::zeroed::<IWMOutputMediaProps>();
        (::windows::core::Interface::vtable(self).base__.GetOutputProps)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOutputProps<P0>(&self, dwoutputnum: u32, poutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMOutputMediaProps>,
    {
        (::windows::core::Interface::vtable(self).base__.SetOutputProps)(::windows::core::Interface::as_raw(self), dwoutputnum, poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetOutputFormatCount)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__ = ::windows::core::zeroed::<IWMOutputMediaProps>();
        (::windows::core::Interface::vtable(self).base__.GetOutputFormat)(::windows::core::Interface::as_raw(self), dwoutputnum, dwformatnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetOutputNumberForStream)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetStreamNumberForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMaxOutputSampleSize)(::windows::core::Interface::as_raw(self), dwoutput, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMaxStreamSampleSize)(::windows::core::Interface::as_raw(self), wstream, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0>(&self, pstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.OpenStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi()).ok()
    }
    pub unsafe fn SetRangeByTimecode(&self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRangeByTimecode)(::windows::core::Interface::as_raw(self), wstreamnum, pstart, pend).ok()
    }
    pub unsafe fn SetRangeByFrameEx(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).SetRangeByFrameEx)(::windows::core::Interface::as_raw(self), wstreamnum, qwframenumber, cframestoread, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, pallocator: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMReaderAllocatorEx>,
    {
        (::windows::core::Interface::vtable(self).SetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, pallocator.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMReaderAllocatorEx> {
        let mut result__ = ::windows::core::zeroed::<IWMReaderAllocatorEx>();
        (::windows::core::Interface::vtable(self).GetAllocateForOutput)(::windows::core::Interface::as_raw(self), dwoutputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, pallocator: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMReaderAllocatorEx>,
    {
        (::windows::core::Interface::vtable(self).SetAllocateForStream)(::windows::core::Interface::as_raw(self), wstreamnum, pallocator.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<IWMReaderAllocatorEx> {
        let mut result__ = ::windows::core::zeroed::<IWMReaderAllocatorEx>();
        (::windows::core::Interface::vtable(self).GetAllocateForStream)(::windows::core::Interface::as_raw(self), dwsreamnum, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMSyncReader2, ::windows::core::IUnknown, IWMSyncReader);
impl ::core::cmp::PartialEq for IWMSyncReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSyncReader2 {}
impl ::core::fmt::Debug for IWMSyncReader2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSyncReader2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSyncReader2 {
    type Vtable = IWMSyncReader2_Vtbl;
}
impl ::core::clone::Clone for IWMSyncReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMSyncReader2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaed3d21_1b6b_4af7_8cb6_3e189bbc187b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader2_Vtbl {
    pub base__: IWMSyncReader_Vtbl,
    pub SetRangeByTimecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::HRESULT,
    pub SetRangeByFrameEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows::core::HRESULT,
    pub SetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pallocator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pallocator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMVideoMediaProps(::windows::core::IUnknown);
impl IWMVideoMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMediaType)(::windows::core::Interface::as_raw(self), ptype, pcbtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMediaType)(::windows::core::Interface::as_raw(self), ptype).ok()
    }
    pub unsafe fn GetMaxKeyFrameSpacing(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).GetMaxKeyFrameSpacing)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxKeyFrameSpacing(&self, lltime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxKeyFrameSpacing)(::windows::core::Interface::as_raw(self), lltime).ok()
    }
    pub unsafe fn GetQuality(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetQuality)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetQuality(&self, dwquality: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuality)(::windows::core::Interface::as_raw(self), dwquality).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMVideoMediaProps, ::windows::core::IUnknown, IWMMediaProps);
impl ::core::cmp::PartialEq for IWMVideoMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMVideoMediaProps {}
impl ::core::fmt::Debug for IWMVideoMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMVideoMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMVideoMediaProps {
    type Vtable = IWMVideoMediaProps_Vtbl;
}
impl ::core::clone::Clone for IWMVideoMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMVideoMediaProps {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bcf_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMVideoMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    pub GetMaxKeyFrameSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plltime: *mut i64) -> ::windows::core::HRESULT,
    pub SetMaxKeyFrameSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lltime: i64) -> ::windows::core::HRESULT,
    pub GetQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwquality: *mut u32) -> ::windows::core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwquality: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWatermarkInfo(::windows::core::IUnknown);
impl IWMWatermarkInfo {
    pub unsafe fn GetWatermarkEntryCount(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetWatermarkEntryCount)(::windows::core::Interface::as_raw(self), wmettype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetWatermarkEntry(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWatermarkEntry)(::windows::core::Interface::as_raw(self), wmettype, dwentrynum, pentry).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWatermarkInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMWatermarkInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWatermarkInfo {}
impl ::core::fmt::Debug for IWMWatermarkInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWatermarkInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWatermarkInfo {
    type Vtable = IWMWatermarkInfo_Vtbl;
}
impl ::core::clone::Clone for IWMWatermarkInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWatermarkInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f497062_f2e2_4624_8ea7_9dd40d81fc8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWatermarkInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetWatermarkEntryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetWatermarkEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriter(::windows::core::IUnknown);
impl IWMWriter {
    pub unsafe fn SetProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProfileByID)(::windows::core::Interface::as_raw(self), guidprofile).ok()
    }
    pub unsafe fn SetProfile<P0>(&self, pprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMProfile>,
    {
        (::windows::core::Interface::vtable(self).SetProfile)(::windows::core::Interface::as_raw(self), pprofile.into_param().abi()).ok()
    }
    pub unsafe fn SetOutputFilename<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetOutputFilename)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetInputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetInputCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInputProps(&self, dwinputnum: u32) -> ::windows::core::Result<IWMInputMediaProps> {
        let mut result__ = ::windows::core::zeroed::<IWMInputMediaProps>();
        (::windows::core::Interface::vtable(self).GetInputProps)(::windows::core::Interface::as_raw(self), dwinputnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInputProps<P0>(&self, dwinputnum: u32, pinput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMInputMediaProps>,
    {
        (::windows::core::Interface::vtable(self).SetInputProps)(::windows::core::Interface::as_raw(self), dwinputnum, pinput.into_param().abi()).ok()
    }
    pub unsafe fn GetInputFormatCount(&self, dwinputnumber: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetInputFormatCount)(::windows::core::Interface::as_raw(self), dwinputnumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInputFormat(&self, dwinputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMInputMediaProps> {
        let mut result__ = ::windows::core::zeroed::<IWMInputMediaProps>();
        (::windows::core::Interface::vtable(self).GetInputFormat)(::windows::core::Interface::as_raw(self), dwinputnumber, dwformatnumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginWriting)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndWriting)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AllocateSample(&self, dwsamplesize: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::windows::core::zeroed::<INSSBuffer>();
        (::windows::core::Interface::vtable(self).AllocateSample)(::windows::core::Interface::as_raw(self), dwsamplesize, &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteSample<P0>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).WriteSample)(::windows::core::Interface::as_raw(self), dwinputnum, cnssampletime, dwflags, psample.into_param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWriter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriter {}
impl ::core::fmt::Debug for IWMWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriter {
    type Vtable = IWMWriter_Vtbl;
}
impl ::core::clone::Clone for IWMWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd4_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetProfileByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOutputFilename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetInputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, ppinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetInputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pinput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInputFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginWriting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndWriting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsamplesize: u32, ppsample: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterAdvanced(::windows::core::IUnknown);
impl IWMWriterAdvanced {
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSinkCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__ = ::windows::core::zeroed::<IWMWriterSink>();
        (::windows::core::Interface::vtable(self).GetSink)(::windows::core::Interface::as_raw(self), dwsinknum, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMWriterSink>,
    {
        (::windows::core::Interface::vtable(self).AddSink)(::windows::core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn RemoveSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMWriterSink>,
    {
        (::windows::core::Interface::vtable(self).RemoveSink)(::windows::core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<P0>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).WriteStreamSample)(::windows::core::Interface::as_raw(self), wstreamnum, cnssampletime, mssamplesendtime, cnssampleduration, dwflags, psample.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<P0>(&self, fislivesource: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetLiveSource)(::windows::core::Interface::as_raw(self), fislivesource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsRealTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetWriterTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatistics)(::windows::core::Interface::as_raw(self), wstreamnum, pstats).ok()
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSyncTolerance)(::windows::core::Interface::as_raw(self), mswindow).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSyncTolerance)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMWriterAdvanced, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMWriterAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced {}
impl ::core::fmt::Debug for IWMWriterAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterAdvanced {
    type Vtable = IWMWriterAdvanced_Vtbl;
}
impl ::core::clone::Clone for IWMWriterAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterAdvanced {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be3_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSinkCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows::core::HRESULT,
    pub GetSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteStreamSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLiveSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLiveSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRealTime: usize,
    pub GetWriterTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows::core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::HRESULT,
    pub SetSyncTolerance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows::core::HRESULT,
    pub GetSyncTolerance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterAdvanced2(::windows::core::IUnknown);
impl IWMWriterAdvanced2 {
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSinkCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__ = ::windows::core::zeroed::<IWMWriterSink>();
        (::windows::core::Interface::vtable(self).base__.GetSink)(::windows::core::Interface::as_raw(self), dwsinknum, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMWriterSink>,
    {
        (::windows::core::Interface::vtable(self).base__.AddSink)(::windows::core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn RemoveSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMWriterSink>,
    {
        (::windows::core::Interface::vtable(self).base__.RemoveSink)(::windows::core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<P0>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.WriteStreamSample)(::windows::core::Interface::as_raw(self), wstreamnum, cnssampletime, mssamplesendtime, cnssampleduration, dwflags, psample.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<P0>(&self, fislivesource: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetLiveSource)(::windows::core::Interface::as_raw(self), fislivesource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsRealTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).base__.GetWriterTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStatistics)(::windows::core::Interface::as_raw(self), wstreamnum, pstats).ok()
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSyncTolerance)(::windows::core::Interface::as_raw(self), mswindow).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSyncTolerance)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInputSetting<P0>(&self, dwinputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetInputSetting)(::windows::core::Interface::as_raw(self), dwinputnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetInputSetting<P0>(&self, dwinputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetInputSetting)(::windows::core::Interface::as_raw(self), dwinputnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWriterAdvanced2, ::windows::core::IUnknown, IWMWriterAdvanced);
impl ::core::cmp::PartialEq for IWMWriterAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced2 {}
impl ::core::fmt::Debug for IWMWriterAdvanced2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterAdvanced2 {
    type Vtable = IWMWriterAdvanced2_Vtbl;
}
impl ::core::clone::Clone for IWMWriterAdvanced2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterAdvanced2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x962dc1ec_c046_4db8_9cc7_26ceae500817);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced2_Vtbl {
    pub base__: IWMWriterAdvanced_Vtbl,
    pub GetInputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub SetInputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterAdvanced3(::windows::core::IUnknown);
impl IWMWriterAdvanced3 {
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSinkCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__ = ::windows::core::zeroed::<IWMWriterSink>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSink)(::windows::core::Interface::as_raw(self), dwsinknum, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMWriterSink>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddSink)(::windows::core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn RemoveSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMWriterSink>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveSink)(::windows::core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<P0>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.WriteStreamSample)(::windows::core::Interface::as_raw(self), wstreamnum, cnssampletime, mssamplesendtime, cnssampleduration, dwflags, psample.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<P0>(&self, fislivesource: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetLiveSource)(::windows::core::Interface::as_raw(self), fislivesource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.IsRealTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).base__.base__.GetWriterTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetStatistics)(::windows::core::Interface::as_raw(self), wstreamnum, pstats).ok()
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetSyncTolerance)(::windows::core::Interface::as_raw(self), mswindow).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSyncTolerance)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInputSetting<P0>(&self, dwinputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetInputSetting)(::windows::core::Interface::as_raw(self), dwinputnum, pszname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetInputSetting<P0>(&self, dwinputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInputSetting)(::windows::core::Interface::as_raw(self), dwinputnum, pszname.into_param().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetStatisticsEx(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatisticsEx)(::windows::core::Interface::as_raw(self), wstreamnum, pstats).ok()
    }
    pub unsafe fn SetNonBlocking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNonBlocking)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWriterAdvanced3, ::windows::core::IUnknown, IWMWriterAdvanced, IWMWriterAdvanced2);
impl ::core::cmp::PartialEq for IWMWriterAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced3 {}
impl ::core::fmt::Debug for IWMWriterAdvanced3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterAdvanced3 {
    type Vtable = IWMWriterAdvanced3_Vtbl;
}
impl ::core::clone::Clone for IWMWriterAdvanced3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterAdvanced3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd6492d_7c37_4e76_9d3b_59261183a22e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced3_Vtbl {
    pub base__: IWMWriterAdvanced2_Vtbl,
    pub GetStatisticsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::core::HRESULT,
    pub SetNonBlocking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterFileSink(::windows::core::IUnknown);
impl IWMWriterFileSink {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.OnHeader)(::windows::core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsRealTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::windows::core::zeroed::<INSSBuffer>();
        (::windows::core::Interface::vtable(self).base__.AllocateDataUnit)(::windows::core::Interface::as_raw(self), cbdataunit, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.OnDataUnit)(::windows::core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnEndWriting)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWriterFileSink, ::windows::core::IUnknown, IWMWriterSink);
impl ::core::cmp::PartialEq for IWMWriterFileSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink {}
impl ::core::fmt::Debug for IWMWriterFileSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterFileSink {
    type Vtable = IWMWriterFileSink_Vtbl;
}
impl ::core::clone::Clone for IWMWriterFileSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterFileSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be5_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterFileSink2(::windows::core::IUnknown);
impl IWMWriterFileSink2 {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.OnHeader)(::windows::core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.IsRealTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::windows::core::zeroed::<INSSBuffer>();
        (::windows::core::Interface::vtable(self).base__.base__.AllocateDataUnit)(::windows::core::Interface::as_raw(self), cbdataunit, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.OnDataUnit)(::windows::core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.OnEndWriting)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Open)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self), cnsstarttime).ok()
    }
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self), cnsstoptime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStopped(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsStopped)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFileDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetFileDuration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetFileSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsClosed)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMWriterFileSink2, ::windows::core::IUnknown, IWMWriterSink, IWMWriterFileSink);
impl ::core::cmp::PartialEq for IWMWriterFileSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink2 {}
impl ::core::fmt::Debug for IWMWriterFileSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterFileSink2 {
    type Vtable = IWMWriterFileSink2_Vtbl;
}
impl ::core::clone::Clone for IWMWriterFileSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterFileSink2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14282ba7_4aef_4205_8ce5_c229035a05bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink2_Vtbl {
    pub base__: IWMWriterFileSink_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStopped: usize,
    pub GetFileDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClosed: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterFileSink3(::windows::core::IUnknown);
impl IWMWriterFileSink3 {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.OnHeader)(::windows::core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.IsRealTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::windows::core::zeroed::<INSSBuffer>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.AllocateDataUnit)(::windows::core::Interface::as_raw(self), cbdataunit, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.OnDataUnit)(::windows::core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.OnEndWriting)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Open)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Start)(::windows::core::Interface::as_raw(self), cnsstarttime).ok()
    }
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stop)(::windows::core::Interface::as_raw(self), cnsstoptime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStopped(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsStopped)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFileDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).base__.GetFileDuration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).base__.GetFileSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsClosed)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoIndexing<P0>(&self, fdoautoindexing: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAutoIndexing)(::windows::core::Interface::as_raw(self), fdoautoindexing.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAutoIndexing(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetAutoIndexing)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetControlStream<P0>(&self, wstreamnumber: u16, fshouldcontrolstartandstop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetControlStream)(::windows::core::Interface::as_raw(self), wstreamnumber, fshouldcontrolstartandstop.into_param().abi()).ok()
    }
    pub unsafe fn GetMode(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OnDataUnitEx(&self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDataUnitEx)(::windows::core::Interface::as_raw(self), pfilesinkdataunit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnbufferedIO<P0, P1>(&self, funbufferedio: P0, frestrictmemusage: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetUnbufferedIO)(::windows::core::Interface::as_raw(self), funbufferedio.into_param().abi(), frestrictmemusage.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnbufferedIO(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetUnbufferedIO)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CompleteOperations(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompleteOperations)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWriterFileSink3, ::windows::core::IUnknown, IWMWriterSink, IWMWriterFileSink, IWMWriterFileSink2);
impl ::core::cmp::PartialEq for IWMWriterFileSink3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink3 {}
impl ::core::fmt::Debug for IWMWriterFileSink3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterFileSink3 {
    type Vtable = IWMWriterFileSink3_Vtbl;
}
impl ::core::clone::Clone for IWMWriterFileSink3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterFileSink3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fea4feb_2945_47a7_a1dd_c53a8fc4c45c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink3_Vtbl {
    pub base__: IWMWriterFileSink2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoIndexing: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAutoIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAutoIndexing: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetControlStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetControlStream: usize,
    pub GetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfilesinkmode: *mut u32) -> ::windows::core::HRESULT,
    pub OnDataUnitEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUnbufferedIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUnbufferedIO: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUnbufferedIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUnbufferedIO: usize,
    pub CompleteOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterNetworkSink(::windows::core::IUnknown);
impl IWMWriterNetworkSink {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.OnHeader)(::windows::core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsRealTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::windows::core::zeroed::<INSSBuffer>();
        (::windows::core::Interface::vtable(self).base__.AllocateDataUnit)(::windows::core::Interface::as_raw(self), cbdataunit, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.OnDataUnit)(::windows::core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnEndWriting)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetMaximumClients(&self, dwmaxclients: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumClients)(::windows::core::Interface::as_raw(self), dwmaxclients).ok()
    }
    pub unsafe fn GetMaximumClients(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaximumClients)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNetworkProtocol(&self, protocol: WMT_NET_PROTOCOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNetworkProtocol)(::windows::core::Interface::as_raw(self), protocol).ok()
    }
    pub unsafe fn GetNetworkProtocol(&self) -> ::windows::core::Result<WMT_NET_PROTOCOL> {
        let mut result__ = ::windows::core::zeroed::<WMT_NET_PROTOCOL>();
        (::windows::core::Interface::vtable(self).GetNetworkProtocol)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetHostURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHostURL)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszurl), pcchurl).ok()
    }
    pub unsafe fn Open(&self, pdwportnum: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), pdwportnum).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWriterNetworkSink, ::windows::core::IUnknown, IWMWriterSink);
impl ::core::cmp::PartialEq for IWMWriterNetworkSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterNetworkSink {}
impl ::core::fmt::Debug for IWMWriterNetworkSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterNetworkSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterNetworkSink {
    type Vtable = IWMWriterNetworkSink_Vtbl;
}
impl ::core::clone::Clone for IWMWriterNetworkSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterNetworkSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be7_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterNetworkSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    pub SetMaximumClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxclients: u32) -> ::windows::core::HRESULT,
    pub GetMaximumClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxclients: *mut u32) -> ::windows::core::HRESULT,
    pub SetNetworkProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> ::windows::core::HRESULT,
    pub GetNetworkProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows::core::HRESULT,
    pub GetHostURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwportnum: *mut u32) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterPostView(::windows::core::IUnknown);
impl IWMWriterPostView {
    pub unsafe fn SetPostViewCallback<P0>(&self, pcallback: P0, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMWriterPostViewCallback>,
    {
        (::windows::core::Interface::vtable(self).SetPostViewCallback)(::windows::core::Interface::as_raw(self), pcallback.into_param().abi(), pvcontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceivePostViewSamples<P0>(&self, wstreamnum: u16, freceivepostviewsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetReceivePostViewSamples)(::windows::core::Interface::as_raw(self), wstreamnum, freceivepostviewsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceivePostViewSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetReceivePostViewSamples)(::windows::core::Interface::as_raw(self), wstreamnum, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPostViewProps(&self, wstreamnumber: u16) -> ::windows::core::Result<IWMMediaProps> {
        let mut result__ = ::windows::core::zeroed::<IWMMediaProps>();
        (::windows::core::Interface::vtable(self).GetPostViewProps)(::windows::core::Interface::as_raw(self), wstreamnumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPostViewProps<P0>(&self, wstreamnumber: u16, poutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMMediaProps>,
    {
        (::windows::core::Interface::vtable(self).SetPostViewProps)(::windows::core::Interface::as_raw(self), wstreamnumber, poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetPostViewFormatCount(&self, wstreamnumber: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPostViewFormatCount)(::windows::core::Interface::as_raw(self), wstreamnumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPostViewFormat(&self, wstreamnumber: u16, dwformatnumber: u32) -> ::windows::core::Result<IWMMediaProps> {
        let mut result__ = ::windows::core::zeroed::<IWMMediaProps>();
        (::windows::core::Interface::vtable(self).GetPostViewFormat)(::windows::core::Interface::as_raw(self), wstreamnumber, dwformatnumber, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForPostView<P0>(&self, wstreamnumber: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAllocateForPostView)(::windows::core::Interface::as_raw(self), wstreamnumber, fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForPostView(&self, wstreamnumber: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetAllocateForPostView)(::windows::core::Interface::as_raw(self), wstreamnumber, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMWriterPostView, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMWriterPostView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPostView {}
impl ::core::fmt::Debug for IWMWriterPostView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPostView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterPostView {
    type Vtable = IWMWriterPostView_Vtbl;
}
impl ::core::clone::Clone for IWMWriterPostView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterPostView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81e20ce4_75ef_491a_8004_fc53c45bdc3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostView_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetPostViewCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceivePostViewSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceivePostViewSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReceivePostViewSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReceivePostViewSamples: usize,
    pub GetPostViewProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPostViewProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPostViewFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetPostViewFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateForPostView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateForPostView: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateForPostView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateForPostView: usize,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterPostViewCallback(::windows::core::IUnknown);
impl IWMWriterPostViewCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnStatus)(::windows::core::Interface::as_raw(self), status, hr, dwtype, pvalue, pvcontext).ok()
    }
    pub unsafe fn OnPostViewSample<P0>(&self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: P0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).OnPostViewSample)(::windows::core::Interface::as_raw(self), wstreamnumber, cnssampletime, cnssampleduration, dwflags, psample.into_param().abi(), pvcontext).ok()
    }
    pub unsafe fn AllocateForPostView(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateForPostView)(::windows::core::Interface::as_raw(self), wstreamnum, cbbuffer, ::core::mem::transmute(ppbuffer), pvcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWriterPostViewCallback, ::windows::core::IUnknown, IWMStatusCallback);
impl ::core::cmp::PartialEq for IWMWriterPostViewCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPostViewCallback {}
impl ::core::fmt::Debug for IWMWriterPostViewCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPostViewCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterPostViewCallback {
    type Vtable = IWMWriterPostViewCallback_Vtbl;
}
impl ::core::clone::Clone for IWMWriterPostViewCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterPostViewCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9d6549d_a193_4f24_b308_03123d9b7f8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostViewCallback_Vtbl {
    pub base__: IWMStatusCallback_Vtbl,
    pub OnPostViewSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateForPostView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterPreprocess(::windows::core::IUnknown);
impl IWMWriterPreprocess {
    pub unsafe fn GetMaxPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxPreprocessingPasses)(::windows::core::Interface::as_raw(self), dwinputnum, dwflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNumPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNumPreprocessingPasses)(::windows::core::Interface::as_raw(self), dwinputnum, dwflags, dwnumpasses).ok()
    }
    pub unsafe fn BeginPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginPreprocessingPass)(::windows::core::Interface::as_raw(self), dwinputnum, dwflags).ok()
    }
    pub unsafe fn PreprocessSample<P0>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).PreprocessSample)(::windows::core::Interface::as_raw(self), dwinputnum, cnssampletime, dwflags, psample.into_param().abi()).ok()
    }
    pub unsafe fn EndPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndPreprocessingPass)(::windows::core::Interface::as_raw(self), dwinputnum, dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWriterPreprocess, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMWriterPreprocess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPreprocess {}
impl ::core::fmt::Debug for IWMWriterPreprocess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPreprocess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterPreprocess {
    type Vtable = IWMWriterPreprocess_Vtbl;
}
impl ::core::clone::Clone for IWMWriterPreprocess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterPreprocess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc54a285_38c4_45b5_aa23_85b9f7cb424b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPreprocess_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetMaxPreprocessingPasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows::core::HRESULT,
    pub SetNumPreprocessingPasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::HRESULT,
    pub BeginPreprocessingPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub PreprocessSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndPreprocessingPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterPushSink(::windows::core::IUnknown);
impl IWMWriterPushSink {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.OnHeader)(::windows::core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsRealTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::windows::core::zeroed::<INSSBuffer>();
        (::windows::core::Interface::vtable(self).base__.AllocateDataUnit)(::windows::core::Interface::as_raw(self), cbdataunit, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).base__.OnDataUnit)(::windows::core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnEndWriting)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<P0, P1, P2>(&self, pwszurl: P0, pwsztemplateurl: P1, fautodestroy: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), pwsztemplateurl.into_param().abi(), fautodestroy.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndSession(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndSession)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWriterPushSink, ::windows::core::IUnknown, IWMWriterSink);
impl ::core::cmp::PartialEq for IWMWriterPushSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPushSink {}
impl ::core::fmt::Debug for IWMWriterPushSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPushSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterPushSink {
    type Vtable = IWMWriterPushSink_Vtbl;
}
impl ::core::clone::Clone for IWMWriterPushSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterPushSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc10e6a5_072c_467d_bf57_6330a9dde12a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPushSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pwsztemplateurl: ::windows::core::PCWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
pub struct IWMWriterSink(::windows::core::IUnknown);
impl IWMWriterSink {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).OnHeader)(::windows::core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsRealTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::windows::core::zeroed::<INSSBuffer>();
        (::windows::core::Interface::vtable(self).AllocateDataUnit)(::windows::core::Interface::as_raw(self), cbdataunit, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INSSBuffer>,
    {
        (::windows::core::Interface::vtable(self).OnDataUnit)(::windows::core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEndWriting)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMWriterSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMWriterSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterSink {}
impl ::core::fmt::Debug for IWMWriterSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterSink {
    type Vtable = IWMWriterSink_Vtbl;
}
impl ::core::clone::Clone for IWMWriterSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMWriterSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be4_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRealTime: usize,
    pub AllocateDataUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnDataUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataunit: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEndWriting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const CLSID_ClientNetManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd12a3ce_9c42_11d2_beed_0060082f2054);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const CLSID_WMBandwidthSharing_Exclusive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf6060aa_5197_11d2_b6af_00c04fd908e9);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const CLSID_WMBandwidthSharing_Partial: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf6060ab_5197_11d2_b6af_00c04fd908e9);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const CLSID_WMMUTEX_Bitrate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a01_35da_11d1_9034_00a0c90349be);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const CLSID_WMMUTEX_Language: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a00_35da_11d1_9034_00a0c90349be);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const CLSID_WMMUTEX_Presentation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a02_35da_11d1_9034_00a0c90349be);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const CLSID_WMMUTEX_Unknown: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a03_35da_11d1_9034_00a0c90349be);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const DRM_OPL_TYPES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMFORMAT_MPEG2Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMFORMAT_Script: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c8510f2_debe_4ca7_bba5_f07a104f8dff);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMFORMAT_VideoInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMFORMAT_WaveFormatEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMFORMAT_WebStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda1e6b13_8359_4050_b398_388e965bf00c);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_ACELPnet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000130_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_Base: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_DRM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000009_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_I420: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30323449_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_IYUV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56555949_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_M4S2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3253344d_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_MP3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_MP43: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3334504d_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_MP4S: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5334504d_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_MPEG2_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_MSS1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3153534d_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_MSS2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3253534d_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_P422: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32323450_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_PCM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_RGB1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb78_524f_11ce_9f53_0020af0ba770);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_RGB24: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7d_524f_11ce_9f53_0020af0ba770);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_RGB32: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7e_524f_11ce_9f53_0020af0ba770);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_RGB4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb79_524f_11ce_9f53_0020af0ba770);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_RGB555: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7c_524f_11ce_9f53_0020af0ba770);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_RGB565: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7b_524f_11ce_9f53_0020af0ba770);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_RGB8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7a_524f_11ce_9f53_0020af0ba770);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_UYVY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59565955_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_VIDEOIMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d4a45f2_e5f6_4b44_8388_f0ae5c0e0c37);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMAudioV2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMAudioV7: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMAudioV8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMAudioV9: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMAudio_Lossless: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMSP1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000a_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMSP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000b_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMV1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31564d57_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMV2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32564d57_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMV3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33564d57_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMVA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41564d57_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WMVP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50564d57_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WVC1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31435657_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WVP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32505657_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_WebStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x776257d4_c627_41cb_8f81_7ac7ff1c40cc);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_YUY2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32595559_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_YV12: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32315659_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_YVU9: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39555659_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIASUBTYPE_YVYU: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55595659_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIATYPE_Audio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIATYPE_FileTransfer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9e47579_930e_4427_adfc_ad80f290e470);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIATYPE_Image: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a50fd8_8aa5_4386_81fe_a0efe0488e31);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIATYPE_Script: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73636d64_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIATYPE_Text: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bba1ea7_5ab2_4829_ba57_0940209bcf3e);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMMEDIATYPE_Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMSCRIPTTYPE_TwoStrings: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82f38a70_c29f_11d1_97ad_00a0c95ea850);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_DMOCATEGORY_AUDIO_WATERMARK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65221c5a_fa75_4b39_b50c_06c336b6a3ef);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_DMOCATEGORY_VIDEO_WATERMARK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x187cc922_8efc_4404_9daf_63f4830df1bc);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_INTEGER_DENOMINATOR: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER: u32 = 491406834u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER_2: u32 = 491406835u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_ADV_BLENDING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_BLENDING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_INPUT_FRAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_MOTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_OUTPUT_FRAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_ROTATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_USES_CURRENT_INPUT_FRAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_USES_PREVIOUS_INPUT_FRAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_BOW_TIE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_CIRCLE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_CROSS_FADE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_DIAGONAL: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_DIAMOND: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FADE_TO_COLOR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FILLED_V: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FLIP: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_INSET: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_IRIS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_PAGE_ROLL: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_RECTANGLE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_REVEAL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_SLIDE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_SPLIT: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_STAR: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_WHEEL: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CL_INTERLACED420: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CL_PROGRESSIVE420: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CT_BOTTOM_FIELD_FIRST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CT_INTERLACED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CT_REPEAT_FIRST_FIELD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CT_TOP_FIELD_FIRST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_MAX_STREAMS: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_MAX_VIDEO_STREAMS: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtensionGUID_ChromaLocation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c5acca0_9276_4b2c_9e4c_a0edefdd217e);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtensionGUID_ColorSpaceInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf79ada56_30eb_4f2b_9f7a_f24b139a1157);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtensionGUID_ContentType: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd590dc20_07bc_436c_9cf7_f3bbfbf1a4dc);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtensionGUID_FileName: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe165ec0e_19ed_45d7_b4a7_25cbd1e28e9b);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtensionGUID_OutputCleanPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf72a3c6f_6eb4_4ebc_b192_09ad9759e828);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtensionGUID_PixelAspectRatio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b1ee554_f9ea_4bc8_821a_376b74e4c4b8);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtensionGUID_SampleDuration: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6bd9450_867f_4907_83a3_c77921b733ad);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtensionGUID_SampleProtectionSalt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5403deee_b9ee_438f_aa83_3804997e569d);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtensionGUID_Timecode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x399595ec_8667_4e2d_8fdb_98814ce76c1e);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtensionGUID_UserDataInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x732bb4fa_78be_4549_99bd_02db1a55b7a8);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_ChromaLocation_Size: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_ColorSpaceInfo_Size: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_ContentType_Size: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_PixelAspectRatio_Size: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_SampleDuration_Size: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_Timecode_Size: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_dwWMContentAttributes: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_dwWMNSCAttributes: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_dwWMSpecialAttributes: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszASFLeakyBucketPairs: ::windows::core::PCWSTR = ::windows::w!("ASFLeakyBucketPairs");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszAllowInterlacedOutput: ::windows::core::PCWSTR = ::windows::w!("AllowInterlacedOutput");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszAverageLevel: ::windows::core::PCWSTR = ::windows::w!("AverageLevel");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszBufferAverage: ::windows::core::PCWSTR = ::windows::w!("Buffer Average");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszComplexity: ::windows::core::PCWSTR = ::windows::w!("_COMPLEXITYEX");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszComplexityLive: ::windows::core::PCWSTR = ::windows::w!("_COMPLEXITYEXLIVE");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszComplexityMax: ::windows::core::PCWSTR = ::windows::w!("_COMPLEXITYEXMAX");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszComplexityOffline: ::windows::core::PCWSTR = ::windows::w!("_COMPLEXITYEXOFFLINE");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDecoderComplexityRequested: ::windows::core::PCWSTR = ::windows::w!("_DECODERCOMPLEXITYPROFILE");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDedicatedDeliveryThread: ::windows::core::PCWSTR = ::windows::w!("DedicatedDeliveryThread");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDeinterlaceMode: ::windows::core::PCWSTR = ::windows::w!("DeinterlaceMode");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDeliverOnReceive: ::windows::core::PCWSTR = ::windows::w!("DeliverOnReceive");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDeviceConformanceTemplate: ::windows::core::PCWSTR = ::windows::w!("DeviceConformanceTemplate");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDynamicRangeControl: ::windows::core::PCWSTR = ::windows::w!("DynamicRangeControl");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszEDL: ::windows::core::PCWSTR = ::windows::w!("_EDL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszEarlyDataDelivery: ::windows::core::PCWSTR = ::windows::w!("EarlyDataDelivery");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszEnableDiscreteOutput: ::windows::core::PCWSTR = ::windows::w!("EnableDiscreteOutput");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszEnableFrameInterpolation: ::windows::core::PCWSTR = ::windows::w!("EnableFrameInterpolation");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszEnableWMAProSPDIFOutput: ::windows::core::PCWSTR = ::windows::w!("EnableWMAProSPDIFOutput");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszFailSeekOnError: ::windows::core::PCWSTR = ::windows::w!("FailSeekOnError");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszFixedFrameRate: ::windows::core::PCWSTR = ::windows::w!("FixedFrameRate");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszFold6To2Channels3: ::windows::core::PCWSTR = ::windows::w!("Fold6To2Channels3");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszFoldToChannelsTemplate: ::windows::core::PCWSTR = ::windows::w!("Fold%luTo%luChannels%lu");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszInitialPatternForInverseTelecine: ::windows::core::PCWSTR = ::windows::w!("InitialPatternForInverseTelecine");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszInterlacedCoding: ::windows::core::PCWSTR = ::windows::w!("InterlacedCoding");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszIsVBRSupported: ::windows::core::PCWSTR = ::windows::w!("_ISVBRSUPPORTED");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszJPEGCompressionQuality: ::windows::core::PCWSTR = ::windows::w!("JPEGCompressionQuality");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszJustInTimeDecode: ::windows::core::PCWSTR = ::windows::w!("JustInTimeDecode");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszMixedClassMode: ::windows::core::PCWSTR = ::windows::w!("MixedClassMode");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszMusicClassMode: ::windows::core::PCWSTR = ::windows::w!("MusicClassMode");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszMusicSpeechClassMode: ::windows::core::PCWSTR = ::windows::w!("MusicSpeechClassMode");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszNeedsPreviousSample: ::windows::core::PCWSTR = ::windows::w!("NeedsPreviousSample");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszNumPasses: ::windows::core::PCWSTR = ::windows::w!("_PASSESUSED");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszOriginalSourceFormatTag: ::windows::core::PCWSTR = ::windows::w!("_SOURCEFORMATTAG");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszOriginalWaveFormat: ::windows::core::PCWSTR = ::windows::w!("_ORIGINALWAVEFORMAT");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszPeakValue: ::windows::core::PCWSTR = ::windows::w!("PeakValue");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszPermitSeeksBeyondEndOfStream: ::windows::core::PCWSTR = ::windows::w!("PermitSeeksBeyondEndOfStream");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszReloadIndexOnSeek: ::windows::core::PCWSTR = ::windows::w!("ReloadIndexOnSeek");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszScrambledAudio: ::windows::core::PCWSTR = ::windows::w!("ScrambledAudio");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSingleOutputBuffer: ::windows::core::PCWSTR = ::windows::w!("SingleOutputBuffer");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSoftwareScaling: ::windows::core::PCWSTR = ::windows::w!("SoftwareScaling");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSourceBufferTime: ::windows::core::PCWSTR = ::windows::w!("SourceBufferTime");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSourceMaxBytesAtOnce: ::windows::core::PCWSTR = ::windows::w!("SourceMaxBytesAtOnce");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSpeakerConfig: ::windows::core::PCWSTR = ::windows::w!("SpeakerConfig");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSpeechCaps: ::windows::core::PCWSTR = ::windows::w!("SpeechFormatCap");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSpeechClassMode: ::windows::core::PCWSTR = ::windows::w!("SpeechClassMode");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszStreamLanguage: ::windows::core::PCWSTR = ::windows::w!("StreamLanguage");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszStreamNumIndexObjects: ::windows::core::PCWSTR = ::windows::w!("StreamNumIndexObjects");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszUsePacketAtSeekPoint: ::windows::core::PCWSTR = ::windows::w!("UsePacketAtSeekPoint");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVBRBitrateMax: ::windows::core::PCWSTR = ::windows::w!("_RMAX");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVBRBufferWindowMax: ::windows::core::PCWSTR = ::windows::w!("_BMAX");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVBREnabled: ::windows::core::PCWSTR = ::windows::w!("_VBRENABLED");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVBRPeak: ::windows::core::PCWSTR = ::windows::w!("VBR Peak");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVBRQuality: ::windows::core::PCWSTR = ::windows::w!("_VBRQUALITY");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVideoSampleDurations: ::windows::core::PCWSTR = ::windows::w!("VideoSampleDurations");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMADID: ::windows::core::PCWSTR = ::windows::w!("WM/ADID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMASFPacketCount: ::windows::core::PCWSTR = ::windows::w!("WM/ASFPacketCount");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMASFSecurityObjectsSize: ::windows::core::PCWSTR = ::windows::w!("WM/ASFSecurityObjectsSize");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAlbumArtist: ::windows::core::PCWSTR = ::windows::w!("WM/AlbumArtist");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAlbumArtistSort: ::windows::core::PCWSTR = ::windows::w!("WM/AlbumArtistSort");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAlbumCoverURL: ::windows::core::PCWSTR = ::windows::w!("WM/AlbumCoverURL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAlbumTitle: ::windows::core::PCWSTR = ::windows::w!("WM/AlbumTitle");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAlbumTitleSort: ::windows::core::PCWSTR = ::windows::w!("WM/AlbumTitleSort");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAspectRatioX: ::windows::core::PCWSTR = ::windows::w!("AspectRatioX");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAspectRatioY: ::windows::core::PCWSTR = ::windows::w!("AspectRatioY");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAudioFileURL: ::windows::core::PCWSTR = ::windows::w!("WM/AudioFileURL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAudioSourceURL: ::windows::core::PCWSTR = ::windows::w!("WM/AudioSourceURL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAuthor: ::windows::core::PCWSTR = ::windows::w!("Author");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAuthorSort: ::windows::core::PCWSTR = ::windows::w!("AuthorSort");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAuthorURL: ::windows::core::PCWSTR = ::windows::w!("WM/AuthorURL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBannerImageData: ::windows::core::PCWSTR = ::windows::w!("BannerImageData");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBannerImageType: ::windows::core::PCWSTR = ::windows::w!("BannerImageType");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBannerImageURL: ::windows::core::PCWSTR = ::windows::w!("BannerImageURL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBeatsPerMinute: ::windows::core::PCWSTR = ::windows::w!("WM/BeatsPerMinute");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBitrate: ::windows::core::PCWSTR = ::windows::w!("Bitrate");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBroadcast: ::windows::core::PCWSTR = ::windows::w!("Broadcast");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMCategory: ::windows::core::PCWSTR = ::windows::w!("WM/Category");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMCodec: ::windows::core::PCWSTR = ::windows::w!("WM/Codec");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMComposer: ::windows::core::PCWSTR = ::windows::w!("WM/Composer");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMComposerSort: ::windows::core::PCWSTR = ::windows::w!("WM/ComposerSort");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMConductor: ::windows::core::PCWSTR = ::windows::w!("WM/Conductor");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMContainerFormat: ::windows::core::PCWSTR = ::windows::w!("WM/ContainerFormat");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMContentDistributor: ::windows::core::PCWSTR = ::windows::w!("WM/ContentDistributor");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMContentGroupDescription: ::windows::core::PCWSTR = ::windows::w!("WM/ContentGroupDescription");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMCopyright: ::windows::core::PCWSTR = ::windows::w!("Copyright");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMCopyrightURL: ::windows::core::PCWSTR = ::windows::w!("CopyrightURL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMCurrentBitrate: ::windows::core::PCWSTR = ::windows::w!("CurrentBitrate");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM: ::windows::core::PCWSTR = ::windows::w!("WM/DRM");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_ContentID: ::windows::core::PCWSTR = ::windows::w!("DRM_ContentID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_Flags: ::windows::core::PCWSTR = ::windows::w!("DRM_Flags");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_HeaderSignPrivKey: ::windows::core::PCWSTR = ::windows::w!("DRM_HeaderSignPrivKey");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_IndividualizedVersion: ::windows::core::PCWSTR = ::windows::w!("DRM_IndividualizedVersion");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_KeyID: ::windows::core::PCWSTR = ::windows::w!("DRM_KeyID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_KeySeed: ::windows::core::PCWSTR = ::windows::w!("DRM_KeySeed");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_LASignatureCert: ::windows::core::PCWSTR = ::windows::w!("DRM_LASignatureCert");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_LASignatureLicSrvCert: ::windows::core::PCWSTR = ::windows::w!("DRM_LASignatureLicSrvCert");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_LASignaturePrivKey: ::windows::core::PCWSTR = ::windows::w!("DRM_LASignaturePrivKey");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_LASignatureRootCert: ::windows::core::PCWSTR = ::windows::w!("DRM_LASignatureRootCert");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_Level: ::windows::core::PCWSTR = ::windows::w!("DRM_Level");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_LicenseAcqURL: ::windows::core::PCWSTR = ::windows::w!("DRM_LicenseAcqURL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_SourceID: ::windows::core::PCWSTR = ::windows::w!("DRM_SourceID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_V1LicenseAcqURL: ::windows::core::PCWSTR = ::windows::w!("DRM_V1LicenseAcqURL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDVDID: ::windows::core::PCWSTR = ::windows::w!("WM/DVDID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDescription: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDirector: ::windows::core::PCWSTR = ::windows::w!("WM/Director");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDuration: ::windows::core::PCWSTR = ::windows::w!("Duration");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMEncodedBy: ::windows::core::PCWSTR = ::windows::w!("WM/EncodedBy");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMEncodingSettings: ::windows::core::PCWSTR = ::windows::w!("WM/EncodingSettings");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMEncodingTime: ::windows::core::PCWSTR = ::windows::w!("WM/EncodingTime");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMEpisodeNumber: ::windows::core::PCWSTR = ::windows::w!("WM/EpisodeNumber");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMFileSize: ::windows::core::PCWSTR = ::windows::w!("FileSize");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMGenre: ::windows::core::PCWSTR = ::windows::w!("WM/Genre");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMGenreID: ::windows::core::PCWSTR = ::windows::w!("WM/GenreID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasArbitraryDataStream: ::windows::core::PCWSTR = ::windows::w!("HasArbitraryDataStream");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasAttachedImages: ::windows::core::PCWSTR = ::windows::w!("HasAttachedImages");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasAudio: ::windows::core::PCWSTR = ::windows::w!("HasAudio");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasFileTransferStream: ::windows::core::PCWSTR = ::windows::w!("HasFileTransferStream");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasImage: ::windows::core::PCWSTR = ::windows::w!("HasImage");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasScript: ::windows::core::PCWSTR = ::windows::w!("HasScript");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasVideo: ::windows::core::PCWSTR = ::windows::w!("HasVideo");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMISAN: ::windows::core::PCWSTR = ::windows::w!("WM/ISAN");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMISRC: ::windows::core::PCWSTR = ::windows::w!("WM/ISRC");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMInitialKey: ::windows::core::PCWSTR = ::windows::w!("WM/InitialKey");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMIsCompilation: ::windows::core::PCWSTR = ::windows::w!("WM/IsCompilation");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMIsVBR: ::windows::core::PCWSTR = ::windows::w!("IsVBR");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMLanguage: ::windows::core::PCWSTR = ::windows::w!("WM/Language");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMLyrics: ::windows::core::PCWSTR = ::windows::w!("WM/Lyrics");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMLyrics_Synchronised: ::windows::core::PCWSTR = ::windows::w!("WM/Lyrics_Synchronised");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMCDI: ::windows::core::PCWSTR = ::windows::w!("WM/MCDI");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaClassPrimaryID: ::windows::core::PCWSTR = ::windows::w!("WM/MediaClassPrimaryID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaClassSecondaryID: ::windows::core::PCWSTR = ::windows::w!("WM/MediaClassSecondaryID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaCredits: ::windows::core::PCWSTR = ::windows::w!("WM/MediaCredits");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsDelay: ::windows::core::PCWSTR = ::windows::w!("WM/MediaIsDelay");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsFinale: ::windows::core::PCWSTR = ::windows::w!("WM/MediaIsFinale");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsLive: ::windows::core::PCWSTR = ::windows::w!("WM/MediaIsLive");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsPremiere: ::windows::core::PCWSTR = ::windows::w!("WM/MediaIsPremiere");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsRepeat: ::windows::core::PCWSTR = ::windows::w!("WM/MediaIsRepeat");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsSAP: ::windows::core::PCWSTR = ::windows::w!("WM/MediaIsSAP");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsStereo: ::windows::core::PCWSTR = ::windows::w!("WM/MediaIsStereo");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsSubtitled: ::windows::core::PCWSTR = ::windows::w!("WM/MediaIsSubtitled");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsTape: ::windows::core::PCWSTR = ::windows::w!("WM/MediaIsTape");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaNetworkAffiliation: ::windows::core::PCWSTR = ::windows::w!("WM/MediaNetworkAffiliation");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaOriginalBroadcastDateTime: ::windows::core::PCWSTR = ::windows::w!("WM/MediaOriginalBroadcastDateTime");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaOriginalChannel: ::windows::core::PCWSTR = ::windows::w!("WM/MediaOriginalChannel");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaStationCallSign: ::windows::core::PCWSTR = ::windows::w!("WM/MediaStationCallSign");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaStationName: ::windows::core::PCWSTR = ::windows::w!("WM/MediaStationName");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMModifiedBy: ::windows::core::PCWSTR = ::windows::w!("WM/ModifiedBy");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMood: ::windows::core::PCWSTR = ::windows::w!("WM/Mood");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNSCAddress: ::windows::core::PCWSTR = ::windows::w!("NSC_Address");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNSCDescription: ::windows::core::PCWSTR = ::windows::w!("NSC_Description");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNSCEmail: ::windows::core::PCWSTR = ::windows::w!("NSC_Email");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNSCName: ::windows::core::PCWSTR = ::windows::w!("NSC_Name");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNSCPhone: ::windows::core::PCWSTR = ::windows::w!("NSC_Phone");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNumberOfFrames: ::windows::core::PCWSTR = ::windows::w!("NumberOfFrames");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOptimalBitrate: ::windows::core::PCWSTR = ::windows::w!("OptimalBitrate");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalAlbumTitle: ::windows::core::PCWSTR = ::windows::w!("WM/OriginalAlbumTitle");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalArtist: ::windows::core::PCWSTR = ::windows::w!("WM/OriginalArtist");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalFilename: ::windows::core::PCWSTR = ::windows::w!("WM/OriginalFilename");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalLyricist: ::windows::core::PCWSTR = ::windows::w!("WM/OriginalLyricist");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalReleaseTime: ::windows::core::PCWSTR = ::windows::w!("WM/OriginalReleaseTime");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalReleaseYear: ::windows::core::PCWSTR = ::windows::w!("WM/OriginalReleaseYear");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMParentalRating: ::windows::core::PCWSTR = ::windows::w!("WM/ParentalRating");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMParentalRatingReason: ::windows::core::PCWSTR = ::windows::w!("WM/ParentalRatingReason");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPartOfSet: ::windows::core::PCWSTR = ::windows::w!("WM/PartOfSet");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPeakBitrate: ::windows::core::PCWSTR = ::windows::w!("WM/PeakBitrate");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPeriod: ::windows::core::PCWSTR = ::windows::w!("WM/Period");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPicture: ::windows::core::PCWSTR = ::windows::w!("WM/Picture");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPlaylistDelay: ::windows::core::PCWSTR = ::windows::w!("WM/PlaylistDelay");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProducer: ::windows::core::PCWSTR = ::windows::w!("WM/Producer");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPromotionURL: ::windows::core::PCWSTR = ::windows::w!("WM/PromotionURL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProtected: ::windows::core::PCWSTR = ::windows::w!("Is_Protected");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProtectionType: ::windows::core::PCWSTR = ::windows::w!("WM/ProtectionType");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProvider: ::windows::core::PCWSTR = ::windows::w!("WM/Provider");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProviderCopyright: ::windows::core::PCWSTR = ::windows::w!("WM/ProviderCopyright");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProviderRating: ::windows::core::PCWSTR = ::windows::w!("WM/ProviderRating");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProviderStyle: ::windows::core::PCWSTR = ::windows::w!("WM/ProviderStyle");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPublisher: ::windows::core::PCWSTR = ::windows::w!("WM/Publisher");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMRadioStationName: ::windows::core::PCWSTR = ::windows::w!("WM/RadioStationName");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMRadioStationOwner: ::windows::core::PCWSTR = ::windows::w!("WM/RadioStationOwner");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMRating: ::windows::core::PCWSTR = ::windows::w!("Rating");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSeasonNumber: ::windows::core::PCWSTR = ::windows::w!("WM/SeasonNumber");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSeekable: ::windows::core::PCWSTR = ::windows::w!("Seekable");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSharedUserRating: ::windows::core::PCWSTR = ::windows::w!("WM/SharedUserRating");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSignature_Name: ::windows::core::PCWSTR = ::windows::w!("Signature_Name");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSkipBackward: ::windows::core::PCWSTR = ::windows::w!("Can_Skip_Backward");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSkipForward: ::windows::core::PCWSTR = ::windows::w!("Can_Skip_Forward");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMStreamTypeInfo: ::windows::core::PCWSTR = ::windows::w!("WM/StreamTypeInfo");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMStridable: ::windows::core::PCWSTR = ::windows::w!("Stridable");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSubTitle: ::windows::core::PCWSTR = ::windows::w!("WM/SubTitle");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSubTitleDescription: ::windows::core::PCWSTR = ::windows::w!("WM/SubTitleDescription");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSubscriptionContentID: ::windows::core::PCWSTR = ::windows::w!("WM/SubscriptionContentID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMText: ::windows::core::PCWSTR = ::windows::w!("WM/Text");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMTitle: ::windows::core::PCWSTR = ::windows::w!("Title");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMTitleSort: ::windows::core::PCWSTR = ::windows::w!("TitleSort");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMToolName: ::windows::core::PCWSTR = ::windows::w!("WM/ToolName");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMToolVersion: ::windows::core::PCWSTR = ::windows::w!("WM/ToolVersion");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMTrack: ::windows::core::PCWSTR = ::windows::w!("WM/Track");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMTrackNumber: ::windows::core::PCWSTR = ::windows::w!("WM/TrackNumber");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMTrusted: ::windows::core::PCWSTR = ::windows::w!("Is_Trusted");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMUniqueFileIdentifier: ::windows::core::PCWSTR = ::windows::w!("WM/UniqueFileIdentifier");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMUse_Advanced_DRM: ::windows::core::PCWSTR = ::windows::w!("Use_Advanced_DRM");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMUse_DRM: ::windows::core::PCWSTR = ::windows::w!("Use_DRM");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMUserWebURL: ::windows::core::PCWSTR = ::windows::w!("WM/UserWebURL");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMVideoClosedCaptioning: ::windows::core::PCWSTR = ::windows::w!("WM/VideoClosedCaptioning");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMVideoFrameRate: ::windows::core::PCWSTR = ::windows::w!("WM/VideoFrameRate");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMVideoHeight: ::windows::core::PCWSTR = ::windows::w!("WM/VideoHeight");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMVideoWidth: ::windows::core::PCWSTR = ::windows::w!("WM/VideoWidth");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMADRCAverageReference: ::windows::core::PCWSTR = ::windows::w!("WM/WMADRCAverageReference");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMADRCAverageTarget: ::windows::core::PCWSTR = ::windows::w!("WM/WMADRCAverageTarget");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMADRCPeakReference: ::windows::core::PCWSTR = ::windows::w!("WM/WMADRCPeakReference");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMADRCPeakTarget: ::windows::core::PCWSTR = ::windows::w!("WM/WMADRCPeakTarget");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMCPDistributor: ::windows::core::PCWSTR = ::windows::w!("WM/WMCPDistributor");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMCPDistributorID: ::windows::core::PCWSTR = ::windows::w!("WM/WMCPDistributorID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMCollectionGroupID: ::windows::core::PCWSTR = ::windows::w!("WM/WMCollectionGroupID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMCollectionID: ::windows::core::PCWSTR = ::windows::w!("WM/WMCollectionID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMContentID: ::windows::core::PCWSTR = ::windows::w!("WM/WMContentID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMShadowFileSourceDRMType: ::windows::core::PCWSTR = ::windows::w!("WM/WMShadowFileSourceDRMType");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMShadowFileSourceFileType: ::windows::core::PCWSTR = ::windows::w!("WM/WMShadowFileSourceFileType");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWriter: ::windows::core::PCWSTR = ::windows::w!("WM/Writer");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMYear: ::windows::core::PCWSTR = ::windows::w!("WM/Year");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWatermarkCLSID: ::windows::core::PCWSTR = ::windows::w!("WatermarkCLSID");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWatermarkConfig: ::windows::core::PCWSTR = ::windows::w!("WatermarkConfig");
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETSOURCE_URLCREDPOLICY_SETTINGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const NETSOURCE_URLCREDPOLICY_SETTING_SILENTLOGONOK: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const NETSOURCE_URLCREDPOLICY_SETTING_MUSTPROMPTUSER: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const NETSOURCE_URLCREDPOLICY_SETTING_ANONYMOUSONLY: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(2i32);
impl ::core::marker::Copy for NETSOURCE_URLCREDPOLICY_SETTINGS {}
impl ::core::clone::Clone for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NETSOURCE_URLCREDPOLICY_SETTINGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETSOURCE_URLCREDPOLICY_SETTINGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WEBSTREAM_SAMPLE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WEBSTREAM_SAMPLE_TYPE_FILE: WEBSTREAM_SAMPLE_TYPE = WEBSTREAM_SAMPLE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WEBSTREAM_SAMPLE_TYPE_RENDER: WEBSTREAM_SAMPLE_TYPE = WEBSTREAM_SAMPLE_TYPE(2i32);
impl ::core::marker::Copy for WEBSTREAM_SAMPLE_TYPE {}
impl ::core::clone::Clone for WEBSTREAM_SAMPLE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WEBSTREAM_SAMPLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WEBSTREAM_SAMPLE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WEBSTREAM_SAMPLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEBSTREAM_SAMPLE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_ATTR_DATATYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_DWORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_STRING: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_BINARY: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_BOOL: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(3i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_QWORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(4i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_WORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(5i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_GUID: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(6i32);
impl ::core::marker::Copy for WMT_ATTR_DATATYPE {}
impl ::core::clone::Clone for WMT_ATTR_DATATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_ATTR_DATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_ATTR_DATATYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_ATTR_DATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_ATTR_DATATYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_ATTR_IMAGETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IMAGETYPE_BITMAP: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IMAGETYPE_JPEG: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IMAGETYPE_GIF: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(3i32);
impl ::core::marker::Copy for WMT_ATTR_IMAGETYPE {}
impl ::core::clone::Clone for WMT_ATTR_IMAGETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_ATTR_IMAGETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_ATTR_IMAGETYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_ATTR_IMAGETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_ATTR_IMAGETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_CODEC_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CODECINFO_AUDIO: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CODECINFO_VIDEO: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CODECINFO_UNKNOWN: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(-1i32);
impl ::core::marker::Copy for WMT_CODEC_INFO_TYPE {}
impl ::core::clone::Clone for WMT_CODEC_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_CODEC_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_CODEC_INFO_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_CODEC_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_CODEC_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_CREDENTIAL_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CREDENTIAL_SAVE: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CREDENTIAL_DONT_CACHE: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CREDENTIAL_CLEAR_TEXT: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CREDENTIAL_PROXY: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CREDENTIAL_ENCRYPT: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(16i32);
impl ::core::marker::Copy for WMT_CREDENTIAL_FLAGS {}
impl ::core::clone::Clone for WMT_CREDENTIAL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_CREDENTIAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_CREDENTIAL_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_CREDENTIAL_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_DRMLA_TRUST(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_DRMLA_UNTRUSTED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_DRMLA_TRUSTED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_DRMLA_TAMPERED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(2i32);
impl ::core::marker::Copy for WMT_DRMLA_TRUST {}
impl ::core::clone::Clone for WMT_DRMLA_TRUST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_DRMLA_TRUST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_DRMLA_TRUST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_DRMLA_TRUST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_DRMLA_TRUST").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_FILESINK_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_FM_SINGLE_BUFFERS: WMT_FILESINK_MODE = WMT_FILESINK_MODE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_FM_FILESINK_DATA_UNITS: WMT_FILESINK_MODE = WMT_FILESINK_MODE(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_FM_FILESINK_UNBUFFERED: WMT_FILESINK_MODE = WMT_FILESINK_MODE(4i32);
impl ::core::marker::Copy for WMT_FILESINK_MODE {}
impl ::core::clone::Clone for WMT_FILESINK_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_FILESINK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_FILESINK_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_FILESINK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_FILESINK_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_IMAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_NONE: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_BITMAP: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_JPEG: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_GIF: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(3i32);
impl ::core::marker::Copy for WMT_IMAGE_TYPE {}
impl ::core::clone::Clone for WMT_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_IMAGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_INDEXER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_PRESENTATION_TIME: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_FRAME_NUMBERS: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_TIMECODE: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(2i32);
impl ::core::marker::Copy for WMT_INDEXER_TYPE {}
impl ::core::clone::Clone for WMT_INDEXER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_INDEXER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_INDEXER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_INDEXER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_INDEXER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_INDEX_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_NEAREST_DATA_UNIT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_NEAREST_OBJECT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_NEAREST_CLEAN_POINT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(3i32);
impl ::core::marker::Copy for WMT_INDEX_TYPE {}
impl ::core::clone::Clone for WMT_INDEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_INDEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_INDEX_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_INDEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_INDEX_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_MUSICSPEECH_CLASS_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_MS_CLASS_MUSIC: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_MS_CLASS_SPEECH: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_MS_CLASS_MIXED: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(2i32);
impl ::core::marker::Copy for WMT_MUSICSPEECH_CLASS_MODE {}
impl ::core::clone::Clone for WMT_MUSICSPEECH_CLASS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_MUSICSPEECH_CLASS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_MUSICSPEECH_CLASS_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_MUSICSPEECH_CLASS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_MUSICSPEECH_CLASS_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_NET_PROTOCOL(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROTOCOL_HTTP: WMT_NET_PROTOCOL = WMT_NET_PROTOCOL(0i32);
impl ::core::marker::Copy for WMT_NET_PROTOCOL {}
impl ::core::clone::Clone for WMT_NET_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_NET_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_NET_PROTOCOL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_NET_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_NET_PROTOCOL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_OFFSET_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFFSET_FORMAT_100NS: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFFSET_FORMAT_FRAME_NUMBERS: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFFSET_FORMAT_PLAYLIST_OFFSET: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFFSET_FORMAT_TIMECODE: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(3i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFFSET_FORMAT_100NS_APPROXIMATE: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(4i32);
impl ::core::marker::Copy for WMT_OFFSET_FORMAT {}
impl ::core::clone::Clone for WMT_OFFSET_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_OFFSET_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_OFFSET_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_OFFSET_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_OFFSET_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_PLAY_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PLAY_MODE_AUTOSELECT: WMT_PLAY_MODE = WMT_PLAY_MODE(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PLAY_MODE_LOCAL: WMT_PLAY_MODE = WMT_PLAY_MODE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PLAY_MODE_DOWNLOAD: WMT_PLAY_MODE = WMT_PLAY_MODE(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PLAY_MODE_STREAMING: WMT_PLAY_MODE = WMT_PLAY_MODE(3i32);
impl ::core::marker::Copy for WMT_PLAY_MODE {}
impl ::core::clone::Clone for WMT_PLAY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_PLAY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_PLAY_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_PLAY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_PLAY_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_PROXY_SETTINGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXY_SETTING_NONE: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXY_SETTING_MANUAL: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXY_SETTING_AUTO: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXY_SETTING_BROWSER: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(3i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXY_SETTING_MAX: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(4i32);
impl ::core::marker::Copy for WMT_PROXY_SETTINGS {}
impl ::core::clone::Clone for WMT_PROXY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_PROXY_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_PROXY_SETTINGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_PROXY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_PROXY_SETTINGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_RIGHTS(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_PLAYBACK: WMT_RIGHTS = WMT_RIGHTS(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_COPY_TO_NON_SDMI_DEVICE: WMT_RIGHTS = WMT_RIGHTS(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_COPY_TO_CD: WMT_RIGHTS = WMT_RIGHTS(8i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_COPY_TO_SDMI_DEVICE: WMT_RIGHTS = WMT_RIGHTS(16i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_ONE_TIME: WMT_RIGHTS = WMT_RIGHTS(32i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_SAVE_STREAM_PROTECTED: WMT_RIGHTS = WMT_RIGHTS(64i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_COPY: WMT_RIGHTS = WMT_RIGHTS(128i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_COLLABORATIVE_PLAY: WMT_RIGHTS = WMT_RIGHTS(256i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_SDMI_TRIGGER: WMT_RIGHTS = WMT_RIGHTS(65536i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_SDMI_NOMORECOPIES: WMT_RIGHTS = WMT_RIGHTS(131072i32);
impl ::core::marker::Copy for WMT_RIGHTS {}
impl ::core::clone::Clone for WMT_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_RIGHTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_RIGHTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_ERROR: WMT_STATUS = WMT_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OPENED: WMT_STATUS = WMT_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BUFFERING_START: WMT_STATUS = WMT_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BUFFERING_STOP: WMT_STATUS = WMT_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_EOF: WMT_STATUS = WMT_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_END_OF_FILE: WMT_STATUS = WMT_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_END_OF_SEGMENT: WMT_STATUS = WMT_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_END_OF_STREAMING: WMT_STATUS = WMT_STATUS(6i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_LOCATING: WMT_STATUS = WMT_STATUS(7i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CONNECTING: WMT_STATUS = WMT_STATUS(8i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NO_RIGHTS: WMT_STATUS = WMT_STATUS(9i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_MISSING_CODEC: WMT_STATUS = WMT_STATUS(10i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_STARTED: WMT_STATUS = WMT_STATUS(11i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_STOPPED: WMT_STATUS = WMT_STATUS(12i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLOSED: WMT_STATUS = WMT_STATUS(13i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_STRIDING: WMT_STATUS = WMT_STATUS(14i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TIMER: WMT_STATUS = WMT_STATUS(15i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_INDEX_PROGRESS: WMT_STATUS = WMT_STATUS(16i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_SAVEAS_START: WMT_STATUS = WMT_STATUS(17i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_SAVEAS_STOP: WMT_STATUS = WMT_STATUS(18i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NEW_SOURCEFLAGS: WMT_STATUS = WMT_STATUS(19i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NEW_METADATA: WMT_STATUS = WMT_STATUS(20i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BACKUPRESTORE_BEGIN: WMT_STATUS = WMT_STATUS(21i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_SOURCE_SWITCH: WMT_STATUS = WMT_STATUS(22i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_ACQUIRE_LICENSE: WMT_STATUS = WMT_STATUS(23i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_INDIVIDUALIZE: WMT_STATUS = WMT_STATUS(24i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NEEDS_INDIVIDUALIZATION: WMT_STATUS = WMT_STATUS(25i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NO_RIGHTS_EX: WMT_STATUS = WMT_STATUS(26i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BACKUPRESTORE_END: WMT_STATUS = WMT_STATUS(27i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BACKUPRESTORE_CONNECTING: WMT_STATUS = WMT_STATUS(28i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BACKUPRESTORE_DISCONNECTING: WMT_STATUS = WMT_STATUS(29i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_ERROR_WITHURL: WMT_STATUS = WMT_STATUS(30i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RESTRICTED_LICENSE: WMT_STATUS = WMT_STATUS(31i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLIENT_CONNECT: WMT_STATUS = WMT_STATUS(32i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLIENT_DISCONNECT: WMT_STATUS = WMT_STATUS(33i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NATIVE_OUTPUT_PROPS_CHANGED: WMT_STATUS = WMT_STATUS(34i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RECONNECT_START: WMT_STATUS = WMT_STATUS(35i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RECONNECT_END: WMT_STATUS = WMT_STATUS(36i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLIENT_CONNECT_EX: WMT_STATUS = WMT_STATUS(37i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLIENT_DISCONNECT_EX: WMT_STATUS = WMT_STATUS(38i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_SET_FEC_SPAN: WMT_STATUS = WMT_STATUS(39i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PREROLL_READY: WMT_STATUS = WMT_STATUS(40i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PREROLL_COMPLETE: WMT_STATUS = WMT_STATUS(41i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLIENT_PROPERTIES: WMT_STATUS = WMT_STATUS(42i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_LICENSEURL_SIGNATURE_STATE: WMT_STATUS = WMT_STATUS(43i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_INIT_PLAYLIST_BURN: WMT_STATUS = WMT_STATUS(44i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TRANSCRYPTOR_INIT: WMT_STATUS = WMT_STATUS(45i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TRANSCRYPTOR_SEEKED: WMT_STATUS = WMT_STATUS(46i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TRANSCRYPTOR_READ: WMT_STATUS = WMT_STATUS(47i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TRANSCRYPTOR_CLOSED: WMT_STATUS = WMT_STATUS(48i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXIMITY_RESULT: WMT_STATUS = WMT_STATUS(49i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXIMITY_COMPLETED: WMT_STATUS = WMT_STATUS(50i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CONTENT_ENABLER: WMT_STATUS = WMT_STATUS(51i32);
impl ::core::marker::Copy for WMT_STATUS {}
impl ::core::clone::Clone for WMT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_STORAGE_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_Storage_Format_MP3: WMT_STORAGE_FORMAT = WMT_STORAGE_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_Storage_Format_V1: WMT_STORAGE_FORMAT = WMT_STORAGE_FORMAT(1i32);
impl ::core::marker::Copy for WMT_STORAGE_FORMAT {}
impl ::core::clone::Clone for WMT_STORAGE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_STORAGE_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_STORAGE_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_STORAGE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STORAGE_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_STREAM_SELECTION(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFF: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLEANPOINT_ONLY: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_ON: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(2i32);
impl ::core::marker::Copy for WMT_STREAM_SELECTION {}
impl ::core::clone::Clone for WMT_STREAM_SELECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_STREAM_SELECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_STREAM_SELECTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_STREAM_SELECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STREAM_SELECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_TIMECODE_FRAMERATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TIMECODE_FRAMERATE_30: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TIMECODE_FRAMERATE_30DROP: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TIMECODE_FRAMERATE_25: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TIMECODE_FRAMERATE_24: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(3i32);
impl ::core::marker::Copy for WMT_TIMECODE_FRAMERATE {}
impl ::core::clone::Clone for WMT_TIMECODE_FRAMERATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_TIMECODE_FRAMERATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_TIMECODE_FRAMERATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_TIMECODE_FRAMERATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_TIMECODE_FRAMERATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_TRANSPORT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_Transport_Type_Unreliable: WMT_TRANSPORT_TYPE = WMT_TRANSPORT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_Transport_Type_Reliable: WMT_TRANSPORT_TYPE = WMT_TRANSPORT_TYPE(1i32);
impl ::core::marker::Copy for WMT_TRANSPORT_TYPE {}
impl ::core::clone::Clone for WMT_TRANSPORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_TRANSPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_TRANSPORT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_TRANSPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_TRANSPORT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VER_4_0: WMT_VERSION = WMT_VERSION(262144i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VER_7_0: WMT_VERSION = WMT_VERSION(458752i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VER_8_0: WMT_VERSION = WMT_VERSION(524288i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VER_9_0: WMT_VERSION = WMT_VERSION(589824i32);
impl ::core::marker::Copy for WMT_VERSION {}
impl ::core::clone::Clone for WMT_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMT_WATERMARK_ENTRY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_WMETYPE_AUDIO: WMT_WATERMARK_ENTRY_TYPE = WMT_WATERMARK_ENTRY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_WMETYPE_VIDEO: WMT_WATERMARK_ENTRY_TYPE = WMT_WATERMARK_ENTRY_TYPE(2i32);
impl ::core::marker::Copy for WMT_WATERMARK_ENTRY_TYPE {}
impl ::core::clone::Clone for WMT_WATERMARK_ENTRY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_WATERMARK_ENTRY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMT_WATERMARK_ENTRY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMT_WATERMARK_ENTRY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_WATERMARK_ENTRY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WM_AETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_AETYPE_INCLUDE: WM_AETYPE = WM_AETYPE(105i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_AETYPE_EXCLUDE: WM_AETYPE = WM_AETYPE(101i32);
impl ::core::marker::Copy for WM_AETYPE {}
impl ::core::clone::Clone for WM_AETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_AETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WM_AETYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WM_AETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_AETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WM_DM_INTERLACED_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_NOTINTERLACED: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_DEINTERLACE_NORMAL: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_DEINTERLACE_HALFSIZE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_DEINTERLACE_HALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_DEINTERLACE_INVERSETELECINE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_DEINTERLACE_VERTICALHALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(5i32);
impl ::core::marker::Copy for WM_DM_INTERLACED_TYPE {}
impl ::core::clone::Clone for WM_DM_INTERLACED_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_DM_INTERLACED_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WM_DM_INTERLACED_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WM_DM_INTERLACED_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_DM_INTERLACED_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WM_DM_IT_FIRST_FRAME_COHERENCY(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_DISABLE_COHERENT_MODE: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(3i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(4i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(5i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(6i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(7i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(8i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(9i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(10i32);
impl ::core::marker::Copy for WM_DM_IT_FIRST_FRAME_COHERENCY {}
impl ::core::clone::Clone for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WM_DM_IT_FIRST_FRAME_COHERENCY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_DM_IT_FIRST_FRAME_COHERENCY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WM_PLAYBACK_DRC_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_PLAYBACK_DRC_HIGH: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_PLAYBACK_DRC_MEDIUM: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_PLAYBACK_DRC_LOW: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(2i32);
impl ::core::marker::Copy for WM_PLAYBACK_DRC_LEVEL {}
impl ::core::clone::Clone for WM_PLAYBACK_DRC_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_PLAYBACK_DRC_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WM_PLAYBACK_DRC_LEVEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WM_PLAYBACK_DRC_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_PLAYBACK_DRC_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WM_SFEX_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SFEX_NOTASYNCPOINT: WM_SFEX_TYPE = WM_SFEX_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SFEX_DATALOSS: WM_SFEX_TYPE = WM_SFEX_TYPE(4i32);
impl ::core::marker::Copy for WM_SFEX_TYPE {}
impl ::core::clone::Clone for WM_SFEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_SFEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WM_SFEX_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WM_SFEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_SFEX_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WM_SF_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SF_CLEANPOINT: WM_SF_TYPE = WM_SF_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SF_DISCONTINUITY: WM_SF_TYPE = WM_SF_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SF_DATALOSS: WM_SF_TYPE = WM_SF_TYPE(4i32);
impl ::core::marker::Copy for WM_SF_TYPE {}
impl ::core::clone::Clone for WM_SF_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_SF_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WM_SF_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WM_SF_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_SF_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _AM_ASFWRITERCONFIG_PARAM(pub i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const AM_CONFIGASFWRITER_PARAM_AUTOINDEX: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(1i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const AM_CONFIGASFWRITER_PARAM_MULTIPASS: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(2i32);
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const AM_CONFIGASFWRITER_PARAM_DONTCOMPRESS: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(3i32);
impl ::core::marker::Copy for _AM_ASFWRITERCONFIG_PARAM {}
impl ::core::clone::Clone for _AM_ASFWRITERCONFIG_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _AM_ASFWRITERCONFIG_PARAM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for _AM_ASFWRITERCONFIG_PARAM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for _AM_ASFWRITERCONFIG_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_AM_ASFWRITERCONFIG_PARAM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct AM_WMT_EVENT_DATA {
    pub hrStatus: ::windows::core::HRESULT,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for AM_WMT_EVENT_DATA {}
impl ::core::clone::Clone for AM_WMT_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AM_WMT_EVENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AM_WMT_EVENT_DATA").field("hrStatus", &self.hrStatus).field("pData", &self.pData).finish()
    }
}
impl ::windows::core::TypeKind for AM_WMT_EVENT_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AM_WMT_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.hrStatus == other.hrStatus && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for AM_WMT_EVENT_DATA {}
impl ::core::default::Default for AM_WMT_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_COPY_OPL {
    pub wMinimumCopyLevel: u16,
    pub oplIdIncludes: DRM_OPL_OUTPUT_IDS,
    pub oplIdExcludes: DRM_OPL_OUTPUT_IDS,
}
impl ::core::marker::Copy for DRM_COPY_OPL {}
impl ::core::clone::Clone for DRM_COPY_OPL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_COPY_OPL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_COPY_OPL").field("wMinimumCopyLevel", &self.wMinimumCopyLevel).field("oplIdIncludes", &self.oplIdIncludes).field("oplIdExcludes", &self.oplIdExcludes).finish()
    }
}
impl ::windows::core::TypeKind for DRM_COPY_OPL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRM_COPY_OPL {
    fn eq(&self, other: &Self) -> bool {
        self.wMinimumCopyLevel == other.wMinimumCopyLevel && self.oplIdIncludes == other.oplIdIncludes && self.oplIdExcludes == other.oplIdExcludes
    }
}
impl ::core::cmp::Eq for DRM_COPY_OPL {}
impl ::core::default::Default for DRM_COPY_OPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    pub wCompressedDigitalVideo: u16,
    pub wUncompressedDigitalVideo: u16,
    pub wAnalogVideo: u16,
    pub wCompressedDigitalAudio: u16,
    pub wUncompressedDigitalAudio: u16,
}
impl ::core::marker::Copy for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::core::clone::Clone for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS").field("wCompressedDigitalVideo", &self.wCompressedDigitalVideo).field("wUncompressedDigitalVideo", &self.wUncompressedDigitalVideo).field("wAnalogVideo", &self.wAnalogVideo).field("wCompressedDigitalAudio", &self.wCompressedDigitalAudio).field("wUncompressedDigitalAudio", &self.wUncompressedDigitalAudio).finish()
    }
}
impl ::windows::core::TypeKind for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.wCompressedDigitalVideo == other.wCompressedDigitalVideo && self.wUncompressedDigitalVideo == other.wUncompressedDigitalVideo && self.wAnalogVideo == other.wAnalogVideo && self.wCompressedDigitalAudio == other.wCompressedDigitalAudio && self.wUncompressedDigitalAudio == other.wUncompressedDigitalAudio
    }
}
impl ::core::cmp::Eq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::core::default::Default for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_OPL_OUTPUT_IDS {
    pub cIds: u16,
    pub rgIds: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for DRM_OPL_OUTPUT_IDS {}
impl ::core::clone::Clone for DRM_OPL_OUTPUT_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_OPL_OUTPUT_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_OPL_OUTPUT_IDS").field("cIds", &self.cIds).field("rgIds", &self.rgIds).finish()
    }
}
impl ::windows::core::TypeKind for DRM_OPL_OUTPUT_IDS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRM_OPL_OUTPUT_IDS {
    fn eq(&self, other: &Self) -> bool {
        self.cIds == other.cIds && self.rgIds == other.rgIds
    }
}
impl ::core::cmp::Eq for DRM_OPL_OUTPUT_IDS {}
impl ::core::default::Default for DRM_OPL_OUTPUT_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_OUTPUT_PROTECTION {
    pub guidId: ::windows::core::GUID,
    pub bConfigData: u8,
}
impl ::core::marker::Copy for DRM_OUTPUT_PROTECTION {}
impl ::core::clone::Clone for DRM_OUTPUT_PROTECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_OUTPUT_PROTECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_OUTPUT_PROTECTION").field("guidId", &self.guidId).field("bConfigData", &self.bConfigData).finish()
    }
}
impl ::windows::core::TypeKind for DRM_OUTPUT_PROTECTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRM_OUTPUT_PROTECTION {
    fn eq(&self, other: &Self) -> bool {
        self.guidId == other.guidId && self.bConfigData == other.bConfigData
    }
}
impl ::core::cmp::Eq for DRM_OUTPUT_PROTECTION {}
impl ::core::default::Default for DRM_OUTPUT_PROTECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_PLAY_OPL {
    pub minOPL: DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS,
    pub oplIdReserved: DRM_OPL_OUTPUT_IDS,
    pub vopi: DRM_VIDEO_OUTPUT_PROTECTION_IDS,
}
impl ::core::marker::Copy for DRM_PLAY_OPL {}
impl ::core::clone::Clone for DRM_PLAY_OPL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_PLAY_OPL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_PLAY_OPL").field("minOPL", &self.minOPL).field("oplIdReserved", &self.oplIdReserved).field("vopi", &self.vopi).finish()
    }
}
impl ::windows::core::TypeKind for DRM_PLAY_OPL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRM_PLAY_OPL {
    fn eq(&self, other: &Self) -> bool {
        self.minOPL == other.minOPL && self.oplIdReserved == other.oplIdReserved && self.vopi == other.vopi
    }
}
impl ::core::cmp::Eq for DRM_PLAY_OPL {}
impl ::core::default::Default for DRM_PLAY_OPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_VAL16 {
    pub val: [u8; 16],
}
impl ::core::marker::Copy for DRM_VAL16 {}
impl ::core::clone::Clone for DRM_VAL16 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_VAL16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_VAL16").field("val", &self.val).finish()
    }
}
impl ::windows::core::TypeKind for DRM_VAL16 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRM_VAL16 {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}
impl ::core::cmp::Eq for DRM_VAL16 {}
impl ::core::default::Default for DRM_VAL16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    pub cEntries: u16,
    pub rgVop: *mut DRM_OUTPUT_PROTECTION,
}
impl ::core::marker::Copy for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::core::clone::Clone for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_VIDEO_OUTPUT_PROTECTION_IDS").field("cEntries", &self.cEntries).field("rgVop", &self.rgVop).finish()
    }
}
impl ::windows::core::TypeKind for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.rgVop == other.rgVop
    }
}
impl ::core::cmp::Eq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::core::default::Default for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMDRM_IMPORT_INIT_STRUCT {
    pub dwVersion: u32,
    pub cbEncryptedSessionKeyMessage: u32,
    pub pbEncryptedSessionKeyMessage: *mut u8,
    pub cbEncryptedKeyMessage: u32,
    pub pbEncryptedKeyMessage: *mut u8,
}
impl ::core::marker::Copy for WMDRM_IMPORT_INIT_STRUCT {}
impl ::core::clone::Clone for WMDRM_IMPORT_INIT_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMDRM_IMPORT_INIT_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDRM_IMPORT_INIT_STRUCT").field("dwVersion", &self.dwVersion).field("cbEncryptedSessionKeyMessage", &self.cbEncryptedSessionKeyMessage).field("pbEncryptedSessionKeyMessage", &self.pbEncryptedSessionKeyMessage).field("cbEncryptedKeyMessage", &self.cbEncryptedKeyMessage).field("pbEncryptedKeyMessage", &self.pbEncryptedKeyMessage).finish()
    }
}
impl ::windows::core::TypeKind for WMDRM_IMPORT_INIT_STRUCT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WMDRM_IMPORT_INIT_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbEncryptedSessionKeyMessage == other.cbEncryptedSessionKeyMessage && self.pbEncryptedSessionKeyMessage == other.pbEncryptedSessionKeyMessage && self.cbEncryptedKeyMessage == other.cbEncryptedKeyMessage && self.pbEncryptedKeyMessage == other.pbEncryptedKeyMessage
    }
}
impl ::core::cmp::Eq for WMDRM_IMPORT_INIT_STRUCT {}
impl ::core::default::Default for WMDRM_IMPORT_INIT_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMMPEG2VIDEOINFO {
    pub hdr: WMVIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub dwSequenceHeader: [u32; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMMPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMMPEG2VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WMMPEG2VIDEOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMMPEG2VIDEOINFO").field("hdr", &self.hdr).field("dwStartTimeCode", &self.dwStartTimeCode).field("cbSequenceHeader", &self.cbSequenceHeader).field("dwProfile", &self.dwProfile).field("dwLevel", &self.dwLevel).field("dwFlags", &self.dwFlags).field("dwSequenceHeader", &self.dwSequenceHeader).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for WMMPEG2VIDEOINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMMPEG2VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwStartTimeCode == other.dwStartTimeCode && self.cbSequenceHeader == other.cbSequenceHeader && self.dwProfile == other.dwProfile && self.dwLevel == other.dwLevel && self.dwFlags == other.dwFlags && self.dwSequenceHeader == other.dwSequenceHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMMPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMMPEG2VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMSCRIPTFORMAT {
    pub scriptType: ::windows::core::GUID,
}
impl ::core::marker::Copy for WMSCRIPTFORMAT {}
impl ::core::clone::Clone for WMSCRIPTFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMSCRIPTFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMSCRIPTFORMAT").field("scriptType", &self.scriptType).finish()
    }
}
impl ::windows::core::TypeKind for WMSCRIPTFORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WMSCRIPTFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.scriptType == other.scriptType
    }
}
impl ::core::cmp::Eq for WMSCRIPTFORMAT {}
impl ::core::default::Default for WMSCRIPTFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_BUFFER_SEGMENT {
    pub pBuffer: ::std::mem::ManuallyDrop<::core::option::Option<INSSBuffer>>,
    pub cbOffset: u32,
    pub cbLength: u32,
}
impl ::core::clone::Clone for WMT_BUFFER_SEGMENT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for WMT_BUFFER_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_BUFFER_SEGMENT").field("pBuffer", &self.pBuffer).field("cbOffset", &self.cbOffset).field("cbLength", &self.cbLength).finish()
    }
}
impl ::windows::core::TypeKind for WMT_BUFFER_SEGMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WMT_BUFFER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.cbOffset == other.cbOffset && self.cbLength == other.cbLength
    }
}
impl ::core::cmp::Eq for WMT_BUFFER_SEGMENT {}
impl ::core::default::Default for WMT_BUFFER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_COLORSPACEINFO_EXTENSION_DATA {
    pub ucColorPrimaries: u8,
    pub ucColorTransferChar: u8,
    pub ucColorMatrixCoef: u8,
}
impl ::core::marker::Copy for WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::core::clone::Clone for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_COLORSPACEINFO_EXTENSION_DATA").field("ucColorPrimaries", &self.ucColorPrimaries).field("ucColorTransferChar", &self.ucColorTransferChar).field("ucColorMatrixCoef", &self.ucColorMatrixCoef).finish()
    }
}
impl ::windows::core::TypeKind for WMT_COLORSPACEINFO_EXTENSION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ucColorPrimaries == other.ucColorPrimaries && self.ucColorTransferChar == other.ucColorTransferChar && self.ucColorMatrixCoef == other.ucColorMatrixCoef
    }
}
impl ::core::cmp::Eq for WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::core::default::Default for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_FILESINK_DATA_UNIT {
    pub packetHeaderBuffer: WMT_BUFFER_SEGMENT,
    pub cPayloads: u32,
    pub pPayloadHeaderBuffers: *mut WMT_BUFFER_SEGMENT,
    pub cPayloadDataFragments: u32,
    pub pPayloadDataFragments: *mut WMT_PAYLOAD_FRAGMENT,
}
impl ::core::clone::Clone for WMT_FILESINK_DATA_UNIT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for WMT_FILESINK_DATA_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_FILESINK_DATA_UNIT").field("packetHeaderBuffer", &self.packetHeaderBuffer).field("cPayloads", &self.cPayloads).field("pPayloadHeaderBuffers", &self.pPayloadHeaderBuffers).field("cPayloadDataFragments", &self.cPayloadDataFragments).field("pPayloadDataFragments", &self.pPayloadDataFragments).finish()
    }
}
impl ::windows::core::TypeKind for WMT_FILESINK_DATA_UNIT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WMT_FILESINK_DATA_UNIT {
    fn eq(&self, other: &Self) -> bool {
        self.packetHeaderBuffer == other.packetHeaderBuffer && self.cPayloads == other.cPayloads && self.pPayloadHeaderBuffers == other.pPayloadHeaderBuffers && self.cPayloadDataFragments == other.cPayloadDataFragments && self.pPayloadDataFragments == other.pPayloadDataFragments
    }
}
impl ::core::cmp::Eq for WMT_FILESINK_DATA_UNIT {}
impl ::core::default::Default for WMT_FILESINK_DATA_UNIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_PAYLOAD_FRAGMENT {
    pub dwPayloadIndex: u32,
    pub segmentData: WMT_BUFFER_SEGMENT,
}
impl ::core::clone::Clone for WMT_PAYLOAD_FRAGMENT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for WMT_PAYLOAD_FRAGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_PAYLOAD_FRAGMENT").field("dwPayloadIndex", &self.dwPayloadIndex).field("segmentData", &self.segmentData).finish()
    }
}
impl ::windows::core::TypeKind for WMT_PAYLOAD_FRAGMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WMT_PAYLOAD_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.dwPayloadIndex == other.dwPayloadIndex && self.segmentData == other.segmentData
    }
}
impl ::core::cmp::Eq for WMT_PAYLOAD_FRAGMENT {}
impl ::core::default::Default for WMT_PAYLOAD_FRAGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_TIMECODE_EXTENSION_DATA {
    pub wRange: u16,
    pub dwTimecode: u32,
    pub dwUserbits: u32,
    pub dwAmFlags: u32,
}
impl ::core::marker::Copy for WMT_TIMECODE_EXTENSION_DATA {}
impl ::core::clone::Clone for WMT_TIMECODE_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WMT_TIMECODE_EXTENSION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WMT_TIMECODE_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_VIDEOIMAGE_SAMPLE {
    pub dwMagic: u32,
    pub cbStruct: u32,
    pub dwControlFlags: u32,
    pub dwInputFlagsCur: u32,
    pub lCurMotionXtoX: i32,
    pub lCurMotionYtoX: i32,
    pub lCurMotionXoffset: i32,
    pub lCurMotionXtoY: i32,
    pub lCurMotionYtoY: i32,
    pub lCurMotionYoffset: i32,
    pub lCurBlendCoef1: i32,
    pub lCurBlendCoef2: i32,
    pub dwInputFlagsPrev: u32,
    pub lPrevMotionXtoX: i32,
    pub lPrevMotionYtoX: i32,
    pub lPrevMotionXoffset: i32,
    pub lPrevMotionXtoY: i32,
    pub lPrevMotionYtoY: i32,
    pub lPrevMotionYoffset: i32,
    pub lPrevBlendCoef1: i32,
    pub lPrevBlendCoef2: i32,
}
impl ::core::marker::Copy for WMT_VIDEOIMAGE_SAMPLE {}
impl ::core::clone::Clone for WMT_VIDEOIMAGE_SAMPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_VIDEOIMAGE_SAMPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_VIDEOIMAGE_SAMPLE")
            .field("dwMagic", &self.dwMagic)
            .field("cbStruct", &self.cbStruct)
            .field("dwControlFlags", &self.dwControlFlags)
            .field("dwInputFlagsCur", &self.dwInputFlagsCur)
            .field("lCurMotionXtoX", &self.lCurMotionXtoX)
            .field("lCurMotionYtoX", &self.lCurMotionYtoX)
            .field("lCurMotionXoffset", &self.lCurMotionXoffset)
            .field("lCurMotionXtoY", &self.lCurMotionXtoY)
            .field("lCurMotionYtoY", &self.lCurMotionYtoY)
            .field("lCurMotionYoffset", &self.lCurMotionYoffset)
            .field("lCurBlendCoef1", &self.lCurBlendCoef1)
            .field("lCurBlendCoef2", &self.lCurBlendCoef2)
            .field("dwInputFlagsPrev", &self.dwInputFlagsPrev)
            .field("lPrevMotionXtoX", &self.lPrevMotionXtoX)
            .field("lPrevMotionYtoX", &self.lPrevMotionYtoX)
            .field("lPrevMotionXoffset", &self.lPrevMotionXoffset)
            .field("lPrevMotionXtoY", &self.lPrevMotionXtoY)
            .field("lPrevMotionYtoY", &self.lPrevMotionYtoY)
            .field("lPrevMotionYoffset", &self.lPrevMotionYoffset)
            .field("lPrevBlendCoef1", &self.lPrevBlendCoef1)
            .field("lPrevBlendCoef2", &self.lPrevBlendCoef2)
            .finish()
    }
}
impl ::windows::core::TypeKind for WMT_VIDEOIMAGE_SAMPLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic
            && self.cbStruct == other.cbStruct
            && self.dwControlFlags == other.dwControlFlags
            && self.dwInputFlagsCur == other.dwInputFlagsCur
            && self.lCurMotionXtoX == other.lCurMotionXtoX
            && self.lCurMotionYtoX == other.lCurMotionYtoX
            && self.lCurMotionXoffset == other.lCurMotionXoffset
            && self.lCurMotionXtoY == other.lCurMotionXtoY
            && self.lCurMotionYtoY == other.lCurMotionYtoY
            && self.lCurMotionYoffset == other.lCurMotionYoffset
            && self.lCurBlendCoef1 == other.lCurBlendCoef1
            && self.lCurBlendCoef2 == other.lCurBlendCoef2
            && self.dwInputFlagsPrev == other.dwInputFlagsPrev
            && self.lPrevMotionXtoX == other.lPrevMotionXtoX
            && self.lPrevMotionYtoX == other.lPrevMotionYtoX
            && self.lPrevMotionXoffset == other.lPrevMotionXoffset
            && self.lPrevMotionXtoY == other.lPrevMotionXtoY
            && self.lPrevMotionYtoY == other.lPrevMotionYtoY
            && self.lPrevMotionYoffset == other.lPrevMotionYoffset
            && self.lPrevBlendCoef1 == other.lPrevBlendCoef1
            && self.lPrevBlendCoef2 == other.lPrevBlendCoef2
    }
}
impl ::core::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE {}
impl ::core::default::Default for WMT_VIDEOIMAGE_SAMPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WMT_VIDEOIMAGE_SAMPLE2 {
    pub dwMagic: u32,
    pub dwStructSize: u32,
    pub dwControlFlags: u32,
    pub dwViewportWidth: u32,
    pub dwViewportHeight: u32,
    pub dwCurrImageWidth: u32,
    pub dwCurrImageHeight: u32,
    pub fCurrRegionX0: f32,
    pub fCurrRegionY0: f32,
    pub fCurrRegionWidth: f32,
    pub fCurrRegionHeight: f32,
    pub fCurrBlendCoef: f32,
    pub dwPrevImageWidth: u32,
    pub dwPrevImageHeight: u32,
    pub fPrevRegionX0: f32,
    pub fPrevRegionY0: f32,
    pub fPrevRegionWidth: f32,
    pub fPrevRegionHeight: f32,
    pub fPrevBlendCoef: f32,
    pub dwEffectType: u32,
    pub dwNumEffectParas: u32,
    pub fEffectPara0: f32,
    pub fEffectPara1: f32,
    pub fEffectPara2: f32,
    pub fEffectPara3: f32,
    pub fEffectPara4: f32,
    pub bKeepPrevImage: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WMT_VIDEOIMAGE_SAMPLE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WMT_VIDEOIMAGE_SAMPLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WMT_VIDEOIMAGE_SAMPLE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_VIDEOIMAGE_SAMPLE2")
            .field("dwMagic", &self.dwMagic)
            .field("dwStructSize", &self.dwStructSize)
            .field("dwControlFlags", &self.dwControlFlags)
            .field("dwViewportWidth", &self.dwViewportWidth)
            .field("dwViewportHeight", &self.dwViewportHeight)
            .field("dwCurrImageWidth", &self.dwCurrImageWidth)
            .field("dwCurrImageHeight", &self.dwCurrImageHeight)
            .field("fCurrRegionX0", &self.fCurrRegionX0)
            .field("fCurrRegionY0", &self.fCurrRegionY0)
            .field("fCurrRegionWidth", &self.fCurrRegionWidth)
            .field("fCurrRegionHeight", &self.fCurrRegionHeight)
            .field("fCurrBlendCoef", &self.fCurrBlendCoef)
            .field("dwPrevImageWidth", &self.dwPrevImageWidth)
            .field("dwPrevImageHeight", &self.dwPrevImageHeight)
            .field("fPrevRegionX0", &self.fPrevRegionX0)
            .field("fPrevRegionY0", &self.fPrevRegionY0)
            .field("fPrevRegionWidth", &self.fPrevRegionWidth)
            .field("fPrevRegionHeight", &self.fPrevRegionHeight)
            .field("fPrevBlendCoef", &self.fPrevBlendCoef)
            .field("dwEffectType", &self.dwEffectType)
            .field("dwNumEffectParas", &self.dwNumEffectParas)
            .field("fEffectPara0", &self.fEffectPara0)
            .field("fEffectPara1", &self.fEffectPara1)
            .field("fEffectPara2", &self.fEffectPara2)
            .field("fEffectPara3", &self.fEffectPara3)
            .field("fEffectPara4", &self.fEffectPara4)
            .field("bKeepPrevImage", &self.bKeepPrevImage)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WMT_VIDEOIMAGE_SAMPLE2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic
            && self.dwStructSize == other.dwStructSize
            && self.dwControlFlags == other.dwControlFlags
            && self.dwViewportWidth == other.dwViewportWidth
            && self.dwViewportHeight == other.dwViewportHeight
            && self.dwCurrImageWidth == other.dwCurrImageWidth
            && self.dwCurrImageHeight == other.dwCurrImageHeight
            && self.fCurrRegionX0 == other.fCurrRegionX0
            && self.fCurrRegionY0 == other.fCurrRegionY0
            && self.fCurrRegionWidth == other.fCurrRegionWidth
            && self.fCurrRegionHeight == other.fCurrRegionHeight
            && self.fCurrBlendCoef == other.fCurrBlendCoef
            && self.dwPrevImageWidth == other.dwPrevImageWidth
            && self.dwPrevImageHeight == other.dwPrevImageHeight
            && self.fPrevRegionX0 == other.fPrevRegionX0
            && self.fPrevRegionY0 == other.fPrevRegionY0
            && self.fPrevRegionWidth == other.fPrevRegionWidth
            && self.fPrevRegionHeight == other.fPrevRegionHeight
            && self.fPrevBlendCoef == other.fPrevBlendCoef
            && self.dwEffectType == other.dwEffectType
            && self.dwNumEffectParas == other.dwNumEffectParas
            && self.fEffectPara0 == other.fEffectPara0
            && self.fEffectPara1 == other.fEffectPara1
            && self.fEffectPara2 == other.fEffectPara2
            && self.fEffectPara3 == other.fEffectPara3
            && self.fEffectPara4 == other.fEffectPara4
            && self.bKeepPrevImage == other.bKeepPrevImage
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WMT_VIDEOIMAGE_SAMPLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_WATERMARK_ENTRY {
    pub wmetType: WMT_WATERMARK_ENTRY_TYPE,
    pub clsid: ::windows::core::GUID,
    pub cbDisplayName: u32,
    pub pwszDisplayName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WMT_WATERMARK_ENTRY {}
impl ::core::clone::Clone for WMT_WATERMARK_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_WATERMARK_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WATERMARK_ENTRY").field("wmetType", &self.wmetType).field("clsid", &self.clsid).field("cbDisplayName", &self.cbDisplayName).field("pwszDisplayName", &self.pwszDisplayName).finish()
    }
}
impl ::windows::core::TypeKind for WMT_WATERMARK_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WMT_WATERMARK_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.wmetType == other.wmetType && self.clsid == other.clsid && self.cbDisplayName == other.cbDisplayName && self.pwszDisplayName == other.pwszDisplayName
    }
}
impl ::core::cmp::Eq for WMT_WATERMARK_ENTRY {}
impl ::core::default::Default for WMT_WATERMARK_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_WEBSTREAM_FORMAT {
    pub cbSize: u16,
    pub cbSampleHeaderFixedData: u16,
    pub wVersion: u16,
    pub wReserved: u16,
}
impl ::core::marker::Copy for WMT_WEBSTREAM_FORMAT {}
impl ::core::clone::Clone for WMT_WEBSTREAM_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_WEBSTREAM_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WEBSTREAM_FORMAT").field("cbSize", &self.cbSize).field("cbSampleHeaderFixedData", &self.cbSampleHeaderFixedData).field("wVersion", &self.wVersion).field("wReserved", &self.wReserved).finish()
    }
}
impl ::windows::core::TypeKind for WMT_WEBSTREAM_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WMT_WEBSTREAM_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cbSampleHeaderFixedData == other.cbSampleHeaderFixedData && self.wVersion == other.wVersion && self.wReserved == other.wReserved
    }
}
impl ::core::cmp::Eq for WMT_WEBSTREAM_FORMAT {}
impl ::core::default::Default for WMT_WEBSTREAM_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_WEBSTREAM_SAMPLE_HEADER {
    pub cbLength: u16,
    pub wPart: u16,
    pub cTotalParts: u16,
    pub wSampleType: u16,
    pub wszURL: [u16; 1],
}
impl ::core::marker::Copy for WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::core::clone::Clone for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WEBSTREAM_SAMPLE_HEADER").field("cbLength", &self.cbLength).field("wPart", &self.wPart).field("cTotalParts", &self.cTotalParts).field("wSampleType", &self.wSampleType).field("wszURL", &self.wszURL).finish()
    }
}
impl ::windows::core::TypeKind for WMT_WEBSTREAM_SAMPLE_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbLength == other.cbLength && self.wPart == other.wPart && self.cTotalParts == other.cTotalParts && self.wSampleType == other.wSampleType && self.wszURL == other.wszURL
    }
}
impl ::core::cmp::Eq for WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::core::default::Default for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMVIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMVIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMVIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WMVIDEOINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMVIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for WMVIDEOINFOHEADER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMVIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwBitRate == other.dwBitRate && self.dwBitErrorRate == other.dwBitErrorRate && self.AvgTimePerFrame == other.AvgTimePerFrame && self.bmiHeader == other.bmiHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMVIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMVIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMVIDEOINFOHEADER2 {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMVIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMVIDEOINFOHEADER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WMVIDEOINFOHEADER2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMVIDEOINFOHEADER2")
            .field("rcSource", &self.rcSource)
            .field("rcTarget", &self.rcTarget)
            .field("dwBitRate", &self.dwBitRate)
            .field("dwBitErrorRate", &self.dwBitErrorRate)
            .field("AvgTimePerFrame", &self.AvgTimePerFrame)
            .field("dwInterlaceFlags", &self.dwInterlaceFlags)
            .field("dwCopyProtectFlags", &self.dwCopyProtectFlags)
            .field("dwPictAspectRatioX", &self.dwPictAspectRatioX)
            .field("dwPictAspectRatioY", &self.dwPictAspectRatioY)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("bmiHeader", &self.bmiHeader)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for WMVIDEOINFOHEADER2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMVIDEOINFOHEADER2 {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwBitRate == other.dwBitRate && self.dwBitErrorRate == other.dwBitErrorRate && self.AvgTimePerFrame == other.AvgTimePerFrame && self.dwInterlaceFlags == other.dwInterlaceFlags && self.dwCopyProtectFlags == other.dwCopyProtectFlags && self.dwPictAspectRatioX == other.dwPictAspectRatioX && self.dwPictAspectRatioY == other.dwPictAspectRatioY && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.bmiHeader == other.bmiHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMVIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMVIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_ADDRESS_ACCESSENTRY {
    pub dwIPAddress: u32,
    pub dwMask: u32,
}
impl ::core::marker::Copy for WM_ADDRESS_ACCESSENTRY {}
impl ::core::clone::Clone for WM_ADDRESS_ACCESSENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_ADDRESS_ACCESSENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_ADDRESS_ACCESSENTRY").field("dwIPAddress", &self.dwIPAddress).field("dwMask", &self.dwMask).finish()
    }
}
impl ::windows::core::TypeKind for WM_ADDRESS_ACCESSENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WM_ADDRESS_ACCESSENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPAddress == other.dwIPAddress && self.dwMask == other.dwMask
    }
}
impl ::core::cmp::Eq for WM_ADDRESS_ACCESSENTRY {}
impl ::core::default::Default for WM_ADDRESS_ACCESSENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_CLIENT_PROPERTIES {
    pub dwIPAddress: u32,
    pub dwPort: u32,
}
impl ::core::marker::Copy for WM_CLIENT_PROPERTIES {}
impl ::core::clone::Clone for WM_CLIENT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_CLIENT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_CLIENT_PROPERTIES").field("dwIPAddress", &self.dwIPAddress).field("dwPort", &self.dwPort).finish()
    }
}
impl ::windows::core::TypeKind for WM_CLIENT_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WM_CLIENT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPAddress == other.dwIPAddress && self.dwPort == other.dwPort
    }
}
impl ::core::cmp::Eq for WM_CLIENT_PROPERTIES {}
impl ::core::default::Default for WM_CLIENT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_CLIENT_PROPERTIES_EX {
    pub cbSize: u32,
    pub pwszIPAddress: ::windows::core::PCWSTR,
    pub pwszPort: ::windows::core::PCWSTR,
    pub pwszDNSName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WM_CLIENT_PROPERTIES_EX {}
impl ::core::clone::Clone for WM_CLIENT_PROPERTIES_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_CLIENT_PROPERTIES_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_CLIENT_PROPERTIES_EX").field("cbSize", &self.cbSize).field("pwszIPAddress", &self.pwszIPAddress).field("pwszPort", &self.pwszPort).field("pwszDNSName", &self.pwszDNSName).finish()
    }
}
impl ::windows::core::TypeKind for WM_CLIENT_PROPERTIES_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WM_CLIENT_PROPERTIES_EX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pwszIPAddress == other.pwszIPAddress && self.pwszPort == other.pwszPort && self.pwszDNSName == other.pwszDNSName
    }
}
impl ::core::cmp::Eq for WM_CLIENT_PROPERTIES_EX {}
impl ::core::default::Default for WM_CLIENT_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_LEAKY_BUCKET_PAIR {
    pub dwBitrate: u32,
    pub msBufferWindow: u32,
}
impl ::core::marker::Copy for WM_LEAKY_BUCKET_PAIR {}
impl ::core::clone::Clone for WM_LEAKY_BUCKET_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WM_LEAKY_BUCKET_PAIR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WM_LEAKY_BUCKET_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_MEDIA_TYPE {
    pub majortype: ::windows::core::GUID,
    pub subtype: ::windows::core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows::core::GUID,
    pub pUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_MEDIA_TYPE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WM_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_MEDIA_TYPE").field("majortype", &self.majortype).field("subtype", &self.subtype).field("bFixedSizeSamples", &self.bFixedSizeSamples).field("bTemporalCompression", &self.bTemporalCompression).field("lSampleSize", &self.lSampleSize).field("formattype", &self.formattype).field("pUnk", &self.pUnk).field("cbFormat", &self.cbFormat).field("pbFormat", &self.pbFormat).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WM_MEDIA_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_MEDIA_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype && self.pUnk == other.pUnk && self.cbFormat == other.cbFormat && self.pbFormat == other.pbFormat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_PICTURE {
    pub pwszMIMEType: ::windows::core::PWSTR,
    pub bPictureType: u8,
    pub pwszDescription: ::windows::core::PWSTR,
    pub dwDataLen: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for WM_PICTURE {}
impl ::core::clone::Clone for WM_PICTURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WM_PICTURE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WM_PICTURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_PORT_NUMBER_RANGE {
    pub wPortBegin: u16,
    pub wPortEnd: u16,
}
impl ::core::marker::Copy for WM_PORT_NUMBER_RANGE {}
impl ::core::clone::Clone for WM_PORT_NUMBER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_PORT_NUMBER_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_PORT_NUMBER_RANGE").field("wPortBegin", &self.wPortBegin).field("wPortEnd", &self.wPortEnd).finish()
    }
}
impl ::windows::core::TypeKind for WM_PORT_NUMBER_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WM_PORT_NUMBER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.wPortBegin == other.wPortBegin && self.wPortEnd == other.wPortEnd
    }
}
impl ::core::cmp::Eq for WM_PORT_NUMBER_RANGE {}
impl ::core::default::Default for WM_PORT_NUMBER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_READER_CLIENTINFO {
    pub cbSize: u32,
    pub wszLang: ::windows::core::PWSTR,
    pub wszBrowserUserAgent: ::windows::core::PWSTR,
    pub wszBrowserWebPage: ::windows::core::PWSTR,
    pub qwReserved: u64,
    pub pReserved: *mut super::super::Foundation::LPARAM,
    pub wszHostExe: ::windows::core::PWSTR,
    pub qwHostVersion: u64,
    pub wszPlayerUserAgent: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_READER_CLIENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_READER_CLIENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WM_READER_CLIENTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_READER_CLIENTINFO").field("cbSize", &self.cbSize).field("wszLang", &self.wszLang).field("wszBrowserUserAgent", &self.wszBrowserUserAgent).field("wszBrowserWebPage", &self.wszBrowserWebPage).field("qwReserved", &self.qwReserved).field("pReserved", &self.pReserved).field("wszHostExe", &self.wszHostExe).field("qwHostVersion", &self.qwHostVersion).field("wszPlayerUserAgent", &self.wszPlayerUserAgent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WM_READER_CLIENTINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_READER_CLIENTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wszLang == other.wszLang && self.wszBrowserUserAgent == other.wszBrowserUserAgent && self.wszBrowserWebPage == other.wszBrowserWebPage && self.qwReserved == other.qwReserved && self.pReserved == other.pReserved && self.wszHostExe == other.wszHostExe && self.qwHostVersion == other.qwHostVersion && self.wszPlayerUserAgent == other.wszPlayerUserAgent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_READER_CLIENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_READER_CLIENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_READER_STATISTICS {
    pub cbSize: u32,
    pub dwBandwidth: u32,
    pub cPacketsReceived: u32,
    pub cPacketsRecovered: u32,
    pub cPacketsLost: u32,
    pub wQuality: u16,
}
impl ::core::marker::Copy for WM_READER_STATISTICS {}
impl ::core::clone::Clone for WM_READER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_READER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_READER_STATISTICS").field("cbSize", &self.cbSize).field("dwBandwidth", &self.dwBandwidth).field("cPacketsReceived", &self.cPacketsReceived).field("cPacketsRecovered", &self.cPacketsRecovered).field("cPacketsLost", &self.cPacketsLost).field("wQuality", &self.wQuality).finish()
    }
}
impl ::windows::core::TypeKind for WM_READER_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WM_READER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwBandwidth == other.dwBandwidth && self.cPacketsReceived == other.cPacketsReceived && self.cPacketsRecovered == other.cPacketsRecovered && self.cPacketsLost == other.cPacketsLost && self.wQuality == other.wQuality
    }
}
impl ::core::cmp::Eq for WM_READER_STATISTICS {}
impl ::core::default::Default for WM_READER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_STREAM_PRIORITY_RECORD {
    pub wStreamNumber: u16,
    pub fMandatory: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_STREAM_PRIORITY_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_STREAM_PRIORITY_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WM_STREAM_PRIORITY_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_STREAM_PRIORITY_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_STREAM_TYPE_INFO {
    pub guidMajorType: ::windows::core::GUID,
    pub cbFormat: u32,
}
impl ::core::marker::Copy for WM_STREAM_TYPE_INFO {}
impl ::core::clone::Clone for WM_STREAM_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WM_STREAM_TYPE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WM_STREAM_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: u8,
    pub bContentType: u8,
    pub pwszContentDescriptor: ::windows::core::PWSTR,
    pub dwLyricsLen: u32,
    pub pbLyrics: *mut u8,
}
impl ::core::marker::Copy for WM_SYNCHRONISED_LYRICS {}
impl ::core::clone::Clone for WM_SYNCHRONISED_LYRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WM_SYNCHRONISED_LYRICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WM_SYNCHRONISED_LYRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_USER_TEXT {
    pub pwszDescription: ::windows::core::PWSTR,
    pub pwszText: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WM_USER_TEXT {}
impl ::core::clone::Clone for WM_USER_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WM_USER_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WM_USER_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_USER_WEB_URL {
    pub pwszDescription: ::windows::core::PWSTR,
    pub pwszURL: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WM_USER_WEB_URL {}
impl ::core::clone::Clone for WM_USER_WEB_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WM_USER_WEB_URL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WM_USER_WEB_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_WRITER_STATISTICS {
    pub qwSampleCount: u64,
    pub qwByteCount: u64,
    pub qwDroppedSampleCount: u64,
    pub qwDroppedByteCount: u64,
    pub dwCurrentBitrate: u32,
    pub dwAverageBitrate: u32,
    pub dwExpectedBitrate: u32,
    pub dwCurrentSampleRate: u32,
    pub dwAverageSampleRate: u32,
    pub dwExpectedSampleRate: u32,
}
impl ::core::marker::Copy for WM_WRITER_STATISTICS {}
impl ::core::clone::Clone for WM_WRITER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_WRITER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_WRITER_STATISTICS")
            .field("qwSampleCount", &self.qwSampleCount)
            .field("qwByteCount", &self.qwByteCount)
            .field("qwDroppedSampleCount", &self.qwDroppedSampleCount)
            .field("qwDroppedByteCount", &self.qwDroppedByteCount)
            .field("dwCurrentBitrate", &self.dwCurrentBitrate)
            .field("dwAverageBitrate", &self.dwAverageBitrate)
            .field("dwExpectedBitrate", &self.dwExpectedBitrate)
            .field("dwCurrentSampleRate", &self.dwCurrentSampleRate)
            .field("dwAverageSampleRate", &self.dwAverageSampleRate)
            .field("dwExpectedSampleRate", &self.dwExpectedSampleRate)
            .finish()
    }
}
impl ::windows::core::TypeKind for WM_WRITER_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WM_WRITER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.qwSampleCount == other.qwSampleCount && self.qwByteCount == other.qwByteCount && self.qwDroppedSampleCount == other.qwDroppedSampleCount && self.qwDroppedByteCount == other.qwDroppedByteCount && self.dwCurrentBitrate == other.dwCurrentBitrate && self.dwAverageBitrate == other.dwAverageBitrate && self.dwExpectedBitrate == other.dwExpectedBitrate && self.dwCurrentSampleRate == other.dwCurrentSampleRate && self.dwAverageSampleRate == other.dwAverageSampleRate && self.dwExpectedSampleRate == other.dwExpectedSampleRate
    }
}
impl ::core::cmp::Eq for WM_WRITER_STATISTICS {}
impl ::core::default::Default for WM_WRITER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_WRITER_STATISTICS_EX {
    pub dwBitratePlusOverhead: u32,
    pub dwCurrentSampleDropRateInQueue: u32,
    pub dwCurrentSampleDropRateInCodec: u32,
    pub dwCurrentSampleDropRateInMultiplexer: u32,
    pub dwTotalSampleDropsInQueue: u32,
    pub dwTotalSampleDropsInCodec: u32,
    pub dwTotalSampleDropsInMultiplexer: u32,
}
impl ::core::marker::Copy for WM_WRITER_STATISTICS_EX {}
impl ::core::clone::Clone for WM_WRITER_STATISTICS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_WRITER_STATISTICS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_WRITER_STATISTICS_EX")
            .field("dwBitratePlusOverhead", &self.dwBitratePlusOverhead)
            .field("dwCurrentSampleDropRateInQueue", &self.dwCurrentSampleDropRateInQueue)
            .field("dwCurrentSampleDropRateInCodec", &self.dwCurrentSampleDropRateInCodec)
            .field("dwCurrentSampleDropRateInMultiplexer", &self.dwCurrentSampleDropRateInMultiplexer)
            .field("dwTotalSampleDropsInQueue", &self.dwTotalSampleDropsInQueue)
            .field("dwTotalSampleDropsInCodec", &self.dwTotalSampleDropsInCodec)
            .field("dwTotalSampleDropsInMultiplexer", &self.dwTotalSampleDropsInMultiplexer)
            .finish()
    }
}
impl ::windows::core::TypeKind for WM_WRITER_STATISTICS_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WM_WRITER_STATISTICS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.dwBitratePlusOverhead == other.dwBitratePlusOverhead && self.dwCurrentSampleDropRateInQueue == other.dwCurrentSampleDropRateInQueue && self.dwCurrentSampleDropRateInCodec == other.dwCurrentSampleDropRateInCodec && self.dwCurrentSampleDropRateInMultiplexer == other.dwCurrentSampleDropRateInMultiplexer && self.dwTotalSampleDropsInQueue == other.dwTotalSampleDropsInQueue && self.dwTotalSampleDropsInCodec == other.dwTotalSampleDropsInCodec && self.dwTotalSampleDropsInMultiplexer == other.dwTotalSampleDropsInMultiplexer
    }
}
impl ::core::cmp::Eq for WM_WRITER_STATISTICS_EX {}
impl ::core::default::Default for WM_WRITER_STATISTICS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
