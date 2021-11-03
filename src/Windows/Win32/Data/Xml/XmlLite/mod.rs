#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateXmlReader<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IMalloc>>(riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void, pmalloc: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReader(riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void, pmalloc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        CreateXmlReader(::std::mem::transmute(riid), ::std::mem::transmute(ppvobject), pmalloc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingCodePage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
    pinputstream: Param0,
    pmalloc: Param1,
    nencodingcodepage: u32,
    fencodinghint: Param3,
    pwszbaseuri: Param4,
) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReaderInputWithEncodingCodePage(pinputstream: ::windows::runtime::RawPtr, pmalloc: ::windows::runtime::RawPtr, nencodingcodepage: u32, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: super::super::super::Foundation::PWSTR, ppinput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateXmlReaderInputWithEncodingCodePage(pinputstream.into_param().abi(), pmalloc.into_param().abi(), ::std::mem::transmute(nencodingcodepage), fencodinghint.into_param().abi(), pwszbaseuri.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
    pinputstream: Param0,
    pmalloc: Param1,
    pwszencodingname: Param2,
    fencodinghint: Param3,
    pwszbaseuri: Param4,
) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReaderInputWithEncodingName(pinputstream: ::windows::runtime::RawPtr, pmalloc: ::windows::runtime::RawPtr, pwszencodingname: super::super::super::Foundation::PWSTR, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: super::super::super::Foundation::PWSTR, ppinput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateXmlReaderInputWithEncodingName(pinputstream.into_param().abi(), pmalloc.into_param().abi(), pwszencodingname.into_param().abi(), fencodinghint.into_param().abi(), pwszbaseuri.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateXmlWriter<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IMalloc>>(riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void, pmalloc: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriter(riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void, pmalloc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        CreateXmlWriter(::std::mem::transmute(riid), ::std::mem::transmute(ppvobject), pmalloc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingCodePage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IMalloc>>(poutputstream: Param0, pmalloc: Param1, nencodingcodepage: u32) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriterOutputWithEncodingCodePage(poutputstream: ::windows::runtime::RawPtr, pmalloc: ::windows::runtime::RawPtr, nencodingcodepage: u32, ppoutput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateXmlWriterOutputWithEncodingCodePage(poutputstream.into_param().abi(), pmalloc.into_param().abi(), ::std::mem::transmute(nencodingcodepage), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(poutputstream: Param0, pmalloc: Param1, pwszencodingname: Param2) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriterOutputWithEncodingName(poutputstream: ::windows::runtime::RawPtr, pmalloc: ::windows::runtime::RawPtr, pwszencodingname: super::super::super::Foundation::PWSTR, ppoutput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateXmlWriterOutputWithEncodingName(poutputstream.into_param().abi(), pmalloc.into_param().abi(), pwszencodingname.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DtdProcessing(pub i32);
pub const DtdProcessing_Prohibit: DtdProcessing = DtdProcessing(0i32);
pub const DtdProcessing_Parse: DtdProcessing = DtdProcessing(1i32);
pub const _DtdProcessing_Last: DtdProcessing = DtdProcessing(1i32);
impl ::std::convert::From<i32> for DtdProcessing {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DtdProcessing {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXmlReader(::windows::runtime::IUnknown);
impl IXmlReader {
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn SetInput<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pinput: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pinput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::runtime::Result<isize> {
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(nproperty), &mut result__).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(nproperty), ::std::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn Read(&self) -> ::windows::runtime::Result<XmlNodeType> {
        let mut result__: <XmlNodeType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<XmlNodeType>(result__)
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn GetNodeType(&self) -> ::windows::runtime::Result<XmlNodeType> {
        let mut result__: <XmlNodeType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<XmlNodeType>(result__)
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn MoveToFirstAttribute(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn MoveToNextAttribute(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn MoveToAttributeByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszlocalname: Param0, pwsznamespaceuri: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn MoveToElement(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn GetQualifiedName(&self, ppwszqualifiedname: *mut super::super::super::Foundation::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppwszqualifiedname), ::std::mem::transmute(pcwchqualifiedname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn GetNamespaceUri(&self, ppwsznamespaceuri: *mut super::super::super::Foundation::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppwsznamespaceuri), ::std::mem::transmute(pcwchnamespaceuri)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn GetLocalName(&self, ppwszlocalname: *mut super::super::super::Foundation::PWSTR, pcwchlocalname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppwszlocalname), ::std::mem::transmute(pcwchlocalname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn GetPrefix(&self, ppwszprefix: *mut super::super::super::Foundation::PWSTR, pcwchprefix: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppwszprefix), ::std::mem::transmute(pcwchprefix)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn GetValue(&self, ppwszvalue: *mut super::super::super::Foundation::PWSTR, pcwchvalue: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppwszvalue), ::std::mem::transmute(pcwchvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn ReadValueChunk(&self, pwchbuffer: super::super::super::Foundation::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwchbuffer), ::std::mem::transmute(cwchchunksize), ::std::mem::transmute(pcwchread)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn GetBaseUri(&self, ppwszbaseuri: *mut super::super::super::Foundation::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppwszbaseuri), ::std::mem::transmute(pcwchbaseuri)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn IsDefault(&self) -> super::super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn IsEmptyElement(&self) -> super::super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn GetLineNumber(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn GetLinePosition(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn GetAttributeCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn GetDepth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn IsEOF(&self) -> super::super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IXmlReader {
    type Vtable = IXmlReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1920597121, 28829, 16533, [182, 61, 105, 254, 75, 13, 144, 48]);
}
impl ::std::convert::From<IXmlReader> for ::windows::runtime::IUnknown {
    fn from(value: IXmlReader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXmlReader> for ::windows::runtime::IUnknown {
    fn from(value: &IXmlReader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXmlReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXmlReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinput: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nproperty: u32, ppvalue: *mut isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nproperty: u32, pvalue: isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnodetype: *mut XmlNodeType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnodetype: *mut XmlNodeType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszqualifiedname: *mut super::super::super::Foundation::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwsznamespaceuri: *mut super::super::super::Foundation::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszlocalname: *mut super::super::super::Foundation::PWSTR, pcwchlocalname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszprefix: *mut super::super::super::Foundation::PWSTR, pcwchprefix: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszvalue: *mut super::super::super::Foundation::PWSTR, pcwchvalue: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwchbuffer: super::super::super::Foundation::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszbaseuri: *mut super::super::super::Foundation::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnlinenumber: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnlineposition: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnattributecount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pndepth: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXmlResolver(::windows::runtime::IUnknown);
impl IXmlResolver {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn ResolveUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszbaseuri: Param0, pwszpublicidentifier: Param1, pwszsystemidentifier: Param2) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszbaseuri.into_param().abi(), pwszpublicidentifier.into_param().abi(), pwszsystemidentifier.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXmlResolver {
    type Vtable = IXmlResolver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1920597122, 28829, 16533, [182, 61, 105, 254, 75, 13, 144, 48]);
}
impl ::std::convert::From<IXmlResolver> for ::windows::runtime::IUnknown {
    fn from(value: IXmlResolver) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXmlResolver> for ::windows::runtime::IUnknown {
    fn from(value: &IXmlResolver) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXmlResolver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXmlResolver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlResolver_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszbaseuri: super::super::super::Foundation::PWSTR, pwszpublicidentifier: super::super::super::Foundation::PWSTR, pwszsystemidentifier: super::super::super::Foundation::PWSTR, ppresolvedinput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXmlWriter(::windows::runtime::IUnknown);
impl IXmlWriter {
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn SetOutput<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, poutput: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::runtime::Result<isize> {
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(nproperty), &mut result__).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(nproperty), ::std::mem::transmute(pvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlReader>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteAttributeString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
        &self,
        pwszprefix: Param0,
        pwszlocalname: Param1,
        pwsznamespaceuri: Param2,
        pwszvalue: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteCData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteChars<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pwch.into_param().abi(), ::std::mem::transmute(cwch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteComment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszcomment: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pwszcomment.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteDocType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwszpublicid: Param1, pwszsystemid: Param2, pwszsubset: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pwszname.into_param().abi(), pwszpublicid.into_param().abi(), pwszsystemid.into_param().abi(), pwszsubset.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteElementString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
        &self,
        pwszprefix: Param0,
        pwszlocalname: Param1,
        pwsznamespaceuri: Param2,
        pwszvalue: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn WriteEndDocument(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn WriteEndElement(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteEntityRef<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn WriteFullEndElement(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteNmToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsznmtoken: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pwsznmtoken.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteNode<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlReader>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteNodeShallow<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlReader>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteProcessingInstruction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwsztext: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), pwszname.into_param().abi(), pwsztext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteQualifiedName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszlocalname: Param0, pwsznamespaceuri: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteRaw<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszdata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), pwszdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteRawChars<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), pwch.into_param().abi(), ::std::mem::transmute(cwch)).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(standalone)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteStartElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(wchlow), ::std::mem::transmute(wchhigh)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteWhitespace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszwhitespace: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), pwszwhitespace.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXmlWriter {
    type Vtable = IXmlWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1920597128, 28829, 16533, [182, 61, 105, 254, 75, 13, 144, 48]);
}
impl ::std::convert::From<IXmlWriter> for ::windows::runtime::IUnknown {
    fn from(value: IXmlWriter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXmlWriter> for ::windows::runtime::IUnknown {
    fn from(value: &IXmlWriter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXmlWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXmlWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poutput: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nproperty: u32, ppvalue: *mut isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nproperty: u32, pvalue: isize) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preader: ::windows::runtime::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wch: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preader: ::windows::runtime::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preader: ::windows::runtime::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, standalone: XmlStandalone) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wchlow: u16, wchhigh: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXmlWriterLite(::windows::runtime::IUnknown);
impl IXmlWriterLite {
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn SetOutput<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, poutput: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::runtime::Result<isize> {
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(nproperty), &mut result__).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(nproperty), ::std::mem::transmute(pvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlReader>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteAttributeString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32, pwszvalue: Param2, cwszvalue: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszqname.into_param().abi(), ::std::mem::transmute(cwszqname), pwszvalue.into_param().abi(), ::std::mem::transmute(cwszvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteCData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteChars<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pwch.into_param().abi(), ::std::mem::transmute(cwch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteComment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszcomment: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pwszcomment.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteDocType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwszpublicid: Param1, pwszsystemid: Param2, pwszsubset: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pwszname.into_param().abi(), pwszpublicid.into_param().abi(), pwszsystemid.into_param().abi(), pwszsubset.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteElementString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32, pwszvalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pwszqname.into_param().abi(), ::std::mem::transmute(cwszqname), pwszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn WriteEndDocument(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteEndElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pwszqname.into_param().abi(), ::std::mem::transmute(cwszqname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteEntityRef<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteFullEndElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pwszqname.into_param().abi(), ::std::mem::transmute(cwszqname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteNmToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsznmtoken: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pwsznmtoken.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteNode<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlReader>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteNodeShallow<'a, Param0: ::windows::runtime::IntoParam<'a, IXmlReader>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteProcessingInstruction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwsztext: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), pwszname.into_param().abi(), pwsztext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteRaw<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszdata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pwszdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteRawChars<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), pwch.into_param().abi(), ::std::mem::transmute(cwch)).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(standalone)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteStartElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), pwszqname.into_param().abi(), ::std::mem::transmute(cwszqname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(wchlow), ::std::mem::transmute(wchhigh)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`*"]
    pub unsafe fn WriteWhitespace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszwhitespace: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), pwszwhitespace.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXmlWriterLite {
    type Vtable = IXmlWriterLite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2250544326, 4880, 19117, [179, 205, 45, 190, 235, 246, 112, 211]);
}
impl ::std::convert::From<IXmlWriterLite> for ::windows::runtime::IUnknown {
    fn from(value: IXmlWriterLite) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXmlWriterLite> for ::windows::runtime::IUnknown {
    fn from(value: &IXmlWriterLite) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXmlWriterLite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXmlWriterLite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriterLite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poutput: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nproperty: u32, ppvalue: *mut isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nproperty: u32, pvalue: isize) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preader: ::windows::runtime::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR, cwszvalue: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wch: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preader: ::windows::runtime::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preader: ::windows::runtime::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, standalone: XmlStandalone) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wchlow: u16, wchhigh: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlConformanceLevel(pub i32);
pub const XmlConformanceLevel_Auto: XmlConformanceLevel = XmlConformanceLevel(0i32);
pub const XmlConformanceLevel_Fragment: XmlConformanceLevel = XmlConformanceLevel(1i32);
pub const XmlConformanceLevel_Document: XmlConformanceLevel = XmlConformanceLevel(2i32);
pub const _XmlConformanceLevel_Last: XmlConformanceLevel = XmlConformanceLevel(2i32);
impl ::std::convert::From<i32> for XmlConformanceLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XmlConformanceLevel {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlError(pub i32);
pub const MX_E_MX: XmlError = XmlError(-1072894464i32);
pub const MX_E_INPUTEND: XmlError = XmlError(-1072894463i32);
pub const MX_E_ENCODING: XmlError = XmlError(-1072894462i32);
pub const MX_E_ENCODINGSWITCH: XmlError = XmlError(-1072894461i32);
pub const MX_E_ENCODINGSIGNATURE: XmlError = XmlError(-1072894460i32);
pub const WC_E_WC: XmlError = XmlError(-1072894432i32);
pub const WC_E_WHITESPACE: XmlError = XmlError(-1072894431i32);
pub const WC_E_SEMICOLON: XmlError = XmlError(-1072894430i32);
pub const WC_E_GREATERTHAN: XmlError = XmlError(-1072894429i32);
pub const WC_E_QUOTE: XmlError = XmlError(-1072894428i32);
pub const WC_E_EQUAL: XmlError = XmlError(-1072894427i32);
pub const WC_E_LESSTHAN: XmlError = XmlError(-1072894426i32);
pub const WC_E_HEXDIGIT: XmlError = XmlError(-1072894425i32);
pub const WC_E_DIGIT: XmlError = XmlError(-1072894424i32);
pub const WC_E_LEFTBRACKET: XmlError = XmlError(-1072894423i32);
pub const WC_E_LEFTPAREN: XmlError = XmlError(-1072894422i32);
pub const WC_E_XMLCHARACTER: XmlError = XmlError(-1072894421i32);
pub const WC_E_NAMECHARACTER: XmlError = XmlError(-1072894420i32);
pub const WC_E_SYNTAX: XmlError = XmlError(-1072894419i32);
pub const WC_E_CDSECT: XmlError = XmlError(-1072894418i32);
pub const WC_E_COMMENT: XmlError = XmlError(-1072894417i32);
pub const WC_E_CONDSECT: XmlError = XmlError(-1072894416i32);
pub const WC_E_DECLATTLIST: XmlError = XmlError(-1072894415i32);
pub const WC_E_DECLDOCTYPE: XmlError = XmlError(-1072894414i32);
pub const WC_E_DECLELEMENT: XmlError = XmlError(-1072894413i32);
pub const WC_E_DECLENTITY: XmlError = XmlError(-1072894412i32);
pub const WC_E_DECLNOTATION: XmlError = XmlError(-1072894411i32);
pub const WC_E_NDATA: XmlError = XmlError(-1072894410i32);
pub const WC_E_PUBLIC: XmlError = XmlError(-1072894409i32);
pub const WC_E_SYSTEM: XmlError = XmlError(-1072894408i32);
pub const WC_E_NAME: XmlError = XmlError(-1072894407i32);
pub const WC_E_ROOTELEMENT: XmlError = XmlError(-1072894406i32);
pub const WC_E_ELEMENTMATCH: XmlError = XmlError(-1072894405i32);
pub const WC_E_UNIQUEATTRIBUTE: XmlError = XmlError(-1072894404i32);
pub const WC_E_TEXTXMLDECL: XmlError = XmlError(-1072894403i32);
pub const WC_E_LEADINGXML: XmlError = XmlError(-1072894402i32);
pub const WC_E_TEXTDECL: XmlError = XmlError(-1072894401i32);
pub const WC_E_XMLDECL: XmlError = XmlError(-1072894400i32);
pub const WC_E_ENCNAME: XmlError = XmlError(-1072894399i32);
pub const WC_E_PUBLICID: XmlError = XmlError(-1072894398i32);
pub const WC_E_PESINTERNALSUBSET: XmlError = XmlError(-1072894397i32);
pub const WC_E_PESBETWEENDECLS: XmlError = XmlError(-1072894396i32);
pub const WC_E_NORECURSION: XmlError = XmlError(-1072894395i32);
pub const WC_E_ENTITYCONTENT: XmlError = XmlError(-1072894394i32);
pub const WC_E_UNDECLAREDENTITY: XmlError = XmlError(-1072894393i32);
pub const WC_E_PARSEDENTITY: XmlError = XmlError(-1072894392i32);
pub const WC_E_NOEXTERNALENTITYREF: XmlError = XmlError(-1072894391i32);
pub const WC_E_PI: XmlError = XmlError(-1072894390i32);
pub const WC_E_SYSTEMID: XmlError = XmlError(-1072894389i32);
pub const WC_E_QUESTIONMARK: XmlError = XmlError(-1072894388i32);
pub const WC_E_CDSECTEND: XmlError = XmlError(-1072894387i32);
pub const WC_E_MOREDATA: XmlError = XmlError(-1072894386i32);
pub const WC_E_DTDPROHIBITED: XmlError = XmlError(-1072894385i32);
pub const WC_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894384i32);
pub const NC_E_NC: XmlError = XmlError(-1072894368i32);
pub const NC_E_QNAMECHARACTER: XmlError = XmlError(-1072894367i32);
pub const NC_E_QNAMECOLON: XmlError = XmlError(-1072894366i32);
pub const NC_E_NAMECOLON: XmlError = XmlError(-1072894365i32);
pub const NC_E_DECLAREDPREFIX: XmlError = XmlError(-1072894364i32);
pub const NC_E_UNDECLAREDPREFIX: XmlError = XmlError(-1072894363i32);
pub const NC_E_EMPTYURI: XmlError = XmlError(-1072894362i32);
pub const NC_E_XMLPREFIXRESERVED: XmlError = XmlError(-1072894361i32);
pub const NC_E_XMLNSPREFIXRESERVED: XmlError = XmlError(-1072894360i32);
pub const NC_E_XMLURIRESERVED: XmlError = XmlError(-1072894359i32);
pub const NC_E_XMLNSURIRESERVED: XmlError = XmlError(-1072894358i32);
pub const SC_E_SC: XmlError = XmlError(-1072894336i32);
pub const SC_E_MAXELEMENTDEPTH: XmlError = XmlError(-1072894335i32);
pub const SC_E_MAXENTITYEXPANSION: XmlError = XmlError(-1072894334i32);
pub const WR_E_WR: XmlError = XmlError(-1072894208i32);
pub const WR_E_NONWHITESPACE: XmlError = XmlError(-1072894207i32);
pub const WR_E_NSPREFIXDECLARED: XmlError = XmlError(-1072894206i32);
pub const WR_E_NSPREFIXWITHEMPTYNSURI: XmlError = XmlError(-1072894205i32);
pub const WR_E_DUPLICATEATTRIBUTE: XmlError = XmlError(-1072894204i32);
pub const WR_E_XMLNSPREFIXDECLARATION: XmlError = XmlError(-1072894203i32);
pub const WR_E_XMLPREFIXDECLARATION: XmlError = XmlError(-1072894202i32);
pub const WR_E_XMLURIDECLARATION: XmlError = XmlError(-1072894201i32);
pub const WR_E_XMLNSURIDECLARATION: XmlError = XmlError(-1072894200i32);
pub const WR_E_NAMESPACEUNDECLARED: XmlError = XmlError(-1072894199i32);
pub const WR_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894198i32);
pub const WR_E_INVALIDACTION: XmlError = XmlError(-1072894197i32);
pub const WR_E_INVALIDSURROGATEPAIR: XmlError = XmlError(-1072894196i32);
pub const XML_E_INVALID_DECIMAL: XmlError = XmlError(-1072898019i32);
pub const XML_E_INVALID_HEXIDECIMAL: XmlError = XmlError(-1072898018i32);
pub const XML_E_INVALID_UNICODE: XmlError = XmlError(-1072898017i32);
pub const XML_E_INVALIDENCODING: XmlError = XmlError(-1072897938i32);
impl ::std::convert::From<i32> for XmlError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XmlError {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlNodeType(pub i32);
pub const XmlNodeType_None: XmlNodeType = XmlNodeType(0i32);
pub const XmlNodeType_Element: XmlNodeType = XmlNodeType(1i32);
pub const XmlNodeType_Attribute: XmlNodeType = XmlNodeType(2i32);
pub const XmlNodeType_Text: XmlNodeType = XmlNodeType(3i32);
pub const XmlNodeType_CDATA: XmlNodeType = XmlNodeType(4i32);
pub const XmlNodeType_ProcessingInstruction: XmlNodeType = XmlNodeType(7i32);
pub const XmlNodeType_Comment: XmlNodeType = XmlNodeType(8i32);
pub const XmlNodeType_DocumentType: XmlNodeType = XmlNodeType(10i32);
pub const XmlNodeType_Whitespace: XmlNodeType = XmlNodeType(13i32);
pub const XmlNodeType_EndElement: XmlNodeType = XmlNodeType(15i32);
pub const XmlNodeType_XmlDeclaration: XmlNodeType = XmlNodeType(17i32);
pub const _XmlNodeType_Last: XmlNodeType = XmlNodeType(17i32);
impl ::std::convert::From<i32> for XmlNodeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XmlNodeType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlReadState(pub i32);
pub const XmlReadState_Initial: XmlReadState = XmlReadState(0i32);
pub const XmlReadState_Interactive: XmlReadState = XmlReadState(1i32);
pub const XmlReadState_Error: XmlReadState = XmlReadState(2i32);
pub const XmlReadState_EndOfFile: XmlReadState = XmlReadState(3i32);
pub const XmlReadState_Closed: XmlReadState = XmlReadState(4i32);
impl ::std::convert::From<i32> for XmlReadState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XmlReadState {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlReaderProperty(pub i32);
pub const XmlReaderProperty_MultiLanguage: XmlReaderProperty = XmlReaderProperty(0i32);
pub const XmlReaderProperty_ConformanceLevel: XmlReaderProperty = XmlReaderProperty(1i32);
pub const XmlReaderProperty_RandomAccess: XmlReaderProperty = XmlReaderProperty(2i32);
pub const XmlReaderProperty_XmlResolver: XmlReaderProperty = XmlReaderProperty(3i32);
pub const XmlReaderProperty_DtdProcessing: XmlReaderProperty = XmlReaderProperty(4i32);
pub const XmlReaderProperty_ReadState: XmlReaderProperty = XmlReaderProperty(5i32);
pub const XmlReaderProperty_MaxElementDepth: XmlReaderProperty = XmlReaderProperty(6i32);
pub const XmlReaderProperty_MaxEntityExpansion: XmlReaderProperty = XmlReaderProperty(7i32);
pub const _XmlReaderProperty_Last: XmlReaderProperty = XmlReaderProperty(7i32);
impl ::std::convert::From<i32> for XmlReaderProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XmlReaderProperty {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlStandalone(pub i32);
pub const XmlStandalone_Omit: XmlStandalone = XmlStandalone(0i32);
pub const XmlStandalone_Yes: XmlStandalone = XmlStandalone(1i32);
pub const XmlStandalone_No: XmlStandalone = XmlStandalone(2i32);
pub const _XmlStandalone_Last: XmlStandalone = XmlStandalone(2i32);
impl ::std::convert::From<i32> for XmlStandalone {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XmlStandalone {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Data_Xml_XmlLite`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlWriterProperty(pub i32);
pub const XmlWriterProperty_MultiLanguage: XmlWriterProperty = XmlWriterProperty(0i32);
pub const XmlWriterProperty_Indent: XmlWriterProperty = XmlWriterProperty(1i32);
pub const XmlWriterProperty_ByteOrderMark: XmlWriterProperty = XmlWriterProperty(2i32);
pub const XmlWriterProperty_OmitXmlDeclaration: XmlWriterProperty = XmlWriterProperty(3i32);
pub const XmlWriterProperty_ConformanceLevel: XmlWriterProperty = XmlWriterProperty(4i32);
pub const XmlWriterProperty_CompactEmptyElement: XmlWriterProperty = XmlWriterProperty(5i32);
pub const _XmlWriterProperty_Last: XmlWriterProperty = XmlWriterProperty(5i32);
impl ::std::convert::From<i32> for XmlWriterProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XmlWriterProperty {
    type Abi = Self;
}
pub const _IID_IXmlReader: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1920597121, 28829, 16533, [182, 61, 105, 254, 75, 13, 144, 48]);
pub const _IID_IXmlResolver: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1920597122, 28829, 16533, [182, 61, 105, 254, 75, 13, 144, 48]);
pub const _IID_IXmlWriter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1920597128, 28829, 16533, [182, 61, 105, 254, 75, 13, 144, 48]);
