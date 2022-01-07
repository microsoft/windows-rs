#[cfg(feature = "implement_exclusive")]
pub trait IDtdEntityImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn PublicId(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NotationName(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDtdEntity {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IDtdEntity";
}
#[cfg(feature = "implement_exclusive")]
impl IDtdEntityVtbl {
    pub const fn new<Impl: IDtdEntityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtdEntityVtbl {
        unsafe extern "system" fn PublicId<Impl: IDtdEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PublicId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemId<Impl: IDtdEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotationName<Impl: IDtdEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotationName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtdEntity>, base.5, PublicId::<Impl, OFFSET>, SystemId::<Impl, OFFSET>, NotationName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDtdNotationImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn PublicId(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SystemId(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDtdNotation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IDtdNotation";
}
#[cfg(feature = "implement_exclusive")]
impl IDtdNotationVtbl {
    pub const fn new<Impl: IDtdNotationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtdNotationVtbl {
        unsafe extern "system" fn PublicId<Impl: IDtdNotationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PublicId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemId<Impl: IDtdNotationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtdNotation>, base.5, PublicId::<Impl, OFFSET>, SystemId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlAttributeImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Specified(&self) -> ::windows::core::Result<bool>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlAttribute {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlAttribute";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlAttributeVtbl {
    pub const fn new<Impl: IXmlAttributeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlAttributeVtbl {
        unsafe extern "system" fn Name<Impl: IXmlAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Specified<Impl: IXmlAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Specified() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IXmlAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IXmlAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlAttribute>, base.5, Name::<Impl, OFFSET>, Specified::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlCDataSectionImpl: Sized + IXmlCharacterDataImpl + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl + IXmlTextImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlCDataSection {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlCDataSection";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlCDataSectionVtbl {
    pub const fn new<Impl: IXmlCDataSectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlCDataSectionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlCDataSection>, base.5)
    }
}
pub trait IXmlCharacterDataImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn SubstringData(&self, offset: u32, count: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppendData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn InsertData(&self, offset: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeleteData(&self, offset: u32, count: u32) -> ::windows::core::Result<()>;
    fn ReplaceData(&self, offset: u32, count: u32, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXmlCharacterData {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlCharacterData";
}
impl IXmlCharacterDataVtbl {
    pub const fn new<Impl: IXmlCharacterDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlCharacterDataVtbl {
        unsafe extern "system" fn Data<Impl: IXmlCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IXmlCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Length<Impl: IXmlCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubstringData<Impl: IXmlCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u32, count: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubstringData(offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendData<Impl: IXmlCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AppendData(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertData<Impl: IXmlCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u32, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).InsertData(offset, &*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeleteData<Impl: IXmlCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DeleteData(offset, count).into()
        }
        unsafe extern "system" fn ReplaceData<Impl: IXmlCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u32, count: u32, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReplaceData(offset, count, &*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlCharacterData>, base.5, Data::<Impl, OFFSET>, SetData::<Impl, OFFSET>, Length::<Impl, OFFSET>, SubstringData::<Impl, OFFSET>, AppendData::<Impl, OFFSET>, InsertData::<Impl, OFFSET>, DeleteData::<Impl, OFFSET>, ReplaceData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlCommentImpl: Sized + IXmlCharacterDataImpl + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlComment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlComment";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlCommentVtbl {
    pub const fn new<Impl: IXmlCommentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlCommentVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlComment>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn Doctype(&self) -> ::windows::core::Result<XmlDocumentType>;
    fn Implementation(&self) -> ::windows::core::Result<XmlDomImplementation>;
    fn DocumentElement(&self) -> ::windows::core::Result<XmlElement>;
    fn CreateElement(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement>;
    fn CreateDocumentFragment(&self) -> ::windows::core::Result<XmlDocumentFragment>;
    fn CreateTextNode(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlText>;
    fn CreateComment(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlComment>;
    fn CreateProcessingInstruction(&self, target: &::windows::core::HSTRING, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlProcessingInstruction>;
    fn CreateAttribute(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>;
    fn CreateEntityReference(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XmlEntityReference>;
    fn GetElementsByTagName(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList>;
    fn CreateCDataSection(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<XmlCDataSection>;
    fn DocumentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateAttributeNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, qualifiedname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>;
    fn CreateElementNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, qualifiedname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement>;
    fn GetElementById(&self, elementid: &::windows::core::HSTRING) -> ::windows::core::Result<XmlElement>;
    fn ImportNode(&self, node: &::core::option::Option<IXmlNode>, deep: bool) -> ::windows::core::Result<IXmlNode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlDocument";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlDocumentVtbl {
    pub const fn new<Impl: IXmlDocumentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlDocumentVtbl {
        unsafe extern "system" fn Doctype<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Doctype() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Implementation<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Implementation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentElement<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DocumentElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateElement<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateElement(&*(&tagname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentFragment<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDocumentFragment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextNode<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTextNode(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComment<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateComment(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProcessingInstruction<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateProcessingInstruction(&*(&target as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttribute<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAttribute(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEntityReference<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEntityReference(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElementsByTagName<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetElementsByTagName(&*(&tagname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCDataSection<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCDataSection(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentUri<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DocumentUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttributeNS<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAttributeNS(&*(&namespaceuri as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&qualifiedname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateElementNS<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateElementNS(&*(&namespaceuri as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&qualifiedname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElementById<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elementid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetElementById(&*(&elementid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportNode<Impl: IXmlDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, deep: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImportNode(&*(&node as *const <IXmlNode as ::windows::core::Abi>::Abi as *const <IXmlNode as ::windows::core::DefaultType>::DefaultType), deep) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IXmlDocument>,
            base.5,
            Doctype::<Impl, OFFSET>,
            Implementation::<Impl, OFFSET>,
            DocumentElement::<Impl, OFFSET>,
            CreateElement::<Impl, OFFSET>,
            CreateDocumentFragment::<Impl, OFFSET>,
            CreateTextNode::<Impl, OFFSET>,
            CreateComment::<Impl, OFFSET>,
            CreateProcessingInstruction::<Impl, OFFSET>,
            CreateAttribute::<Impl, OFFSET>,
            CreateEntityReference::<Impl, OFFSET>,
            GetElementsByTagName::<Impl, OFFSET>,
            CreateCDataSection::<Impl, OFFSET>,
            DocumentUri::<Impl, OFFSET>,
            CreateAttributeNS::<Impl, OFFSET>,
            CreateElementNS::<Impl, OFFSET>,
            GetElementById::<Impl, OFFSET>,
            ImportNode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentFragmentImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlDocumentFragment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlDocumentFragment";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlDocumentFragmentVtbl {
    pub const fn new<Impl: IXmlDocumentFragmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlDocumentFragmentVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlDocumentFragment>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentIOImpl: Sized {
    fn LoadXml(&self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LoadXmlWithSettings(&self, xml: &::windows::core::HSTRING, loadsettings: &::core::option::Option<XmlLoadSettings>) -> ::windows::core::Result<()>;
    fn SaveToFileAsync(&self, file: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlDocumentIO {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlDocumentIO";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlDocumentIOVtbl {
    pub const fn new<Impl: IXmlDocumentIOImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlDocumentIOVtbl {
        unsafe extern "system" fn LoadXml<Impl: IXmlDocumentIOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LoadXml(&*(&xml as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoadXmlWithSettings<Impl: IXmlDocumentIOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loadsettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LoadXmlWithSettings(&*(&xml as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&loadsettings as *const <XmlLoadSettings as ::windows::core::Abi>::Abi as *const <XmlLoadSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SaveToFileAsync<Impl: IXmlDocumentIOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveToFileAsync(&*(&file as *const <super::super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlDocumentIO>, base.5, LoadXml::<Impl, OFFSET>, LoadXmlWithSettings::<Impl, OFFSET>, SaveToFileAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentIO2Impl: Sized {
    fn LoadXmlFromBuffer(&self, buffer: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn LoadXmlFromBufferWithSettings(&self, buffer: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, loadsettings: &::core::option::Option<XmlLoadSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlDocumentIO2 {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlDocumentIO2";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlDocumentIO2Vtbl {
    pub const fn new<Impl: IXmlDocumentIO2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlDocumentIO2Vtbl {
        unsafe extern "system" fn LoadXmlFromBuffer<Impl: IXmlDocumentIO2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LoadXmlFromBuffer(&*(&buffer as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoadXmlFromBufferWithSettings<Impl: IXmlDocumentIO2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, loadsettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LoadXmlFromBufferWithSettings(&*(&buffer as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&loadsettings as *const <XmlLoadSettings as ::windows::core::Abi>::Abi as *const <XmlLoadSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlDocumentIO2>, base.5, LoadXmlFromBuffer::<Impl, OFFSET>, LoadXmlFromBufferWithSettings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentStaticsImpl: Sized {
    fn LoadFromUriAsync(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>;
    fn LoadFromUriWithSettingsAsync(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, loadsettings: &::core::option::Option<XmlLoadSettings>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>;
    fn LoadFromFileAsync(&self, file: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>;
    fn LoadFromFileWithSettingsAsync(&self, file: &::core::option::Option<super::super::super::Storage::IStorageFile>, loadsettings: &::core::option::Option<XmlLoadSettings>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlDocumentStatics {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlDocumentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlDocumentStaticsVtbl {
    pub const fn new<Impl: IXmlDocumentStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlDocumentStaticsVtbl {
        unsafe extern "system" fn LoadFromUriAsync<Impl: IXmlDocumentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadFromUriAsync(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromUriWithSettingsAsync<Impl: IXmlDocumentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, loadsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadFromUriWithSettingsAsync(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&loadsettings as *const <XmlLoadSettings as ::windows::core::Abi>::Abi as *const <XmlLoadSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromFileAsync<Impl: IXmlDocumentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadFromFileAsync(&*(&file as *const <super::super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromFileWithSettingsAsync<Impl: IXmlDocumentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, loadsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadFromFileWithSettingsAsync(&*(&file as *const <super::super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&loadsettings as *const <XmlLoadSettings as ::windows::core::Abi>::Abi as *const <XmlLoadSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlDocumentStatics>, base.5, LoadFromUriAsync::<Impl, OFFSET>, LoadFromUriWithSettingsAsync::<Impl, OFFSET>, LoadFromFileAsync::<Impl, OFFSET>, LoadFromFileWithSettingsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDocumentTypeImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Entities(&self) -> ::windows::core::Result<XmlNamedNodeMap>;
    fn Notations(&self) -> ::windows::core::Result<XmlNamedNodeMap>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlDocumentType {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlDocumentType";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlDocumentTypeVtbl {
    pub const fn new<Impl: IXmlDocumentTypeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlDocumentTypeVtbl {
        unsafe extern "system" fn Name<Impl: IXmlDocumentTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Entities<Impl: IXmlDocumentTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Entities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notations<Impl: IXmlDocumentTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Notations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlDocumentType>, base.5, Name::<Impl, OFFSET>, Entities::<Impl, OFFSET>, Notations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlDomImplementationImpl: Sized {
    fn HasFeature(&self, feature: &::windows::core::HSTRING, version: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlDomImplementation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlDomImplementation";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlDomImplementationVtbl {
    pub const fn new<Impl: IXmlDomImplementationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlDomImplementationVtbl {
        unsafe extern "system" fn HasFeature<Impl: IXmlDomImplementationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, version: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasFeature(&*(&feature as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&version as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlDomImplementation>, base.5, HasFeature::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlElementImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn TagName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAttribute(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAttribute(&self, attributename: &::windows::core::HSTRING, attributevalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveAttribute(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetAttributeNode(&self, attributename: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>;
    fn SetAttributeNode(&self, newattribute: &::core::option::Option<XmlAttribute>) -> ::windows::core::Result<XmlAttribute>;
    fn RemoveAttributeNode(&self, attributenode: &::core::option::Option<XmlAttribute>) -> ::windows::core::Result<XmlAttribute>;
    fn GetElementsByTagName(&self, tagname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList>;
    fn SetAttributeNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, qualifiedname: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetAttributeNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, localname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoveAttributeNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, localname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetAttributeNodeNS(&self, newattribute: &::core::option::Option<XmlAttribute>) -> ::windows::core::Result<XmlAttribute>;
    fn GetAttributeNodeNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, localname: &::windows::core::HSTRING) -> ::windows::core::Result<XmlAttribute>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlElement {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlElement";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlElementVtbl {
    pub const fn new<Impl: IXmlElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlElementVtbl {
        unsafe extern "system" fn TagName<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TagName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttribute(&*(&attributename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttribute<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, attributevalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAttribute(&*(&attributename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&attributevalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAttribute<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAttribute(&*(&attributename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAttributeNode<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeNode(&*(&attributename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeNode<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newattribute: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAttributeNode(&*(&newattribute as *const <XmlAttribute as ::windows::core::Abi>::Abi as *const <XmlAttribute as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAttributeNode<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributenode: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAttributeNode(&*(&attributenode as *const <XmlAttribute as ::windows::core::Abi>::Abi as *const <XmlAttribute as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElementsByTagName<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetElementsByTagName(&*(&tagname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeNS<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .SetAttributeNS(
                    &*(&namespaceuri as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&qualifiedname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn GetAttributeNS<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeNS(&*(&namespaceuri as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&localname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAttributeNS<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAttributeNS(&*(&namespaceuri as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&localname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetAttributeNodeNS<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newattribute: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAttributeNodeNS(&*(&newattribute as *const <XmlAttribute as ::windows::core::Abi>::Abi as *const <XmlAttribute as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeNodeNS<Impl: IXmlElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeNodeNS(&*(&namespaceuri as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&localname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IXmlElement>,
            base.5,
            TagName::<Impl, OFFSET>,
            GetAttribute::<Impl, OFFSET>,
            SetAttribute::<Impl, OFFSET>,
            RemoveAttribute::<Impl, OFFSET>,
            GetAttributeNode::<Impl, OFFSET>,
            SetAttributeNode::<Impl, OFFSET>,
            RemoveAttributeNode::<Impl, OFFSET>,
            GetElementsByTagName::<Impl, OFFSET>,
            SetAttributeNS::<Impl, OFFSET>,
            GetAttributeNS::<Impl, OFFSET>,
            RemoveAttributeNS::<Impl, OFFSET>,
            SetAttributeNodeNS::<Impl, OFFSET>,
            GetAttributeNodeNS::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlEntityReferenceImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlEntityReference {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlEntityReference";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlEntityReferenceVtbl {
    pub const fn new<Impl: IXmlEntityReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlEntityReferenceVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlEntityReference>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlLoadSettingsImpl: Sized {
    fn MaxElementDepth(&self) -> ::windows::core::Result<u32>;
    fn SetMaxElementDepth(&self, value: u32) -> ::windows::core::Result<()>;
    fn ProhibitDtd(&self) -> ::windows::core::Result<bool>;
    fn SetProhibitDtd(&self, value: bool) -> ::windows::core::Result<()>;
    fn ResolveExternals(&self) -> ::windows::core::Result<bool>;
    fn SetResolveExternals(&self, value: bool) -> ::windows::core::Result<()>;
    fn ValidateOnParse(&self) -> ::windows::core::Result<bool>;
    fn SetValidateOnParse(&self, value: bool) -> ::windows::core::Result<()>;
    fn ElementContentWhiteSpace(&self) -> ::windows::core::Result<bool>;
    fn SetElementContentWhiteSpace(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlLoadSettings {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlLoadSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlLoadSettingsVtbl {
    pub const fn new<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlLoadSettingsVtbl {
        unsafe extern "system" fn MaxElementDepth<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxElementDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxElementDepth<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxElementDepth(value).into()
        }
        unsafe extern "system" fn ProhibitDtd<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProhibitDtd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProhibitDtd<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProhibitDtd(value).into()
        }
        unsafe extern "system" fn ResolveExternals<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResolveExternals() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResolveExternals<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetResolveExternals(value).into()
        }
        unsafe extern "system" fn ValidateOnParse<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidateOnParse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValidateOnParse<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValidateOnParse(value).into()
        }
        unsafe extern "system" fn ElementContentWhiteSpace<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ElementContentWhiteSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElementContentWhiteSpace<Impl: IXmlLoadSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetElementContentWhiteSpace(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlLoadSettings>, base.5, MaxElementDepth::<Impl, OFFSET>, SetMaxElementDepth::<Impl, OFFSET>, ProhibitDtd::<Impl, OFFSET>, SetProhibitDtd::<Impl, OFFSET>, ResolveExternals::<Impl, OFFSET>, SetResolveExternals::<Impl, OFFSET>, ValidateOnParse::<Impl, OFFSET>, SetValidateOnParse::<Impl, OFFSET>, ElementContentWhiteSpace::<Impl, OFFSET>, SetElementContentWhiteSpace::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IXmlNamedNodeMapImpl: Sized + IIterableImpl<IXmlNode> + IVectorViewImpl<IXmlNode> {
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode>;
    fn GetNamedItem(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn SetNamedItem(&self, node: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
    fn RemoveNamedItem(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn GetNamedItemNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn RemoveNamedItemNS(&self, namespaceuri: &::core::option::Option<::windows::core::IInspectable>, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn SetNamedItemNS(&self, node: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXmlNamedNodeMap {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlNamedNodeMap";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IXmlNamedNodeMapVtbl {
    pub const fn new<Impl: IXmlNamedNodeMapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlNamedNodeMapVtbl {
        unsafe extern "system" fn Length<Impl: IXmlNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IXmlNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedItem<Impl: IXmlNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNamedItem(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamedItem<Impl: IXmlNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNamedItem(&*(&node as *const <IXmlNode as ::windows::core::Abi>::Abi as *const <IXmlNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNamedItem<Impl: IXmlNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveNamedItem(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedItemNS<Impl: IXmlNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNamedItemNS(&*(&namespaceuri as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNamedItemNS<Impl: IXmlNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveNamedItemNS(&*(&namespaceuri as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamedItemNS<Impl: IXmlNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNamedItemNS(&*(&node as *const <IXmlNode as ::windows::core::Abi>::Abi as *const <IXmlNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlNamedNodeMap>, base.5, Length::<Impl, OFFSET>, Item::<Impl, OFFSET>, GetNamedItem::<Impl, OFFSET>, SetNamedItem::<Impl, OFFSET>, RemoveNamedItem::<Impl, OFFSET>, GetNamedItemNS::<Impl, OFFSET>, RemoveNamedItemNS::<Impl, OFFSET>, SetNamedItemNS::<Impl, OFFSET>)
    }
}
pub trait IXmlNodeImpl: Sized + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn NodeValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetNodeValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn NodeType(&self) -> ::windows::core::Result<NodeType>;
    fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentNode(&self) -> ::windows::core::Result<IXmlNode>;
    fn ChildNodes(&self) -> ::windows::core::Result<XmlNodeList>;
    fn FirstChild(&self) -> ::windows::core::Result<IXmlNode>;
    fn LastChild(&self) -> ::windows::core::Result<IXmlNode>;
    fn PreviousSibling(&self) -> ::windows::core::Result<IXmlNode>;
    fn NextSibling(&self) -> ::windows::core::Result<IXmlNode>;
    fn Attributes(&self) -> ::windows::core::Result<XmlNamedNodeMap>;
    fn HasChildNodes(&self) -> ::windows::core::Result<bool>;
    fn OwnerDocument(&self) -> ::windows::core::Result<XmlDocument>;
    fn InsertBefore(&self, newchild: &::core::option::Option<IXmlNode>, referencechild: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
    fn ReplaceChild(&self, newchild: &::core::option::Option<IXmlNode>, referencechild: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
    fn RemoveChild(&self, childnode: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
    fn AppendChild(&self, newchild: &::core::option::Option<IXmlNode>) -> ::windows::core::Result<IXmlNode>;
    fn CloneNode(&self, deep: bool) -> ::windows::core::Result<IXmlNode>;
    fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn LocalName(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Prefix(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Normalize(&self) -> ::windows::core::Result<()>;
    fn SetPrefix(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXmlNode {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlNode";
}
impl IXmlNodeVtbl {
    pub const fn new<Impl: IXmlNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlNodeVtbl {
        unsafe extern "system" fn NodeValue<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NodeValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNodeValue<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNodeValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NodeType<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NodeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NodeName<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NodeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentNode<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParentNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildNodes<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChildNodes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstChild<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FirstChild() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastChild<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastChild() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviousSibling<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviousSibling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextSibling<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextSibling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildNodes<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasChildNodes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerDocument<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OwnerDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertBefore<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, referencechild: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertBefore(&*(&newchild as *const <IXmlNode as ::windows::core::Abi>::Abi as *const <IXmlNode as ::windows::core::DefaultType>::DefaultType), &*(&referencechild as *const <IXmlNode as ::windows::core::Abi>::Abi as *const <IXmlNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceChild<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, referencechild: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReplaceChild(&*(&newchild as *const <IXmlNode as ::windows::core::Abi>::Abi as *const <IXmlNode as ::windows::core::DefaultType>::DefaultType), &*(&referencechild as *const <IXmlNode as ::windows::core::Abi>::Abi as *const <IXmlNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChild<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childnode: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveChild(&*(&childnode as *const <IXmlNode as ::windows::core::Abi>::Abi as *const <IXmlNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendChild<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppendChild(&*(&newchild as *const <IXmlNode as ::windows::core::Abi>::Abi as *const <IXmlNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloneNode<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deep: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloneNode(deep) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceUri<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NamespaceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalName<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Prefix<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Prefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Normalize<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Normalize().into()
        }
        unsafe extern "system" fn SetPrefix<Impl: IXmlNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPrefix(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IXmlNode>,
            base.5,
            NodeValue::<Impl, OFFSET>,
            SetNodeValue::<Impl, OFFSET>,
            NodeType::<Impl, OFFSET>,
            NodeName::<Impl, OFFSET>,
            ParentNode::<Impl, OFFSET>,
            ChildNodes::<Impl, OFFSET>,
            FirstChild::<Impl, OFFSET>,
            LastChild::<Impl, OFFSET>,
            PreviousSibling::<Impl, OFFSET>,
            NextSibling::<Impl, OFFSET>,
            Attributes::<Impl, OFFSET>,
            HasChildNodes::<Impl, OFFSET>,
            OwnerDocument::<Impl, OFFSET>,
            InsertBefore::<Impl, OFFSET>,
            ReplaceChild::<Impl, OFFSET>,
            RemoveChild::<Impl, OFFSET>,
            AppendChild::<Impl, OFFSET>,
            CloneNode::<Impl, OFFSET>,
            NamespaceUri::<Impl, OFFSET>,
            LocalName::<Impl, OFFSET>,
            Prefix::<Impl, OFFSET>,
            Normalize::<Impl, OFFSET>,
            SetPrefix::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IXmlNodeListImpl: Sized + IIterableImpl<IXmlNode> + IVectorViewImpl<IXmlNode> {
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IXmlNode>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXmlNodeList {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlNodeList";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IXmlNodeListVtbl {
    pub const fn new<Impl: IXmlNodeListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlNodeListVtbl {
        unsafe extern "system" fn Length<Impl: IXmlNodeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IXmlNodeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlNodeList>, base.5, Length::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
pub trait IXmlNodeSelectorImpl: Sized {
    fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList>;
    fn SelectSingleNodeNS(&self, xpath: &::windows::core::HSTRING, namespaces: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IXmlNode>;
    fn SelectNodesNS(&self, xpath: &::windows::core::HSTRING, namespaces: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XmlNodeList>;
}
impl ::windows::core::RuntimeName for IXmlNodeSelector {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlNodeSelector";
}
impl IXmlNodeSelectorVtbl {
    pub const fn new<Impl: IXmlNodeSelectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlNodeSelectorVtbl {
        unsafe extern "system" fn SelectSingleNode<Impl: IXmlNodeSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectSingleNode(&*(&xpath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectNodes<Impl: IXmlNodeSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectNodes(&*(&xpath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectSingleNodeNS<Impl: IXmlNodeSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectSingleNodeNS(&*(&xpath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&namespaces as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectNodesNS<Impl: IXmlNodeSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectNodesNS(&*(&xpath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&namespaces as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlNodeSelector>, base.5, SelectSingleNode::<Impl, OFFSET>, SelectNodes::<Impl, OFFSET>, SelectSingleNodeNS::<Impl, OFFSET>, SelectNodesNS::<Impl, OFFSET>)
    }
}
pub trait IXmlNodeSerializerImpl: Sized {
    fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXmlNodeSerializer {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlNodeSerializer";
}
impl IXmlNodeSerializerVtbl {
    pub const fn new<Impl: IXmlNodeSerializerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlNodeSerializerVtbl {
        unsafe extern "system" fn GetXml<Impl: IXmlNodeSerializerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetXml() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InnerText<Impl: IXmlNodeSerializerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InnerText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInnerText<Impl: IXmlNodeSerializerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInnerText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlNodeSerializer>, base.5, GetXml::<Impl, OFFSET>, InnerText::<Impl, OFFSET>, SetInnerText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXmlProcessingInstructionImpl: Sized + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn Target(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXmlProcessingInstruction {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlProcessingInstruction";
}
#[cfg(feature = "implement_exclusive")]
impl IXmlProcessingInstructionVtbl {
    pub const fn new<Impl: IXmlProcessingInstructionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlProcessingInstructionVtbl {
        unsafe extern "system" fn Target<Impl: IXmlProcessingInstructionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IXmlProcessingInstructionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IXmlProcessingInstructionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlProcessingInstruction>, base.5, Target::<Impl, OFFSET>, Data::<Impl, OFFSET>, SetData::<Impl, OFFSET>)
    }
}
pub trait IXmlTextImpl: Sized + IXmlCharacterDataImpl + IXmlNodeImpl + IXmlNodeSelectorImpl + IXmlNodeSerializerImpl {
    fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText>;
}
impl ::windows::core::RuntimeName for IXmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlText";
}
impl IXmlTextVtbl {
    pub const fn new<Impl: IXmlTextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXmlTextVtbl {
        unsafe extern "system" fn SplitText<Impl: IXmlTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SplitText(offset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXmlText>, base.5, SplitText::<Impl, OFFSET>)
    }
}
