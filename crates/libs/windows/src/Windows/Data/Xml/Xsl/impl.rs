#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IXsltProcessorImpl: Sized {
    fn TransformToString(&self, inputnode: &::core::option::Option<super::Dom::IXmlNode>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXsltProcessor {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.IXsltProcessor";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IXsltProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXsltProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXsltProcessorVtbl {
        unsafe extern "system" fn TransformToString<Impl: IXsltProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputnode: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformToString(&*(&inputnode as *const <super::Dom::IXmlNode as ::windows::core::Abi>::Abi as *const <super::Dom::IXmlNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXsltProcessor, BASE_OFFSET>(),
            TransformToString: TransformToString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXsltProcessor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IXsltProcessor2Impl: Sized {
    fn TransformToDocument(&self, inputnode: &::core::option::Option<super::Dom::IXmlNode>) -> ::windows::core::Result<super::Dom::XmlDocument>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXsltProcessor2 {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.IXsltProcessor2";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IXsltProcessor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXsltProcessor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXsltProcessor2Vtbl {
        unsafe extern "system" fn TransformToDocument<Impl: IXsltProcessor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputnode: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformToDocument(&*(&inputnode as *const <super::Dom::IXmlNode as ::windows::core::Abi>::Abi as *const <super::Dom::IXmlNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXsltProcessor2, BASE_OFFSET>(),
            TransformToDocument: TransformToDocument::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXsltProcessor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IXsltProcessorFactoryImpl: Sized {
    fn CreateInstance(&self, document: &::core::option::Option<super::Dom::XmlDocument>) -> ::windows::core::Result<XsltProcessor>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXsltProcessorFactory {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.IXsltProcessorFactory";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IXsltProcessorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXsltProcessorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXsltProcessorFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXsltProcessorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&document as *const <super::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXsltProcessorFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXsltProcessorFactory as ::windows::core::Interface>::IID
    }
}
