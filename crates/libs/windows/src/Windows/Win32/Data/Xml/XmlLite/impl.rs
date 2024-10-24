pub trait IXmlReader_Impl: Sized + windows_core::IUnknownImpl {
    fn SetInput(&self, pinput: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetProperty(&self, nproperty: u32) -> windows_core::Result<isize>;
    fn SetProperty(&self, nproperty: u32, pvalue: isize) -> windows_core::Result<()>;
    fn Read(&self, pnodetype: *mut XmlNodeType) -> windows_core::HRESULT;
    fn GetNodeType(&self) -> windows_core::Result<XmlNodeType>;
    fn MoveToFirstAttribute(&self) -> windows_core::HRESULT;
    fn MoveToNextAttribute(&self) -> windows_core::HRESULT;
    fn MoveToAttributeByName(&self, pwszlocalname: &windows_core::PCWSTR, pwsznamespaceuri: &windows_core::PCWSTR) -> windows_core::HRESULT;
    fn MoveToElement(&self) -> windows_core::Result<()>;
    fn GetQualifiedName(&self, ppwszqualifiedname: *mut windows_core::PCWSTR, pcwchqualifiedname: *mut u32) -> windows_core::Result<()>;
    fn GetNamespaceUri(&self, ppwsznamespaceuri: *mut windows_core::PCWSTR, pcwchnamespaceuri: *mut u32) -> windows_core::Result<()>;
    fn GetLocalName(&self, ppwszlocalname: *mut windows_core::PCWSTR, pcwchlocalname: *mut u32) -> windows_core::Result<()>;
    fn GetPrefix(&self, ppwszprefix: *mut windows_core::PCWSTR, pcwchprefix: *mut u32) -> windows_core::Result<()>;
    fn GetValue(&self, ppwszvalue: *mut windows_core::PCWSTR, pcwchvalue: *mut u32) -> windows_core::Result<()>;
    fn ReadValueChunk(&self, pwchbuffer: windows_core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> windows_core::HRESULT;
    fn GetBaseUri(&self, ppwszbaseuri: *mut windows_core::PCWSTR, pcwchbaseuri: *mut u32) -> windows_core::Result<()>;
    fn IsDefault(&self) -> super::super::super::Foundation::BOOL;
    fn IsEmptyElement(&self) -> super::super::super::Foundation::BOOL;
    fn GetLineNumber(&self) -> windows_core::Result<u32>;
    fn GetLinePosition(&self) -> windows_core::Result<u32>;
    fn GetAttributeCount(&self) -> windows_core::Result<u32>;
    fn GetDepth(&self) -> windows_core::Result<u32>;
    fn IsEOF(&self) -> super::super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IXmlReader {}
impl IXmlReader_Vtbl {
    pub const fn new<Identity: IXmlReader_Impl, const OFFSET: isize>() -> IXmlReader_Vtbl {
        unsafe extern "system" fn SetInput<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::SetInput(this, windows_core::from_raw_borrowed(&pinput)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlReader_Impl::GetProperty(this, core::mem::transmute_copy(&nproperty)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, pvalue: isize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::SetProperty(this, core::mem::transmute_copy(&nproperty), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn Read<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnodetype: *mut XmlNodeType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::Read(this, core::mem::transmute_copy(&pnodetype))
        }
        unsafe extern "system" fn GetNodeType<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnodetype: *mut XmlNodeType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlReader_Impl::GetNodeType(this) {
                Ok(ok__) => {
                    pnodetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveToFirstAttribute<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::MoveToFirstAttribute(this)
        }
        unsafe extern "system" fn MoveToNextAttribute<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::MoveToNextAttribute(this)
        }
        unsafe extern "system" fn MoveToAttributeByName<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszlocalname: windows_core::PCWSTR, pwsznamespaceuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::MoveToAttributeByName(this, core::mem::transmute(&pwszlocalname), core::mem::transmute(&pwsznamespaceuri))
        }
        unsafe extern "system" fn MoveToElement<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::MoveToElement(this).into()
        }
        unsafe extern "system" fn GetQualifiedName<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszqualifiedname: *mut windows_core::PCWSTR, pcwchqualifiedname: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::GetQualifiedName(this, core::mem::transmute_copy(&ppwszqualifiedname), core::mem::transmute_copy(&pcwchqualifiedname)).into()
        }
        unsafe extern "system" fn GetNamespaceUri<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwsznamespaceuri: *mut windows_core::PCWSTR, pcwchnamespaceuri: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::GetNamespaceUri(this, core::mem::transmute_copy(&ppwsznamespaceuri), core::mem::transmute_copy(&pcwchnamespaceuri)).into()
        }
        unsafe extern "system" fn GetLocalName<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszlocalname: *mut windows_core::PCWSTR, pcwchlocalname: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::GetLocalName(this, core::mem::transmute_copy(&ppwszlocalname), core::mem::transmute_copy(&pcwchlocalname)).into()
        }
        unsafe extern "system" fn GetPrefix<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszprefix: *mut windows_core::PCWSTR, pcwchprefix: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::GetPrefix(this, core::mem::transmute_copy(&ppwszprefix), core::mem::transmute_copy(&pcwchprefix)).into()
        }
        unsafe extern "system" fn GetValue<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszvalue: *mut windows_core::PCWSTR, pcwchvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::GetValue(this, core::mem::transmute_copy(&ppwszvalue), core::mem::transmute_copy(&pcwchvalue)).into()
        }
        unsafe extern "system" fn ReadValueChunk<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchbuffer: windows_core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::ReadValueChunk(this, core::mem::transmute_copy(&pwchbuffer), core::mem::transmute_copy(&cwchchunksize), core::mem::transmute_copy(&pcwchread))
        }
        unsafe extern "system" fn GetBaseUri<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszbaseuri: *mut windows_core::PCWSTR, pcwchbaseuri: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::GetBaseUri(this, core::mem::transmute_copy(&ppwszbaseuri), core::mem::transmute_copy(&pcwchbaseuri)).into()
        }
        unsafe extern "system" fn IsDefault<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::IsDefault(this)
        }
        unsafe extern "system" fn IsEmptyElement<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::IsEmptyElement(this)
        }
        unsafe extern "system" fn GetLineNumber<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnlinenumber: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlReader_Impl::GetLineNumber(this) {
                Ok(ok__) => {
                    pnlinenumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLinePosition<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnlineposition: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlReader_Impl::GetLinePosition(this) {
                Ok(ok__) => {
                    pnlineposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeCount<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnattributecount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlReader_Impl::GetAttributeCount(this) {
                Ok(ok__) => {
                    pnattributecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDepth<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pndepth: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlReader_Impl::GetDepth(this) {
                Ok(ok__) => {
                    pndepth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEOF<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlReader_Impl::IsEOF(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInput: SetInput::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            GetNodeType: GetNodeType::<Identity, OFFSET>,
            MoveToFirstAttribute: MoveToFirstAttribute::<Identity, OFFSET>,
            MoveToNextAttribute: MoveToNextAttribute::<Identity, OFFSET>,
            MoveToAttributeByName: MoveToAttributeByName::<Identity, OFFSET>,
            MoveToElement: MoveToElement::<Identity, OFFSET>,
            GetQualifiedName: GetQualifiedName::<Identity, OFFSET>,
            GetNamespaceUri: GetNamespaceUri::<Identity, OFFSET>,
            GetLocalName: GetLocalName::<Identity, OFFSET>,
            GetPrefix: GetPrefix::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            ReadValueChunk: ReadValueChunk::<Identity, OFFSET>,
            GetBaseUri: GetBaseUri::<Identity, OFFSET>,
            IsDefault: IsDefault::<Identity, OFFSET>,
            IsEmptyElement: IsEmptyElement::<Identity, OFFSET>,
            GetLineNumber: GetLineNumber::<Identity, OFFSET>,
            GetLinePosition: GetLinePosition::<Identity, OFFSET>,
            GetAttributeCount: GetAttributeCount::<Identity, OFFSET>,
            GetDepth: GetDepth::<Identity, OFFSET>,
            IsEOF: IsEOF::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlReader as windows_core::Interface>::IID
    }
}
pub trait IXmlResolver_Impl: Sized + windows_core::IUnknownImpl {
    fn ResolveUri(&self, pwszbaseuri: &windows_core::PCWSTR, pwszpublicidentifier: &windows_core::PCWSTR, pwszsystemidentifier: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IXmlResolver {}
impl IXmlResolver_Vtbl {
    pub const fn new<Identity: IXmlResolver_Impl, const OFFSET: isize>() -> IXmlResolver_Vtbl {
        unsafe extern "system" fn ResolveUri<Identity: IXmlResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszbaseuri: windows_core::PCWSTR, pwszpublicidentifier: windows_core::PCWSTR, pwszsystemidentifier: windows_core::PCWSTR, ppresolvedinput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlResolver_Impl::ResolveUri(this, core::mem::transmute(&pwszbaseuri), core::mem::transmute(&pwszpublicidentifier), core::mem::transmute(&pwszsystemidentifier)) {
                Ok(ok__) => {
                    ppresolvedinput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ResolveUri: ResolveUri::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlResolver as windows_core::Interface>::IID
    }
}
pub trait IXmlWriter_Impl: Sized + windows_core::IUnknownImpl {
    fn SetOutput(&self, poutput: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetProperty(&self, nproperty: u32) -> windows_core::Result<isize>;
    fn SetProperty(&self, nproperty: u32, pvalue: isize) -> windows_core::Result<()>;
    fn WriteAttributes(&self, preader: Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn WriteAttributeString(&self, pwszprefix: &windows_core::PCWSTR, pwszlocalname: &windows_core::PCWSTR, pwsznamespaceuri: &windows_core::PCWSTR, pwszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteCData(&self, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteCharEntity(&self, wch: u16) -> windows_core::Result<()>;
    fn WriteChars(&self, pwch: &windows_core::PCWSTR, cwch: u32) -> windows_core::Result<()>;
    fn WriteComment(&self, pwszcomment: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteDocType(&self, pwszname: &windows_core::PCWSTR, pwszpublicid: &windows_core::PCWSTR, pwszsystemid: &windows_core::PCWSTR, pwszsubset: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteElementString(&self, pwszprefix: &windows_core::PCWSTR, pwszlocalname: &windows_core::PCWSTR, pwsznamespaceuri: &windows_core::PCWSTR, pwszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteEndDocument(&self) -> windows_core::Result<()>;
    fn WriteEndElement(&self) -> windows_core::Result<()>;
    fn WriteEntityRef(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteFullEndElement(&self) -> windows_core::Result<()>;
    fn WriteName(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteNmToken(&self, pwsznmtoken: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteNode(&self, preader: Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn WriteNodeShallow(&self, preader: Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn WriteProcessingInstruction(&self, pwszname: &windows_core::PCWSTR, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteQualifiedName(&self, pwszlocalname: &windows_core::PCWSTR, pwsznamespaceuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteRaw(&self, pwszdata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteRawChars(&self, pwch: &windows_core::PCWSTR, cwch: u32) -> windows_core::Result<()>;
    fn WriteStartDocument(&self, standalone: XmlStandalone) -> windows_core::Result<()>;
    fn WriteStartElement(&self, pwszprefix: &windows_core::PCWSTR, pwszlocalname: &windows_core::PCWSTR, pwsznamespaceuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteString(&self, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> windows_core::Result<()>;
    fn WriteWhitespace(&self, pwszwhitespace: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXmlWriter {}
impl IXmlWriter_Vtbl {
    pub const fn new<Identity: IXmlWriter_Impl, const OFFSET: isize>() -> IXmlWriter_Vtbl {
        unsafe extern "system" fn SetOutput<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::SetOutput(this, windows_core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlWriter_Impl::GetProperty(this, core::mem::transmute_copy(&nproperty)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, pvalue: isize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::SetProperty(this, core::mem::transmute_copy(&nproperty), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn WriteAttributes<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteAttributes(this, windows_core::from_raw_borrowed(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteAttributeString<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprefix: windows_core::PCWSTR, pwszlocalname: windows_core::PCWSTR, pwsznamespaceuri: windows_core::PCWSTR, pwszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteAttributeString(this, core::mem::transmute(&pwszprefix), core::mem::transmute(&pwszlocalname), core::mem::transmute(&pwsznamespaceuri), core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteCData<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteCData(this, core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteCharEntity<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wch: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteCharEntity(this, core::mem::transmute_copy(&wch)).into()
        }
        unsafe extern "system" fn WriteChars<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwch: windows_core::PCWSTR, cwch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteChars(this, core::mem::transmute(&pwch), core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteComment<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcomment: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteComment(this, core::mem::transmute(&pwszcomment)).into()
        }
        unsafe extern "system" fn WriteDocType<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pwszpublicid: windows_core::PCWSTR, pwszsystemid: windows_core::PCWSTR, pwszsubset: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteDocType(this, core::mem::transmute(&pwszname), core::mem::transmute(&pwszpublicid), core::mem::transmute(&pwszsystemid), core::mem::transmute(&pwszsubset)).into()
        }
        unsafe extern "system" fn WriteElementString<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprefix: windows_core::PCWSTR, pwszlocalname: windows_core::PCWSTR, pwsznamespaceuri: windows_core::PCWSTR, pwszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteElementString(this, core::mem::transmute(&pwszprefix), core::mem::transmute(&pwszlocalname), core::mem::transmute(&pwsznamespaceuri), core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteEndDocument<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteEndDocument(this).into()
        }
        unsafe extern "system" fn WriteEndElement<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteEndElement(this).into()
        }
        unsafe extern "system" fn WriteEntityRef<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteEntityRef(this, core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn WriteFullEndElement<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteFullEndElement(this).into()
        }
        unsafe extern "system" fn WriteName<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteName(this, core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn WriteNmToken<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsznmtoken: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteNmToken(this, core::mem::transmute(&pwsznmtoken)).into()
        }
        unsafe extern "system" fn WriteNode<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteNode(this, windows_core::from_raw_borrowed(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteNodeShallow<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteNodeShallow(this, windows_core::from_raw_borrowed(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteProcessingInstruction<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteProcessingInstruction(this, core::mem::transmute(&pwszname), core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteQualifiedName<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszlocalname: windows_core::PCWSTR, pwsznamespaceuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteQualifiedName(this, core::mem::transmute(&pwszlocalname), core::mem::transmute(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn WriteRaw<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdata: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteRaw(this, core::mem::transmute(&pwszdata)).into()
        }
        unsafe extern "system" fn WriteRawChars<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwch: windows_core::PCWSTR, cwch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteRawChars(this, core::mem::transmute(&pwch), core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteStartDocument<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, standalone: XmlStandalone) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteStartDocument(this, core::mem::transmute_copy(&standalone)).into()
        }
        unsafe extern "system" fn WriteStartElement<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprefix: windows_core::PCWSTR, pwszlocalname: windows_core::PCWSTR, pwsznamespaceuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteStartElement(this, core::mem::transmute(&pwszprefix), core::mem::transmute(&pwszlocalname), core::mem::transmute(&pwsznamespaceuri)).into()
        }
        unsafe extern "system" fn WriteString<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteString(this, core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wchlow: u16, wchhigh: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteSurrogateCharEntity(this, core::mem::transmute_copy(&wchlow), core::mem::transmute_copy(&wchhigh)).into()
        }
        unsafe extern "system" fn WriteWhitespace<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszwhitespace: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::WriteWhitespace(this, core::mem::transmute(&pwszwhitespace)).into()
        }
        unsafe extern "system" fn Flush<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriter_Impl::Flush(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOutput: SetOutput::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            WriteAttributes: WriteAttributes::<Identity, OFFSET>,
            WriteAttributeString: WriteAttributeString::<Identity, OFFSET>,
            WriteCData: WriteCData::<Identity, OFFSET>,
            WriteCharEntity: WriteCharEntity::<Identity, OFFSET>,
            WriteChars: WriteChars::<Identity, OFFSET>,
            WriteComment: WriteComment::<Identity, OFFSET>,
            WriteDocType: WriteDocType::<Identity, OFFSET>,
            WriteElementString: WriteElementString::<Identity, OFFSET>,
            WriteEndDocument: WriteEndDocument::<Identity, OFFSET>,
            WriteEndElement: WriteEndElement::<Identity, OFFSET>,
            WriteEntityRef: WriteEntityRef::<Identity, OFFSET>,
            WriteFullEndElement: WriteFullEndElement::<Identity, OFFSET>,
            WriteName: WriteName::<Identity, OFFSET>,
            WriteNmToken: WriteNmToken::<Identity, OFFSET>,
            WriteNode: WriteNode::<Identity, OFFSET>,
            WriteNodeShallow: WriteNodeShallow::<Identity, OFFSET>,
            WriteProcessingInstruction: WriteProcessingInstruction::<Identity, OFFSET>,
            WriteQualifiedName: WriteQualifiedName::<Identity, OFFSET>,
            WriteRaw: WriteRaw::<Identity, OFFSET>,
            WriteRawChars: WriteRawChars::<Identity, OFFSET>,
            WriteStartDocument: WriteStartDocument::<Identity, OFFSET>,
            WriteStartElement: WriteStartElement::<Identity, OFFSET>,
            WriteString: WriteString::<Identity, OFFSET>,
            WriteSurrogateCharEntity: WriteSurrogateCharEntity::<Identity, OFFSET>,
            WriteWhitespace: WriteWhitespace::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlWriter as windows_core::Interface>::IID
    }
}
pub trait IXmlWriterLite_Impl: Sized + windows_core::IUnknownImpl {
    fn SetOutput(&self, poutput: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetProperty(&self, nproperty: u32) -> windows_core::Result<isize>;
    fn SetProperty(&self, nproperty: u32, pvalue: isize) -> windows_core::Result<()>;
    fn WriteAttributes(&self, preader: Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn WriteAttributeString(&self, pwszqname: &windows_core::PCWSTR, cwszqname: u32, pwszvalue: &windows_core::PCWSTR, cwszvalue: u32) -> windows_core::Result<()>;
    fn WriteCData(&self, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteCharEntity(&self, wch: u16) -> windows_core::Result<()>;
    fn WriteChars(&self, pwch: &windows_core::PCWSTR, cwch: u32) -> windows_core::Result<()>;
    fn WriteComment(&self, pwszcomment: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteDocType(&self, pwszname: &windows_core::PCWSTR, pwszpublicid: &windows_core::PCWSTR, pwszsystemid: &windows_core::PCWSTR, pwszsubset: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteElementString(&self, pwszqname: &windows_core::PCWSTR, cwszqname: u32, pwszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteEndDocument(&self) -> windows_core::Result<()>;
    fn WriteEndElement(&self, pwszqname: &windows_core::PCWSTR, cwszqname: u32) -> windows_core::Result<()>;
    fn WriteEntityRef(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteFullEndElement(&self, pwszqname: &windows_core::PCWSTR, cwszqname: u32) -> windows_core::Result<()>;
    fn WriteName(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteNmToken(&self, pwsznmtoken: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteNode(&self, preader: Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn WriteNodeShallow(&self, preader: Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn WriteProcessingInstruction(&self, pwszname: &windows_core::PCWSTR, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteRaw(&self, pwszdata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteRawChars(&self, pwch: &windows_core::PCWSTR, cwch: u32) -> windows_core::Result<()>;
    fn WriteStartDocument(&self, standalone: XmlStandalone) -> windows_core::Result<()>;
    fn WriteStartElement(&self, pwszqname: &windows_core::PCWSTR, cwszqname: u32) -> windows_core::Result<()>;
    fn WriteString(&self, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> windows_core::Result<()>;
    fn WriteWhitespace(&self, pwszwhitespace: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXmlWriterLite {}
impl IXmlWriterLite_Vtbl {
    pub const fn new<Identity: IXmlWriterLite_Impl, const OFFSET: isize>() -> IXmlWriterLite_Vtbl {
        unsafe extern "system" fn SetOutput<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::SetOutput(this, windows_core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlWriterLite_Impl::GetProperty(this, core::mem::transmute_copy(&nproperty)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, pvalue: isize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::SetProperty(this, core::mem::transmute_copy(&nproperty), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn WriteAttributes<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteAttributes(this, windows_core::from_raw_borrowed(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteAttributeString<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszqname: windows_core::PCWSTR, cwszqname: u32, pwszvalue: windows_core::PCWSTR, cwszvalue: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteAttributeString(this, core::mem::transmute(&pwszqname), core::mem::transmute_copy(&cwszqname), core::mem::transmute(&pwszvalue), core::mem::transmute_copy(&cwszvalue)).into()
        }
        unsafe extern "system" fn WriteCData<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteCData(this, core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteCharEntity<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wch: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteCharEntity(this, core::mem::transmute_copy(&wch)).into()
        }
        unsafe extern "system" fn WriteChars<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwch: windows_core::PCWSTR, cwch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteChars(this, core::mem::transmute(&pwch), core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteComment<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcomment: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteComment(this, core::mem::transmute(&pwszcomment)).into()
        }
        unsafe extern "system" fn WriteDocType<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pwszpublicid: windows_core::PCWSTR, pwszsystemid: windows_core::PCWSTR, pwszsubset: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteDocType(this, core::mem::transmute(&pwszname), core::mem::transmute(&pwszpublicid), core::mem::transmute(&pwszsystemid), core::mem::transmute(&pwszsubset)).into()
        }
        unsafe extern "system" fn WriteElementString<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszqname: windows_core::PCWSTR, cwszqname: u32, pwszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteElementString(this, core::mem::transmute(&pwszqname), core::mem::transmute_copy(&cwszqname), core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn WriteEndDocument<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteEndDocument(this).into()
        }
        unsafe extern "system" fn WriteEndElement<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszqname: windows_core::PCWSTR, cwszqname: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteEndElement(this, core::mem::transmute(&pwszqname), core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteEntityRef<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteEntityRef(this, core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn WriteFullEndElement<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszqname: windows_core::PCWSTR, cwszqname: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteFullEndElement(this, core::mem::transmute(&pwszqname), core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteName<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteName(this, core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn WriteNmToken<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsznmtoken: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteNmToken(this, core::mem::transmute(&pwsznmtoken)).into()
        }
        unsafe extern "system" fn WriteNode<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteNode(this, windows_core::from_raw_borrowed(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteNodeShallow<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteNodeShallow(this, windows_core::from_raw_borrowed(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
        }
        unsafe extern "system" fn WriteProcessingInstruction<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteProcessingInstruction(this, core::mem::transmute(&pwszname), core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteRaw<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdata: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteRaw(this, core::mem::transmute(&pwszdata)).into()
        }
        unsafe extern "system" fn WriteRawChars<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwch: windows_core::PCWSTR, cwch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteRawChars(this, core::mem::transmute(&pwch), core::mem::transmute_copy(&cwch)).into()
        }
        unsafe extern "system" fn WriteStartDocument<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, standalone: XmlStandalone) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteStartDocument(this, core::mem::transmute_copy(&standalone)).into()
        }
        unsafe extern "system" fn WriteStartElement<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszqname: windows_core::PCWSTR, cwszqname: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteStartElement(this, core::mem::transmute(&pwszqname), core::mem::transmute_copy(&cwszqname)).into()
        }
        unsafe extern "system" fn WriteString<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteString(this, core::mem::transmute(&pwsztext)).into()
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wchlow: u16, wchhigh: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteSurrogateCharEntity(this, core::mem::transmute_copy(&wchlow), core::mem::transmute_copy(&wchhigh)).into()
        }
        unsafe extern "system" fn WriteWhitespace<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszwhitespace: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::WriteWhitespace(this, core::mem::transmute(&pwszwhitespace)).into()
        }
        unsafe extern "system" fn Flush<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlWriterLite_Impl::Flush(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOutput: SetOutput::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            WriteAttributes: WriteAttributes::<Identity, OFFSET>,
            WriteAttributeString: WriteAttributeString::<Identity, OFFSET>,
            WriteCData: WriteCData::<Identity, OFFSET>,
            WriteCharEntity: WriteCharEntity::<Identity, OFFSET>,
            WriteChars: WriteChars::<Identity, OFFSET>,
            WriteComment: WriteComment::<Identity, OFFSET>,
            WriteDocType: WriteDocType::<Identity, OFFSET>,
            WriteElementString: WriteElementString::<Identity, OFFSET>,
            WriteEndDocument: WriteEndDocument::<Identity, OFFSET>,
            WriteEndElement: WriteEndElement::<Identity, OFFSET>,
            WriteEntityRef: WriteEntityRef::<Identity, OFFSET>,
            WriteFullEndElement: WriteFullEndElement::<Identity, OFFSET>,
            WriteName: WriteName::<Identity, OFFSET>,
            WriteNmToken: WriteNmToken::<Identity, OFFSET>,
            WriteNode: WriteNode::<Identity, OFFSET>,
            WriteNodeShallow: WriteNodeShallow::<Identity, OFFSET>,
            WriteProcessingInstruction: WriteProcessingInstruction::<Identity, OFFSET>,
            WriteRaw: WriteRaw::<Identity, OFFSET>,
            WriteRawChars: WriteRawChars::<Identity, OFFSET>,
            WriteStartDocument: WriteStartDocument::<Identity, OFFSET>,
            WriteStartElement: WriteStartElement::<Identity, OFFSET>,
            WriteString: WriteString::<Identity, OFFSET>,
            WriteSurrogateCharEntity: WriteSurrogateCharEntity::<Identity, OFFSET>,
            WriteWhitespace: WriteWhitespace::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlWriterLite as windows_core::Interface>::IID
    }
}
