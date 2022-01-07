#[cfg(feature = "implement_exclusive")]
pub trait IXsltProcessorImpl: Sized {
    fn TransformToString(&self, inputnode: &::core::option::Option<super::Dom::IXmlNode>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXsltProcessor {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.IXsltProcessor";
}
#[cfg(feature = "implement_exclusive")]
impl IXsltProcessorVtbl {
    pub const fn new<Impl: IXsltProcessorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXsltProcessorVtbl {
        unsafe extern "system" fn TransformToString<Impl: IXsltProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputnode: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformToString(&*(&inputnode as *const <super::Dom::IXmlNode as ::windows::core::Abi>::Abi as *const <super::Dom::IXmlNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXsltProcessor>, base.5, TransformToString::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXsltProcessor2Impl: Sized {
    fn TransformToDocument(&self, inputnode: &::core::option::Option<super::Dom::IXmlNode>) -> ::windows::core::Result<super::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXsltProcessor2 {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.IXsltProcessor2";
}
#[cfg(feature = "implement_exclusive")]
impl IXsltProcessor2Vtbl {
    pub const fn new<Impl: IXsltProcessor2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXsltProcessor2Vtbl {
        unsafe extern "system" fn TransformToDocument<Impl: IXsltProcessor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputnode: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformToDocument(&*(&inputnode as *const <super::Dom::IXmlNode as ::windows::core::Abi>::Abi as *const <super::Dom::IXmlNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXsltProcessor2>, base.5, TransformToDocument::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXsltProcessorFactoryImpl: Sized {
    fn CreateInstance(&self, document: &::core::option::Option<super::Dom::XmlDocument>) -> ::windows::core::Result<XsltProcessor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXsltProcessorFactory {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.IXsltProcessorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXsltProcessorFactoryVtbl {
    pub const fn new<Impl: IXsltProcessorFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXsltProcessorFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXsltProcessorFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&document as *const <super::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXsltProcessorFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
