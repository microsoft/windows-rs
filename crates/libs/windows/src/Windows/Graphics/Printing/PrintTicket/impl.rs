#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IPrintTicketCapabilitiesImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DocumentBindingFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentCollateFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentDuplexFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentHolePunchFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentInputBinFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentNUpFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentStapleFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn JobPasscodeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageBorderlessFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaSizeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaTypeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOrientationFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputColorFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputQualityFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageResolutionFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn GetFeature(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketFeature>;
    fn GetParameterDefinition(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterDefinition>;
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPrintTicketCapabilities>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, IMPL_OFFSET>,
            XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            DocumentBindingFeature::<Impl, IMPL_OFFSET>,
            DocumentCollateFeature::<Impl, IMPL_OFFSET>,
            DocumentDuplexFeature::<Impl, IMPL_OFFSET>,
            DocumentHolePunchFeature::<Impl, IMPL_OFFSET>,
            DocumentInputBinFeature::<Impl, IMPL_OFFSET>,
            DocumentNUpFeature::<Impl, IMPL_OFFSET>,
            DocumentStapleFeature::<Impl, IMPL_OFFSET>,
            JobPasscodeFeature::<Impl, IMPL_OFFSET>,
            PageBorderlessFeature::<Impl, IMPL_OFFSET>,
            PageMediaSizeFeature::<Impl, IMPL_OFFSET>,
            PageMediaTypeFeature::<Impl, IMPL_OFFSET>,
            PageOrientationFeature::<Impl, IMPL_OFFSET>,
            PageOutputColorFeature::<Impl, IMPL_OFFSET>,
            PageOutputQualityFeature::<Impl, IMPL_OFFSET>,
            PageResolutionFeature::<Impl, IMPL_OFFSET>,
            GetFeature::<Impl, IMPL_OFFSET>,
            GetParameterDefinition::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrintTicketFeatureImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetOption(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketOption>;
    fn Options(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PrintTicketOption>>;
    fn GetSelectedOption(&self) -> ::windows::core::Result<PrintTicketOption>;
    fn SetSelectedOption(&self, value: &::core::option::Option<PrintTicketOption>) -> ::windows::core::Result<()>;
    fn SelectionType(&self) -> ::windows::core::Result<PrintTicketFeatureSelectionType>;
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPrintTicketFeature>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, IMPL_OFFSET>,
            XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            GetOption::<Impl, IMPL_OFFSET>,
            Options::<Impl, IMPL_OFFSET>,
            GetSelectedOption::<Impl, IMPL_OFFSET>,
            SetSelectedOption::<Impl, IMPL_OFFSET>,
            SelectionType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketFeature as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IPrintTicketOptionImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetPropertyNode(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn GetScoredPropertyNode(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn GetPropertyValue(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketValue>;
    fn GetScoredPropertyValue(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketValue>;
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPrintTicketOption>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, IMPL_OFFSET>,
            XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            GetPropertyNode::<Impl, IMPL_OFFSET>,
            GetScoredPropertyNode::<Impl, IMPL_OFFSET>,
            GetPropertyValue::<Impl, IMPL_OFFSET>,
            GetScoredPropertyValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IPrintTicketParameterDefinitionImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DataType(&self) -> ::windows::core::Result<PrintTicketParameterDataType>;
    fn UnitType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RangeMin(&self) -> ::windows::core::Result<i32>;
    fn RangeMax(&self) -> ::windows::core::Result<i32>;
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPrintTicketParameterDefinition>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, IMPL_OFFSET>,
            XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            DataType::<Impl, IMPL_OFFSET>,
            UnitType::<Impl, IMPL_OFFSET>,
            RangeMin::<Impl, IMPL_OFFSET>,
            RangeMax::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketParameterDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IPrintTicketParameterInitializerImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn SetValue(&self, value: &::core::option::Option<PrintTicketValue>) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<PrintTicketValue>;
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTicketParameterInitializer>, ::windows::core::GetTrustLevel, Name::<Impl, IMPL_OFFSET>, XmlNamespace::<Impl, IMPL_OFFSET>, XmlNode::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketParameterInitializer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTicketValueImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<PrintTicketValueType>;
    fn GetValueAsInteger(&self) -> ::windows::core::Result<i32>;
    fn GetValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTicketValue>, ::windows::core::GetTrustLevel, Type::<Impl, IMPL_OFFSET>, GetValueAsInteger::<Impl, IMPL_OFFSET>, GetValueAsString::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketValue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWorkflowPrintTicketImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn GetCapabilities(&self) -> ::windows::core::Result<PrintTicketCapabilities>;
    fn DocumentBindingFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentCollateFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentDuplexFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentHolePunchFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentInputBinFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentNUpFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentStapleFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn JobPasscodeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageBorderlessFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaSizeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaTypeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOrientationFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputColorFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputQualityFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageResolutionFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn GetFeature(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketFeature>;
    fn NotifyXmlChangedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ValidateAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>>;
    fn GetParameterInitializer(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterInitializer>;
    fn SetParameterInitializerAsInteger(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING, integervalue: i32) -> ::windows::core::Result<PrintTicketParameterInitializer>;
    fn SetParameterInitializerAsString(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING, stringvalue: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterInitializer>;
    fn MergeAndValidateTicket(&self, deltashematicket: &::core::option::Option<WorkflowPrintTicket>) -> ::windows::core::Result<WorkflowPrintTicket>;
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWorkflowPrintTicket>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, IMPL_OFFSET>,
            XmlNamespace::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            DocumentBindingFeature::<Impl, IMPL_OFFSET>,
            DocumentCollateFeature::<Impl, IMPL_OFFSET>,
            DocumentDuplexFeature::<Impl, IMPL_OFFSET>,
            DocumentHolePunchFeature::<Impl, IMPL_OFFSET>,
            DocumentInputBinFeature::<Impl, IMPL_OFFSET>,
            DocumentNUpFeature::<Impl, IMPL_OFFSET>,
            DocumentStapleFeature::<Impl, IMPL_OFFSET>,
            JobPasscodeFeature::<Impl, IMPL_OFFSET>,
            PageBorderlessFeature::<Impl, IMPL_OFFSET>,
            PageMediaSizeFeature::<Impl, IMPL_OFFSET>,
            PageMediaTypeFeature::<Impl, IMPL_OFFSET>,
            PageOrientationFeature::<Impl, IMPL_OFFSET>,
            PageOutputColorFeature::<Impl, IMPL_OFFSET>,
            PageOutputQualityFeature::<Impl, IMPL_OFFSET>,
            PageResolutionFeature::<Impl, IMPL_OFFSET>,
            GetFeature::<Impl, IMPL_OFFSET>,
            NotifyXmlChangedAsync::<Impl, IMPL_OFFSET>,
            ValidateAsync::<Impl, IMPL_OFFSET>,
            GetParameterInitializer::<Impl, IMPL_OFFSET>,
            SetParameterInitializerAsInteger::<Impl, IMPL_OFFSET>,
            SetParameterInitializerAsString::<Impl, IMPL_OFFSET>,
            MergeAndValidateTicket::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkflowPrintTicket as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWorkflowPrintTicketValidationResultImpl: Sized {
    fn Validated(&self) -> ::windows::core::Result<bool>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkflowPrintTicketValidationResult>, ::windows::core::GetTrustLevel, Validated::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkflowPrintTicketValidationResult as ::windows::core::Interface>::IID
    }
}
