#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingCodePage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
    pinputstream: Param0,
    pmalloc: Param1,
    nencodingcodepage: u32,
    fencodinghint: Param3,
    pwszbaseuri: Param4,
) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReaderInputWithEncodingCodePage(pinputstream: ::windows::core::RawPtr, pmalloc: ::windows::core::RawPtr, nencodingcodepage: u32, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: super::super::super::Foundation::PWSTR, ppinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateXmlReaderInputWithEncodingCodePage(pinputstream.into_param().abi(), pmalloc.into_param().abi(), ::core::mem::transmute(nencodingcodepage), fencodinghint.into_param().abi(), pwszbaseuri.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
    pinputstream: Param0,
    pmalloc: Param1,
    pwszencodingname: Param2,
    fencodinghint: Param3,
    pwszbaseuri: Param4,
) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReaderInputWithEncodingName(pinputstream: ::windows::core::RawPtr, pmalloc: ::windows::core::RawPtr, pwszencodingname: super::super::super::Foundation::PWSTR, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: super::super::super::Foundation::PWSTR, ppinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateXmlReaderInputWithEncodingName(pinputstream.into_param().abi(), pmalloc.into_param().abi(), pwszencodingname.into_param().abi(), fencodinghint.into_param().abi(), pwszbaseuri.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingCodePage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(poutputstream: Param0, pmalloc: Param1, nencodingcodepage: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriterOutputWithEncodingCodePage(poutputstream: ::windows::core::RawPtr, pmalloc: ::windows::core::RawPtr, nencodingcodepage: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateXmlWriterOutputWithEncodingCodePage(poutputstream.into_param().abi(), pmalloc.into_param().abi(), ::core::mem::transmute(nencodingcodepage), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(poutputstream: Param0, pmalloc: Param1, pwszencodingname: Param2) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriterOutputWithEncodingName(poutputstream: ::windows::core::RawPtr, pmalloc: ::windows::core::RawPtr, pwszencodingname: super::super::super::Foundation::PWSTR, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateXmlWriterOutputWithEncodingName(poutputstream.into_param().abi(), pmalloc.into_param().abi(), pwszencodingname.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DtdProcessing(pub i32);
pub const DtdProcessing_Prohibit: DtdProcessing = DtdProcessing(0i32);
pub const DtdProcessing_Parse: DtdProcessing = DtdProcessing(1i32);
pub const _DtdProcessing_Last: DtdProcessing = DtdProcessing(1i32);
impl ::core::convert::From<i32> for DtdProcessing {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DtdProcessing {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXmlReader(pub ::windows::core::IUnknown);
impl IXmlReader {
    pub unsafe fn SetInput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pinput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pinput.into_param().abi()).ok()
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__: <isize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), &mut result__).from_abi::<isize>(result__)
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn Read(&self) -> ::windows::core::Result<XmlNodeType> {
        let mut result__: <XmlNodeType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XmlNodeType>(result__)
    }
    pub unsafe fn GetNodeType(&self) -> ::windows::core::Result<XmlNodeType> {
        let mut result__: <XmlNodeType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XmlNodeType>(result__)
    }
    pub unsafe fn MoveToFirstAttribute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn MoveToNextAttribute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveToAttributeByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszlocalname: Param0, pwsznamespaceuri: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    pub unsafe fn MoveToElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQualifiedName(&self, ppwszqualifiedname: *mut super::super::super::Foundation::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszqualifiedname), ::core::mem::transmute(pcwchqualifiedname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamespaceUri(&self, ppwsznamespaceuri: *mut super::super::super::Foundation::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwsznamespaceuri), ::core::mem::transmute(pcwchnamespaceuri)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalName(&self, ppwszlocalname: *mut super::super::super::Foundation::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszlocalname), ::core::mem::transmute(pcwchlocalname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrefix(&self, ppwszprefix: *mut super::super::super::Foundation::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszprefix), ::core::mem::transmute(pcwchprefix)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValue(&self, ppwszvalue: *mut super::super::super::Foundation::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszvalue), ::core::mem::transmute(pcwchvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadValueChunk(&self, pwchbuffer: super::super::super::Foundation::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwchbuffer), ::core::mem::transmute(cwchchunksize), ::core::mem::transmute(pcwchread)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBaseUri(&self, ppwszbaseuri: *mut super::super::super::Foundation::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszbaseuri), ::core::mem::transmute(pcwchbaseuri)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDefault(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEmptyElement(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetLineNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetLinePosition(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAttributeCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEOF(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IXmlReader {
    type Vtable = IXmlReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc81_709d_4095_b63d_69fe4b0d9030);
}
impl ::core::convert::From<IXmlReader> for ::windows::core::IUnknown {
    fn from(value: IXmlReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXmlReader> for ::windows::core::IUnknown {
    fn from(value: &IXmlReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwszqualifiedname: *mut super::super::super::Foundation::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwsznamespaceuri: *mut super::super::super::Foundation::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwszlocalname: *mut super::super::super::Foundation::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwszprefix: *mut super::super::super::Foundation::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwszvalue: *mut super::super::super::Foundation::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwchbuffer: super::super::super::Foundation::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwszbaseuri: *mut super::super::super::Foundation::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnlinenumber: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnlineposition: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnattributecount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pndepth: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXmlResolver(pub ::windows::core::IUnknown);
impl IXmlResolver {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResolveUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszbaseuri: Param0, pwszpublicidentifier: Param1, pwszsystemidentifier: Param2) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszbaseuri.into_param().abi(), pwszpublicidentifier.into_param().abi(), pwszsystemidentifier.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for IXmlResolver {
    type Vtable = IXmlResolver_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc82_709d_4095_b63d_69fe4b0d9030);
}
impl ::core::convert::From<IXmlResolver> for ::windows::core::IUnknown {
    fn from(value: IXmlResolver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXmlResolver> for ::windows::core::IUnknown {
    fn from(value: &IXmlResolver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlResolver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlResolver_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszbaseuri: super::super::super::Foundation::PWSTR, pwszpublicidentifier: super::super::super::Foundation::PWSTR, pwszsystemidentifier: super::super::super::Foundation::PWSTR, ppresolvedinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXmlWriter(pub ::windows::core::IUnknown);
impl IXmlWriter {
    pub unsafe fn SetOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, poutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__: <isize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), &mut result__).from_abi::<isize>(result__)
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributes<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributeString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2, pwszvalue: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteChars<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwch.into_param().abi(), ::core::mem::transmute(cwch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteComment<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszcomment: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pwszcomment.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteDocType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwszpublicid: Param1, pwszsystemid: Param2, pwszsubset: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwszpublicid.into_param().abi(), pwszsystemid.into_param().abi(), pwszsubset.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteElementString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2, pwszvalue: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    pub unsafe fn WriteEndDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WriteEndElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteEntityRef<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn WriteFullEndElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNmToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsznmtoken: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pwsznmtoken.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNode<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNodeShallow<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteProcessingInstruction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwsztext: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwsztext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteQualifiedName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszlocalname: Param0, pwsznamespaceuri: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteRaw<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pwszdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteRawChars<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), pwch.into_param().abi(), ::core::mem::transmute(cwch)).ok()
    }
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(standalone)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteStartElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(wchlow), ::core::mem::transmute(wchhigh)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteWhitespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszwhitespace: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), pwszwhitespace.into_param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IXmlWriter {
    type Vtable = IXmlWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc88_709d_4095_b63d_69fe4b0d9030);
}
impl ::core::convert::From<IXmlWriter> for ::windows::core::IUnknown {
    fn from(value: IXmlWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXmlWriter> for ::windows::core::IUnknown {
    fn from(value: &IXmlWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wch: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, standalone: XmlStandalone) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXmlWriterLite(pub ::windows::core::IUnknown);
impl IXmlWriterLite {
    pub unsafe fn SetOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, poutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__: <isize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), &mut result__).from_abi::<isize>(result__)
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributes<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributeString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32, pwszvalue: Param2, cwszvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszqname.into_param().abi(), ::core::mem::transmute(cwszqname), pwszvalue.into_param().abi(), ::core::mem::transmute(cwszvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteChars<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwch.into_param().abi(), ::core::mem::transmute(cwch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteComment<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszcomment: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pwszcomment.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteDocType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwszpublicid: Param1, pwszsystemid: Param2, pwszsubset: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwszpublicid.into_param().abi(), pwszsystemid.into_param().abi(), pwszsubset.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteElementString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32, pwszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwszqname.into_param().abi(), ::core::mem::transmute(cwszqname), pwszvalue.into_param().abi()).ok()
    }
    pub unsafe fn WriteEndDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteEndElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pwszqname.into_param().abi(), ::core::mem::transmute(cwszqname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteEntityRef<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteFullEndElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), pwszqname.into_param().abi(), ::core::mem::transmute(cwszqname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNmToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsznmtoken: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pwsznmtoken.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNode<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNodeShallow<'a, Param0: ::windows::core::IntoParam<'a, IXmlReader>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteProcessingInstruction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwsztext: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwsztext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteRaw<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pwszdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteRawChars<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwch: Param0, cwch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pwch.into_param().abi(), ::core::mem::transmute(cwch)).ok()
    }
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(standalone)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteStartElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszqname: Param0, cwszqname: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), pwszqname.into_param().abi(), ::core::mem::transmute(cwszqname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwsztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), pwsztext.into_param().abi()).ok()
    }
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(wchlow), ::core::mem::transmute(wchhigh)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteWhitespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszwhitespace: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), pwszwhitespace.into_param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IXmlWriterLite {
    type Vtable = IXmlWriterLite_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x862494c6_1310_4aad_b3cd_2dbeebf670d3);
}
impl ::core::convert::From<IXmlWriterLite> for ::windows::core::IUnknown {
    fn from(value: IXmlWriterLite) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXmlWriterLite> for ::windows::core::IUnknown {
    fn from(value: &IXmlWriterLite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXmlWriterLite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXmlWriterLite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriterLite_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR, cwszvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wch: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, standalone: XmlStandalone) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlConformanceLevel(pub i32);
pub const XmlConformanceLevel_Auto: XmlConformanceLevel = XmlConformanceLevel(0i32);
pub const XmlConformanceLevel_Fragment: XmlConformanceLevel = XmlConformanceLevel(1i32);
pub const XmlConformanceLevel_Document: XmlConformanceLevel = XmlConformanceLevel(2i32);
pub const _XmlConformanceLevel_Last: XmlConformanceLevel = XmlConformanceLevel(2i32);
impl ::core::convert::From<i32> for XmlConformanceLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XmlConformanceLevel {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for XmlError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XmlError {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for XmlNodeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XmlNodeType {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlReadState(pub i32);
pub const XmlReadState_Initial: XmlReadState = XmlReadState(0i32);
pub const XmlReadState_Interactive: XmlReadState = XmlReadState(1i32);
pub const XmlReadState_Error: XmlReadState = XmlReadState(2i32);
pub const XmlReadState_EndOfFile: XmlReadState = XmlReadState(3i32);
pub const XmlReadState_Closed: XmlReadState = XmlReadState(4i32);
impl ::core::convert::From<i32> for XmlReadState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XmlReadState {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for XmlReaderProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XmlReaderProperty {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlStandalone(pub i32);
pub const XmlStandalone_Omit: XmlStandalone = XmlStandalone(0i32);
pub const XmlStandalone_Yes: XmlStandalone = XmlStandalone(1i32);
pub const XmlStandalone_No: XmlStandalone = XmlStandalone(2i32);
pub const _XmlStandalone_Last: XmlStandalone = XmlStandalone(2i32);
impl ::core::convert::From<i32> for XmlStandalone {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XmlStandalone {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XmlWriterProperty(pub i32);
pub const XmlWriterProperty_MultiLanguage: XmlWriterProperty = XmlWriterProperty(0i32);
pub const XmlWriterProperty_Indent: XmlWriterProperty = XmlWriterProperty(1i32);
pub const XmlWriterProperty_ByteOrderMark: XmlWriterProperty = XmlWriterProperty(2i32);
pub const XmlWriterProperty_OmitXmlDeclaration: XmlWriterProperty = XmlWriterProperty(3i32);
pub const XmlWriterProperty_ConformanceLevel: XmlWriterProperty = XmlWriterProperty(4i32);
pub const XmlWriterProperty_CompactEmptyElement: XmlWriterProperty = XmlWriterProperty(5i32);
pub const _XmlWriterProperty_Last: XmlWriterProperty = XmlWriterProperty(5i32);
impl ::core::convert::From<i32> for XmlWriterProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XmlWriterProperty {
    type Abi = Self;
}
pub const _IID_IXmlReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc81_709d_4095_b63d_69fe4b0d9030);
pub const _IID_IXmlResolver: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc82_709d_4095_b63d_69fe4b0d9030);
pub const _IID_IXmlWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc88_709d_4095_b63d_69fe4b0d9030);
