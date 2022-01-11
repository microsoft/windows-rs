#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IIppAttributeErrorImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<IppAttributeErrorReason>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetUnsupportedValues(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IppAttributeValue>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IIppAttributeError {
    const NAME: &'static str = "Windows.Devices.Printers.IIppAttributeError";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IIppAttributeErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppAttributeErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppAttributeErrorVtbl {
        unsafe extern "system" fn Reason<Impl: IIppAttributeErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IppAttributeErrorReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IIppAttributeErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetUnsupportedValues<Impl: IIppAttributeErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnsupportedValues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppAttributeError, BASE_OFFSET>(),
            Reason: Reason::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
            GetUnsupportedValues: GetUnsupportedValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppAttributeError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IIppAttributeValueImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<IppAttributeValueKind>;
    fn GetIntegerArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>>;
    fn GetBooleanArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<bool>>;
    fn GetEnumArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>>;
    fn GetOctetStringArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>;
    fn GetDateTimeArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::DateTime>>;
    fn GetResolutionArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppResolution>>;
    fn GetRangeOfIntegerArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppIntegerRange>>;
    fn GetCollectionArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IppAttributeValue>>>;
    fn GetTextWithLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppTextWithLanguage>>;
    fn GetNameWithLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppTextWithLanguage>>;
    fn GetTextWithoutLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetNameWithoutLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetKeywordArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetUriArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn GetUriSchemaArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetCharsetArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetNaturalLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetMimeMediaTypeArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IIppAttributeValue {
    const NAME: &'static str = "Windows.Devices.Printers.IIppAttributeValue";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IIppAttributeValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppAttributeValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppAttributeValueVtbl {
        unsafe extern "system" fn Kind<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IppAttributeValueKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntegerArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBooleanArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBooleanArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOctetStringArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOctetStringArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDateTimeArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDateTimeArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolutionArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResolutionArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeOfIntegerArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRangeOfIntegerArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollectionArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextWithLanguageArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextWithLanguageArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameWithLanguageArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameWithLanguageArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextWithoutLanguageArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextWithoutLanguageArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameWithoutLanguageArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameWithoutLanguageArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeywordArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeywordArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUriArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUriArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUriSchemaArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUriSchemaArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharsetArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharsetArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNaturalLanguageArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNaturalLanguageArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMimeMediaTypeArray<Impl: IIppAttributeValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMimeMediaTypeArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppAttributeValue, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
            GetIntegerArray: GetIntegerArray::<Impl, IMPL_OFFSET>,
            GetBooleanArray: GetBooleanArray::<Impl, IMPL_OFFSET>,
            GetEnumArray: GetEnumArray::<Impl, IMPL_OFFSET>,
            GetOctetStringArray: GetOctetStringArray::<Impl, IMPL_OFFSET>,
            GetDateTimeArray: GetDateTimeArray::<Impl, IMPL_OFFSET>,
            GetResolutionArray: GetResolutionArray::<Impl, IMPL_OFFSET>,
            GetRangeOfIntegerArray: GetRangeOfIntegerArray::<Impl, IMPL_OFFSET>,
            GetCollectionArray: GetCollectionArray::<Impl, IMPL_OFFSET>,
            GetTextWithLanguageArray: GetTextWithLanguageArray::<Impl, IMPL_OFFSET>,
            GetNameWithLanguageArray: GetNameWithLanguageArray::<Impl, IMPL_OFFSET>,
            GetTextWithoutLanguageArray: GetTextWithoutLanguageArray::<Impl, IMPL_OFFSET>,
            GetNameWithoutLanguageArray: GetNameWithoutLanguageArray::<Impl, IMPL_OFFSET>,
            GetKeywordArray: GetKeywordArray::<Impl, IMPL_OFFSET>,
            GetUriArray: GetUriArray::<Impl, IMPL_OFFSET>,
            GetUriSchemaArray: GetUriSchemaArray::<Impl, IMPL_OFFSET>,
            GetCharsetArray: GetCharsetArray::<Impl, IMPL_OFFSET>,
            GetNaturalLanguageArray: GetNaturalLanguageArray::<Impl, IMPL_OFFSET>,
            GetMimeMediaTypeArray: GetMimeMediaTypeArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppAttributeValue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IIppAttributeValueStaticsImpl: Sized {
    fn CreateUnsupported(&self) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateUnknown(&self) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNoValue(&self) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateInteger(&self, value: i32) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateIntegerArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<i32>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateBoolean(&self, value: bool) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateBooleanArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<bool>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateEnum(&self, value: i32) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateEnumArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<i32>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateOctetString(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateOctetStringArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::Streams::IBuffer>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateDateTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateDateTimeArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::DateTime>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateResolution(&self, value: &::core::option::Option<IppResolution>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateResolutionArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<IppResolution>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateRangeOfInteger(&self, value: &::core::option::Option<IppIntegerRange>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateRangeOfIntegerArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<IppIntegerRange>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateCollection(&self, memberattributes: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateCollectionArray(&self, memberattributesarray: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateTextWithLanguage(&self, value: &::core::option::Option<IppTextWithLanguage>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateTextWithLanguageArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<IppTextWithLanguage>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNameWithLanguage(&self, value: &::core::option::Option<IppTextWithLanguage>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNameWithLanguageArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<IppTextWithLanguage>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateTextWithoutLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateTextWithoutLanguageArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNameWithoutLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNameWithoutLanguageArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateKeyword(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateKeywordArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateUriArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateUriSchema(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateUriSchemaArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateCharset(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateCharsetArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNaturalLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNaturalLanguageArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateMimeMedia(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateMimeMediaArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IIppAttributeValueStatics {
    const NAME: &'static str = "Windows.Devices.Printers.IIppAttributeValueStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IIppAttributeValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppAttributeValueStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppAttributeValueStaticsVtbl {
        unsafe extern "system" fn CreateUnsupported<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUnsupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUnknown<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUnknown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNoValue<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNoValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInteger<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInteger(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateIntegerArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateIntegerArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBoolean<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBoolean(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBooleanArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBooleanArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<bool> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<bool> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnum<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEnum(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnumArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEnumArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOctetString<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOctetString(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOctetStringArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOctetStringArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::Streams::IBuffer> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::Streams::IBuffer> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDateTime<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDateTimeArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTimeArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResolution<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResolution(&*(&value as *const <IppResolution as ::windows::core::Abi>::Abi as *const <IppResolution as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResolutionArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResolutionArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<IppResolution> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<IppResolution> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRangeOfInteger<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRangeOfInteger(&*(&value as *const <IppIntegerRange as ::windows::core::Abi>::Abi as *const <IppIntegerRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRangeOfIntegerArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRangeOfIntegerArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<IppIntegerRange> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<IppIntegerRange> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCollection<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memberattributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCollection(&*(&memberattributes as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCollectionArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memberattributesarray: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCollectionArray(&*(&memberattributesarray as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextWithLanguage<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTextWithLanguage(&*(&value as *const <IppTextWithLanguage as ::windows::core::Abi>::Abi as *const <IppTextWithLanguage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextWithLanguageArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTextWithLanguageArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<IppTextWithLanguage> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<IppTextWithLanguage> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNameWithLanguage<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNameWithLanguage(&*(&value as *const <IppTextWithLanguage as ::windows::core::Abi>::Abi as *const <IppTextWithLanguage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNameWithLanguageArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNameWithLanguageArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<IppTextWithLanguage> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<IppTextWithLanguage> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextWithoutLanguage<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTextWithoutLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextWithoutLanguageArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTextWithoutLanguageArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNameWithoutLanguage<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNameWithoutLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNameWithoutLanguageArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNameWithoutLanguageArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKeyword<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKeyword(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKeywordArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKeywordArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUri<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUriArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUriArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUriSchema<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUriSchema(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUriSchemaArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUriSchemaArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCharset<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCharset(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCharsetArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCharsetArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNaturalLanguage<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNaturalLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNaturalLanguageArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNaturalLanguageArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMimeMedia<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMimeMedia(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMimeMediaArray<Impl: IIppAttributeValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMimeMediaArray(&*(&values as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppAttributeValueStatics, BASE_OFFSET>(),
            CreateUnsupported: CreateUnsupported::<Impl, IMPL_OFFSET>,
            CreateUnknown: CreateUnknown::<Impl, IMPL_OFFSET>,
            CreateNoValue: CreateNoValue::<Impl, IMPL_OFFSET>,
            CreateInteger: CreateInteger::<Impl, IMPL_OFFSET>,
            CreateIntegerArray: CreateIntegerArray::<Impl, IMPL_OFFSET>,
            CreateBoolean: CreateBoolean::<Impl, IMPL_OFFSET>,
            CreateBooleanArray: CreateBooleanArray::<Impl, IMPL_OFFSET>,
            CreateEnum: CreateEnum::<Impl, IMPL_OFFSET>,
            CreateEnumArray: CreateEnumArray::<Impl, IMPL_OFFSET>,
            CreateOctetString: CreateOctetString::<Impl, IMPL_OFFSET>,
            CreateOctetStringArray: CreateOctetStringArray::<Impl, IMPL_OFFSET>,
            CreateDateTime: CreateDateTime::<Impl, IMPL_OFFSET>,
            CreateDateTimeArray: CreateDateTimeArray::<Impl, IMPL_OFFSET>,
            CreateResolution: CreateResolution::<Impl, IMPL_OFFSET>,
            CreateResolutionArray: CreateResolutionArray::<Impl, IMPL_OFFSET>,
            CreateRangeOfInteger: CreateRangeOfInteger::<Impl, IMPL_OFFSET>,
            CreateRangeOfIntegerArray: CreateRangeOfIntegerArray::<Impl, IMPL_OFFSET>,
            CreateCollection: CreateCollection::<Impl, IMPL_OFFSET>,
            CreateCollectionArray: CreateCollectionArray::<Impl, IMPL_OFFSET>,
            CreateTextWithLanguage: CreateTextWithLanguage::<Impl, IMPL_OFFSET>,
            CreateTextWithLanguageArray: CreateTextWithLanguageArray::<Impl, IMPL_OFFSET>,
            CreateNameWithLanguage: CreateNameWithLanguage::<Impl, IMPL_OFFSET>,
            CreateNameWithLanguageArray: CreateNameWithLanguageArray::<Impl, IMPL_OFFSET>,
            CreateTextWithoutLanguage: CreateTextWithoutLanguage::<Impl, IMPL_OFFSET>,
            CreateTextWithoutLanguageArray: CreateTextWithoutLanguageArray::<Impl, IMPL_OFFSET>,
            CreateNameWithoutLanguage: CreateNameWithoutLanguage::<Impl, IMPL_OFFSET>,
            CreateNameWithoutLanguageArray: CreateNameWithoutLanguageArray::<Impl, IMPL_OFFSET>,
            CreateKeyword: CreateKeyword::<Impl, IMPL_OFFSET>,
            CreateKeywordArray: CreateKeywordArray::<Impl, IMPL_OFFSET>,
            CreateUri: CreateUri::<Impl, IMPL_OFFSET>,
            CreateUriArray: CreateUriArray::<Impl, IMPL_OFFSET>,
            CreateUriSchema: CreateUriSchema::<Impl, IMPL_OFFSET>,
            CreateUriSchemaArray: CreateUriSchemaArray::<Impl, IMPL_OFFSET>,
            CreateCharset: CreateCharset::<Impl, IMPL_OFFSET>,
            CreateCharsetArray: CreateCharsetArray::<Impl, IMPL_OFFSET>,
            CreateNaturalLanguage: CreateNaturalLanguage::<Impl, IMPL_OFFSET>,
            CreateNaturalLanguageArray: CreateNaturalLanguageArray::<Impl, IMPL_OFFSET>,
            CreateMimeMedia: CreateMimeMedia::<Impl, IMPL_OFFSET>,
            CreateMimeMediaArray: CreateMimeMediaArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppAttributeValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppIntegerRangeImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<i32>;
    fn End(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIppIntegerRange {
    const NAME: &'static str = "Windows.Devices.Printers.IIppIntegerRange";
}
#[cfg(feature = "implement_exclusive")]
impl IIppIntegerRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppIntegerRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppIntegerRangeVtbl {
        unsafe extern "system" fn Start<Impl: IIppIntegerRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: IIppIntegerRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).End() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppIntegerRange, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppIntegerRange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppIntegerRangeFactoryImpl: Sized {
    fn CreateInstance(&self, start: i32, end: i32) -> ::windows::core::Result<IppIntegerRange>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIppIntegerRangeFactory {
    const NAME: &'static str = "Windows.Devices.Printers.IIppIntegerRangeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IIppIntegerRangeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppIntegerRangeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppIntegerRangeFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IIppIntegerRangeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: i32, end: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(start, end) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppIntegerRangeFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppIntegerRangeFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IIppPrintDeviceImpl: Sized {
    fn PrinterName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PrinterUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetPrinterAttributesAsBuffer(&self, attributenames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn GetPrinterAttributes(&self, attributenames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IppAttributeValue>>;
    fn SetPrinterAttributesFromBuffer(&self, printerattributesbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<IppSetAttributesResult>;
    fn SetPrinterAttributes(&self, printerattributes: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>>) -> ::windows::core::Result<IppSetAttributesResult>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IIppPrintDevice {
    const NAME: &'static str = "Windows.Devices.Printers.IIppPrintDevice";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IIppPrintDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppPrintDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppPrintDeviceVtbl {
        unsafe extern "system" fn PrinterName<Impl: IIppPrintDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrinterUri<Impl: IIppPrintDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrinterAttributesAsBuffer<Impl: IIppPrintDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributenames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrinterAttributesAsBuffer(&*(&attributenames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrinterAttributes<Impl: IIppPrintDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributenames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrinterAttributes(&*(&attributenames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrinterAttributesFromBuffer<Impl: IIppPrintDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printerattributesbuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrinterAttributesFromBuffer(&*(&printerattributesbuffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrinterAttributes<Impl: IIppPrintDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printerattributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrinterAttributes(&*(&printerattributes as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppPrintDevice, BASE_OFFSET>(),
            PrinterName: PrinterName::<Impl, IMPL_OFFSET>,
            PrinterUri: PrinterUri::<Impl, IMPL_OFFSET>,
            GetPrinterAttributesAsBuffer: GetPrinterAttributesAsBuffer::<Impl, IMPL_OFFSET>,
            GetPrinterAttributes: GetPrinterAttributes::<Impl, IMPL_OFFSET>,
            SetPrinterAttributesFromBuffer: SetPrinterAttributesFromBuffer::<Impl, IMPL_OFFSET>,
            SetPrinterAttributes: SetPrinterAttributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppPrintDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppResolutionImpl: Sized {
    fn Width(&self) -> ::windows::core::Result<i32>;
    fn Height(&self) -> ::windows::core::Result<i32>;
    fn Unit(&self) -> ::windows::core::Result<IppResolutionUnit>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIppResolution {
    const NAME: &'static str = "Windows.Devices.Printers.IIppResolution";
}
#[cfg(feature = "implement_exclusive")]
impl IIppResolutionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppResolutionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppResolutionVtbl {
        unsafe extern "system" fn Width<Impl: IIppResolutionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IIppResolutionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unit<Impl: IIppResolutionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IppResolutionUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppResolution, BASE_OFFSET>(),
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            Unit: Unit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppResolution as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppResolutionFactoryImpl: Sized {
    fn CreateInstance(&self, width: i32, height: i32, unit: IppResolutionUnit) -> ::windows::core::Result<IppResolution>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIppResolutionFactory {
    const NAME: &'static str = "Windows.Devices.Printers.IIppResolutionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IIppResolutionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppResolutionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppResolutionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IIppResolutionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, unit: IppResolutionUnit, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(width, height, unit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppResolutionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppResolutionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IIppSetAttributesResultImpl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn AttributeErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IppAttributeError>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IIppSetAttributesResult {
    const NAME: &'static str = "Windows.Devices.Printers.IIppSetAttributesResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IIppSetAttributesResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppSetAttributesResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppSetAttributesResultVtbl {
        unsafe extern "system" fn Succeeded<Impl: IIppSetAttributesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Succeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttributeErrors<Impl: IIppSetAttributesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttributeErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppSetAttributesResult, BASE_OFFSET>(),
            Succeeded: Succeeded::<Impl, IMPL_OFFSET>,
            AttributeErrors: AttributeErrors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppSetAttributesResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppTextWithLanguageImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIppTextWithLanguage {
    const NAME: &'static str = "Windows.Devices.Printers.IIppTextWithLanguage";
}
#[cfg(feature = "implement_exclusive")]
impl IIppTextWithLanguageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppTextWithLanguageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppTextWithLanguageVtbl {
        unsafe extern "system" fn Language<Impl: IIppTextWithLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IIppTextWithLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppTextWithLanguage, BASE_OFFSET>(),
            Language: Language::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppTextWithLanguage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppTextWithLanguageFactoryImpl: Sized {
    fn CreateInstance(&self, language: &::windows::core::HSTRING, text: &::windows::core::HSTRING) -> ::windows::core::Result<IppTextWithLanguage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIppTextWithLanguageFactory {
    const NAME: &'static str = "Windows.Devices.Printers.IIppTextWithLanguageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IIppTextWithLanguageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIppTextWithLanguageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIppTextWithLanguageFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IIppTextWithLanguageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIppTextWithLanguageFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIppTextWithLanguageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DDeviceImpl: Sized {
    fn PrintSchema(&self) -> ::windows::core::Result<PrintSchema>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DDevice {
    const NAME: &'static str = "Windows.Devices.Printers.IPrint3DDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DDeviceVtbl {
        unsafe extern "system" fn PrintSchema<Impl: IPrint3DDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintSchema() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DDevice, BASE_OFFSET>(), PrintSchema: PrintSchema::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrint3DDeviceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Print3DDevice>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrint3DDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Printers.IPrint3DDeviceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrint3DDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DDeviceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DDeviceStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IPrint3DDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IPrint3DDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DDeviceStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrintSchemaImpl: Sized {
    fn GetDefaultPrintTicketAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
    fn GetCapabilitiesAsync(&self, constrainticket: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamWithContentType>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
    fn MergeAndValidateWithDefaultPrintTicketAsync(&self, deltaticket: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamWithContentType>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSchema {
    const NAME: &'static str = "Windows.Devices.Printers.IPrintSchema";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintSchemaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaVtbl {
        unsafe extern "system" fn GetDefaultPrintTicketAsync<Impl: IPrintSchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultPrintTicketAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilitiesAsync<Impl: IPrintSchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, constrainticket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilitiesAsync(&*(&constrainticket as *const <super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MergeAndValidateWithDefaultPrintTicketAsync<Impl: IPrintSchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deltaticket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MergeAndValidateWithDefaultPrintTicketAsync(&*(&deltaticket as *const <super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintSchema, BASE_OFFSET>(),
            GetDefaultPrintTicketAsync: GetDefaultPrintTicketAsync::<Impl, IMPL_OFFSET>,
            GetCapabilitiesAsync: GetCapabilitiesAsync::<Impl, IMPL_OFFSET>,
            MergeAndValidateWithDefaultPrintTicketAsync: MergeAndValidateWithDefaultPrintTicketAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchema as ::windows::core::Interface>::IID
    }
}
