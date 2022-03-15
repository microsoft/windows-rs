#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingCodePage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pinputstream: Param0, pmalloc: Param1, nencodingcodepage: u32, fencodinghint: Param3, pwszbaseuri: Param4) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReaderInputWithEncodingCodePage(pinputstream: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr, nencodingcodepage: u32, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: ::windows::core::PCWSTR, ppinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CreateXmlReaderInputWithEncodingCodePage(pinputstream.into_param().abi(), pmalloc.into_param().abi(), ::core::mem::transmute(nencodingcodepage), fencodinghint.into_param().abi(), pwszbaseuri.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pinputstream: Param0, pmalloc: Param1, pwszencodingname: Param2, fencodinghint: Param3, pwszbaseuri: Param4) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReaderInputWithEncodingName(pinputstream: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr, pwszencodingname: ::windows::core::PCWSTR, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: ::windows::core::PCWSTR, ppinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CreateXmlReaderInputWithEncodingName(pinputstream.into_param().abi(), pmalloc.into_param().abi(), pwszencodingname.into_param().abi(), fencodinghint.into_param().abi(), pwszbaseuri.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(poutputstream: Param0, pmalloc: Param1, pwszencodingname: Param2) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriterOutputWithEncodingName(poutputstream: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr, pwszencodingname: ::windows::core::PCWSTR, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CreateXmlWriterOutputWithEncodingName(poutputstream.into_param().abi(), pmalloc.into_param().abi(), pwszencodingname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DtdProcessing(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const DtdProcessing_Prohibit: DtdProcessing = DtdProcessing(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const DtdProcessing_Parse: DtdProcessing = DtdProcessing(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _DtdProcessing_Last: DtdProcessing = DtdProcessing(1i32);
impl ::core::marker::Copy for DtdProcessing {}
impl ::core::clone::Clone for DtdProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DtdProcessing {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DtdProcessing {
    type Abi = Self;
}
impl ::core::fmt::Debug for DtdProcessing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtdProcessing").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
pub struct IXmlReader(::windows::core::IUnknown);
impl IXmlReader {
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn SetInput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pinput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInput)(::core::mem::transmute_copy(self), pinput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__: isize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn Read(&self) -> ::windows::core::Result<XmlNodeType> {
        let mut result__: XmlNodeType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XmlNodeType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetNodeType(&self) -> ::windows::core::Result<XmlNodeType> {
        let mut result__: XmlNodeType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNodeType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XmlNodeType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn MoveToFirstAttribute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MoveToFirstAttribute)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn MoveToNextAttribute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MoveToNextAttribute)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn MoveToAttributeByName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszlocalname: Param0, pwsznamespaceuri: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MoveToAttributeByName)(::core::mem::transmute_copy(self), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn MoveToElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MoveToElement)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetQualifiedName(&self, ppwszqualifiedname: *mut ::windows::core::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQualifiedName)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszqualifiedname), ::core::mem::transmute(pcwchqualifiedname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetNamespaceUri(&self, ppwsznamespaceuri: *mut ::windows::core::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNamespaceUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwsznamespaceuri), ::core::mem::transmute(pcwchnamespaceuri)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetLocalName(&self, ppwszlocalname: *mut ::windows::core::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLocalName)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszlocalname), ::core::mem::transmute(pcwchlocalname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetPrefix(&self, ppwszprefix: *mut ::windows::core::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrefix)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszprefix), ::core::mem::transmute(pcwchprefix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetValue(&self, ppwszvalue: *mut ::windows::core::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszvalue), ::core::mem::transmute(pcwchvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn ReadValueChunk(&self, pwchbuffer: &mut [u16], pcwchread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadValueChunk)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwchbuffer)), pwchbuffer.len() as _, ::core::mem::transmute(pcwchread)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetBaseUri(&self, ppwszbaseuri: *mut ::windows::core::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBaseUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszbaseuri), ::core::mem::transmute(pcwchbaseuri)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDefault(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsDefault)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEmptyElement(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsEmptyElement)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetLineNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLineNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetLinePosition(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLinePosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetAttributeCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttributeCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDepth)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEOF(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsEOF)(::core::mem::transmute_copy(self)))
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlReader {
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
    type Vtable = IXmlReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc81_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlReader_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT,
    pub GetNodeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT,
    pub MoveToFirstAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveToNextAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveToAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub MoveToElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetQualifiedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszqualifiedname: *mut ::windows::core::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::HRESULT,
    pub GetNamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwsznamespaceuri: *mut ::windows::core::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::HRESULT,
    pub GetLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszlocalname: *mut ::windows::core::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::HRESULT,
    pub GetPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszprefix: *mut ::windows::core::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszvalue: *mut ::windows::core::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::HRESULT,
    pub ReadValueChunk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchbuffer: ::windows::core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::HRESULT,
    pub GetBaseUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszbaseuri: *mut ::windows::core::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDefault: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEmptyElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEmptyElement: usize,
    pub GetLineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlinenumber: *mut u32) -> ::windows::core::HRESULT,
    pub GetLinePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlineposition: *mut u32) -> ::windows::core::HRESULT,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnattributecount: *mut u32) -> ::windows::core::HRESULT,
    pub GetDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pndepth: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEOF: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
pub struct IXmlResolver(::windows::core::IUnknown);
impl IXmlResolver {
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn ResolveUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszbaseuri: Param0, pwszpublicidentifier: Param1, pwszsystemidentifier: Param2) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ResolveUri)(::core::mem::transmute_copy(self), pwszbaseuri.into_param().abi(), pwszpublicidentifier.into_param().abi(), pwszsystemidentifier.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlResolver {
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
    type Vtable = IXmlResolver_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc82_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlResolver_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ResolveUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszbaseuri: ::windows::core::PCWSTR, pwszpublicidentifier: ::windows::core::PCWSTR, pwszsystemidentifier: ::windows::core::PCWSTR, ppresolvedinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
pub struct IXmlWriter(::windows::core::IUnknown);
impl IXmlWriter {
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn SetOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, poutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutput)(::core::mem::transmute_copy(self), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__: isize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributes<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteAttributes)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteAttributeString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2, pwszvalue: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteAttributeString)(::core::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteCData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteCData)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteCharEntity)(::core::mem::transmute_copy(self), ::core::mem::transmute(wch)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteChars(&self, pwch: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteChars)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwch)), pwch.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszcomment: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteComment)(::core::mem::transmute_copy(self), pwszcomment.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteDocType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0, pwszpublicid: Param1, pwszsystemid: Param2, pwszsubset: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteDocType)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwszpublicid.into_param().abi(), pwszsystemid.into_param().abi(), pwszsubset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteElementString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2, pwszvalue: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteElementString)(::core::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteEndDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteEndDocument)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteEndElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteEndElement)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteEntityRef<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteEntityRef)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteFullEndElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteFullEndElement)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteName)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteNmToken<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwsznmtoken: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNmToken)(::core::mem::transmute_copy(self), pwsznmtoken.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNode<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNode)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNodeShallow<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNodeShallow)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteProcessingInstruction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0, pwsztext: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteProcessingInstruction)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteQualifiedName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszlocalname: Param0, pwsznamespaceuri: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteQualifiedName)(::core::mem::transmute_copy(self), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteRaw<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteRaw)(::core::mem::transmute_copy(self), pwszdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteRawChars(&self, pwch: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteRawChars)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwch)), pwch.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteStartDocument)(::core::mem::transmute_copy(self), ::core::mem::transmute(standalone)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteStartElement<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteStartElement)(::core::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteString)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteSurrogateCharEntity)(::core::mem::transmute_copy(self), ::core::mem::transmute(wchlow), ::core::mem::transmute(wchhigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteWhitespace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszwhitespace: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteWhitespace)(::core::mem::transmute_copy(self), pwszwhitespace.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlWriter {
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
    type Vtable = IXmlWriter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc88_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteAttributes: usize,
    pub WriteAttributeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: ::windows::core::PCWSTR, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteCData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT,
    pub WriteChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT,
    pub WriteComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomment: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteDocType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwszpublicid: ::windows::core::PCWSTR, pwszsystemid: ::windows::core::PCWSTR, pwszsubset: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteElementString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: ::windows::core::PCWSTR, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteEndDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteEntityRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteFullEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteNmToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznmtoken: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNodeShallow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNodeShallow: usize,
    pub WriteProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteQualifiedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteRawChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT,
    pub WriteStartDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT,
    pub WriteStartElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: ::windows::core::PCWSTR, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteSurrogateCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT,
    pub WriteWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszwhitespace: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
pub struct IXmlWriterLite(::windows::core::IUnknown);
impl IXmlWriterLite {
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn SetOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, poutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutput)(::core::mem::transmute_copy(self), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__: isize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributes<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteAttributes)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteAttributeString(&self, pwszqname: &[u16], pwszvalue: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteAttributeString)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwszqname)), pwszqname.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pwszvalue)), pwszvalue.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteCData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteCData)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteCharEntity)(::core::mem::transmute_copy(self), ::core::mem::transmute(wch)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteChars(&self, pwch: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteChars)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwch)), pwch.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszcomment: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteComment)(::core::mem::transmute_copy(self), pwszcomment.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteDocType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0, pwszpublicid: Param1, pwszsystemid: Param2, pwszsubset: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteDocType)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwszpublicid.into_param().abi(), pwszsystemid.into_param().abi(), pwszsubset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteElementString<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszqname: &[u16], pwszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteElementString)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwszqname)), pwszqname.len() as _, pwszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteEndDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteEndDocument)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteEndElement(&self, pwszqname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteEndElement)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwszqname)), pwszqname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteEntityRef<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteEntityRef)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteFullEndElement(&self, pwszqname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteFullEndElement)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwszqname)), pwszqname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteName)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteNmToken<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwsznmtoken: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNmToken)(::core::mem::transmute_copy(self), pwsznmtoken.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNode<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNode)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNodeShallow<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNodeShallow)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteProcessingInstruction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0, pwsztext: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteProcessingInstruction)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteRaw<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteRaw)(::core::mem::transmute_copy(self), pwszdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteRawChars(&self, pwch: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteRawChars)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwch)), pwch.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteStartDocument)(::core::mem::transmute_copy(self), ::core::mem::transmute(standalone)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteStartElement(&self, pwszqname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteStartElement)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwszqname)), pwszqname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteString)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteSurrogateCharEntity)(::core::mem::transmute_copy(self), ::core::mem::transmute(wchlow), ::core::mem::transmute(wchhigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn WriteWhitespace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszwhitespace: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteWhitespace)(::core::mem::transmute_copy(self), pwszwhitespace.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlWriterLite {
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
    type Vtable = IXmlWriterLite_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x862494c6_1310_4aad_b3cd_2dbeebf670d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriterLite_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteAttributes: usize,
    pub WriteAttributeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32, pwszvalue: ::windows::core::PCWSTR, cwszvalue: u32) -> ::windows::core::HRESULT,
    pub WriteCData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT,
    pub WriteChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT,
    pub WriteComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomment: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteDocType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwszpublicid: ::windows::core::PCWSTR, pwszsystemid: ::windows::core::PCWSTR, pwszsubset: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteElementString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteEndDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    pub WriteEntityRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteFullEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    pub WriteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteNmToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznmtoken: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNodeShallow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNodeShallow: usize,
    pub WriteProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteRawChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT,
    pub WriteStartDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT,
    pub WriteStartElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    pub WriteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteSurrogateCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT,
    pub WriteWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszwhitespace: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlConformanceLevel(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlConformanceLevel_Auto: XmlConformanceLevel = XmlConformanceLevel(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlConformanceLevel_Fragment: XmlConformanceLevel = XmlConformanceLevel(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlConformanceLevel_Document: XmlConformanceLevel = XmlConformanceLevel(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlConformanceLevel_Last: XmlConformanceLevel = XmlConformanceLevel(2i32);
impl ::core::marker::Copy for XmlConformanceLevel {}
impl ::core::clone::Clone for XmlConformanceLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlConformanceLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlConformanceLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlConformanceLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlConformanceLevel").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlError(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_MX: XmlError = XmlError(-1072894464i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_INPUTEND: XmlError = XmlError(-1072894463i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_ENCODING: XmlError = XmlError(-1072894462i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_ENCODINGSWITCH: XmlError = XmlError(-1072894461i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_ENCODINGSIGNATURE: XmlError = XmlError(-1072894460i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_WC: XmlError = XmlError(-1072894432i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_WHITESPACE: XmlError = XmlError(-1072894431i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SEMICOLON: XmlError = XmlError(-1072894430i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_GREATERTHAN: XmlError = XmlError(-1072894429i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_QUOTE: XmlError = XmlError(-1072894428i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_EQUAL: XmlError = XmlError(-1072894427i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LESSTHAN: XmlError = XmlError(-1072894426i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_HEXDIGIT: XmlError = XmlError(-1072894425i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DIGIT: XmlError = XmlError(-1072894424i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LEFTBRACKET: XmlError = XmlError(-1072894423i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LEFTPAREN: XmlError = XmlError(-1072894422i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_XMLCHARACTER: XmlError = XmlError(-1072894421i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NAMECHARACTER: XmlError = XmlError(-1072894420i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SYNTAX: XmlError = XmlError(-1072894419i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_CDSECT: XmlError = XmlError(-1072894418i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_COMMENT: XmlError = XmlError(-1072894417i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_CONDSECT: XmlError = XmlError(-1072894416i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLATTLIST: XmlError = XmlError(-1072894415i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLDOCTYPE: XmlError = XmlError(-1072894414i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLELEMENT: XmlError = XmlError(-1072894413i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLENTITY: XmlError = XmlError(-1072894412i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLNOTATION: XmlError = XmlError(-1072894411i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NDATA: XmlError = XmlError(-1072894410i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PUBLIC: XmlError = XmlError(-1072894409i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SYSTEM: XmlError = XmlError(-1072894408i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NAME: XmlError = XmlError(-1072894407i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ROOTELEMENT: XmlError = XmlError(-1072894406i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ELEMENTMATCH: XmlError = XmlError(-1072894405i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_UNIQUEATTRIBUTE: XmlError = XmlError(-1072894404i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_TEXTXMLDECL: XmlError = XmlError(-1072894403i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LEADINGXML: XmlError = XmlError(-1072894402i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_TEXTDECL: XmlError = XmlError(-1072894401i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_XMLDECL: XmlError = XmlError(-1072894400i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ENCNAME: XmlError = XmlError(-1072894399i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PUBLICID: XmlError = XmlError(-1072894398i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PESINTERNALSUBSET: XmlError = XmlError(-1072894397i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PESBETWEENDECLS: XmlError = XmlError(-1072894396i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NORECURSION: XmlError = XmlError(-1072894395i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ENTITYCONTENT: XmlError = XmlError(-1072894394i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_UNDECLAREDENTITY: XmlError = XmlError(-1072894393i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PARSEDENTITY: XmlError = XmlError(-1072894392i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NOEXTERNALENTITYREF: XmlError = XmlError(-1072894391i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PI: XmlError = XmlError(-1072894390i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SYSTEMID: XmlError = XmlError(-1072894389i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_QUESTIONMARK: XmlError = XmlError(-1072894388i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_CDSECTEND: XmlError = XmlError(-1072894387i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_MOREDATA: XmlError = XmlError(-1072894386i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DTDPROHIBITED: XmlError = XmlError(-1072894385i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894384i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_NC: XmlError = XmlError(-1072894368i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_QNAMECHARACTER: XmlError = XmlError(-1072894367i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_QNAMECOLON: XmlError = XmlError(-1072894366i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_NAMECOLON: XmlError = XmlError(-1072894365i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_DECLAREDPREFIX: XmlError = XmlError(-1072894364i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_UNDECLAREDPREFIX: XmlError = XmlError(-1072894363i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_EMPTYURI: XmlError = XmlError(-1072894362i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLPREFIXRESERVED: XmlError = XmlError(-1072894361i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLNSPREFIXRESERVED: XmlError = XmlError(-1072894360i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLURIRESERVED: XmlError = XmlError(-1072894359i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLNSURIRESERVED: XmlError = XmlError(-1072894358i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const SC_E_SC: XmlError = XmlError(-1072894336i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const SC_E_MAXELEMENTDEPTH: XmlError = XmlError(-1072894335i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const SC_E_MAXENTITYEXPANSION: XmlError = XmlError(-1072894334i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_WR: XmlError = XmlError(-1072894208i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NONWHITESPACE: XmlError = XmlError(-1072894207i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NSPREFIXDECLARED: XmlError = XmlError(-1072894206i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NSPREFIXWITHEMPTYNSURI: XmlError = XmlError(-1072894205i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_DUPLICATEATTRIBUTE: XmlError = XmlError(-1072894204i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLNSPREFIXDECLARATION: XmlError = XmlError(-1072894203i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLPREFIXDECLARATION: XmlError = XmlError(-1072894202i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLURIDECLARATION: XmlError = XmlError(-1072894201i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLNSURIDECLARATION: XmlError = XmlError(-1072894200i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NAMESPACEUNDECLARED: XmlError = XmlError(-1072894199i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894198i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_INVALIDACTION: XmlError = XmlError(-1072894197i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_INVALIDSURROGATEPAIR: XmlError = XmlError(-1072894196i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALID_DECIMAL: XmlError = XmlError(-1072898019i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALID_HEXIDECIMAL: XmlError = XmlError(-1072898018i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALID_UNICODE: XmlError = XmlError(-1072898017i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALIDENCODING: XmlError = XmlError(-1072897938i32);
impl ::core::marker::Copy for XmlError {}
impl ::core::clone::Clone for XmlError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlError {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlError").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlNodeType(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_None: XmlNodeType = XmlNodeType(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Element: XmlNodeType = XmlNodeType(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Attribute: XmlNodeType = XmlNodeType(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Text: XmlNodeType = XmlNodeType(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_CDATA: XmlNodeType = XmlNodeType(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_ProcessingInstruction: XmlNodeType = XmlNodeType(7i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Comment: XmlNodeType = XmlNodeType(8i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_DocumentType: XmlNodeType = XmlNodeType(10i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Whitespace: XmlNodeType = XmlNodeType(13i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_EndElement: XmlNodeType = XmlNodeType(15i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_XmlDeclaration: XmlNodeType = XmlNodeType(17i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlNodeType_Last: XmlNodeType = XmlNodeType(17i32);
impl ::core::marker::Copy for XmlNodeType {}
impl ::core::clone::Clone for XmlNodeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlNodeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlNodeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlNodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlNodeType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlReadState(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Initial: XmlReadState = XmlReadState(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Interactive: XmlReadState = XmlReadState(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Error: XmlReadState = XmlReadState(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_EndOfFile: XmlReadState = XmlReadState(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Closed: XmlReadState = XmlReadState(4i32);
impl ::core::marker::Copy for XmlReadState {}
impl ::core::clone::Clone for XmlReadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlReadState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlReadState {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlReadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlReadState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlReaderProperty(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_MultiLanguage: XmlReaderProperty = XmlReaderProperty(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_ConformanceLevel: XmlReaderProperty = XmlReaderProperty(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_RandomAccess: XmlReaderProperty = XmlReaderProperty(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_XmlResolver: XmlReaderProperty = XmlReaderProperty(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_DtdProcessing: XmlReaderProperty = XmlReaderProperty(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_ReadState: XmlReaderProperty = XmlReaderProperty(5i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_MaxElementDepth: XmlReaderProperty = XmlReaderProperty(6i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_MaxEntityExpansion: XmlReaderProperty = XmlReaderProperty(7i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlReaderProperty_Last: XmlReaderProperty = XmlReaderProperty(7i32);
impl ::core::marker::Copy for XmlReaderProperty {}
impl ::core::clone::Clone for XmlReaderProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlReaderProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlReaderProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlReaderProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlReaderProperty").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlStandalone(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlStandalone_Omit: XmlStandalone = XmlStandalone(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlStandalone_Yes: XmlStandalone = XmlStandalone(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlStandalone_No: XmlStandalone = XmlStandalone(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlStandalone_Last: XmlStandalone = XmlStandalone(2i32);
impl ::core::marker::Copy for XmlStandalone {}
impl ::core::clone::Clone for XmlStandalone {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlStandalone {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlStandalone {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlStandalone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlStandalone").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlWriterProperty(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_MultiLanguage: XmlWriterProperty = XmlWriterProperty(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_Indent: XmlWriterProperty = XmlWriterProperty(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_ByteOrderMark: XmlWriterProperty = XmlWriterProperty(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_OmitXmlDeclaration: XmlWriterProperty = XmlWriterProperty(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_ConformanceLevel: XmlWriterProperty = XmlWriterProperty(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_CompactEmptyElement: XmlWriterProperty = XmlWriterProperty(5i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlWriterProperty_Last: XmlWriterProperty = XmlWriterProperty(5i32);
impl ::core::marker::Copy for XmlWriterProperty {}
impl ::core::clone::Clone for XmlWriterProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlWriterProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlWriterProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlWriterProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlWriterProperty").field(&self.0).finish()
    }
}
pub const _IID_IXmlReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc81_709d_4095_b63d_69fe4b0d9030);
pub const _IID_IXmlResolver: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc82_709d_4095_b63d_69fe4b0d9030);
pub const _IID_IXmlWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc88_709d_4095_b63d_69fe4b0d9030);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
