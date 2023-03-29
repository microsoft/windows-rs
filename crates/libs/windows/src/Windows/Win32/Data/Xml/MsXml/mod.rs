#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMXAttributes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMXAttributes {
    pub unsafe fn addAttribute<P0, P1, P2, P3, P4>(&self, struri: P0, strlocalname: P1, strqname: P2, strtype: P3, strvalue: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).addAttribute)(::windows::core::Interface::as_raw(self), struri.into_param().abi(), strlocalname.into_param().abi(), strqname.into_param().abi(), strtype.into_param().abi(), strvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn addAttributeFromIndex(&self, varatts: super::super::super::System::Com::VARIANT, nindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).addAttributeFromIndex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varatts), nindex).ok()
    }
    pub unsafe fn clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).clear)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn removeAttribute(&self, nindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).removeAttribute)(::windows::core::Interface::as_raw(self), nindex).ok()
    }
    pub unsafe fn setAttribute<P0, P1, P2, P3, P4>(&self, nindex: i32, struri: P0, strlocalname: P1, strqname: P2, strtype: P3, strvalue: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setAttribute)(::windows::core::Interface::as_raw(self), nindex, struri.into_param().abi(), strlocalname.into_param().abi(), strqname.into_param().abi(), strtype.into_param().abi(), strvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setAttributes(&self, varatts: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).setAttributes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varatts)).ok()
    }
    pub unsafe fn setLocalName<P0>(&self, nindex: i32, strlocalname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setLocalName)(::windows::core::Interface::as_raw(self), nindex, strlocalname.into_param().abi()).ok()
    }
    pub unsafe fn setQName<P0>(&self, nindex: i32, strqname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setQName)(::windows::core::Interface::as_raw(self), nindex, strqname.into_param().abi()).ok()
    }
    pub unsafe fn setType<P0>(&self, nindex: i32, strtype: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setType)(::windows::core::Interface::as_raw(self), nindex, strtype.into_param().abi()).ok()
    }
    pub unsafe fn setURI<P0>(&self, nindex: i32, struri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setURI)(::windows::core::Interface::as_raw(self), nindex, struri.into_param().abi()).ok()
    }
    pub unsafe fn setValue<P0>(&self, nindex: i32, strvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setValue)(::windows::core::Interface::as_raw(self), nindex, strvalue.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IMXAttributes, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMXAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMXAttributes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMXAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMXAttributes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMXAttributes {
    type Vtable = IMXAttributes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMXAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IMXAttributes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf10d27cc_3ec0_415c_8ed8_77ab1c5e7262);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXAttributes_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub addAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub addAttributeFromIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varatts: super::super::super::System::Com::VARIANT, nindex: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    addAttributeFromIndex: usize,
    pub clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub removeAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT,
    pub setAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varatts: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setAttributes: usize,
    pub setLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub setQName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub setType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, strtype: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub setURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub setValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct IMXNamespaceManager(::windows::core::IUnknown);
impl IMXNamespaceManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn putAllowOverride<P0>(&self, foverride: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).putAllowOverride)(::windows::core::Interface::as_raw(self), foverride.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getAllowOverride(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).getAllowOverride)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn pushContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).pushContext)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn pushNodeContext<P0, P1>(&self, contextnode: P0, fdeep: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).pushNodeContext)(::windows::core::Interface::as_raw(self), contextnode.into_param().abi(), fdeep.into_param().abi()).ok()
    }
    pub unsafe fn popContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).popContext)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn declarePrefix<P0, P1>(&self, prefix: P0, namespaceuri: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).declarePrefix)(::windows::core::Interface::as_raw(self), prefix.into_param().abi(), namespaceuri.into_param().abi()).ok()
    }
    pub unsafe fn getDeclaredPrefix(&self, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).getDeclaredPrefix)(::windows::core::Interface::as_raw(self), nindex, ::core::mem::transmute(pwchprefix), pcchprefix).ok()
    }
    pub unsafe fn getPrefix<P0>(&self, pwsznamespaceuri: P0, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).getPrefix)(::windows::core::Interface::as_raw(self), pwsznamespaceuri.into_param().abi(), nindex, ::core::mem::transmute(pwchprefix), pcchprefix).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getURI<P0, P1>(&self, pwchprefix: P0, pcontextnode: P1, pwchuri: ::windows::core::PWSTR, pcchuri: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).getURI)(::windows::core::Interface::as_raw(self), pwchprefix.into_param().abi(), pcontextnode.into_param().abi(), ::core::mem::transmute(pwchuri), pcchuri).ok()
    }
}
::windows::imp::interface_hierarchy!(IMXNamespaceManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMXNamespaceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMXNamespaceManager {}
impl ::core::fmt::Debug for IMXNamespaceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMXNamespaceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMXNamespaceManager {
    type Vtable = IMXNamespaceManager_Vtbl;
}
impl ::core::clone::Clone for IMXNamespaceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMXNamespaceManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc90352f6_643c_4fbc_bb23_e996eb2d51fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMXNamespaceManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub putAllowOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    putAllowOverride: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub getAllowOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foverride: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getAllowOverride: usize,
    pub reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub pushContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub pushNodeContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contextnode: *mut ::core::ffi::c_void, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    pushNodeContext: usize,
    pub popContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub declarePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefix: ::windows::core::PCWSTR, namespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub getDeclaredPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT,
    pub getPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznamespaceuri: ::windows::core::PCWSTR, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub getURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchprefix: ::windows::core::PCWSTR, pcontextnode: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PWSTR, pcchuri: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getURI: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMXNamespacePrefixes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMXNamespacePrefixes {
    pub unsafe fn get_item(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._newEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IMXNamespacePrefixes, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMXNamespacePrefixes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMXNamespacePrefixes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMXNamespacePrefixes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMXNamespacePrefixes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMXNamespacePrefixes {
    type Vtable = IMXNamespacePrefixes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMXNamespacePrefixes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IMXNamespacePrefixes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc90352f4_643c_4fbc_bb23_e996eb2d51fd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXNamespacePrefixes_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub get_item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, prefix: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMXReaderControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMXReaderControl {
    pub unsafe fn abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).suspend)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IMXReaderControl, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMXReaderControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMXReaderControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMXReaderControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMXReaderControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMXReaderControl {
    type Vtable = IMXReaderControl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMXReaderControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IMXReaderControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x808f4e35_8d5a_4fbe_8466_33a41279ed30);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXReaderControl_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub suspend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMXSchemaDeclHandler(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMXSchemaDeclHandler {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schemaElementDecl<P0>(&self, oschemaelement: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISchemaElement>,
    {
        (::windows::core::Interface::vtable(self).schemaElementDecl)(::windows::core::Interface::as_raw(self), oschemaelement.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IMXSchemaDeclHandler, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMXSchemaDeclHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMXSchemaDeclHandler {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMXSchemaDeclHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMXSchemaDeclHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMXSchemaDeclHandler {
    type Vtable = IMXSchemaDeclHandler_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMXSchemaDeclHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IMXSchemaDeclHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa4bb38c_faf9_4cca_9302_d1dd0fe520db);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXSchemaDeclHandler_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub schemaElementDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oschemaelement: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    schemaElementDecl: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMXWriter(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMXWriter {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setoutput(&self, vardestination: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Setoutput)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(vardestination)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn output(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).output)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Setencoding<P0>(&self, strencoding: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Setencoding)(::windows::core::Interface::as_raw(self), strencoding.into_param().abi()).ok()
    }
    pub unsafe fn encoding(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).encoding)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetbyteOrderMark<P0>(&self, fwritebyteordermark: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetbyteOrderMark)(::windows::core::Interface::as_raw(self), fwritebyteordermark.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn byteOrderMark(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).byteOrderMark)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setindent<P0>(&self, findentmode: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).Setindent)(::windows::core::Interface::as_raw(self), findentmode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn indent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).indent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setstandalone<P0>(&self, fvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).Setstandalone)(::windows::core::Interface::as_raw(self), fvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn standalone(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).standalone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetomitXMLDeclaration<P0>(&self, fvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetomitXMLDeclaration)(::windows::core::Interface::as_raw(self), fvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn omitXMLDeclaration(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).omitXMLDeclaration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Setversion<P0>(&self, strversion: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Setversion)(::windows::core::Interface::as_raw(self), strversion.into_param().abi()).ok()
    }
    pub unsafe fn version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).version)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetdisableOutputEscaping<P0>(&self, fvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetdisableOutputEscaping)(::windows::core::Interface::as_raw(self), fvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn disableOutputEscaping(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).disableOutputEscaping)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).flush)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IMXWriter, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMXWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMXWriter {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMXWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMXWriter").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMXWriter {
    type Vtable = IMXWriter_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMXWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IMXWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d7ff4ba_1565_4ea8_94e1_6e724a46f98d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXWriter_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Setoutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardestination: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Setoutput: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub output: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardestination: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    output: usize,
    pub Setencoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strencoding: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub encoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strencoding: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetbyteOrderMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fwritebyteordermark: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetbyteOrderMark: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub byteOrderMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fwritebyteordermark: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    byteOrderMark: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Setindent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findentmode: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Setindent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub indent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findentmode: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    indent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Setstandalone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Setstandalone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub standalone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    standalone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetomitXMLDeclaration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetomitXMLDeclaration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub omitXMLDeclaration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    omitXMLDeclaration: usize,
    pub Setversion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strversion: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strversion: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetdisableOutputEscaping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetdisableOutputEscaping: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub disableOutputEscaping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    disableOutputEscaping: usize,
    pub flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMXXMLFilter(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMXXMLFilter {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getFeature<P0>(&self, strname: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).getFeature)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn putFeature<P0, P1>(&self, strname: P0, fvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).putFeature)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), fvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getProperty<P0>(&self, strname: P0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getProperty)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putProperty<P0>(&self, strname: P0, varvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).putProperty)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn entityResolver(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).entityResolver)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putref_entityResolver<P0>(&self, oresolver: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).putref_entityResolver)(::windows::core::Interface::as_raw(self), oresolver.into_param().abi()).ok()
    }
    pub unsafe fn contentHandler(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).contentHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putref_contentHandler<P0>(&self, ohandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).putref_contentHandler)(::windows::core::Interface::as_raw(self), ohandler.into_param().abi()).ok()
    }
    pub unsafe fn dtdHandler(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).dtdHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putref_dtdHandler<P0>(&self, ohandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).putref_dtdHandler)(::windows::core::Interface::as_raw(self), ohandler.into_param().abi()).ok()
    }
    pub unsafe fn errorHandler(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).errorHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putref_errorHandler<P0>(&self, ohandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).putref_errorHandler)(::windows::core::Interface::as_raw(self), ohandler.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IMXXMLFilter, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMXXMLFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMXXMLFilter {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMXXMLFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMXXMLFilter").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMXXMLFilter {
    type Vtable = IMXXMLFilter_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMXXMLFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IMXXMLFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc90352f7_643c_4fbc_bb23_e996eb2d51fd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXXMLFilter_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub getFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getFeature: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub putFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    putFeature: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putProperty: usize,
    pub entityResolver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub putref_entityResolver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub contentHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub putref_contentHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub dtdHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub putref_dtdHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub errorHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub putref_errorHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct ISAXAttributes(::windows::core::IUnknown);
impl ISAXAttributes {
    pub unsafe fn getLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).getLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn getURI(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).getURI)(::windows::core::Interface::as_raw(self), nindex, ppwchuri, pcchuri).ok()
    }
    pub unsafe fn getLocalName(&self, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).getLocalName)(::windows::core::Interface::as_raw(self), nindex, ppwchlocalname, pcchlocalname).ok()
    }
    pub unsafe fn getQName(&self, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).getQName)(::windows::core::Interface::as_raw(self), nindex, ppwchqname, pcchqname).ok()
    }
    pub unsafe fn getName(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).getName)(::windows::core::Interface::as_raw(self), nindex, ppwchuri, pcchuri, ppwchlocalname, pcchlocalname, ppwchqname, pcchqname).ok()
    }
    pub unsafe fn getIndexFromName<P0, P1>(&self, pwchuri: P0, cchuri: i32, pwchlocalname: P1, cchlocalname: i32) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).getIndexFromName)(::windows::core::Interface::as_raw(self), pwchuri.into_param().abi(), cchuri, pwchlocalname.into_param().abi(), cchlocalname, &mut result__).from_abi(result__)
    }
    pub unsafe fn getIndexFromQName<P0>(&self, pwchqname: P0, cchqname: i32) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).getIndexFromQName)(::windows::core::Interface::as_raw(self), pwchqname.into_param().abi(), cchqname, &mut result__).from_abi(result__)
    }
    pub unsafe fn getType(&self, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).getType)(::windows::core::Interface::as_raw(self), nindex, ppwchtype, pcchtype).ok()
    }
    pub unsafe fn getTypeFromName<P0, P1>(&self, pwchuri: P0, cchuri: i32, pwchlocalname: P1, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).getTypeFromName)(::windows::core::Interface::as_raw(self), pwchuri.into_param().abi(), cchuri, pwchlocalname.into_param().abi(), cchlocalname, ppwchtype, pcchtype).ok()
    }
    pub unsafe fn getTypeFromQName<P0>(&self, pwchqname: P0, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).getTypeFromQName)(::windows::core::Interface::as_raw(self), pwchqname.into_param().abi(), cchqname, ppwchtype, pcchtype).ok()
    }
    pub unsafe fn getValue(&self, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).getValue)(::windows::core::Interface::as_raw(self), nindex, ppwchvalue, pcchvalue).ok()
    }
    pub unsafe fn getValueFromName<P0, P1>(&self, pwchuri: P0, cchuri: i32, pwchlocalname: P1, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).getValueFromName)(::windows::core::Interface::as_raw(self), pwchuri.into_param().abi(), cchuri, pwchlocalname.into_param().abi(), cchlocalname, ppwchvalue, pcchvalue).ok()
    }
    pub unsafe fn getValueFromQName<P0>(&self, pwchqname: P0, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).getValueFromQName)(::windows::core::Interface::as_raw(self), pwchqname.into_param().abi(), cchqname, ppwchvalue, pcchvalue).ok()
    }
}
::windows::imp::interface_hierarchy!(ISAXAttributes, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISAXAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISAXAttributes {}
impl ::core::fmt::Debug for ISAXAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISAXAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISAXAttributes {
    type Vtable = ISAXAttributes_Vtbl;
}
impl ::core::clone::Clone for ISAXAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISAXAttributes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf078abe1_45d2_4832_91ea_4466ce2f25c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXAttributes_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub getLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlength: *mut i32) -> ::windows::core::HRESULT,
    pub getURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows::core::HRESULT,
    pub getLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows::core::HRESULT,
    pub getQName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT,
    pub getName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT,
    pub getIndexFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PCWSTR, cchuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT,
    pub getIndexFromQName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchqname: ::windows::core::PCWSTR, cchqname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT,
    pub getType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT,
    pub getTypeFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PCWSTR, cchuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT,
    pub getTypeFromQName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchqname: ::windows::core::PCWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT,
    pub getValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT,
    pub getValueFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PCWSTR, cchuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT,
    pub getValueFromQName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchqname: ::windows::core::PCWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct ISAXContentHandler(::windows::core::IUnknown);
impl ISAXContentHandler {
    pub unsafe fn putDocumentLocator<P0>(&self, plocator: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXLocator>,
    {
        (::windows::core::Interface::vtable(self).putDocumentLocator)(::windows::core::Interface::as_raw(self), plocator.into_param().abi()).ok()
    }
    pub unsafe fn startDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).startDocument)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn endDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).endDocument)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn startPrefixMapping<P0, P1>(&self, pwchprefix: P0, cchprefix: i32, pwchuri: P1, cchuri: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).startPrefixMapping)(::windows::core::Interface::as_raw(self), pwchprefix.into_param().abi(), cchprefix, pwchuri.into_param().abi(), cchuri).ok()
    }
    pub unsafe fn endPrefixMapping<P0>(&self, pwchprefix: P0, cchprefix: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).endPrefixMapping)(::windows::core::Interface::as_raw(self), pwchprefix.into_param().abi(), cchprefix).ok()
    }
    pub unsafe fn startElement<P0, P1, P2, P3>(&self, pwchnamespaceuri: P0, cchnamespaceuri: i32, pwchlocalname: P1, cchlocalname: i32, pwchqname: P2, cchqname: i32, pattributes: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<ISAXAttributes>,
    {
        (::windows::core::Interface::vtable(self).startElement)(::windows::core::Interface::as_raw(self), pwchnamespaceuri.into_param().abi(), cchnamespaceuri, pwchlocalname.into_param().abi(), cchlocalname, pwchqname.into_param().abi(), cchqname, pattributes.into_param().abi()).ok()
    }
    pub unsafe fn endElement<P0, P1, P2>(&self, pwchnamespaceuri: P0, cchnamespaceuri: i32, pwchlocalname: P1, cchlocalname: i32, pwchqname: P2, cchqname: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).endElement)(::windows::core::Interface::as_raw(self), pwchnamespaceuri.into_param().abi(), cchnamespaceuri, pwchlocalname.into_param().abi(), cchlocalname, pwchqname.into_param().abi(), cchqname).ok()
    }
    pub unsafe fn characters<P0>(&self, pwchchars: P0, cchchars: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).characters)(::windows::core::Interface::as_raw(self), pwchchars.into_param().abi(), cchchars).ok()
    }
    pub unsafe fn ignorableWhitespace<P0>(&self, pwchchars: P0, cchchars: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ignorableWhitespace)(::windows::core::Interface::as_raw(self), pwchchars.into_param().abi(), cchchars).ok()
    }
    pub unsafe fn processingInstruction<P0, P1>(&self, pwchtarget: P0, cchtarget: i32, pwchdata: P1, cchdata: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).processingInstruction)(::windows::core::Interface::as_raw(self), pwchtarget.into_param().abi(), cchtarget, pwchdata.into_param().abi(), cchdata).ok()
    }
    pub unsafe fn skippedEntity<P0>(&self, pwchname: P0, cchname: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).skippedEntity)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), cchname).ok()
    }
}
::windows::imp::interface_hierarchy!(ISAXContentHandler, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISAXContentHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISAXContentHandler {}
impl ::core::fmt::Debug for ISAXContentHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISAXContentHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISAXContentHandler {
    type Vtable = ISAXContentHandler_Vtbl;
}
impl ::core::clone::Clone for ISAXContentHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISAXContentHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1545cdfa_9e4e_4497_a8a4_2bf7d0112c44);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXContentHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub putDocumentLocator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub startDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub endDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub startPrefixMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchprefix: ::windows::core::PCWSTR, cchprefix: i32, pwchuri: ::windows::core::PCWSTR, cchuri: i32) -> ::windows::core::HRESULT,
    pub endPrefixMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchprefix: ::windows::core::PCWSTR, cchprefix: i32) -> ::windows::core::HRESULT,
    pub startElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchnamespaceuri: ::windows::core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, pwchqname: ::windows::core::PCWSTR, cchqname: i32, pattributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub endElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchnamespaceuri: ::windows::core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, pwchqname: ::windows::core::PCWSTR, cchqname: i32) -> ::windows::core::HRESULT,
    pub characters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchchars: ::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::HRESULT,
    pub ignorableWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchchars: ::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::HRESULT,
    pub processingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchtarget: ::windows::core::PCWSTR, cchtarget: i32, pwchdata: ::windows::core::PCWSTR, cchdata: i32) -> ::windows::core::HRESULT,
    pub skippedEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct ISAXDTDHandler(::windows::core::IUnknown);
impl ISAXDTDHandler {
    pub unsafe fn notationDecl<P0, P1, P2>(&self, pwchname: P0, cchname: i32, pwchpublicid: P1, cchpublicid: i32, pwchsystemid: P2, cchsystemid: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).notationDecl)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), cchname, pwchpublicid.into_param().abi(), cchpublicid, pwchsystemid.into_param().abi(), cchsystemid).ok()
    }
    pub unsafe fn unparsedEntityDecl<P0, P1, P2, P3>(&self, pwchname: P0, cchname: i32, pwchpublicid: P1, cchpublicid: i32, pwchsystemid: P2, cchsystemid: i32, pwchnotationname: P3, cchnotationname: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).unparsedEntityDecl)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), cchname, pwchpublicid.into_param().abi(), cchpublicid, pwchsystemid.into_param().abi(), cchsystemid, pwchnotationname.into_param().abi(), cchnotationname).ok()
    }
}
::windows::imp::interface_hierarchy!(ISAXDTDHandler, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISAXDTDHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISAXDTDHandler {}
impl ::core::fmt::Debug for ISAXDTDHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISAXDTDHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISAXDTDHandler {
    type Vtable = ISAXDTDHandler_Vtbl;
}
impl ::core::clone::Clone for ISAXDTDHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISAXDTDHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe15c1baf_afb3_4d60_8c36_19a8c45defed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXDTDHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub notationDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::HRESULT,
    pub unparsedEntityDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32, pwchnotationname: ::windows::core::PCWSTR, cchnotationname: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct ISAXDeclHandler(::windows::core::IUnknown);
impl ISAXDeclHandler {
    pub unsafe fn elementDecl<P0, P1>(&self, pwchname: P0, cchname: i32, pwchmodel: P1, cchmodel: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).elementDecl)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), cchname, pwchmodel.into_param().abi(), cchmodel).ok()
    }
    pub unsafe fn attributeDecl<P0, P1, P2, P3, P4>(&self, pwchelementname: P0, cchelementname: i32, pwchattributename: P1, cchattributename: i32, pwchtype: P2, cchtype: i32, pwchvaluedefault: P3, cchvaluedefault: i32, pwchvalue: P4, cchvalue: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).attributeDecl)(::windows::core::Interface::as_raw(self), pwchelementname.into_param().abi(), cchelementname, pwchattributename.into_param().abi(), cchattributename, pwchtype.into_param().abi(), cchtype, pwchvaluedefault.into_param().abi(), cchvaluedefault, pwchvalue.into_param().abi(), cchvalue).ok()
    }
    pub unsafe fn internalEntityDecl<P0, P1>(&self, pwchname: P0, cchname: i32, pwchvalue: P1, cchvalue: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).internalEntityDecl)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), cchname, pwchvalue.into_param().abi(), cchvalue).ok()
    }
    pub unsafe fn externalEntityDecl<P0, P1, P2>(&self, pwchname: P0, cchname: i32, pwchpublicid: P1, cchpublicid: i32, pwchsystemid: P2, cchsystemid: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).externalEntityDecl)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), cchname, pwchpublicid.into_param().abi(), cchpublicid, pwchsystemid.into_param().abi(), cchsystemid).ok()
    }
}
::windows::imp::interface_hierarchy!(ISAXDeclHandler, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISAXDeclHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISAXDeclHandler {}
impl ::core::fmt::Debug for ISAXDeclHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISAXDeclHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISAXDeclHandler {
    type Vtable = ISAXDeclHandler_Vtbl;
}
impl ::core::clone::Clone for ISAXDeclHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISAXDeclHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x862629ac_771a_47b2_8337_4e6843c1be90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXDeclHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub elementDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchmodel: ::windows::core::PCWSTR, cchmodel: i32) -> ::windows::core::HRESULT,
    pub attributeDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchelementname: ::windows::core::PCWSTR, cchelementname: i32, pwchattributename: ::windows::core::PCWSTR, cchattributename: i32, pwchtype: ::windows::core::PCWSTR, cchtype: i32, pwchvaluedefault: ::windows::core::PCWSTR, cchvaluedefault: i32, pwchvalue: ::windows::core::PCWSTR, cchvalue: i32) -> ::windows::core::HRESULT,
    pub internalEntityDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchvalue: ::windows::core::PCWSTR, cchvalue: i32) -> ::windows::core::HRESULT,
    pub externalEntityDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct ISAXEntityResolver(::windows::core::IUnknown);
impl ISAXEntityResolver {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn resolveEntity<P0, P1>(&self, pwchpublicid: P0, pwchsystemid: P1) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).resolveEntity)(::windows::core::Interface::as_raw(self), pwchpublicid.into_param().abi(), pwchsystemid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISAXEntityResolver, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISAXEntityResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISAXEntityResolver {}
impl ::core::fmt::Debug for ISAXEntityResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISAXEntityResolver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISAXEntityResolver {
    type Vtable = ISAXEntityResolver_Vtbl;
}
impl ::core::clone::Clone for ISAXEntityResolver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISAXEntityResolver {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99bca7bd_e8c4_4d5f_a0cf_6d907901ff07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXEntityResolver_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub resolveEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchpublicid: ::windows::core::PCWSTR, pwchsystemid: ::windows::core::PCWSTR, pvarinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    resolveEntity: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct ISAXErrorHandler(::windows::core::IUnknown);
impl ISAXErrorHandler {
    pub unsafe fn error<P0, P1>(&self, plocator: P0, pwcherrormessage: P1, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXLocator>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).error)(::windows::core::Interface::as_raw(self), plocator.into_param().abi(), pwcherrormessage.into_param().abi(), hrerrorcode).ok()
    }
    pub unsafe fn fatalError<P0, P1>(&self, plocator: P0, pwcherrormessage: P1, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXLocator>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).fatalError)(::windows::core::Interface::as_raw(self), plocator.into_param().abi(), pwcherrormessage.into_param().abi(), hrerrorcode).ok()
    }
    pub unsafe fn ignorableWarning<P0, P1>(&self, plocator: P0, pwcherrormessage: P1, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXLocator>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ignorableWarning)(::windows::core::Interface::as_raw(self), plocator.into_param().abi(), pwcherrormessage.into_param().abi(), hrerrorcode).ok()
    }
}
::windows::imp::interface_hierarchy!(ISAXErrorHandler, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISAXErrorHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISAXErrorHandler {}
impl ::core::fmt::Debug for ISAXErrorHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISAXErrorHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISAXErrorHandler {
    type Vtable = ISAXErrorHandler_Vtbl;
}
impl ::core::clone::Clone for ISAXErrorHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISAXErrorHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa60511c4_ccf5_479e_98a3_dc8dc545b7d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXErrorHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub fatalError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ignorableWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct ISAXLexicalHandler(::windows::core::IUnknown);
impl ISAXLexicalHandler {
    pub unsafe fn startDTD<P0, P1, P2>(&self, pwchname: P0, cchname: i32, pwchpublicid: P1, cchpublicid: i32, pwchsystemid: P2, cchsystemid: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).startDTD)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), cchname, pwchpublicid.into_param().abi(), cchpublicid, pwchsystemid.into_param().abi(), cchsystemid).ok()
    }
    pub unsafe fn endDTD(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).endDTD)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn startEntity<P0>(&self, pwchname: P0, cchname: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).startEntity)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), cchname).ok()
    }
    pub unsafe fn endEntity<P0>(&self, pwchname: P0, cchname: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).endEntity)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), cchname).ok()
    }
    pub unsafe fn startCDATA(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).startCDATA)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn endCDATA(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).endCDATA)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn comment<P0>(&self, pwchchars: P0, cchchars: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).comment)(::windows::core::Interface::as_raw(self), pwchchars.into_param().abi(), cchchars).ok()
    }
}
::windows::imp::interface_hierarchy!(ISAXLexicalHandler, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISAXLexicalHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISAXLexicalHandler {}
impl ::core::fmt::Debug for ISAXLexicalHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISAXLexicalHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISAXLexicalHandler {
    type Vtable = ISAXLexicalHandler_Vtbl;
}
impl ::core::clone::Clone for ISAXLexicalHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISAXLexicalHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f85d5f5_47a8_4497_bda5_84ba04819ea6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXLexicalHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub startDTD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::HRESULT,
    pub endDTD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub startEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32) -> ::windows::core::HRESULT,
    pub endEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32) -> ::windows::core::HRESULT,
    pub startCDATA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub endCDATA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchchars: ::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct ISAXLocator(::windows::core::IUnknown);
impl ISAXLocator {
    pub unsafe fn getColumnNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).getColumnNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn getLineNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).getLineNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn getPublicId(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__ = ::windows::core::zeroed::<*mut u16>();
        (::windows::core::Interface::vtable(self).getPublicId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn getSystemId(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__ = ::windows::core::zeroed::<*mut u16>();
        (::windows::core::Interface::vtable(self).getSystemId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISAXLocator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISAXLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISAXLocator {}
impl ::core::fmt::Debug for ISAXLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISAXLocator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISAXLocator {
    type Vtable = ISAXLocator_Vtbl;
}
impl ::core::clone::Clone for ISAXLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISAXLocator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e472a_0de4_4640_bff3_84d38a051c31);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXLocator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub getColumnNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncolumn: *mut i32) -> ::windows::core::HRESULT,
    pub getLineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnline: *mut i32) -> ::windows::core::HRESULT,
    pub getPublicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwchpublicid: *mut *mut u16) -> ::windows::core::HRESULT,
    pub getSystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwchsystemid: *mut *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct ISAXXMLFilter(::windows::core::IUnknown);
impl ISAXXMLFilter {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getFeature<P0>(&self, pwchname: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.getFeature)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn putFeature<P0, P1>(&self, pwchname: P0, vfvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.putFeature)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), vfvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getProperty<P0>(&self, pwchname: P0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.getProperty)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putProperty<P0>(&self, pwchname: P0, varvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.putProperty)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), ::core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn getEntityResolver(&self) -> ::windows::core::Result<ISAXEntityResolver> {
        let mut result__ = ::windows::core::zeroed::<ISAXEntityResolver>();
        (::windows::core::Interface::vtable(self).base__.getEntityResolver)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putEntityResolver<P0>(&self, presolver: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXEntityResolver>,
    {
        (::windows::core::Interface::vtable(self).base__.putEntityResolver)(::windows::core::Interface::as_raw(self), presolver.into_param().abi()).ok()
    }
    pub unsafe fn getContentHandler(&self) -> ::windows::core::Result<ISAXContentHandler> {
        let mut result__ = ::windows::core::zeroed::<ISAXContentHandler>();
        (::windows::core::Interface::vtable(self).base__.getContentHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putContentHandler<P0>(&self, phandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXContentHandler>,
    {
        (::windows::core::Interface::vtable(self).base__.putContentHandler)(::windows::core::Interface::as_raw(self), phandler.into_param().abi()).ok()
    }
    pub unsafe fn getDTDHandler(&self) -> ::windows::core::Result<ISAXDTDHandler> {
        let mut result__ = ::windows::core::zeroed::<ISAXDTDHandler>();
        (::windows::core::Interface::vtable(self).base__.getDTDHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putDTDHandler<P0>(&self, phandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXDTDHandler>,
    {
        (::windows::core::Interface::vtable(self).base__.putDTDHandler)(::windows::core::Interface::as_raw(self), phandler.into_param().abi()).ok()
    }
    pub unsafe fn getErrorHandler(&self) -> ::windows::core::Result<ISAXErrorHandler> {
        let mut result__ = ::windows::core::zeroed::<ISAXErrorHandler>();
        (::windows::core::Interface::vtable(self).base__.getErrorHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putErrorHandler<P0>(&self, phandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXErrorHandler>,
    {
        (::windows::core::Interface::vtable(self).base__.putErrorHandler)(::windows::core::Interface::as_raw(self), phandler.into_param().abi()).ok()
    }
    pub unsafe fn getBaseURL(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__ = ::windows::core::zeroed::<*mut u16>();
        (::windows::core::Interface::vtable(self).base__.getBaseURL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putBaseURL<P0>(&self, pwchbaseurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.putBaseURL)(::windows::core::Interface::as_raw(self), pwchbaseurl.into_param().abi()).ok()
    }
    pub unsafe fn getSecureBaseURL(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__ = ::windows::core::zeroed::<*mut u16>();
        (::windows::core::Interface::vtable(self).base__.getSecureBaseURL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putSecureBaseURL<P0>(&self, pwchsecurebaseurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.putSecureBaseURL)(::windows::core::Interface::as_raw(self), pwchsecurebaseurl.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn parse(&self, varinput: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parse)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varinput)).ok()
    }
    pub unsafe fn parseURL<P0>(&self, pwchurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.parseURL)(::windows::core::Interface::as_raw(self), pwchurl.into_param().abi()).ok()
    }
    pub unsafe fn getParent(&self) -> ::windows::core::Result<ISAXXMLReader> {
        let mut result__ = ::windows::core::zeroed::<ISAXXMLReader>();
        (::windows::core::Interface::vtable(self).getParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putParent<P0>(&self, preader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXXMLReader>,
    {
        (::windows::core::Interface::vtable(self).putParent)(::windows::core::Interface::as_raw(self), preader.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ISAXXMLFilter, ::windows::core::IUnknown, ISAXXMLReader);
impl ::core::cmp::PartialEq for ISAXXMLFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISAXXMLFilter {}
impl ::core::fmt::Debug for ISAXXMLFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISAXXMLFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISAXXMLFilter {
    type Vtable = ISAXXMLFilter_Vtbl;
}
impl ::core::clone::Clone for ISAXXMLFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISAXXMLFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70409222_ca09_4475_acb8_40312fe8d145);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXXMLFilter_Vtbl {
    pub base__: ISAXXMLReader_Vtbl,
    pub getParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub putParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct ISAXXMLReader(::windows::core::IUnknown);
impl ISAXXMLReader {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getFeature<P0>(&self, pwchname: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).getFeature)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn putFeature<P0, P1>(&self, pwchname: P0, vfvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).putFeature)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), vfvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getProperty<P0>(&self, pwchname: P0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getProperty)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putProperty<P0>(&self, pwchname: P0, varvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).putProperty)(::windows::core::Interface::as_raw(self), pwchname.into_param().abi(), ::core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn getEntityResolver(&self) -> ::windows::core::Result<ISAXEntityResolver> {
        let mut result__ = ::windows::core::zeroed::<ISAXEntityResolver>();
        (::windows::core::Interface::vtable(self).getEntityResolver)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putEntityResolver<P0>(&self, presolver: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXEntityResolver>,
    {
        (::windows::core::Interface::vtable(self).putEntityResolver)(::windows::core::Interface::as_raw(self), presolver.into_param().abi()).ok()
    }
    pub unsafe fn getContentHandler(&self) -> ::windows::core::Result<ISAXContentHandler> {
        let mut result__ = ::windows::core::zeroed::<ISAXContentHandler>();
        (::windows::core::Interface::vtable(self).getContentHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putContentHandler<P0>(&self, phandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXContentHandler>,
    {
        (::windows::core::Interface::vtable(self).putContentHandler)(::windows::core::Interface::as_raw(self), phandler.into_param().abi()).ok()
    }
    pub unsafe fn getDTDHandler(&self) -> ::windows::core::Result<ISAXDTDHandler> {
        let mut result__ = ::windows::core::zeroed::<ISAXDTDHandler>();
        (::windows::core::Interface::vtable(self).getDTDHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putDTDHandler<P0>(&self, phandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXDTDHandler>,
    {
        (::windows::core::Interface::vtable(self).putDTDHandler)(::windows::core::Interface::as_raw(self), phandler.into_param().abi()).ok()
    }
    pub unsafe fn getErrorHandler(&self) -> ::windows::core::Result<ISAXErrorHandler> {
        let mut result__ = ::windows::core::zeroed::<ISAXErrorHandler>();
        (::windows::core::Interface::vtable(self).getErrorHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putErrorHandler<P0>(&self, phandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISAXErrorHandler>,
    {
        (::windows::core::Interface::vtable(self).putErrorHandler)(::windows::core::Interface::as_raw(self), phandler.into_param().abi()).ok()
    }
    pub unsafe fn getBaseURL(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__ = ::windows::core::zeroed::<*mut u16>();
        (::windows::core::Interface::vtable(self).getBaseURL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putBaseURL<P0>(&self, pwchbaseurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).putBaseURL)(::windows::core::Interface::as_raw(self), pwchbaseurl.into_param().abi()).ok()
    }
    pub unsafe fn getSecureBaseURL(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__ = ::windows::core::zeroed::<*mut u16>();
        (::windows::core::Interface::vtable(self).getSecureBaseURL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putSecureBaseURL<P0>(&self, pwchsecurebaseurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).putSecureBaseURL)(::windows::core::Interface::as_raw(self), pwchsecurebaseurl.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn parse(&self, varinput: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).parse)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varinput)).ok()
    }
    pub unsafe fn parseURL<P0>(&self, pwchurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).parseURL)(::windows::core::Interface::as_raw(self), pwchurl.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ISAXXMLReader, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISAXXMLReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISAXXMLReader {}
impl ::core::fmt::Debug for ISAXXMLReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISAXXMLReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISAXXMLReader {
    type Vtable = ISAXXMLReader_Vtbl;
}
impl ::core::clone::Clone for ISAXXMLReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISAXXMLReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4f96ed0_f829_476e_81c0_cdc7bd2a0802);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXXMLReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub getFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, pvfvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getFeature: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub putFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, vfvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    putFeature: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, varvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putProperty: usize,
    pub getEntityResolver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub putEntityResolver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub getContentHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub putContentHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub getDTDHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub putDTDHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub getErrorHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub putErrorHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub getBaseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwchbaseurl: *mut *mut u16) -> ::windows::core::HRESULT,
    pub putBaseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchbaseurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub getSecureBaseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwchsecurebaseurl: *mut *mut u16) -> ::windows::core::HRESULT,
    pub putSecureBaseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchsecurebaseurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varinput: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    parse: usize,
    pub parseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchema(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchema {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn targetNamespace(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).targetNamespace)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).version)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn types(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).types)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn elements(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).elements)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributeGroups(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).attributeGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn modelGroups(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).modelGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn notations(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).notations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schemaLocations(&self) -> ::windows::core::Result<ISchemaStringCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaStringCollection>();
        (::windows::core::Interface::vtable(self).schemaLocations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchema, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchema {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchema {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchema {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchema").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchema {
    type Vtable = ISchema_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchema {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchema {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08b4_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchema_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    pub targetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetnamespace: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub types: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, types: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    types: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub elements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elements: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    elements: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    attributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub attributeGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    attributeGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub modelGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelgroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    modelGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub notations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notations: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    notations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub schemaLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, schemalocations: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    schemaLocations: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaAny(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaAny {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn minOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.minOccurs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn maxOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.maxOccurs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn namespaces(&self) -> ::windows::core::Result<ISchemaStringCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaStringCollection>();
        (::windows::core::Interface::vtable(self).namespaces)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn processContents(&self) -> ::windows::core::Result<SCHEMAPROCESSCONTENTS> {
        let mut result__ = ::windows::core::zeroed::<SCHEMAPROCESSCONTENTS>();
        (::windows::core::Interface::vtable(self).processContents)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaAny, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem, ISchemaParticle);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaAny {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaAny {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaAny {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaAny").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaAny {
    type Vtable = ISchemaAny_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaAny {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaAny {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08bc_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaAny_Vtbl {
    pub base__: ISchemaParticle_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub namespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    namespaces: usize,
    pub processContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processcontents: *mut SCHEMAPROCESSCONTENTS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaAttribute(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaAttribute {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn r#type(&self) -> ::windows::core::Result<ISchemaType> {
        let mut result__ = ::windows::core::zeroed::<ISchemaType>();
        (::windows::core::Interface::vtable(self).r#type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn scope(&self) -> ::windows::core::Result<ISchemaComplexType> {
        let mut result__ = ::windows::core::zeroed::<ISchemaComplexType>();
        (::windows::core::Interface::vtable(self).scope)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn defaultValue(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).defaultValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn fixedValue(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).fixedValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn r#use(&self) -> ::windows::core::Result<SCHEMAUSE> {
        let mut result__ = ::windows::core::zeroed::<SCHEMAUSE>();
        (::windows::core::Interface::vtable(self).r#use)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isReference(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).isReference)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaAttribute, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaAttribute {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaAttribute").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaAttribute {
    type Vtable = ISchemaAttribute_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08b6_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaAttribute_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub r#type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    r#type: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    scope: usize,
    pub defaultValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub fixedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fixedvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub r#use: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#use: *mut SCHEMAUSE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub isReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isReference: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaAttributeGroup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaAttributeGroup {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn anyAttribute(&self) -> ::windows::core::Result<ISchemaAny> {
        let mut result__ = ::windows::core::zeroed::<ISchemaAny>();
        (::windows::core::Interface::vtable(self).anyAttribute)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaAttributeGroup, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaAttributeGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaAttributeGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaAttributeGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaAttributeGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaAttributeGroup {
    type Vtable = ISchemaAttributeGroup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaAttributeGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaAttributeGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08ba_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaAttributeGroup_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub anyAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anyattribute: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    anyAttribute: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    attributes: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaComplexType(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaComplexType {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn baseTypes(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).base__.baseTypes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn r#final(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD> {
        let mut result__ = ::windows::core::zeroed::<SCHEMADERIVATIONMETHOD>();
        (::windows::core::Interface::vtable(self).base__.r#final)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn variety(&self) -> ::windows::core::Result<SCHEMATYPEVARIETY> {
        let mut result__ = ::windows::core::zeroed::<SCHEMATYPEVARIETY>();
        (::windows::core::Interface::vtable(self).base__.variety)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn derivedBy(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD> {
        let mut result__ = ::windows::core::zeroed::<SCHEMADERIVATIONMETHOD>();
        (::windows::core::Interface::vtable(self).base__.derivedBy)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isValid<P0>(&self, data: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.isValid)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn minExclusive(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.minExclusive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn minInclusive(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.minInclusive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn maxExclusive(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.maxExclusive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn maxInclusive(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.maxInclusive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn totalDigits(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.totalDigits)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn fractionDigits(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.fractionDigits)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn length(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn minLength(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.minLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn maxLength(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.maxLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn enumeration(&self) -> ::windows::core::Result<ISchemaStringCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaStringCollection>();
        (::windows::core::Interface::vtable(self).base__.enumeration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn whitespace(&self) -> ::windows::core::Result<SCHEMAWHITESPACE> {
        let mut result__ = ::windows::core::zeroed::<SCHEMAWHITESPACE>();
        (::windows::core::Interface::vtable(self).base__.whitespace)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn patterns(&self) -> ::windows::core::Result<ISchemaStringCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaStringCollection>();
        (::windows::core::Interface::vtable(self).base__.patterns)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isAbstract(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).isAbstract)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn anyAttribute(&self) -> ::windows::core::Result<ISchemaAny> {
        let mut result__ = ::windows::core::zeroed::<ISchemaAny>();
        (::windows::core::Interface::vtable(self).anyAttribute)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn contentType(&self) -> ::windows::core::Result<SCHEMACONTENTTYPE> {
        let mut result__ = ::windows::core::zeroed::<SCHEMACONTENTTYPE>();
        (::windows::core::Interface::vtable(self).contentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn contentModel(&self) -> ::windows::core::Result<ISchemaModelGroup> {
        let mut result__ = ::windows::core::zeroed::<ISchemaModelGroup>();
        (::windows::core::Interface::vtable(self).contentModel)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn prohibitedSubstitutions(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD> {
        let mut result__ = ::windows::core::zeroed::<SCHEMADERIVATIONMETHOD>();
        (::windows::core::Interface::vtable(self).prohibitedSubstitutions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaComplexType, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem, ISchemaType);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaComplexType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaComplexType {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaComplexType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaComplexType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaComplexType {
    type Vtable = ISchemaComplexType_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaComplexType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaComplexType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08b9_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaComplexType_Vtbl {
    pub base__: ISchemaType_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub isAbstract: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#abstract: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isAbstract: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub anyAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anyattribute: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    anyAttribute: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    attributes: usize,
    pub contentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *mut SCHEMACONTENTTYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub contentModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentmodel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    contentModel: usize,
    pub prohibitedSubstitutions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibited: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaElement(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaElement {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn minOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.minOccurs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn maxOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.maxOccurs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn r#type(&self) -> ::windows::core::Result<ISchemaType> {
        let mut result__ = ::windows::core::zeroed::<ISchemaType>();
        (::windows::core::Interface::vtable(self).r#type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn scope(&self) -> ::windows::core::Result<ISchemaComplexType> {
        let mut result__ = ::windows::core::zeroed::<ISchemaComplexType>();
        (::windows::core::Interface::vtable(self).scope)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn defaultValue(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).defaultValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn fixedValue(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).fixedValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isNillable(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).isNillable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn identityConstraints(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).identityConstraints)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn substitutionGroup(&self) -> ::windows::core::Result<ISchemaElement> {
        let mut result__ = ::windows::core::zeroed::<ISchemaElement>();
        (::windows::core::Interface::vtable(self).substitutionGroup)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn substitutionGroupExclusions(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD> {
        let mut result__ = ::windows::core::zeroed::<SCHEMADERIVATIONMETHOD>();
        (::windows::core::Interface::vtable(self).substitutionGroupExclusions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn disallowedSubstitutions(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD> {
        let mut result__ = ::windows::core::zeroed::<SCHEMADERIVATIONMETHOD>();
        (::windows::core::Interface::vtable(self).disallowedSubstitutions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isAbstract(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).isAbstract)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isReference(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).isReference)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaElement, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem, ISchemaParticle);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaElement {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaElement").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaElement {
    type Vtable = ISchemaElement_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08b7_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaElement_Vtbl {
    pub base__: ISchemaParticle_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub r#type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    r#type: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    scope: usize,
    pub defaultValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub fixedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fixedvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub isNillable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nillable: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isNillable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub identityConstraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, constraints: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    identityConstraints: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub substitutionGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    substitutionGroup: usize,
    pub substitutionGroupExclusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, exclusions: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT,
    pub disallowedSubstitutions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disallowed: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub isAbstract: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#abstract: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isAbstract: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub isReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isReference: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaIdentityConstraint(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaIdentityConstraint {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn selector(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).selector)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn fields(&self) -> ::windows::core::Result<ISchemaStringCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaStringCollection>();
        (::windows::core::Interface::vtable(self).fields)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn referencedKey(&self) -> ::windows::core::Result<ISchemaIdentityConstraint> {
        let mut result__ = ::windows::core::zeroed::<ISchemaIdentityConstraint>();
        (::windows::core::Interface::vtable(self).referencedKey)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaIdentityConstraint, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaIdentityConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaIdentityConstraint {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaIdentityConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaIdentityConstraint").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaIdentityConstraint {
    type Vtable = ISchemaIdentityConstraint_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaIdentityConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaIdentityConstraint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08bd_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaIdentityConstraint_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    pub selector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selector: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub fields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fields: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    fields: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub referencedKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    referencedKey: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaItem {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaItem, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaItem {
    type Vtable = ISchemaItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08b3_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaItem_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub namespaceURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub schema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, schema: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    schema: usize,
    pub id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub itemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemtype: *mut SOMITEMTYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub unhandledAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    unhandledAttributes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub writeAnnotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotationsink: *mut ::core::ffi::c_void, iswritten: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    writeAnnotation: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaItemCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaItemCollection {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_item(&self, index: i32) -> ::windows::core::Result<ISchemaItem> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItem>();
        (::windows::core::Interface::vtable(self).get_item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn itemByName<P0>(&self, name: P0) -> ::windows::core::Result<ISchemaItem>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISchemaItem>();
        (::windows::core::Interface::vtable(self).itemByName)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn itemByQName<P0, P1>(&self, name: P0, namespaceuri: P1) -> ::windows::core::Result<ISchemaItem>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISchemaItem>();
        (::windows::core::Interface::vtable(self).itemByQName)(::windows::core::Interface::as_raw(self), name.into_param().abi(), namespaceuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._newEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaItemCollection, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaItemCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaItemCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaItemCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaItemCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaItemCollection {
    type Vtable = ISchemaItemCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaItemCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaItemCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08b2_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaItemCollection_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub itemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    itemByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub itemByQName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    itemByQName: usize,
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaModelGroup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaModelGroup {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn minOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.minOccurs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn maxOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.maxOccurs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn particles(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).particles)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaModelGroup, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem, ISchemaParticle);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaModelGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaModelGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaModelGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaModelGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaModelGroup {
    type Vtable = ISchemaModelGroup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaModelGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaModelGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08bb_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaModelGroup_Vtbl {
    pub base__: ISchemaParticle_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub particles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, particles: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    particles: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaNotation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaNotation {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn systemIdentifier(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).systemIdentifier)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn publicIdentifier(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).publicIdentifier)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaNotation, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaNotation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaNotation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaNotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaNotation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaNotation {
    type Vtable = ISchemaNotation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaNotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaNotation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08be_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaNotation_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    pub systemIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub publicIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaParticle(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaParticle {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn minOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).minOccurs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn maxOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).maxOccurs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaParticle, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaParticle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaParticle {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaParticle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaParticle").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaParticle {
    type Vtable = ISchemaParticle_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaParticle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaParticle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08b5_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaParticle_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub minOccurs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    minOccurs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub maxOccurs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    maxOccurs: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaStringCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaStringCollection {
    pub unsafe fn get_item(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._newEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaStringCollection, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaStringCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaStringCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaStringCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaStringCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaStringCollection {
    type Vtable = ISchemaStringCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaStringCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaStringCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08b1_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaStringCollection_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub get_item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, bstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchemaType(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchemaType {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn schema(&self) -> ::windows::core::Result<ISchema> {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).base__.schema)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE> {
        let mut result__ = ::windows::core::zeroed::<SOMITEMTYPE>();
        (::windows::core::Interface::vtable(self).base__.itemType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXAttributes>();
        (::windows::core::Interface::vtable(self).base__.unhandledAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.writeAnnotation)(::windows::core::Interface::as_raw(self), annotationsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn baseTypes(&self) -> ::windows::core::Result<ISchemaItemCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaItemCollection>();
        (::windows::core::Interface::vtable(self).baseTypes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn r#final(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD> {
        let mut result__ = ::windows::core::zeroed::<SCHEMADERIVATIONMETHOD>();
        (::windows::core::Interface::vtable(self).r#final)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn variety(&self) -> ::windows::core::Result<SCHEMATYPEVARIETY> {
        let mut result__ = ::windows::core::zeroed::<SCHEMATYPEVARIETY>();
        (::windows::core::Interface::vtable(self).variety)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn derivedBy(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD> {
        let mut result__ = ::windows::core::zeroed::<SCHEMADERIVATIONMETHOD>();
        (::windows::core::Interface::vtable(self).derivedBy)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isValid<P0>(&self, data: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).isValid)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn minExclusive(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).minExclusive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn minInclusive(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).minInclusive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn maxExclusive(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).maxExclusive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn maxInclusive(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).maxInclusive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn totalDigits(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).totalDigits)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn fractionDigits(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).fractionDigits)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn length(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn minLength(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).minLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn maxLength(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).maxLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn enumeration(&self) -> ::windows::core::Result<ISchemaStringCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaStringCollection>();
        (::windows::core::Interface::vtable(self).enumeration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn whitespace(&self) -> ::windows::core::Result<SCHEMAWHITESPACE> {
        let mut result__ = ::windows::core::zeroed::<SCHEMAWHITESPACE>();
        (::windows::core::Interface::vtable(self).whitespace)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn patterns(&self) -> ::windows::core::Result<ISchemaStringCollection> {
        let mut result__ = ::windows::core::zeroed::<ISchemaStringCollection>();
        (::windows::core::Interface::vtable(self).patterns)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISchemaType, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchemaType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchemaType {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchemaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchemaType {
    type Vtable = ISchemaType_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchemaType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISchemaType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08b8_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaType_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub baseTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, basetypes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    baseTypes: usize,
    pub r#final: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#final: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT,
    pub variety: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variety: *mut SCHEMATYPEVARIETY) -> ::windows::core::HRESULT,
    pub derivedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, derivedby: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub isValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>, valid: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isValid: usize,
    pub minExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minexclusive: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub minInclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mininclusive: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub maxExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxexclusive: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub maxInclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxinclusive: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub totalDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, totaldigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    totalDigits: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fractionDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fractiondigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    fractionDigits: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    length: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub minLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    minLength: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub maxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    maxLength: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub enumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumeration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    enumeration: usize,
    pub whitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitespace: *mut SCHEMAWHITESPACE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub patterns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, patterns: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    patterns: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IServerXMLHTTPRequest(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IServerXMLHTTPRequest {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn open<P0, P1>(&self, bstrmethod: P0, bstrurl: P1, varasync: super::super::super::System::Com::VARIANT, bstruser: super::super::super::System::Com::VARIANT, bstrpassword: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.open)(::windows::core::Interface::as_raw(self), bstrmethod.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(varasync), ::core::mem::transmute(bstruser), ::core::mem::transmute(bstrpassword)).ok()
    }
    pub unsafe fn setRequestHeader<P0, P1>(&self, bstrheader: P0, bstrvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.setRequestHeader)(::windows::core::Interface::as_raw(self), bstrheader.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    pub unsafe fn getResponseHeader<P0>(&self, bstrheader: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.getResponseHeader)(::windows::core::Interface::as_raw(self), bstrheader.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn getAllResponseHeaders(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.getAllResponseHeaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn send(&self, varbody: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.send)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varbody)).ok()
    }
    pub unsafe fn abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn status(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn statusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.statusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn responseXML(&self) -> ::windows::core::Result<super::super::super::System::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).base__.responseXML)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn responseText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.responseText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn responseBody(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.responseBody)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn responseStream(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.responseStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn readyState(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.readyState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Setonreadystatechange<P0>(&self, preadystatesink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.Setonreadystatechange)(::windows::core::Interface::as_raw(self), preadystatesink.into_param().abi()).ok()
    }
    pub unsafe fn setTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).setTimeouts)(::windows::core::Interface::as_raw(self), resolvetimeout, connecttimeout, sendtimeout, receivetimeout).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn waitForResponse(&self, timeoutinseconds: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).waitForResponse)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(timeoutinseconds), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getOption(&self, option: SERVERXMLHTTP_OPTION) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getOption)(::windows::core::Interface::as_raw(self), option, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setOption(&self, option: SERVERXMLHTTP_OPTION, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).setOption)(::windows::core::Interface::as_raw(self), option, ::core::mem::transmute(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IServerXMLHTTPRequest, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLHTTPRequest);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IServerXMLHTTPRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IServerXMLHTTPRequest {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IServerXMLHTTPRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServerXMLHTTPRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IServerXMLHTTPRequest {
    type Vtable = IServerXMLHTTPRequest_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IServerXMLHTTPRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IServerXMLHTTPRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e9196bf_13ba_4dd4_91ca_6c571f281495);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IServerXMLHTTPRequest_Vtbl {
    pub base__: IXMLHTTPRequest_Vtbl,
    pub setTimeouts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub waitForResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeoutinseconds: super::super::super::System::Com::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    waitForResponse: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getOption: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setOption: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IServerXMLHTTPRequest2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IServerXMLHTTPRequest2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn open<P0, P1>(&self, bstrmethod: P0, bstrurl: P1, varasync: super::super::super::System::Com::VARIANT, bstruser: super::super::super::System::Com::VARIANT, bstrpassword: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.open)(::windows::core::Interface::as_raw(self), bstrmethod.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(varasync), ::core::mem::transmute(bstruser), ::core::mem::transmute(bstrpassword)).ok()
    }
    pub unsafe fn setRequestHeader<P0, P1>(&self, bstrheader: P0, bstrvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.setRequestHeader)(::windows::core::Interface::as_raw(self), bstrheader.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    pub unsafe fn getResponseHeader<P0>(&self, bstrheader: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.getResponseHeader)(::windows::core::Interface::as_raw(self), bstrheader.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn getAllResponseHeaders(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.getAllResponseHeaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn send(&self, varbody: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.send)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varbody)).ok()
    }
    pub unsafe fn abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn status(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.base__.status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn statusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.statusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn responseXML(&self) -> ::windows::core::Result<super::super::super::System::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).base__.base__.responseXML)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn responseText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.responseText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn responseBody(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.responseBody)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn responseStream(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.responseStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn readyState(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.base__.readyState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Setonreadystatechange<P0>(&self, preadystatesink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Setonreadystatechange)(::windows::core::Interface::as_raw(self), preadystatesink.into_param().abi()).ok()
    }
    pub unsafe fn setTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.setTimeouts)(::windows::core::Interface::as_raw(self), resolvetimeout, connecttimeout, sendtimeout, receivetimeout).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn waitForResponse(&self, timeoutinseconds: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.waitForResponse)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(timeoutinseconds), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getOption(&self, option: SERVERXMLHTTP_OPTION) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.getOption)(::windows::core::Interface::as_raw(self), option, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setOption(&self, option: SERVERXMLHTTP_OPTION, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.setOption)(::windows::core::Interface::as_raw(self), option, ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setProxy(&self, proxysetting: SXH_PROXY_SETTING, varproxyserver: super::super::super::System::Com::VARIANT, varbypasslist: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).setProxy)(::windows::core::Interface::as_raw(self), proxysetting, ::core::mem::transmute(varproxyserver), ::core::mem::transmute(varbypasslist)).ok()
    }
    pub unsafe fn setProxyCredentials<P0, P1>(&self, bstrusername: P0, bstrpassword: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setProxyCredentials)(::windows::core::Interface::as_raw(self), bstrusername.into_param().abi(), bstrpassword.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IServerXMLHTTPRequest2, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLHTTPRequest, IServerXMLHTTPRequest);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IServerXMLHTTPRequest2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IServerXMLHTTPRequest2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IServerXMLHTTPRequest2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServerXMLHTTPRequest2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IServerXMLHTTPRequest2 {
    type Vtable = IServerXMLHTTPRequest2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IServerXMLHTTPRequest2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IServerXMLHTTPRequest2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e01311b_c322_4b0a_bd77_b90cfdc8dce7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IServerXMLHTTPRequest2_Vtbl {
    pub base__: IServerXMLHTTPRequest_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, proxysetting: SXH_PROXY_SETTING, varproxyserver: super::super::super::System::Com::VARIANT, varbypasslist: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setProxy: usize,
    pub setProxyCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBMXNamespaceManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBMXNamespaceManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetallowOverride<P0>(&self, foverride: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetallowOverride)(::windows::core::Interface::as_raw(self), foverride.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn allowOverride(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).allowOverride)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn pushContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).pushContext)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn pushNodeContext<P0, P1>(&self, contextnode: P0, fdeep: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).pushNodeContext)(::windows::core::Interface::as_raw(self), contextnode.into_param().abi(), fdeep.into_param().abi()).ok()
    }
    pub unsafe fn popContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).popContext)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn declarePrefix<P0, P1>(&self, prefix: P0, namespaceuri: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).declarePrefix)(::windows::core::Interface::as_raw(self), prefix.into_param().abi(), namespaceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getDeclaredPrefixes(&self) -> ::windows::core::Result<IMXNamespacePrefixes> {
        let mut result__ = ::windows::core::zeroed::<IMXNamespacePrefixes>();
        (::windows::core::Interface::vtable(self).getDeclaredPrefixes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getPrefixes<P0>(&self, namespaceuri: P0) -> ::windows::core::Result<IMXNamespacePrefixes>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMXNamespacePrefixes>();
        (::windows::core::Interface::vtable(self).getPrefixes)(::windows::core::Interface::as_raw(self), namespaceuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getURI<P0>(&self, prefix: P0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getURI)(::windows::core::Interface::as_raw(self), prefix.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getURIFromNode<P0, P1>(&self, strprefix: P0, contextnode: P1) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getURIFromNode)(::windows::core::Interface::as_raw(self), strprefix.into_param().abi(), contextnode.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBMXNamespaceManager, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBMXNamespaceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBMXNamespaceManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBMXNamespaceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBMXNamespaceManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBMXNamespaceManager {
    type Vtable = IVBMXNamespaceManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBMXNamespaceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBMXNamespaceManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc90352f5_643c_4fbc_bb23_e996eb2d51fd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBMXNamespaceManager_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetallowOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetallowOverride: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub allowOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foverride: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    allowOverride: usize,
    pub reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub pushContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub pushNodeContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contextnode: *mut ::core::ffi::c_void, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    pushNodeContext: usize,
    pub popContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub declarePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefix: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub getDeclaredPrefixes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefixes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getDeclaredPrefixes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getPrefixes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, prefixes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getPrefixes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefix: ::std::mem::MaybeUninit<::windows::core::BSTR>, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getURI: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getURIFromNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strprefix: ::std::mem::MaybeUninit<::windows::core::BSTR>, contextnode: *mut ::core::ffi::c_void, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getURIFromNode: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBSAXAttributes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXAttributes {
    pub unsafe fn length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn getURI(&self, nindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getURI)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn getLocalName(&self, nindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getLocalName)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn getQName(&self, nindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getQName)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn getIndexFromName<P0, P1>(&self, struri: P0, strlocalname: P1) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).getIndexFromName)(::windows::core::Interface::as_raw(self), struri.into_param().abi(), strlocalname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn getIndexFromQName<P0>(&self, strqname: P0) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).getIndexFromQName)(::windows::core::Interface::as_raw(self), strqname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn getType(&self, nindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getType)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn getTypeFromName<P0, P1>(&self, struri: P0, strlocalname: P1) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getTypeFromName)(::windows::core::Interface::as_raw(self), struri.into_param().abi(), strlocalname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn getTypeFromQName<P0>(&self, strqname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getTypeFromQName)(::windows::core::Interface::as_raw(self), strqname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn getValue(&self, nindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getValue)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn getValueFromName<P0, P1>(&self, struri: P0, strlocalname: P1) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getValueFromName)(::windows::core::Interface::as_raw(self), struri.into_param().abi(), strlocalname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn getValueFromQName<P0>(&self, strqname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getValueFromQName)(::windows::core::Interface::as_raw(self), strqname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBSAXAttributes, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBSAXAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBSAXAttributes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBSAXAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBSAXAttributes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBSAXAttributes {
    type Vtable = IVBSAXAttributes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBSAXAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBSAXAttributes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10dc0586_132b_4cac_8bb3_db00ac8b7ee0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXAttributes_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nlength: *mut i32) -> ::windows::core::HRESULT,
    pub getURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, struri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getQName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, strqname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getIndexFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT,
    pub getIndexFromQName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT,
    pub getType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, strtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getTypeFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getTypeFromQName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getValueFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getValueFromQName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBSAXContentHandler(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXContentHandler {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_documentLocator<P0>(&self, olocator: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IVBSAXLocator>,
    {
        (::windows::core::Interface::vtable(self).putref_documentLocator)(::windows::core::Interface::as_raw(self), olocator.into_param().abi()).ok()
    }
    pub unsafe fn startDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).startDocument)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn endDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).endDocument)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn startPrefixMapping(&self, strprefix: *mut ::windows::core::BSTR, struri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).startPrefixMapping)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strprefix), ::core::mem::transmute(struri)).ok()
    }
    pub unsafe fn endPrefixMapping(&self, strprefix: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).endPrefixMapping)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strprefix)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn startElement<P0>(&self, strnamespaceuri: *mut ::windows::core::BSTR, strlocalname: *mut ::windows::core::BSTR, strqname: *mut ::windows::core::BSTR, oattributes: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IVBSAXAttributes>,
    {
        (::windows::core::Interface::vtable(self).startElement)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strnamespaceuri), ::core::mem::transmute(strlocalname), ::core::mem::transmute(strqname), oattributes.into_param().abi()).ok()
    }
    pub unsafe fn endElement(&self, strnamespaceuri: *mut ::windows::core::BSTR, strlocalname: *mut ::windows::core::BSTR, strqname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).endElement)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strnamespaceuri), ::core::mem::transmute(strlocalname), ::core::mem::transmute(strqname)).ok()
    }
    pub unsafe fn characters(&self, strchars: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).characters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strchars)).ok()
    }
    pub unsafe fn ignorableWhitespace(&self, strchars: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ignorableWhitespace)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strchars)).ok()
    }
    pub unsafe fn processingInstruction(&self, strtarget: *mut ::windows::core::BSTR, strdata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).processingInstruction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strtarget), ::core::mem::transmute(strdata)).ok()
    }
    pub unsafe fn skippedEntity(&self, strname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).skippedEntity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strname)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBSAXContentHandler, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBSAXContentHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBSAXContentHandler {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBSAXContentHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBSAXContentHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBSAXContentHandler {
    type Vtable = IVBSAXContentHandler_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBSAXContentHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBSAXContentHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ed7290a_4dd5_4b46_bb26_4e4155e77faa);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXContentHandler_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_documentLocator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_documentLocator: usize,
    pub startDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub endDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub startPrefixMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strprefix: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, struri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub endPrefixMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strprefix: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub startElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strqname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, oattributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    startElement: usize,
    pub endElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strqname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub characters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ignorableWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub processingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strtarget: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub skippedEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBSAXDTDHandler(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXDTDHandler {
    pub unsafe fn notationDecl(&self, strname: *mut ::windows::core::BSTR, strpublicid: *mut ::windows::core::BSTR, strsystemid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).notationDecl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strname), ::core::mem::transmute(strpublicid), ::core::mem::transmute(strsystemid)).ok()
    }
    pub unsafe fn unparsedEntityDecl(&self, strname: *mut ::windows::core::BSTR, strpublicid: *mut ::windows::core::BSTR, strsystemid: *mut ::windows::core::BSTR, strnotationname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).unparsedEntityDecl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strname), ::core::mem::transmute(strpublicid), ::core::mem::transmute(strsystemid), ::core::mem::transmute(strnotationname)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBSAXDTDHandler, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBSAXDTDHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBSAXDTDHandler {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBSAXDTDHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBSAXDTDHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBSAXDTDHandler {
    type Vtable = IVBSAXDTDHandler_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBSAXDTDHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBSAXDTDHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24fb3297_302d_4620_ba39_3a732d850558);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXDTDHandler_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub notationDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub unparsedEntityDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strnotationname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBSAXDeclHandler(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXDeclHandler {
    pub unsafe fn elementDecl(&self, strname: *mut ::windows::core::BSTR, strmodel: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).elementDecl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strname), ::core::mem::transmute(strmodel)).ok()
    }
    pub unsafe fn attributeDecl(&self, strelementname: *mut ::windows::core::BSTR, strattributename: *mut ::windows::core::BSTR, strtype: *mut ::windows::core::BSTR, strvaluedefault: *mut ::windows::core::BSTR, strvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).attributeDecl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strelementname), ::core::mem::transmute(strattributename), ::core::mem::transmute(strtype), ::core::mem::transmute(strvaluedefault), ::core::mem::transmute(strvalue)).ok()
    }
    pub unsafe fn internalEntityDecl(&self, strname: *mut ::windows::core::BSTR, strvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).internalEntityDecl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strname), ::core::mem::transmute(strvalue)).ok()
    }
    pub unsafe fn externalEntityDecl(&self, strname: *mut ::windows::core::BSTR, strpublicid: *mut ::windows::core::BSTR, strsystemid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).externalEntityDecl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strname), ::core::mem::transmute(strpublicid), ::core::mem::transmute(strsystemid)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBSAXDeclHandler, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBSAXDeclHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBSAXDeclHandler {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBSAXDeclHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBSAXDeclHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBSAXDeclHandler {
    type Vtable = IVBSAXDeclHandler_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBSAXDeclHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBSAXDeclHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8917260_7579_4be1_b5dd_7afbfa6f077b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXDeclHandler_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub elementDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strmodel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub attributeDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strelementname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strattributename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strvaluedefault: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub internalEntityDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub externalEntityDecl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBSAXEntityResolver(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXEntityResolver {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn resolveEntity(&self, strpublicid: *mut ::windows::core::BSTR, strsystemid: *mut ::windows::core::BSTR, varinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).resolveEntity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strpublicid), ::core::mem::transmute(strsystemid), varinput).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBSAXEntityResolver, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBSAXEntityResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBSAXEntityResolver {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBSAXEntityResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBSAXEntityResolver").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBSAXEntityResolver {
    type Vtable = IVBSAXEntityResolver_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBSAXEntityResolver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBSAXEntityResolver {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c05d096_f45b_4aca_ad1a_aa0bc25518dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXEntityResolver_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub resolveEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, varinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    resolveEntity: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBSAXErrorHandler(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXErrorHandler {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn error<P0>(&self, olocator: P0, strerrormessage: *mut ::windows::core::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IVBSAXLocator>,
    {
        (::windows::core::Interface::vtable(self).error)(::windows::core::Interface::as_raw(self), olocator.into_param().abi(), ::core::mem::transmute(strerrormessage), nerrorcode).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn fatalError<P0>(&self, olocator: P0, strerrormessage: *mut ::windows::core::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IVBSAXLocator>,
    {
        (::windows::core::Interface::vtable(self).fatalError)(::windows::core::Interface::as_raw(self), olocator.into_param().abi(), ::core::mem::transmute(strerrormessage), nerrorcode).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ignorableWarning<P0>(&self, olocator: P0, strerrormessage: *mut ::windows::core::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IVBSAXLocator>,
    {
        (::windows::core::Interface::vtable(self).ignorableWarning)(::windows::core::Interface::as_raw(self), olocator.into_param().abi(), ::core::mem::transmute(strerrormessage), nerrorcode).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBSAXErrorHandler, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBSAXErrorHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBSAXErrorHandler {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBSAXErrorHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBSAXErrorHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBSAXErrorHandler {
    type Vtable = IVBSAXErrorHandler_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBSAXErrorHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBSAXErrorHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd963d3fe_173c_4862_9095_b92f66995f52);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXErrorHandler_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, nerrorcode: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    error: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub fatalError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, nerrorcode: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    fatalError: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ignorableWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, nerrorcode: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ignorableWarning: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBSAXLexicalHandler(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXLexicalHandler {
    pub unsafe fn startDTD(&self, strname: *mut ::windows::core::BSTR, strpublicid: *mut ::windows::core::BSTR, strsystemid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).startDTD)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strname), ::core::mem::transmute(strpublicid), ::core::mem::transmute(strsystemid)).ok()
    }
    pub unsafe fn endDTD(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).endDTD)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn startEntity(&self, strname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).startEntity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strname)).ok()
    }
    pub unsafe fn endEntity(&self, strname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).endEntity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strname)).ok()
    }
    pub unsafe fn startCDATA(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).startCDATA)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn endCDATA(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).endCDATA)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn comment(&self, strchars: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).comment)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(strchars)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBSAXLexicalHandler, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBSAXLexicalHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBSAXLexicalHandler {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBSAXLexicalHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBSAXLexicalHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBSAXLexicalHandler {
    type Vtable = IVBSAXLexicalHandler_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBSAXLexicalHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBSAXLexicalHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x032aac35_8c0e_4d9d_979f_e3b702935576);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXLexicalHandler_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub startDTD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub endDTD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub startEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub endEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub startCDATA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub endCDATA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBSAXLocator(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXLocator {
    pub unsafe fn columnNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).columnNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn lineNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).lineNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn publicId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).publicId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn systemId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).systemId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBSAXLocator, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBSAXLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBSAXLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBSAXLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBSAXLocator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBSAXLocator {
    type Vtable = IVBSAXLocator_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBSAXLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBSAXLocator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x796e7ac5_5aa2_4eff_acad_3faaf01a3288);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXLocator_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub columnNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolumn: *mut i32) -> ::windows::core::HRESULT,
    pub lineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nline: *mut i32) -> ::windows::core::HRESULT,
    pub publicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub systemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBSAXXMLFilter(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXXMLFilter {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parent(&self) -> ::windows::core::Result<IVBSAXXMLReader> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXXMLReader>();
        (::windows::core::Interface::vtable(self).parent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_parent<P0>(&self, oreader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IVBSAXXMLReader>,
    {
        (::windows::core::Interface::vtable(self).putref_parent)(::windows::core::Interface::as_raw(self), oreader.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBSAXXMLFilter, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBSAXXMLFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBSAXXMLFilter {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBSAXXMLFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBSAXXMLFilter").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBSAXXMLFilter {
    type Vtable = IVBSAXXMLFilter_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBSAXXMLFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBSAXXMLFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1299eb1b_5b88_433e_82de_82ca75ad4e04);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXXMLFilter_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    parent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oreader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_parent: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IVBSAXXMLReader(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXXMLReader {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getFeature<P0>(&self, strname: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).getFeature)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn putFeature<P0, P1>(&self, strname: P0, fvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).putFeature)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), fvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getProperty<P0>(&self, strname: P0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getProperty)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putProperty<P0>(&self, strname: P0, varvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).putProperty)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(varvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn entityResolver(&self) -> ::windows::core::Result<IVBSAXEntityResolver> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXEntityResolver>();
        (::windows::core::Interface::vtable(self).entityResolver)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_entityResolver<P0>(&self, oresolver: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IVBSAXEntityResolver>,
    {
        (::windows::core::Interface::vtable(self).putref_entityResolver)(::windows::core::Interface::as_raw(self), oresolver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn contentHandler(&self) -> ::windows::core::Result<IVBSAXContentHandler> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXContentHandler>();
        (::windows::core::Interface::vtable(self).contentHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_contentHandler<P0>(&self, ohandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IVBSAXContentHandler>,
    {
        (::windows::core::Interface::vtable(self).putref_contentHandler)(::windows::core::Interface::as_raw(self), ohandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn dtdHandler(&self) -> ::windows::core::Result<IVBSAXDTDHandler> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXDTDHandler>();
        (::windows::core::Interface::vtable(self).dtdHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_dtdHandler<P0>(&self, ohandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IVBSAXDTDHandler>,
    {
        (::windows::core::Interface::vtable(self).putref_dtdHandler)(::windows::core::Interface::as_raw(self), ohandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn errorHandler(&self) -> ::windows::core::Result<IVBSAXErrorHandler> {
        let mut result__ = ::windows::core::zeroed::<IVBSAXErrorHandler>();
        (::windows::core::Interface::vtable(self).errorHandler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_errorHandler<P0>(&self, ohandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IVBSAXErrorHandler>,
    {
        (::windows::core::Interface::vtable(self).putref_errorHandler)(::windows::core::Interface::as_raw(self), ohandler.into_param().abi()).ok()
    }
    pub unsafe fn baseURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).baseURL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetbaseURL<P0>(&self, strbaseurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetbaseURL)(::windows::core::Interface::as_raw(self), strbaseurl.into_param().abi()).ok()
    }
    pub unsafe fn secureBaseURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).secureBaseURL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetsecureBaseURL<P0>(&self, strsecurebaseurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetsecureBaseURL)(::windows::core::Interface::as_raw(self), strsecurebaseurl.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn parse(&self, varinput: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).parse)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varinput)).ok()
    }
    pub unsafe fn parseURL<P0>(&self, strurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).parseURL)(::windows::core::Interface::as_raw(self), strurl.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IVBSAXXMLReader, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IVBSAXXMLReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IVBSAXXMLReader {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IVBSAXXMLReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBSAXXMLReader").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IVBSAXXMLReader {
    type Vtable = IVBSAXXMLReader_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IVBSAXXMLReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IVBSAXXMLReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c033caa_6cd6_4f73_b728_4531af74945f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXXMLReader_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub getFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getFeature: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub putFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    putFeature: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putProperty: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub entityResolver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    entityResolver: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_entityResolver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_entityResolver: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub contentHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    contentHandler: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_contentHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_contentHandler: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub dtdHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    dtdHandler: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_dtdHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_dtdHandler: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub errorHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    errorHandler: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_errorHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_errorHandler: usize,
    pub baseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strbaseurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetbaseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strbaseurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub secureBaseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsecurebaseurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetsecureBaseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsecurebaseurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varinput: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    parse: usize,
    pub parseURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLAttribute(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLAttribute {
    pub unsafe fn name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn value(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).value)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLAttribute, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLAttribute {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLAttribute").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLAttribute {
    type Vtable = IXMLAttribute_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4d4a0fc_3b73_11d1_b2b4_00c04fb92596);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLAttribute_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, n: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMAttribute(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMAttribute {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    pub unsafe fn name(&self, attributename: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(attributename)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn value(&self, attributevalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).value)(::windows::core::Interface::as_raw(self), attributevalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setvalue(&self, attributevalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Setvalue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(attributevalue)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMAttribute, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMAttribute {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMAttribute").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMAttribute {
    type Vtable = IXMLDOMAttribute_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf85_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMAttribute_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributevalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Setvalue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributevalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Setvalue: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMCDATASection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMCDATASection {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    pub unsafe fn data(&self, data: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.data)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn Setdata<P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Setdata)(::windows::core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn length(&self, datalength: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.length)(::windows::core::Interface::as_raw(self), datalength).ok()
    }
    pub unsafe fn substringData(&self, offset: i32, count: i32, data: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.substringData)(::windows::core::Interface::as_raw(self), offset, count, ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn appendData<P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.appendData)(::windows::core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn insertData<P0>(&self, offset: i32, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.insertData)(::windows::core::Interface::as_raw(self), offset, data.into_param().abi()).ok()
    }
    pub unsafe fn deleteData(&self, offset: i32, count: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.deleteData)(::windows::core::Interface::as_raw(self), offset, count).ok()
    }
    pub unsafe fn replaceData<P0>(&self, offset: i32, count: i32, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.replaceData)(::windows::core::Interface::as_raw(self), offset, count, data.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn splitText(&self, offset: i32) -> ::windows::core::Result<IXMLDOMText> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMText>();
        (::windows::core::Interface::vtable(self).base__.splitText)(::windows::core::Interface::as_raw(self), offset, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMCDATASection, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode, IXMLDOMCharacterData, IXMLDOMText);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMCDATASection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMCDATASection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMCDATASection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMCDATASection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMCDATASection {
    type Vtable = IXMLDOMCDATASection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMCDATASection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMCDATASection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf8a_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMCDATASection_Vtbl {
    pub base__: IXMLDOMText_Vtbl,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMCharacterData(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMCharacterData {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    pub unsafe fn data(&self, data: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).data)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn Setdata<P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Setdata)(::windows::core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn length(&self, datalength: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), datalength).ok()
    }
    pub unsafe fn substringData(&self, offset: i32, count: i32, data: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).substringData)(::windows::core::Interface::as_raw(self), offset, count, ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn appendData<P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).appendData)(::windows::core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn insertData<P0>(&self, offset: i32, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).insertData)(::windows::core::Interface::as_raw(self), offset, data.into_param().abi()).ok()
    }
    pub unsafe fn deleteData(&self, offset: i32, count: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).deleteData)(::windows::core::Interface::as_raw(self), offset, count).ok()
    }
    pub unsafe fn replaceData<P0>(&self, offset: i32, count: i32, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).replaceData)(::windows::core::Interface::as_raw(self), offset, count, data.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMCharacterData, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMCharacterData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMCharacterData {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMCharacterData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMCharacterData").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMCharacterData {
    type Vtable = IXMLDOMCharacterData_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMCharacterData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMCharacterData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf84_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMCharacterData_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Setdata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datalength: *mut i32) -> ::windows::core::HRESULT,
    pub substringData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub appendData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub insertData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: i32, data: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub deleteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: i32, count: i32) -> ::windows::core::HRESULT,
    pub replaceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMComment(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMComment {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    pub unsafe fn data(&self, data: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.data)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn Setdata<P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Setdata)(::windows::core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn length(&self, datalength: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.length)(::windows::core::Interface::as_raw(self), datalength).ok()
    }
    pub unsafe fn substringData(&self, offset: i32, count: i32, data: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.substringData)(::windows::core::Interface::as_raw(self), offset, count, ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn appendData<P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.appendData)(::windows::core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn insertData<P0>(&self, offset: i32, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.insertData)(::windows::core::Interface::as_raw(self), offset, data.into_param().abi()).ok()
    }
    pub unsafe fn deleteData(&self, offset: i32, count: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.deleteData)(::windows::core::Interface::as_raw(self), offset, count).ok()
    }
    pub unsafe fn replaceData<P0>(&self, offset: i32, count: i32, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.replaceData)(::windows::core::Interface::as_raw(self), offset, count, data.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMComment, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode, IXMLDOMCharacterData);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMComment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMComment {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMComment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMComment").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMComment {
    type Vtable = IXMLDOMComment_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMComment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMComment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf88_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMComment_Vtbl {
    pub base__: IXMLDOMCharacterData_Vtbl,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMDocument(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocument {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn doctype(&self) -> ::windows::core::Result<IXMLDOMDocumentType> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocumentType>();
        (::windows::core::Interface::vtable(self).doctype)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn implementation(&self) -> ::windows::core::Result<IXMLDOMImplementation> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMImplementation>();
        (::windows::core::Interface::vtable(self).implementation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn documentElement(&self) -> ::windows::core::Result<IXMLDOMElement> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMElement>();
        (::windows::core::Interface::vtable(self).documentElement)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_documentElement<P0>(&self, domelement: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMElement>,
    {
        (::windows::core::Interface::vtable(self).putref_documentElement)(::windows::core::Interface::as_raw(self), domelement.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createElement<P0>(&self, tagname: P0) -> ::windows::core::Result<IXMLDOMElement>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMElement>();
        (::windows::core::Interface::vtable(self).createElement)(::windows::core::Interface::as_raw(self), tagname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createDocumentFragment(&self) -> ::windows::core::Result<IXMLDOMDocumentFragment> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocumentFragment>();
        (::windows::core::Interface::vtable(self).createDocumentFragment)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createTextNode<P0>(&self, data: P0) -> ::windows::core::Result<IXMLDOMText>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMText>();
        (::windows::core::Interface::vtable(self).createTextNode)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createComment<P0>(&self, data: P0) -> ::windows::core::Result<IXMLDOMComment>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMComment>();
        (::windows::core::Interface::vtable(self).createComment)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createCDATASection<P0>(&self, data: P0) -> ::windows::core::Result<IXMLDOMCDATASection>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMCDATASection>();
        (::windows::core::Interface::vtable(self).createCDATASection)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createProcessingInstruction<P0, P1>(&self, target: P0, data: P1) -> ::windows::core::Result<IXMLDOMProcessingInstruction>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMProcessingInstruction>();
        (::windows::core::Interface::vtable(self).createProcessingInstruction)(::windows::core::Interface::as_raw(self), target.into_param().abi(), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createAttribute<P0>(&self, name: P0) -> ::windows::core::Result<IXMLDOMAttribute>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMAttribute>();
        (::windows::core::Interface::vtable(self).createAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createEntityReference<P0>(&self, name: P0) -> ::windows::core::Result<IXMLDOMEntityReference>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMEntityReference>();
        (::windows::core::Interface::vtable(self).createEntityReference)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getElementsByTagName<P0>(&self, tagname: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).getElementsByTagName)(::windows::core::Interface::as_raw(self), tagname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn createNode<P0, P1>(&self, r#type: super::super::super::System::Com::VARIANT, name: P0, namespaceuri: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).createNode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(r#type), name.into_param().abi(), namespaceuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nodeFromID<P0>(&self, idstring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).nodeFromID)(::windows::core::Interface::as_raw(self), idstring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn load(&self, xmlsource: super::super::super::System::Com::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).load)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlsource), issuccessful).ok()
    }
    pub unsafe fn readyState(&self, value: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).readyState)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parseError(&self) -> ::windows::core::Result<IXMLDOMParseError> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMParseError>();
        (::windows::core::Interface::vtable(self).parseError)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn url(&self, urlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).url)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(urlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn r#async(&self, isasync: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).r#async)(::windows::core::Interface::as_raw(self), isasync).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setasync<P0>(&self, isasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).Setasync)(::windows::core::Interface::as_raw(self), isasync.into_param().abi()).ok()
    }
    pub unsafe fn abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn loadXML<P0>(&self, bstrxml: P0, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).loadXML)(::windows::core::Interface::as_raw(self), bstrxml.into_param().abi(), issuccessful).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn save(&self, destination: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).save)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(destination)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn validateOnParse(&self, isvalidating: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).validateOnParse)(::windows::core::Interface::as_raw(self), isvalidating).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetvalidateOnParse<P0>(&self, isvalidating: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetvalidateOnParse)(::windows::core::Interface::as_raw(self), isvalidating.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn resolveExternals(&self, isresolving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).resolveExternals)(::windows::core::Interface::as_raw(self), isresolving).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetresolveExternals<P0>(&self, isresolving: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetresolveExternals)(::windows::core::Interface::as_raw(self), isresolving.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn preserveWhiteSpace(&self, ispreserving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).preserveWhiteSpace)(::windows::core::Interface::as_raw(self), ispreserving).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetpreserveWhiteSpace<P0>(&self, ispreserving: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetpreserveWhiteSpace)(::windows::core::Interface::as_raw(self), ispreserving.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setonreadystatechange(&self, readystatechangesink: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Setonreadystatechange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(readystatechangesink)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setondataavailable(&self, ondataavailablesink: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Setondataavailable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ondataavailablesink)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setontransformnode(&self, ontransformnodesink: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Setontransformnode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ontransformnodesink)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMDocument, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMDocument {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMDocument").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMDocument {
    type Vtable = IXMLDOMDocument_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf81_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMDocument_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub doctype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    doctype: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub implementation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#impl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    implementation: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub documentElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domelement: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    documentElement: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_documentElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domelement: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_documentElement: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows::core::BSTR>, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createElement: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createDocumentFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, docfrag: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createDocumentFragment: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createTextNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>, text: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createTextNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>, comment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createComment: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createCDATASection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>, cdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createCDATASection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::std::mem::MaybeUninit<::windows::core::BSTR>, data: ::std::mem::MaybeUninit<::windows::core::BSTR>, pi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createProcessingInstruction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, attribute: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createAttribute: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createEntityReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, entityref: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createEntityReference: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows::core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getElementsByTagName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub createNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: super::super::super::System::Com::VARIANT, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, node: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    createNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub nodeFromID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idstring: ::std::mem::MaybeUninit<::windows::core::BSTR>, node: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    nodeFromID: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xmlsource: super::super::super::System::Com::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    load: usize,
    pub readyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub parseError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    parseError: usize,
    pub url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urlstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub r#async: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isasync: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    r#async: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Setasync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isasync: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Setasync: usize,
    pub abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub loadXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxml: ::std::mem::MaybeUninit<::windows::core::BSTR>, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    loadXML: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    save: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub validateOnParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isvalidating: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    validateOnParse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetvalidateOnParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isvalidating: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetvalidateOnParse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub resolveExternals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isresolving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    resolveExternals: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetresolveExternals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isresolving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetresolveExternals: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub preserveWhiteSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ispreserving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    preserveWhiteSpace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetpreserveWhiteSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ispreserving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetpreserveWhiteSpace: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Setonreadystatechange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readystatechangesink: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Setonreadystatechange: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Setondataavailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ondataavailablesink: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Setondataavailable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Setontransformnode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ontransformnodesink: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Setontransformnode: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMDocument2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocument2 {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn doctype(&self) -> ::windows::core::Result<IXMLDOMDocumentType> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocumentType>();
        (::windows::core::Interface::vtable(self).base__.doctype)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn implementation(&self) -> ::windows::core::Result<IXMLDOMImplementation> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMImplementation>();
        (::windows::core::Interface::vtable(self).base__.implementation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn documentElement(&self) -> ::windows::core::Result<IXMLDOMElement> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMElement>();
        (::windows::core::Interface::vtable(self).base__.documentElement)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_documentElement<P0>(&self, domelement: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMElement>,
    {
        (::windows::core::Interface::vtable(self).base__.putref_documentElement)(::windows::core::Interface::as_raw(self), domelement.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createElement<P0>(&self, tagname: P0) -> ::windows::core::Result<IXMLDOMElement>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMElement>();
        (::windows::core::Interface::vtable(self).base__.createElement)(::windows::core::Interface::as_raw(self), tagname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createDocumentFragment(&self) -> ::windows::core::Result<IXMLDOMDocumentFragment> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocumentFragment>();
        (::windows::core::Interface::vtable(self).base__.createDocumentFragment)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createTextNode<P0>(&self, data: P0) -> ::windows::core::Result<IXMLDOMText>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMText>();
        (::windows::core::Interface::vtable(self).base__.createTextNode)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createComment<P0>(&self, data: P0) -> ::windows::core::Result<IXMLDOMComment>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMComment>();
        (::windows::core::Interface::vtable(self).base__.createComment)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createCDATASection<P0>(&self, data: P0) -> ::windows::core::Result<IXMLDOMCDATASection>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMCDATASection>();
        (::windows::core::Interface::vtable(self).base__.createCDATASection)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createProcessingInstruction<P0, P1>(&self, target: P0, data: P1) -> ::windows::core::Result<IXMLDOMProcessingInstruction>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMProcessingInstruction>();
        (::windows::core::Interface::vtable(self).base__.createProcessingInstruction)(::windows::core::Interface::as_raw(self), target.into_param().abi(), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createAttribute<P0>(&self, name: P0) -> ::windows::core::Result<IXMLDOMAttribute>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMAttribute>();
        (::windows::core::Interface::vtable(self).base__.createAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createEntityReference<P0>(&self, name: P0) -> ::windows::core::Result<IXMLDOMEntityReference>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMEntityReference>();
        (::windows::core::Interface::vtable(self).base__.createEntityReference)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getElementsByTagName<P0>(&self, tagname: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.getElementsByTagName)(::windows::core::Interface::as_raw(self), tagname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn createNode<P0, P1>(&self, r#type: super::super::super::System::Com::VARIANT, name: P0, namespaceuri: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.createNode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(r#type), name.into_param().abi(), namespaceuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nodeFromID<P0>(&self, idstring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nodeFromID)(::windows::core::Interface::as_raw(self), idstring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn load(&self, xmlsource: super::super::super::System::Com::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.load)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlsource), issuccessful).ok()
    }
    pub unsafe fn readyState(&self, value: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.readyState)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parseError(&self) -> ::windows::core::Result<IXMLDOMParseError> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMParseError>();
        (::windows::core::Interface::vtable(self).base__.parseError)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn url(&self, urlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.url)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(urlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn r#async(&self, isasync: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.r#async)(::windows::core::Interface::as_raw(self), isasync).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setasync<P0>(&self, isasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Setasync)(::windows::core::Interface::as_raw(self), isasync.into_param().abi()).ok()
    }
    pub unsafe fn abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn loadXML<P0>(&self, bstrxml: P0, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.loadXML)(::windows::core::Interface::as_raw(self), bstrxml.into_param().abi(), issuccessful).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn save(&self, destination: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.save)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(destination)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn validateOnParse(&self, isvalidating: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.validateOnParse)(::windows::core::Interface::as_raw(self), isvalidating).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetvalidateOnParse<P0>(&self, isvalidating: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetvalidateOnParse)(::windows::core::Interface::as_raw(self), isvalidating.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn resolveExternals(&self, isresolving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.resolveExternals)(::windows::core::Interface::as_raw(self), isresolving).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetresolveExternals<P0>(&self, isresolving: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetresolveExternals)(::windows::core::Interface::as_raw(self), isresolving.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn preserveWhiteSpace(&self, ispreserving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.preserveWhiteSpace)(::windows::core::Interface::as_raw(self), ispreserving).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetpreserveWhiteSpace<P0>(&self, ispreserving: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetpreserveWhiteSpace)(::windows::core::Interface::as_raw(self), ispreserving.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setonreadystatechange(&self, readystatechangesink: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Setonreadystatechange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(readystatechangesink)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setondataavailable(&self, ondataavailablesink: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Setondataavailable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ondataavailablesink)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setontransformnode(&self, ontransformnodesink: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Setontransformnode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ontransformnodesink)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn namespaces(&self) -> ::windows::core::Result<IXMLDOMSchemaCollection> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMSchemaCollection>();
        (::windows::core::Interface::vtable(self).namespaces)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn schemas(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).schemas)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putref_schemas(&self, othercollection: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).putref_schemas)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(othercollection)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn validate(&self) -> ::windows::core::Result<IXMLDOMParseError> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMParseError>();
        (::windows::core::Interface::vtable(self).validate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setProperty<P0>(&self, name: P0, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setProperty)(::windows::core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getProperty<P0>(&self, name: P0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getProperty)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMDocument2, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode, IXMLDOMDocument);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMDocument2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMDocument2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMDocument2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMDocument2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMDocument2 {
    type Vtable = IXMLDOMDocument2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMDocument2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMDocument2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf95_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMDocument2_Vtbl {
    pub base__: IXMLDOMDocument_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub namespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespacecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    namespaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub schemas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, othercollection: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    schemas: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_schemas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, othercollection: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_schemas: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    validate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getProperty: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMDocument3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocument3 {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn doctype(&self) -> ::windows::core::Result<IXMLDOMDocumentType> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocumentType>();
        (::windows::core::Interface::vtable(self).base__.base__.doctype)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn implementation(&self) -> ::windows::core::Result<IXMLDOMImplementation> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMImplementation>();
        (::windows::core::Interface::vtable(self).base__.base__.implementation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn documentElement(&self) -> ::windows::core::Result<IXMLDOMElement> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMElement>();
        (::windows::core::Interface::vtable(self).base__.base__.documentElement)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_documentElement<P0>(&self, domelement: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMElement>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.putref_documentElement)(::windows::core::Interface::as_raw(self), domelement.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createElement<P0>(&self, tagname: P0) -> ::windows::core::Result<IXMLDOMElement>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMElement>();
        (::windows::core::Interface::vtable(self).base__.base__.createElement)(::windows::core::Interface::as_raw(self), tagname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createDocumentFragment(&self) -> ::windows::core::Result<IXMLDOMDocumentFragment> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocumentFragment>();
        (::windows::core::Interface::vtable(self).base__.base__.createDocumentFragment)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createTextNode<P0>(&self, data: P0) -> ::windows::core::Result<IXMLDOMText>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMText>();
        (::windows::core::Interface::vtable(self).base__.base__.createTextNode)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createComment<P0>(&self, data: P0) -> ::windows::core::Result<IXMLDOMComment>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMComment>();
        (::windows::core::Interface::vtable(self).base__.base__.createComment)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createCDATASection<P0>(&self, data: P0) -> ::windows::core::Result<IXMLDOMCDATASection>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMCDATASection>();
        (::windows::core::Interface::vtable(self).base__.base__.createCDATASection)(::windows::core::Interface::as_raw(self), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createProcessingInstruction<P0, P1>(&self, target: P0, data: P1) -> ::windows::core::Result<IXMLDOMProcessingInstruction>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMProcessingInstruction>();
        (::windows::core::Interface::vtable(self).base__.base__.createProcessingInstruction)(::windows::core::Interface::as_raw(self), target.into_param().abi(), data.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createAttribute<P0>(&self, name: P0) -> ::windows::core::Result<IXMLDOMAttribute>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMAttribute>();
        (::windows::core::Interface::vtable(self).base__.base__.createAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createEntityReference<P0>(&self, name: P0) -> ::windows::core::Result<IXMLDOMEntityReference>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMEntityReference>();
        (::windows::core::Interface::vtable(self).base__.base__.createEntityReference)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getElementsByTagName<P0>(&self, tagname: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.getElementsByTagName)(::windows::core::Interface::as_raw(self), tagname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn createNode<P0, P1>(&self, r#type: super::super::super::System::Com::VARIANT, name: P0, namespaceuri: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.createNode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(r#type), name.into_param().abi(), namespaceuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nodeFromID<P0>(&self, idstring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.nodeFromID)(::windows::core::Interface::as_raw(self), idstring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn load(&self, xmlsource: super::super::super::System::Com::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.load)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlsource), issuccessful).ok()
    }
    pub unsafe fn readyState(&self, value: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.readyState)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parseError(&self) -> ::windows::core::Result<IXMLDOMParseError> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMParseError>();
        (::windows::core::Interface::vtable(self).base__.base__.parseError)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn url(&self, urlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.url)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(urlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn r#async(&self, isasync: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.r#async)(::windows::core::Interface::as_raw(self), isasync).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setasync<P0>(&self, isasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Setasync)(::windows::core::Interface::as_raw(self), isasync.into_param().abi()).ok()
    }
    pub unsafe fn abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn loadXML<P0>(&self, bstrxml: P0, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.loadXML)(::windows::core::Interface::as_raw(self), bstrxml.into_param().abi(), issuccessful).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn save(&self, destination: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.save)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(destination)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn validateOnParse(&self, isvalidating: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.validateOnParse)(::windows::core::Interface::as_raw(self), isvalidating).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetvalidateOnParse<P0>(&self, isvalidating: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetvalidateOnParse)(::windows::core::Interface::as_raw(self), isvalidating.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn resolveExternals(&self, isresolving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.resolveExternals)(::windows::core::Interface::as_raw(self), isresolving).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetresolveExternals<P0>(&self, isresolving: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetresolveExternals)(::windows::core::Interface::as_raw(self), isresolving.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn preserveWhiteSpace(&self, ispreserving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.preserveWhiteSpace)(::windows::core::Interface::as_raw(self), ispreserving).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetpreserveWhiteSpace<P0>(&self, ispreserving: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetpreserveWhiteSpace)(::windows::core::Interface::as_raw(self), ispreserving.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setonreadystatechange(&self, readystatechangesink: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Setonreadystatechange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(readystatechangesink)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setondataavailable(&self, ondataavailablesink: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Setondataavailable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ondataavailablesink)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setontransformnode(&self, ontransformnodesink: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Setontransformnode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ontransformnodesink)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn namespaces(&self) -> ::windows::core::Result<IXMLDOMSchemaCollection> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMSchemaCollection>();
        (::windows::core::Interface::vtable(self).base__.namespaces)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn schemas(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.schemas)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putref_schemas(&self, othercollection: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.putref_schemas)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(othercollection)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn validate(&self) -> ::windows::core::Result<IXMLDOMParseError> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMParseError>();
        (::windows::core::Interface::vtable(self).base__.validate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setProperty<P0>(&self, name: P0, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.setProperty)(::windows::core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getProperty<P0>(&self, name: P0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.getProperty)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn validateNode<P0>(&self, node: P0) -> ::windows::core::Result<IXMLDOMParseError>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMParseError>();
        (::windows::core::Interface::vtable(self).validateNode)(::windows::core::Interface::as_raw(self), node.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn importNode<P0, P1>(&self, node: P0, deep: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).importNode)(::windows::core::Interface::as_raw(self), node.into_param().abi(), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMDocument3, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode, IXMLDOMDocument, IXMLDOMDocument2);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMDocument3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMDocument3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMDocument3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMDocument3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMDocument3 {
    type Vtable = IXMLDOMDocument3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMDocument3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMDocument3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf96_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMDocument3_Vtbl {
    pub base__: IXMLDOMDocument2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub validateNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    validateNode: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub importNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, deep: super::super::super::Foundation::VARIANT_BOOL, clone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    importNode: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMDocumentFragment(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocumentFragment {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMDocumentFragment, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMDocumentFragment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMDocumentFragment {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMDocumentFragment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMDocumentFragment").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMDocumentFragment {
    type Vtable = IXMLDOMDocumentFragment_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMDocumentFragment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMDocumentFragment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3efaa413_272f_11d2_836f_0000f87a7782);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMDocumentFragment_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMDocumentType(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocumentType {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    pub unsafe fn name(&self, rootname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rootname)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn entities(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).entities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn notations(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).notations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMDocumentType, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMDocumentType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMDocumentType {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMDocumentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMDocumentType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMDocumentType {
    type Vtable = IXMLDOMDocumentType_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMDocumentType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMDocumentType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf8b_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMDocumentType_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub entities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entitymap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    entities: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub notations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notationmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    notations: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMElement(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMElement {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    pub unsafe fn tagName(&self, tagname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).tagName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(tagname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getAttribute<P0>(&self, name: P0, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).getAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setAttribute<P0>(&self, name: P0, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn removeAttribute<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).removeAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAttributeNode<P0>(&self, name: P0) -> ::windows::core::Result<IXMLDOMAttribute>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMAttribute>();
        (::windows::core::Interface::vtable(self).getAttributeNode)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn setAttributeNode<P0>(&self, domattribute: P0) -> ::windows::core::Result<IXMLDOMAttribute>
    where
        P0: ::windows::core::IntoParam<IXMLDOMAttribute>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMAttribute>();
        (::windows::core::Interface::vtable(self).setAttributeNode)(::windows::core::Interface::as_raw(self), domattribute.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeAttributeNode<P0>(&self, domattribute: P0) -> ::windows::core::Result<IXMLDOMAttribute>
    where
        P0: ::windows::core::IntoParam<IXMLDOMAttribute>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMAttribute>();
        (::windows::core::Interface::vtable(self).removeAttributeNode)(::windows::core::Interface::as_raw(self), domattribute.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getElementsByTagName<P0>(&self, tagname: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).getElementsByTagName)(::windows::core::Interface::as_raw(self), tagname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn normalize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).normalize)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMElement, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMElement {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMElement").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMElement {
    type Vtable = IXMLDOMElement_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf86_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMElement_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub tagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getAttribute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setAttribute: usize,
    pub removeAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub getAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAttributeNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub setAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domattribute: *mut ::core::ffi::c_void, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    setAttributeNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub removeAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domattribute: *mut ::core::ffi::c_void, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    removeAttributeNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows::core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getElementsByTagName: usize,
    pub normalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMEntity(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMEntity {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn publicId(&self, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).publicId)(::windows::core::Interface::as_raw(self), publicid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn systemId(&self, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).systemId)(::windows::core::Interface::as_raw(self), systemid).ok()
    }
    pub unsafe fn notationName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).notationName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMEntity, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMEntity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMEntity {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMEntity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMEntity").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMEntity {
    type Vtable = IXMLDOMEntity_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMEntity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf8d_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMEntity_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub publicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    publicId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub systemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    systemId: usize,
    pub notationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMEntityReference(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMEntityReference {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMEntityReference, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMEntityReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMEntityReference {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMEntityReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMEntityReference").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMEntityReference {
    type Vtable = IXMLDOMEntityReference_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMEntityReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMEntityReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf8e_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMEntityReference_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMImplementation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMImplementation {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasFeature<P0, P1>(&self, feature: P0, version: P1, hasfeature: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).hasFeature)(::windows::core::Interface::as_raw(self), feature.into_param().abi(), version.into_param().abi(), hasfeature).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMImplementation, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMImplementation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMImplementation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMImplementation {
    type Vtable = IXMLDOMImplementation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMImplementation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf8f_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMImplementation_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub hasFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows::core::BSTR>, version: ::std::mem::MaybeUninit<::windows::core::BSTR>, hasfeature: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    hasFeature: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMNamedNodeMap(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNamedNodeMap {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getNamedItem<P0>(&self, name: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).getNamedItem)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn setNamedItem<P0>(&self, newitem: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).setNamedItem)(::windows::core::Interface::as_raw(self), newitem.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeNamedItem<P0>(&self, name: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).removeNamedItem)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_item(&self, index: i32) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).get_item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn length(&self, listlength: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), listlength).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getQualifiedItem<P0, P1>(&self, basename: P0, namespaceuri: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).getQualifiedItem)(::windows::core::Interface::as_raw(self), basename.into_param().abi(), namespaceuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeQualifiedItem<P0, P1>(&self, basename: P0, namespaceuri: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).removeQualifiedItem)(::windows::core::Interface::as_raw(self), basename.into_param().abi(), namespaceuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).nextNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._newEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMNamedNodeMap, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMNamedNodeMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMNamedNodeMap {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMNamedNodeMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMNamedNodeMap").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMNamedNodeMap {
    type Vtable = IXMLDOMNamedNodeMap_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMNamedNodeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMNamedNodeMap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf83_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMNamedNodeMap_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub getNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, nameditem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getNamedItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub setNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newitem: *mut ::core::ffi::c_void, nameitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    setNamedItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub removeNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, nameditem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    removeNamedItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_item: usize,
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub getQualifiedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, qualifieditem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getQualifiedItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub removeQualifiedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, qualifieditem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    removeQualifiedItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub nextNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    nextNode: usize,
    pub reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMNode(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNode {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMNode, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMNode {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMNode").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMNode {
    type Vtable = IXMLDOMNode_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf80_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMNode_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub nodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub nodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    nodeValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetnodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetnodeValue: usize,
    pub nodeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut DOMNodeType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub parentNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    parentNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub childNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    childNodes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub firstChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firstchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    firstChild: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub lastChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    lastChild: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub previousSibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previoussibling: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    previousSibling: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub nextSibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextsibling: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    nextSibling: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributemap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    attributes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub insertBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, refchild: super::super::super::System::Com::VARIANT, outnewchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    insertBefore: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub replaceChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, oldchild: *mut ::core::ffi::c_void, outoldchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    replaceChild: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub removeChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childnode: *mut ::core::ffi::c_void, oldchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    removeChild: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub appendChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, outnewchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    appendChild: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub hasChildNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    hasChildNodes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ownerDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xmldomdocument: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ownerDocument: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub cloneNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deep: super::super::super::Foundation::VARIANT_BOOL, cloneroot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    cloneNode: usize,
    pub nodeTypeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Settext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub specified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    specified: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub definition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definitionnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    definition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub nodeTypedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    nodeTypedValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetnodeTypedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetnodeTypedValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub dataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    dataType: usize,
    pub SetdataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatypename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xmlstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub transformNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void, xmlstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    transformNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub selectNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querystring: ::std::mem::MaybeUninit<::windows::core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    selectNodes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub selectSingleNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querystring: ::std::mem::MaybeUninit<::windows::core::BSTR>, resultnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    selectSingleNode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub parsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    parsed: usize,
    pub namespaceURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub prefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefixstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub baseName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namestring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub transformNodeToObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    transformNodeToObject: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMNodeList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNodeList {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_item(&self, index: i32) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).get_item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn length(&self, listlength: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), listlength).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).nextNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._newEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMNodeList, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMNodeList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMNodeList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMNodeList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMNodeList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMNodeList {
    type Vtable = IXMLDOMNodeList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMNodeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMNodeList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf82_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMNodeList_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_item: usize,
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub nextNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    nextNode: usize,
    pub reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMNotation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNotation {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn publicId(&self, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).publicId)(::windows::core::Interface::as_raw(self), publicid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn systemId(&self, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).systemId)(::windows::core::Interface::as_raw(self), systemid).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMNotation, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMNotation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMNotation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMNotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMNotation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMNotation {
    type Vtable = IXMLDOMNotation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMNotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMNotation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf8c_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMNotation_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub publicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    publicId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub systemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    systemId: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMParseError(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMParseError {
    pub unsafe fn errorCode(&self, errorcode: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).errorCode)(::windows::core::Interface::as_raw(self), errorcode).ok()
    }
    pub unsafe fn url(&self, urlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).url)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(urlstring)).ok()
    }
    pub unsafe fn reason(&self, reasonstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).reason)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(reasonstring)).ok()
    }
    pub unsafe fn srcText(&self, sourcestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).srcText)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(sourcestring)).ok()
    }
    pub unsafe fn line(&self, linenumber: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).line)(::windows::core::Interface::as_raw(self), linenumber).ok()
    }
    pub unsafe fn linepos(&self, lineposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).linepos)(::windows::core::Interface::as_raw(self), lineposition).ok()
    }
    pub unsafe fn filepos(&self, fileposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).filepos)(::windows::core::Interface::as_raw(self), fileposition).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMParseError, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMParseError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMParseError {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMParseError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMParseError").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMParseError {
    type Vtable = IXMLDOMParseError_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMParseError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMParseError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3efaa426_272f_11d2_836f_0000f87a7782);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMParseError_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub errorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorcode: *mut i32) -> ::windows::core::HRESULT,
    pub url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urlstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reasonstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub srcText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub line: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows::core::HRESULT,
    pub linepos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineposition: *mut i32) -> ::windows::core::HRESULT,
    pub filepos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileposition: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMParseError2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMParseError2 {
    pub unsafe fn errorCode(&self, errorcode: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.errorCode)(::windows::core::Interface::as_raw(self), errorcode).ok()
    }
    pub unsafe fn url(&self, urlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.url)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(urlstring)).ok()
    }
    pub unsafe fn reason(&self, reasonstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.reason)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(reasonstring)).ok()
    }
    pub unsafe fn srcText(&self, sourcestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.srcText)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(sourcestring)).ok()
    }
    pub unsafe fn line(&self, linenumber: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.line)(::windows::core::Interface::as_raw(self), linenumber).ok()
    }
    pub unsafe fn linepos(&self, lineposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.linepos)(::windows::core::Interface::as_raw(self), lineposition).ok()
    }
    pub unsafe fn filepos(&self, fileposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.filepos)(::windows::core::Interface::as_raw(self), fileposition).ok()
    }
    pub unsafe fn errorXPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).errorXPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn allErrors(&self) -> ::windows::core::Result<IXMLDOMParseErrorCollection> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMParseErrorCollection>();
        (::windows::core::Interface::vtable(self).allErrors)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn errorParameters(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).errorParameters)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn errorParametersCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).errorParametersCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMParseError2, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMParseError);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMParseError2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMParseError2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMParseError2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMParseError2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMParseError2 {
    type Vtable = IXMLDOMParseError2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMParseError2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMParseError2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3efaa428_272f_11d2_836f_0000f87a7782);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMParseError2_Vtbl {
    pub base__: IXMLDOMParseError_Vtbl,
    pub errorXPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpathexpr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub allErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allerrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    allErrors: usize,
    pub errorParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, param1: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub errorParametersCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMParseErrorCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMParseErrorCollection {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_item(&self, index: i32) -> ::windows::core::Result<IXMLDOMParseError2> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMParseError2>();
        (::windows::core::Interface::vtable(self).get_item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn next(&self) -> ::windows::core::Result<IXMLDOMParseError2> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMParseError2>();
        (::windows::core::Interface::vtable(self).next)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._newEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMParseErrorCollection, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMParseErrorCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMParseErrorCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMParseErrorCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMParseErrorCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMParseErrorCollection {
    type Vtable = IXMLDOMParseErrorCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMParseErrorCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMParseErrorCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3efaa429_272f_11d2_836f_0000f87a7782);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMParseErrorCollection_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, error: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_item: usize,
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    next: usize,
    pub reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMProcessingInstruction(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMProcessingInstruction {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    pub unsafe fn target(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).target)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    pub unsafe fn data(&self, value: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).data)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn Setdata<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Setdata)(::windows::core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMProcessingInstruction, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMProcessingInstruction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMProcessingInstruction {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMProcessingInstruction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMProcessingInstruction").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMProcessingInstruction {
    type Vtable = IXMLDOMProcessingInstruction_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMProcessingInstruction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMProcessingInstruction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf89_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMProcessingInstruction_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Setdata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMSchemaCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMSchemaCollection {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn add<P0>(&self, namespaceuri: P0, var: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).add)(::windows::core::Interface::as_raw(self), namespaceuri.into_param().abi(), ::core::mem::transmute(var)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get<P0>(&self, namespaceuri: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).get)(::windows::core::Interface::as_raw(self), namespaceuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn remove<P0>(&self, namespaceuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).remove)(::windows::core::Interface::as_raw(self), namespaceuri.into_param().abi()).ok()
    }
    pub unsafe fn length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_namespaceURI(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_namespaceURI)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn addCollection<P0>(&self, othercollection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMSchemaCollection>,
    {
        (::windows::core::Interface::vtable(self).addCollection)(::windows::core::Interface::as_raw(self), othercollection.into_param().abi()).ok()
    }
    pub unsafe fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._newEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMSchemaCollection, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMSchemaCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMSchemaCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMSchemaCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMSchemaCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMSchemaCollection {
    type Vtable = IXMLDOMSchemaCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMSchemaCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMSchemaCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x373984c8_b845_449b_91e7_45ac83036ade);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMSchemaCollection_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, var: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, schemanode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get: usize,
    pub remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT,
    pub get_namespaceURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, length: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub addCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, othercollection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    addCollection: usize,
    pub _newEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMSchemaCollection2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMSchemaCollection2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn add<P0>(&self, namespaceuri: P0, var: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.add)(::windows::core::Interface::as_raw(self), namespaceuri.into_param().abi(), ::core::mem::transmute(var)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get<P0>(&self, namespaceuri: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.get)(::windows::core::Interface::as_raw(self), namespaceuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn remove<P0>(&self, namespaceuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.remove)(::windows::core::Interface::as_raw(self), namespaceuri.into_param().abi()).ok()
    }
    pub unsafe fn length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_namespaceURI(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.get_namespaceURI)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn addCollection<P0>(&self, othercollection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMSchemaCollection>,
    {
        (::windows::core::Interface::vtable(self).base__.addCollection)(::windows::core::Interface::as_raw(self), othercollection.into_param().abi()).ok()
    }
    pub unsafe fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__._newEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn validate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).validate)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetvalidateOnLoad<P0>(&self, validateonload: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetvalidateOnLoad)(::windows::core::Interface::as_raw(self), validateonload.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn validateOnLoad(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).validateOnLoad)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getSchema<P0>(&self, namespaceuri: P0) -> ::windows::core::Result<ISchema>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISchema>();
        (::windows::core::Interface::vtable(self).getSchema)(::windows::core::Interface::as_raw(self), namespaceuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getDeclaration<P0>(&self, node: P0) -> ::windows::core::Result<ISchemaItem>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<ISchemaItem>();
        (::windows::core::Interface::vtable(self).getDeclaration)(::windows::core::Interface::as_raw(self), node.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMSchemaCollection2, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMSchemaCollection);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMSchemaCollection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMSchemaCollection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMSchemaCollection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMSchemaCollection2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMSchemaCollection2 {
    type Vtable = IXMLDOMSchemaCollection2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMSchemaCollection2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMSchemaCollection2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ea08b0_dd1b_4664_9a50_c2f40f4bd79a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMSchemaCollection2_Vtbl {
    pub base__: IXMLDOMSchemaCollection_Vtbl,
    pub validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetvalidateOnLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, validateonload: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetvalidateOnLoad: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub validateOnLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, validateonload: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    validateOnLoad: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, schema: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getSchema: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getDeclaration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getDeclaration: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMSelection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMSelection {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_item(&self, index: i32) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.get_item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn length(&self, listlength: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.length)(::windows::core::Interface::as_raw(self), listlength).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__._newEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn expr(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).expr)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Setexpr<P0>(&self, expression: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Setexpr)(::windows::core::Interface::as_raw(self), expression.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn context(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).context)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_context<P0>(&self, pnode: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).putref_context)(::windows::core::Interface::as_raw(self), pnode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn peekNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).peekNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn matches<P0>(&self, pnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).matches)(::windows::core::Interface::as_raw(self), pnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeNext(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).removeNext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn removeAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).removeAll)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn clone(&self) -> ::windows::core::Result<IXMLDOMSelection> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMSelection>();
        (::windows::core::Interface::vtable(self).clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getProperty<P0>(&self, name: P0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getProperty)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setProperty<P0>(&self, name: P0, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setProperty)(::windows::core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMSelection, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNodeList);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMSelection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMSelection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMSelection {
    type Vtable = IXMLDOMSelection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMSelection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMSelection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa634fc7_5888_44a7_a257_3a47150d3a0e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMSelection_Vtbl {
    pub base__: IXMLDOMNodeList_Vtbl,
    pub expr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expression: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Setexpr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expression: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    context: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_context: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub peekNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    peekNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub matches: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    matches: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub removeNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    removeNext: usize,
    pub removeAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    clone: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setProperty: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDOMText(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMText {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    pub unsafe fn data(&self, data: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.data)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn Setdata<P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Setdata)(::windows::core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn length(&self, datalength: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.length)(::windows::core::Interface::as_raw(self), datalength).ok()
    }
    pub unsafe fn substringData(&self, offset: i32, count: i32, data: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.substringData)(::windows::core::Interface::as_raw(self), offset, count, ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn appendData<P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.appendData)(::windows::core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    pub unsafe fn insertData<P0>(&self, offset: i32, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.insertData)(::windows::core::Interface::as_raw(self), offset, data.into_param().abi()).ok()
    }
    pub unsafe fn deleteData(&self, offset: i32, count: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.deleteData)(::windows::core::Interface::as_raw(self), offset, count).ok()
    }
    pub unsafe fn replaceData<P0>(&self, offset: i32, count: i32, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.replaceData)(::windows::core::Interface::as_raw(self), offset, count, data.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn splitText(&self, offset: i32) -> ::windows::core::Result<IXMLDOMText> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMText>();
        (::windows::core::Interface::vtable(self).splitText)(::windows::core::Interface::as_raw(self), offset, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDOMText, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode, IXMLDOMCharacterData);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDOMText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDOMText {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDOMText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDOMText").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDOMText {
    type Vtable = IXMLDOMText_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDOMText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDOMText {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf87_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMText_Vtbl {
    pub base__: IXMLDOMCharacterData_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub splitText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: i32, righthandtextnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    splitText: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDSOControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDSOControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn XMLDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).XMLDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetXMLDocument<P0>(&self, ppdoc: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMDocument>,
    {
        (::windows::core::Interface::vtable(self).SetXMLDocument)(::windows::core::Interface::as_raw(self), ppdoc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JavaDSOCompatible(&self, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).JavaDSOCompatible)(::windows::core::Interface::as_raw(self), fjavadsocompatible).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetJavaDSOCompatible<P0>(&self, fjavadsocompatible: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetJavaDSOCompatible)(::windows::core::Interface::as_raw(self), fjavadsocompatible.into_param().abi()).ok()
    }
    pub unsafe fn readyState(&self, state: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).readyState)(::windows::core::Interface::as_raw(self), state).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDSOControl, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDSOControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDSOControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDSOControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDSOControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDSOControl {
    type Vtable = IXMLDSOControl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDSOControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDSOControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x310afa62_0575_11d2_9ca9_0060b0ec3d39);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDSOControl_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub XMLDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdoc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    XMLDocument: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetXMLDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdoc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetXMLDocument: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub JavaDSOCompatible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    JavaDSOCompatible: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetJavaDSOCompatible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetJavaDSOCompatible: usize,
    pub readyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDocument(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDocument {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn root(&self) -> ::windows::core::Result<IXMLElement> {
        let mut result__ = ::windows::core::zeroed::<IXMLElement>();
        (::windows::core::Interface::vtable(self).root)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn fileSize(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).fileSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn fileModifiedDate(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).fileModifiedDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn fileUpdatedDate(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).fileUpdatedDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn URL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).URL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetURL<P0>(&self, p: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetURL)(::windows::core::Interface::as_raw(self), p.into_param().abi()).ok()
    }
    pub unsafe fn mimeType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).mimeType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn readyState(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).readyState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn charset(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).charset)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Setcharset<P0>(&self, p: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Setcharset)(::windows::core::Interface::as_raw(self), p.into_param().abi()).ok()
    }
    pub unsafe fn version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).version)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn doctype(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).doctype)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn dtdURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).dtdURL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn createElement(&self, vtype: super::super::super::System::Com::VARIANT, var1: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLElement> {
        let mut result__ = ::windows::core::zeroed::<IXMLElement>();
        (::windows::core::Interface::vtable(self).createElement)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(vtype), ::core::mem::transmute(var1), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDocument, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDocument {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDocument").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDocument {
    type Vtable = IXMLDocument_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf52e2b61_18a1_11d1_b105_00805f49916b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDocument_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub root: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    root: usize,
    pub fileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub fileModifiedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub fileUpdatedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub URL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub mimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub readyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT,
    pub charset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Setcharset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub doctype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub dtdURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub createElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtype: super::super::super::System::Com::VARIANT, var1: super::super::super::System::Com::VARIANT, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    createElement: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLDocument2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLDocument2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn root(&self) -> ::windows::core::Result<IXMLElement2> {
        let mut result__ = ::windows::core::zeroed::<IXMLElement2>();
        (::windows::core::Interface::vtable(self).root)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn fileSize(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).fileSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn fileModifiedDate(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).fileModifiedDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn fileUpdatedDate(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).fileUpdatedDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn URL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).URL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetURL<P0>(&self, p: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetURL)(::windows::core::Interface::as_raw(self), p.into_param().abi()).ok()
    }
    pub unsafe fn mimeType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).mimeType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn readyState(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).readyState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn charset(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).charset)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Setcharset<P0>(&self, p: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Setcharset)(::windows::core::Interface::as_raw(self), p.into_param().abi()).ok()
    }
    pub unsafe fn version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).version)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn doctype(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).doctype)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn dtdURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).dtdURL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn createElement(&self, vtype: super::super::super::System::Com::VARIANT, var1: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLElement2> {
        let mut result__ = ::windows::core::zeroed::<IXMLElement2>();
        (::windows::core::Interface::vtable(self).createElement)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(vtype), ::core::mem::transmute(var1), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn r#async(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).r#async)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setasync<P0>(&self, f: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).Setasync)(::windows::core::Interface::as_raw(self), f.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLDocument2, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLDocument2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLDocument2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLDocument2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLDocument2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLDocument2 {
    type Vtable = IXMLDocument2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLDocument2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLDocument2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b8de2fe_8d2d_11d1_b2fc_00c04fd915a9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDocument2_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub root: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    root: usize,
    pub fileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub fileModifiedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub fileUpdatedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub URL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub mimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub readyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT,
    pub charset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Setcharset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub doctype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub dtdURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub createElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtype: super::super::super::System::Com::VARIANT, var1: super::super::super::System::Com::VARIANT, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    createElement: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub r#async: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pf: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    r#async: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Setasync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, f: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Setasync: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLElement(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLElement {
    pub unsafe fn tagName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).tagName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettagName<P0>(&self, p: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SettagName)(::windows::core::Interface::as_raw(self), p.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parent(&self) -> ::windows::core::Result<IXMLElement> {
        let mut result__ = ::windows::core::zeroed::<IXMLElement>();
        (::windows::core::Interface::vtable(self).parent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setAttribute<P0>(&self, strpropertyname: P0, propertyvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setAttribute)(::windows::core::Interface::as_raw(self), strpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getAttribute<P0>(&self, strpropertyname: P0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getAttribute)(::windows::core::Interface::as_raw(self), strpropertyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn removeAttribute<P0>(&self, strpropertyname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).removeAttribute)(::windows::core::Interface::as_raw(self), strpropertyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn children(&self) -> ::windows::core::Result<IXMLElementCollection> {
        let mut result__ = ::windows::core::zeroed::<IXMLElementCollection>();
        (::windows::core::Interface::vtable(self).children)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn r#type(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).r#type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn text(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).text)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Settext<P0>(&self, p: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Settext)(::windows::core::Interface::as_raw(self), p.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn addChild<P0>(&self, pchildelem: P0, lindex: i32, lreserved: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLElement>,
    {
        (::windows::core::Interface::vtable(self).addChild)(::windows::core::Interface::as_raw(self), pchildelem.into_param().abi(), lindex, lreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, pchildelem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLElement>,
    {
        (::windows::core::Interface::vtable(self).removeChild)(::windows::core::Interface::as_raw(self), pchildelem.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLElement, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLElement {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLElement").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLElement {
    type Vtable = IXMLElement_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f7f31ac_e15f_11d0_9c25_00c04fc99c8e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLElement_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub tagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SettagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    parent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setAttribute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getAttribute: usize,
    pub removeAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    children: usize,
    pub r#type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT,
    pub text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Settext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub addChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    addChild: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub removeChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    removeChild: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLElement2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLElement2 {
    pub unsafe fn tagName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).tagName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettagName<P0>(&self, p: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SettagName)(::windows::core::Interface::as_raw(self), p.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parent(&self) -> ::windows::core::Result<IXMLElement2> {
        let mut result__ = ::windows::core::zeroed::<IXMLElement2>();
        (::windows::core::Interface::vtable(self).parent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setAttribute<P0>(&self, strpropertyname: P0, propertyvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setAttribute)(::windows::core::Interface::as_raw(self), strpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getAttribute<P0>(&self, strpropertyname: P0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getAttribute)(::windows::core::Interface::as_raw(self), strpropertyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn removeAttribute<P0>(&self, strpropertyname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).removeAttribute)(::windows::core::Interface::as_raw(self), strpropertyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn children(&self) -> ::windows::core::Result<IXMLElementCollection> {
        let mut result__ = ::windows::core::zeroed::<IXMLElementCollection>();
        (::windows::core::Interface::vtable(self).children)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn r#type(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).r#type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn text(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).text)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Settext<P0>(&self, p: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Settext)(::windows::core::Interface::as_raw(self), p.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn addChild<P0>(&self, pchildelem: P0, lindex: i32, lreserved: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLElement2>,
    {
        (::windows::core::Interface::vtable(self).addChild)(::windows::core::Interface::as_raw(self), pchildelem.into_param().abi(), lindex, lreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, pchildelem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLElement2>,
    {
        (::windows::core::Interface::vtable(self).removeChild)(::windows::core::Interface::as_raw(self), pchildelem.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLElementCollection> {
        let mut result__ = ::windows::core::zeroed::<IXMLElementCollection>();
        (::windows::core::Interface::vtable(self).attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLElement2, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLElement2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLElement2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLElement2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLElement2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLElement2 {
    type Vtable = IXMLElement2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLElement2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLElement2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b8de2ff_8d2d_11d1_b2fc_00c04fd915a9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLElement2_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub tagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SettagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    parent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setAttribute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getAttribute: usize,
    pub removeAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    children: usize,
    pub r#type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT,
    pub text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Settext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub addChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    addChild: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub removeChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    removeChild: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    attributes: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLElementCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLElementCollection {
    pub unsafe fn Setlength(&self, v: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Setlength)(::windows::core::Interface::as_raw(self), v).ok()
    }
    pub unsafe fn length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._newEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn item(&self, var1: super::super::super::System::Com::VARIANT, var2: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::System::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).item)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(var1), ::core::mem::transmute(var2), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLElementCollection, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLElementCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLElementCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLElementCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLElementCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLElementCollection {
    type Vtable = IXMLElementCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLElementCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLElementCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65725580_9b5d_11d0_9bfe_00c04fc99c8e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLElementCollection_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Setlength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: i32) -> ::windows::core::HRESULT,
    pub length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut i32) -> ::windows::core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, var1: super::super::super::System::Com::VARIANT, var2: super::super::super::System::Com::VARIANT, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    item: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct IXMLError(::windows::core::IUnknown);
impl IXMLError {
    pub unsafe fn GetErrorInfo(&self, perrorreturn: *mut XML_ERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetErrorInfo)(::windows::core::Interface::as_raw(self), perrorreturn).ok()
    }
}
::windows::imp::interface_hierarchy!(IXMLError, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IXMLError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXMLError {}
impl ::core::fmt::Debug for IXMLError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXMLError {
    type Vtable = IXMLError_Vtbl;
}
impl ::core::clone::Clone for IXMLError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXMLError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x948c5ad3_c58d_11d0_9c0b_00c04fc99c8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLError_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrorreturn: *mut XML_ERROR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXMLHTTPRequest(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn open<P0, P1>(&self, bstrmethod: P0, bstrurl: P1, varasync: super::super::super::System::Com::VARIANT, bstruser: super::super::super::System::Com::VARIANT, bstrpassword: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).open)(::windows::core::Interface::as_raw(self), bstrmethod.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(varasync), ::core::mem::transmute(bstruser), ::core::mem::transmute(bstrpassword)).ok()
    }
    pub unsafe fn setRequestHeader<P0, P1>(&self, bstrheader: P0, bstrvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setRequestHeader)(::windows::core::Interface::as_raw(self), bstrheader.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    pub unsafe fn getResponseHeader<P0>(&self, bstrheader: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getResponseHeader)(::windows::core::Interface::as_raw(self), bstrheader.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn getAllResponseHeaders(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).getAllResponseHeaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn send(&self, varbody: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).send)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varbody)).ok()
    }
    pub unsafe fn abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn status(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn statusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).statusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn responseXML(&self) -> ::windows::core::Result<super::super::super::System::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).responseXML)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn responseText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).responseText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn responseBody(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).responseBody)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn responseStream(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).responseStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn readyState(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).readyState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Setonreadystatechange<P0>(&self, preadystatesink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).Setonreadystatechange)(::windows::core::Interface::as_raw(self), preadystatesink.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXMLHTTPRequest, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXMLHTTPRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXMLHTTPRequest {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXMLHTTPRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLHTTPRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXMLHTTPRequest {
    type Vtable = IXMLHTTPRequest_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXMLHTTPRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXMLHTTPRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed8c108d_4349_11d2_91a4_00c04f7969e8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLHTTPRequest_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmethod: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, varasync: super::super::super::System::Com::VARIANT, bstruser: super::super::super::System::Com::VARIANT, bstrpassword: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    open: usize,
    pub setRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrheader: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getResponseHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrheader: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub getAllResponseHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrheaders: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varbody: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    send: usize,
    pub abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows::core::HRESULT,
    pub statusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatus: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub responseXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbody: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    responseXML: usize,
    pub responseText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbody: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub responseBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    responseBody: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub responseStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    responseStream: usize,
    pub readyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Setonreadystatechange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preadystatesink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Setonreadystatechange: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct IXMLHTTPRequest2(::windows::core::IUnknown);
impl IXMLHTTPRequest2 {
    pub unsafe fn Open<P0, P1, P2, P3, P4, P5, P6>(&self, pwszmethod: P0, pwszurl: P1, pstatuscallback: P2, pwszusername: P3, pwszpassword: P4, pwszproxyusername: P5, pwszproxypassword: P6) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IXMLHTTPRequest2Callback>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P6: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), pwszmethod.into_param().abi(), pwszurl.into_param().abi(), pstatuscallback.into_param().abi(), pwszusername.into_param().abi(), pwszpassword.into_param().abi(), pwszproxyusername.into_param().abi(), pwszproxypassword.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Send<P0>(&self, pbody: P0, cbbody: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::ISequentialStream>,
    {
        (::windows::core::Interface::vtable(self).Send)(::windows::core::Interface::as_raw(self), pbody.into_param().abi(), cbbody).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCookie(&self, pcookie: *const XHR_COOKIE) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SetCookie)(::windows::core::Interface::as_raw(self), pcookie, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCustomResponseStream<P0>(&self, psequentialstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::ISequentialStream>,
    {
        (::windows::core::Interface::vtable(self).SetCustomResponseStream)(::windows::core::Interface::as_raw(self), psequentialstream.into_param().abi()).ok()
    }
    pub unsafe fn SetProperty(&self, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), eproperty, ullvalue).ok()
    }
    pub unsafe fn SetRequestHeader<P0, P1>(&self, pwszheader: P0, pwszvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRequestHeader)(::windows::core::Interface::as_raw(self), pwszheader.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    pub unsafe fn GetAllResponseHeaders(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__ = ::windows::core::zeroed::<*mut u16>();
        (::windows::core::Interface::vtable(self).GetAllResponseHeaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCookie<P0, P1>(&self, pwszurl: P0, pwszname: P1, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetCookie)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), pwszname.into_param().abi(), dwflags, pccookies, ppcookies).ok()
    }
    pub unsafe fn GetResponseHeader<P0>(&self, pwszheader: P0) -> ::windows::core::Result<*mut u16>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<*mut u16>();
        (::windows::core::Interface::vtable(self).GetResponseHeader)(::windows::core::Interface::as_raw(self), pwszheader.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IXMLHTTPRequest2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IXMLHTTPRequest2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXMLHTTPRequest2 {}
impl ::core::fmt::Debug for IXMLHTTPRequest2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLHTTPRequest2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXMLHTTPRequest2 {
    type Vtable = IXMLHTTPRequest2_Vtbl;
}
impl ::core::clone::Clone for IXMLHTTPRequest2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXMLHTTPRequest2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5d37dc0_552a_4d52_9cc0_a14d546fbd04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLHTTPRequest2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmethod: ::windows::core::PCWSTR, pwszurl: ::windows::core::PCWSTR, pstatuscallback: *mut ::core::ffi::c_void, pwszusername: ::windows::core::PCWSTR, pwszpassword: ::windows::core::PCWSTR, pwszproxyusername: ::windows::core::PCWSTR, pwszproxypassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *mut ::core::ffi::c_void, cbbody: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Send: usize,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcookie: *const XHR_COOKIE, pdwcookiestate: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCookie: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCustomResponseStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psequentialstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCustomResponseStream: usize,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::HRESULT,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszheader: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetAllResponseHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszheaders: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pwszname: ::windows::core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCookie: usize,
    pub GetResponseHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszheader: ::windows::core::PCWSTR, ppwszvalue: *mut *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct IXMLHTTPRequest2Callback(::windows::core::IUnknown);
impl IXMLHTTPRequest2Callback {
    pub unsafe fn OnRedirect<P0, P1>(&self, pxhr: P0, pwszredirecturl: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest2>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnRedirect)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), pwszredirecturl.into_param().abi()).ok()
    }
    pub unsafe fn OnHeadersAvailable<P0, P1>(&self, pxhr: P0, dwstatus: u32, pwszstatus: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest2>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnHeadersAvailable)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), dwstatus, pwszstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnDataAvailable<P0, P1>(&self, pxhr: P0, presponsestream: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest2>,
        P1: ::windows::core::IntoParam<super::super::super::System::Com::ISequentialStream>,
    {
        (::windows::core::Interface::vtable(self).OnDataAvailable)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), presponsestream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnResponseReceived<P0, P1>(&self, pxhr: P0, presponsestream: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest2>,
        P1: ::windows::core::IntoParam<super::super::super::System::Com::ISequentialStream>,
    {
        (::windows::core::Interface::vtable(self).OnResponseReceived)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), presponsestream.into_param().abi()).ok()
    }
    pub unsafe fn OnError<P0>(&self, pxhr: P0, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest2>,
    {
        (::windows::core::Interface::vtable(self).OnError)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), hrerror).ok()
    }
}
::windows::imp::interface_hierarchy!(IXMLHTTPRequest2Callback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IXMLHTTPRequest2Callback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXMLHTTPRequest2Callback {}
impl ::core::fmt::Debug for IXMLHTTPRequest2Callback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLHTTPRequest2Callback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXMLHTTPRequest2Callback {
    type Vtable = IXMLHTTPRequest2Callback_Vtbl;
}
impl ::core::clone::Clone for IXMLHTTPRequest2Callback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXMLHTTPRequest2Callback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa44a9299_e321_40de_8866_341b41669162);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLHTTPRequest2Callback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, pwszredirecturl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnHeadersAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, dwstatus: u32, pwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, presponsestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDataAvailable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnResponseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, presponsestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnResponseReceived: usize,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct IXMLHTTPRequest3(::windows::core::IUnknown);
impl IXMLHTTPRequest3 {
    pub unsafe fn Open<P0, P1, P2, P3, P4, P5, P6>(&self, pwszmethod: P0, pwszurl: P1, pstatuscallback: P2, pwszusername: P3, pwszpassword: P4, pwszproxyusername: P5, pwszproxypassword: P6) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IXMLHTTPRequest2Callback>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P6: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Open)(::windows::core::Interface::as_raw(self), pwszmethod.into_param().abi(), pwszurl.into_param().abi(), pstatuscallback.into_param().abi(), pwszusername.into_param().abi(), pwszpassword.into_param().abi(), pwszproxyusername.into_param().abi(), pwszproxypassword.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Send<P0>(&self, pbody: P0, cbbody: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::ISequentialStream>,
    {
        (::windows::core::Interface::vtable(self).base__.Send)(::windows::core::Interface::as_raw(self), pbody.into_param().abi(), cbbody).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCookie(&self, pcookie: *const XHR_COOKIE) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.SetCookie)(::windows::core::Interface::as_raw(self), pcookie, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCustomResponseStream<P0>(&self, psequentialstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::ISequentialStream>,
    {
        (::windows::core::Interface::vtable(self).base__.SetCustomResponseStream)(::windows::core::Interface::as_raw(self), psequentialstream.into_param().abi()).ok()
    }
    pub unsafe fn SetProperty(&self, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), eproperty, ullvalue).ok()
    }
    pub unsafe fn SetRequestHeader<P0, P1>(&self, pwszheader: P0, pwszvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRequestHeader)(::windows::core::Interface::as_raw(self), pwszheader.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    pub unsafe fn GetAllResponseHeaders(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__ = ::windows::core::zeroed::<*mut u16>();
        (::windows::core::Interface::vtable(self).base__.GetAllResponseHeaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCookie<P0, P1>(&self, pwszurl: P0, pwszname: P1, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetCookie)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), pwszname.into_param().abi(), dwflags, pccookies, ppcookies).ok()
    }
    pub unsafe fn GetResponseHeader<P0>(&self, pwszheader: P0) -> ::windows::core::Result<*mut u16>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<*mut u16>();
        (::windows::core::Interface::vtable(self).base__.GetResponseHeader)(::windows::core::Interface::as_raw(self), pwszheader.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClientCertificate<P0>(&self, pbclientcertificatehash: &[u8], pwszpin: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetClientCertificate)(::windows::core::Interface::as_raw(self), pbclientcertificatehash.len() as _, ::core::mem::transmute(pbclientcertificatehash.as_ptr()), pwszpin.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IXMLHTTPRequest3, ::windows::core::IUnknown, IXMLHTTPRequest2);
impl ::core::cmp::PartialEq for IXMLHTTPRequest3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXMLHTTPRequest3 {}
impl ::core::fmt::Debug for IXMLHTTPRequest3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLHTTPRequest3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXMLHTTPRequest3 {
    type Vtable = IXMLHTTPRequest3_Vtbl;
}
impl ::core::clone::Clone for IXMLHTTPRequest3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXMLHTTPRequest3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1c9feee_0617_4f23_9d58_8961ea43567c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLHTTPRequest3_Vtbl {
    pub base__: IXMLHTTPRequest2_Vtbl,
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
pub struct IXMLHTTPRequest3Callback(::windows::core::IUnknown);
impl IXMLHTTPRequest3Callback {
    pub unsafe fn OnRedirect<P0, P1>(&self, pxhr: P0, pwszredirecturl: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest2>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.OnRedirect)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), pwszredirecturl.into_param().abi()).ok()
    }
    pub unsafe fn OnHeadersAvailable<P0, P1>(&self, pxhr: P0, dwstatus: u32, pwszstatus: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest2>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.OnHeadersAvailable)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), dwstatus, pwszstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnDataAvailable<P0, P1>(&self, pxhr: P0, presponsestream: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest2>,
        P1: ::windows::core::IntoParam<super::super::super::System::Com::ISequentialStream>,
    {
        (::windows::core::Interface::vtable(self).base__.OnDataAvailable)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), presponsestream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnResponseReceived<P0, P1>(&self, pxhr: P0, presponsestream: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest2>,
        P1: ::windows::core::IntoParam<super::super::super::System::Com::ISequentialStream>,
    {
        (::windows::core::Interface::vtable(self).base__.OnResponseReceived)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), presponsestream.into_param().abi()).ok()
    }
    pub unsafe fn OnError<P0>(&self, pxhr: P0, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest2>,
    {
        (::windows::core::Interface::vtable(self).base__.OnError)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), hrerror).ok()
    }
    pub unsafe fn OnServerCertificateReceived<P0>(&self, pxhr: P0, dwcertificateerrors: u32, rgservercertificatechain: &[XHR_CERT]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest3>,
    {
        (::windows::core::Interface::vtable(self).OnServerCertificateReceived)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), dwcertificateerrors, rgservercertificatechain.len() as _, ::core::mem::transmute(rgservercertificatechain.as_ptr())).ok()
    }
    pub unsafe fn OnClientCertificateRequested<P0>(&self, pxhr: P0, rgpwszissuerlist: &[*const u16]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLHTTPRequest3>,
    {
        (::windows::core::Interface::vtable(self).OnClientCertificateRequested)(::windows::core::Interface::as_raw(self), pxhr.into_param().abi(), rgpwszissuerlist.len() as _, ::core::mem::transmute(rgpwszissuerlist.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IXMLHTTPRequest3Callback, ::windows::core::IUnknown, IXMLHTTPRequest2Callback);
impl ::core::cmp::PartialEq for IXMLHTTPRequest3Callback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXMLHTTPRequest3Callback {}
impl ::core::fmt::Debug for IXMLHTTPRequest3Callback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLHTTPRequest3Callback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXMLHTTPRequest3Callback {
    type Vtable = IXMLHTTPRequest3Callback_Vtbl;
}
impl ::core::clone::Clone for IXMLHTTPRequest3Callback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXMLHTTPRequest3Callback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9e57830_8c6c_4a6f_9c13_47772bb047bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLHTTPRequest3Callback_Vtbl {
    pub base__: IXMLHTTPRequest2Callback_Vtbl,
    pub OnServerCertificateReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows::core::HRESULT,
    pub OnClientCertificateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXSLProcessor(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXSLProcessor {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setinput(&self, var: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Setinput)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(var)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn input(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).input)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerTemplate(&self) -> ::windows::core::Result<IXSLTemplate> {
        let mut result__ = ::windows::core::zeroed::<IXSLTemplate>();
        (::windows::core::Interface::vtable(self).ownerTemplate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn setStartMode<P0, P1>(&self, mode: P0, namespaceuri: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setStartMode)(::windows::core::Interface::as_raw(self), mode.into_param().abi(), namespaceuri.into_param().abi()).ok()
    }
    pub unsafe fn startMode(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).startMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn startModeURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).startModeURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Setoutput(&self, output: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Setoutput)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(output)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn output(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).output)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn transform(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).transform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn readyState(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).readyState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn addParameter<P0, P1>(&self, basename: P0, parameter: super::super::super::System::Com::VARIANT, namespaceuri: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).addParameter)(::windows::core::Interface::as_raw(self), basename.into_param().abi(), ::core::mem::transmute(parameter), namespaceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn addObject<P0, P1>(&self, obj: P0, namespaceuri: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).addObject)(::windows::core::Interface::as_raw(self), obj.into_param().abi(), namespaceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn stylesheet(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).stylesheet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXSLProcessor, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXSLProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXSLProcessor {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXSLProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXSLProcessor").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXSLProcessor {
    type Vtable = IXSLProcessor_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXSLProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXSLProcessor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf92_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXSLProcessor_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Setinput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, var: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Setinput: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub input: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    input: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ownerTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ownerTemplate: usize,
    pub setStartMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub startMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub startModeURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Setoutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, output: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Setoutput: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub output: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    output: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub transform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdone: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    transform: usize,
    pub reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub readyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preadystate: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub addParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows::core::BSTR>, parameter: super::super::super::System::Com::VARIANT, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    addParameter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub addObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, obj: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    addObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub stylesheet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesheet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    stylesheet: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXSLTemplate(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXSLTemplate {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_stylesheet<P0>(&self, stylesheet: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).putref_stylesheet)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn stylesheet(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).stylesheet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn createProcessor(&self) -> ::windows::core::Result<IXSLProcessor> {
        let mut result__ = ::windows::core::zeroed::<IXSLProcessor>();
        (::windows::core::Interface::vtable(self).createProcessor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXSLTemplate, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXSLTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXSLTemplate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXSLTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXSLTemplate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXSLTemplate {
    type Vtable = IXSLTemplate_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXSLTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXSLTemplate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf93_7b36_11d2_b20e_00c04f983e60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXSLTemplate_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_stylesheet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_stylesheet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub stylesheet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesheet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    stylesheet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub createProcessor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprocessor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    createProcessor: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXTLRuntime(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXTLRuntime {
    pub unsafe fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeValue)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeValue(&self, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeType)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.parentNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.childNodes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.firstChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.lastChild)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.previousSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.nextSibling)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNamedNodeMap>();
        (::windows::core::Interface::vtable(self).base__.attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.insertBefore)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), ::core::mem::transmute(refchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.replaceChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), oldchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.removeChild)(::windows::core::Interface::as_raw(self), childnode.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.appendChild)(::windows::core::Interface::as_raw(self), newchild.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.hasChildNodes)(::windows::core::Interface::as_raw(self), haschild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMDocument>();
        (::windows::core::Interface::vtable(self).base__.ownerDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn cloneNode<P0>(&self, deep: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.cloneNode)(::windows::core::Interface::as_raw(self), deep.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypeString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(nodetype)).ok()
    }
    pub unsafe fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.text)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(text)).ok()
    }
    pub unsafe fn Settext<P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settext)(::windows::core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.specified)(::windows::core::Interface::as_raw(self), isspecified).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn definition(&self) -> ::windows::core::Result<IXMLDOMNode> {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.definition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.nodeTypedValue)(::windows::core::Interface::as_raw(self), typedvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetnodeTypedValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(typedvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.dataType)(::windows::core::Interface::as_raw(self), datatypename).ok()
    }
    pub unsafe fn SetdataType<P0>(&self, datatypename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdataType)(::windows::core::Interface::as_raw(self), datatypename.into_param().abi()).ok()
    }
    pub unsafe fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNode)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(xmlstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectNodes<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).base__.selectNodes)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn selectSingleNode<P0>(&self, querystring: P0) -> ::windows::core::Result<IXMLDOMNode>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IXMLDOMNode>();
        (::windows::core::Interface::vtable(self).base__.selectSingleNode)(::windows::core::Interface::as_raw(self), querystring.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.parsed)(::windows::core::Interface::as_raw(self), isparsed).ok()
    }
    pub unsafe fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.namespaceURI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namespaceuri)).ok()
    }
    pub unsafe fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.prefix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prefixstring)).ok()
    }
    pub unsafe fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.baseName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(namestring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).base__.transformNodeToObject)(::windows::core::Interface::as_raw(self), stylesheet.into_param().abi(), ::core::mem::transmute(outputobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn uniqueID<P0>(&self, pnode: P0, pid: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).uniqueID)(::windows::core::Interface::as_raw(self), pnode.into_param().abi(), pid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn depth<P0>(&self, pnode: P0, pdepth: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).depth)(::windows::core::Interface::as_raw(self), pnode.into_param().abi(), pdepth).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn childNumber<P0>(&self, pnode: P0, pnumber: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).childNumber)(::windows::core::Interface::as_raw(self), pnode.into_param().abi(), pnumber).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ancestorChildNumber<P0, P1>(&self, bstrnodename: P0, pnode: P1, pnumber: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).ancestorChildNumber)(::windows::core::Interface::as_raw(self), bstrnodename.into_param().abi(), pnode.into_param().abi(), pnumber).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn absoluteChildNumber<P0>(&self, pnode: P0, pnumber: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IXMLDOMNode>,
    {
        (::windows::core::Interface::vtable(self).absoluteChildNumber)(::windows::core::Interface::as_raw(self), pnode.into_param().abi(), pnumber).ok()
    }
    pub unsafe fn formatIndex<P0>(&self, lindex: i32, bstrformat: P0, pbstrformattedstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).formatIndex)(::windows::core::Interface::as_raw(self), lindex, bstrformat.into_param().abi(), ::core::mem::transmute(pbstrformattedstring)).ok()
    }
    pub unsafe fn formatNumber<P0>(&self, dblnumber: f64, bstrformat: P0, pbstrformattedstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).formatNumber)(::windows::core::Interface::as_raw(self), dblnumber, bstrformat.into_param().abi(), ::core::mem::transmute(pbstrformattedstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn formatDate<P0>(&self, vardate: super::super::super::System::Com::VARIANT, bstrformat: P0, vardestlocale: super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).formatDate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(vardate), bstrformat.into_param().abi(), ::core::mem::transmute(vardestlocale), ::core::mem::transmute(pbstrformattedstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn formatTime<P0>(&self, vartime: super::super::super::System::Com::VARIANT, bstrformat: P0, vardestlocale: super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).formatTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(vartime), bstrformat.into_param().abi(), ::core::mem::transmute(vardestlocale), ::core::mem::transmute(pbstrformattedstring)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IXTLRuntime, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXTLRuntime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXTLRuntime {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXTLRuntime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXTLRuntime").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IXTLRuntime {
    type Vtable = IXTLRuntime_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXTLRuntime {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IXTLRuntime {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3efaa425_272f_11d2_836f_0000f87a7782);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXTLRuntime_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub uniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    uniqueID: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub depth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pdepth: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    depth: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub childNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    childNumber: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ancestorChildNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnodename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ancestorChildNumber: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub absoluteChildNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    absoluteChildNumber: usize,
    pub formatIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, bstrformat: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub formatNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dblnumber: f64, bstrformat: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub formatDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardate: super::super::super::System::Com::VARIANT, bstrformat: ::std::mem::MaybeUninit<::windows::core::BSTR>, vardestlocale: super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    formatDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub formatTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vartime: super::super::super::System::Com::VARIANT, bstrformat: ::std::mem::MaybeUninit<::windows::core::BSTR>, vardestlocale: super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    formatTime: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct XMLDOMDocumentEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl XMLDOMDocumentEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(XMLDOMDocumentEvents, ::windows::core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for XMLDOMDocumentEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for XMLDOMDocumentEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for XMLDOMDocumentEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XMLDOMDocumentEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for XMLDOMDocumentEvents {
    type Vtable = XMLDOMDocumentEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for XMLDOMDocumentEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for XMLDOMDocumentEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3efaa427_272f_11d2_836f_0000f87a7782);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct XMLDOMDocumentEvents_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ATTRIBUTE: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ATTRIBUTE_GETNAME: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ATTRIBUTE_SPECIFIED: u32 = 119u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ATTRIBUTE_VALUE: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ATTRIBUTE__TOP: u32 = 121u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_BASE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_COLLECTION_BASE: u32 = 1000000u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_COLLECTION_MAX: u32 = 2999999u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DATA: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DATA_APPEND: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DATA_DATA: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DATA_DELETE: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DATA_INSERT: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DATA_LENGTH: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DATA_REPLACE: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DATA_SUBSTRING: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DATA__TOP: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENTFRAGMENT: u32 = 94u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENTFRAGMENT__TOP: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENTTYPE: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENTTYPE_ENTITIES: u32 = 132u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENTTYPE_NAME: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENTTYPE_NOTATIONS: u32 = 133u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENTTYPE__TOP: u32 = 134u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_CREATEATTRIBUTE: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_CREATECDATASECTION: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_CREATECOMMENT: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_CREATEDOCUMENTFRAGMENT: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_CREATEELEMENT: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_CREATEENTITY: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_CREATEENTITYREFERENCE: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_CREATEPROCESSINGINSTRUCTION: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_CREATETEXTNODE: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_DOCTYPE: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_DOCUMENTELEMENT: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_GETELEMENTSBYTAGNAME: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_IMPLEMENTATION: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_DOCUMENT_TOP: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT_GETATTRIBUTE: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT_GETATTRIBUTENODE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT_GETATTRIBUTES: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT_GETELEMENTSBYTAGNAME: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT_GETTAGNAME: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT_NORMALIZE: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT_REMOVEATTRIBUTE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT_REMOVEATTRIBUTENODE: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT_SETATTRIBUTE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT_SETATTRIBUTENODE: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ELEMENT__TOP: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ENTITY: u32 = 139u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ENTITY_NOTATIONNAME: u32 = 142u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ENTITY_PUBLICID: u32 = 140u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ENTITY_SYSTEMID: u32 = 141u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ENTITY__TOP: u32 = 143u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR: u32 = 177u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR2: u32 = 186u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR2_ALLERRORS: u32 = 187u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR2_ERRORPARAMETERS: u32 = 188u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR2_ERRORPARAMETERSCOUNT: u32 = 189u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR2_ERRORXPATH: u32 = 190u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR2__TOP: u32 = 191u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERRORCOLLECTION: u32 = 192u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERRORCOLLECTION_LENGTH: u32 = 193u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERRORCOLLECTION_NEXT: u32 = 194u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERRORCOLLECTION_RESET: u32 = 195u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERRORCOLLECTION__TOP: u32 = 196u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR_ERRORCODE: u32 = 178u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR_FILEPOS: u32 = 184u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR_LINE: u32 = 182u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR_LINEPOS: u32 = 183u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR_REASON: u32 = 180u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR_SRCTEXT: u32 = 181u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR_URL: u32 = 179u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_ERROR__TOP: u32 = 185u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_IMPLEMENTATION: u32 = 144u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_IMPLEMENTATION_HASFEATURE: u32 = 145u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_IMPLEMENTATION__TOP: u32 = 146u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NAMEDNODEMAP: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NAMEDNODEMAP_GETNAMEDITEM: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NAMEDNODEMAP_REMOVENAMEDITEM: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NAMEDNODEMAP_SETNAMEDITEM: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODELIST: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODELIST_ITEM: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODELIST_LENGTH: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_APPENDCHILD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_ATTRIBUTES: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_CHILDNODES: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_CLONENODE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_FIRSTCHILD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_HASCHILDNODES: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_INSERTBEFORE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_LASTCHILD: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_NEXTSIBLING: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_NODENAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_NODETYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_NODETYPEENUM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_NODEVALUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_OWNERDOC: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_PARENTNODE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_PREVIOUSSIBLING: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_REMOVECHILD: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NODE_REPLACECHILD: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NOTATION: u32 = 135u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NOTATION_PUBLICID: u32 = 136u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NOTATION_SYSTEMID: u32 = 137u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_NOTATION__TOP: u32 = 138u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_PI: u32 = 126u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_PI_DATA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_PI_TARGET: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_PI__TOP: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_TEXT: u32 = 122u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_TEXT_JOINTEXT: u32 = 124u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_TEXT_SPLITTEXT: u32 = 123u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_TEXT__TOP: u32 = 125u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_W3CWRAPPERS: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM_W3CWRAPPERS_TOP: u32 = 143u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_DOM__TOP: u32 = 176u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER: u32 = 1418u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER_CONTENTHANDLER: u32 = 1419u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER_DTDHANDLER: u32 = 1420u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER_ENTITYRESOLVER: u32 = 1421u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER_ERRORHANDLER: u32 = 1422u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER_GETFEATURE: u32 = 1423u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER_GETPROPERTY: u32 = 1424u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER_PUTFEATURE: u32 = 1425u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER_PUTPROPERTY: u32 = 1426u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER__BASE: u32 = 1418u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MXXML_FILTER__TOP: u32 = 1427u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES: u32 = 1372u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_ADDATTRIBUTE: u32 = 1373u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_ADDATTRIBUTEFROMINDEX: u32 = 1383u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_CLEAR: u32 = 1374u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_REMOVEATTRIBUTE: u32 = 1375u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_SETATTRIBUTE: u32 = 1376u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_SETATTRIBUTES: u32 = 1377u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_SETLOCALNAME: u32 = 1378u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_SETQNAME: u32 = 1379u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_SETTYPE: u32 = 1380u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_SETURI: u32 = 1381u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES_SETVALUE: u32 = 1382u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES__BASE: u32 = 1372u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_ATTRIBUTES__TOP: u32 = 1383u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR: u32 = 1405u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_ALLOWOVERRIDE: u32 = 1406u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_DECLAREPREFIX: u32 = 1411u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_GETDECLAREDPREFIXES: u32 = 1412u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_GETPREFIXES: u32 = 1413u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_GETURI: u32 = 1414u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_GETURIFROMNODE: u32 = 1415u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_LENGTH: u32 = 1416u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_POPCONTEXT: u32 = 1410u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_PUSHCONTEXT: u32 = 1408u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_PUSHNODECONTEXT: u32 = 1409u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR_RESET: u32 = 1407u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR__BASE: u32 = 1405u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_NSMGR__TOP: u32 = 1417u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_READER_CONTROL: u32 = 1397u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_READER_CONTROL_ABORT: u32 = 1398u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_READER_CONTROL_RESUME: u32 = 1399u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_READER_CONTROL_SUSPEND: u32 = 1400u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_READER_CONTROL__BASE: u32 = 1397u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_READER_CONTROL__TOP: u32 = 1401u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_SCHEMADECLHANDLER: u32 = 1402u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_SCHEMADECLHANDLER_SCHEMAELEMENTDECL: u32 = 1403u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_SCHEMADECLHANDLER__BASE: u32 = 1402u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_SCHEMADECLHANDLER__TOP: u32 = 1404u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER: u32 = 1384u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_BYTEORDERMARK: u32 = 1388u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_DESTINATION: u32 = 1386u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_DISABLEOUTPUTESCAPING: u32 = 1393u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_ENCODING: u32 = 1387u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_FLUSH: u32 = 1394u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_INDENT: u32 = 1389u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_OMITXMLDECLARATION: u32 = 1391u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_OUTPUT: u32 = 1385u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_RESET: u32 = 1395u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_STANDALONE: u32 = 1390u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER_VERSION: u32 = 1392u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER__BASE: u32 = 1384u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_MX_WRITER__TOP: u32 = 1396u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE: u32 = 66036u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODELIST: u32 = 66136u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODELIST_CURRENT: u32 = 66139u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODELIST_ITEM: u32 = 66143u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODELIST_LENGTH: u32 = 66142u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODELIST_MOVE: u32 = 66140u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODELIST_MOVETONODE: u32 = 66141u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODELIST_NEWENUM: u32 = 66137u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODELIST_NEXT: u32 = 66138u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_ADD: u32 = 66045u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_ATTRIBUTES: u32 = 66044u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_CHILDREN: u32 = 66047u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_GETATTRIBUTE: u32 = 66042u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_NAME: u32 = 66037u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_PARENT: u32 = 66038u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_REMOVE: u32 = 66046u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_REMOVEATTRIBUTE: u32 = 66043u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_SETATTRIBUTE: u32 = 66041u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_TYPE: u32 = 66039u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_NODE_VALUE: u32 = 66040u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES: u32 = 1343u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETINDEXFROMNAME: u32 = 1348u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETINDEXFROMQNAME: u32 = 1349u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETLOCALNAME: u32 = 1346u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETQNAME: u32 = 1347u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETTYPE: u32 = 1350u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETTYPEFROMNAME: u32 = 1351u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETTYPEFROMQNAME: u32 = 1352u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETURI: u32 = 1345u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETVALUE: u32 = 1353u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETVALUEFROMNAME: u32 = 1354u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_GETVALUEFROMQNAME: u32 = 1355u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES_LENGTH: u32 = 1344u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES__BASE: u32 = 1343u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ATTRIBUTES__TOP: u32 = 1356u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER: u32 = 1321u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_CHARACTERS: u32 = 1329u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_DOCUMENTLOCATOR: u32 = 1322u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_ENDDOCUMENT: u32 = 1324u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_ENDELEMENT: u32 = 1328u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_ENDPREFIXMAPPING: u32 = 1326u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_IGNORABLEWHITESPACE: u32 = 1330u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_PROCESSINGINSTRUCTION: u32 = 1331u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_SKIPPEDENTITY: u32 = 1332u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_STARTDOCUMENT: u32 = 1323u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_STARTELEMENT: u32 = 1327u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER_STARTPREFIXMAPPING: u32 = 1325u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER__BASE: u32 = 1321u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_CONTENTHANDLER__TOP: u32 = 1333u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DECLHANDLER: u32 = 1366u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DECLHANDLER_ATTRIBUTEDECL: u32 = 1368u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DECLHANDLER_ELEMENTDECL: u32 = 1367u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DECLHANDLER_EXTERNALENTITYDECL: u32 = 1370u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DECLHANDLER_INTERNALENTITYDECL: u32 = 1369u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DECLHANDLER__BASE: u32 = 1366u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DECLHANDLER__TOP: u32 = 1371u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DTDHANDLER: u32 = 1334u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DTDHANDLER_NOTATIONDECL: u32 = 1335u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DTDHANDLER_UNPARSEDENTITYDECL: u32 = 1336u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DTDHANDLER__BASE: u32 = 1334u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_DTDHANDLER__TOP: u32 = 1337u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ENTITYRESOLVER: u32 = 1318u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ENTITYRESOLVER_RESOLVEENTITY: u32 = 1319u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ENTITYRESOLVER__BASE: u32 = 1318u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ENTITYRESOLVER__TOP: u32 = 1320u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ERRORHANDLER: u32 = 1338u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ERRORHANDLER_ERROR: u32 = 1339u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ERRORHANDLER_FATALERROR: u32 = 1340u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ERRORHANDLER_IGNORABLEWARNING: u32 = 1341u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ERRORHANDLER__BASE: u32 = 1338u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_ERRORHANDLER__TOP: u32 = 1342u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LEXICALHANDLER: u32 = 1357u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LEXICALHANDLER_COMMENT: u32 = 1364u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LEXICALHANDLER_ENDCDATA: u32 = 1363u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LEXICALHANDLER_ENDDTD: u32 = 1359u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LEXICALHANDLER_ENDENTITY: u32 = 1361u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LEXICALHANDLER_STARTCDATA: u32 = 1362u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LEXICALHANDLER_STARTDTD: u32 = 1358u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LEXICALHANDLER_STARTENTITY: u32 = 1360u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LEXICALHANDLER__BASE: u32 = 1357u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LEXICALHANDLER__TOP: u32 = 1365u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LOCATOR: u32 = 1312u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LOCATOR_COLUMNNUMBER: u32 = 1313u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LOCATOR_LINENUMBER: u32 = 1314u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LOCATOR_PUBLICID: u32 = 1315u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LOCATOR_SYSTEMID: u32 = 1316u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LOCATOR__BASE: u32 = 1312u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_LOCATOR__TOP: u32 = 1317u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER: u32 = 1296u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_BASEURL: u32 = 1305u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_CONTENTHANDLER: u32 = 1302u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_DTDHANDLER: u32 = 1303u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_ENTITYRESOLVER: u32 = 1301u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_ERRORHANDLER: u32 = 1304u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_GETFEATURE: u32 = 1297u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_GETPROPERTY: u32 = 1299u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_PARENT: u32 = 1309u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_PARSE: u32 = 1307u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_PARSEURL: u32 = 1308u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_PUTFEATURE: u32 = 1298u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_PUTPROPERTY: u32 = 1300u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER_SECUREBASEURL: u32 = 1306u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER__BASE: u32 = 1296u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLFILTER__TOP: u32 = 1311u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_BASEURL: u32 = 1290u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_CONTENTHANDLER: u32 = 1287u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_DTDHANDLER: u32 = 1288u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_ENTITYRESOLVER: u32 = 1286u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_ERRORHANDLER: u32 = 1289u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_GETFEATURE: u32 = 1282u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_GETPROPERTY: u32 = 1284u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_PARENT: u32 = 1294u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_PARSE: u32 = 1292u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_PARSEURL: u32 = 1293u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_PUTFEATURE: u32 = 1283u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_PUTPROPERTY: u32 = 1285u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER_SECUREBASEURL: u32 = 1291u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER__BASE: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER__MAX: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER__MIN: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SAX_XMLREADER__TOP: u32 = 1295u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM: u32 = 1418u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ANYATTRIBUTE: u32 = 1425u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ATTRIBUTEGROUPS: u32 = 1426u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ATTRIBUTES: u32 = 1427u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_BASETYPES: u32 = 1428u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_CONTENTMODEL: u32 = 1429u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_CONTENTTYPE: u32 = 1430u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_DEFAULTVALUE: u32 = 1431u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_DERIVEDBY: u32 = 1432u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_DISALLOWED: u32 = 1433u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ELEMENTS: u32 = 1434u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ENUMERATION: u32 = 1435u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_EXCLUSIONS: u32 = 1472u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_FIELDS: u32 = 1436u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_FINAL: u32 = 1437u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_FIXEDVALUE: u32 = 1438u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_FRACTIONDIGITS: u32 = 1439u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_GETDECLARATION: u32 = 1422u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_GETSCHEMA: u32 = 1421u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ID: u32 = 1440u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_IDCONSTRAINTS: u32 = 1441u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ISABSTRACT: u32 = 1442u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ISNILLABLE: u32 = 1443u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ISREFERENCE: u32 = 1444u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ISVALID: u32 = 1445u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ITEMBYNAME: u32 = 1423u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ITEMBYQNAME: u32 = 1424u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_ITEMTYPE: u32 = 1446u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_LENGTH: u32 = 1447u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_MAXEXCLUSIVE: u32 = 1448u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_MAXINCLUSIVE: u32 = 1449u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_MAXLENGTH: u32 = 1450u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_MAXOCCURS: u32 = 1451u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_MINEXCLUSIVE: u32 = 1452u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_MININCLUSIVE: u32 = 1453u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_MINLENGTH: u32 = 1454u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_MINOCCURS: u32 = 1455u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_MODELGROUPS: u32 = 1456u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_NAME: u32 = 1457u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_NAMESPACES: u32 = 1458u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_NAMESPACEURI: u32 = 1459u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_NOTATIONS: u32 = 1460u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_PARTICLES: u32 = 1461u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_PATTERNS: u32 = 1462u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_PROCESSCONTENTS: u32 = 1463u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_PROHIBITED: u32 = 1464u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_PUBLICIDENTIFIER: u32 = 1465u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_REFERENCEDKEY: u32 = 1466u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_SCHEMA: u32 = 1467u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_SCHEMALOCATIONS: u32 = 1468u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_SCOPE: u32 = 1469u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_SELECTOR: u32 = 1470u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_SUBSTITUTIONGROUP: u32 = 1471u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_SYSTEMIDENTIFIER: u32 = 1473u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_TARGETNAMESPACE: u32 = 1474u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_TOP: u32 = 1484u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_TOTALDIGITS: u32 = 1475u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_TYPE: u32 = 1476u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_TYPES: u32 = 1477u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_UNHANDLEDATTRS: u32 = 1478u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_USE: u32 = 1479u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_VALIDATE: u32 = 1419u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_VALIDATEONLOAD: u32 = 1420u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_VARIETY: u32 = 1480u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_VERSION: u32 = 1481u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_WHITESPACE: u32 = 1482u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_SOM_WRITEANNOTATION: u32 = 1483u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLATTRIBUTE: u32 = 65936u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLATTRIBUTE_NAME: u32 = 65937u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLATTRIBUTE_VALUE: u32 = 65938u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT: u32 = 65636u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_ASYNC: u32 = 65649u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_BASEURL: u32 = 65651u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_CASEINSENSITIVE: u32 = 65650u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_CHARSET: u32 = 65645u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_COMMIT: u32 = 65655u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_CREATEELEMENT: u32 = 65644u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_DOCTYPE: u32 = 65647u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_DTDURL: u32 = 65648u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_FILEMODIFIEDDATE: u32 = 65639u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_FILESIZE: u32 = 65638u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_FILEUPDATEDDATE: u32 = 65640u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_LASTERROR: u32 = 65653u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_MIMETYPE: u32 = 65642u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_READYSTATE: u32 = 65643u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_ROOT: u32 = 65637u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_TRIMWHITESPACE: u32 = 65654u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_URL: u32 = 65641u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_VERSION: u32 = 65646u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOCUMENT_XML: u32 = 65652u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOMEVENT: u32 = 197u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOMEVENT_ONDATAAVAILABLE: u32 = 198u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOMEVENT_ONREADYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOMEVENT__TOP: u32 = 199u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT2: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT2_GETPROPERTY: u32 = 205u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT2_NAMESPACES: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT2_SCHEMAS: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT2_SETPROPERTY: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT2_VALIDATE: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT2__TOP: u32 = 206u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT3: u32 = 207u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT3_IMPORTNODE: u32 = 209u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT3_VALIDATENODE: u32 = 208u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT3__TOP: u32 = 210u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_ABORT: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_ASYNC: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_CREATENODE: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_CREATENODEEX: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_DOCUMENTNAMESPACES: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_DOCUMENTNODE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_LOAD: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_LOADXML: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_NODEFROMID: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_ONDATAAVAILABLE: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_ONREADYSTATECHANGE: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_ONTRANSFORMNODE: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_PARSEERROR: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_PRESERVEWHITESPACE: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_RESOLVENAMESPACE: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_SAVE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_URL: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT_VALIDATE: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_DOCUMENT__TOP: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NAMEDNODEMAP: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NAMEDNODEMAP_GETQUALIFIEDITEM: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NAMEDNODEMAP_NEWENUM: u32 = 91u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NAMEDNODEMAP_NEXTNODE: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NAMEDNODEMAP_REMOVEQUALIFIEDITEM: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NAMEDNODEMAP_RESET: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NAMEDNODEMAP__TOP: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODELIST: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODELIST_NEWENUM: u32 = 78u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODELIST_NEXTNODE: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODELIST_RESET: u32 = 77u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODELIST__TOP: u32 = 79u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_BASENAME: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_DATATYPE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_DEFINITION: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_NAMESPACE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_NODETYPEDVALUE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_PARSED: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_PREFIX: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_SELECTNODES: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_SELECTSINGLENODE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_SPECIFIED: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_STRINGTYPE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_TEXT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_TRANSFORMNODE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_TRANSFORMNODETOOBJECT: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE_XML: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_NODE__TOP: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_ADDOBJECT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_ADDPARAMETER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_INPUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_OUTPUT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_READYSTATE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_RESET: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_SETSTARTMODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_STARTMODE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_STARTMODEURI: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_STYLESHEET: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_TRANSFORM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR_XSLTEMPLATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_PROCESSOR__TOP: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SCHEMACOLLECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SCHEMACOLLECTION_ADD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SCHEMACOLLECTION_ADDCOLLECTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SCHEMACOLLECTION_GET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SCHEMACOLLECTION_LENGTH: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SCHEMACOLLECTION_NAMESPACEURI: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SCHEMACOLLECTION_REMOVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SCHEMACOLLECTION__TOP: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION_CLONE: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION_CONTEXT: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION_EXPR: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION_GETPROPERTY: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION_MATCHES: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION_PEEKNODE: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION_REMOVEALL: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION_REMOVENEXT: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION_SETPROPERTY: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_SELECTION__TOP: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_TEMPLATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_TEMPLATE_CREATEPROCESSOR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_TEMPLATE_STYLESHEET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDOM_TEMPLATE__TOP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG_CREATEKEYFROMCSP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG_CREATEKEYFROMHMACSECRET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG_CREATEKEYFROMNODE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG_CREATESAXPROXY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG_GETVERIFYINGCERTIFICATE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG_SETREFERENCEDATA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG_SIGN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG_SIGNATURE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG_STORE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSIG_VERIFY: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSO: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSO_DOCUMENT: u32 = 65537u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLDSO_JAVADSOCOMPATIBLE: u32 = 65538u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT: u32 = 65736u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENTCOLLECTION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENTCOLLECTION_ITEM: u32 = 65539u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENTCOLLECTION_LENGTH: u32 = 65537u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENTCOLLECTION_NEWENUM: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_ADDCHILD: u32 = 65745u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_ATTRIBUTES: u32 = 65747u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_CHILDREN: u32 = 65742u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_GETATTRIBUTE: u32 = 65740u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_PARENT: u32 = 65738u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_REMOVEATTRIBUTE: u32 = 65741u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_REMOVECHILD: u32 = 65746u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_SETATTRIBUTE: u32 = 65739u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_TAGNAME: u32 = 65737u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_TEXT: u32 = 65744u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLELEMENT_TYPE: u32 = 65743u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLERROR: u32 = 65936u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLERROR_LINE: u32 = 65938u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLERROR_POS: u32 = 65939u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLERROR_REASON: u32 = 65937u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLNOTIFSINK: u32 = 65836u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XMLNOTIFSINK_CHILDADDED: u32 = 65837u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XOBJ_BASE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XOBJ_MAX: u32 = 131071u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XOBJ_MIN: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME: u32 = 186u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME_ABSOLUTECHILDNUMBER: u32 = 191u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME_ANCESTORCHILDNUMBER: u32 = 190u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME_CHILDNUMBER: u32 = 189u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME_DEPTH: u32 = 188u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME_FORMATDATE: u32 = 194u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME_FORMATINDEX: u32 = 192u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME_FORMATNUMBER: u32 = 193u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME_FORMATTIME: u32 = 195u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME_UNIQUEID: u32 = 187u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DISPID_XTLRUNTIME__TOP: u32 = 196u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DOMDocument: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf90_7b36_11d2_b20e_00c04f983e60);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DOMDocument60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a05_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const DOMFreeThreadedDocument: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2933bf91_7b36_11d2_b20e_00c04f983e60);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const E_XML_BUFFERTOOSMALL: i32 = -1072897498i32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const E_XML_INVALID: i32 = -1072897499i32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const E_XML_NODTD: i32 = -1072897500i32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const E_XML_NOTWF: i32 = -1072897501i32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const FreeThreadedDOMDocument60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a06_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const FreeThreadedXMLHTTP60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a09_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const MXHTMLWriter60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a10_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const MXNamespaceManager60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a11_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const MXXMLWriter60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a0f_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SAXAttributes60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a0e_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SAXXMLReader60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a0c_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const ServerXMLHTTP60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a0b_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_ONDATA_ALWAYS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_ONDATA_NEVER: u64 = 18446744073709551615u64;
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLDSOControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x550dda30_0541_11d2_9ca9_0060b0ec3d39);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLDocument: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfc399af_d876_11d0_9c10_00c04fc99c8e);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLHTTP60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a0a_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLHTTPRequest: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed8c108e_4349_11d2_91a4_00c04f7969e8);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLSchemaCache60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a07_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XSLTemplate60: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d96a08_f192_11d4_a65f_0040963251e5);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOMNodeType(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_INVALID: DOMNodeType = DOMNodeType(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_ELEMENT: DOMNodeType = DOMNodeType(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_ATTRIBUTE: DOMNodeType = DOMNodeType(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_TEXT: DOMNodeType = DOMNodeType(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_CDATA_SECTION: DOMNodeType = DOMNodeType(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_ENTITY_REFERENCE: DOMNodeType = DOMNodeType(5i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_ENTITY: DOMNodeType = DOMNodeType(6i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_PROCESSING_INSTRUCTION: DOMNodeType = DOMNodeType(7i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_COMMENT: DOMNodeType = DOMNodeType(8i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_DOCUMENT: DOMNodeType = DOMNodeType(9i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_DOCUMENT_TYPE: DOMNodeType = DOMNodeType(10i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_DOCUMENT_FRAGMENT: DOMNodeType = DOMNodeType(11i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const NODE_NOTATION: DOMNodeType = DOMNodeType(12i32);
impl ::core::marker::Copy for DOMNodeType {}
impl ::core::clone::Clone for DOMNodeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOMNodeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOMNodeType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOMNodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOMNodeType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCHEMACONTENTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMACONTENTTYPE_EMPTY: SCHEMACONTENTTYPE = SCHEMACONTENTTYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMACONTENTTYPE_TEXTONLY: SCHEMACONTENTTYPE = SCHEMACONTENTTYPE(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMACONTENTTYPE_ELEMENTONLY: SCHEMACONTENTTYPE = SCHEMACONTENTTYPE(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMACONTENTTYPE_MIXED: SCHEMACONTENTTYPE = SCHEMACONTENTTYPE(3i32);
impl ::core::marker::Copy for SCHEMACONTENTTYPE {}
impl ::core::clone::Clone for SCHEMACONTENTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCHEMACONTENTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCHEMACONTENTTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCHEMACONTENTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCHEMACONTENTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCHEMADERIVATIONMETHOD(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMADERIVATIONMETHOD_EMPTY: SCHEMADERIVATIONMETHOD = SCHEMADERIVATIONMETHOD(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMADERIVATIONMETHOD_SUBSTITUTION: SCHEMADERIVATIONMETHOD = SCHEMADERIVATIONMETHOD(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMADERIVATIONMETHOD_EXTENSION: SCHEMADERIVATIONMETHOD = SCHEMADERIVATIONMETHOD(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMADERIVATIONMETHOD_RESTRICTION: SCHEMADERIVATIONMETHOD = SCHEMADERIVATIONMETHOD(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMADERIVATIONMETHOD_LIST: SCHEMADERIVATIONMETHOD = SCHEMADERIVATIONMETHOD(8i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMADERIVATIONMETHOD_UNION: SCHEMADERIVATIONMETHOD = SCHEMADERIVATIONMETHOD(16i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMADERIVATIONMETHOD_ALL: SCHEMADERIVATIONMETHOD = SCHEMADERIVATIONMETHOD(255i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMADERIVATIONMETHOD_NONE: SCHEMADERIVATIONMETHOD = SCHEMADERIVATIONMETHOD(256i32);
impl ::core::marker::Copy for SCHEMADERIVATIONMETHOD {}
impl ::core::clone::Clone for SCHEMADERIVATIONMETHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCHEMADERIVATIONMETHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCHEMADERIVATIONMETHOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCHEMADERIVATIONMETHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCHEMADERIVATIONMETHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCHEMAPROCESSCONTENTS(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAPROCESSCONTENTS_NONE: SCHEMAPROCESSCONTENTS = SCHEMAPROCESSCONTENTS(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAPROCESSCONTENTS_SKIP: SCHEMAPROCESSCONTENTS = SCHEMAPROCESSCONTENTS(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAPROCESSCONTENTS_LAX: SCHEMAPROCESSCONTENTS = SCHEMAPROCESSCONTENTS(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAPROCESSCONTENTS_STRICT: SCHEMAPROCESSCONTENTS = SCHEMAPROCESSCONTENTS(3i32);
impl ::core::marker::Copy for SCHEMAPROCESSCONTENTS {}
impl ::core::clone::Clone for SCHEMAPROCESSCONTENTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCHEMAPROCESSCONTENTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCHEMAPROCESSCONTENTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCHEMAPROCESSCONTENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCHEMAPROCESSCONTENTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCHEMATYPEVARIETY(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMATYPEVARIETY_NONE: SCHEMATYPEVARIETY = SCHEMATYPEVARIETY(-1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMATYPEVARIETY_ATOMIC: SCHEMATYPEVARIETY = SCHEMATYPEVARIETY(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMATYPEVARIETY_LIST: SCHEMATYPEVARIETY = SCHEMATYPEVARIETY(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMATYPEVARIETY_UNION: SCHEMATYPEVARIETY = SCHEMATYPEVARIETY(2i32);
impl ::core::marker::Copy for SCHEMATYPEVARIETY {}
impl ::core::clone::Clone for SCHEMATYPEVARIETY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCHEMATYPEVARIETY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCHEMATYPEVARIETY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCHEMATYPEVARIETY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCHEMATYPEVARIETY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCHEMAUSE(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAUSE_OPTIONAL: SCHEMAUSE = SCHEMAUSE(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAUSE_PROHIBITED: SCHEMAUSE = SCHEMAUSE(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAUSE_REQUIRED: SCHEMAUSE = SCHEMAUSE(2i32);
impl ::core::marker::Copy for SCHEMAUSE {}
impl ::core::clone::Clone for SCHEMAUSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCHEMAUSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCHEMAUSE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCHEMAUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCHEMAUSE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCHEMAWHITESPACE(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAWHITESPACE_NONE: SCHEMAWHITESPACE = SCHEMAWHITESPACE(-1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAWHITESPACE_PRESERVE: SCHEMAWHITESPACE = SCHEMAWHITESPACE(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAWHITESPACE_REPLACE: SCHEMAWHITESPACE = SCHEMAWHITESPACE(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SCHEMAWHITESPACE_COLLAPSE: SCHEMAWHITESPACE = SCHEMAWHITESPACE(2i32);
impl ::core::marker::Copy for SCHEMAWHITESPACE {}
impl ::core::clone::Clone for SCHEMAWHITESPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCHEMAWHITESPACE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCHEMAWHITESPACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCHEMAWHITESPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCHEMAWHITESPACE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVERXMLHTTP_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_OPTION_URL: SERVERXMLHTTP_OPTION = SERVERXMLHTTP_OPTION(-1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_OPTION_URL_CODEPAGE: SERVERXMLHTTP_OPTION = SERVERXMLHTTP_OPTION(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_OPTION_ESCAPE_PERCENT_IN_URL: SERVERXMLHTTP_OPTION = SERVERXMLHTTP_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_OPTION_IGNORE_SERVER_SSL_CERT_ERROR_FLAGS: SERVERXMLHTTP_OPTION = SERVERXMLHTTP_OPTION(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_OPTION_SELECT_CLIENT_SSL_CERT: SERVERXMLHTTP_OPTION = SERVERXMLHTTP_OPTION(3i32);
impl ::core::marker::Copy for SERVERXMLHTTP_OPTION {}
impl ::core::clone::Clone for SERVERXMLHTTP_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVERXMLHTTP_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SERVERXMLHTTP_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SERVERXMLHTTP_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVERXMLHTTP_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SOMITEMTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_SCHEMA: SOMITEMTYPE = SOMITEMTYPE(4096i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_ATTRIBUTE: SOMITEMTYPE = SOMITEMTYPE(4097i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_ATTRIBUTEGROUP: SOMITEMTYPE = SOMITEMTYPE(4098i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_NOTATION: SOMITEMTYPE = SOMITEMTYPE(4099i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_ANNOTATION: SOMITEMTYPE = SOMITEMTYPE(4100i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_IDENTITYCONSTRAINT: SOMITEMTYPE = SOMITEMTYPE(4352i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_KEY: SOMITEMTYPE = SOMITEMTYPE(4353i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_KEYREF: SOMITEMTYPE = SOMITEMTYPE(4354i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_UNIQUE: SOMITEMTYPE = SOMITEMTYPE(4355i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_ANYTYPE: SOMITEMTYPE = SOMITEMTYPE(8192i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE: SOMITEMTYPE = SOMITEMTYPE(8448i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_ANYTYPE: SOMITEMTYPE = SOMITEMTYPE(8449i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_ANYURI: SOMITEMTYPE = SOMITEMTYPE(8450i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_BASE64BINARY: SOMITEMTYPE = SOMITEMTYPE(8451i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_BOOLEAN: SOMITEMTYPE = SOMITEMTYPE(8452i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_BYTE: SOMITEMTYPE = SOMITEMTYPE(8453i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_DATE: SOMITEMTYPE = SOMITEMTYPE(8454i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_DATETIME: SOMITEMTYPE = SOMITEMTYPE(8455i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_DAY: SOMITEMTYPE = SOMITEMTYPE(8456i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_DECIMAL: SOMITEMTYPE = SOMITEMTYPE(8457i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_DOUBLE: SOMITEMTYPE = SOMITEMTYPE(8458i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_DURATION: SOMITEMTYPE = SOMITEMTYPE(8459i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_ENTITIES: SOMITEMTYPE = SOMITEMTYPE(8460i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_ENTITY: SOMITEMTYPE = SOMITEMTYPE(8461i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_FLOAT: SOMITEMTYPE = SOMITEMTYPE(8462i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_HEXBINARY: SOMITEMTYPE = SOMITEMTYPE(8463i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_ID: SOMITEMTYPE = SOMITEMTYPE(8464i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_IDREF: SOMITEMTYPE = SOMITEMTYPE(8465i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_IDREFS: SOMITEMTYPE = SOMITEMTYPE(8466i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_INT: SOMITEMTYPE = SOMITEMTYPE(8467i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_INTEGER: SOMITEMTYPE = SOMITEMTYPE(8468i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_LANGUAGE: SOMITEMTYPE = SOMITEMTYPE(8469i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_LONG: SOMITEMTYPE = SOMITEMTYPE(8470i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_MONTH: SOMITEMTYPE = SOMITEMTYPE(8471i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_MONTHDAY: SOMITEMTYPE = SOMITEMTYPE(8472i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_NAME: SOMITEMTYPE = SOMITEMTYPE(8473i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_NCNAME: SOMITEMTYPE = SOMITEMTYPE(8474i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_NEGATIVEINTEGER: SOMITEMTYPE = SOMITEMTYPE(8475i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_NMTOKEN: SOMITEMTYPE = SOMITEMTYPE(8476i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_NMTOKENS: SOMITEMTYPE = SOMITEMTYPE(8477i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_NONNEGATIVEINTEGER: SOMITEMTYPE = SOMITEMTYPE(8478i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_NONPOSITIVEINTEGER: SOMITEMTYPE = SOMITEMTYPE(8479i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_NORMALIZEDSTRING: SOMITEMTYPE = SOMITEMTYPE(8480i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_NOTATION: SOMITEMTYPE = SOMITEMTYPE(8481i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_POSITIVEINTEGER: SOMITEMTYPE = SOMITEMTYPE(8482i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_QNAME: SOMITEMTYPE = SOMITEMTYPE(8483i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_SHORT: SOMITEMTYPE = SOMITEMTYPE(8484i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_STRING: SOMITEMTYPE = SOMITEMTYPE(8485i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_TIME: SOMITEMTYPE = SOMITEMTYPE(8486i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_TOKEN: SOMITEMTYPE = SOMITEMTYPE(8487i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_UNSIGNEDBYTE: SOMITEMTYPE = SOMITEMTYPE(8488i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_UNSIGNEDINT: SOMITEMTYPE = SOMITEMTYPE(8489i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_UNSIGNEDLONG: SOMITEMTYPE = SOMITEMTYPE(8490i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_UNSIGNEDSHORT: SOMITEMTYPE = SOMITEMTYPE(8491i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_YEAR: SOMITEMTYPE = SOMITEMTYPE(8492i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_YEARMONTH: SOMITEMTYPE = SOMITEMTYPE(8493i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_DATATYPE_ANYSIMPLETYPE: SOMITEMTYPE = SOMITEMTYPE(8703i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_SIMPLETYPE: SOMITEMTYPE = SOMITEMTYPE(8704i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_COMPLEXTYPE: SOMITEMTYPE = SOMITEMTYPE(9216i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_PARTICLE: SOMITEMTYPE = SOMITEMTYPE(16384i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_ANY: SOMITEMTYPE = SOMITEMTYPE(16385i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_ANYATTRIBUTE: SOMITEMTYPE = SOMITEMTYPE(16386i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_ELEMENT: SOMITEMTYPE = SOMITEMTYPE(16387i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_GROUP: SOMITEMTYPE = SOMITEMTYPE(16640i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_ALL: SOMITEMTYPE = SOMITEMTYPE(16641i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_CHOICE: SOMITEMTYPE = SOMITEMTYPE(16642i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_SEQUENCE: SOMITEMTYPE = SOMITEMTYPE(16643i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_EMPTYPARTICLE: SOMITEMTYPE = SOMITEMTYPE(16644i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_NULL: SOMITEMTYPE = SOMITEMTYPE(2048i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_NULL_TYPE: SOMITEMTYPE = SOMITEMTYPE(10240i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_NULL_ANY: SOMITEMTYPE = SOMITEMTYPE(18433i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_NULL_ANYATTRIBUTE: SOMITEMTYPE = SOMITEMTYPE(18434i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SOMITEM_NULL_ELEMENT: SOMITEMTYPE = SOMITEMTYPE(18435i32);
impl ::core::marker::Copy for SOMITEMTYPE {}
impl ::core::clone::Clone for SOMITEMTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SOMITEMTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SOMITEMTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SOMITEMTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOMITEMTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SXH_PROXY_SETTING(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_PROXY_SET_DEFAULT: SXH_PROXY_SETTING = SXH_PROXY_SETTING(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_PROXY_SET_PRECONFIG: SXH_PROXY_SETTING = SXH_PROXY_SETTING(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_PROXY_SET_DIRECT: SXH_PROXY_SETTING = SXH_PROXY_SETTING(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_PROXY_SET_PROXY: SXH_PROXY_SETTING = SXH_PROXY_SETTING(2i32);
impl ::core::marker::Copy for SXH_PROXY_SETTING {}
impl ::core::clone::Clone for SXH_PROXY_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SXH_PROXY_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SXH_PROXY_SETTING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SXH_PROXY_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SXH_PROXY_SETTING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SXH_SERVER_CERT_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_SERVER_CERT_IGNORE_UNKNOWN_CA: SXH_SERVER_CERT_OPTION = SXH_SERVER_CERT_OPTION(256i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_SERVER_CERT_IGNORE_WRONG_USAGE: SXH_SERVER_CERT_OPTION = SXH_SERVER_CERT_OPTION(512i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_SERVER_CERT_IGNORE_CERT_CN_INVALID: SXH_SERVER_CERT_OPTION = SXH_SERVER_CERT_OPTION(4096i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_SERVER_CERT_IGNORE_CERT_DATE_INVALID: SXH_SERVER_CERT_OPTION = SXH_SERVER_CERT_OPTION(8192i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const SXH_SERVER_CERT_IGNORE_ALL_SERVER_ERRORS: SXH_SERVER_CERT_OPTION = SXH_SERVER_CERT_OPTION(13056i32);
impl ::core::marker::Copy for SXH_SERVER_CERT_OPTION {}
impl ::core::clone::Clone for SXH_SERVER_CERT_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SXH_SERVER_CERT_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SXH_SERVER_CERT_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SXH_SERVER_CERT_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SXH_SERVER_CERT_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XHR_AUTH(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_AUTH_ALL: XHR_AUTH = XHR_AUTH(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_AUTH_NONE: XHR_AUTH = XHR_AUTH(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_AUTH_PROXY: XHR_AUTH = XHR_AUTH(2i32);
impl ::core::marker::Copy for XHR_AUTH {}
impl ::core::clone::Clone for XHR_AUTH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XHR_AUTH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for XHR_AUTH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XHR_AUTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XHR_AUTH").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XHR_CERT_ERROR_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CERT_ERROR_REVOCATION_FAILED: XHR_CERT_ERROR_FLAG = XHR_CERT_ERROR_FLAG(8388608u32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CERT_ERROR_UNKNOWN_CA: XHR_CERT_ERROR_FLAG = XHR_CERT_ERROR_FLAG(16777216u32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CERT_ERROR_CERT_CN_INVALID: XHR_CERT_ERROR_FLAG = XHR_CERT_ERROR_FLAG(33554432u32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CERT_ERROR_CERT_DATE_INVALID: XHR_CERT_ERROR_FLAG = XHR_CERT_ERROR_FLAG(67108864u32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CERT_ERROR_ALL_SERVER_ERRORS: XHR_CERT_ERROR_FLAG = XHR_CERT_ERROR_FLAG(125829120u32);
impl ::core::marker::Copy for XHR_CERT_ERROR_FLAG {}
impl ::core::clone::Clone for XHR_CERT_ERROR_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XHR_CERT_ERROR_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for XHR_CERT_ERROR_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XHR_CERT_ERROR_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XHR_CERT_ERROR_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XHR_CERT_IGNORE_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CERT_IGNORE_REVOCATION_FAILED: XHR_CERT_IGNORE_FLAG = XHR_CERT_IGNORE_FLAG(128u32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CERT_IGNORE_UNKNOWN_CA: XHR_CERT_IGNORE_FLAG = XHR_CERT_IGNORE_FLAG(256u32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CERT_IGNORE_CERT_CN_INVALID: XHR_CERT_IGNORE_FLAG = XHR_CERT_IGNORE_FLAG(4096u32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CERT_IGNORE_CERT_DATE_INVALID: XHR_CERT_IGNORE_FLAG = XHR_CERT_IGNORE_FLAG(8192u32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CERT_IGNORE_ALL_SERVER_ERRORS: XHR_CERT_IGNORE_FLAG = XHR_CERT_IGNORE_FLAG(12672u32);
impl ::core::marker::Copy for XHR_CERT_IGNORE_FLAG {}
impl ::core::clone::Clone for XHR_CERT_IGNORE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XHR_CERT_IGNORE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for XHR_CERT_IGNORE_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XHR_CERT_IGNORE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XHR_CERT_IGNORE_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XHR_COOKIE_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_IS_SECURE: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_IS_SESSION: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_THIRD_PARTY: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_PROMPT_REQUIRED: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_EVALUATE_P3P: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(64i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_APPLY_P3P: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(128i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_P3P_ENABLED: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(256i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_IS_RESTRICTED: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(512i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_IE6: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(1024i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_IS_LEGACY: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(2048i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_NON_SCRIPT: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(4096i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_HTTPONLY: XHR_COOKIE_FLAG = XHR_COOKIE_FLAG(8192i32);
impl ::core::marker::Copy for XHR_COOKIE_FLAG {}
impl ::core::clone::Clone for XHR_COOKIE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XHR_COOKIE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for XHR_COOKIE_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XHR_COOKIE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XHR_COOKIE_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XHR_COOKIE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_STATE_UNKNOWN: XHR_COOKIE_STATE = XHR_COOKIE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_STATE_ACCEPT: XHR_COOKIE_STATE = XHR_COOKIE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_STATE_PROMPT: XHR_COOKIE_STATE = XHR_COOKIE_STATE(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_STATE_LEASH: XHR_COOKIE_STATE = XHR_COOKIE_STATE(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_STATE_DOWNGRADE: XHR_COOKIE_STATE = XHR_COOKIE_STATE(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_COOKIE_STATE_REJECT: XHR_COOKIE_STATE = XHR_COOKIE_STATE(5i32);
impl ::core::marker::Copy for XHR_COOKIE_STATE {}
impl ::core::clone::Clone for XHR_COOKIE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XHR_COOKIE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for XHR_COOKIE_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XHR_COOKIE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XHR_COOKIE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XHR_CRED_PROMPT(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CRED_PROMPT_ALL: XHR_CRED_PROMPT = XHR_CRED_PROMPT(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CRED_PROMPT_NONE: XHR_CRED_PROMPT = XHR_CRED_PROMPT(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_CRED_PROMPT_PROXY: XHR_CRED_PROMPT = XHR_CRED_PROMPT(2i32);
impl ::core::marker::Copy for XHR_CRED_PROMPT {}
impl ::core::clone::Clone for XHR_CRED_PROMPT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XHR_CRED_PROMPT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for XHR_CRED_PROMPT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XHR_CRED_PROMPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XHR_CRED_PROMPT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XHR_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_NO_CRED_PROMPT: XHR_PROPERTY = XHR_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_NO_AUTH: XHR_PROPERTY = XHR_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_TIMEOUT: XHR_PROPERTY = XHR_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_NO_DEFAULT_HEADERS: XHR_PROPERTY = XHR_PROPERTY(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_REPORT_REDIRECT_STATUS: XHR_PROPERTY = XHR_PROPERTY(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_NO_CACHE: XHR_PROPERTY = XHR_PROPERTY(5i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_EXTENDED_ERROR: XHR_PROPERTY = XHR_PROPERTY(6i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_QUERY_STRING_UTF8: XHR_PROPERTY = XHR_PROPERTY(7i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_IGNORE_CERT_ERRORS: XHR_PROPERTY = XHR_PROPERTY(8i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_ONDATA_THRESHOLD: XHR_PROPERTY = XHR_PROPERTY(9i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_SET_ENTERPRISEID: XHR_PROPERTY = XHR_PROPERTY(10i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XHR_PROP_MAX_CONNECTIONS: XHR_PROPERTY = XHR_PROPERTY(11i32);
impl ::core::marker::Copy for XHR_PROPERTY {}
impl ::core::clone::Clone for XHR_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XHR_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for XHR_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XHR_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XHR_PROPERTY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XMLELEM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLELEMTYPE_ELEMENT: XMLELEM_TYPE = XMLELEM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLELEMTYPE_TEXT: XMLELEM_TYPE = XMLELEM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLELEMTYPE_COMMENT: XMLELEM_TYPE = XMLELEM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLELEMTYPE_DOCUMENT: XMLELEM_TYPE = XMLELEM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLELEMTYPE_DTD: XMLELEM_TYPE = XMLELEM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLELEMTYPE_PI: XMLELEM_TYPE = XMLELEM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub const XMLELEMTYPE_OTHER: XMLELEM_TYPE = XMLELEM_TYPE(6i32);
impl ::core::marker::Copy for XMLELEM_TYPE {}
impl ::core::clone::Clone for XMLELEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XMLELEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for XMLELEM_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XMLELEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XMLELEM_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub struct XHR_CERT {
    pub cbCert: u32,
    pub pbCert: *mut u8,
}
impl ::core::marker::Copy for XHR_CERT {}
impl ::core::clone::Clone for XHR_CERT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XHR_CERT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XHR_CERT").field("cbCert", &self.cbCert).field("pbCert", &self.pbCert).finish()
    }
}
impl ::windows::core::TypeKind for XHR_CERT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for XHR_CERT {
    fn eq(&self, other: &Self) -> bool {
        self.cbCert == other.cbCert && self.pbCert == other.pbCert
    }
}
impl ::core::cmp::Eq for XHR_CERT {}
impl ::core::default::Default for XHR_CERT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct XHR_COOKIE {
    pub pwszUrl: ::windows::core::PWSTR,
    pub pwszName: ::windows::core::PWSTR,
    pub pwszValue: ::windows::core::PWSTR,
    pub pwszP3PPolicy: ::windows::core::PWSTR,
    pub ftExpires: super::super::super::Foundation::FILETIME,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XHR_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XHR_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for XHR_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XHR_COOKIE").field("pwszUrl", &self.pwszUrl).field("pwszName", &self.pwszName).field("pwszValue", &self.pwszValue).field("pwszP3PPolicy", &self.pwszP3PPolicy).field("ftExpires", &self.ftExpires).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for XHR_COOKIE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for XHR_COOKIE {
    fn eq(&self, other: &Self) -> bool {
        self.pwszUrl == other.pwszUrl && self.pwszName == other.pwszName && self.pwszValue == other.pwszValue && self.pwszP3PPolicy == other.pwszP3PPolicy && self.ftExpires == other.ftExpires && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for XHR_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XHR_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub struct XML_ERROR {
    pub _nLine: u32,
    pub _pchBuf: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub _cchBuf: u32,
    pub _ich: u32,
    pub _pszFound: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub _pszExpected: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub _reserved1: u32,
    pub _reserved2: u32,
}
impl ::core::clone::Clone for XML_ERROR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for XML_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XML_ERROR").field("_nLine", &self._nLine).field("_pchBuf", &self._pchBuf).field("_cchBuf", &self._cchBuf).field("_ich", &self._ich).field("_pszFound", &self._pszFound).field("_pszExpected", &self._pszExpected).field("_reserved1", &self._reserved1).field("_reserved2", &self._reserved2).finish()
    }
}
impl ::windows::core::TypeKind for XML_ERROR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for XML_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self._nLine == other._nLine && self._pchBuf == other._pchBuf && self._cchBuf == other._cchBuf && self._ich == other._ich && self._pszFound == other._pszFound && self._pszExpected == other._pszExpected && self._reserved1 == other._reserved1 && self._reserved2 == other._reserved2
    }
}
impl ::core::cmp::Eq for XML_ERROR {}
impl ::core::default::Default for XML_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
pub struct __msxml6_ReferenceRemainingTypes__ {
    pub __tagDomNodeType__: DOMNodeType,
    pub __domNodeType__: DOMNodeType,
    pub __serverXmlHttpOptionEnum__: SERVERXMLHTTP_OPTION,
    pub __serverXmlHttpOption__: SERVERXMLHTTP_OPTION,
    pub __serverCertOptionEnum__: SXH_SERVER_CERT_OPTION,
    pub __serverCertOption__: SXH_SERVER_CERT_OPTION,
    pub __proxySettingEnum__: SXH_PROXY_SETTING,
    pub __proxySetting__: SXH_PROXY_SETTING,
    pub __somItemTypeEnum__: SOMITEMTYPE,
    pub __somItemType__: SOMITEMTYPE,
    pub __schemaUseEnum__: SCHEMAUSE,
    pub __schemaUse__: SCHEMAUSE,
    pub __schemaDerivationMethodEnum__: SCHEMADERIVATIONMETHOD,
    pub __schemaDerivationMethod__: SCHEMADERIVATIONMETHOD,
    pub __schemaContentTypeEnum__: SCHEMACONTENTTYPE,
    pub __schemaContentType__: SCHEMACONTENTTYPE,
    pub __schemaProcessContentsEnum__: SCHEMAPROCESSCONTENTS,
    pub __schemaProcessContents__: SCHEMAPROCESSCONTENTS,
    pub __schemaWhitespaceEnum__: SCHEMAWHITESPACE,
    pub __schemaWhitespace__: SCHEMAWHITESPACE,
    pub __schemaTypeVarietyEnum__: SCHEMATYPEVARIETY,
    pub __schemaTypeVariety__: SCHEMATYPEVARIETY,
}
impl ::core::marker::Copy for __msxml6_ReferenceRemainingTypes__ {}
impl ::core::clone::Clone for __msxml6_ReferenceRemainingTypes__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for __msxml6_ReferenceRemainingTypes__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("__msxml6_ReferenceRemainingTypes__")
            .field("__tagDomNodeType__", &self.__tagDomNodeType__)
            .field("__domNodeType__", &self.__domNodeType__)
            .field("__serverXmlHttpOptionEnum__", &self.__serverXmlHttpOptionEnum__)
            .field("__serverXmlHttpOption__", &self.__serverXmlHttpOption__)
            .field("__serverCertOptionEnum__", &self.__serverCertOptionEnum__)
            .field("__serverCertOption__", &self.__serverCertOption__)
            .field("__proxySettingEnum__", &self.__proxySettingEnum__)
            .field("__proxySetting__", &self.__proxySetting__)
            .field("__somItemTypeEnum__", &self.__somItemTypeEnum__)
            .field("__somItemType__", &self.__somItemType__)
            .field("__schemaUseEnum__", &self.__schemaUseEnum__)
            .field("__schemaUse__", &self.__schemaUse__)
            .field("__schemaDerivationMethodEnum__", &self.__schemaDerivationMethodEnum__)
            .field("__schemaDerivationMethod__", &self.__schemaDerivationMethod__)
            .field("__schemaContentTypeEnum__", &self.__schemaContentTypeEnum__)
            .field("__schemaContentType__", &self.__schemaContentType__)
            .field("__schemaProcessContentsEnum__", &self.__schemaProcessContentsEnum__)
            .field("__schemaProcessContents__", &self.__schemaProcessContents__)
            .field("__schemaWhitespaceEnum__", &self.__schemaWhitespaceEnum__)
            .field("__schemaWhitespace__", &self.__schemaWhitespace__)
            .field("__schemaTypeVarietyEnum__", &self.__schemaTypeVarietyEnum__)
            .field("__schemaTypeVariety__", &self.__schemaTypeVariety__)
            .finish()
    }
}
impl ::windows::core::TypeKind for __msxml6_ReferenceRemainingTypes__ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for __msxml6_ReferenceRemainingTypes__ {
    fn eq(&self, other: &Self) -> bool {
        self.__tagDomNodeType__ == other.__tagDomNodeType__
            && self.__domNodeType__ == other.__domNodeType__
            && self.__serverXmlHttpOptionEnum__ == other.__serverXmlHttpOptionEnum__
            && self.__serverXmlHttpOption__ == other.__serverXmlHttpOption__
            && self.__serverCertOptionEnum__ == other.__serverCertOptionEnum__
            && self.__serverCertOption__ == other.__serverCertOption__
            && self.__proxySettingEnum__ == other.__proxySettingEnum__
            && self.__proxySetting__ == other.__proxySetting__
            && self.__somItemTypeEnum__ == other.__somItemTypeEnum__
            && self.__somItemType__ == other.__somItemType__
            && self.__schemaUseEnum__ == other.__schemaUseEnum__
            && self.__schemaUse__ == other.__schemaUse__
            && self.__schemaDerivationMethodEnum__ == other.__schemaDerivationMethodEnum__
            && self.__schemaDerivationMethod__ == other.__schemaDerivationMethod__
            && self.__schemaContentTypeEnum__ == other.__schemaContentTypeEnum__
            && self.__schemaContentType__ == other.__schemaContentType__
            && self.__schemaProcessContentsEnum__ == other.__schemaProcessContentsEnum__
            && self.__schemaProcessContents__ == other.__schemaProcessContents__
            && self.__schemaWhitespaceEnum__ == other.__schemaWhitespaceEnum__
            && self.__schemaWhitespace__ == other.__schemaWhitespace__
            && self.__schemaTypeVarietyEnum__ == other.__schemaTypeVarietyEnum__
            && self.__schemaTypeVariety__ == other.__schemaTypeVariety__
    }
}
impl ::core::cmp::Eq for __msxml6_ReferenceRemainingTypes__ {}
impl ::core::default::Default for __msxml6_ReferenceRemainingTypes__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
