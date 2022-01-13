#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IPrintTicketCapabilitiesImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&mut self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DocumentBindingFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentCollateFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentDuplexFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentHolePunchFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentInputBinFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentNUpFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentStapleFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn JobPasscodeFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageBorderlessFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaSizeFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaTypeFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOrientationFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputColorFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputQualityFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageResolutionFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn GetFeature(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketFeature>;
    fn GetParameterDefinition(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterDefinition>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTicketCapabilities {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.IPrintTicketCapabilities";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IPrintTicketCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTicketCapabilitiesVtbl {
        unsafe extern "system" fn Name<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNamespace<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNode<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentBindingFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentBindingFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentCollateFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentCollateFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentDuplexFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentDuplexFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentHolePunchFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentHolePunchFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentInputBinFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentInputBinFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentNUpFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentNUpFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentStapleFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentStapleFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobPasscodeFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobPasscodeFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageBorderlessFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageBorderlessFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageMediaSizeFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageMediaSizeFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageMediaTypeFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageMediaTypeFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageOrientationFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageOrientationFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageOutputColorFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageOutputColorFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageOutputQualityFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageOutputQualityFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageResolutionFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageResolutionFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeature(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameterDefinition<Impl: IPrintTicketCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameterDefinition(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTicketCapabilities, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            XmlNamespace: XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode: XmlNode::<Impl, IMPL_OFFSET>,
            DocumentBindingFeature: DocumentBindingFeature::<Impl, IMPL_OFFSET>,
            DocumentCollateFeature: DocumentCollateFeature::<Impl, IMPL_OFFSET>,
            DocumentDuplexFeature: DocumentDuplexFeature::<Impl, IMPL_OFFSET>,
            DocumentHolePunchFeature: DocumentHolePunchFeature::<Impl, IMPL_OFFSET>,
            DocumentInputBinFeature: DocumentInputBinFeature::<Impl, IMPL_OFFSET>,
            DocumentNUpFeature: DocumentNUpFeature::<Impl, IMPL_OFFSET>,
            DocumentStapleFeature: DocumentStapleFeature::<Impl, IMPL_OFFSET>,
            JobPasscodeFeature: JobPasscodeFeature::<Impl, IMPL_OFFSET>,
            PageBorderlessFeature: PageBorderlessFeature::<Impl, IMPL_OFFSET>,
            PageMediaSizeFeature: PageMediaSizeFeature::<Impl, IMPL_OFFSET>,
            PageMediaTypeFeature: PageMediaTypeFeature::<Impl, IMPL_OFFSET>,
            PageOrientationFeature: PageOrientationFeature::<Impl, IMPL_OFFSET>,
            PageOutputColorFeature: PageOutputColorFeature::<Impl, IMPL_OFFSET>,
            PageOutputQualityFeature: PageOutputQualityFeature::<Impl, IMPL_OFFSET>,
            PageResolutionFeature: PageResolutionFeature::<Impl, IMPL_OFFSET>,
            GetFeature: GetFeature::<Impl, IMPL_OFFSET>,
            GetParameterDefinition: GetParameterDefinition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrintTicketFeatureImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&mut self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetOption(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketOption>;
    fn Options(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PrintTicketOption>>;
    fn GetSelectedOption(&mut self) -> ::windows::core::Result<PrintTicketOption>;
    fn SetSelectedOption(&mut self, value: &::core::option::Option<PrintTicketOption>) -> ::windows::core::Result<()>;
    fn SelectionType(&mut self) -> ::windows::core::Result<PrintTicketFeatureSelectionType>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTicketFeature {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.IPrintTicketFeature";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrintTicketFeatureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketFeatureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTicketFeatureVtbl {
        unsafe extern "system" fn Name<Impl: IPrintTicketFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNamespace<Impl: IPrintTicketFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNode<Impl: IPrintTicketFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IPrintTicketFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOption<Impl: IPrintTicketFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOption(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: IPrintTicketFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedOption<Impl: IPrintTicketFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectedOption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedOption<Impl: IPrintTicketFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedOption(&*(&value as *const <PrintTicketOption as ::windows::core::Abi>::Abi as *const <PrintTicketOption as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionType<Impl: IPrintTicketFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintTicketFeatureSelectionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTicketFeature, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            XmlNamespace: XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode: XmlNode::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            GetOption: GetOption::<Impl, IMPL_OFFSET>,
            Options: Options::<Impl, IMPL_OFFSET>,
            GetSelectedOption: GetSelectedOption::<Impl, IMPL_OFFSET>,
            SetSelectedOption: SetSelectedOption::<Impl, IMPL_OFFSET>,
            SelectionType: SelectionType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketFeature as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IPrintTicketOptionImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&mut self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetPropertyNode(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn GetScoredPropertyNode(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn GetPropertyValue(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketValue>;
    fn GetScoredPropertyValue(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketValue>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTicketOption {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.IPrintTicketOption";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IPrintTicketOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTicketOptionVtbl {
        unsafe extern "system" fn Name<Impl: IPrintTicketOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNamespace<Impl: IPrintTicketOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNode<Impl: IPrintTicketOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IPrintTicketOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyNode<Impl: IPrintTicketOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyNode(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScoredPropertyNode<Impl: IPrintTicketOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScoredPropertyNode(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Impl: IPrintTicketOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyValue(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScoredPropertyValue<Impl: IPrintTicketOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScoredPropertyValue(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTicketOption, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            XmlNamespace: XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode: XmlNode::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            GetPropertyNode: GetPropertyNode::<Impl, IMPL_OFFSET>,
            GetScoredPropertyNode: GetScoredPropertyNode::<Impl, IMPL_OFFSET>,
            GetPropertyValue: GetPropertyValue::<Impl, IMPL_OFFSET>,
            GetScoredPropertyValue: GetScoredPropertyValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IPrintTicketParameterDefinitionImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&mut self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DataType(&mut self) -> ::windows::core::Result<PrintTicketParameterDataType>;
    fn UnitType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RangeMin(&mut self) -> ::windows::core::Result<i32>;
    fn RangeMax(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTicketParameterDefinition {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterDefinition";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IPrintTicketParameterDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketParameterDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTicketParameterDefinitionVtbl {
        unsafe extern "system" fn Name<Impl: IPrintTicketParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNamespace<Impl: IPrintTicketParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNode<Impl: IPrintTicketParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataType<Impl: IPrintTicketParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintTicketParameterDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnitType<Impl: IPrintTicketParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnitType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMin<Impl: IPrintTicketParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeMin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMax<Impl: IPrintTicketParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeMax() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTicketParameterDefinition, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            XmlNamespace: XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode: XmlNode::<Impl, IMPL_OFFSET>,
            DataType: DataType::<Impl, IMPL_OFFSET>,
            UnitType: UnitType::<Impl, IMPL_OFFSET>,
            RangeMin: RangeMin::<Impl, IMPL_OFFSET>,
            RangeMax: RangeMax::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketParameterDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IPrintTicketParameterInitializerImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&mut self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn SetValue(&mut self, value: &::core::option::Option<PrintTicketValue>) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<PrintTicketValue>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTicketParameterInitializer {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterInitializer";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IPrintTicketParameterInitializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketParameterInitializerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTicketParameterInitializerVtbl {
        unsafe extern "system" fn Name<Impl: IPrintTicketParameterInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNamespace<Impl: IPrintTicketParameterInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNode<Impl: IPrintTicketParameterInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IPrintTicketParameterInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <PrintTicketValue as ::windows::core::Abi>::Abi as *const <PrintTicketValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: IPrintTicketParameterInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTicketParameterInitializer, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            XmlNamespace: XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode: XmlNode::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketParameterInitializer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTicketValueImpl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<PrintTicketValueType>;
    fn GetValueAsInteger(&mut self) -> ::windows::core::Result<i32>;
    fn GetValueAsString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTicketValue {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.IPrintTicketValue";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTicketValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTicketValueVtbl {
        unsafe extern "system" fn Type<Impl: IPrintTicketValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintTicketValueType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAsInteger<Impl: IPrintTicketValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValueAsInteger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAsString<Impl: IPrintTicketValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValueAsString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTicketValue, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            GetValueAsInteger: GetValueAsInteger::<Impl, IMPL_OFFSET>,
            GetValueAsString: GetValueAsString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketValue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWorkflowPrintTicketImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&mut self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn GetCapabilities(&mut self) -> ::windows::core::Result<PrintTicketCapabilities>;
    fn DocumentBindingFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentCollateFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentDuplexFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentHolePunchFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentInputBinFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentNUpFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentStapleFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn JobPasscodeFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageBorderlessFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaSizeFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaTypeFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOrientationFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputColorFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputQualityFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageResolutionFeature(&mut self) -> ::windows::core::Result<PrintTicketFeature>;
    fn GetFeature(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketFeature>;
    fn NotifyXmlChangedAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ValidateAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>>;
    fn GetParameterInitializer(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterInitializer>;
    fn SetParameterInitializerAsInteger(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING, integervalue: i32) -> ::windows::core::Result<PrintTicketParameterInitializer>;
    fn SetParameterInitializerAsString(&mut self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING, stringvalue: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterInitializer>;
    fn MergeAndValidateTicket(&mut self, deltashematicket: &::core::option::Option<WorkflowPrintTicket>) -> ::windows::core::Result<WorkflowPrintTicket>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWorkflowPrintTicket {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicket";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IWorkflowPrintTicketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkflowPrintTicketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkflowPrintTicketVtbl {
        unsafe extern "system" fn Name<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNamespace<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlNode<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentBindingFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentBindingFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentCollateFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentCollateFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentDuplexFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentDuplexFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentHolePunchFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentHolePunchFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentInputBinFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentInputBinFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentNUpFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentNUpFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentStapleFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentStapleFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobPasscodeFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobPasscodeFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageBorderlessFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageBorderlessFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageMediaSizeFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageMediaSizeFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageMediaTypeFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageMediaTypeFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageOrientationFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageOrientationFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageOutputColorFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageOutputColorFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageOutputQualityFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageOutputQualityFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageResolutionFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageResolutionFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeature(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyXmlChangedAsync<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyXmlChangedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateAsync<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameterInitializer<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameterInitializer(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameterInitializerAsInteger<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, integervalue: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetParameterInitializerAsInteger(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), integervalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameterInitializerAsString<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, stringvalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetParameterInitializerAsString(
                &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&xmlnamespace as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&stringvalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MergeAndValidateTicket<Impl: IWorkflowPrintTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deltashematicket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MergeAndValidateTicket(&*(&deltashematicket as *const <WorkflowPrintTicket as ::windows::core::Abi>::Abi as *const <WorkflowPrintTicket as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWorkflowPrintTicket, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            XmlNamespace: XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode: XmlNode::<Impl, IMPL_OFFSET>,
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            DocumentBindingFeature: DocumentBindingFeature::<Impl, IMPL_OFFSET>,
            DocumentCollateFeature: DocumentCollateFeature::<Impl, IMPL_OFFSET>,
            DocumentDuplexFeature: DocumentDuplexFeature::<Impl, IMPL_OFFSET>,
            DocumentHolePunchFeature: DocumentHolePunchFeature::<Impl, IMPL_OFFSET>,
            DocumentInputBinFeature: DocumentInputBinFeature::<Impl, IMPL_OFFSET>,
            DocumentNUpFeature: DocumentNUpFeature::<Impl, IMPL_OFFSET>,
            DocumentStapleFeature: DocumentStapleFeature::<Impl, IMPL_OFFSET>,
            JobPasscodeFeature: JobPasscodeFeature::<Impl, IMPL_OFFSET>,
            PageBorderlessFeature: PageBorderlessFeature::<Impl, IMPL_OFFSET>,
            PageMediaSizeFeature: PageMediaSizeFeature::<Impl, IMPL_OFFSET>,
            PageMediaTypeFeature: PageMediaTypeFeature::<Impl, IMPL_OFFSET>,
            PageOrientationFeature: PageOrientationFeature::<Impl, IMPL_OFFSET>,
            PageOutputColorFeature: PageOutputColorFeature::<Impl, IMPL_OFFSET>,
            PageOutputQualityFeature: PageOutputQualityFeature::<Impl, IMPL_OFFSET>,
            PageResolutionFeature: PageResolutionFeature::<Impl, IMPL_OFFSET>,
            GetFeature: GetFeature::<Impl, IMPL_OFFSET>,
            NotifyXmlChangedAsync: NotifyXmlChangedAsync::<Impl, IMPL_OFFSET>,
            ValidateAsync: ValidateAsync::<Impl, IMPL_OFFSET>,
            GetParameterInitializer: GetParameterInitializer::<Impl, IMPL_OFFSET>,
            SetParameterInitializerAsInteger: SetParameterInitializerAsInteger::<Impl, IMPL_OFFSET>,
            SetParameterInitializerAsString: SetParameterInitializerAsString::<Impl, IMPL_OFFSET>,
            MergeAndValidateTicket: MergeAndValidateTicket::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkflowPrintTicket as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWorkflowPrintTicketValidationResultImpl: Sized {
    fn Validated(&mut self) -> ::windows::core::Result<bool>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWorkflowPrintTicketValidationResult {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketValidationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IWorkflowPrintTicketValidationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkflowPrintTicketValidationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkflowPrintTicketValidationResultVtbl {
        unsafe extern "system" fn Validated<Impl: IWorkflowPrintTicketValidationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Validated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IWorkflowPrintTicketValidationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWorkflowPrintTicketValidationResult, BASE_OFFSET>(),
            Validated: Validated::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkflowPrintTicketValidationResult as ::windows::core::Interface>::IID
    }
}
