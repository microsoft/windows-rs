#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_System_Com'*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlReader<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReader(riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CreateXmlReader(::core::mem::transmute(riid), ::core::mem::transmute(ppvobject), pmalloc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation', 'Win32_System_Com'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingCodePage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pinputstream: Param0, pmalloc: Param1, nencodingcodepage: u32, fencodinghint: Param3, pwszbaseuri: Param4) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReaderInputWithEncodingCodePage(pinputstream: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr, nencodingcodepage: u32, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: super::super::super::Foundation::PWSTR, ppinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CreateXmlReaderInputWithEncodingCodePage(pinputstream.into_param().abi(), pmalloc.into_param().abi(), ::core::mem::transmute(nencodingcodepage), fencodinghint.into_param().abi(), pwszbaseuri.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation', 'Win32_System_Com'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pinputstream: Param0, pmalloc: Param1, pwszencodingname: Param2, fencodinghint: Param3, pwszbaseuri: Param4) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReaderInputWithEncodingName(pinputstream: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr, pwszencodingname: super::super::super::Foundation::PWSTR, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: super::super::super::Foundation::PWSTR, ppinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CreateXmlReaderInputWithEncodingName(pinputstream.into_param().abi(), pmalloc.into_param().abi(), pwszencodingname.into_param().abi(), fencodinghint.into_param().abi(), pwszbaseuri.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_System_Com'*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriter<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriter(riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CreateXmlWriter(::core::mem::transmute(riid), ::core::mem::transmute(ppvobject), pmalloc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_System_Com'*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingCodePage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(poutputstream: Param0, pmalloc: Param1, nencodingcodepage: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriterOutputWithEncodingCodePage(poutputstream: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr, nencodingcodepage: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CreateXmlWriterOutputWithEncodingCodePage(poutputstream.into_param().abi(), pmalloc.into_param().abi(), ::core::mem::transmute(nencodingcodepage), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation', 'Win32_System_Com'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(poutputstream: Param0, pmalloc: Param1, pwszencodingname: Param2) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriterOutputWithEncodingName(poutputstream: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr, pwszencodingname: super::super::super::Foundation::PWSTR, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CreateXmlWriterOutputWithEncodingName(poutputstream.into_param().abi(), pmalloc.into_param().abi(), pwszencodingname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub type DtdProcessing = i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const DtdProcessing_Prohibit: DtdProcessing = 0i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const DtdProcessing_Parse: DtdProcessing = 1i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const _DtdProcessing_Last: DtdProcessing = 1i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
#[repr(transparent)]
pub struct IXmlReader(::windows::core::IUnknown);
impl IXmlReader {
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn SetInput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pinput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pinput.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__: isize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn Read(&self) -> ::windows::core::Result<XmlNodeType> {
        let mut result__: XmlNodeType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XmlNodeType>(result__)
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn GetNodeType(&self) -> ::windows::core::Result<XmlNodeType> {
        let mut result__: XmlNodeType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XmlNodeType>(result__)
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn MoveToFirstAttribute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn MoveToNextAttribute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveToAttributeByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszlocalname: Param0, pwsznamespaceuri: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn MoveToElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQualifiedName(&self, ppwszqualifiedname: *mut super::super::super::Foundation::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszqualifiedname), ::core::mem::transmute(pcwchqualifiedname)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamespaceUri(&self, ppwsznamespaceuri: *mut super::super::super::Foundation::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwsznamespaceuri), ::core::mem::transmute(pcwchnamespaceuri)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalName(&self, ppwszlocalname: *mut super::super::super::Foundation::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszlocalname), ::core::mem::transmute(pcwchlocalname)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrefix(&self, ppwszprefix: *mut super::super::super::Foundation::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszprefix), ::core::mem::transmute(pcwchprefix)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValue(&self, ppwszvalue: *mut super::super::super::Foundation::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszvalue), ::core::mem::transmute(pcwchvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadValueChunk(&self, pwchbuffer: super::super::super::Foundation::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwchbuffer), ::core::mem::transmute(cwchchunksize), ::core::mem::transmute(pcwchread)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBaseUri(&self, ppwszbaseuri: *mut super::super::super::Foundation::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszbaseuri), ::core::mem::transmute(pcwchbaseuri)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDefault(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEmptyElement(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn GetLineNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn GetLinePosition(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn GetAttributeCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn GetDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEOF(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IXmlReader> for ::windows::core::IUnknown {
    fn from(value: IXmlReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlReader> for ::windows::core::IUnknown {
    fn from(value: &IXmlReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXmlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXmlReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlReader {}
impl ::core::fmt::Debug for IXmlReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXmlReader {
    type Vtable = IXmlReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc81_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszqualifiedname: *mut super::super::super::Foundation::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwsznamespaceuri: *mut super::super::super::Foundation::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszlocalname: *mut super::super::super::Foundation::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszprefix: *mut super::super::super::Foundation::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszvalue: *mut super::super::super::Foundation::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchbuffer: super::super::super::Foundation::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszbaseuri: *mut super::super::super::Foundation::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlinenumber: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlineposition: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnattributecount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pndepth: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
#[repr(transparent)]
pub struct IXmlResolver(::windows::core::IUnknown);
impl IXmlResolver {
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResolveUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszbaseuri: Param0, pwszpublicidentifier: Param1, pwszsystemidentifier: Param2) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszbaseuri.into_param().abi(), pwszpublicidentifier.into_param().abi(), pwszsystemidentifier.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IXmlResolver> for ::windows::core::IUnknown {
    fn from(value: IXmlResolver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlResolver> for ::windows::core::IUnknown {
    fn from(value: &IXmlResolver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXmlResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXmlResolver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlResolver {}
impl ::core::fmt::Debug for IXmlResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlResolver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXmlResolver {
    type Vtable = IXmlResolverVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc82_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlResolverVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszbaseuri: super::super::super::Foundation::PWSTR, pwszpublicidentifier: super::super::super::Foundation::PWSTR, pwszsystemidentifier: super::super::super::Foundation::PWSTR, ppresolvedinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
#[repr(transparent)]
pub struct IXmlWriter(::windows::core::IUnknown);
impl IXmlWriter {
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn SetOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, poutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__: isize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributes<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributeString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2, pwszvalue: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wch)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteChars<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwch.into_param().abi(), ::core::mem::transmute(cwch)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteComment<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszcomment: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pwszcomment.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteDocType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwszpublicid: Param1, pwszsystemid: Param2, pwszsubset: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwszpublicid.into_param().abi(), pwszsystemid.into_param().abi(), pwszsubset.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteElementString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2, pwszvalue: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn WriteEndDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn WriteEndElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteEntityRef<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn WriteFullEndElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNmToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsznmtoken: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pwsznmtoken.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNode<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNodeShallow<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteProcessingInstruction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwsztext: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteQualifiedName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszlocalname: Param0, pwsznamespaceuri: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteRaw<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pwszdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteRawChars<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), pwch.into_param().abi(), ::core::mem::transmute(cwch)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(standalone)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteStartElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(wchlow), ::core::mem::transmute(wchhigh)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteWhitespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszwhitespace: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), pwszwhitespace.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IXmlWriter> for ::windows::core::IUnknown {
    fn from(value: IXmlWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlWriter> for ::windows::core::IUnknown {
    fn from(value: &IXmlWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXmlWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXmlWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlWriter {}
impl ::core::fmt::Debug for IXmlWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXmlWriter {
    type Vtable = IXmlWriterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc88_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
#[repr(transparent)]
pub struct IXmlWriterLite(::windows::core::IUnknown);
impl IXmlWriterLite {
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn SetOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, poutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__: isize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributes<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributeString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32, pwszvalue: Param2, cwszvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszqname.into_param().abi(), ::core::mem::transmute(cwszqname), pwszvalue.into_param().abi(), ::core::mem::transmute(cwszvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wch)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteChars<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwch.into_param().abi(), ::core::mem::transmute(cwch)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteComment<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszcomment: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pwszcomment.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteDocType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwszpublicid: Param1, pwszsystemid: Param2, pwszsubset: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwszpublicid.into_param().abi(), pwszsystemid.into_param().abi(), pwszsubset.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteElementString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32, pwszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwszqname.into_param().abi(), ::core::mem::transmute(cwszqname), pwszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn WriteEndDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteEndElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pwszqname.into_param().abi(), ::core::mem::transmute(cwszqname)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteEntityRef<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteFullEndElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), pwszqname.into_param().abi(), ::core::mem::transmute(cwszqname)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNmToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsznmtoken: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pwsznmtoken.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNode<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNodeShallow<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteProcessingInstruction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwsztext: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteRaw<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pwszdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteRawChars<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pwch.into_param().abi(), ::core::mem::transmute(cwch)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(standalone)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteStartElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), pwszqname.into_param().abi(), ::core::mem::transmute(cwszqname)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(wchlow), ::core::mem::transmute(wchhigh)).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteWhitespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszwhitespace: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), pwszwhitespace.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IXmlWriterLite> for ::windows::core::IUnknown {
    fn from(value: IXmlWriterLite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlWriterLite> for ::windows::core::IUnknown {
    fn from(value: &IXmlWriterLite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlWriterLite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXmlWriterLite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXmlWriterLite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlWriterLite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlWriterLite {}
impl ::core::fmt::Debug for IXmlWriterLite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlWriterLite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXmlWriterLite {
    type Vtable = IXmlWriterLiteVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x862494c6_1310_4aad_b3cd_2dbeebf670d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriterLiteVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR, cwszvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub type XmlConformanceLevel = i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlConformanceLevel_Auto: XmlConformanceLevel = 0i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlConformanceLevel_Fragment: XmlConformanceLevel = 1i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlConformanceLevel_Document: XmlConformanceLevel = 2i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const _XmlConformanceLevel_Last: XmlConformanceLevel = 2i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub type XmlError = i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const MX_E_MX: XmlError = -1072894464i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const MX_E_INPUTEND: XmlError = -1072894463i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const MX_E_ENCODING: XmlError = -1072894462i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const MX_E_ENCODINGSWITCH: XmlError = -1072894461i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const MX_E_ENCODINGSIGNATURE: XmlError = -1072894460i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_WC: XmlError = -1072894432i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_WHITESPACE: XmlError = -1072894431i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_SEMICOLON: XmlError = -1072894430i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_GREATERTHAN: XmlError = -1072894429i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_QUOTE: XmlError = -1072894428i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_EQUAL: XmlError = -1072894427i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_LESSTHAN: XmlError = -1072894426i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_HEXDIGIT: XmlError = -1072894425i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_DIGIT: XmlError = -1072894424i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_LEFTBRACKET: XmlError = -1072894423i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_LEFTPAREN: XmlError = -1072894422i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_XMLCHARACTER: XmlError = -1072894421i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_NAMECHARACTER: XmlError = -1072894420i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_SYNTAX: XmlError = -1072894419i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_CDSECT: XmlError = -1072894418i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_COMMENT: XmlError = -1072894417i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_CONDSECT: XmlError = -1072894416i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_DECLATTLIST: XmlError = -1072894415i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_DECLDOCTYPE: XmlError = -1072894414i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_DECLELEMENT: XmlError = -1072894413i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_DECLENTITY: XmlError = -1072894412i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_DECLNOTATION: XmlError = -1072894411i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_NDATA: XmlError = -1072894410i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_PUBLIC: XmlError = -1072894409i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_SYSTEM: XmlError = -1072894408i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_NAME: XmlError = -1072894407i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_ROOTELEMENT: XmlError = -1072894406i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_ELEMENTMATCH: XmlError = -1072894405i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_UNIQUEATTRIBUTE: XmlError = -1072894404i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_TEXTXMLDECL: XmlError = -1072894403i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_LEADINGXML: XmlError = -1072894402i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_TEXTDECL: XmlError = -1072894401i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_XMLDECL: XmlError = -1072894400i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_ENCNAME: XmlError = -1072894399i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_PUBLICID: XmlError = -1072894398i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_PESINTERNALSUBSET: XmlError = -1072894397i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_PESBETWEENDECLS: XmlError = -1072894396i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_NORECURSION: XmlError = -1072894395i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_ENTITYCONTENT: XmlError = -1072894394i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_UNDECLAREDENTITY: XmlError = -1072894393i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_PARSEDENTITY: XmlError = -1072894392i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_NOEXTERNALENTITYREF: XmlError = -1072894391i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_PI: XmlError = -1072894390i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_SYSTEMID: XmlError = -1072894389i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_QUESTIONMARK: XmlError = -1072894388i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_CDSECTEND: XmlError = -1072894387i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_MOREDATA: XmlError = -1072894386i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_DTDPROHIBITED: XmlError = -1072894385i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WC_E_INVALIDXMLSPACE: XmlError = -1072894384i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_NC: XmlError = -1072894368i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_QNAMECHARACTER: XmlError = -1072894367i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_QNAMECOLON: XmlError = -1072894366i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_NAMECOLON: XmlError = -1072894365i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_DECLAREDPREFIX: XmlError = -1072894364i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_UNDECLAREDPREFIX: XmlError = -1072894363i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_EMPTYURI: XmlError = -1072894362i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_XMLPREFIXRESERVED: XmlError = -1072894361i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_XMLNSPREFIXRESERVED: XmlError = -1072894360i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_XMLURIRESERVED: XmlError = -1072894359i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const NC_E_XMLNSURIRESERVED: XmlError = -1072894358i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const SC_E_SC: XmlError = -1072894336i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const SC_E_MAXELEMENTDEPTH: XmlError = -1072894335i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const SC_E_MAXENTITYEXPANSION: XmlError = -1072894334i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_WR: XmlError = -1072894208i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_NONWHITESPACE: XmlError = -1072894207i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_NSPREFIXDECLARED: XmlError = -1072894206i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_NSPREFIXWITHEMPTYNSURI: XmlError = -1072894205i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_DUPLICATEATTRIBUTE: XmlError = -1072894204i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_XMLNSPREFIXDECLARATION: XmlError = -1072894203i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_XMLPREFIXDECLARATION: XmlError = -1072894202i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_XMLURIDECLARATION: XmlError = -1072894201i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_XMLNSURIDECLARATION: XmlError = -1072894200i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_NAMESPACEUNDECLARED: XmlError = -1072894199i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_INVALIDXMLSPACE: XmlError = -1072894198i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_INVALIDACTION: XmlError = -1072894197i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const WR_E_INVALIDSURROGATEPAIR: XmlError = -1072894196i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XML_E_INVALID_DECIMAL: XmlError = -1072898019i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XML_E_INVALID_HEXIDECIMAL: XmlError = -1072898018i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XML_E_INVALID_UNICODE: XmlError = -1072898017i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XML_E_INVALIDENCODING: XmlError = -1072897938i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub type XmlNodeType = i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_None: XmlNodeType = 0i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_Element: XmlNodeType = 1i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_Attribute: XmlNodeType = 2i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_Text: XmlNodeType = 3i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_CDATA: XmlNodeType = 4i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_ProcessingInstruction: XmlNodeType = 7i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_Comment: XmlNodeType = 8i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_DocumentType: XmlNodeType = 10i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_Whitespace: XmlNodeType = 13i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_EndElement: XmlNodeType = 15i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlNodeType_XmlDeclaration: XmlNodeType = 17i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const _XmlNodeType_Last: XmlNodeType = 17i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub type XmlReadState = i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReadState_Initial: XmlReadState = 0i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReadState_Interactive: XmlReadState = 1i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReadState_Error: XmlReadState = 2i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReadState_EndOfFile: XmlReadState = 3i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReadState_Closed: XmlReadState = 4i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub type XmlReaderProperty = i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReaderProperty_MultiLanguage: XmlReaderProperty = 0i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReaderProperty_ConformanceLevel: XmlReaderProperty = 1i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReaderProperty_RandomAccess: XmlReaderProperty = 2i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReaderProperty_XmlResolver: XmlReaderProperty = 3i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReaderProperty_DtdProcessing: XmlReaderProperty = 4i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReaderProperty_ReadState: XmlReaderProperty = 5i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReaderProperty_MaxElementDepth: XmlReaderProperty = 6i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlReaderProperty_MaxEntityExpansion: XmlReaderProperty = 7i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const _XmlReaderProperty_Last: XmlReaderProperty = 7i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub type XmlStandalone = i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlStandalone_Omit: XmlStandalone = 0i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlStandalone_Yes: XmlStandalone = 1i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlStandalone_No: XmlStandalone = 2i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const _XmlStandalone_Last: XmlStandalone = 2i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub type XmlWriterProperty = i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlWriterProperty_MultiLanguage: XmlWriterProperty = 0i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlWriterProperty_Indent: XmlWriterProperty = 1i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlWriterProperty_ByteOrderMark: XmlWriterProperty = 2i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlWriterProperty_OmitXmlDeclaration: XmlWriterProperty = 3i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlWriterProperty_ConformanceLevel: XmlWriterProperty = 4i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const XmlWriterProperty_CompactEmptyElement: XmlWriterProperty = 5i32;
#[doc = "*Required features: 'Win32_Data_Xml_XmlLite'*"]
pub const _XmlWriterProperty_Last: XmlWriterProperty = 5i32;
pub const _IID_IXmlReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc81_709d_4095_b63d_69fe4b0d9030);
pub const _IID_IXmlResolver: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc82_709d_4095_b63d_69fe4b0d9030);
pub const _IID_IXmlWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc88_709d_4095_b63d_69fe4b0d9030);
