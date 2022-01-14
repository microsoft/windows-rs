#[cfg(feature = "implement_exclusive")]
pub trait IJsonArray_Impl: Sized + IJsonValue_Impl {
    fn GetObjectAt(&mut self, index: u32) -> ::windows::core::Result<JsonObject>;
    fn GetArrayAt(&mut self, index: u32) -> ::windows::core::Result<JsonArray>;
    fn GetStringAt(&mut self, index: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNumberAt(&mut self, index: u32) -> ::windows::core::Result<f64>;
    fn GetBooleanAt(&mut self, index: u32) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonArray {
    const NAME: &'static str = "Windows.Data.Json.IJsonArray";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonArray_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonArray_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonArray_Vtbl {
        unsafe extern "system" fn GetObjectAt<Impl: IJsonArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetArrayAt<Impl: IJsonArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetStringAt<Impl: IJsonArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNumberAt<Impl: IJsonArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBooleanAt<Impl: IJsonArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJsonArray, BASE_OFFSET>(),
            GetObjectAt: GetObjectAt::<Impl, IMPL_OFFSET>,
            GetArrayAt: GetArrayAt::<Impl, IMPL_OFFSET>,
            GetStringAt: GetStringAt::<Impl, IMPL_OFFSET>,
            GetNumberAt: GetNumberAt::<Impl, IMPL_OFFSET>,
            GetBooleanAt: GetBooleanAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonArray as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonArrayStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonArray>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonArray>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonArrayStatics {
    const NAME: &'static str = "Windows.Data.Json.IJsonArrayStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonArrayStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonArrayStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonArrayStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IJsonArrayStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IJsonArrayStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJsonArrayStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonArrayStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonErrorStatics2_Impl: Sized {
    fn GetJsonStatus(&mut self, hresult: i32) -> ::windows::core::Result<JsonErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonErrorStatics2 {
    const NAME: &'static str = "Windows.Data.Json.IJsonErrorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonErrorStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonErrorStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonErrorStatics2_Vtbl {
        unsafe extern "system" fn GetJsonStatus<Impl: IJsonErrorStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut JsonErrorStatus) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IJsonErrorStatics2, BASE_OFFSET>(), GetJsonStatus: GetJsonStatus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonErrorStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonObject_Impl: Sized + IJsonValue_Impl {
    fn GetNamedValue(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue>;
    fn SetNamedValue(&mut self, name: &::windows::core::HSTRING, value: &::core::option::Option<IJsonValue>) -> ::windows::core::Result<()>;
    fn GetNamedObject(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonObject>;
    fn GetNamedArray(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonArray>;
    fn GetNamedString(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNamedNumber(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<f64>;
    fn GetNamedBoolean(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonObject {
    const NAME: &'static str = "Windows.Data.Json.IJsonObject";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonObject_Vtbl {
        unsafe extern "system" fn GetNamedValue<Impl: IJsonObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNamedValue<Impl: IJsonObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamedValue(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <IJsonValue as ::windows::core::Abi>::Abi as *const <IJsonValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetNamedObject<Impl: IJsonObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNamedArray<Impl: IJsonObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNamedString<Impl: IJsonObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNamedNumber<Impl: IJsonObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNamedBoolean<Impl: IJsonObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJsonObject, BASE_OFFSET>(),
            GetNamedValue: GetNamedValue::<Impl, IMPL_OFFSET>,
            SetNamedValue: SetNamedValue::<Impl, IMPL_OFFSET>,
            GetNamedObject: GetNamedObject::<Impl, IMPL_OFFSET>,
            GetNamedArray: GetNamedArray::<Impl, IMPL_OFFSET>,
            GetNamedString: GetNamedString::<Impl, IMPL_OFFSET>,
            GetNamedNumber: GetNamedNumber::<Impl, IMPL_OFFSET>,
            GetNamedBoolean: GetNamedBoolean::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonObjectStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonObject>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonObjectStatics {
    const NAME: &'static str = "Windows.Data.Json.IJsonObjectStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonObjectStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonObjectStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonObjectStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IJsonObjectStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IJsonObjectStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJsonObjectStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonObjectStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonObjectWithDefaultValues_Impl: Sized + IJsonObject_Impl + IJsonValue_Impl {
    fn GetNamedValueOrDefault(&mut self, name: &::windows::core::HSTRING, defaultvalue: &::core::option::Option<JsonValue>) -> ::windows::core::Result<JsonValue>;
    fn GetNamedObjectOrDefault(&mut self, name: &::windows::core::HSTRING, defaultvalue: &::core::option::Option<JsonObject>) -> ::windows::core::Result<JsonObject>;
    fn GetNamedStringOrDefault(&mut self, name: &::windows::core::HSTRING, defaultvalue: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNamedArrayOrDefault(&mut self, name: &::windows::core::HSTRING, defaultvalue: &::core::option::Option<JsonArray>) -> ::windows::core::Result<JsonArray>;
    fn GetNamedNumberOrDefault(&mut self, name: &::windows::core::HSTRING, defaultvalue: f64) -> ::windows::core::Result<f64>;
    fn GetNamedBooleanOrDefault(&mut self, name: &::windows::core::HSTRING, defaultvalue: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonObjectWithDefaultValues {
    const NAME: &'static str = "Windows.Data.Json.IJsonObjectWithDefaultValues";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonObjectWithDefaultValues_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonObjectWithDefaultValues_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonObjectWithDefaultValues_Vtbl {
        unsafe extern "system" fn GetNamedValueOrDefault<Impl: IJsonObjectWithDefaultValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNamedObjectOrDefault<Impl: IJsonObjectWithDefaultValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNamedStringOrDefault<Impl: IJsonObjectWithDefaultValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNamedArrayOrDefault<Impl: IJsonObjectWithDefaultValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNamedNumberOrDefault<Impl: IJsonObjectWithDefaultValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: f64, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNamedBooleanOrDefault<Impl: IJsonObjectWithDefaultValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: bool, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJsonObjectWithDefaultValues, BASE_OFFSET>(),
            GetNamedValueOrDefault: GetNamedValueOrDefault::<Impl, IMPL_OFFSET>,
            GetNamedObjectOrDefault: GetNamedObjectOrDefault::<Impl, IMPL_OFFSET>,
            GetNamedStringOrDefault: GetNamedStringOrDefault::<Impl, IMPL_OFFSET>,
            GetNamedArrayOrDefault: GetNamedArrayOrDefault::<Impl, IMPL_OFFSET>,
            GetNamedNumberOrDefault: GetNamedNumberOrDefault::<Impl, IMPL_OFFSET>,
            GetNamedBooleanOrDefault: GetNamedBooleanOrDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonObjectWithDefaultValues as ::windows::core::Interface>::IID
    }
}
pub trait IJsonValue_Impl: Sized {
    fn ValueType(&mut self) -> ::windows::core::Result<JsonValueType>;
    fn Stringify(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNumber(&mut self) -> ::windows::core::Result<f64>;
    fn GetBoolean(&mut self) -> ::windows::core::Result<bool>;
    fn GetArray(&mut self) -> ::windows::core::Result<JsonArray>;
    fn GetObject(&mut self) -> ::windows::core::Result<JsonObject>;
}
impl ::windows::core::RuntimeName for IJsonValue {
    const NAME: &'static str = "Windows.Data.Json.IJsonValue";
}
impl IJsonValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonValue_Vtbl {
        unsafe extern "system" fn ValueType<Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut JsonValueType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Stringify<Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetString<Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNumber<Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBoolean<Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetArray<Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetObject<Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJsonValue, BASE_OFFSET>(),
            ValueType: ValueType::<Impl, IMPL_OFFSET>,
            Stringify: Stringify::<Impl, IMPL_OFFSET>,
            GetString: GetString::<Impl, IMPL_OFFSET>,
            GetNumber: GetNumber::<Impl, IMPL_OFFSET>,
            GetBoolean: GetBoolean::<Impl, IMPL_OFFSET>,
            GetArray: GetArray::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonValue>) -> ::windows::core::Result<bool>;
    fn CreateBooleanValue(&mut self, input: bool) -> ::windows::core::Result<JsonValue>;
    fn CreateNumberValue(&mut self, input: f64) -> ::windows::core::Result<JsonValue>;
    fn CreateStringValue(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonValueStatics {
    const NAME: &'static str = "Windows.Data.Json.IJsonValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IJsonValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IJsonValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateBooleanValue<Impl: IJsonValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateNumberValue<Impl: IJsonValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateStringValue<Impl: IJsonValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJsonValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
            CreateBooleanValue: CreateBooleanValue::<Impl, IMPL_OFFSET>,
            CreateNumberValue: CreateNumberValue::<Impl, IMPL_OFFSET>,
            CreateStringValue: CreateStringValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonValueStatics2_Impl: Sized {
    fn CreateNullValue(&mut self) -> ::windows::core::Result<JsonValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJsonValueStatics2 {
    const NAME: &'static str = "Windows.Data.Json.IJsonValueStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IJsonValueStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJsonValueStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJsonValueStatics2_Vtbl {
        unsafe extern "system" fn CreateNullValue<Impl: IJsonValueStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJsonValueStatics2, BASE_OFFSET>(),
            CreateNullValue: CreateNullValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJsonValueStatics2 as ::windows::core::Interface>::IID
    }
}
