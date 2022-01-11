#[cfg(feature = "Win32_Foundation")]
pub trait IXmlReaderImpl: Sized {
    fn SetInput();
    fn GetProperty();
    fn SetProperty();
    fn Read();
    fn GetNodeType();
    fn MoveToFirstAttribute();
    fn MoveToNextAttribute();
    fn MoveToAttributeByName();
    fn MoveToElement();
    fn GetQualifiedName();
    fn GetNamespaceUri();
    fn GetLocalName();
    fn GetPrefix();
    fn GetValue();
    fn ReadValueChunk();
    fn GetBaseUri();
    fn IsDefault();
    fn IsEmptyElement();
    fn GetLineNumber();
    fn GetLinePosition();
    fn GetAttributeCount();
    fn GetDepth();
    fn IsEOF();
}
#[cfg(feature = "Win32_Foundation")]
impl IXmlReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXmlReaderVtbl {
        unsafe extern "system" fn SetInput<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Read<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNodeType<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveToFirstAttribute<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveToNextAttribute<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveToAttributeByName<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveToElement<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQualifiedName<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszqualifiedname: *mut super::super::super::Foundation::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNamespaceUri<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwsznamespaceuri: *mut super::super::super::Foundation::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalName<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszlocalname: *mut super::super::super::Foundation::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrefix<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszprefix: *mut super::super::super::Foundation::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszvalue: *mut super::super::super::Foundation::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadValueChunk<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchbuffer: super::super::super::Foundation::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBaseUri<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszbaseuri: *mut super::super::super::Foundation::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDefault<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEmptyElement<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineNumber<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlinenumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLinePosition<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlineposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeCount<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnattributecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDepth<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pndepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEOF<Impl: IXmlReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetInput::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            Read::<Impl, IMPL_OFFSET>,
            GetNodeType::<Impl, IMPL_OFFSET>,
            MoveToFirstAttribute::<Impl, IMPL_OFFSET>,
            MoveToNextAttribute::<Impl, IMPL_OFFSET>,
            MoveToAttributeByName::<Impl, IMPL_OFFSET>,
            MoveToElement::<Impl, IMPL_OFFSET>,
            GetQualifiedName::<Impl, IMPL_OFFSET>,
            GetNamespaceUri::<Impl, IMPL_OFFSET>,
            GetLocalName::<Impl, IMPL_OFFSET>,
            GetPrefix::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            ReadValueChunk::<Impl, IMPL_OFFSET>,
            GetBaseUri::<Impl, IMPL_OFFSET>,
            IsDefault::<Impl, IMPL_OFFSET>,
            IsEmptyElement::<Impl, IMPL_OFFSET>,
            GetLineNumber::<Impl, IMPL_OFFSET>,
            GetLinePosition::<Impl, IMPL_OFFSET>,
            GetAttributeCount::<Impl, IMPL_OFFSET>,
            GetDepth::<Impl, IMPL_OFFSET>,
            IsEOF::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXmlResolverImpl: Sized {
    fn ResolveUri();
}
#[cfg(feature = "Win32_Foundation")]
impl IXmlResolverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlResolverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXmlResolverVtbl {
        unsafe extern "system" fn ResolveUri<Impl: IXmlResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszbaseuri: super::super::super::Foundation::PWSTR, pwszpublicidentifier: super::super::super::Foundation::PWSTR, pwszsystemidentifier: super::super::super::Foundation::PWSTR, ppresolvedinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ResolveUri::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlResolver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXmlWriterImpl: Sized {
    fn SetOutput();
    fn GetProperty();
    fn SetProperty();
    fn WriteAttributes();
    fn WriteAttributeString();
    fn WriteCData();
    fn WriteCharEntity();
    fn WriteChars();
    fn WriteComment();
    fn WriteDocType();
    fn WriteElementString();
    fn WriteEndDocument();
    fn WriteEndElement();
    fn WriteEntityRef();
    fn WriteFullEndElement();
    fn WriteName();
    fn WriteNmToken();
    fn WriteNode();
    fn WriteNodeShallow();
    fn WriteProcessingInstruction();
    fn WriteQualifiedName();
    fn WriteRaw();
    fn WriteRawChars();
    fn WriteStartDocument();
    fn WriteStartElement();
    fn WriteString();
    fn WriteSurrogateCharEntity();
    fn WriteWhitespace();
    fn Flush();
}
#[cfg(feature = "Win32_Foundation")]
impl IXmlWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXmlWriterVtbl {
        unsafe extern "system" fn SetOutput<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteAttributes<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteAttributeString<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteCData<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteCharEntity<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteChars<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteComment<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteDocType<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteElementString<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteEndDocument<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteEndElement<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteEntityRef<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteFullEndElement<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteName<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteNmToken<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteNode<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteNodeShallow<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteProcessingInstruction<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteQualifiedName<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteRaw<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteRawChars<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteStartDocument<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteStartElement<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteString<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteWhitespace<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IXmlWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetOutput::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            WriteAttributes::<Impl, IMPL_OFFSET>,
            WriteAttributeString::<Impl, IMPL_OFFSET>,
            WriteCData::<Impl, IMPL_OFFSET>,
            WriteCharEntity::<Impl, IMPL_OFFSET>,
            WriteChars::<Impl, IMPL_OFFSET>,
            WriteComment::<Impl, IMPL_OFFSET>,
            WriteDocType::<Impl, IMPL_OFFSET>,
            WriteElementString::<Impl, IMPL_OFFSET>,
            WriteEndDocument::<Impl, IMPL_OFFSET>,
            WriteEndElement::<Impl, IMPL_OFFSET>,
            WriteEntityRef::<Impl, IMPL_OFFSET>,
            WriteFullEndElement::<Impl, IMPL_OFFSET>,
            WriteName::<Impl, IMPL_OFFSET>,
            WriteNmToken::<Impl, IMPL_OFFSET>,
            WriteNode::<Impl, IMPL_OFFSET>,
            WriteNodeShallow::<Impl, IMPL_OFFSET>,
            WriteProcessingInstruction::<Impl, IMPL_OFFSET>,
            WriteQualifiedName::<Impl, IMPL_OFFSET>,
            WriteRaw::<Impl, IMPL_OFFSET>,
            WriteRawChars::<Impl, IMPL_OFFSET>,
            WriteStartDocument::<Impl, IMPL_OFFSET>,
            WriteStartElement::<Impl, IMPL_OFFSET>,
            WriteString::<Impl, IMPL_OFFSET>,
            WriteSurrogateCharEntity::<Impl, IMPL_OFFSET>,
            WriteWhitespace::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXmlWriterLiteImpl: Sized {
    fn SetOutput();
    fn GetProperty();
    fn SetProperty();
    fn WriteAttributes();
    fn WriteAttributeString();
    fn WriteCData();
    fn WriteCharEntity();
    fn WriteChars();
    fn WriteComment();
    fn WriteDocType();
    fn WriteElementString();
    fn WriteEndDocument();
    fn WriteEndElement();
    fn WriteEntityRef();
    fn WriteFullEndElement();
    fn WriteName();
    fn WriteNmToken();
    fn WriteNode();
    fn WriteNodeShallow();
    fn WriteProcessingInstruction();
    fn WriteRaw();
    fn WriteRawChars();
    fn WriteStartDocument();
    fn WriteStartElement();
    fn WriteString();
    fn WriteSurrogateCharEntity();
    fn WriteWhitespace();
    fn Flush();
}
#[cfg(feature = "Win32_Foundation")]
impl IXmlWriterLiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXmlWriterLiteVtbl {
        unsafe extern "system" fn SetOutput<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteAttributes<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteAttributeString<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR, cwszvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteCData<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteCharEntity<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteChars<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteComment<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteDocType<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteElementString<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteEndDocument<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteEndElement<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteEntityRef<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteFullEndElement<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteName<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteNmToken<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteNode<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteNodeShallow<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteProcessingInstruction<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteRaw<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteRawChars<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteStartDocument<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteStartElement<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteString<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteWhitespace<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IXmlWriterLiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetOutput::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            WriteAttributes::<Impl, IMPL_OFFSET>,
            WriteAttributeString::<Impl, IMPL_OFFSET>,
            WriteCData::<Impl, IMPL_OFFSET>,
            WriteCharEntity::<Impl, IMPL_OFFSET>,
            WriteChars::<Impl, IMPL_OFFSET>,
            WriteComment::<Impl, IMPL_OFFSET>,
            WriteDocType::<Impl, IMPL_OFFSET>,
            WriteElementString::<Impl, IMPL_OFFSET>,
            WriteEndDocument::<Impl, IMPL_OFFSET>,
            WriteEndElement::<Impl, IMPL_OFFSET>,
            WriteEntityRef::<Impl, IMPL_OFFSET>,
            WriteFullEndElement::<Impl, IMPL_OFFSET>,
            WriteName::<Impl, IMPL_OFFSET>,
            WriteNmToken::<Impl, IMPL_OFFSET>,
            WriteNode::<Impl, IMPL_OFFSET>,
            WriteNodeShallow::<Impl, IMPL_OFFSET>,
            WriteProcessingInstruction::<Impl, IMPL_OFFSET>,
            WriteRaw::<Impl, IMPL_OFFSET>,
            WriteRawChars::<Impl, IMPL_OFFSET>,
            WriteStartDocument::<Impl, IMPL_OFFSET>,
            WriteStartElement::<Impl, IMPL_OFFSET>,
            WriteString::<Impl, IMPL_OFFSET>,
            WriteSurrogateCharEntity::<Impl, IMPL_OFFSET>,
            WriteWhitespace::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlWriterLite as ::windows::core::Interface>::IID
    }
}
