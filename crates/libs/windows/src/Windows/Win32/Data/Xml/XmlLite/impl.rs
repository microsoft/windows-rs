#[cfg(feature = "Win32_Foundation")]
pub trait IXmlReader_Impl: Sized {
    fn SetInput(&self, pinput: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize>;
    fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()>;
    fn Read(&self) -> ::windows::core::Result<XmlNodeType>;
    fn GetNodeType(&self) -> ::windows::core::Result<XmlNodeType>;
    fn MoveToFirstAttribute(&self) -> ::windows::core::Result<()>;
    fn MoveToNextAttribute(&self) -> ::windows::core::Result<()>;
    fn MoveToAttributeByName(&self, pwszlocalname: &::windows::core::PCWSTR, pwsznamespaceuri: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn MoveToElement(&self) -> ::windows::core::Result<()>;
    fn GetQualifiedName(&self, ppwszqualifiedname: *mut ::windows::core::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::Result<()>;
    fn GetNamespaceUri(&self, ppwsznamespaceuri: *mut ::windows::core::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::Result<()>;
    fn GetLocalName(&self, ppwszlocalname: *mut ::windows::core::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::Result<()>;
    fn GetPrefix(&self, ppwszprefix: *mut ::windows::core::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::Result<()>;
    fn GetValue(&self, ppwszvalue: *mut ::windows::core::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::Result<()>;
    fn ReadValueChunk(&self, pwchbuffer: ::windows::core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::Result<()>;
    fn GetBaseUri(&self, ppwszbaseuri: *mut ::windows::core::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::Result<()>;
    fn IsDefault(&self) -> super::super::super::Foundation::BOOL;
    fn IsEmptyElement(&self) -> super::super::super::Foundation::BOOL;
    fn GetLineNumber(&self) -> ::windows::core::Result<u32>;
    fn GetLinePosition(&self) -> ::windows::core::Result<u32>;
    fn GetAttributeCount(&self) -> ::windows::core::Result<u32>;
    fn GetDepth(&self) -> ::windows::core::Result<u32>;
    fn IsEOF(&self) -> super::super::super::Foundation::BOOL;
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
        unsafe extern "system" fn MoveToAttributeByName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveToAttributeByName(::core::mem::transmute(&pwszlocalname), ::core::mem::transmute(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn MoveToElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveToElement().into()
        }
        unsafe extern "system" fn GetQualifiedName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszqualifiedname: *mut ::windows::core::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetQualifiedName(::core::mem::transmute_copy(&ppwszqualifiedname), ::core::mem::transmute_copy(&pcwchqualifiedname)).into()
        }
        unsafe extern "system" fn GetNamespaceUri<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwsznamespaceuri: *mut ::windows::core::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNamespaceUri(::core::mem::transmute_copy(&ppwsznamespaceuri), ::core::mem::transmute_copy(&pcwchnamespaceuri)).into()
        }
        unsafe extern "system" fn GetLocalName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszlocalname: *mut ::windows::core::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLocalName(::core::mem::transmute_copy(&ppwszlocalname), ::core::mem::transmute_copy(&pcwchlocalname)).into()
        }
        unsafe extern "system" fn GetPrefix<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszprefix: *mut ::windows::core::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrefix(::core::mem::transmute_copy(&ppwszprefix), ::core::mem::transmute_copy(&pcwchprefix)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszvalue: *mut ::windows::core::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&ppwszvalue), ::core::mem::transmute_copy(&pcwchvalue)).into()
        }
        unsafe extern "system" fn ReadValueChunk<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchbuffer: ::windows::core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadValueChunk(::core::mem::transmute_copy(&pwchbuffer), ::core::mem::transmute_copy(&cwchchunksize), ::core::mem::transmute_copy(&pcwchread)).into()
        }
        unsafe extern "system" fn GetBaseUri<Identity: ::windows::core::IUnknownImpl, Impl: IXmlReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszbaseuri: *mut ::windows::core::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IXmlResolver_Impl: Sized {
    fn ResolveUri(&self, pwszbaseuri: &::windows::core::PCWSTR, pwszpublicidentifier: &::windows::core::PCWSTR, pwszsystemidentifier: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IXmlResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlResolver_Impl, const OFFSET: isize>() -> IXmlResolver_Vtbl {
        unsafe extern "system" fn ResolveUri<Identity: ::windows::core::IUnknownImpl, Impl: IXmlResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszbaseuri: ::windows::core::PCWSTR, pwszpublicidentifier: ::windows::core::PCWSTR, pwszsystemidentifier: ::windows::core::PCWSTR, ppresolvedinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResolveUri(::core::mem::transmute(&pwszbaseuri), ::core::mem::transmute(&pwszpublicidentifier), ::core::mem::transmute(&pwszsystemidentifier)) {
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
    fn SetOutput(&self, poutput: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize>;
    fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()>;
    fn WriteAttributes(&self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteAttributeString(&self, pwszprefix: &::windows::core::PCWSTR, pwszlocalname: &::windows::core::PCWSTR, pwsznamespaceuri: &::windows::core::PCWSTR, pwszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteCData(&self, pwsztext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteCharEntity(&self, wch: u16) -> ::windows::core::Result<()>;
    fn WriteChars(&self, pwch: &::windows::core::PCWSTR, cwch: u32) -> ::windows::core::Result<()>;
    fn WriteComment(&self, pwszcomment: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteDocType(&self, pwszname: &::windows::core::PCWSTR, pwszpublicid: &::windows::core::PCWSTR, pwszsystemid: &::windows::core::PCWSTR, pwszsubset: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteElementString(&self, pwszprefix: &::windows::core::PCWSTR, pwszlocalname: &::windows::core::PCWSTR, pwsznamespaceuri: &::windows::core::PCWSTR, pwszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteEndDocument(&self) -> ::windows::core::Result<()>;
    fn WriteEndElement(&self) -> ::windows::core::Result<()>;
    fn WriteEntityRef(&self, pwszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteFullEndElement(&self) -> ::windows::core::Result<()>;
    fn WriteName(&self, pwszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteNmToken(&self, pwsznmtoken: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteNode(&self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteNodeShallow(&self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteProcessingInstruction(&self, pwszname: &::windows::core::PCWSTR, pwsztext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteQualifiedName(&self, pwszlocalname: &::windows::core::PCWSTR, pwsznamespaceuri: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteRaw(&self, pwszdata: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteRawChars(&self, pwch: &::windows::core::PCWSTR, cwch: u32) -> ::windows::core::Result<()>;
    fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::core::Result<()>;
    fn WriteStartElement(&self, pwszprefix: &::windows::core::PCWSTR, pwszlocalname: &::windows::core::PCWSTR, pwsznamespaceuri: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteString(&self, pwsztext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()>;
    fn WriteWhitespace(&self, pwszwhitespace: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Flush(&self) -> ::windows::core::Result<()>;
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
        unsafe extern "system" fn WriteAttributeString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: ::windows::core::PCWSTR, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteAttributeString(::core::mem::transmute(&pwszprefix), ::core::mem::transmute(&pwszlocalname), ::core::mem::transmute(&pwsznamespaceuri), ::core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteCData<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteCData(::core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteCharEntity<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteCharEntity(::core::mem::transmute_copy(&wch)).into()
        }
        unsafe extern "system" fn WriteChars<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteChars(::core::mem::transmute(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteComment<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteComment(::core::mem::transmute(&pwszcomment)).into()
        }
        unsafe extern "system" fn WriteDocType<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwszpublicid: ::windows::core::PCWSTR, pwszsystemid: ::windows::core::PCWSTR, pwszsubset: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteDocType(::core::mem::transmute(&pwszname), ::core::mem::transmute(&pwszpublicid), ::core::mem::transmute(&pwszsystemid), ::core::mem::transmute(&pwszsubset)).into()
        }
        unsafe extern "system" fn WriteElementString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: ::windows::core::PCWSTR, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteElementString(::core::mem::transmute(&pwszprefix), ::core::mem::transmute(&pwszlocalname), ::core::mem::transmute(&pwsznamespaceuri), ::core::mem::transmute(&pwszvalue)).into()
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
        unsafe extern "system" fn WriteEntityRef<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEntityRef(::core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn WriteFullEndElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteFullEndElement().into()
        }
        unsafe extern "system" fn WriteName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteName(::core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn WriteNmToken<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznmtoken: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteNmToken(::core::mem::transmute(&pwsznmtoken)).into()
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
        unsafe extern "system" fn WriteProcessingInstruction<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteProcessingInstruction(::core::mem::transmute(&pwszname), ::core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteQualifiedName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteQualifiedName(::core::mem::transmute(&pwszlocalname), ::core::mem::transmute(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn WriteRaw<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteRaw(::core::mem::transmute(&pwszdata)).into()
        }
        unsafe extern "system" fn WriteRawChars<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteRawChars(::core::mem::transmute(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteStartDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStartDocument(::core::mem::transmute_copy(&standalone)).into()
        }
        unsafe extern "system" fn WriteStartElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprefix: ::windows::core::PCWSTR, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStartElement(::core::mem::transmute(&pwszprefix), ::core::mem::transmute(&pwszlocalname), ::core::mem::transmute(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn WriteString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteString(::core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteSurrogateCharEntity(::core::mem::transmute_copy(&wchlow), ::core::mem::transmute_copy(&wchhigh)).into()
        }
        unsafe extern "system" fn WriteWhitespace<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszwhitespace: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteWhitespace(::core::mem::transmute(&pwszwhitespace)).into()
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
    fn SetOutput(&self, poutput: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize>;
    fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()>;
    fn WriteAttributes(&self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteAttributeString(&self, pwszqname: &::windows::core::PCWSTR, cwszqname: u32, pwszvalue: &::windows::core::PCWSTR, cwszvalue: u32) -> ::windows::core::Result<()>;
    fn WriteCData(&self, pwsztext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteCharEntity(&self, wch: u16) -> ::windows::core::Result<()>;
    fn WriteChars(&self, pwch: &::windows::core::PCWSTR, cwch: u32) -> ::windows::core::Result<()>;
    fn WriteComment(&self, pwszcomment: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteDocType(&self, pwszname: &::windows::core::PCWSTR, pwszpublicid: &::windows::core::PCWSTR, pwszsystemid: &::windows::core::PCWSTR, pwszsubset: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteElementString(&self, pwszqname: &::windows::core::PCWSTR, cwszqname: u32, pwszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteEndDocument(&self) -> ::windows::core::Result<()>;
    fn WriteEndElement(&self, pwszqname: &::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::Result<()>;
    fn WriteEntityRef(&self, pwszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteFullEndElement(&self, pwszqname: &::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::Result<()>;
    fn WriteName(&self, pwszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteNmToken(&self, pwsznmtoken: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteNode(&self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteNodeShallow(&self, preader: &::core::option::Option<IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteProcessingInstruction(&self, pwszname: &::windows::core::PCWSTR, pwsztext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteRaw(&self, pwszdata: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteRawChars(&self, pwch: &::windows::core::PCWSTR, cwch: u32) -> ::windows::core::Result<()>;
    fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::core::Result<()>;
    fn WriteStartElement(&self, pwszqname: &::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::Result<()>;
    fn WriteString(&self, pwsztext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()>;
    fn WriteWhitespace(&self, pwszwhitespace: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Flush(&self) -> ::windows::core::Result<()>;
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
        unsafe extern "system" fn WriteAttributeString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32, pwszvalue: ::windows::core::PCWSTR, cwszvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteAttributeString(::core::mem::transmute(&pwszqname), ::core::mem::transmute_copy(&cwszqname), ::core::mem::transmute(&pwszvalue), ::core::mem::transmute_copy(&cwszvalue)).into()
        }
        unsafe extern "system" fn WriteCData<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteCData(::core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteCharEntity<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteCharEntity(::core::mem::transmute_copy(&wch)).into()
        }
        unsafe extern "system" fn WriteChars<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteChars(::core::mem::transmute(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteComment<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteComment(::core::mem::transmute(&pwszcomment)).into()
        }
        unsafe extern "system" fn WriteDocType<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwszpublicid: ::windows::core::PCWSTR, pwszsystemid: ::windows::core::PCWSTR, pwszsubset: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteDocType(::core::mem::transmute(&pwszname), ::core::mem::transmute(&pwszpublicid), ::core::mem::transmute(&pwszsystemid), ::core::mem::transmute(&pwszsubset)).into()
        }
        unsafe extern "system" fn WriteElementString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteElementString(::core::mem::transmute(&pwszqname), ::core::mem::transmute_copy(&cwszqname), ::core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteEndDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEndDocument().into()
        }
        unsafe extern "system" fn WriteEndElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEndElement(::core::mem::transmute(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteEntityRef<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEntityRef(::core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn WriteFullEndElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteFullEndElement(::core::mem::transmute(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteName(::core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn WriteNmToken<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznmtoken: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteNmToken(::core::mem::transmute(&pwsznmtoken)).into()
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
        unsafe extern "system" fn WriteProcessingInstruction<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteProcessingInstruction(::core::mem::transmute(&pwszname), ::core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteRaw<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteRaw(::core::mem::transmute(&pwszdata)).into()
        }
        unsafe extern "system" fn WriteRawChars<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteRawChars(::core::mem::transmute(&pwch), ::core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteStartDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStartDocument(::core::mem::transmute_copy(&standalone)).into()
        }
        unsafe extern "system" fn WriteStartElement<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStartElement(::core::mem::transmute(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteString<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteString(::core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteSurrogateCharEntity(::core::mem::transmute_copy(&wchlow), ::core::mem::transmute_copy(&wchhigh)).into()
        }
        unsafe extern "system" fn WriteWhitespace<Identity: ::windows::core::IUnknownImpl, Impl: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszwhitespace: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteWhitespace(::core::mem::transmute(&pwszwhitespace)).into()
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
