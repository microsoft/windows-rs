pub trait IXmlCharacterData_Impl: Sized + IXmlNode_Impl + IXmlNodeSelector_Impl + IXmlNodeSerializer_Impl {
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
impl IXmlCharacterData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlCharacterData_Impl, const OFFSET: isize>() -> IXmlCharacterData_Vtbl {
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl, Impl: IXmlCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: IXmlCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl, Impl: IXmlCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubstringData<Identity: ::windows::core::IUnknownImpl, Impl: IXmlCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: u32, count: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubstringData(offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendData<Identity: ::windows::core::IUnknownImpl, Impl: IXmlCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AppendData(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn InsertData<Identity: ::windows::core::IUnknownImpl, Impl: IXmlCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: u32, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertData(offset, ::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn DeleteData<Identity: ::windows::core::IUnknownImpl, Impl: IXmlCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteData(offset, count).into()
        }
        unsafe extern "system" fn ReplaceData<Identity: ::windows::core::IUnknownImpl, Impl: IXmlCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: u32, count: u32, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReplaceData(offset, count, ::core::mem::transmute(&data)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXmlCharacterData, OFFSET>(),
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            SubstringData: SubstringData::<Identity, Impl, OFFSET>,
            AppendData: AppendData::<Identity, Impl, OFFSET>,
            InsertData: InsertData::<Identity, Impl, OFFSET>,
            DeleteData: DeleteData::<Identity, Impl, OFFSET>,
            ReplaceData: ReplaceData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlCharacterData as ::windows::core::Interface>::IID
    }
}
pub trait IXmlNode_Impl: Sized + IXmlNodeSelector_Impl + IXmlNodeSerializer_Impl {
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
impl IXmlNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>() -> IXmlNode_Vtbl {
        unsafe extern "system" fn NodeValue<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NodeValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNodeValue<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNodeValue(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn NodeType<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NodeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NodeName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NodeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentNode<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ParentNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildNodes<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ChildNodes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstChild<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirstChild() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastChild<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastChild() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviousSibling<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PreviousSibling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextSibling<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NextSibling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildNodes<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasChildNodes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OwnerDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertBefore<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, referencechild: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InsertBefore(::core::mem::transmute(&newchild), ::core::mem::transmute(&referencechild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceChild<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, referencechild: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReplaceChild(::core::mem::transmute(&newchild), ::core::mem::transmute(&referencechild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChild<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childnode: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoveChild(::core::mem::transmute(&childnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendChild<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppendChild(::core::mem::transmute(&newchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloneNode<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deep: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CloneNode(deep) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceUri<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NamespaceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalName<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Prefix<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Prefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Normalize<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Normalize().into()
        }
        unsafe extern "system" fn SetPrefix<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrefix(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXmlNode, OFFSET>(),
            NodeValue: NodeValue::<Identity, Impl, OFFSET>,
            SetNodeValue: SetNodeValue::<Identity, Impl, OFFSET>,
            NodeType: NodeType::<Identity, Impl, OFFSET>,
            NodeName: NodeName::<Identity, Impl, OFFSET>,
            ParentNode: ParentNode::<Identity, Impl, OFFSET>,
            ChildNodes: ChildNodes::<Identity, Impl, OFFSET>,
            FirstChild: FirstChild::<Identity, Impl, OFFSET>,
            LastChild: LastChild::<Identity, Impl, OFFSET>,
            PreviousSibling: PreviousSibling::<Identity, Impl, OFFSET>,
            NextSibling: NextSibling::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            HasChildNodes: HasChildNodes::<Identity, Impl, OFFSET>,
            OwnerDocument: OwnerDocument::<Identity, Impl, OFFSET>,
            InsertBefore: InsertBefore::<Identity, Impl, OFFSET>,
            ReplaceChild: ReplaceChild::<Identity, Impl, OFFSET>,
            RemoveChild: RemoveChild::<Identity, Impl, OFFSET>,
            AppendChild: AppendChild::<Identity, Impl, OFFSET>,
            CloneNode: CloneNode::<Identity, Impl, OFFSET>,
            NamespaceUri: NamespaceUri::<Identity, Impl, OFFSET>,
            LocalName: LocalName::<Identity, Impl, OFFSET>,
            Prefix: Prefix::<Identity, Impl, OFFSET>,
            Normalize: Normalize::<Identity, Impl, OFFSET>,
            SetPrefix: SetPrefix::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlNode as ::windows::core::Interface>::IID
    }
}
pub trait IXmlNodeSelector_Impl: Sized {
    fn SelectSingleNode(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<IXmlNode>;
    fn SelectNodes(&self, xpath: &::windows::core::HSTRING) -> ::windows::core::Result<XmlNodeList>;
    fn SelectSingleNodeNS(&self, xpath: &::windows::core::HSTRING, namespaces: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IXmlNode>;
    fn SelectNodesNS(&self, xpath: &::windows::core::HSTRING, namespaces: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XmlNodeList>;
}
impl ::windows::core::RuntimeName for IXmlNodeSelector {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlNodeSelector";
}
impl IXmlNodeSelector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNodeSelector_Impl, const OFFSET: isize>() -> IXmlNodeSelector_Vtbl {
        unsafe extern "system" fn SelectSingleNode<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNodeSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectSingleNode(::core::mem::transmute(&xpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectNodes<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNodeSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectNodes(::core::mem::transmute(&xpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectSingleNodeNS<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNodeSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectSingleNodeNS(::core::mem::transmute(&xpath), ::core::mem::transmute(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectNodesNS<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNodeSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectNodesNS(::core::mem::transmute(&xpath), ::core::mem::transmute(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXmlNodeSelector, OFFSET>(),
            SelectSingleNode: SelectSingleNode::<Identity, Impl, OFFSET>,
            SelectNodes: SelectNodes::<Identity, Impl, OFFSET>,
            SelectSingleNodeNS: SelectSingleNodeNS::<Identity, Impl, OFFSET>,
            SelectNodesNS: SelectNodesNS::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlNodeSelector as ::windows::core::Interface>::IID
    }
}
pub trait IXmlNodeSerializer_Impl: Sized {
    fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InnerText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetInnerText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXmlNodeSerializer {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlNodeSerializer";
}
impl IXmlNodeSerializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNodeSerializer_Impl, const OFFSET: isize>() -> IXmlNodeSerializer_Vtbl {
        unsafe extern "system" fn GetXml<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNodeSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetXml() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InnerText<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNodeSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InnerText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInnerText<Identity: ::windows::core::IUnknownImpl, Impl: IXmlNodeSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInnerText(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXmlNodeSerializer, OFFSET>(),
            GetXml: GetXml::<Identity, Impl, OFFSET>,
            InnerText: InnerText::<Identity, Impl, OFFSET>,
            SetInnerText: SetInnerText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlNodeSerializer as ::windows::core::Interface>::IID
    }
}
pub trait IXmlText_Impl: Sized + IXmlCharacterData_Impl + IXmlNode_Impl + IXmlNodeSelector_Impl + IXmlNodeSerializer_Impl {
    fn SplitText(&self, offset: u32) -> ::windows::core::Result<IXmlText>;
}
impl ::windows::core::RuntimeName for IXmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlText";
}
impl IXmlText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXmlText_Impl, const OFFSET: isize>() -> IXmlText_Vtbl {
        unsafe extern "system" fn SplitText<Identity: ::windows::core::IUnknownImpl, Impl: IXmlText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SplitText(offset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXmlText, OFFSET>(), SplitText: SplitText::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXmlText as ::windows::core::Interface>::IID
    }
}
