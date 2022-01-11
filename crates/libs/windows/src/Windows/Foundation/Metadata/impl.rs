#[cfg(feature = "implement_exclusive")]
pub trait IApiInformationStaticsImpl: Sized {
    fn IsTypePresent(&self, typename: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsMethodPresent(&self, typename: &::windows::core::HSTRING, methodname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsMethodPresentWithArity(&self, typename: &::windows::core::HSTRING, methodname: &::windows::core::HSTRING, inputparametercount: u32) -> ::windows::core::Result<bool>;
    fn IsEventPresent(&self, typename: &::windows::core::HSTRING, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsPropertyPresent(&self, typename: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsReadOnlyPropertyPresent(&self, typename: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsWriteablePropertyPresent(&self, typename: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsEnumNamedValuePresent(&self, enumtypename: &::windows::core::HSTRING, valuename: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsApiContractPresentByMajor(&self, contractname: &::windows::core::HSTRING, majorversion: u16) -> ::windows::core::Result<bool>;
    fn IsApiContractPresentByMajorAndMinor(&self, contractname: &::windows::core::HSTRING, majorversion: u16, minorversion: u16) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApiInformationStatics {
    const NAME: &'static str = "Windows.Foundation.Metadata.IApiInformationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApiInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApiInformationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApiInformationStaticsVtbl {
        unsafe extern "system" fn IsTypePresent<Impl: IApiInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTypePresent(&*(&typename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMethodPresent<Impl: IApiInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, methodname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMethodPresent(&*(&typename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&methodname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMethodPresentWithArity<Impl: IApiInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, methodname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputparametercount: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMethodPresentWithArity(&*(&typename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&methodname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), inputparametercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEventPresent<Impl: IApiInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEventPresent(&*(&typename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyPresent<Impl: IApiInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPropertyPresent(&*(&typename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnlyPropertyPresent<Impl: IApiInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnlyPropertyPresent(&*(&typename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWriteablePropertyPresent<Impl: IApiInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWriteablePropertyPresent(&*(&typename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnumNamedValuePresent<Impl: IApiInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumtypename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, valuename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnumNamedValuePresent(&*(&enumtypename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&valuename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApiContractPresentByMajor<Impl: IApiInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contractname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, majorversion: u16, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsApiContractPresentByMajor(&*(&contractname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), majorversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApiContractPresentByMajorAndMinor<Impl: IApiInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contractname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, majorversion: u16, minorversion: u16, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsApiContractPresentByMajorAndMinor(&*(&contractname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), majorversion, minorversion) {
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
            ::windows::core::GetRuntimeClassName::<IApiInformationStatics>,
            ::windows::core::GetTrustLevel,
            IsTypePresent::<Impl, IMPL_OFFSET>,
            IsMethodPresent::<Impl, IMPL_OFFSET>,
            IsMethodPresentWithArity::<Impl, IMPL_OFFSET>,
            IsEventPresent::<Impl, IMPL_OFFSET>,
            IsPropertyPresent::<Impl, IMPL_OFFSET>,
            IsReadOnlyPropertyPresent::<Impl, IMPL_OFFSET>,
            IsWriteablePropertyPresent::<Impl, IMPL_OFFSET>,
            IsEnumNamedValuePresent::<Impl, IMPL_OFFSET>,
            IsApiContractPresentByMajor::<Impl, IMPL_OFFSET>,
            IsApiContractPresentByMajorAndMinor::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApiInformationStatics as ::windows::core::Interface>::IID
    }
}
