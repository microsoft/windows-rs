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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>() -> IXmlReader_Vtbl {
        unsafe extern "system" fn SetInput<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInput(::core::mem::transmute(&pinput)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&nproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&nproperty), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Read() {
                ::core::result::Result::Ok(ok__) => {
                    *pnodetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNodeType<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNodeType() {
                ::core::result::Result::Ok(ok__) => {
                    *pnodetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveToFirstAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveToFirstAttribute().into()
        }
        unsafe extern "system" fn MoveToNextAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveToNextAttribute().into()
        }
        unsafe extern "system" fn MoveToAttributeByName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveToAttributeByName(::core::mem::transmute_copy(&pwszlocalname), ::core::mem::transmute_copy(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn MoveToElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveToElement().into()
        }
        unsafe extern "system" fn GetQualifiedName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszqualifiedname: *mut super::super::super::Foundation::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetQualifiedName(::core::mem::transmute_copy(&ppwszqualifiedname), ::core::mem::transmute_copy(&pcwchqualifiedname)).into()
        }
        unsafe extern "system" fn GetNamespaceUri<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwsznamespaceuri: *mut super::super::super::Foundation::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNamespaceUri(::core::mem::transmute_copy(&ppwsznamespaceuri), ::core::mem::transmute_copy(&pcwchnamespaceuri)).into()
        }
        unsafe extern "system" fn GetLocalName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszlocalname: *mut super::super::super::Foundation::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLocalName(::core::mem::transmute_copy(&ppwszlocalname), ::core::mem::transmute_copy(&pcwchlocalname)).into()
        }
        unsafe extern "system" fn GetPrefix<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszprefix: *mut super::super::super::Foundation::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrefix(::core::mem::transmute_copy(&ppwszprefix), ::core::mem::transmute_copy(&pcwchprefix)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszvalue: *mut super::super::super::Foundation::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&ppwszvalue), ::core::mem::transmute_copy(&pcwchvalue)).into()
        }
        unsafe extern "system" fn ReadValueChunk<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchbuffer: super::super::super::Foundation::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadValueChunk(::core::mem::transmute_copy(&pwchbuffer), ::core::mem::transmute_copy(&cwchchunksize), ::core::mem::transmute_copy(&pcwchread)).into()
        }
        unsafe extern "system" fn GetBaseUri<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszbaseuri: *mut super::super::super::Foundation::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBaseUri(::core::mem::transmute_copy(&ppwszbaseuri), ::core::mem::transmute_copy(&pcwchbaseuri)).into()
        }
        unsafe extern "system" fn IsDefault<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDefault()
        }
        unsafe extern "system" fn IsEmptyElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsEmptyElement()
        }
        unsafe extern "system" fn GetLineNumber<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlinenumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pnlinenumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLinePosition<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlineposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLinePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pnlineposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeCount<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnattributecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pnattributecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDepth<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pndepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *pndepth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEOF<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsEOF()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetInput: SetInput::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            GetNodeType: GetNodeType::<Identity, Impl, OFFSET>,
            MoveToFirstAttribute: MoveToFirstAttribute::<Identity, Impl, OFFSET>,
            MoveToNextAttribute: MoveToNextAttribute::<Identity, Impl, OFFSET>,
            MoveToAttributeByName: MoveToAttributeByName::<Identity, Impl, OFFSET>,
            MoveToElement: MoveToElement::<Identity, Impl, OFFSET>,
            GetQualifiedName: GetQualifiedName::<Identity, Impl, OFFSET>,
            GetNamespaceUri: GetNamespaceUri::<Identity, Impl, OFFSET>,
            GetLocalName: GetLocalName::<Identity, Impl, OFFSET>,
            GetPrefix: GetPrefix::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            ReadValueChunk: ReadValueChunk::<Identity, Impl, OFFSET>,
            GetBaseUri: GetBaseUri::<Identity, Impl, OFFSET>,
            IsDefault: IsDefault::<Identity, Impl, OFFSET>,
            IsEmptyElement: IsEmptyElement::<Identity, Impl, OFFSET>,
            GetLineNumber: GetLineNumber::<Identity, Impl, OFFSET>,
            GetLinePosition: GetLinePosition::<Identity, Impl, OFFSET>,
            GetAttributeCount: GetAttributeCount::<Identity, Impl, OFFSET>,
            GetDepth: GetDepth::<Identity, Impl, OFFSET>,
            IsEOF: IsEOF::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlResolver_Impl, const OFFSET: isize>() -> IXmlResolver_Vtbl {
        unsafe extern "system" fn ResolveUri<Identity: ::windows::core::IUnknownImpl, Impl: IXmlResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszbaseuri: super::super::super::Foundation::PWSTR, pwszpublicidentifier: super::super::super::Foundation::PWSTR, pwszsystemidentifier: super::super::super::Foundation::PWSTR, ppresolvedinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResolveUri(::core::mem::transmute_copy(&pwszbaseuri), ::core::mem::transmute_copy(&pwszpublicidentifier), ::core::mem::transmute_copy(&pwszsystemidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresolvedinput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ResolveUri: ResolveUri::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>() -> IXmlWriter_Vtbl {
        unsafe extern "system" fn SetOutput<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutput(::core::mem::transmute(&poutput)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&nproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&nproperty), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn WriteAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteAttributes(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteAttributeString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteAttributeString(::core::mem::transmute_copy(&pwszprefix), ::core::mem::transmute_copy(&pwszlocalname), ::core::mem::transmute_copy(&pwsznamespaceuri), ::core::mem::transmute_copy(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteCData<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteCData(::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteCharEntity<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteCharEntity(::core::mem::transmute_copy(&wch)).into()
        }
        unsafe extern "system" fn WriteChars<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteChars(::core::mem::transmute_copy(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteComment<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteComment(::core::mem::transmute_copy(&pwszcomment)).into()
        }
        unsafe extern "system" fn WriteDocType<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteDocType(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwszpublicid), ::core::mem::transmute_copy(&pwszsystemid), ::core::mem::transmute_copy(&pwszsubset)).into()
        }
        unsafe extern "system" fn WriteElementString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteElementString(::core::mem::transmute_copy(&pwszprefix), ::core::mem::transmute_copy(&pwszlocalname), ::core::mem::transmute_copy(&pwsznamespaceuri), ::core::mem::transmute_copy(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteEndDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEndDocument().into()
        }
        unsafe extern "system" fn WriteEndElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEndElement().into()
        }
        unsafe extern "system" fn WriteEntityRef<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEntityRef(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn WriteFullEndElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteFullEndElement().into()
        }
        unsafe extern "system" fn WriteName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteName(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn WriteNmToken<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteNmToken(::core::mem::transmute_copy(&pwsznmtoken)).into()
        }
        unsafe extern "system" fn WriteNode<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteNode(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteNodeShallow<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteNodeShallow(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteProcessingInstruction<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteProcessingInstruction(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteQualifiedName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteQualifiedName(::core::mem::transmute_copy(&pwszlocalname), ::core::mem::transmute_copy(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn WriteRaw<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteRaw(::core::mem::transmute_copy(&pwszdata)).into()
        }
        unsafe extern "system" fn WriteRawChars<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteRawChars(::core::mem::transmute_copy(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteStartDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStartDocument(::core::mem::transmute_copy(&standalone)).into()
        }
        unsafe extern "system" fn WriteStartElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: super::super::super::Foundation::PWSTR, pwszlocalname: super::super::super::Foundation::PWSTR, pwsznamespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStartElement(::core::mem::transmute_copy(&pwszprefix), ::core::mem::transmute_copy(&pwszlocalname), ::core::mem::transmute_copy(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn WriteString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteString(::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteSurrogateCharEntity(::core::mem::transmute_copy(&wchlow), ::core::mem::transmute_copy(&wchhigh)).into()
        }
        unsafe extern "system" fn WriteWhitespace<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteWhitespace(::core::mem::transmute_copy(&pwszwhitespace)).into()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flush().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetOutput: SetOutput::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            WriteAttributes: WriteAttributes::<Identity, Impl, OFFSET>,
            WriteAttributeString: WriteAttributeString::<Identity, Impl, OFFSET>,
            WriteCData: WriteCData::<Identity, Impl, OFFSET>,
            WriteCharEntity: WriteCharEntity::<Identity, Impl, OFFSET>,
            WriteChars: WriteChars::<Identity, Impl, OFFSET>,
            WriteComment: WriteComment::<Identity, Impl, OFFSET>,
            WriteDocType: WriteDocType::<Identity, Impl, OFFSET>,
            WriteElementString: WriteElementString::<Identity, Impl, OFFSET>,
            WriteEndDocument: WriteEndDocument::<Identity, Impl, OFFSET>,
            WriteEndElement: WriteEndElement::<Identity, Impl, OFFSET>,
            WriteEntityRef: WriteEntityRef::<Identity, Impl, OFFSET>,
            WriteFullEndElement: WriteFullEndElement::<Identity, Impl, OFFSET>,
            WriteName: WriteName::<Identity, Impl, OFFSET>,
            WriteNmToken: WriteNmToken::<Identity, Impl, OFFSET>,
            WriteNode: WriteNode::<Identity, Impl, OFFSET>,
            WriteNodeShallow: WriteNodeShallow::<Identity, Impl, OFFSET>,
            WriteProcessingInstruction: WriteProcessingInstruction::<Identity, Impl, OFFSET>,
            WriteQualifiedName: WriteQualifiedName::<Identity, Impl, OFFSET>,
            WriteRaw: WriteRaw::<Identity, Impl, OFFSET>,
            WriteRawChars: WriteRawChars::<Identity, Impl, OFFSET>,
            WriteStartDocument: WriteStartDocument::<Identity, Impl, OFFSET>,
            WriteStartElement: WriteStartElement::<Identity, Impl, OFFSET>,
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            WriteSurrogateCharEntity: WriteSurrogateCharEntity::<Identity, Impl, OFFSET>,
            WriteWhitespace: WriteWhitespace::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>() -> IXmlWriterLite_Vtbl {
        unsafe extern "system" fn SetOutput<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutput(::core::mem::transmute(&poutput)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&nproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&nproperty), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn WriteAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteAttributes(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteAttributeString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR, cwszvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteAttributeString(::core::mem::transmute_copy(&pwszqname), ::core::mem::transmute_copy(&cwszqname), ::core::mem::transmute_copy(&pwszvalue), ::core::mem::transmute_copy(&cwszvalue)).into()
        }
        unsafe extern "system" fn WriteCData<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteCData(::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteCharEntity<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteCharEntity(::core::mem::transmute_copy(&wch)).into()
        }
        unsafe extern "system" fn WriteChars<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteChars(::core::mem::transmute_copy(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteComment<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteComment(::core::mem::transmute_copy(&pwszcomment)).into()
        }
        unsafe extern "system" fn WriteDocType<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwszpublicid: super::super::super::Foundation::PWSTR, pwszsystemid: super::super::super::Foundation::PWSTR, pwszsubset: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteDocType(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwszpublicid), ::core::mem::transmute_copy(&pwszsystemid), ::core::mem::transmute_copy(&pwszsubset)).into()
        }
        unsafe extern "system" fn WriteElementString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteElementString(::core::mem::transmute_copy(&pwszqname), ::core::mem::transmute_copy(&cwszqname), ::core::mem::transmute_copy(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteEndDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEndDocument().into()
        }
        unsafe extern "system" fn WriteEndElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEndElement(::core::mem::transmute_copy(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteEntityRef<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEntityRef(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn WriteFullEndElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteFullEndElement(::core::mem::transmute_copy(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteName(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn WriteNmToken<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznmtoken: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteNmToken(::core::mem::transmute_copy(&pwsznmtoken)).into()
        }
        unsafe extern "system" fn WriteNode<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteNode(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteNodeShallow<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteNodeShallow(::core::mem::transmute(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteProcessingInstruction<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteProcessingInstruction(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteRaw<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteRaw(::core::mem::transmute_copy(&pwszdata)).into()
        }
        unsafe extern "system" fn WriteRawChars<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: super::super::super::Foundation::PWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteRawChars(::core::mem::transmute_copy(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteStartDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStartDocument(::core::mem::transmute_copy(&standalone)).into()
        }
        unsafe extern "system" fn WriteStartElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: super::super::super::Foundation::PWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStartElement(::core::mem::transmute_copy(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteString(::core::mem::transmute_copy(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteSurrogateCharEntity(::core::mem::transmute_copy(&wchlow), ::core::mem::transmute_copy(&wchhigh)).into()
        }
        unsafe extern "system" fn WriteWhitespace<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszwhitespace: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteWhitespace(::core::mem::transmute_copy(&pwszwhitespace)).into()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flush().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetOutput: SetOutput::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            WriteAttributes: WriteAttributes::<Identity, Impl, OFFSET>,
            WriteAttributeString: WriteAttributeString::<Identity, Impl, OFFSET>,
            WriteCData: WriteCData::<Identity, Impl, OFFSET>,
            WriteCharEntity: WriteCharEntity::<Identity, Impl, OFFSET>,
            WriteChars: WriteChars::<Identity, Impl, OFFSET>,
            WriteComment: WriteComment::<Identity, Impl, OFFSET>,
            WriteDocType: WriteDocType::<Identity, Impl, OFFSET>,
            WriteElementString: WriteElementString::<Identity, Impl, OFFSET>,
            WriteEndDocument: WriteEndDocument::<Identity, Impl, OFFSET>,
            WriteEndElement: WriteEndElement::<Identity, Impl, OFFSET>,
            WriteEntityRef: WriteEntityRef::<Identity, Impl, OFFSET>,
            WriteFullEndElement: WriteFullEndElement::<Identity, Impl, OFFSET>,
            WriteName: WriteName::<Identity, Impl, OFFSET>,
            WriteNmToken: WriteNmToken::<Identity, Impl, OFFSET>,
            WriteNode: WriteNode::<Identity, Impl, OFFSET>,
            WriteNodeShallow: WriteNodeShallow::<Identity, Impl, OFFSET>,
            WriteProcessingInstruction: WriteProcessingInstruction::<Identity, Impl, OFFSET>,
            WriteRaw: WriteRaw::<Identity, Impl, OFFSET>,
            WriteRawChars: WriteRawChars::<Identity, Impl, OFFSET>,
            WriteStartDocument: WriteStartDocument::<Identity, Impl, OFFSET>,
            WriteStartElement: WriteStartElement::<Identity, Impl, OFFSET>,
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            WriteSurrogateCharEntity: WriteSurrogateCharEntity::<Identity, Impl, OFFSET>,
            WriteWhitespace: WriteWhitespace::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlWriterLite as ::windows::core::Interface>::IID
    }
}
