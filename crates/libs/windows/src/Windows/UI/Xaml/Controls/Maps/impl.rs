#[cfg(feature = "implement_exclusive")]
pub trait ICustomMapTileDataSourceImpl: Sized {
    fn BitmapRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CustomMapTileDataSource, MapTileBitmapRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBitmapRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.ICustomMapTileDataSource";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomMapTileDataSourceVtbl {
    pub const fn new<Impl: ICustomMapTileDataSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomMapTileDataSourceVtbl {
        unsafe extern "system" fn BitmapRequested<Impl: ICustomMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapRequested(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CustomMapTileDataSource, MapTileBitmapRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CustomMapTileDataSource, MapTileBitmapRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBitmapRequested<Impl: ICustomMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBitmapRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomMapTileDataSource>, base.5, BitmapRequested::<Impl, OFFSET>, RemoveBitmapRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomMapTileDataSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CustomMapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomMapTileDataSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.ICustomMapTileDataSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomMapTileDataSourceFactoryVtbl {
    pub const fn new<Impl: ICustomMapTileDataSourceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomMapTileDataSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICustomMapTileDataSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomMapTileDataSourceFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMapTileDataSourceImpl: Sized {
    fn UriFormatString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUriFormatString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AdditionalRequestHeaders(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn AllowCaching(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCaching(&self, value: bool) -> ::windows::core::Result<()>;
    fn UriRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<HttpMapTileDataSource, MapTileUriRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUriRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IHttpMapTileDataSource";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMapTileDataSourceVtbl {
    pub const fn new<Impl: IHttpMapTileDataSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpMapTileDataSourceVtbl {
        unsafe extern "system" fn UriFormatString<Impl: IHttpMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UriFormatString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUriFormatString<Impl: IHttpMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUriFormatString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdditionalRequestHeaders<Impl: IHttpMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdditionalRequestHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowCaching<Impl: IHttpMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowCaching() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowCaching<Impl: IHttpMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowCaching(value).into()
        }
        unsafe extern "system" fn UriRequested<Impl: IHttpMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UriRequested(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<HttpMapTileDataSource, MapTileUriRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<HttpMapTileDataSource, MapTileUriRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUriRequested<Impl: IHttpMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUriRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHttpMapTileDataSource>, base.5, UriFormatString::<Impl, OFFSET>, SetUriFormatString::<Impl, OFFSET>, AdditionalRequestHeaders::<Impl, OFFSET>, AllowCaching::<Impl, OFFSET>, SetAllowCaching::<Impl, OFFSET>, UriRequested::<Impl, OFFSET>, RemoveUriRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMapTileDataSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HttpMapTileDataSource>;
    fn CreateInstanceWithUriFormatString(&self, uriformatstring: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HttpMapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMapTileDataSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IHttpMapTileDataSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMapTileDataSourceFactoryVtbl {
    pub const fn new<Impl: IHttpMapTileDataSourceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpMapTileDataSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IHttpMapTileDataSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithUriFormatString<Impl: IHttpMapTileDataSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uriformatstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithUriFormatString(&*(&uriformatstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHttpMapTileDataSourceFactory>, base.5, CreateInstance::<Impl, OFFSET>, CreateInstanceWithUriFormatString::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalMapTileDataSourceImpl: Sized {
    fn UriFormatString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUriFormatString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UriRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<LocalMapTileDataSource, MapTileUriRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUriRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.ILocalMapTileDataSource";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalMapTileDataSourceVtbl {
    pub const fn new<Impl: ILocalMapTileDataSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILocalMapTileDataSourceVtbl {
        unsafe extern "system" fn UriFormatString<Impl: ILocalMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UriFormatString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUriFormatString<Impl: ILocalMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUriFormatString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UriRequested<Impl: ILocalMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UriRequested(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<LocalMapTileDataSource, MapTileUriRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<LocalMapTileDataSource, MapTileUriRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUriRequested<Impl: ILocalMapTileDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUriRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILocalMapTileDataSource>, base.5, UriFormatString::<Impl, OFFSET>, SetUriFormatString::<Impl, OFFSET>, UriRequested::<Impl, OFFSET>, RemoveUriRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalMapTileDataSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<LocalMapTileDataSource>;
    fn CreateInstanceWithUriFormatString(&self, uriformatstring: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<LocalMapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalMapTileDataSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.ILocalMapTileDataSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalMapTileDataSourceFactoryVtbl {
    pub const fn new<Impl: ILocalMapTileDataSourceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILocalMapTileDataSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ILocalMapTileDataSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithUriFormatString<Impl: ILocalMapTileDataSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uriformatstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithUriFormatString(&*(&uriformatstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILocalMapTileDataSourceFactory>, base.5, CreateInstance::<Impl, OFFSET>, CreateInstanceWithUriFormatString::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangedEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapActualCameraChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapActualCameraChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapActualCameraChangedEventArgsVtbl {
    pub const fn new<Impl: IMapActualCameraChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapActualCameraChangedEventArgsVtbl {
        unsafe extern "system" fn Camera<Impl: IMapActualCameraChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Camera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapActualCameraChangedEventArgs>, base.5, Camera::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangedEventArgs2Impl: Sized {
    fn ChangeReason(&self) -> ::windows::core::Result<MapCameraChangeReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapActualCameraChangedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapActualCameraChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapActualCameraChangedEventArgs2Vtbl {
    pub const fn new<Impl: IMapActualCameraChangedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapActualCameraChangedEventArgs2Vtbl {
        unsafe extern "system" fn ChangeReason<Impl: IMapActualCameraChangedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangeReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapActualCameraChangedEventArgs2>, base.5, ChangeReason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangingEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapActualCameraChangingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapActualCameraChangingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapActualCameraChangingEventArgsVtbl {
    pub const fn new<Impl: IMapActualCameraChangingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapActualCameraChangingEventArgsVtbl {
        unsafe extern "system" fn Camera<Impl: IMapActualCameraChangingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Camera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapActualCameraChangingEventArgs>, base.5, Camera::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangingEventArgs2Impl: Sized {
    fn ChangeReason(&self) -> ::windows::core::Result<MapCameraChangeReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapActualCameraChangingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapActualCameraChangingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapActualCameraChangingEventArgs2Vtbl {
    pub const fn new<Impl: IMapActualCameraChangingEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapActualCameraChangingEventArgs2Vtbl {
        unsafe extern "system" fn ChangeReason<Impl: IMapActualCameraChangingEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangeReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapActualCameraChangingEventArgs2>, base.5, ChangeReason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapBillboardImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn NormalizedAnchorPoint(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetNormalizedAnchorPoint(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Image(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetImage(&self, value: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn CollisionBehaviorDesired(&self) -> ::windows::core::Result<MapElementCollisionBehavior>;
    fn SetCollisionBehaviorDesired(&self, value: MapElementCollisionBehavior) -> ::windows::core::Result<()>;
    fn ReferenceCamera(&self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapBillboard {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapBillboard";
}
#[cfg(feature = "implement_exclusive")]
impl IMapBillboardVtbl {
    pub const fn new<Impl: IMapBillboardImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapBillboardVtbl {
        unsafe extern "system" fn Location<Impl: IMapBillboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IMapBillboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalizedAnchorPoint<Impl: IMapBillboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedAnchorPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedAnchorPoint<Impl: IMapBillboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNormalizedAnchorPoint(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IMapBillboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: IMapBillboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CollisionBehaviorDesired<Impl: IMapBillboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapElementCollisionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CollisionBehaviorDesired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollisionBehaviorDesired<Impl: IMapBillboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapElementCollisionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCollisionBehaviorDesired(value).into()
        }
        unsafe extern "system" fn ReferenceCamera<Impl: IMapBillboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReferenceCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapBillboard>, base.5, Location::<Impl, OFFSET>, SetLocation::<Impl, OFFSET>, NormalizedAnchorPoint::<Impl, OFFSET>, SetNormalizedAnchorPoint::<Impl, OFFSET>, Image::<Impl, OFFSET>, SetImage::<Impl, OFFSET>, CollisionBehaviorDesired::<Impl, OFFSET>, SetCollisionBehaviorDesired::<Impl, OFFSET>, ReferenceCamera::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapBillboardFactoryImpl: Sized {
    fn CreateInstanceFromCamera(&self, camera: &::core::option::Option<MapCamera>) -> ::windows::core::Result<MapBillboard>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapBillboardFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapBillboardFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapBillboardFactoryVtbl {
    pub const fn new<Impl: IMapBillboardFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapBillboardFactoryVtbl {
        unsafe extern "system" fn CreateInstanceFromCamera<Impl: IMapBillboardFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, camera: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceFromCamera(&*(&camera as *const <MapCamera as ::windows::core::Abi>::Abi as *const <MapCamera as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapBillboardFactory>, base.5, CreateInstanceFromCamera::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapBillboardStaticsImpl: Sized {
    fn LocationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn NormalizedAnchorPointProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CollisionBehaviorDesiredProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapBillboardStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapBillboardStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapBillboardStaticsVtbl {
    pub const fn new<Impl: IMapBillboardStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapBillboardStaticsVtbl {
        unsafe extern "system" fn LocationProperty<Impl: IMapBillboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedAnchorPointProperty<Impl: IMapBillboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedAnchorPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollisionBehaviorDesiredProperty<Impl: IMapBillboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CollisionBehaviorDesiredProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapBillboardStatics>, base.5, LocationProperty::<Impl, OFFSET>, NormalizedAnchorPointProperty::<Impl, OFFSET>, CollisionBehaviorDesiredProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCameraImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Heading(&self) -> ::windows::core::Result<f64>;
    fn SetHeading(&self, value: f64) -> ::windows::core::Result<()>;
    fn Pitch(&self) -> ::windows::core::Result<f64>;
    fn SetPitch(&self, value: f64) -> ::windows::core::Result<()>;
    fn Roll(&self) -> ::windows::core::Result<f64>;
    fn SetRoll(&self, value: f64) -> ::windows::core::Result<()>;
    fn FieldOfView(&self) -> ::windows::core::Result<f64>;
    fn SetFieldOfView(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapCamera {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCamera";
}
#[cfg(feature = "implement_exclusive")]
impl IMapCameraVtbl {
    pub const fn new<Impl: IMapCameraImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapCameraVtbl {
        unsafe extern "system" fn Location<Impl: IMapCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IMapCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Heading<Impl: IMapCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Heading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeading<Impl: IMapCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHeading(value).into()
        }
        unsafe extern "system" fn Pitch<Impl: IMapCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPitch<Impl: IMapCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPitch(value).into()
        }
        unsafe extern "system" fn Roll<Impl: IMapCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Roll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoll<Impl: IMapCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoll(value).into()
        }
        unsafe extern "system" fn FieldOfView<Impl: IMapCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FieldOfView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFieldOfView<Impl: IMapCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFieldOfView(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapCamera>, base.5, Location::<Impl, OFFSET>, SetLocation::<Impl, OFFSET>, Heading::<Impl, OFFSET>, SetHeading::<Impl, OFFSET>, Pitch::<Impl, OFFSET>, SetPitch::<Impl, OFFSET>, Roll::<Impl, OFFSET>, SetRoll::<Impl, OFFSET>, FieldOfView::<Impl, OFFSET>, SetFieldOfView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCameraFactoryImpl: Sized {
    fn CreateInstanceWithLocation(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationAndHeading(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationHeadingAndPitch(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationHeadingPitchRollAndFieldOfView(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64, rollindegrees: f64, fieldofviewindegrees: f64) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapCameraFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCameraFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapCameraFactoryVtbl {
    pub const fn new<Impl: IMapCameraFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapCameraFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithLocation<Impl: IMapCameraFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithLocation(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithLocationAndHeading<Impl: IMapCameraFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithLocationAndHeading(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), headingindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithLocationHeadingAndPitch<Impl: IMapCameraFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithLocationHeadingAndPitch(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithLocationHeadingPitchRollAndFieldOfView<Impl: IMapCameraFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, rollindegrees: f64, fieldofviewindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithLocationHeadingPitchRollAndFieldOfView(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees, rollindegrees, fieldofviewindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapCameraFactory>, base.5, CreateInstanceWithLocation::<Impl, OFFSET>, CreateInstanceWithLocationAndHeading::<Impl, OFFSET>, CreateInstanceWithLocationHeadingAndPitch::<Impl, OFFSET>, CreateInstanceWithLocationHeadingPitchRollAndFieldOfView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapContextRequestedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapContextRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapContextRequestedEventArgsVtbl {
    pub const fn new<Impl: IMapContextRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapContextRequestedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapContextRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapContextRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElements<Impl: IMapContextRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapContextRequestedEventArgs>, base.5, Position::<Impl, OFFSET>, Location::<Impl, OFFSET>, MapElements::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlImpl: Sized {
    fn Center(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetCenter(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>>;
    fn ColorScheme(&self) -> ::windows::core::Result<MapColorScheme>;
    fn SetColorScheme(&self, value: MapColorScheme) -> ::windows::core::Result<()>;
    fn DesiredPitch(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredPitch(&self, value: f64) -> ::windows::core::Result<()>;
    fn Heading(&self) -> ::windows::core::Result<f64>;
    fn SetHeading(&self, value: f64) -> ::windows::core::Result<()>;
    fn LandmarksVisible(&self) -> ::windows::core::Result<bool>;
    fn SetLandmarksVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn LoadingStatus(&self) -> ::windows::core::Result<MapLoadingStatus>;
    fn MapServiceToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMapServiceToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MaxZoomLevel(&self) -> ::windows::core::Result<f64>;
    fn MinZoomLevel(&self) -> ::windows::core::Result<f64>;
    fn PedestrianFeaturesVisible(&self) -> ::windows::core::Result<bool>;
    fn SetPedestrianFeaturesVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Pitch(&self) -> ::windows::core::Result<f64>;
    fn Style(&self) -> ::windows::core::Result<MapStyle>;
    fn SetStyle(&self, value: MapStyle) -> ::windows::core::Result<()>;
    fn TrafficFlowVisible(&self) -> ::windows::core::Result<bool>;
    fn SetTrafficFlowVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransformOrigin(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetTransformOrigin(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn WatermarkMode(&self) -> ::windows::core::Result<MapWatermarkMode>;
    fn SetWatermarkMode(&self, value: MapWatermarkMode) -> ::windows::core::Result<()>;
    fn ZoomLevel(&self) -> ::windows::core::Result<f64>;
    fn SetZoomLevel(&self, value: f64) -> ::windows::core::Result<()>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
    fn Routes(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapRouteView>>;
    fn TileSources(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapTileSource>>;
    fn CenterChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCenterChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HeadingChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadingChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LoadingStatusChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoadingStatusChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapDoubleTapped(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapDoubleTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapHolding(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapHolding(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapTapped(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PitchChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePitchChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransformOriginChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransformOriginChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ZoomLevelChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveZoomLevelChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindMapElementsAtOffset(&self, offset: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
    fn GetLocationFromOffset(&self, offset: &super::super::super::super::Foundation::Point, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn GetOffsetFromLocation(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, offset: &mut super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsLocationInView(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, isinview: &mut bool) -> ::windows::core::Result<()>;
    fn TrySetViewBoundsAsync(&self, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, margin: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::Thickness>>, animation: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterAsync(&self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterAndZoomAsync(&self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, zoomlevel: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterZoomHeadingAndPitchAsync(&self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, zoomlevel: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, heading: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, desiredpitch: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync(&self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, zoomlevel: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, heading: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, desiredpitch: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, animation: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlVtbl {
    pub const fn new<Impl: IMapControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlVtbl {
        unsafe extern "system" fn Center<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Children<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorScheme<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapColorScheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorScheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorScheme<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapColorScheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColorScheme(value).into()
        }
        unsafe extern "system" fn DesiredPitch<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredPitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPitch<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredPitch(value).into()
        }
        unsafe extern "system" fn Heading<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Heading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeading<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHeading(value).into()
        }
        unsafe extern "system" fn LandmarksVisible<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LandmarksVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLandmarksVisible<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLandmarksVisible(value).into()
        }
        unsafe extern "system" fn LoadingStatus<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapLoadingStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadingStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapServiceToken<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapServiceToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapServiceToken<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMapServiceToken(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxZoomLevel<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinZoomLevel<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PedestrianFeaturesVisible<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PedestrianFeaturesVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPedestrianFeaturesVisible<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPedestrianFeaturesVisible(value).into()
        }
        unsafe extern "system" fn Pitch<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Style<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Style() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStyle(value).into()
        }
        unsafe extern "system" fn TrafficFlowVisible<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrafficFlowVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrafficFlowVisible<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTrafficFlowVisible(value).into()
        }
        unsafe extern "system" fn TransformOrigin<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformOrigin<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransformOrigin(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WatermarkMode<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapWatermarkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WatermarkMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWatermarkMode<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapWatermarkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWatermarkMode(value).into()
        }
        unsafe extern "system" fn ZoomLevel<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoomLevel<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetZoomLevel(value).into()
        }
        unsafe extern "system" fn MapElements<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Routes<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Routes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileSources<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TileSources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCenterChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCenterChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HeadingChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeadingChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeadingChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHeadingChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoadingStatusChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadingStatusChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoadingStatusChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLoadingStatusChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapDoubleTapped<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapDoubleTapped(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapDoubleTapped<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapDoubleTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapHolding<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapHolding(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapHolding<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapHolding(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapTapped<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapTapped(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapTapped<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PitchChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PitchChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePitchChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePitchChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformOriginChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformOriginChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransformOriginChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTransformOriginChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ZoomLevelChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomLevelChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveZoomLevelChanged<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveZoomLevelChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindMapElementsAtOffset<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindMapElementsAtOffset(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocationFromOffset<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, location: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetLocationFromOffset(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&location)).into()
        }
        unsafe extern "system" fn GetOffsetFromLocation<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, offset: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetOffsetFromLocation(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn IsLocationInView<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, isinview: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).IsLocationInView(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isinview)).into()
        }
        unsafe extern "system" fn TrySetViewBoundsAsync<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, margin: ::windows::core::RawPtr, animation: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetViewBoundsAsync(
                &*(&bounds as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType),
                &*(&margin as *const <super::super::super::super::Foundation::IReference<super::super::Thickness> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::Thickness> as ::windows::core::DefaultType>::DefaultType),
                animation,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetViewWithCenterAsync<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetViewWithCenterAsync(&*(&center as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetViewWithCenterAndZoomAsync<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetViewWithCenterAndZoomAsync(&*(&center as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), &*(&zoomlevel as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetViewWithCenterZoomHeadingAndPitchAsync<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, heading: ::windows::core::RawPtr, desiredpitch: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetViewWithCenterZoomHeadingAndPitchAsync(
                &*(&center as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType),
                &*(&zoomlevel as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType),
                &*(&heading as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType),
                &*(&desiredpitch as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync<Impl: IMapControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, heading: ::windows::core::RawPtr, desiredpitch: ::windows::core::RawPtr, animation: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync(
                &*(&center as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType),
                &*(&zoomlevel as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType),
                &*(&heading as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType),
                &*(&desiredpitch as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType),
                animation,
            ) {
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
            ::windows::core::GetRuntimeClassName::<IMapControl>,
            base.5,
            Center::<Impl, OFFSET>,
            SetCenter::<Impl, OFFSET>,
            Children::<Impl, OFFSET>,
            ColorScheme::<Impl, OFFSET>,
            SetColorScheme::<Impl, OFFSET>,
            DesiredPitch::<Impl, OFFSET>,
            SetDesiredPitch::<Impl, OFFSET>,
            Heading::<Impl, OFFSET>,
            SetHeading::<Impl, OFFSET>,
            LandmarksVisible::<Impl, OFFSET>,
            SetLandmarksVisible::<Impl, OFFSET>,
            LoadingStatus::<Impl, OFFSET>,
            MapServiceToken::<Impl, OFFSET>,
            SetMapServiceToken::<Impl, OFFSET>,
            MaxZoomLevel::<Impl, OFFSET>,
            MinZoomLevel::<Impl, OFFSET>,
            PedestrianFeaturesVisible::<Impl, OFFSET>,
            SetPedestrianFeaturesVisible::<Impl, OFFSET>,
            Pitch::<Impl, OFFSET>,
            Style::<Impl, OFFSET>,
            SetStyle::<Impl, OFFSET>,
            TrafficFlowVisible::<Impl, OFFSET>,
            SetTrafficFlowVisible::<Impl, OFFSET>,
            TransformOrigin::<Impl, OFFSET>,
            SetTransformOrigin::<Impl, OFFSET>,
            WatermarkMode::<Impl, OFFSET>,
            SetWatermarkMode::<Impl, OFFSET>,
            ZoomLevel::<Impl, OFFSET>,
            SetZoomLevel::<Impl, OFFSET>,
            MapElements::<Impl, OFFSET>,
            Routes::<Impl, OFFSET>,
            TileSources::<Impl, OFFSET>,
            CenterChanged::<Impl, OFFSET>,
            RemoveCenterChanged::<Impl, OFFSET>,
            HeadingChanged::<Impl, OFFSET>,
            RemoveHeadingChanged::<Impl, OFFSET>,
            LoadingStatusChanged::<Impl, OFFSET>,
            RemoveLoadingStatusChanged::<Impl, OFFSET>,
            MapDoubleTapped::<Impl, OFFSET>,
            RemoveMapDoubleTapped::<Impl, OFFSET>,
            MapHolding::<Impl, OFFSET>,
            RemoveMapHolding::<Impl, OFFSET>,
            MapTapped::<Impl, OFFSET>,
            RemoveMapTapped::<Impl, OFFSET>,
            PitchChanged::<Impl, OFFSET>,
            RemovePitchChanged::<Impl, OFFSET>,
            TransformOriginChanged::<Impl, OFFSET>,
            RemoveTransformOriginChanged::<Impl, OFFSET>,
            ZoomLevelChanged::<Impl, OFFSET>,
            RemoveZoomLevelChanged::<Impl, OFFSET>,
            FindMapElementsAtOffset::<Impl, OFFSET>,
            GetLocationFromOffset::<Impl, OFFSET>,
            GetOffsetFromLocation::<Impl, OFFSET>,
            IsLocationInView::<Impl, OFFSET>,
            TrySetViewBoundsAsync::<Impl, OFFSET>,
            TrySetViewWithCenterAsync::<Impl, OFFSET>,
            TrySetViewWithCenterAndZoomAsync::<Impl, OFFSET>,
            TrySetViewWithCenterZoomHeadingAndPitchAsync::<Impl, OFFSET>,
            TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl2Impl: Sized {
    fn BusinessLandmarksVisible(&self) -> ::windows::core::Result<bool>;
    fn SetBusinessLandmarksVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransitFeaturesVisible(&self) -> ::windows::core::Result<bool>;
    fn SetTransitFeaturesVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn PanInteractionMode(&self) -> ::windows::core::Result<MapPanInteractionMode>;
    fn SetPanInteractionMode(&self, value: MapPanInteractionMode) -> ::windows::core::Result<()>;
    fn RotateInteractionMode(&self) -> ::windows::core::Result<MapInteractionMode>;
    fn SetRotateInteractionMode(&self, value: MapInteractionMode) -> ::windows::core::Result<()>;
    fn TiltInteractionMode(&self) -> ::windows::core::Result<MapInteractionMode>;
    fn SetTiltInteractionMode(&self, value: MapInteractionMode) -> ::windows::core::Result<()>;
    fn ZoomInteractionMode(&self) -> ::windows::core::Result<MapInteractionMode>;
    fn SetZoomInteractionMode(&self, value: MapInteractionMode) -> ::windows::core::Result<()>;
    fn Is3DSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStreetsideSupported(&self) -> ::windows::core::Result<bool>;
    fn Scene(&self) -> ::windows::core::Result<MapScene>;
    fn SetScene(&self, value: &::core::option::Option<MapScene>) -> ::windows::core::Result<()>;
    fn ActualCamera(&self) -> ::windows::core::Result<MapCamera>;
    fn TargetCamera(&self) -> ::windows::core::Result<MapCamera>;
    fn CustomExperience(&self) -> ::windows::core::Result<MapCustomExperience>;
    fn SetCustomExperience(&self, value: &::core::option::Option<MapCustomExperience>) -> ::windows::core::Result<()>;
    fn MapElementClick(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementClick(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerEntered(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerEntered(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerExited(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerExited(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActualCameraChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualCameraChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActualCameraChanging(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangingEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualCameraChanging(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TargetCameraChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapTargetCameraChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetCameraChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CustomExperienceChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapCustomExperienceChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCustomExperienceChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartContinuousRotate(&self, rateindegreespersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousRotate(&self) -> ::windows::core::Result<()>;
    fn StartContinuousTilt(&self, rateindegreespersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousTilt(&self) -> ::windows::core::Result<()>;
    fn StartContinuousZoom(&self, rateofchangepersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousZoom(&self) -> ::windows::core::Result<()>;
    fn TryRotateAsync(&self, degrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryRotateToAsync(&self, angleindegrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryTiltAsync(&self, degrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryTiltToAsync(&self, angleindegrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryZoomInAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryZoomOutAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryZoomToAsync(&self, zoomlevel: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetSceneAsync(&self, scene: &::core::option::Option<MapScene>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetSceneWithAnimationAsync(&self, scene: &::core::option::Option<MapScene>, animationkind: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControl2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControl2Vtbl {
    pub const fn new<Impl: IMapControl2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControl2Vtbl {
        unsafe extern "system" fn BusinessLandmarksVisible<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarksVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusinessLandmarksVisible<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBusinessLandmarksVisible(value).into()
        }
        unsafe extern "system" fn TransitFeaturesVisible<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitFeaturesVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransitFeaturesVisible<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransitFeaturesVisible(value).into()
        }
        unsafe extern "system" fn PanInteractionMode<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapPanInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PanInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPanInteractionMode<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapPanInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPanInteractionMode(value).into()
        }
        unsafe extern "system" fn RotateInteractionMode<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotateInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotateInteractionMode<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotateInteractionMode(value).into()
        }
        unsafe extern "system" fn TiltInteractionMode<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TiltInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTiltInteractionMode<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTiltInteractionMode(value).into()
        }
        unsafe extern "system" fn ZoomInteractionMode<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoomInteractionMode<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetZoomInteractionMode(value).into()
        }
        unsafe extern "system" fn Is3DSupported<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Is3DSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStreetsideSupported<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStreetsideSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scene<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scene() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScene<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScene(&*(&value as *const <MapScene as ::windows::core::Abi>::Abi as *const <MapScene as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualCamera<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetCamera<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomExperience<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomExperience() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomExperience<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCustomExperience(&*(&value as *const <MapCustomExperience as ::windows::core::Abi>::Abi as *const <MapCustomExperience as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementClick<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElementClick(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementClickEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementClickEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementClick<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapElementClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerEntered<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElementPointerEntered(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerEnteredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerEnteredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementPointerEntered<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerExited<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElementPointerExited(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerExitedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerExitedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementPointerExited<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualCameraChanged<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualCameraChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActualCameraChanged<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveActualCameraChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualCameraChanging<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualCameraChanging(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActualCameraChanging<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveActualCameraChanging(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetCameraChanged<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetCameraChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapTargetCameraChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapTargetCameraChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTargetCameraChanged<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTargetCameraChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CustomExperienceChanged<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomExperienceChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapCustomExperienceChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapCustomExperienceChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCustomExperienceChanged<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCustomExperienceChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartContinuousRotate<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rateindegreespersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartContinuousRotate(rateindegreespersecond).into()
        }
        unsafe extern "system" fn StopContinuousRotate<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopContinuousRotate().into()
        }
        unsafe extern "system" fn StartContinuousTilt<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rateindegreespersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartContinuousTilt(rateindegreespersecond).into()
        }
        unsafe extern "system" fn StopContinuousTilt<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopContinuousTilt().into()
        }
        unsafe extern "system" fn StartContinuousZoom<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rateofchangepersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartContinuousZoom(rateofchangepersecond).into()
        }
        unsafe extern "system" fn StopContinuousZoom<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopContinuousZoom().into()
        }
        unsafe extern "system" fn TryRotateAsync<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, degrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryRotateAsync(degrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRotateToAsync<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, angleindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryRotateToAsync(angleindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTiltAsync<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, degrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryTiltAsync(degrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTiltToAsync<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, angleindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryTiltToAsync(angleindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryZoomInAsync<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryZoomInAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryZoomOutAsync<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryZoomOutAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryZoomToAsync<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, zoomlevel: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryZoomToAsync(zoomlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetSceneAsync<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scene: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetSceneAsync(&*(&scene as *const <MapScene as ::windows::core::Abi>::Abi as *const <MapScene as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetSceneWithAnimationAsync<Impl: IMapControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scene: ::windows::core::RawPtr, animationkind: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetSceneWithAnimationAsync(&*(&scene as *const <MapScene as ::windows::core::Abi>::Abi as *const <MapScene as ::windows::core::DefaultType>::DefaultType), animationkind) {
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
            ::windows::core::GetRuntimeClassName::<IMapControl2>,
            base.5,
            BusinessLandmarksVisible::<Impl, OFFSET>,
            SetBusinessLandmarksVisible::<Impl, OFFSET>,
            TransitFeaturesVisible::<Impl, OFFSET>,
            SetTransitFeaturesVisible::<Impl, OFFSET>,
            PanInteractionMode::<Impl, OFFSET>,
            SetPanInteractionMode::<Impl, OFFSET>,
            RotateInteractionMode::<Impl, OFFSET>,
            SetRotateInteractionMode::<Impl, OFFSET>,
            TiltInteractionMode::<Impl, OFFSET>,
            SetTiltInteractionMode::<Impl, OFFSET>,
            ZoomInteractionMode::<Impl, OFFSET>,
            SetZoomInteractionMode::<Impl, OFFSET>,
            Is3DSupported::<Impl, OFFSET>,
            IsStreetsideSupported::<Impl, OFFSET>,
            Scene::<Impl, OFFSET>,
            SetScene::<Impl, OFFSET>,
            ActualCamera::<Impl, OFFSET>,
            TargetCamera::<Impl, OFFSET>,
            CustomExperience::<Impl, OFFSET>,
            SetCustomExperience::<Impl, OFFSET>,
            MapElementClick::<Impl, OFFSET>,
            RemoveMapElementClick::<Impl, OFFSET>,
            MapElementPointerEntered::<Impl, OFFSET>,
            RemoveMapElementPointerEntered::<Impl, OFFSET>,
            MapElementPointerExited::<Impl, OFFSET>,
            RemoveMapElementPointerExited::<Impl, OFFSET>,
            ActualCameraChanged::<Impl, OFFSET>,
            RemoveActualCameraChanged::<Impl, OFFSET>,
            ActualCameraChanging::<Impl, OFFSET>,
            RemoveActualCameraChanging::<Impl, OFFSET>,
            TargetCameraChanged::<Impl, OFFSET>,
            RemoveTargetCameraChanged::<Impl, OFFSET>,
            CustomExperienceChanged::<Impl, OFFSET>,
            RemoveCustomExperienceChanged::<Impl, OFFSET>,
            StartContinuousRotate::<Impl, OFFSET>,
            StopContinuousRotate::<Impl, OFFSET>,
            StartContinuousTilt::<Impl, OFFSET>,
            StopContinuousTilt::<Impl, OFFSET>,
            StartContinuousZoom::<Impl, OFFSET>,
            StopContinuousZoom::<Impl, OFFSET>,
            TryRotateAsync::<Impl, OFFSET>,
            TryRotateToAsync::<Impl, OFFSET>,
            TryTiltAsync::<Impl, OFFSET>,
            TryTiltToAsync::<Impl, OFFSET>,
            TryZoomInAsync::<Impl, OFFSET>,
            TryZoomOutAsync::<Impl, OFFSET>,
            TryZoomToAsync::<Impl, OFFSET>,
            TrySetSceneAsync::<Impl, OFFSET>,
            TrySetSceneWithAnimationAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl3Impl: Sized {
    fn MapRightTapped(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapRightTappedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapRightTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControl3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl3";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControl3Vtbl {
    pub const fn new<Impl: IMapControl3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControl3Vtbl {
        unsafe extern "system" fn MapRightTapped<Impl: IMapControl3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapRightTapped(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapRightTappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapRightTappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapRightTapped<Impl: IMapControl3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapRightTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControl3>, base.5, MapRightTapped::<Impl, OFFSET>, RemoveMapRightTapped::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl4Impl: Sized {
    fn BusinessLandmarksEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetBusinessLandmarksEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransitFeaturesEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetTransitFeaturesEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetVisibleRegion(&self, region: MapVisibleRegionKind) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControl4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl4";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControl4Vtbl {
    pub const fn new<Impl: IMapControl4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControl4Vtbl {
        unsafe extern "system" fn BusinessLandmarksEnabled<Impl: IMapControl4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarksEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusinessLandmarksEnabled<Impl: IMapControl4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBusinessLandmarksEnabled(value).into()
        }
        unsafe extern "system" fn TransitFeaturesEnabled<Impl: IMapControl4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitFeaturesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransitFeaturesEnabled<Impl: IMapControl4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransitFeaturesEnabled(value).into()
        }
        unsafe extern "system" fn GetVisibleRegion<Impl: IMapControl4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, region: MapVisibleRegionKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisibleRegion(region) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControl4>, base.5, BusinessLandmarksEnabled::<Impl, OFFSET>, SetBusinessLandmarksEnabled::<Impl, OFFSET>, TransitFeaturesEnabled::<Impl, OFFSET>, SetTransitFeaturesEnabled::<Impl, OFFSET>, GetVisibleRegion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl5Impl: Sized {
    fn MapProjection(&self) -> ::windows::core::Result<MapProjection>;
    fn SetMapProjection(&self, value: MapProjection) -> ::windows::core::Result<()>;
    fn StyleSheet(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn SetStyleSheet(&self, value: &::core::option::Option<MapStyleSheet>) -> ::windows::core::Result<()>;
    fn ViewPadding(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetViewPadding(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn MapContextRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapContextRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapContextRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindMapElementsAtOffsetWithRadius(&self, offset: &super::super::super::super::Foundation::Point, radius: f64) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
    fn GetLocationFromOffsetWithReferenceSystem(&self, offset: &super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn StartContinuousPan(&self, horizontalpixelspersecond: f64, verticalpixelspersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousPan(&self) -> ::windows::core::Result<()>;
    fn TryPanAsync(&self, horizontalpixels: f64, verticalpixels: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryPanToAsync(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControl5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl5";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControl5Vtbl {
    pub const fn new<Impl: IMapControl5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControl5Vtbl {
        unsafe extern "system" fn MapProjection<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapProjection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapProjection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapProjection<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapProjection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMapProjection(value).into()
        }
        unsafe extern "system" fn StyleSheet<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StyleSheet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyleSheet<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStyleSheet(&*(&value as *const <MapStyleSheet as ::windows::core::Abi>::Abi as *const <MapStyleSheet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ViewPadding<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewPadding<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetViewPadding(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapContextRequested<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapContextRequested(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapContextRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapContextRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapContextRequested<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapContextRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindMapElementsAtOffsetWithRadius<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindMapElementsAtOffsetWithRadius(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocationFromOffsetWithReferenceSystem<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetLocationFromOffsetWithReferenceSystem(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), desiredreferencesystem, ::core::mem::transmute_copy(&location)).into()
        }
        unsafe extern "system" fn StartContinuousPan<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizontalpixelspersecond: f64, verticalpixelspersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartContinuousPan(horizontalpixelspersecond, verticalpixelspersecond).into()
        }
        unsafe extern "system" fn StopContinuousPan<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopContinuousPan().into()
        }
        unsafe extern "system" fn TryPanAsync<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizontalpixels: f64, verticalpixels: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryPanAsync(horizontalpixels, verticalpixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryPanToAsync<Impl: IMapControl5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryPanToAsync(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IMapControl5>,
            base.5,
            MapProjection::<Impl, OFFSET>,
            SetMapProjection::<Impl, OFFSET>,
            StyleSheet::<Impl, OFFSET>,
            SetStyleSheet::<Impl, OFFSET>,
            ViewPadding::<Impl, OFFSET>,
            SetViewPadding::<Impl, OFFSET>,
            MapContextRequested::<Impl, OFFSET>,
            RemoveMapContextRequested::<Impl, OFFSET>,
            FindMapElementsAtOffsetWithRadius::<Impl, OFFSET>,
            GetLocationFromOffsetWithReferenceSystem::<Impl, OFFSET>,
            StartContinuousPan::<Impl, OFFSET>,
            StopContinuousPan::<Impl, OFFSET>,
            TryPanAsync::<Impl, OFFSET>,
            TryPanToAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl6Impl: Sized {
    fn Layers(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapLayer>>;
    fn SetLayers(&self, value: &::core::option::Option<super::super::super::super::Foundation::Collections::IVector<MapLayer>>) -> ::windows::core::Result<()>;
    fn TryGetLocationFromOffset(&self, offset: &super::super::super::super::Foundation::Point, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<bool>;
    fn TryGetLocationFromOffsetWithReferenceSystem(&self, offset: &super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControl6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl6";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControl6Vtbl {
    pub const fn new<Impl: IMapControl6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControl6Vtbl {
        unsafe extern "system" fn Layers<Impl: IMapControl6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Layers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLayers<Impl: IMapControl6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLayers(&*(&value as *const <super::super::super::super::Foundation::Collections::IVector<MapLayer> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVector<MapLayer> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryGetLocationFromOffset<Impl: IMapControl6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, location: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetLocationFromOffset(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetLocationFromOffsetWithReferenceSystem<Impl: IMapControl6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetLocationFromOffsetWithReferenceSystem(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), desiredreferencesystem, ::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControl6>, base.5, Layers::<Impl, OFFSET>, SetLayers::<Impl, OFFSET>, TryGetLocationFromOffset::<Impl, OFFSET>, TryGetLocationFromOffsetWithReferenceSystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl7Impl: Sized {
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRegion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControl7 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl7";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControl7Vtbl {
    pub const fn new<Impl: IMapControl7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControl7Vtbl {
        unsafe extern "system" fn Region<Impl: IMapControl7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Region() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegion<Impl: IMapControl7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRegion(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControl7>, base.5, Region::<Impl, OFFSET>, SetRegion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl8Impl: Sized {
    fn CanTiltDown(&self) -> ::windows::core::Result<bool>;
    fn CanTiltUp(&self) -> ::windows::core::Result<bool>;
    fn CanZoomIn(&self) -> ::windows::core::Result<bool>;
    fn CanZoomOut(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControl8 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl8";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControl8Vtbl {
    pub const fn new<Impl: IMapControl8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControl8Vtbl {
        unsafe extern "system" fn CanTiltDown<Impl: IMapControl8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanTiltDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanTiltUp<Impl: IMapControl8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanTiltUp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanZoomIn<Impl: IMapControl8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanZoomIn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanZoomOut<Impl: IMapControl8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanZoomOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControl8>, base.5, CanTiltDown::<Impl, OFFSET>, CanTiltUp::<Impl, OFFSET>, CanZoomIn::<Impl, OFFSET>, CanZoomOut::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlBusinessLandmarkClickEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkClickEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlBusinessLandmarkClickEventArgsVtbl {
    pub const fn new<Impl: IMapControlBusinessLandmarkClickEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlBusinessLandmarkClickEventArgsVtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkClickEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlBusinessLandmarkClickEventArgs>, base.5, LocalLocations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlBusinessLandmarkPointerEnteredEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkPointerEnteredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlBusinessLandmarkPointerEnteredEventArgsVtbl {
    pub const fn new<Impl: IMapControlBusinessLandmarkPointerEnteredEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlBusinessLandmarkPointerEnteredEventArgsVtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkPointerEnteredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlBusinessLandmarkPointerEnteredEventArgs>, base.5, LocalLocations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlBusinessLandmarkPointerExitedEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkPointerExitedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlBusinessLandmarkPointerExitedEventArgsVtbl {
    pub const fn new<Impl: IMapControlBusinessLandmarkPointerExitedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlBusinessLandmarkPointerExitedEventArgsVtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkPointerExitedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlBusinessLandmarkPointerExitedEventArgs>, base.5, LocalLocations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlBusinessLandmarkRightTappedEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkRightTappedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlBusinessLandmarkRightTappedEventArgsVtbl {
    pub const fn new<Impl: IMapControlBusinessLandmarkRightTappedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlBusinessLandmarkRightTappedEventArgsVtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkRightTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlBusinessLandmarkRightTappedEventArgs>, base.5, LocalLocations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlDataHelperImpl: Sized {
    fn BusinessLandmarkClick(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkClick(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeatureClick(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeatureClick(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BusinessLandmarkRightTapped(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkRightTappedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkRightTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeatureRightTapped(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureRightTappedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeatureRightTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlDataHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlDataHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlDataHelperVtbl {
    pub const fn new<Impl: IMapControlDataHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlDataHelperVtbl {
        unsafe extern "system" fn BusinessLandmarkClick<Impl: IMapControlDataHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarkClick(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkClickEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkClickEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBusinessLandmarkClick<Impl: IMapControlDataHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeatureClick<Impl: IMapControlDataHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitFeatureClick(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureClickEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureClickEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransitFeatureClick<Impl: IMapControlDataHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeatureClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BusinessLandmarkRightTapped<Impl: IMapControlDataHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarkRightTapped(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkRightTappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkRightTappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBusinessLandmarkRightTapped<Impl: IMapControlDataHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkRightTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeatureRightTapped<Impl: IMapControlDataHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitFeatureRightTapped(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureRightTappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureRightTappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransitFeatureRightTapped<Impl: IMapControlDataHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeatureRightTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlDataHelper>, base.5, BusinessLandmarkClick::<Impl, OFFSET>, RemoveBusinessLandmarkClick::<Impl, OFFSET>, TransitFeatureClick::<Impl, OFFSET>, RemoveTransitFeatureClick::<Impl, OFFSET>, BusinessLandmarkRightTapped::<Impl, OFFSET>, RemoveBusinessLandmarkRightTapped::<Impl, OFFSET>, TransitFeatureRightTapped::<Impl, OFFSET>, RemoveTransitFeatureRightTapped::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlDataHelper2Impl: Sized {
    fn BusinessLandmarkPointerEntered(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkPointerEntered(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeaturePointerEntered(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeaturePointerEntered(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BusinessLandmarkPointerExited(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkPointerExited(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeaturePointerExited(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeaturePointerExited(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlDataHelper2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlDataHelper2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlDataHelper2Vtbl {
    pub const fn new<Impl: IMapControlDataHelper2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlDataHelper2Vtbl {
        unsafe extern "system" fn BusinessLandmarkPointerEntered<Impl: IMapControlDataHelper2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarkPointerEntered(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerEnteredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerEnteredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBusinessLandmarkPointerEntered<Impl: IMapControlDataHelper2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkPointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeaturePointerEntered<Impl: IMapControlDataHelper2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitFeaturePointerEntered(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerEnteredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerEnteredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransitFeaturePointerEntered<Impl: IMapControlDataHelper2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeaturePointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BusinessLandmarkPointerExited<Impl: IMapControlDataHelper2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarkPointerExited(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerExitedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerExitedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBusinessLandmarkPointerExited<Impl: IMapControlDataHelper2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkPointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeaturePointerExited<Impl: IMapControlDataHelper2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitFeaturePointerExited(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerExitedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerExitedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransitFeaturePointerExited<Impl: IMapControlDataHelper2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeaturePointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMapControlDataHelper2>,
            base.5,
            BusinessLandmarkPointerEntered::<Impl, OFFSET>,
            RemoveBusinessLandmarkPointerEntered::<Impl, OFFSET>,
            TransitFeaturePointerEntered::<Impl, OFFSET>,
            RemoveTransitFeaturePointerEntered::<Impl, OFFSET>,
            BusinessLandmarkPointerExited::<Impl, OFFSET>,
            RemoveBusinessLandmarkPointerExited::<Impl, OFFSET>,
            TransitFeaturePointerExited::<Impl, OFFSET>,
            RemoveTransitFeaturePointerExited::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlDataHelperFactoryImpl: Sized {
    fn CreateInstance(&self, map: &::core::option::Option<MapControl>) -> ::windows::core::Result<MapControlDataHelper>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlDataHelperFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlDataHelperFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlDataHelperFactoryVtbl {
    pub const fn new<Impl: IMapControlDataHelperFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlDataHelperFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapControlDataHelperFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, map: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&map as *const <MapControl as ::windows::core::Abi>::Abi as *const <MapControl as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlDataHelperFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlDataHelperStaticsImpl: Sized {
    fn CreateMapControl(&self, rasterrendermode: bool) -> ::windows::core::Result<MapControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlDataHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlDataHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlDataHelperStaticsVtbl {
    pub const fn new<Impl: IMapControlDataHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlDataHelperStaticsVtbl {
        unsafe extern "system" fn CreateMapControl<Impl: IMapControlDataHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rasterrendermode: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMapControl(rasterrendermode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlDataHelperStatics>, base.5, CreateMapControl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStaticsImpl: Sized {
    fn CenterProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ChildrenProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ColorSchemeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DesiredPitchProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HeadingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LandmarksVisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LoadingStatusProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MapServiceTokenProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PedestrianFeaturesVisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PitchProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StyleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TrafficFlowVisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TransformOriginProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn WatermarkModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZoomLevelProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MapElementsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RoutesProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TileSourcesProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LocationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetLocation(&self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&self, element: &::core::option::Option<super::super::DependencyObject>, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn NormalizedAnchorPointProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetNormalizedAnchorPoint(&self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetNormalizedAnchorPoint(&self, element: &::core::option::Option<super::super::DependencyObject>, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStaticsVtbl {
    pub const fn new<Impl: IMapControlStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlStaticsVtbl {
        unsafe extern "system" fn CenterProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildrenProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChildrenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorSchemeProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorSchemeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredPitchProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredPitchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeadingProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeadingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LandmarksVisibleProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LandmarksVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadingStatusProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadingStatusProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapServiceTokenProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapServiceTokenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PedestrianFeaturesVisibleProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PedestrianFeaturesVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PitchProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PitchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StyleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrafficFlowVisibleProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrafficFlowVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformOriginProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformOriginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WatermarkModeProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WatermarkModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevelProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomLevelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElementsProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElementsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutesProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoutesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileSourcesProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TileSourcesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocation<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLocation(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalizedAnchorPointProperty<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedAnchorPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNormalizedAnchorPoint<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNormalizedAnchorPoint(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedAnchorPoint<Impl: IMapControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNormalizedAnchorPoint(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMapControlStatics>,
            base.5,
            CenterProperty::<Impl, OFFSET>,
            ChildrenProperty::<Impl, OFFSET>,
            ColorSchemeProperty::<Impl, OFFSET>,
            DesiredPitchProperty::<Impl, OFFSET>,
            HeadingProperty::<Impl, OFFSET>,
            LandmarksVisibleProperty::<Impl, OFFSET>,
            LoadingStatusProperty::<Impl, OFFSET>,
            MapServiceTokenProperty::<Impl, OFFSET>,
            PedestrianFeaturesVisibleProperty::<Impl, OFFSET>,
            PitchProperty::<Impl, OFFSET>,
            StyleProperty::<Impl, OFFSET>,
            TrafficFlowVisibleProperty::<Impl, OFFSET>,
            TransformOriginProperty::<Impl, OFFSET>,
            WatermarkModeProperty::<Impl, OFFSET>,
            ZoomLevelProperty::<Impl, OFFSET>,
            MapElementsProperty::<Impl, OFFSET>,
            RoutesProperty::<Impl, OFFSET>,
            TileSourcesProperty::<Impl, OFFSET>,
            LocationProperty::<Impl, OFFSET>,
            GetLocation::<Impl, OFFSET>,
            SetLocation::<Impl, OFFSET>,
            NormalizedAnchorPointProperty::<Impl, OFFSET>,
            GetNormalizedAnchorPoint::<Impl, OFFSET>,
            SetNormalizedAnchorPoint::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics2Impl: Sized {
    fn BusinessLandmarksVisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TransitFeaturesVisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PanInteractionModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RotateInteractionModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TiltInteractionModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZoomInteractionModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn Is3DSupportedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsStreetsideSupportedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SceneProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics2Vtbl {
    pub const fn new<Impl: IMapControlStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlStatics2Vtbl {
        unsafe extern "system" fn BusinessLandmarksVisibleProperty<Impl: IMapControlStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarksVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitFeaturesVisibleProperty<Impl: IMapControlStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitFeaturesVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PanInteractionModeProperty<Impl: IMapControlStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PanInteractionModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotateInteractionModeProperty<Impl: IMapControlStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotateInteractionModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TiltInteractionModeProperty<Impl: IMapControlStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TiltInteractionModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomInteractionModeProperty<Impl: IMapControlStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomInteractionModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is3DSupportedProperty<Impl: IMapControlStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Is3DSupportedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStreetsideSupportedProperty<Impl: IMapControlStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStreetsideSupportedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SceneProperty<Impl: IMapControlStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SceneProperty() {
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
            ::windows::core::GetRuntimeClassName::<IMapControlStatics2>,
            base.5,
            BusinessLandmarksVisibleProperty::<Impl, OFFSET>,
            TransitFeaturesVisibleProperty::<Impl, OFFSET>,
            PanInteractionModeProperty::<Impl, OFFSET>,
            RotateInteractionModeProperty::<Impl, OFFSET>,
            TiltInteractionModeProperty::<Impl, OFFSET>,
            ZoomInteractionModeProperty::<Impl, OFFSET>,
            Is3DSupportedProperty::<Impl, OFFSET>,
            IsStreetsideSupportedProperty::<Impl, OFFSET>,
            SceneProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics4Impl: Sized {
    fn BusinessLandmarksEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TransitFeaturesEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics4Vtbl {
    pub const fn new<Impl: IMapControlStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlStatics4Vtbl {
        unsafe extern "system" fn BusinessLandmarksEnabledProperty<Impl: IMapControlStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarksEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitFeaturesEnabledProperty<Impl: IMapControlStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitFeaturesEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlStatics4>, base.5, BusinessLandmarksEnabledProperty::<Impl, OFFSET>, TransitFeaturesEnabledProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics5Impl: Sized {
    fn MapProjectionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StyleSheetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ViewPaddingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics5Vtbl {
    pub const fn new<Impl: IMapControlStatics5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlStatics5Vtbl {
        unsafe extern "system" fn MapProjectionProperty<Impl: IMapControlStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapProjectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleSheetProperty<Impl: IMapControlStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StyleSheetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewPaddingProperty<Impl: IMapControlStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewPaddingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlStatics5>, base.5, MapProjectionProperty::<Impl, OFFSET>, StyleSheetProperty::<Impl, OFFSET>, ViewPaddingProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics6Impl: Sized {
    fn LayersProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics6Vtbl {
    pub const fn new<Impl: IMapControlStatics6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlStatics6Vtbl {
        unsafe extern "system" fn LayersProperty<Impl: IMapControlStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LayersProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlStatics6>, base.5, LayersProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics7Impl: Sized {
    fn RegionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics7";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics7Vtbl {
    pub const fn new<Impl: IMapControlStatics7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlStatics7Vtbl {
        unsafe extern "system" fn RegionProperty<Impl: IMapControlStatics7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlStatics7>, base.5, RegionProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics8Impl: Sized {
    fn CanTiltDownProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CanTiltUpProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CanZoomInProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CanZoomOutProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics8 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics8";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics8Vtbl {
    pub const fn new<Impl: IMapControlStatics8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlStatics8Vtbl {
        unsafe extern "system" fn CanTiltDownProperty<Impl: IMapControlStatics8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanTiltDownProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanTiltUpProperty<Impl: IMapControlStatics8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanTiltUpProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanZoomInProperty<Impl: IMapControlStatics8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanZoomInProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanZoomOutProperty<Impl: IMapControlStatics8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanZoomOutProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlStatics8>, base.5, CanTiltDownProperty::<Impl, OFFSET>, CanTiltUpProperty::<Impl, OFFSET>, CanZoomInProperty::<Impl, OFFSET>, CanZoomOutProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlTransitFeatureClickEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlTransitFeatureClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeatureClickEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlTransitFeatureClickEventArgsVtbl {
    pub const fn new<Impl: IMapControlTransitFeatureClickEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlTransitFeatureClickEventArgsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeatureClickEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeatureClickEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeatureClickEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlTransitFeatureClickEventArgs>, base.5, DisplayName::<Impl, OFFSET>, Location::<Impl, OFFSET>, TransitProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlTransitFeaturePointerEnteredEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlTransitFeaturePointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeaturePointerEnteredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlTransitFeaturePointerEnteredEventArgsVtbl {
    pub const fn new<Impl: IMapControlTransitFeaturePointerEnteredEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlTransitFeaturePointerEnteredEventArgsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeaturePointerEnteredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeaturePointerEnteredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeaturePointerEnteredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlTransitFeaturePointerEnteredEventArgs>, base.5, DisplayName::<Impl, OFFSET>, Location::<Impl, OFFSET>, TransitProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlTransitFeaturePointerExitedEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlTransitFeaturePointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeaturePointerExitedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlTransitFeaturePointerExitedEventArgsVtbl {
    pub const fn new<Impl: IMapControlTransitFeaturePointerExitedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlTransitFeaturePointerExitedEventArgsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeaturePointerExitedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeaturePointerExitedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeaturePointerExitedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlTransitFeaturePointerExitedEventArgs>, base.5, DisplayName::<Impl, OFFSET>, Location::<Impl, OFFSET>, TransitProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlTransitFeatureRightTappedEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlTransitFeatureRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeatureRightTappedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlTransitFeatureRightTappedEventArgsVtbl {
    pub const fn new<Impl: IMapControlTransitFeatureRightTappedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlTransitFeatureRightTappedEventArgsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeatureRightTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeatureRightTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeatureRightTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlTransitFeatureRightTappedEventArgs>, base.5, DisplayName::<Impl, OFFSET>, Location::<Impl, OFFSET>, TransitProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCustomExperienceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapCustomExperience {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCustomExperience";
}
#[cfg(feature = "implement_exclusive")]
impl IMapCustomExperienceVtbl {
    pub const fn new<Impl: IMapCustomExperienceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapCustomExperienceVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapCustomExperience>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCustomExperienceChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapCustomExperienceChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCustomExperienceChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapCustomExperienceChangedEventArgsVtbl {
    pub const fn new<Impl: IMapCustomExperienceChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapCustomExperienceChangedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapCustomExperienceChangedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCustomExperienceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapCustomExperience>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapCustomExperienceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCustomExperienceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapCustomExperienceFactoryVtbl {
    pub const fn new<Impl: IMapCustomExperienceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapCustomExperienceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapCustomExperienceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapCustomExperienceFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementImpl: Sized {
    fn ZIndex(&self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn SetVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementVtbl {
    pub const fn new<Impl: IMapElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementVtbl {
        unsafe extern "system" fn ZIndex<Impl: IMapElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Impl: IMapElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        unsafe extern "system" fn Visible<Impl: IMapElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: IMapElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElement>, base.5, ZIndex::<Impl, OFFSET>, SetZIndex::<Impl, OFFSET>, Visible::<Impl, OFFSET>, SetVisible::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement2Impl: Sized {
    fn MapTabIndex(&self) -> ::windows::core::Result<i32>;
    fn SetMapTabIndex(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElement2Vtbl {
    pub const fn new<Impl: IMapElement2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElement2Vtbl {
        unsafe extern "system" fn MapTabIndex<Impl: IMapElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapTabIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapTabIndex<Impl: IMapElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMapTabIndex(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElement2>, base.5, MapTabIndex::<Impl, OFFSET>, SetMapTabIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement3Impl: Sized {
    fn MapStyleSheetEntry(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMapStyleSheetEntry(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MapStyleSheetEntryState(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMapStyleSheetEntryState(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement3";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElement3Vtbl {
    pub const fn new<Impl: IMapElement3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElement3Vtbl {
        unsafe extern "system" fn MapStyleSheetEntry<Impl: IMapElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapStyleSheetEntry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapStyleSheetEntry<Impl: IMapElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMapStyleSheetEntry(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapStyleSheetEntryState<Impl: IMapElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapStyleSheetEntryState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapStyleSheetEntryState<Impl: IMapElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMapStyleSheetEntryState(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IMapElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IMapElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElement3>, base.5, MapStyleSheetEntry::<Impl, OFFSET>, SetMapStyleSheetEntry::<Impl, OFFSET>, MapStyleSheetEntryState::<Impl, OFFSET>, SetMapStyleSheetEntryState::<Impl, OFFSET>, Tag::<Impl, OFFSET>, SetTag::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement3DImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Model(&self) -> ::windows::core::Result<MapModel3D>;
    fn SetModel(&self, value: &::core::option::Option<MapModel3D>) -> ::windows::core::Result<()>;
    fn Heading(&self) -> ::windows::core::Result<f64>;
    fn SetHeading(&self, value: f64) -> ::windows::core::Result<()>;
    fn Pitch(&self) -> ::windows::core::Result<f64>;
    fn SetPitch(&self, value: f64) -> ::windows::core::Result<()>;
    fn Roll(&self) -> ::windows::core::Result<f64>;
    fn SetRoll(&self, value: f64) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&self, value: &super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement3D {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement3D";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElement3DVtbl {
    pub const fn new<Impl: IMapElement3DImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElement3DVtbl {
        unsafe extern "system" fn Location<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Model<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Model() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModel<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetModel(&*(&value as *const <MapModel3D as ::windows::core::Abi>::Abi as *const <MapModel3D as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Heading<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Heading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeading<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHeading(value).into()
        }
        unsafe extern "system" fn Pitch<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPitch<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPitch(value).into()
        }
        unsafe extern "system" fn Roll<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Roll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoll<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoll(value).into()
        }
        unsafe extern "system" fn Scale<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: IMapElement3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElement3D>, base.5, Location::<Impl, OFFSET>, SetLocation::<Impl, OFFSET>, Model::<Impl, OFFSET>, SetModel::<Impl, OFFSET>, Heading::<Impl, OFFSET>, SetHeading::<Impl, OFFSET>, Pitch::<Impl, OFFSET>, SetPitch::<Impl, OFFSET>, Roll::<Impl, OFFSET>, SetRoll::<Impl, OFFSET>, Scale::<Impl, OFFSET>, SetScale::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement3DStaticsImpl: Sized {
    fn LocationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HeadingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PitchProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RollProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ScaleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement3DStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement3DStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElement3DStaticsVtbl {
    pub const fn new<Impl: IMapElement3DStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElement3DStaticsVtbl {
        unsafe extern "system" fn LocationProperty<Impl: IMapElement3DStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeadingProperty<Impl: IMapElement3DStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeadingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PitchProperty<Impl: IMapElement3DStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PitchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RollProperty<Impl: IMapElement3DStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RollProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleProperty<Impl: IMapElement3DStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElement3DStatics>, base.5, LocationProperty::<Impl, OFFSET>, HeadingProperty::<Impl, OFFSET>, PitchProperty::<Impl, OFFSET>, RollProperty::<Impl, OFFSET>, ScaleProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement4Impl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement4";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElement4Vtbl {
    pub const fn new<Impl: IMapElement4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElement4Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMapElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IMapElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElement4>, base.5, IsEnabled::<Impl, OFFSET>, SetIsEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementClickEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementClickEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementClickEventArgsVtbl {
    pub const fn new<Impl: IMapElementClickEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementClickEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementClickEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementClickEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElements<Impl: IMapElementClickEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementClickEventArgs>, base.5, Position::<Impl, OFFSET>, Location::<Impl, OFFSET>, MapElements::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementFactoryVtbl {
    pub const fn new<Impl: IMapElementFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapElementFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementPointerEnteredEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementPointerEnteredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementPointerEnteredEventArgsVtbl {
    pub const fn new<Impl: IMapElementPointerEnteredEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementPointerEnteredEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementPointerEnteredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementPointerEnteredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElement<Impl: IMapElementPointerEnteredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementPointerEnteredEventArgs>, base.5, Position::<Impl, OFFSET>, Location::<Impl, OFFSET>, MapElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementPointerExitedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementPointerExitedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementPointerExitedEventArgsVtbl {
    pub const fn new<Impl: IMapElementPointerExitedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementPointerExitedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementPointerExitedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementPointerExitedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElement<Impl: IMapElementPointerExitedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementPointerExitedEventArgs>, base.5, Position::<Impl, OFFSET>, Location::<Impl, OFFSET>, MapElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStaticsImpl: Sized {
    fn ZIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementStaticsVtbl {
    pub const fn new<Impl: IMapElementStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementStaticsVtbl {
        unsafe extern "system" fn ZIndexProperty<Impl: IMapElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibleProperty<Impl: IMapElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementStatics>, base.5, ZIndexProperty::<Impl, OFFSET>, VisibleProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStatics2Impl: Sized {
    fn MapTabIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementStatics2Vtbl {
    pub const fn new<Impl: IMapElementStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementStatics2Vtbl {
        unsafe extern "system" fn MapTabIndexProperty<Impl: IMapElementStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapTabIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementStatics2>, base.5, MapTabIndexProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStatics3Impl: Sized {
    fn MapStyleSheetEntryProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MapStyleSheetEntryStateProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TagProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementStatics3Vtbl {
    pub const fn new<Impl: IMapElementStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementStatics3Vtbl {
        unsafe extern "system" fn MapStyleSheetEntryProperty<Impl: IMapElementStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapStyleSheetEntryProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapStyleSheetEntryStateProperty<Impl: IMapElementStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapStyleSheetEntryStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TagProperty<Impl: IMapElementStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TagProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementStatics3>, base.5, MapStyleSheetEntryProperty::<Impl, OFFSET>, MapStyleSheetEntryStateProperty::<Impl, OFFSET>, TagProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStatics4Impl: Sized {
    fn IsEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementStatics4Vtbl {
    pub const fn new<Impl: IMapElementStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementStatics4Vtbl {
        unsafe extern "system" fn IsEnabledProperty<Impl: IMapElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementStatics4>, base.5, IsEnabledProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerImpl: Sized {
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
    fn SetMapElements(&self, value: &::core::option::Option<super::super::super::super::Foundation::Collections::IVector<MapElement>>) -> ::windows::core::Result<()>;
    fn MapElementClick(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementClick(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerEntered(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerEntered(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerExited(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerExited(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapContextRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerContextRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapContextRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementsLayer {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayer";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementsLayerVtbl {
    pub const fn new<Impl: IMapElementsLayerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementsLayerVtbl {
        unsafe extern "system" fn MapElements<Impl: IMapElementsLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapElements<Impl: IMapElementsLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMapElements(&*(&value as *const <super::super::super::super::Foundation::Collections::IVector<MapElement> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVector<MapElement> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementClick<Impl: IMapElementsLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElementClick(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerClickEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerClickEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementClick<Impl: IMapElementsLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapElementClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerEntered<Impl: IMapElementsLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElementPointerEntered(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerEnteredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerEnteredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementPointerEntered<Impl: IMapElementsLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerExited<Impl: IMapElementsLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElementPointerExited(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerExitedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerExitedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementPointerExited<Impl: IMapElementsLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapContextRequested<Impl: IMapElementsLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapContextRequested(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerContextRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerContextRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapContextRequested<Impl: IMapElementsLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMapContextRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMapElementsLayer>,
            base.5,
            MapElements::<Impl, OFFSET>,
            SetMapElements::<Impl, OFFSET>,
            MapElementClick::<Impl, OFFSET>,
            RemoveMapElementClick::<Impl, OFFSET>,
            MapElementPointerEntered::<Impl, OFFSET>,
            RemoveMapElementPointerEntered::<Impl, OFFSET>,
            MapElementPointerExited::<Impl, OFFSET>,
            RemoveMapElementPointerExited::<Impl, OFFSET>,
            MapContextRequested::<Impl, OFFSET>,
            RemoveMapContextRequested::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerClickEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementsLayerClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerClickEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementsLayerClickEventArgsVtbl {
    pub const fn new<Impl: IMapElementsLayerClickEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementsLayerClickEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerClickEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementsLayerClickEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElements<Impl: IMapElementsLayerClickEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementsLayerClickEventArgs>, base.5, Position::<Impl, OFFSET>, Location::<Impl, OFFSET>, MapElements::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerContextRequestedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementsLayerContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerContextRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementsLayerContextRequestedEventArgsVtbl {
    pub const fn new<Impl: IMapElementsLayerContextRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementsLayerContextRequestedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerContextRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementsLayerContextRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElements<Impl: IMapElementsLayerContextRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementsLayerContextRequestedEventArgs>, base.5, Position::<Impl, OFFSET>, Location::<Impl, OFFSET>, MapElements::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerPointerEnteredEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementsLayerPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerPointerEnteredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementsLayerPointerEnteredEventArgsVtbl {
    pub const fn new<Impl: IMapElementsLayerPointerEnteredEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementsLayerPointerEnteredEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerPointerEnteredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementsLayerPointerEnteredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElement<Impl: IMapElementsLayerPointerEnteredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementsLayerPointerEnteredEventArgs>, base.5, Position::<Impl, OFFSET>, Location::<Impl, OFFSET>, MapElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerPointerExitedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementsLayerPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerPointerExitedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementsLayerPointerExitedEventArgsVtbl {
    pub const fn new<Impl: IMapElementsLayerPointerExitedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementsLayerPointerExitedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerPointerExitedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementsLayerPointerExitedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElement<Impl: IMapElementsLayerPointerExitedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementsLayerPointerExitedEventArgs>, base.5, Position::<Impl, OFFSET>, Location::<Impl, OFFSET>, MapElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerStaticsImpl: Sized {
    fn MapElementsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementsLayerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementsLayerStaticsVtbl {
    pub const fn new<Impl: IMapElementsLayerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapElementsLayerStaticsVtbl {
        unsafe extern "system" fn MapElementsProperty<Impl: IMapElementsLayerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapElementsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapElementsLayerStatics>, base.5, MapElementsProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIconImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NormalizedAnchorPoint(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetNormalizedAnchorPoint(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Image(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetImage(&self, value: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapIcon {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapIcon";
}
#[cfg(feature = "implement_exclusive")]
impl IMapIconVtbl {
    pub const fn new<Impl: IMapIconImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapIconVtbl {
        unsafe extern "system" fn Location<Impl: IMapIconImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IMapIconImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Title<Impl: IMapIconImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IMapIconImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalizedAnchorPoint<Impl: IMapIconImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedAnchorPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedAnchorPoint<Impl: IMapIconImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNormalizedAnchorPoint(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IMapIconImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: IMapIconImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapIcon>, base.5, Location::<Impl, OFFSET>, SetLocation::<Impl, OFFSET>, Title::<Impl, OFFSET>, SetTitle::<Impl, OFFSET>, NormalizedAnchorPoint::<Impl, OFFSET>, SetNormalizedAnchorPoint::<Impl, OFFSET>, Image::<Impl, OFFSET>, SetImage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIcon2Impl: Sized {
    fn CollisionBehaviorDesired(&self) -> ::windows::core::Result<MapElementCollisionBehavior>;
    fn SetCollisionBehaviorDesired(&self, value: MapElementCollisionBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapIcon2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapIcon2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapIcon2Vtbl {
    pub const fn new<Impl: IMapIcon2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapIcon2Vtbl {
        unsafe extern "system" fn CollisionBehaviorDesired<Impl: IMapIcon2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapElementCollisionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CollisionBehaviorDesired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollisionBehaviorDesired<Impl: IMapIcon2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapElementCollisionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCollisionBehaviorDesired(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapIcon2>, base.5, CollisionBehaviorDesired::<Impl, OFFSET>, SetCollisionBehaviorDesired::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIconStaticsImpl: Sized {
    fn LocationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TitleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn NormalizedAnchorPointProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapIconStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapIconStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapIconStaticsVtbl {
    pub const fn new<Impl: IMapIconStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapIconStaticsVtbl {
        unsafe extern "system" fn LocationProperty<Impl: IMapIconStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleProperty<Impl: IMapIconStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TitleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedAnchorPointProperty<Impl: IMapIconStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedAnchorPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapIconStatics>, base.5, LocationProperty::<Impl, OFFSET>, TitleProperty::<Impl, OFFSET>, NormalizedAnchorPointProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIconStatics2Impl: Sized {
    fn CollisionBehaviorDesiredProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapIconStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapIconStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapIconStatics2Vtbl {
    pub const fn new<Impl: IMapIconStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapIconStatics2Vtbl {
        unsafe extern "system" fn CollisionBehaviorDesiredProperty<Impl: IMapIconStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CollisionBehaviorDesiredProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapIconStatics2>, base.5, CollisionBehaviorDesiredProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapInputEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapInputEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapInputEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapInputEventArgsVtbl {
    pub const fn new<Impl: IMapInputEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapInputEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapInputEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapInputEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapInputEventArgs>, base.5, Position::<Impl, OFFSET>, Location::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapItemsControlImpl: Sized {
    fn ItemsSource(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetItemsSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>>;
    fn ItemTemplate(&self) -> ::windows::core::Result<super::super::DataTemplate>;
    fn SetItemTemplate(&self, value: &::core::option::Option<super::super::DataTemplate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapItemsControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapItemsControl";
}
#[cfg(feature = "implement_exclusive")]
impl IMapItemsControlVtbl {
    pub const fn new<Impl: IMapItemsControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapItemsControlVtbl {
        unsafe extern "system" fn ItemsSource<Impl: IMapItemsControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemsSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemsSource<Impl: IMapItemsControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetItemsSource(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Items<Impl: IMapItemsControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemTemplate<Impl: IMapItemsControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemTemplate<Impl: IMapItemsControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetItemTemplate(&*(&value as *const <super::super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::super::DataTemplate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapItemsControl>, base.5, ItemsSource::<Impl, OFFSET>, SetItemsSource::<Impl, OFFSET>, Items::<Impl, OFFSET>, ItemTemplate::<Impl, OFFSET>, SetItemTemplate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapItemsControlStaticsImpl: Sized {
    fn ItemsSourceProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemTemplateProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapItemsControlStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapItemsControlStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapItemsControlStaticsVtbl {
    pub const fn new<Impl: IMapItemsControlStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapItemsControlStaticsVtbl {
        unsafe extern "system" fn ItemsSourceProperty<Impl: IMapItemsControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemsSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemsProperty<Impl: IMapItemsControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemTemplateProperty<Impl: IMapItemsControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemTemplateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapItemsControlStatics>, base.5, ItemsSourceProperty::<Impl, OFFSET>, ItemsProperty::<Impl, OFFSET>, ItemTemplateProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLayerImpl: Sized {
    fn MapTabIndex(&self) -> ::windows::core::Result<i32>;
    fn SetMapTabIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn SetVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn ZIndex(&self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapLayer {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapLayer";
}
#[cfg(feature = "implement_exclusive")]
impl IMapLayerVtbl {
    pub const fn new<Impl: IMapLayerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapLayerVtbl {
        unsafe extern "system" fn MapTabIndex<Impl: IMapLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapTabIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapTabIndex<Impl: IMapLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMapTabIndex(value).into()
        }
        unsafe extern "system" fn Visible<Impl: IMapLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: IMapLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        unsafe extern "system" fn ZIndex<Impl: IMapLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Impl: IMapLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapLayer>, base.5, MapTabIndex::<Impl, OFFSET>, SetMapTabIndex::<Impl, OFFSET>, Visible::<Impl, OFFSET>, SetVisible::<Impl, OFFSET>, ZIndex::<Impl, OFFSET>, SetZIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLayerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapLayer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapLayerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapLayerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapLayerFactoryVtbl {
    pub const fn new<Impl: IMapLayerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapLayerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapLayerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapLayerFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLayerStaticsImpl: Sized {
    fn MapTabIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapLayerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapLayerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapLayerStaticsVtbl {
    pub const fn new<Impl: IMapLayerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapLayerStaticsVtbl {
        unsafe extern "system" fn MapTabIndexProperty<Impl: IMapLayerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapTabIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibleProperty<Impl: IMapLayerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZIndexProperty<Impl: IMapLayerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapLayerStatics>, base.5, MapTabIndexProperty::<Impl, OFFSET>, VisibleProperty::<Impl, OFFSET>, ZIndexProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapModel3DImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapModel3D {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapModel3D";
}
#[cfg(feature = "implement_exclusive")]
impl IMapModel3DVtbl {
    pub const fn new<Impl: IMapModel3DImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapModel3DVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapModel3D>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapModel3DFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapModel3D>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapModel3DFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapModel3DFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapModel3DFactoryVtbl {
    pub const fn new<Impl: IMapModel3DFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapModel3DFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapModel3DFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapModel3DFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapModel3DStaticsImpl: Sized {
    fn CreateFrom3MFAsync(&self, source: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>>;
    fn CreateFrom3MFWithShadingOptionAsync(&self, source: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>, shadingoption: MapModel3DShadingOption) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapModel3DStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapModel3DStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapModel3DStaticsVtbl {
    pub const fn new<Impl: IMapModel3DStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapModel3DStaticsVtbl {
        unsafe extern "system" fn CreateFrom3MFAsync<Impl: IMapModel3DStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrom3MFAsync(&*(&source as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrom3MFWithShadingOptionAsync<Impl: IMapModel3DStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, shadingoption: MapModel3DShadingOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrom3MFWithShadingOptionAsync(&*(&source as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType), shadingoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapModel3DStatics>, base.5, CreateFrom3MFAsync::<Impl, OFFSET>, CreateFrom3MFWithShadingOptionAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolygonImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath>;
    fn SetPath(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopath>) -> ::windows::core::Result<()>;
    fn StrokeColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetStrokeColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn StrokeThickness(&self) -> ::windows::core::Result<f64>;
    fn SetStrokeThickness(&self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeDashed(&self) -> ::windows::core::Result<bool>;
    fn SetStrokeDashed(&self, value: bool) -> ::windows::core::Result<()>;
    fn FillColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetFillColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapPolygon {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolygon";
}
#[cfg(feature = "implement_exclusive")]
impl IMapPolygonVtbl {
    pub const fn new<Impl: IMapPolygonImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapPolygonVtbl {
        unsafe extern "system" fn Path<Impl: IMapPolygonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IMapPolygonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeColor<Impl: IMapPolygonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StrokeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeColor<Impl: IMapPolygonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStrokeColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeThickness<Impl: IMapPolygonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StrokeThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Impl: IMapPolygonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStrokeThickness(value).into()
        }
        unsafe extern "system" fn StrokeDashed<Impl: IMapPolygonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StrokeDashed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashed<Impl: IMapPolygonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStrokeDashed(value).into()
        }
        unsafe extern "system" fn FillColor<Impl: IMapPolygonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillColor<Impl: IMapPolygonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFillColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapPolygon>, base.5, Path::<Impl, OFFSET>, SetPath::<Impl, OFFSET>, StrokeColor::<Impl, OFFSET>, SetStrokeColor::<Impl, OFFSET>, StrokeThickness::<Impl, OFFSET>, SetStrokeThickness::<Impl, OFFSET>, StrokeDashed::<Impl, OFFSET>, SetStrokeDashed::<Impl, OFFSET>, FillColor::<Impl, OFFSET>, SetFillColor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolygon2Impl: Sized {
    fn Paths(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Devices::Geolocation::Geopath>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapPolygon2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolygon2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapPolygon2Vtbl {
    pub const fn new<Impl: IMapPolygon2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapPolygon2Vtbl {
        unsafe extern "system" fn Paths<Impl: IMapPolygon2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Paths() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapPolygon2>, base.5, Paths::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolygonStaticsImpl: Sized {
    fn PathProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StrokeThicknessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StrokeDashedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapPolygonStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolygonStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapPolygonStaticsVtbl {
    pub const fn new<Impl: IMapPolygonStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapPolygonStaticsVtbl {
        unsafe extern "system" fn PathProperty<Impl: IMapPolygonStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PathProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeThicknessProperty<Impl: IMapPolygonStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StrokeThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeDashedProperty<Impl: IMapPolygonStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StrokeDashedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapPolygonStatics>, base.5, PathProperty::<Impl, OFFSET>, StrokeThicknessProperty::<Impl, OFFSET>, StrokeDashedProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolylineImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath>;
    fn SetPath(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopath>) -> ::windows::core::Result<()>;
    fn StrokeColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetStrokeColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn StrokeThickness(&self) -> ::windows::core::Result<f64>;
    fn SetStrokeThickness(&self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeDashed(&self) -> ::windows::core::Result<bool>;
    fn SetStrokeDashed(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapPolyline {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolyline";
}
#[cfg(feature = "implement_exclusive")]
impl IMapPolylineVtbl {
    pub const fn new<Impl: IMapPolylineImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapPolylineVtbl {
        unsafe extern "system" fn Path<Impl: IMapPolylineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IMapPolylineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeColor<Impl: IMapPolylineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StrokeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeColor<Impl: IMapPolylineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStrokeColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeThickness<Impl: IMapPolylineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StrokeThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Impl: IMapPolylineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStrokeThickness(value).into()
        }
        unsafe extern "system" fn StrokeDashed<Impl: IMapPolylineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StrokeDashed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashed<Impl: IMapPolylineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStrokeDashed(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapPolyline>, base.5, Path::<Impl, OFFSET>, SetPath::<Impl, OFFSET>, StrokeColor::<Impl, OFFSET>, SetStrokeColor::<Impl, OFFSET>, StrokeThickness::<Impl, OFFSET>, SetStrokeThickness::<Impl, OFFSET>, StrokeDashed::<Impl, OFFSET>, SetStrokeDashed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolylineStaticsImpl: Sized {
    fn PathProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StrokeDashedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapPolylineStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolylineStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapPolylineStaticsVtbl {
    pub const fn new<Impl: IMapPolylineStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapPolylineStaticsVtbl {
        unsafe extern "system" fn PathProperty<Impl: IMapPolylineStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PathProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeDashedProperty<Impl: IMapPolylineStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StrokeDashedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapPolylineStatics>, base.5, PathProperty::<Impl, OFFSET>, StrokeDashedProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRightTappedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapRightTappedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRightTappedEventArgsVtbl {
    pub const fn new<Impl: IMapRightTappedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRightTappedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapRightTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapRightTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRightTappedEventArgs>, base.5, Position::<Impl, OFFSET>, Location::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteViewImpl: Sized {
    fn RouteColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetRouteColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn OutlineColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetOutlineColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn Route(&self) -> ::windows::core::Result<super::super::super::super::Services::Maps::MapRoute>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteView {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapRouteView";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteViewVtbl {
    pub const fn new<Impl: IMapRouteViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteViewVtbl {
        unsafe extern "system" fn RouteColor<Impl: IMapRouteViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RouteColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRouteColor<Impl: IMapRouteViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRouteColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutlineColor<Impl: IMapRouteViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutlineColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutlineColor<Impl: IMapRouteViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOutlineColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Route<Impl: IMapRouteViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Route() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteView>, base.5, RouteColor::<Impl, OFFSET>, SetRouteColor::<Impl, OFFSET>, OutlineColor::<Impl, OFFSET>, SetOutlineColor::<Impl, OFFSET>, Route::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteViewFactoryImpl: Sized {
    fn CreateInstanceWithMapRoute(&self, route: &::core::option::Option<super::super::super::super::Services::Maps::MapRoute>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapRouteView>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteViewFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapRouteViewFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteViewFactoryVtbl {
    pub const fn new<Impl: IMapRouteViewFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteViewFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithMapRoute<Impl: IMapRouteViewFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, route: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithMapRoute(&*(&route as *const <super::super::super::super::Services::Maps::MapRoute as ::windows::core::Abi>::Abi as *const <super::super::super::super::Services::Maps::MapRoute as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteViewFactory>, base.5, CreateInstanceWithMapRoute::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapSceneImpl: Sized {
    fn TargetCamera(&self) -> ::windows::core::Result<MapCamera>;
    fn TargetCameraChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapScene, MapTargetCameraChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetCameraChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapScene {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapScene";
}
#[cfg(feature = "implement_exclusive")]
impl IMapSceneVtbl {
    pub const fn new<Impl: IMapSceneImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapSceneVtbl {
        unsafe extern "system" fn TargetCamera<Impl: IMapSceneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetCameraChanged<Impl: IMapSceneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetCameraChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapScene, MapTargetCameraChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapScene, MapTargetCameraChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTargetCameraChanged<Impl: IMapSceneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTargetCameraChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapScene>, base.5, TargetCamera::<Impl, OFFSET>, TargetCameraChanged::<Impl, OFFSET>, RemoveTargetCameraChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapSceneStaticsImpl: Sized {
    fn CreateFromBoundingBox(&self, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>) -> ::windows::core::Result<MapScene>;
    fn CreateFromBoundingBoxWithHeadingAndPitch(&self, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromCamera(&self, camera: &::core::option::Option<MapCamera>) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocation(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationWithHeadingAndPitch(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationAndRadius(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, radiusinmeters: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationAndRadiusWithHeadingAndPitch(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, radiusinmeters: f64, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocations(&self, locations: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint>>) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationsWithHeadingAndPitch(&self, locations: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint>>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapSceneStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapSceneStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapSceneStaticsVtbl {
    pub const fn new<Impl: IMapSceneStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapSceneStaticsVtbl {
        unsafe extern "system" fn CreateFromBoundingBox<Impl: IMapSceneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromBoundingBox(&*(&bounds as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBoundingBoxWithHeadingAndPitch<Impl: IMapSceneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromBoundingBoxWithHeadingAndPitch(&*(&bounds as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromCamera<Impl: IMapSceneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, camera: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromCamera(&*(&camera as *const <MapCamera as ::windows::core::Abi>::Abi as *const <MapCamera as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocation<Impl: IMapSceneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromLocation(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocationWithHeadingAndPitch<Impl: IMapSceneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromLocationWithHeadingAndPitch(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocationAndRadius<Impl: IMapSceneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromLocationAndRadius(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), radiusinmeters) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocationAndRadiusWithHeadingAndPitch<Impl: IMapSceneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromLocationAndRadiusWithHeadingAndPitch(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), radiusinmeters, headingindegrees, pitchindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocations<Impl: IMapSceneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, locations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromLocations(&*(&locations as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocationsWithHeadingAndPitch<Impl: IMapSceneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, locations: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromLocationsWithHeadingAndPitch(&*(&locations as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint> as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees) {
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
            ::windows::core::GetRuntimeClassName::<IMapSceneStatics>,
            base.5,
            CreateFromBoundingBox::<Impl, OFFSET>,
            CreateFromBoundingBoxWithHeadingAndPitch::<Impl, OFFSET>,
            CreateFromCamera::<Impl, OFFSET>,
            CreateFromLocation::<Impl, OFFSET>,
            CreateFromLocationWithHeadingAndPitch::<Impl, OFFSET>,
            CreateFromLocationAndRadius::<Impl, OFFSET>,
            CreateFromLocationAndRadiusWithHeadingAndPitch::<Impl, OFFSET>,
            CreateFromLocations::<Impl, OFFSET>,
            CreateFromLocationsWithHeadingAndPitch::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheetImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapStyleSheet {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapStyleSheet";
}
#[cfg(feature = "implement_exclusive")]
impl IMapStyleSheetVtbl {
    pub const fn new<Impl: IMapStyleSheetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapStyleSheetVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapStyleSheet>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheetEntriesStaticsImpl: Sized {
    fn Area(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Airport(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Cemetery(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Continent(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Education(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IndigenousPeoplesReserve(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Island(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Medical(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Military(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Nautical(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Neighborhood(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Runway(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sand(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShoppingCenter(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Stadium(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Vegetation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Forest(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GolfCourse(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Park(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PlayingField(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Reserve(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Point(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NaturalPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Peak(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VolcanicPeak(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WaterPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PointOfInterest(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Business(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FoodPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PopulatedPlace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capital(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdminDistrictCapital(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CountryRegionCapital(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoadShield(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoadExit(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transit(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Political(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CountryRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdminDistrict(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn District(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Structure(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Building(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EducationBuilding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MedicalBuilding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransitBuilding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transportation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Road(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ControlledAccessHighway(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HighSpeedRamp(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Highway(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MajorRoad(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ArterialRoad(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Street(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ramp(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UnpavedStreet(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TollRoad(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Railway(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Trail(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WaterRoute(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Water(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn River(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RouteLine(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WalkingRoute(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DrivingRoute(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapStyleSheetEntriesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapStyleSheetEntriesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapStyleSheetEntriesStaticsVtbl {
    pub const fn new<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapStyleSheetEntriesStaticsVtbl {
        unsafe extern "system" fn Area<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Area() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Airport<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Airport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cemetery<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cemetery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Continent<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Continent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Education<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Education() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndigenousPeoplesReserve<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IndigenousPeoplesReserve() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Island<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Island() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Medical<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Medical() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Military<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Military() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Nautical<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Nautical() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Neighborhood<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Neighborhood() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Runway<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Runway() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sand<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Sand() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShoppingCenter<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShoppingCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stadium<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stadium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vegetation<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Vegetation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forest<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Forest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GolfCourse<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GolfCourse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Park<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Park() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlayingField<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlayingField() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserve<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reserve() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Point<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalPoint<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NaturalPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peak<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Peak() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolcanicPeak<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VolcanicPeak() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaterPoint<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaterPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointOfInterest<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointOfInterest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Business<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Business() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FoodPoint<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FoodPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopulatedPlace<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PopulatedPlace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capital<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capital() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminDistrictCapital<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdminDistrictCapital() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryRegionCapital<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CountryRegionCapital() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadShield<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoadShield() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadExit<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoadExit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transit<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Political<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Political() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryRegion<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CountryRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminDistrict<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdminDistrict() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn District<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).District() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Structure<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Structure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Building<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Building() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EducationBuilding<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EducationBuilding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MedicalBuilding<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MedicalBuilding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitBuilding<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitBuilding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transportation<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transportation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Road<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Road() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlledAccessHighway<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControlledAccessHighway() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighSpeedRamp<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HighSpeedRamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Highway<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Highway() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorRoad<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MajorRoad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArterialRoad<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ArterialRoad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Street<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Street() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ramp<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ramp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnpavedStreet<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnpavedStreet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TollRoad<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TollRoad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Railway<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Railway() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trail<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Trail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaterRoute<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaterRoute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Water<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Water() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn River<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).River() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RouteLine<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RouteLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WalkingRoute<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WalkingRoute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrivingRoute<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrivingRoute() {
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
            ::windows::core::GetRuntimeClassName::<IMapStyleSheetEntriesStatics>,
            base.5,
            Area::<Impl, OFFSET>,
            Airport::<Impl, OFFSET>,
            Cemetery::<Impl, OFFSET>,
            Continent::<Impl, OFFSET>,
            Education::<Impl, OFFSET>,
            IndigenousPeoplesReserve::<Impl, OFFSET>,
            Island::<Impl, OFFSET>,
            Medical::<Impl, OFFSET>,
            Military::<Impl, OFFSET>,
            Nautical::<Impl, OFFSET>,
            Neighborhood::<Impl, OFFSET>,
            Runway::<Impl, OFFSET>,
            Sand::<Impl, OFFSET>,
            ShoppingCenter::<Impl, OFFSET>,
            Stadium::<Impl, OFFSET>,
            Vegetation::<Impl, OFFSET>,
            Forest::<Impl, OFFSET>,
            GolfCourse::<Impl, OFFSET>,
            Park::<Impl, OFFSET>,
            PlayingField::<Impl, OFFSET>,
            Reserve::<Impl, OFFSET>,
            Point::<Impl, OFFSET>,
            NaturalPoint::<Impl, OFFSET>,
            Peak::<Impl, OFFSET>,
            VolcanicPeak::<Impl, OFFSET>,
            WaterPoint::<Impl, OFFSET>,
            PointOfInterest::<Impl, OFFSET>,
            Business::<Impl, OFFSET>,
            FoodPoint::<Impl, OFFSET>,
            PopulatedPlace::<Impl, OFFSET>,
            Capital::<Impl, OFFSET>,
            AdminDistrictCapital::<Impl, OFFSET>,
            CountryRegionCapital::<Impl, OFFSET>,
            RoadShield::<Impl, OFFSET>,
            RoadExit::<Impl, OFFSET>,
            Transit::<Impl, OFFSET>,
            Political::<Impl, OFFSET>,
            CountryRegion::<Impl, OFFSET>,
            AdminDistrict::<Impl, OFFSET>,
            District::<Impl, OFFSET>,
            Structure::<Impl, OFFSET>,
            Building::<Impl, OFFSET>,
            EducationBuilding::<Impl, OFFSET>,
            MedicalBuilding::<Impl, OFFSET>,
            TransitBuilding::<Impl, OFFSET>,
            Transportation::<Impl, OFFSET>,
            Road::<Impl, OFFSET>,
            ControlledAccessHighway::<Impl, OFFSET>,
            HighSpeedRamp::<Impl, OFFSET>,
            Highway::<Impl, OFFSET>,
            MajorRoad::<Impl, OFFSET>,
            ArterialRoad::<Impl, OFFSET>,
            Street::<Impl, OFFSET>,
            Ramp::<Impl, OFFSET>,
            UnpavedStreet::<Impl, OFFSET>,
            TollRoad::<Impl, OFFSET>,
            Railway::<Impl, OFFSET>,
            Trail::<Impl, OFFSET>,
            WaterRoute::<Impl, OFFSET>,
            Water::<Impl, OFFSET>,
            River::<Impl, OFFSET>,
            RouteLine::<Impl, OFFSET>,
            WalkingRoute::<Impl, OFFSET>,
            DrivingRoute::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheetEntryStatesStaticsImpl: Sized {
    fn Disabled(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hover(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Selected(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapStyleSheetEntryStatesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapStyleSheetEntryStatesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapStyleSheetEntryStatesStaticsVtbl {
    pub const fn new<Impl: IMapStyleSheetEntryStatesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapStyleSheetEntryStatesStaticsVtbl {
        unsafe extern "system" fn Disabled<Impl: IMapStyleSheetEntryStatesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hover<Impl: IMapStyleSheetEntryStatesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hover() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Selected<Impl: IMapStyleSheetEntryStatesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapStyleSheetEntryStatesStatics>, base.5, Disabled::<Impl, OFFSET>, Hover::<Impl, OFFSET>, Selected::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheetStaticsImpl: Sized {
    fn Aerial(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn AerialWithOverlay(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadLight(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadDark(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadHighContrastLight(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadHighContrastDark(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn Combine(&self, stylesheets: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<MapStyleSheet>>) -> ::windows::core::Result<MapStyleSheet>;
    fn ParseFromJson(&self, styleasjson: &::windows::core::HSTRING) -> ::windows::core::Result<MapStyleSheet>;
    fn TryParseFromJson(&self, styleasjson: &::windows::core::HSTRING, stylesheet: &mut ::core::option::Option<MapStyleSheet>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapStyleSheetStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapStyleSheetStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapStyleSheetStaticsVtbl {
    pub const fn new<Impl: IMapStyleSheetStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapStyleSheetStaticsVtbl {
        unsafe extern "system" fn Aerial<Impl: IMapStyleSheetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Aerial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AerialWithOverlay<Impl: IMapStyleSheetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AerialWithOverlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadLight<Impl: IMapStyleSheetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoadLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadDark<Impl: IMapStyleSheetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoadDark() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadHighContrastLight<Impl: IMapStyleSheetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoadHighContrastLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadHighContrastDark<Impl: IMapStyleSheetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoadHighContrastDark() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Combine<Impl: IMapStyleSheetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Combine(&*(&stylesheets as *const <super::super::super::super::Foundation::Collections::IIterable<MapStyleSheet> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<MapStyleSheet> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseFromJson<Impl: IMapStyleSheetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, styleasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParseFromJson(&*(&styleasjson as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParseFromJson<Impl: IMapStyleSheetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, styleasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, stylesheet: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryParseFromJson(&*(&styleasjson as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&stylesheet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapStyleSheetStatics>, base.5, Aerial::<Impl, OFFSET>, AerialWithOverlay::<Impl, OFFSET>, RoadLight::<Impl, OFFSET>, RoadDark::<Impl, OFFSET>, RoadHighContrastLight::<Impl, OFFSET>, RoadHighContrastDark::<Impl, OFFSET>, Combine::<Impl, OFFSET>, ParseFromJson::<Impl, OFFSET>, TryParseFromJson::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTargetCameraChangedEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTargetCameraChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTargetCameraChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTargetCameraChangedEventArgsVtbl {
    pub const fn new<Impl: IMapTargetCameraChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTargetCameraChangedEventArgsVtbl {
        unsafe extern "system" fn Camera<Impl: IMapTargetCameraChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Camera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTargetCameraChangedEventArgs>, base.5, Camera::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTargetCameraChangedEventArgs2Impl: Sized {
    fn ChangeReason(&self) -> ::windows::core::Result<MapCameraChangeReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTargetCameraChangedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTargetCameraChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTargetCameraChangedEventArgs2Vtbl {
    pub const fn new<Impl: IMapTargetCameraChangedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTargetCameraChangedEventArgs2Vtbl {
        unsafe extern "system" fn ChangeReason<Impl: IMapTargetCameraChangedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangeReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTargetCameraChangedEventArgs2>, base.5, ChangeReason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestImpl: Sized {
    fn PixelData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetPixelData(&self, value: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<MapTileBitmapRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileBitmapRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileBitmapRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileBitmapRequestVtbl {
    pub const fn new<Impl: IMapTileBitmapRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileBitmapRequestVtbl {
        unsafe extern "system" fn PixelData<Impl: IMapTileBitmapRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PixelData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelData<Impl: IMapTileBitmapRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPixelData(&*(&value as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMapTileBitmapRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileBitmapRequest>, base.5, PixelData::<Impl, OFFSET>, SetPixelData::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileBitmapRequestDeferral {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileBitmapRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileBitmapRequestDeferralVtbl {
    pub const fn new<Impl: IMapTileBitmapRequestDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileBitmapRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IMapTileBitmapRequestDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileBitmapRequestDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestedEventArgsImpl: Sized {
    fn X(&self) -> ::windows::core::Result<i32>;
    fn Y(&self) -> ::windows::core::Result<i32>;
    fn ZoomLevel(&self) -> ::windows::core::Result<i32>;
    fn Request(&self) -> ::windows::core::Result<MapTileBitmapRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileBitmapRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileBitmapRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileBitmapRequestedEventArgsVtbl {
    pub const fn new<Impl: IMapTileBitmapRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileBitmapRequestedEventArgsVtbl {
        unsafe extern "system" fn X<Impl: IMapTileBitmapRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).X() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Y<Impl: IMapTileBitmapRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Y() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevel<Impl: IMapTileBitmapRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Impl: IMapTileBitmapRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileBitmapRequestedEventArgs>, base.5, X::<Impl, OFFSET>, Y::<Impl, OFFSET>, ZoomLevel::<Impl, OFFSET>, Request::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestedEventArgs2Impl: Sized {
    fn FrameIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileBitmapRequestedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileBitmapRequestedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileBitmapRequestedEventArgs2Vtbl {
    pub const fn new<Impl: IMapTileBitmapRequestedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileBitmapRequestedEventArgs2Vtbl {
        unsafe extern "system" fn FrameIndex<Impl: IMapTileBitmapRequestedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileBitmapRequestedEventArgs2>, base.5, FrameIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileDataSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileDataSource";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileDataSourceVtbl {
    pub const fn new<Impl: IMapTileDataSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileDataSourceVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileDataSource>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileDataSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileDataSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileDataSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileDataSourceFactoryVtbl {
    pub const fn new<Impl: IMapTileDataSourceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileDataSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapTileDataSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileDataSourceFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSourceImpl: Sized {
    fn DataSource(&self) -> ::windows::core::Result<MapTileDataSource>;
    fn SetDataSource(&self, value: &::core::option::Option<MapTileDataSource>) -> ::windows::core::Result<()>;
    fn Layer(&self) -> ::windows::core::Result<MapTileLayer>;
    fn SetLayer(&self, value: MapTileLayer) -> ::windows::core::Result<()>;
    fn ZoomLevelRange(&self) -> ::windows::core::Result<MapZoomLevelRange>;
    fn SetZoomLevelRange(&self, value: &MapZoomLevelRange) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::GeoboundingBox>;
    fn SetBounds(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>) -> ::windows::core::Result<()>;
    fn AllowOverstretch(&self) -> ::windows::core::Result<bool>;
    fn SetAllowOverstretch(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFadingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsFadingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsTransparencyEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTransparencyEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRetryEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsRetryEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ZIndex(&self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn TilePixelSize(&self) -> ::windows::core::Result<i32>;
    fn SetTilePixelSize(&self, value: i32) -> ::windows::core::Result<()>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn SetVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSource";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileSourceVtbl {
    pub const fn new<Impl: IMapTileSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileSourceVtbl {
        unsafe extern "system" fn DataSource<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSource<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDataSource(&*(&value as *const <MapTileDataSource as ::windows::core::Abi>::Abi as *const <MapTileDataSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Layer<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapTileLayer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Layer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLayer<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapTileLayer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLayer(value).into()
        }
        unsafe extern "system" fn ZoomLevelRange<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapZoomLevelRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomLevelRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoomLevelRange<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapZoomLevelRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetZoomLevelRange(&*(&value as *const <MapZoomLevelRange as ::windows::core::Abi>::Abi as *const <MapZoomLevelRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bounds<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounds<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBounds(&*(&value as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowOverstretch<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowOverstretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOverstretch<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowOverstretch(value).into()
        }
        unsafe extern "system" fn IsFadingEnabled<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFadingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFadingEnabled<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsFadingEnabled(value).into()
        }
        unsafe extern "system" fn IsTransparencyEnabled<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTransparencyEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTransparencyEnabled<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsTransparencyEnabled(value).into()
        }
        unsafe extern "system" fn IsRetryEnabled<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRetryEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRetryEnabled<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsRetryEnabled(value).into()
        }
        unsafe extern "system" fn ZIndex<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        unsafe extern "system" fn TilePixelSize<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TilePixelSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTilePixelSize<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTilePixelSize(value).into()
        }
        unsafe extern "system" fn Visible<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: IMapTileSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMapTileSource>,
            base.5,
            DataSource::<Impl, OFFSET>,
            SetDataSource::<Impl, OFFSET>,
            Layer::<Impl, OFFSET>,
            SetLayer::<Impl, OFFSET>,
            ZoomLevelRange::<Impl, OFFSET>,
            SetZoomLevelRange::<Impl, OFFSET>,
            Bounds::<Impl, OFFSET>,
            SetBounds::<Impl, OFFSET>,
            AllowOverstretch::<Impl, OFFSET>,
            SetAllowOverstretch::<Impl, OFFSET>,
            IsFadingEnabled::<Impl, OFFSET>,
            SetIsFadingEnabled::<Impl, OFFSET>,
            IsTransparencyEnabled::<Impl, OFFSET>,
            SetIsTransparencyEnabled::<Impl, OFFSET>,
            IsRetryEnabled::<Impl, OFFSET>,
            SetIsRetryEnabled::<Impl, OFFSET>,
            ZIndex::<Impl, OFFSET>,
            SetZIndex::<Impl, OFFSET>,
            TilePixelSize::<Impl, OFFSET>,
            SetTilePixelSize::<Impl, OFFSET>,
            Visible::<Impl, OFFSET>,
            SetVisible::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSource2Impl: Sized {
    fn AnimationState(&self) -> ::windows::core::Result<MapTileAnimationState>;
    fn AutoPlay(&self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()>;
    fn FrameCount(&self) -> ::windows::core::Result<i32>;
    fn SetFrameCount(&self, value: i32) -> ::windows::core::Result<()>;
    fn FrameDuration(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn SetFrameDuration(&self, value: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Play(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileSource2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSource2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileSource2Vtbl {
    pub const fn new<Impl: IMapTileSource2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileSource2Vtbl {
        unsafe extern "system" fn AnimationState<Impl: IMapTileSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapTileAnimationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnimationState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoPlay<Impl: IMapTileSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoPlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoPlay<Impl: IMapTileSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoPlay(value).into()
        }
        unsafe extern "system" fn FrameCount<Impl: IMapTileSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameCount<Impl: IMapTileSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFrameCount(value).into()
        }
        unsafe extern "system" fn FrameDuration<Impl: IMapTileSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameDuration<Impl: IMapTileSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFrameDuration(&*(&value as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pause<Impl: IMapTileSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Play<Impl: IMapTileSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Play().into()
        }
        unsafe extern "system" fn Stop<Impl: IMapTileSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileSource2>, base.5, AnimationState::<Impl, OFFSET>, AutoPlay::<Impl, OFFSET>, SetAutoPlay::<Impl, OFFSET>, FrameCount::<Impl, OFFSET>, SetFrameCount::<Impl, OFFSET>, FrameDuration::<Impl, OFFSET>, SetFrameDuration::<Impl, OFFSET>, Pause::<Impl, OFFSET>, Play::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSource(&self, datasource: &::core::option::Option<MapTileDataSource>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceAndZoomRange(&self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceZoomRangeAndBounds(&self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize(&self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, tilesizeinpixels: i32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileSourceFactoryVtbl {
    pub const fn new<Impl: IMapTileSourceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapTileSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithDataSource<Impl: IMapTileSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDataSource(&*(&datasource as *const <MapTileDataSource as ::windows::core::Abi>::Abi as *const <MapTileDataSource as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithDataSourceAndZoomRange<Impl: IMapTileSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDataSourceAndZoomRange(
                &*(&datasource as *const <MapTileDataSource as ::windows::core::Abi>::Abi as *const <MapTileDataSource as ::windows::core::DefaultType>::DefaultType),
                &*(&zoomlevelrange as *const <MapZoomLevelRange as ::windows::core::Abi>::Abi as *const <MapZoomLevelRange as ::windows::core::DefaultType>::DefaultType),
                &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&innerinterface),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithDataSourceZoomRangeAndBounds<Impl: IMapTileSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, bounds: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDataSourceZoomRangeAndBounds(
                &*(&datasource as *const <MapTileDataSource as ::windows::core::Abi>::Abi as *const <MapTileDataSource as ::windows::core::DefaultType>::DefaultType),
                &*(&zoomlevelrange as *const <MapZoomLevelRange as ::windows::core::Abi>::Abi as *const <MapZoomLevelRange as ::windows::core::DefaultType>::DefaultType),
                &*(&bounds as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType),
                &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&innerinterface),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize<Impl: IMapTileSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, bounds: ::windows::core::RawPtr, tilesizeinpixels: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize(
                &*(&datasource as *const <MapTileDataSource as ::windows::core::Abi>::Abi as *const <MapTileDataSource as ::windows::core::DefaultType>::DefaultType),
                &*(&zoomlevelrange as *const <MapZoomLevelRange as ::windows::core::Abi>::Abi as *const <MapZoomLevelRange as ::windows::core::DefaultType>::DefaultType),
                &*(&bounds as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType),
                tilesizeinpixels,
                &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&innerinterface),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileSourceFactory>, base.5, CreateInstance::<Impl, OFFSET>, CreateInstanceWithDataSource::<Impl, OFFSET>, CreateInstanceWithDataSourceAndZoomRange::<Impl, OFFSET>, CreateInstanceWithDataSourceZoomRangeAndBounds::<Impl, OFFSET>, CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSourceStaticsImpl: Sized {
    fn DataSourceProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LayerProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZoomLevelRangeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn BoundsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AllowOverstretchProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsFadingEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsTransparencyEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsRetryEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TilePixelSizeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileSourceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileSourceStaticsVtbl {
    pub const fn new<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileSourceStaticsVtbl {
        unsafe extern "system" fn DataSourceProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LayerProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LayerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevelRangeProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomLevelRangeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundsProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BoundsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowOverstretchProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowOverstretchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFadingEnabledProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFadingEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransparencyEnabledProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTransparencyEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRetryEnabledProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRetryEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZIndexProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TilePixelSizeProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TilePixelSizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibleProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VisibleProperty() {
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
            ::windows::core::GetRuntimeClassName::<IMapTileSourceStatics>,
            base.5,
            DataSourceProperty::<Impl, OFFSET>,
            LayerProperty::<Impl, OFFSET>,
            ZoomLevelRangeProperty::<Impl, OFFSET>,
            BoundsProperty::<Impl, OFFSET>,
            AllowOverstretchProperty::<Impl, OFFSET>,
            IsFadingEnabledProperty::<Impl, OFFSET>,
            IsTransparencyEnabledProperty::<Impl, OFFSET>,
            IsRetryEnabledProperty::<Impl, OFFSET>,
            ZIndexProperty::<Impl, OFFSET>,
            TilePixelSizeProperty::<Impl, OFFSET>,
            VisibleProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSourceStatics2Impl: Sized {
    fn AnimationStateProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AutoPlayProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FrameCountProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FrameDurationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileSourceStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSourceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileSourceStatics2Vtbl {
    pub const fn new<Impl: IMapTileSourceStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileSourceStatics2Vtbl {
        unsafe extern "system" fn AnimationStateProperty<Impl: IMapTileSourceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnimationStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoPlayProperty<Impl: IMapTileSourceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoPlayProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameCountProperty<Impl: IMapTileSourceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameCountProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameDurationProperty<Impl: IMapTileSourceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameDurationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileSourceStatics2>, base.5, AnimationStateProperty::<Impl, OFFSET>, AutoPlayProperty::<Impl, OFFSET>, FrameCountProperty::<Impl, OFFSET>, FrameDurationProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<MapTileUriRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileUriRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileUriRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileUriRequestVtbl {
    pub const fn new<Impl: IMapTileUriRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileUriRequestVtbl {
        unsafe extern "system" fn Uri<Impl: IMapTileUriRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: IMapTileUriRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMapTileUriRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileUriRequest>, base.5, Uri::<Impl, OFFSET>, SetUri::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileUriRequestDeferral {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileUriRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileUriRequestDeferralVtbl {
    pub const fn new<Impl: IMapTileUriRequestDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileUriRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IMapTileUriRequestDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileUriRequestDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestedEventArgsImpl: Sized {
    fn X(&self) -> ::windows::core::Result<i32>;
    fn Y(&self) -> ::windows::core::Result<i32>;
    fn ZoomLevel(&self) -> ::windows::core::Result<i32>;
    fn Request(&self) -> ::windows::core::Result<MapTileUriRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileUriRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileUriRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileUriRequestedEventArgsVtbl {
    pub const fn new<Impl: IMapTileUriRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileUriRequestedEventArgsVtbl {
        unsafe extern "system" fn X<Impl: IMapTileUriRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).X() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Y<Impl: IMapTileUriRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Y() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevel<Impl: IMapTileUriRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Impl: IMapTileUriRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileUriRequestedEventArgs>, base.5, X::<Impl, OFFSET>, Y::<Impl, OFFSET>, ZoomLevel::<Impl, OFFSET>, Request::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestedEventArgs2Impl: Sized {
    fn FrameIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileUriRequestedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileUriRequestedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileUriRequestedEventArgs2Vtbl {
    pub const fn new<Impl: IMapTileUriRequestedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapTileUriRequestedEventArgs2Vtbl {
        unsafe extern "system" fn FrameIndex<Impl: IMapTileUriRequestedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapTileUriRequestedEventArgs2>, base.5, FrameIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreetsideExperienceImpl: Sized {
    fn AddressTextVisible(&self) -> ::windows::core::Result<bool>;
    fn SetAddressTextVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn CursorVisible(&self) -> ::windows::core::Result<bool>;
    fn SetCursorVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn OverviewMapVisible(&self) -> ::windows::core::Result<bool>;
    fn SetOverviewMapVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn StreetLabelsVisible(&self) -> ::windows::core::Result<bool>;
    fn SetStreetLabelsVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExitButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetExitButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn ZoomButtonsVisible(&self) -> ::windows::core::Result<bool>;
    fn SetZoomButtonsVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreetsideExperience {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IStreetsideExperience";
}
#[cfg(feature = "implement_exclusive")]
impl IStreetsideExperienceVtbl {
    pub const fn new<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStreetsideExperienceVtbl {
        unsafe extern "system" fn AddressTextVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddressTextVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddressTextVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAddressTextVisible(value).into()
        }
        unsafe extern "system" fn CursorVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CursorVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCursorVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCursorVisible(value).into()
        }
        unsafe extern "system" fn OverviewMapVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverviewMapVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverviewMapVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOverviewMapVisible(value).into()
        }
        unsafe extern "system" fn StreetLabelsVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreetLabelsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreetLabelsVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStreetLabelsVisible(value).into()
        }
        unsafe extern "system" fn ExitButtonVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitButtonVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitButtonVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExitButtonVisible(value).into()
        }
        unsafe extern "system" fn ZoomButtonsVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomButtonsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoomButtonsVisible<Impl: IStreetsideExperienceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetZoomButtonsVisible(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IStreetsideExperience>,
            base.5,
            AddressTextVisible::<Impl, OFFSET>,
            SetAddressTextVisible::<Impl, OFFSET>,
            CursorVisible::<Impl, OFFSET>,
            SetCursorVisible::<Impl, OFFSET>,
            OverviewMapVisible::<Impl, OFFSET>,
            SetOverviewMapVisible::<Impl, OFFSET>,
            StreetLabelsVisible::<Impl, OFFSET>,
            SetStreetLabelsVisible::<Impl, OFFSET>,
            ExitButtonVisible::<Impl, OFFSET>,
            SetExitButtonVisible::<Impl, OFFSET>,
            ZoomButtonsVisible::<Impl, OFFSET>,
            SetZoomButtonsVisible::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreetsideExperienceFactoryImpl: Sized {
    fn CreateInstanceWithPanorama(&self, panorama: &::core::option::Option<StreetsidePanorama>) -> ::windows::core::Result<StreetsideExperience>;
    fn CreateInstanceWithPanoramaHeadingPitchAndFieldOfView(&self, panorama: &::core::option::Option<StreetsidePanorama>, headingindegrees: f64, pitchindegrees: f64, fieldofviewindegrees: f64) -> ::windows::core::Result<StreetsideExperience>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreetsideExperienceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IStreetsideExperienceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStreetsideExperienceFactoryVtbl {
    pub const fn new<Impl: IStreetsideExperienceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStreetsideExperienceFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithPanorama<Impl: IStreetsideExperienceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, panorama: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithPanorama(&*(&panorama as *const <StreetsidePanorama as ::windows::core::Abi>::Abi as *const <StreetsidePanorama as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithPanoramaHeadingPitchAndFieldOfView<Impl: IStreetsideExperienceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, panorama: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, fieldofviewindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithPanoramaHeadingPitchAndFieldOfView(&*(&panorama as *const <StreetsidePanorama as ::windows::core::Abi>::Abi as *const <StreetsidePanorama as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees, fieldofviewindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStreetsideExperienceFactory>, base.5, CreateInstanceWithPanorama::<Impl, OFFSET>, CreateInstanceWithPanoramaHeadingPitchAndFieldOfView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreetsidePanoramaImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreetsidePanorama {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IStreetsidePanorama";
}
#[cfg(feature = "implement_exclusive")]
impl IStreetsidePanoramaVtbl {
    pub const fn new<Impl: IStreetsidePanoramaImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStreetsidePanoramaVtbl {
        unsafe extern "system" fn Location<Impl: IStreetsidePanoramaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStreetsidePanorama>, base.5, Location::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreetsidePanoramaStaticsImpl: Sized {
    fn FindNearbyWithLocationAsync(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>>;
    fn FindNearbyWithLocationAndRadiusAsync(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, radiusinmeters: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreetsidePanoramaStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IStreetsidePanoramaStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStreetsidePanoramaStaticsVtbl {
    pub const fn new<Impl: IStreetsidePanoramaStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStreetsidePanoramaStaticsVtbl {
        unsafe extern "system" fn FindNearbyWithLocationAsync<Impl: IStreetsidePanoramaStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindNearbyWithLocationAsync(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNearbyWithLocationAndRadiusAsync<Impl: IStreetsidePanoramaStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindNearbyWithLocationAndRadiusAsync(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), radiusinmeters) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStreetsidePanoramaStatics>, base.5, FindNearbyWithLocationAsync::<Impl, OFFSET>, FindNearbyWithLocationAndRadiusAsync::<Impl, OFFSET>)
    }
}
