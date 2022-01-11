#[cfg(feature = "implement_exclusive")]
pub trait IJsonArrayImpl: Sized + IJsonValueImpl {
    fn GetObjectAt(&self, index: u32) -> ::windows::core::Result<JsonObject>;
    fn GetArrayAt(&self, index: u32) -> ::windows::core::Result<JsonArray>;
    fn GetStringAt(&self, index: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNumberAt(&self, index: u32) -> ::windows::core::Result<f64>;
    fn GetBooleanAt(&self, index: u32) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonArray {
    const NAME: &'static str = "Windows.Data.Json.IJsonArray";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonArrayVtbl {
        unsafe extern "system" fn GetObjectAt<Impl: IJsonArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArrayAt<Impl: IJsonArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetArrayAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAt<Impl: IJsonArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberAt<Impl: IJsonArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBooleanAt<Impl: IJsonArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBooleanAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IJsonArray>, ::windows::core::GetTrustLevel, GetObjectAt::<Impl, IMPL_OFFSET>, GetArrayAt::<Impl, IMPL_OFFSET>, GetStringAt::<Impl, IMPL_OFFSET>, GetNumberAt::<Impl, IMPL_OFFSET>, GetBooleanAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonArray as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonArrayStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonArray>;
    fn TryParse(&self, input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonArray>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonArrayStatics {
    const NAME: &'static str = "Windows.Data.Json.IJsonArrayStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonArrayStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonArrayStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonArrayStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IJsonArrayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IJsonArrayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IJsonArrayStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, IMPL_OFFSET>, TryParse::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonArrayStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonErrorStatics2Impl: Sized {
    fn GetJsonStatus(&self, hresult: i32) -> ::windows::core::Result<JsonErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonErrorStatics2 {
    const NAME: &'static str = "Windows.Data.Json.IJsonErrorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonErrorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonErrorStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonErrorStatics2Vtbl {
        unsafe extern "system" fn GetJsonStatus<Impl: IJsonErrorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut JsonErrorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJsonStatus(hresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IJsonErrorStatics2>, ::windows::core::GetTrustLevel, GetJsonStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonErrorStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonObjectImpl: Sized + IJsonValueImpl {
    fn GetNamedValue(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue>;
    fn SetNamedValue(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<IJsonValue>) -> ::windows::core::Result<()>;
    fn GetNamedObject(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonObject>;
    fn GetNamedArray(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonArray>;
    fn GetNamedString(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNamedNumber(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<f64>;
    fn GetNamedBoolean(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonObject {
    const NAME: &'static str = "Windows.Data.Json.IJsonObject";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonObjectVtbl {
        unsafe extern "system" fn GetNamedValue<Impl: IJsonObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedValue(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamedValue<Impl: IJsonObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamedValue(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <IJsonValue as ::windows::core::Abi>::Abi as *const <IJsonValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetNamedObject<Impl: IJsonObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedObject(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedArray<Impl: IJsonObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedString<Impl: IJsonObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedString(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedNumber<Impl: IJsonObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedNumber(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedBoolean<Impl: IJsonObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedBoolean(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IJsonObject>,
            ::windows::core::GetTrustLevel,
            GetNamedValue::<Impl, IMPL_OFFSET>,
            SetNamedValue::<Impl, IMPL_OFFSET>,
            GetNamedObject::<Impl, IMPL_OFFSET>,
            GetNamedArray::<Impl, IMPL_OFFSET>,
            GetNamedString::<Impl, IMPL_OFFSET>,
            GetNamedNumber::<Impl, IMPL_OFFSET>,
            GetNamedBoolean::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonObjectStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonObject>;
    fn TryParse(&self, input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonObjectStatics {
    const NAME: &'static str = "Windows.Data.Json.IJsonObjectStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonObjectStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonObjectStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonObjectStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IJsonObjectStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IJsonObjectStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IJsonObjectStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, IMPL_OFFSET>, TryParse::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonObjectStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonObjectWithDefaultValuesImpl: Sized + IJsonObjectImpl + IJsonValueImpl {
    fn GetNamedValueOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &::core::option::Option<JsonValue>) -> ::windows::core::Result<JsonValue>;
    fn GetNamedObjectOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &::core::option::Option<JsonObject>) -> ::windows::core::Result<JsonObject>;
    fn GetNamedStringOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNamedArrayOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &::core::option::Option<JsonArray>) -> ::windows::core::Result<JsonArray>;
    fn GetNamedNumberOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: f64) -> ::windows::core::Result<f64>;
    fn GetNamedBooleanOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonObjectWithDefaultValues {
    const NAME: &'static str = "Windows.Data.Json.IJsonObjectWithDefaultValues";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonObjectWithDefaultValuesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonObjectWithDefaultValuesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonObjectWithDefaultValuesVtbl {
        unsafe extern "system" fn GetNamedValueOrDefault<Impl: IJsonObjectWithDefaultValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedValueOrDefault(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&defaultvalue as *const <JsonValue as ::windows::core::Abi>::Abi as *const <JsonValue as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedObjectOrDefault<Impl: IJsonObjectWithDefaultValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedObjectOrDefault(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&defaultvalue as *const <JsonObject as ::windows::core::Abi>::Abi as *const <JsonObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedStringOrDefault<Impl: IJsonObjectWithDefaultValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedStringOrDefault(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&defaultvalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedArrayOrDefault<Impl: IJsonObjectWithDefaultValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedArrayOrDefault(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&defaultvalue as *const <JsonArray as ::windows::core::Abi>::Abi as *const <JsonArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedNumberOrDefault<Impl: IJsonObjectWithDefaultValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: f64, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedNumberOrDefault(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), defaultvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedBooleanOrDefault<Impl: IJsonObjectWithDefaultValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedBooleanOrDefault(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), defaultvalue) {
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
            ::windows::core::GetRuntimeClassName::<IJsonObjectWithDefaultValues>,
            ::windows::core::GetTrustLevel,
            GetNamedValueOrDefault::<Impl, IMPL_OFFSET>,
            GetNamedObjectOrDefault::<Impl, IMPL_OFFSET>,
            GetNamedStringOrDefault::<Impl, IMPL_OFFSET>,
            GetNamedArrayOrDefault::<Impl, IMPL_OFFSET>,
            GetNamedNumberOrDefault::<Impl, IMPL_OFFSET>,
            GetNamedBooleanOrDefault::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonObjectWithDefaultValues as ::windows::core::Interface>::IID
    }
}
pub trait IJsonValueImpl: Sized {
    fn ValueType(&self) -> ::windows::core::Result<JsonValueType>;
    fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNumber(&self) -> ::windows::core::Result<f64>;
    fn GetBoolean(&self) -> ::windows::core::Result<bool>;
    fn GetArray(&self) -> ::windows::core::Result<JsonArray>;
    fn GetObject(&self) -> ::windows::core::Result<JsonObject>;
}
impl ::windows::core::RuntimeName for IJsonValue {
    const NAME: &'static str = "Windows.Data.Json.IJsonValue";
}
impl IJsonValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonValueVtbl {
        unsafe extern "system" fn ValueType<Impl: IJsonValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut JsonValueType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stringify<Impl: IJsonValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stringify() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: IJsonValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumber<Impl: IJsonValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoolean<Impl: IJsonValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoolean() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArray<Impl: IJsonValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Impl: IJsonValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject() {
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
            ::windows::core::GetRuntimeClassName::<IJsonValue>,
            ::windows::core::GetTrustLevel,
            ValueType::<Impl, IMPL_OFFSET>,
            Stringify::<Impl, IMPL_OFFSET>,
            GetString::<Impl, IMPL_OFFSET>,
            GetNumber::<Impl, IMPL_OFFSET>,
            GetBoolean::<Impl, IMPL_OFFSET>,
            GetArray::<Impl, IMPL_OFFSET>,
            GetObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonValue>) -> ::windows::core::Result<bool>;
    fn CreateBooleanValue(&self, input: bool) -> ::windows::core::Result<JsonValue>;
    fn CreateNumberValue(&self, input: f64) -> ::windows::core::Result<JsonValue>;
    fn CreateStringValue(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonValueStatics {
    const NAME: &'static str = "Windows.Data.Json.IJsonValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonValueStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IJsonValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IJsonValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBooleanValue<Impl: IJsonValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBooleanValue(input) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNumberValue<Impl: IJsonValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNumberValue(input) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStringValue<Impl: IJsonValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStringValue(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IJsonValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, IMPL_OFFSET>, TryParse::<Impl, IMPL_OFFSET>, CreateBooleanValue::<Impl, IMPL_OFFSET>, CreateNumberValue::<Impl, IMPL_OFFSET>, CreateStringValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonValueStatics2Impl: Sized {
    fn CreateNullValue(&self) -> ::windows::core::Result<JsonValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonValueStatics2 {
    const NAME: &'static str = "Windows.Data.Json.IJsonValueStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonValueStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonValueStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonValueStatics2Vtbl {
        unsafe extern "system" fn CreateNullValue<Impl: IJsonValueStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNullValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IJsonValueStatics2>, ::windows::core::GetTrustLevel, CreateNullValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonValueStatics2 as ::windows::core::Interface>::IID
    }
}
