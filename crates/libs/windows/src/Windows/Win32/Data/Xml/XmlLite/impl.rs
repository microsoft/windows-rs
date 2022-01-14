#[cfg(feature = "Win32_Foundation")]
pub trait IXmlReader_Impl: Sized {
    fn SetInput(&mut self, pinput: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, nproperty: u32) -> ::windows::core::Result<isize>;
    fn SetProperty(&mut self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()>;
    fn Read(&mut self) -> ::windows::core::Result<XmlNodeType>;
    fn GetNodeType(&mut self) -> ::windows::core::Result<XmlNodeType>;
    fn MoveToFirstAttribute(&mut self) -> ::windows::core::Result<()>;
    fn MoveToNextAttribute(&mut self) -> ::windows::core::Result<()>;
    fn MoveToAttributeByName(&mut self, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn MoveToElement(&mut self) -> ::windows::core::Result<()>;
    fn GetQualifiedName(&mut self, ppwszqualifiedname: *mut super::super::super::Foundation::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::Result<()>;
    fn GetNamespaceUri(&mut self, ppwsznamespaceuri: *mut super::super::super::Foundation::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::Result<()>;
    fn GetLocalName(&mut self, ppwszlocalname: *mut super::super::super::Foundation::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::Result<()>;
    fn GetPrefix(&mut self, ppwszprefix: *mut super::super::super::Foundation::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::Result<()>;
    fn GetValue(&mut self, ppwszvalue: *mut super::super::super::Foundation::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::Result<()>;
    fn ReadValueChunk(&mut self, pwchbuffer: super::super::super::Foundation::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::Result<()>;
    fn GetBaseUri(&mut self, ppwszbaseuri: *mut super::super::super::Foundation::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::Result<()>;
    fn IsDefault(&mut self) -> super::super::super::Foundation::BOOL;
    fn IsEmptyElement(&mut self) -> super::super::super::Foundation::BOOL;
    fn GetLineNumber(&mut self) -> ::windows::core::Result<u32>;
    fn GetLinePosition(&mut self) -> ::windows::core::Result<u32>;
    fn GetAttributeCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetDepth(&mut self) -> ::windows::core::Result<u32>;
    fn IsEOF(&mut self) -> super::super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IXmlReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXmlReader_Vtbl {
        unsafe extern "system" fn SetInput<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInput(::core::mem::transmute(&pinput)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&nproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&nproperty), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn Read<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read() {
                ::core::result::Result::Ok(ok__) => {
                    *pnodetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNodeType<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNodeType() {
                ::core::result::Result::Ok(ok__) => {
                    *pnodetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveToFirstAttribute<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveToFirstAttribute().into()
        }
        unsafe extern "system" fn MoveToNextAttribute<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveToNextAttribute().into()
        }
        unsafe extern "system" fn MoveToAttributeByName<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveToAttributeByName(::core::mem::transmute_copy(&pwszlocalname), ::core::mem::transmute_copy(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn MoveToElement<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveToElement().into()
        }
        unsafe extern "system" fn GetQualifiedName<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszqualifiedname: *mut super::super::super::Foundation::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQualifiedName(::core::mem::transmute_copy(&ppwszqualifiedname), ::core::mem::transmute_copy(&pcwchqualifiedname)).into()
        }
        unsafe extern "system" fn GetNamespaceUri<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwsznamespaceuri: *mut super::super::super::Foundation::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNamespaceUri(::core::mem::transmute_copy(&ppwsznamespaceuri), ::core::mem::transmute_copy(&pcwchnamespaceuri)).into()
        }
        unsafe extern "system" fn GetLocalName<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszlocalname: *mut super::super::super::Foundation::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocalName(::core::mem::transmute_copy(&ppwszlocalname), ::core::mem::transmute_copy(&pcwchlocalname)).into()
        }
        unsafe extern "system" fn GetPrefix<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszprefix: *mut super::super::super::Foundation::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrefix(::core::mem::transmute_copy(&ppwszprefix), ::core::mem::transmute_copy(&pcwchprefix)).into()
        }
        unsafe extern "system" fn GetValue<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszvalue: *mut super::super::super::Foundation::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&ppwszvalue), ::core::mem::transmute_copy(&pcwchvalue)).into()
        }
        unsafe extern "system" fn ReadValueChunk<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchbuffer: super::super::super::Foundation::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadValueChunk(::core::mem::transmute_copy(&pwchbuffer), ::core::mem::transmute_copy(&cwchchunksize), ::core::mem::transmute_copy(&pcwchread)).into()
        }
        unsafe extern "system" fn GetBaseUri<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszbaseuri: *mut super::super::super::Foundation::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBaseUri(::core::mem::transmute_copy(&ppwszbaseuri), ::core::mem::transmute_copy(&pcwchbaseuri)).into()
        }
        unsafe extern "system" fn IsDefault<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsDefault()
        }
        unsafe extern "system" fn IsEmptyElement<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsEmptyElement()
        }
        unsafe extern "system" fn GetLineNumber<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlinenumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pnlinenumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLinePosition<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlineposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pnlineposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeCount<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnattributecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pnattributecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDepth<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pndepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *pndepth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEOF<Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsEOF()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetInput: SetInput::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            GetNodeType: GetNodeType::<Impl, IMPL_OFFSET>,
            MoveToFirstAttribute: MoveToFirstAttribute::<Impl, IMPL_OFFSET>,
            MoveToNextAttribute: MoveToNextAttribute::<Impl, IMPL_OFFSET>,
            MoveToAttributeByName: MoveToAttributeByName::<Impl, IMPL_OFFSET>,
            MoveToElement: MoveToElement::<Impl, IMPL_OFFSET>,
            GetQualifiedName: GetQualifiedName::<Impl, IMPL_OFFSET>,
            GetNamespaceUri: GetNamespaceUri::<Impl, IMPL_OFFSET>,
            GetLocalName: GetLocalName::<Impl, IMPL_OFFSET>,
            GetPrefix: GetPrefix::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            ReadValueChunk: ReadValueChunk::<Impl, IMPL_OFFSET>,
            GetBaseUri: GetBaseUri::<Impl, IMPL_OFFSET>,
            IsDefault: IsDefault::<Impl, IMPL_OFFSET>,
            IsEmptyElement: IsEmptyElement::<Impl, IMPL_OFFSET>,
            GetLineNumber: GetLineNumber::<Impl, IMPL_OFFSET>,
            GetLinePosition: GetLinePosition::<Impl, IMPL_OFFSET>,
            GetAttributeCount: GetAttributeCount::<Impl, IMPL_OFFSET>,
            GetDepth: GetDepth::<Impl, IMPL_OFFSET>,
            IsEOF: IsEOF::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXmlResolver_Impl: Sized {
    fn ResolveUri(&mut self, pwszbaseuri: super::super::super::Foundation::PWSTR, pwszpublicidentifier: super::super::super::Foundation::PWSTR, pwszsystemidentifier: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXmlResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlResolver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXmlResolver_Vtbl {
        unsafe extern "system" fn ResolveUri<Impl: IXmlResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszbaseuri: super::super::super::Foundation::PWSTR, pwszpublicidentifier: super::super::super::Foundation::PWSTR, pwszsystemidentifier: super::super::super::Foundation::PWSTR, ppresolvedinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveUri(::core::mem::transmute_copy(&pwszbaseuri), ::core::mem::transmute_copy(&pwszpublicidentifier), ::core::mem::transmute_copy(&pwszsystemidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresolvedinput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ResolveUri: ResolveUri::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlResolver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXmlWriter_Impl: Sized {
    fn SetOutput(&mut self, poutput: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, nproperty: u32) -> ::windows::core::Result<isize>;
    fn SetProperty(&mut self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()>;
    fn WriteAttributes(&mut self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteAttributeString(&mut self, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteCData(&mut self, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteCharEntity(&mut self, wch: u16) -> ::windows::core::Result<()>;
    fn WriteChars(&mut self, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::Result<()>;
    fn WriteComment(&mut self, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteDocType(&mut self, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteElementString(&mut self, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteEndDocument(&mut self) -> ::windows::core::Result<()>;
    fn WriteEndElement(&mut self) -> ::windows::core::Result<()>;
    fn WriteEntityRef(&mut self, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteFullEndElement(&mut self) -> ::windows::core::Result<()>;
    fn WriteName(&mut self, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteNmToken(&mut self, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteNode(&mut self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteNodeShallow(&mut self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteProcessingInstruction(&mut self, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteQualifiedName(&mut self, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteRaw(&mut self, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteRawChars(&mut self, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::Result<()>;
    fn WriteStartDocument(&mut self, standalone: XmlStandalone) -> ::windows::core::Result<()>;
    fn WriteStartElement(&mut self, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteString(&mut self, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteSurrogateCharEntity(&mut self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()>;
    fn WriteWhitespace(&mut self, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXmlWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXmlWriter_Vtbl {
        unsafe extern "system" fn SetOutput<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutput(::core::mem::transmute(&poutput)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&nproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&nproperty), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn WriteAttributes<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteAttributes(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteAttributeString<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteAttributeString(::core::mem::transmute_copy(&pwszprefix), ::core::mem::transmute_copy(&pwszlocalname), ::core::mem::transmute_copy(&pwsznamespaceuri), ::core::mem::transmute_copy(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteCData<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteCData(::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteCharEntity<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteCharEntity(::core::mem::transmute_copy(&wch)).into()
        }
        unsafe extern "system" fn WriteChars<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteChars(::core::mem::transmute_copy(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteComment<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteComment(::core::mem::transmute_copy(&pwszcomment)).into()
        }
        unsafe extern "system" fn WriteDocType<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteDocType(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwszpublicid), ::core::mem::transmute_copy(&pwszsystemid), ::core::mem::transmute_copy(&pwszsubset)).into()
        }
        unsafe extern "system" fn WriteElementString<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteElementString(::core::mem::transmute_copy(&pwszprefix), ::core::mem::transmute_copy(&pwszlocalname), ::core::mem::transmute_copy(&pwsznamespaceuri), ::core::mem::transmute_copy(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteEndDocument<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEndDocument().into()
        }
        unsafe extern "system" fn WriteEndElement<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEndElement().into()
        }
        unsafe extern "system" fn WriteEntityRef<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEntityRef(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn WriteFullEndElement<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteFullEndElement().into()
        }
        unsafe extern "system" fn WriteName<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteName(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn WriteNmToken<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteNmToken(::core::mem::transmute_copy(&pwsznmtoken)).into()
        }
        unsafe extern "system" fn WriteNode<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteNode(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteNodeShallow<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteNodeShallow(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteProcessingInstruction<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteProcessingInstruction(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteQualifiedName<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteQualifiedName(::core::mem::transmute_copy(&pwszlocalname), ::core::mem::transmute_copy(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn WriteRaw<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteRaw(::core::mem::transmute_copy(&pwszdata)).into()
        }
        unsafe extern "system" fn WriteRawChars<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteRawChars(::core::mem::transmute_copy(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteStartDocument<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteStartDocument(::core::mem::transmute_copy(&standalone)).into()
        }
        unsafe extern "system" fn WriteStartElement<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteStartElement(::core::mem::transmute_copy(&pwszprefix), ::core::mem::transmute_copy(&pwszlocalname), ::core::mem::transmute_copy(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn WriteString<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteString(::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteSurrogateCharEntity(::core::mem::transmute_copy(&wchlow), ::core::mem::transmute_copy(&wchhigh)).into()
        }
        unsafe extern "system" fn WriteWhitespace<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteWhitespace(::core::mem::transmute_copy(&pwszwhitespace)).into()
        }
        unsafe extern "system" fn Flush<Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetOutput: SetOutput::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            WriteAttributes: WriteAttributes::<Impl, IMPL_OFFSET>,
            WriteAttributeString: WriteAttributeString::<Impl, IMPL_OFFSET>,
            WriteCData: WriteCData::<Impl, IMPL_OFFSET>,
            WriteCharEntity: WriteCharEntity::<Impl, IMPL_OFFSET>,
            WriteChars: WriteChars::<Impl, IMPL_OFFSET>,
            WriteComment: WriteComment::<Impl, IMPL_OFFSET>,
            WriteDocType: WriteDocType::<Impl, IMPL_OFFSET>,
            WriteElementString: WriteElementString::<Impl, IMPL_OFFSET>,
            WriteEndDocument: WriteEndDocument::<Impl, IMPL_OFFSET>,
            WriteEndElement: WriteEndElement::<Impl, IMPL_OFFSET>,
            WriteEntityRef: WriteEntityRef::<Impl, IMPL_OFFSET>,
            WriteFullEndElement: WriteFullEndElement::<Impl, IMPL_OFFSET>,
            WriteName: WriteName::<Impl, IMPL_OFFSET>,
            WriteNmToken: WriteNmToken::<Impl, IMPL_OFFSET>,
            WriteNode: WriteNode::<Impl, IMPL_OFFSET>,
            WriteNodeShallow: WriteNodeShallow::<Impl, IMPL_OFFSET>,
            WriteProcessingInstruction: WriteProcessingInstruction::<Impl, IMPL_OFFSET>,
            WriteQualifiedName: WriteQualifiedName::<Impl, IMPL_OFFSET>,
            WriteRaw: WriteRaw::<Impl, IMPL_OFFSET>,
            WriteRawChars: WriteRawChars::<Impl, IMPL_OFFSET>,
            WriteStartDocument: WriteStartDocument::<Impl, IMPL_OFFSET>,
            WriteStartElement: WriteStartElement::<Impl, IMPL_OFFSET>,
            WriteString: WriteString::<Impl, IMPL_OFFSET>,
            WriteSurrogateCharEntity: WriteSurrogateCharEntity::<Impl, IMPL_OFFSET>,
            WriteWhitespace: WriteWhitespace::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXmlWriterLite_Impl: Sized {
    fn SetOutput(&mut self, poutput: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, nproperty: u32) -> ::windows::core::Result<isize>;
    fn SetProperty(&mut self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()>;
    fn WriteAttributes(&mut self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteAttributeString(&mut self, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR, cwszvalue: u32) -> ::windows::core::Result<()>;
    fn WriteCData(&mut self, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteCharEntity(&mut self, wch: u16) -> ::windows::core::Result<()>;
    fn WriteChars(&mut self, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::Result<()>;
    fn WriteComment(&mut self, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteDocType(&mut self, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteElementString(&mut self, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteEndDocument(&mut self) -> ::windows::core::Result<()>;
    fn WriteEndElement(&mut self, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::Result<()>;
    fn WriteEntityRef(&mut self, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteFullEndElement(&mut self, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::Result<()>;
    fn WriteName(&mut self, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteNmToken(&mut self, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteNode(&mut self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteNodeShallow(&mut self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteProcessingInstruction(&mut self, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteRaw(&mut self, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteRawChars(&mut self, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::Result<()>;
    fn WriteStartDocument(&mut self, standalone: XmlStandalone) -> ::windows::core::Result<()>;
    fn WriteStartElement(&mut self, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::Result<()>;
    fn WriteString(&mut self, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WriteSurrogateCharEntity(&mut self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()>;
    fn WriteWhitespace(&mut self, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXmlWriterLite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXmlWriterLite_Vtbl {
        unsafe extern "system" fn SetOutput<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutput(::core::mem::transmute(&poutput)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&nproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&nproperty), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn WriteAttributes<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteAttributes(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteAttributeString<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR, cwszvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteAttributeString(::core::mem::transmute_copy(&pwszqname), ::core::mem::transmute_copy(&cwszqname), ::core::mem::transmute_copy(&pwszvalue), ::core::mem::transmute_copy(&cwszvalue)).into()
        }
        unsafe extern "system" fn WriteCData<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteCData(::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteCharEntity<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteCharEntity(::core::mem::transmute_copy(&wch)).into()
        }
        unsafe extern "system" fn WriteChars<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteChars(::core::mem::transmute_copy(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteComment<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteComment(::core::mem::transmute_copy(&pwszcomment)).into()
        }
        unsafe extern "system" fn WriteDocType<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteDocType(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwszpublicid), ::core::mem::transmute_copy(&pwszsystemid), ::core::mem::transmute_copy(&pwszsubset)).into()
        }
        unsafe extern "system" fn WriteElementString<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteElementString(::core::mem::transmute_copy(&pwszqname), ::core::mem::transmute_copy(&cwszqname), ::core::mem::transmute_copy(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteEndDocument<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEndDocument().into()
        }
        unsafe extern "system" fn WriteEndElement<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEndElement(::core::mem::transmute_copy(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteEntityRef<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEntityRef(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn WriteFullEndElement<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteFullEndElement(::core::mem::transmute_copy(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteName<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteName(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn WriteNmToken<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteNmToken(::core::mem::transmute_copy(&pwsznmtoken)).into()
        }
        unsafe extern "system" fn WriteNode<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteNode(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteNodeShallow<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteNodeShallow(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteProcessingInstruction<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteProcessingInstruction(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteRaw<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteRaw(::core::mem::transmute_copy(&pwszdata)).into()
        }
        unsafe extern "system" fn WriteRawChars<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteRawChars(::core::mem::transmute_copy(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteStartDocument<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteStartDocument(::core::mem::transmute_copy(&standalone)).into()
        }
        unsafe extern "system" fn WriteStartElement<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteStartElement(::core::mem::transmute_copy(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteString<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteString(::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteSurrogateCharEntity(::core::mem::transmute_copy(&wchlow), ::core::mem::transmute_copy(&wchhigh)).into()
        }
        unsafe extern "system" fn WriteWhitespace<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteWhitespace(::core::mem::transmute_copy(&pwszwhitespace)).into()
        }
        unsafe extern "system" fn Flush<Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetOutput: SetOutput::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            WriteAttributes: WriteAttributes::<Impl, IMPL_OFFSET>,
            WriteAttributeString: WriteAttributeString::<Impl, IMPL_OFFSET>,
            WriteCData: WriteCData::<Impl, IMPL_OFFSET>,
            WriteCharEntity: WriteCharEntity::<Impl, IMPL_OFFSET>,
            WriteChars: WriteChars::<Impl, IMPL_OFFSET>,
            WriteComment: WriteComment::<Impl, IMPL_OFFSET>,
            WriteDocType: WriteDocType::<Impl, IMPL_OFFSET>,
            WriteElementString: WriteElementString::<Impl, IMPL_OFFSET>,
            WriteEndDocument: WriteEndDocument::<Impl, IMPL_OFFSET>,
            WriteEndElement: WriteEndElement::<Impl, IMPL_OFFSET>,
            WriteEntityRef: WriteEntityRef::<Impl, IMPL_OFFSET>,
            WriteFullEndElement: WriteFullEndElement::<Impl, IMPL_OFFSET>,
            WriteName: WriteName::<Impl, IMPL_OFFSET>,
            WriteNmToken: WriteNmToken::<Impl, IMPL_OFFSET>,
            WriteNode: WriteNode::<Impl, IMPL_OFFSET>,
            WriteNodeShallow: WriteNodeShallow::<Impl, IMPL_OFFSET>,
            WriteProcessingInstruction: WriteProcessingInstruction::<Impl, IMPL_OFFSET>,
            WriteRaw: WriteRaw::<Impl, IMPL_OFFSET>,
            WriteRawChars: WriteRawChars::<Impl, IMPL_OFFSET>,
            WriteStartDocument: WriteStartDocument::<Impl, IMPL_OFFSET>,
            WriteStartElement: WriteStartElement::<Impl, IMPL_OFFSET>,
            WriteString: WriteString::<Impl, IMPL_OFFSET>,
            WriteSurrogateCharEntity: WriteSurrogateCharEntity::<Impl, IMPL_OFFSET>,
            WriteWhitespace: WriteWhitespace::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlWriterLite as ::windows::core::Interface>::IID
    }
}
