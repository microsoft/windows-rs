#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICustomMapTileDataSourceImpl: Sized {
    fn BitmapRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CustomMapTileDataSource, MapTileBitmapRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBitmapRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICustomMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.ICustomMapTileDataSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICustomMapTileDataSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomMapTileDataSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomMapTileDataSourceVtbl {
        unsafe extern "system" fn BitmapRequested<Impl: ICustomMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapRequested(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CustomMapTileDataSource, MapTileBitmapRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CustomMapTileDataSource, MapTileBitmapRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBitmapRequested<Impl: ICustomMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBitmapRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICustomMapTileDataSource>, ::windows::core::GetTrustLevel, BitmapRequested::<Impl, IMPL_OFFSET>, RemoveBitmapRequested::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomMapTileDataSource as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomMapTileDataSourceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomMapTileDataSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICustomMapTileDataSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICustomMapTileDataSourceFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomMapTileDataSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpMapTileDataSourceImpl: Sized {
    fn UriFormatString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUriFormatString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AdditionalRequestHeaders(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn AllowCaching(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCaching(&self, value: bool) -> ::windows::core::Result<()>;
    fn UriRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<HttpMapTileDataSource, MapTileUriRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUriRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IHttpMapTileDataSource";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpMapTileDataSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMapTileDataSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMapTileDataSourceVtbl {
        unsafe extern "system" fn UriFormatString<Impl: IHttpMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UriFormatString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUriFormatString<Impl: IHttpMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUriFormatString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdditionalRequestHeaders<Impl: IHttpMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdditionalRequestHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowCaching<Impl: IHttpMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowCaching() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowCaching<Impl: IHttpMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowCaching(value).into()
        }
        unsafe extern "system" fn UriRequested<Impl: IHttpMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UriRequested(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<HttpMapTileDataSource, MapTileUriRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<HttpMapTileDataSource, MapTileUriRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUriRequested<Impl: IHttpMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUriRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHttpMapTileDataSource>,
            ::windows::core::GetTrustLevel,
            UriFormatString::<Impl, IMPL_OFFSET>,
            SetUriFormatString::<Impl, IMPL_OFFSET>,
            AdditionalRequestHeaders::<Impl, IMPL_OFFSET>,
            AllowCaching::<Impl, IMPL_OFFSET>,
            SetAllowCaching::<Impl, IMPL_OFFSET>,
            UriRequested::<Impl, IMPL_OFFSET>,
            RemoveUriRequested::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMapTileDataSource as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMapTileDataSourceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMapTileDataSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IHttpMapTileDataSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithUriFormatString<Impl: IHttpMapTileDataSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriformatstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithUriFormatString(&*(&uriformatstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpMapTileDataSourceFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>, CreateInstanceWithUriFormatString::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMapTileDataSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILocalMapTileDataSourceImpl: Sized {
    fn UriFormatString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUriFormatString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UriRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<LocalMapTileDataSource, MapTileUriRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUriRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILocalMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.ILocalMapTileDataSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILocalMapTileDataSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalMapTileDataSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalMapTileDataSourceVtbl {
        unsafe extern "system" fn UriFormatString<Impl: ILocalMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UriFormatString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUriFormatString<Impl: ILocalMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUriFormatString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UriRequested<Impl: ILocalMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UriRequested(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<LocalMapTileDataSource, MapTileUriRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<LocalMapTileDataSource, MapTileUriRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUriRequested<Impl: ILocalMapTileDataSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUriRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalMapTileDataSource>, ::windows::core::GetTrustLevel, UriFormatString::<Impl, IMPL_OFFSET>, SetUriFormatString::<Impl, IMPL_OFFSET>, UriRequested::<Impl, IMPL_OFFSET>, RemoveUriRequested::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalMapTileDataSource as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalMapTileDataSourceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalMapTileDataSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ILocalMapTileDataSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithUriFormatString<Impl: ILocalMapTileDataSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriformatstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithUriFormatString(&*(&uriformatstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalMapTileDataSourceFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>, CreateInstanceWithUriFormatString::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalMapTileDataSourceFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapActualCameraChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapActualCameraChangedEventArgsVtbl {
        unsafe extern "system" fn Camera<Impl: IMapActualCameraChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Camera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapActualCameraChangedEventArgs>, ::windows::core::GetTrustLevel, Camera::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapActualCameraChangedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapActualCameraChangedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapActualCameraChangedEventArgs2Vtbl {
        unsafe extern "system" fn ChangeReason<Impl: IMapActualCameraChangedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapActualCameraChangedEventArgs2>, ::windows::core::GetTrustLevel, ChangeReason::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapActualCameraChangedEventArgs2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapActualCameraChangingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapActualCameraChangingEventArgsVtbl {
        unsafe extern "system" fn Camera<Impl: IMapActualCameraChangingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Camera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapActualCameraChangingEventArgs>, ::windows::core::GetTrustLevel, Camera::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapActualCameraChangingEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapActualCameraChangingEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapActualCameraChangingEventArgs2Vtbl {
        unsafe extern "system" fn ChangeReason<Impl: IMapActualCameraChangingEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapActualCameraChangingEventArgs2>, ::windows::core::GetTrustLevel, ChangeReason::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapActualCameraChangingEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapBillboard {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapBillboard";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMapBillboardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapBillboardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapBillboardVtbl {
        unsafe extern "system" fn Location<Impl: IMapBillboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IMapBillboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalizedAnchorPoint<Impl: IMapBillboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedAnchorPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedAnchorPoint<Impl: IMapBillboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalizedAnchorPoint(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IMapBillboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: IMapBillboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CollisionBehaviorDesired<Impl: IMapBillboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapElementCollisionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollisionBehaviorDesired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollisionBehaviorDesired<Impl: IMapBillboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapElementCollisionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCollisionBehaviorDesired(value).into()
        }
        unsafe extern "system" fn ReferenceCamera<Impl: IMapBillboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferenceCamera() {
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
            ::windows::core::GetRuntimeClassName::<IMapBillboard>,
            ::windows::core::GetTrustLevel,
            Location::<Impl, IMPL_OFFSET>,
            SetLocation::<Impl, IMPL_OFFSET>,
            NormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
            SetNormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
            Image::<Impl, IMPL_OFFSET>,
            SetImage::<Impl, IMPL_OFFSET>,
            CollisionBehaviorDesired::<Impl, IMPL_OFFSET>,
            SetCollisionBehaviorDesired::<Impl, IMPL_OFFSET>,
            ReferenceCamera::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapBillboard as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapBillboardFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapBillboardFactoryVtbl {
        unsafe extern "system" fn CreateInstanceFromCamera<Impl: IMapBillboardFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, camera: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceFromCamera(&*(&camera as *const <MapCamera as ::windows::core::Abi>::Abi as *const <MapCamera as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapBillboardFactory>, ::windows::core::GetTrustLevel, CreateInstanceFromCamera::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapBillboardFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapBillboardStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapBillboardStaticsVtbl {
        unsafe extern "system" fn LocationProperty<Impl: IMapBillboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedAnchorPointProperty<Impl: IMapBillboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedAnchorPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollisionBehaviorDesiredProperty<Impl: IMapBillboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollisionBehaviorDesiredProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapBillboardStatics>, ::windows::core::GetTrustLevel, LocationProperty::<Impl, IMPL_OFFSET>, NormalizedAnchorPointProperty::<Impl, IMPL_OFFSET>, CollisionBehaviorDesiredProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapBillboardStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapCamera {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCamera";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapCameraVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapCameraImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapCameraVtbl {
        unsafe extern "system" fn Location<Impl: IMapCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IMapCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Heading<Impl: IMapCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Heading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeading<Impl: IMapCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeading(value).into()
        }
        unsafe extern "system" fn Pitch<Impl: IMapCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPitch<Impl: IMapCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPitch(value).into()
        }
        unsafe extern "system" fn Roll<Impl: IMapCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Roll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoll<Impl: IMapCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoll(value).into()
        }
        unsafe extern "system" fn FieldOfView<Impl: IMapCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FieldOfView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFieldOfView<Impl: IMapCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFieldOfView(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapCamera>,
            ::windows::core::GetTrustLevel,
            Location::<Impl, IMPL_OFFSET>,
            SetLocation::<Impl, IMPL_OFFSET>,
            Heading::<Impl, IMPL_OFFSET>,
            SetHeading::<Impl, IMPL_OFFSET>,
            Pitch::<Impl, IMPL_OFFSET>,
            SetPitch::<Impl, IMPL_OFFSET>,
            Roll::<Impl, IMPL_OFFSET>,
            SetRoll::<Impl, IMPL_OFFSET>,
            FieldOfView::<Impl, IMPL_OFFSET>,
            SetFieldOfView::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapCamera as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IMapCameraFactoryImpl: Sized {
    fn CreateInstanceWithLocation(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationAndHeading(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationHeadingAndPitch(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationHeadingPitchRollAndFieldOfView(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64, rollindegrees: f64, fieldofviewindegrees: f64) -> ::windows::core::Result<MapCamera>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapCameraFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCameraFactory";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapCameraFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapCameraFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapCameraFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithLocation<Impl: IMapCameraFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithLocation(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithLocationAndHeading<Impl: IMapCameraFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithLocationAndHeading(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), headingindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithLocationHeadingAndPitch<Impl: IMapCameraFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithLocationHeadingAndPitch(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithLocationHeadingPitchRollAndFieldOfView<Impl: IMapCameraFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, rollindegrees: f64, fieldofviewindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithLocationHeadingPitchRollAndFieldOfView(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees, rollindegrees, fieldofviewindegrees) {
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
            ::windows::core::GetRuntimeClassName::<IMapCameraFactory>,
            ::windows::core::GetTrustLevel,
            CreateInstanceWithLocation::<Impl, IMPL_OFFSET>,
            CreateInstanceWithLocationAndHeading::<Impl, IMPL_OFFSET>,
            CreateInstanceWithLocationHeadingAndPitch::<Impl, IMPL_OFFSET>,
            CreateInstanceWithLocationHeadingPitchRollAndFieldOfView::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapCameraFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapContextRequestedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapContextRequestedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapContextRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapContextRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapContextRequestedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapContextRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapContextRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElements<Impl: IMapContextRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapContextRequestedEventArgs>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, MapElements::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapContextRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlVtbl {
        unsafe extern "system" fn Center<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Children<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorScheme<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapColorScheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorScheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorScheme<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapColorScheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorScheme(value).into()
        }
        unsafe extern "system" fn DesiredPitch<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredPitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPitch<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredPitch(value).into()
        }
        unsafe extern "system" fn Heading<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Heading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeading<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeading(value).into()
        }
        unsafe extern "system" fn LandmarksVisible<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LandmarksVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLandmarksVisible<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLandmarksVisible(value).into()
        }
        unsafe extern "system" fn LoadingStatus<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapLoadingStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadingStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapServiceToken<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapServiceToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapServiceToken<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapServiceToken(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxZoomLevel<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinZoomLevel<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PedestrianFeaturesVisible<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PedestrianFeaturesVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPedestrianFeaturesVisible<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPedestrianFeaturesVisible(value).into()
        }
        unsafe extern "system" fn Pitch<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Style<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Style() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyle(value).into()
        }
        unsafe extern "system" fn TrafficFlowVisible<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrafficFlowVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrafficFlowVisible<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrafficFlowVisible(value).into()
        }
        unsafe extern "system" fn TransformOrigin<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformOrigin<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformOrigin(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WatermarkMode<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapWatermarkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WatermarkMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWatermarkMode<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapWatermarkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWatermarkMode(value).into()
        }
        unsafe extern "system" fn ZoomLevel<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoomLevel<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoomLevel(value).into()
        }
        unsafe extern "system" fn MapElements<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Routes<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Routes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileSources<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileSources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCenterChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCenterChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HeadingChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeadingChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeadingChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHeadingChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoadingStatusChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadingStatusChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoadingStatusChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoadingStatusChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapDoubleTapped<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapDoubleTapped(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapDoubleTapped<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapDoubleTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapHolding<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapHolding(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapHolding<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapHolding(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapTapped<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapTapped(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapTapped<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PitchChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PitchChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePitchChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePitchChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformOriginChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformOriginChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransformOriginChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransformOriginChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ZoomLevelChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevelChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveZoomLevelChanged<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveZoomLevelChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindMapElementsAtOffset<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindMapElementsAtOffset(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocationFromOffset<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, location: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocationFromOffset(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&location)).into()
        }
        unsafe extern "system" fn GetOffsetFromLocation<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, offset: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOffsetFromLocation(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn IsLocationInView<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, isinview: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsLocationInView(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isinview)).into()
        }
        unsafe extern "system" fn TrySetViewBoundsAsync<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, margin: ::windows::core::RawPtr, animation: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn TrySetViewWithCenterAsync<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetViewWithCenterAsync(&*(&center as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetViewWithCenterAndZoomAsync<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetViewWithCenterAndZoomAsync(&*(&center as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), &*(&zoomlevel as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetViewWithCenterZoomHeadingAndPitchAsync<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, heading: ::windows::core::RawPtr, desiredpitch: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync<Impl: IMapControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, heading: ::windows::core::RawPtr, desiredpitch: ::windows::core::RawPtr, animation: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapControl>,
            ::windows::core::GetTrustLevel,
            Center::<Impl, IMPL_OFFSET>,
            SetCenter::<Impl, IMPL_OFFSET>,
            Children::<Impl, IMPL_OFFSET>,
            ColorScheme::<Impl, IMPL_OFFSET>,
            SetColorScheme::<Impl, IMPL_OFFSET>,
            DesiredPitch::<Impl, IMPL_OFFSET>,
            SetDesiredPitch::<Impl, IMPL_OFFSET>,
            Heading::<Impl, IMPL_OFFSET>,
            SetHeading::<Impl, IMPL_OFFSET>,
            LandmarksVisible::<Impl, IMPL_OFFSET>,
            SetLandmarksVisible::<Impl, IMPL_OFFSET>,
            LoadingStatus::<Impl, IMPL_OFFSET>,
            MapServiceToken::<Impl, IMPL_OFFSET>,
            SetMapServiceToken::<Impl, IMPL_OFFSET>,
            MaxZoomLevel::<Impl, IMPL_OFFSET>,
            MinZoomLevel::<Impl, IMPL_OFFSET>,
            PedestrianFeaturesVisible::<Impl, IMPL_OFFSET>,
            SetPedestrianFeaturesVisible::<Impl, IMPL_OFFSET>,
            Pitch::<Impl, IMPL_OFFSET>,
            Style::<Impl, IMPL_OFFSET>,
            SetStyle::<Impl, IMPL_OFFSET>,
            TrafficFlowVisible::<Impl, IMPL_OFFSET>,
            SetTrafficFlowVisible::<Impl, IMPL_OFFSET>,
            TransformOrigin::<Impl, IMPL_OFFSET>,
            SetTransformOrigin::<Impl, IMPL_OFFSET>,
            WatermarkMode::<Impl, IMPL_OFFSET>,
            SetWatermarkMode::<Impl, IMPL_OFFSET>,
            ZoomLevel::<Impl, IMPL_OFFSET>,
            SetZoomLevel::<Impl, IMPL_OFFSET>,
            MapElements::<Impl, IMPL_OFFSET>,
            Routes::<Impl, IMPL_OFFSET>,
            TileSources::<Impl, IMPL_OFFSET>,
            CenterChanged::<Impl, IMPL_OFFSET>,
            RemoveCenterChanged::<Impl, IMPL_OFFSET>,
            HeadingChanged::<Impl, IMPL_OFFSET>,
            RemoveHeadingChanged::<Impl, IMPL_OFFSET>,
            LoadingStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveLoadingStatusChanged::<Impl, IMPL_OFFSET>,
            MapDoubleTapped::<Impl, IMPL_OFFSET>,
            RemoveMapDoubleTapped::<Impl, IMPL_OFFSET>,
            MapHolding::<Impl, IMPL_OFFSET>,
            RemoveMapHolding::<Impl, IMPL_OFFSET>,
            MapTapped::<Impl, IMPL_OFFSET>,
            RemoveMapTapped::<Impl, IMPL_OFFSET>,
            PitchChanged::<Impl, IMPL_OFFSET>,
            RemovePitchChanged::<Impl, IMPL_OFFSET>,
            TransformOriginChanged::<Impl, IMPL_OFFSET>,
            RemoveTransformOriginChanged::<Impl, IMPL_OFFSET>,
            ZoomLevelChanged::<Impl, IMPL_OFFSET>,
            RemoveZoomLevelChanged::<Impl, IMPL_OFFSET>,
            FindMapElementsAtOffset::<Impl, IMPL_OFFSET>,
            GetLocationFromOffset::<Impl, IMPL_OFFSET>,
            GetOffsetFromLocation::<Impl, IMPL_OFFSET>,
            IsLocationInView::<Impl, IMPL_OFFSET>,
            TrySetViewBoundsAsync::<Impl, IMPL_OFFSET>,
            TrySetViewWithCenterAsync::<Impl, IMPL_OFFSET>,
            TrySetViewWithCenterAndZoomAsync::<Impl, IMPL_OFFSET>,
            TrySetViewWithCenterZoomHeadingAndPitchAsync::<Impl, IMPL_OFFSET>,
            TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl2Vtbl {
        unsafe extern "system" fn BusinessLandmarksVisible<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarksVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusinessLandmarksVisible<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusinessLandmarksVisible(value).into()
        }
        unsafe extern "system" fn TransitFeaturesVisible<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitFeaturesVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransitFeaturesVisible<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransitFeaturesVisible(value).into()
        }
        unsafe extern "system" fn PanInteractionMode<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapPanInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PanInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPanInteractionMode<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapPanInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPanInteractionMode(value).into()
        }
        unsafe extern "system" fn RotateInteractionMode<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotateInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotateInteractionMode<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotateInteractionMode(value).into()
        }
        unsafe extern "system" fn TiltInteractionMode<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TiltInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTiltInteractionMode<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTiltInteractionMode(value).into()
        }
        unsafe extern "system" fn ZoomInteractionMode<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoomInteractionMode<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoomInteractionMode(value).into()
        }
        unsafe extern "system" fn Is3DSupported<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Is3DSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStreetsideSupported<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStreetsideSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scene<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scene() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScene<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScene(&*(&value as *const <MapScene as ::windows::core::Abi>::Abi as *const <MapScene as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualCamera<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetCamera<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomExperience<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomExperience() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomExperience<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomExperience(&*(&value as *const <MapCustomExperience as ::windows::core::Abi>::Abi as *const <MapCustomExperience as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementClick<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElementClick(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementClickEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementClickEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementClick<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerEntered<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElementPointerEntered(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerEnteredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerEnteredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementPointerEntered<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerExited<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElementPointerExited(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerExitedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerExitedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementPointerExited<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualCameraChanged<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualCameraChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActualCameraChanged<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActualCameraChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualCameraChanging<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualCameraChanging(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActualCameraChanging<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActualCameraChanging(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetCameraChanged<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetCameraChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapTargetCameraChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapTargetCameraChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTargetCameraChanged<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTargetCameraChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CustomExperienceChanged<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomExperienceChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapCustomExperienceChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapCustomExperienceChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCustomExperienceChanged<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCustomExperienceChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartContinuousRotate<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rateindegreespersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartContinuousRotate(rateindegreespersecond).into()
        }
        unsafe extern "system" fn StopContinuousRotate<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopContinuousRotate().into()
        }
        unsafe extern "system" fn StartContinuousTilt<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rateindegreespersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartContinuousTilt(rateindegreespersecond).into()
        }
        unsafe extern "system" fn StopContinuousTilt<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopContinuousTilt().into()
        }
        unsafe extern "system" fn StartContinuousZoom<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rateofchangepersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartContinuousZoom(rateofchangepersecond).into()
        }
        unsafe extern "system" fn StopContinuousZoom<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopContinuousZoom().into()
        }
        unsafe extern "system" fn TryRotateAsync<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRotateAsync(degrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRotateToAsync<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angleindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRotateToAsync(angleindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTiltAsync<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryTiltAsync(degrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTiltToAsync<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angleindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryTiltToAsync(angleindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryZoomInAsync<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryZoomInAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryZoomOutAsync<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryZoomOutAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryZoomToAsync<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomlevel: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryZoomToAsync(zoomlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetSceneAsync<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scene: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetSceneAsync(&*(&scene as *const <MapScene as ::windows::core::Abi>::Abi as *const <MapScene as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetSceneWithAnimationAsync<Impl: IMapControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scene: ::windows::core::RawPtr, animationkind: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapControl2>,
            ::windows::core::GetTrustLevel,
            BusinessLandmarksVisible::<Impl, IMPL_OFFSET>,
            SetBusinessLandmarksVisible::<Impl, IMPL_OFFSET>,
            TransitFeaturesVisible::<Impl, IMPL_OFFSET>,
            SetTransitFeaturesVisible::<Impl, IMPL_OFFSET>,
            PanInteractionMode::<Impl, IMPL_OFFSET>,
            SetPanInteractionMode::<Impl, IMPL_OFFSET>,
            RotateInteractionMode::<Impl, IMPL_OFFSET>,
            SetRotateInteractionMode::<Impl, IMPL_OFFSET>,
            TiltInteractionMode::<Impl, IMPL_OFFSET>,
            SetTiltInteractionMode::<Impl, IMPL_OFFSET>,
            ZoomInteractionMode::<Impl, IMPL_OFFSET>,
            SetZoomInteractionMode::<Impl, IMPL_OFFSET>,
            Is3DSupported::<Impl, IMPL_OFFSET>,
            IsStreetsideSupported::<Impl, IMPL_OFFSET>,
            Scene::<Impl, IMPL_OFFSET>,
            SetScene::<Impl, IMPL_OFFSET>,
            ActualCamera::<Impl, IMPL_OFFSET>,
            TargetCamera::<Impl, IMPL_OFFSET>,
            CustomExperience::<Impl, IMPL_OFFSET>,
            SetCustomExperience::<Impl, IMPL_OFFSET>,
            MapElementClick::<Impl, IMPL_OFFSET>,
            RemoveMapElementClick::<Impl, IMPL_OFFSET>,
            MapElementPointerEntered::<Impl, IMPL_OFFSET>,
            RemoveMapElementPointerEntered::<Impl, IMPL_OFFSET>,
            MapElementPointerExited::<Impl, IMPL_OFFSET>,
            RemoveMapElementPointerExited::<Impl, IMPL_OFFSET>,
            ActualCameraChanged::<Impl, IMPL_OFFSET>,
            RemoveActualCameraChanged::<Impl, IMPL_OFFSET>,
            ActualCameraChanging::<Impl, IMPL_OFFSET>,
            RemoveActualCameraChanging::<Impl, IMPL_OFFSET>,
            TargetCameraChanged::<Impl, IMPL_OFFSET>,
            RemoveTargetCameraChanged::<Impl, IMPL_OFFSET>,
            CustomExperienceChanged::<Impl, IMPL_OFFSET>,
            RemoveCustomExperienceChanged::<Impl, IMPL_OFFSET>,
            StartContinuousRotate::<Impl, IMPL_OFFSET>,
            StopContinuousRotate::<Impl, IMPL_OFFSET>,
            StartContinuousTilt::<Impl, IMPL_OFFSET>,
            StopContinuousTilt::<Impl, IMPL_OFFSET>,
            StartContinuousZoom::<Impl, IMPL_OFFSET>,
            StopContinuousZoom::<Impl, IMPL_OFFSET>,
            TryRotateAsync::<Impl, IMPL_OFFSET>,
            TryRotateToAsync::<Impl, IMPL_OFFSET>,
            TryTiltAsync::<Impl, IMPL_OFFSET>,
            TryTiltToAsync::<Impl, IMPL_OFFSET>,
            TryZoomInAsync::<Impl, IMPL_OFFSET>,
            TryZoomOutAsync::<Impl, IMPL_OFFSET>,
            TryZoomToAsync::<Impl, IMPL_OFFSET>,
            TrySetSceneAsync::<Impl, IMPL_OFFSET>,
            TrySetSceneWithAnimationAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapControl3Impl: Sized {
    fn MapRightTapped(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapRightTappedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapRightTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapControl3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl3Vtbl {
        unsafe extern "system" fn MapRightTapped<Impl: IMapControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapRightTapped(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapRightTappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapRightTappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapRightTapped<Impl: IMapControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapRightTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControl3>, ::windows::core::GetTrustLevel, MapRightTapped::<Impl, IMPL_OFFSET>, RemoveMapRightTapped::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IMapControl4Impl: Sized {
    fn BusinessLandmarksEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetBusinessLandmarksEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransitFeaturesEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetTransitFeaturesEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetVisibleRegion(&self, region: MapVisibleRegionKind) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl4";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapControl4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl4Vtbl {
        unsafe extern "system" fn BusinessLandmarksEnabled<Impl: IMapControl4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarksEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusinessLandmarksEnabled<Impl: IMapControl4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusinessLandmarksEnabled(value).into()
        }
        unsafe extern "system" fn TransitFeaturesEnabled<Impl: IMapControl4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitFeaturesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransitFeaturesEnabled<Impl: IMapControl4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransitFeaturesEnabled(value).into()
        }
        unsafe extern "system" fn GetVisibleRegion<Impl: IMapControl4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, region: MapVisibleRegionKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisibleRegion(region) {
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
            ::windows::core::GetRuntimeClassName::<IMapControl4>,
            ::windows::core::GetTrustLevel,
            BusinessLandmarksEnabled::<Impl, IMPL_OFFSET>,
            SetBusinessLandmarksEnabled::<Impl, IMPL_OFFSET>,
            TransitFeaturesEnabled::<Impl, IMPL_OFFSET>,
            SetTransitFeaturesEnabled::<Impl, IMPL_OFFSET>,
            GetVisibleRegion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl5";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControl5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl5Vtbl {
        unsafe extern "system" fn MapProjection<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapProjection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapProjection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapProjection<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapProjection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapProjection(value).into()
        }
        unsafe extern "system" fn StyleSheet<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleSheet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyleSheet<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyleSheet(&*(&value as *const <MapStyleSheet as ::windows::core::Abi>::Abi as *const <MapStyleSheet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ViewPadding<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewPadding<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewPadding(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapContextRequested<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapContextRequested(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapContextRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapContextRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapContextRequested<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapContextRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindMapElementsAtOffsetWithRadius<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindMapElementsAtOffsetWithRadius(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocationFromOffsetWithReferenceSystem<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocationFromOffsetWithReferenceSystem(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), desiredreferencesystem, ::core::mem::transmute_copy(&location)).into()
        }
        unsafe extern "system" fn StartContinuousPan<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpixelspersecond: f64, verticalpixelspersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartContinuousPan(horizontalpixelspersecond, verticalpixelspersecond).into()
        }
        unsafe extern "system" fn StopContinuousPan<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopContinuousPan().into()
        }
        unsafe extern "system" fn TryPanAsync<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpixels: f64, verticalpixels: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryPanAsync(horizontalpixels, verticalpixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryPanToAsync<Impl: IMapControl5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapControl5>,
            ::windows::core::GetTrustLevel,
            MapProjection::<Impl, IMPL_OFFSET>,
            SetMapProjection::<Impl, IMPL_OFFSET>,
            StyleSheet::<Impl, IMPL_OFFSET>,
            SetStyleSheet::<Impl, IMPL_OFFSET>,
            ViewPadding::<Impl, IMPL_OFFSET>,
            SetViewPadding::<Impl, IMPL_OFFSET>,
            MapContextRequested::<Impl, IMPL_OFFSET>,
            RemoveMapContextRequested::<Impl, IMPL_OFFSET>,
            FindMapElementsAtOffsetWithRadius::<Impl, IMPL_OFFSET>,
            GetLocationFromOffsetWithReferenceSystem::<Impl, IMPL_OFFSET>,
            StartContinuousPan::<Impl, IMPL_OFFSET>,
            StopContinuousPan::<Impl, IMPL_OFFSET>,
            TryPanAsync::<Impl, IMPL_OFFSET>,
            TryPanToAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControl6Impl: Sized {
    fn Layers(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapLayer>>;
    fn SetLayers(&self, value: &::core::option::Option<super::super::super::super::Foundation::Collections::IVector<MapLayer>>) -> ::windows::core::Result<()>;
    fn TryGetLocationFromOffset(&self, offset: &super::super::super::super::Foundation::Point, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<bool>;
    fn TryGetLocationFromOffsetWithReferenceSystem(&self, offset: &super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl6";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControl6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl6Vtbl {
        unsafe extern "system" fn Layers<Impl: IMapControl6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Layers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLayers<Impl: IMapControl6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLayers(&*(&value as *const <super::super::super::super::Foundation::Collections::IVector<MapLayer> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVector<MapLayer> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryGetLocationFromOffset<Impl: IMapControl6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, location: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetLocationFromOffset(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetLocationFromOffsetWithReferenceSystem<Impl: IMapControl6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetLocationFromOffsetWithReferenceSystem(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), desiredreferencesystem, ::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControl6>, ::windows::core::GetTrustLevel, Layers::<Impl, IMPL_OFFSET>, SetLayers::<Impl, IMPL_OFFSET>, TryGetLocationFromOffset::<Impl, IMPL_OFFSET>, TryGetLocationFromOffsetWithReferenceSystem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl6 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl7Vtbl {
        unsafe extern "system" fn Region<Impl: IMapControl7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Region() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegion<Impl: IMapControl7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRegion(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControl7>, ::windows::core::GetTrustLevel, Region::<Impl, IMPL_OFFSET>, SetRegion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl7 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl8Vtbl {
        unsafe extern "system" fn CanTiltDown<Impl: IMapControl8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanTiltDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanTiltUp<Impl: IMapControl8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanTiltUp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanZoomIn<Impl: IMapControl8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanZoomIn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanZoomOut<Impl: IMapControl8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanZoomOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControl8>, ::windows::core::GetTrustLevel, CanTiltDown::<Impl, IMPL_OFFSET>, CanTiltUp::<Impl, IMPL_OFFSET>, CanZoomIn::<Impl, IMPL_OFFSET>, CanZoomOut::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
pub trait IMapControlBusinessLandmarkClickEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkClickEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl IMapControlBusinessLandmarkClickEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlBusinessLandmarkClickEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlBusinessLandmarkClickEventArgsVtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkClickEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlBusinessLandmarkClickEventArgs>, ::windows::core::GetTrustLevel, LocalLocations::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlBusinessLandmarkClickEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
pub trait IMapControlBusinessLandmarkPointerEnteredEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkPointerEnteredEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl IMapControlBusinessLandmarkPointerEnteredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlBusinessLandmarkPointerEnteredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlBusinessLandmarkPointerEnteredEventArgsVtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkPointerEnteredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlBusinessLandmarkPointerEnteredEventArgs>, ::windows::core::GetTrustLevel, LocalLocations::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlBusinessLandmarkPointerEnteredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
pub trait IMapControlBusinessLandmarkPointerExitedEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkPointerExitedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl IMapControlBusinessLandmarkPointerExitedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlBusinessLandmarkPointerExitedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlBusinessLandmarkPointerExitedEventArgsVtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkPointerExitedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlBusinessLandmarkPointerExitedEventArgs>, ::windows::core::GetTrustLevel, LocalLocations::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlBusinessLandmarkPointerExitedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
pub trait IMapControlBusinessLandmarkRightTappedEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkRightTappedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl IMapControlBusinessLandmarkRightTappedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlBusinessLandmarkRightTappedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlBusinessLandmarkRightTappedEventArgsVtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkRightTappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlBusinessLandmarkRightTappedEventArgs>, ::windows::core::GetTrustLevel, LocalLocations::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlBusinessLandmarkRightTappedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlDataHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlDataHelper";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapControlDataHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlDataHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlDataHelperVtbl {
        unsafe extern "system" fn BusinessLandmarkClick<Impl: IMapControlDataHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarkClick(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkClickEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkClickEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBusinessLandmarkClick<Impl: IMapControlDataHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeatureClick<Impl: IMapControlDataHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitFeatureClick(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureClickEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureClickEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransitFeatureClick<Impl: IMapControlDataHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeatureClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BusinessLandmarkRightTapped<Impl: IMapControlDataHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarkRightTapped(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkRightTappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkRightTappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBusinessLandmarkRightTapped<Impl: IMapControlDataHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkRightTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeatureRightTapped<Impl: IMapControlDataHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitFeatureRightTapped(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureRightTappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureRightTappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransitFeatureRightTapped<Impl: IMapControlDataHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeatureRightTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapControlDataHelper>,
            ::windows::core::GetTrustLevel,
            BusinessLandmarkClick::<Impl, IMPL_OFFSET>,
            RemoveBusinessLandmarkClick::<Impl, IMPL_OFFSET>,
            TransitFeatureClick::<Impl, IMPL_OFFSET>,
            RemoveTransitFeatureClick::<Impl, IMPL_OFFSET>,
            BusinessLandmarkRightTapped::<Impl, IMPL_OFFSET>,
            RemoveBusinessLandmarkRightTapped::<Impl, IMPL_OFFSET>,
            TransitFeatureRightTapped::<Impl, IMPL_OFFSET>,
            RemoveTransitFeatureRightTapped::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlDataHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlDataHelper2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlDataHelper2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapControlDataHelper2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlDataHelper2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlDataHelper2Vtbl {
        unsafe extern "system" fn BusinessLandmarkPointerEntered<Impl: IMapControlDataHelper2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarkPointerEntered(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerEnteredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerEnteredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBusinessLandmarkPointerEntered<Impl: IMapControlDataHelper2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkPointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeaturePointerEntered<Impl: IMapControlDataHelper2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitFeaturePointerEntered(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerEnteredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerEnteredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransitFeaturePointerEntered<Impl: IMapControlDataHelper2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeaturePointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BusinessLandmarkPointerExited<Impl: IMapControlDataHelper2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarkPointerExited(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerExitedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerExitedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBusinessLandmarkPointerExited<Impl: IMapControlDataHelper2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkPointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeaturePointerExited<Impl: IMapControlDataHelper2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitFeaturePointerExited(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerExitedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerExitedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransitFeaturePointerExited<Impl: IMapControlDataHelper2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeaturePointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapControlDataHelper2>,
            ::windows::core::GetTrustLevel,
            BusinessLandmarkPointerEntered::<Impl, IMPL_OFFSET>,
            RemoveBusinessLandmarkPointerEntered::<Impl, IMPL_OFFSET>,
            TransitFeaturePointerEntered::<Impl, IMPL_OFFSET>,
            RemoveTransitFeaturePointerEntered::<Impl, IMPL_OFFSET>,
            BusinessLandmarkPointerExited::<Impl, IMPL_OFFSET>,
            RemoveBusinessLandmarkPointerExited::<Impl, IMPL_OFFSET>,
            TransitFeaturePointerExited::<Impl, IMPL_OFFSET>,
            RemoveTransitFeaturePointerExited::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlDataHelper2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlDataHelperFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlDataHelperFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapControlDataHelperFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, map: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&map as *const <MapControl as ::windows::core::Abi>::Abi as *const <MapControl as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlDataHelperFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlDataHelperFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlDataHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlDataHelperStaticsVtbl {
        unsafe extern "system" fn CreateMapControl<Impl: IMapControlDataHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rasterrendermode: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMapControl(rasterrendermode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlDataHelperStatics>, ::windows::core::GetTrustLevel, CreateMapControl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlDataHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapControlStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStaticsVtbl {
        unsafe extern "system" fn CenterProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildrenProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildrenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorSchemeProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorSchemeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredPitchProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredPitchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeadingProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeadingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LandmarksVisibleProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LandmarksVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadingStatusProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadingStatusProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapServiceTokenProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapServiceTokenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PedestrianFeaturesVisibleProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PedestrianFeaturesVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PitchProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PitchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrafficFlowVisibleProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrafficFlowVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformOriginProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformOriginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WatermarkModeProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WatermarkModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevelProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElementsProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElementsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutesProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileSourcesProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileSourcesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocation<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocation(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalizedAnchorPointProperty<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedAnchorPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNormalizedAnchorPoint<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNormalizedAnchorPoint(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedAnchorPoint<Impl: IMapControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalizedAnchorPoint(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapControlStatics>,
            ::windows::core::GetTrustLevel,
            CenterProperty::<Impl, IMPL_OFFSET>,
            ChildrenProperty::<Impl, IMPL_OFFSET>,
            ColorSchemeProperty::<Impl, IMPL_OFFSET>,
            DesiredPitchProperty::<Impl, IMPL_OFFSET>,
            HeadingProperty::<Impl, IMPL_OFFSET>,
            LandmarksVisibleProperty::<Impl, IMPL_OFFSET>,
            LoadingStatusProperty::<Impl, IMPL_OFFSET>,
            MapServiceTokenProperty::<Impl, IMPL_OFFSET>,
            PedestrianFeaturesVisibleProperty::<Impl, IMPL_OFFSET>,
            PitchProperty::<Impl, IMPL_OFFSET>,
            StyleProperty::<Impl, IMPL_OFFSET>,
            TrafficFlowVisibleProperty::<Impl, IMPL_OFFSET>,
            TransformOriginProperty::<Impl, IMPL_OFFSET>,
            WatermarkModeProperty::<Impl, IMPL_OFFSET>,
            ZoomLevelProperty::<Impl, IMPL_OFFSET>,
            MapElementsProperty::<Impl, IMPL_OFFSET>,
            RoutesProperty::<Impl, IMPL_OFFSET>,
            TileSourcesProperty::<Impl, IMPL_OFFSET>,
            LocationProperty::<Impl, IMPL_OFFSET>,
            GetLocation::<Impl, IMPL_OFFSET>,
            SetLocation::<Impl, IMPL_OFFSET>,
            NormalizedAnchorPointProperty::<Impl, IMPL_OFFSET>,
            GetNormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
            SetNormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics2Vtbl {
        unsafe extern "system" fn BusinessLandmarksVisibleProperty<Impl: IMapControlStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarksVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitFeaturesVisibleProperty<Impl: IMapControlStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitFeaturesVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PanInteractionModeProperty<Impl: IMapControlStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PanInteractionModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotateInteractionModeProperty<Impl: IMapControlStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotateInteractionModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TiltInteractionModeProperty<Impl: IMapControlStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TiltInteractionModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomInteractionModeProperty<Impl: IMapControlStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomInteractionModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is3DSupportedProperty<Impl: IMapControlStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Is3DSupportedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStreetsideSupportedProperty<Impl: IMapControlStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStreetsideSupportedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SceneProperty<Impl: IMapControlStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapControlStatics2>,
            ::windows::core::GetTrustLevel,
            BusinessLandmarksVisibleProperty::<Impl, IMPL_OFFSET>,
            TransitFeaturesVisibleProperty::<Impl, IMPL_OFFSET>,
            PanInteractionModeProperty::<Impl, IMPL_OFFSET>,
            RotateInteractionModeProperty::<Impl, IMPL_OFFSET>,
            TiltInteractionModeProperty::<Impl, IMPL_OFFSET>,
            ZoomInteractionModeProperty::<Impl, IMPL_OFFSET>,
            Is3DSupportedProperty::<Impl, IMPL_OFFSET>,
            IsStreetsideSupportedProperty::<Impl, IMPL_OFFSET>,
            SceneProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics4Vtbl {
        unsafe extern "system" fn BusinessLandmarksEnabledProperty<Impl: IMapControlStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessLandmarksEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitFeaturesEnabledProperty<Impl: IMapControlStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitFeaturesEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlStatics4>, ::windows::core::GetTrustLevel, BusinessLandmarksEnabledProperty::<Impl, IMPL_OFFSET>, TransitFeaturesEnabledProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics5Vtbl {
        unsafe extern "system" fn MapProjectionProperty<Impl: IMapControlStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapProjectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleSheetProperty<Impl: IMapControlStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleSheetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewPaddingProperty<Impl: IMapControlStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewPaddingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlStatics5>, ::windows::core::GetTrustLevel, MapProjectionProperty::<Impl, IMPL_OFFSET>, StyleSheetProperty::<Impl, IMPL_OFFSET>, ViewPaddingProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics5 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics6Vtbl {
        unsafe extern "system" fn LayersProperty<Impl: IMapControlStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LayersProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlStatics6>, ::windows::core::GetTrustLevel, LayersProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics6 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics7Vtbl {
        unsafe extern "system" fn RegionProperty<Impl: IMapControlStatics7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlStatics7>, ::windows::core::GetTrustLevel, RegionProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics7 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics8Vtbl {
        unsafe extern "system" fn CanTiltDownProperty<Impl: IMapControlStatics8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanTiltDownProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanTiltUpProperty<Impl: IMapControlStatics8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanTiltUpProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanZoomInProperty<Impl: IMapControlStatics8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanZoomInProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanZoomOutProperty<Impl: IMapControlStatics8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanZoomOutProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlStatics8>, ::windows::core::GetTrustLevel, CanTiltDownProperty::<Impl, IMPL_OFFSET>, CanTiltUpProperty::<Impl, IMPL_OFFSET>, CanZoomInProperty::<Impl, IMPL_OFFSET>, CanZoomOutProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControlTransitFeatureClickEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlTransitFeatureClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeatureClickEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControlTransitFeatureClickEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlTransitFeatureClickEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlTransitFeatureClickEventArgsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeatureClickEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeatureClickEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeatureClickEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlTransitFeatureClickEventArgs>, ::windows::core::GetTrustLevel, DisplayName::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, TransitProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlTransitFeatureClickEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControlTransitFeaturePointerEnteredEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlTransitFeaturePointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeaturePointerEnteredEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControlTransitFeaturePointerEnteredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlTransitFeaturePointerEnteredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlTransitFeaturePointerEnteredEventArgsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeaturePointerEnteredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeaturePointerEnteredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeaturePointerEnteredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlTransitFeaturePointerEnteredEventArgs>, ::windows::core::GetTrustLevel, DisplayName::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, TransitProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlTransitFeaturePointerEnteredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControlTransitFeaturePointerExitedEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlTransitFeaturePointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeaturePointerExitedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControlTransitFeaturePointerExitedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlTransitFeaturePointerExitedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlTransitFeaturePointerExitedEventArgsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeaturePointerExitedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeaturePointerExitedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeaturePointerExitedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlTransitFeaturePointerExitedEventArgs>, ::windows::core::GetTrustLevel, DisplayName::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, TransitProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlTransitFeaturePointerExitedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControlTransitFeatureRightTappedEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlTransitFeatureRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeatureRightTappedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControlTransitFeatureRightTappedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlTransitFeatureRightTappedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlTransitFeatureRightTappedEventArgsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeatureRightTappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeatureRightTappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeatureRightTappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlTransitFeatureRightTappedEventArgs>, ::windows::core::GetTrustLevel, DisplayName::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, TransitProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlTransitFeatureRightTappedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapCustomExperienceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapCustomExperienceVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapCustomExperience>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapCustomExperience as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapCustomExperienceChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapCustomExperienceChangedEventArgsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapCustomExperienceChangedEventArgs>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapCustomExperienceChangedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapCustomExperienceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapCustomExperienceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapCustomExperienceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapCustomExperienceFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapCustomExperienceFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementVtbl {
        unsafe extern "system" fn ZIndex<Impl: IMapElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Impl: IMapElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        unsafe extern "system" fn Visible<Impl: IMapElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: IMapElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElement>, ::windows::core::GetTrustLevel, ZIndex::<Impl, IMPL_OFFSET>, SetZIndex::<Impl, IMPL_OFFSET>, Visible::<Impl, IMPL_OFFSET>, SetVisible::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement2Vtbl {
        unsafe extern "system" fn MapTabIndex<Impl: IMapElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapTabIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapTabIndex<Impl: IMapElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapTabIndex(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElement2>, ::windows::core::GetTrustLevel, MapTabIndex::<Impl, IMPL_OFFSET>, SetMapTabIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement3Vtbl {
        unsafe extern "system" fn MapStyleSheetEntry<Impl: IMapElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapStyleSheetEntry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapStyleSheetEntry<Impl: IMapElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapStyleSheetEntry(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapStyleSheetEntryState<Impl: IMapElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapStyleSheetEntryState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapStyleSheetEntryState<Impl: IMapElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapStyleSheetEntryState(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IMapElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IMapElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapElement3>,
            ::windows::core::GetTrustLevel,
            MapStyleSheetEntry::<Impl, IMPL_OFFSET>,
            SetMapStyleSheetEntry::<Impl, IMPL_OFFSET>,
            MapStyleSheetEntryState::<Impl, IMPL_OFFSET>,
            SetMapStyleSheetEntryState::<Impl, IMPL_OFFSET>,
            Tag::<Impl, IMPL_OFFSET>,
            SetTag::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElement3D {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement3D";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IMapElement3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement3DVtbl {
        unsafe extern "system" fn Location<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Model<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Model() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModel<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModel(&*(&value as *const <MapModel3D as ::windows::core::Abi>::Abi as *const <MapModel3D as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Heading<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Heading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeading<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeading(value).into()
        }
        unsafe extern "system" fn Pitch<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPitch<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPitch(value).into()
        }
        unsafe extern "system" fn Roll<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Roll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoll<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoll(value).into()
        }
        unsafe extern "system" fn Scale<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: IMapElement3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapElement3D>,
            ::windows::core::GetTrustLevel,
            Location::<Impl, IMPL_OFFSET>,
            SetLocation::<Impl, IMPL_OFFSET>,
            Model::<Impl, IMPL_OFFSET>,
            SetModel::<Impl, IMPL_OFFSET>,
            Heading::<Impl, IMPL_OFFSET>,
            SetHeading::<Impl, IMPL_OFFSET>,
            Pitch::<Impl, IMPL_OFFSET>,
            SetPitch::<Impl, IMPL_OFFSET>,
            Roll::<Impl, IMPL_OFFSET>,
            SetRoll::<Impl, IMPL_OFFSET>,
            Scale::<Impl, IMPL_OFFSET>,
            SetScale::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement3D as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement3DStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement3DStaticsVtbl {
        unsafe extern "system" fn LocationProperty<Impl: IMapElement3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeadingProperty<Impl: IMapElement3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeadingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PitchProperty<Impl: IMapElement3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PitchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RollProperty<Impl: IMapElement3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RollProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleProperty<Impl: IMapElement3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElement3DStatics>, ::windows::core::GetTrustLevel, LocationProperty::<Impl, IMPL_OFFSET>, HeadingProperty::<Impl, IMPL_OFFSET>, PitchProperty::<Impl, IMPL_OFFSET>, RollProperty::<Impl, IMPL_OFFSET>, ScaleProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement3DStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement4Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMapElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IMapElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElement4>, ::windows::core::GetTrustLevel, IsEnabled::<Impl, IMPL_OFFSET>, SetIsEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapElementClickEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementClickEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapElementClickEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementClickEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementClickEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementClickEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementClickEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElements<Impl: IMapElementClickEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementClickEventArgs>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, MapElements::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementClickEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapElementFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapElementPointerEnteredEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementPointerEnteredEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapElementPointerEnteredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementPointerEnteredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementPointerEnteredEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementPointerEnteredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementPointerEnteredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElement<Impl: IMapElementPointerEnteredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementPointerEnteredEventArgs>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, MapElement::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementPointerEnteredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapElementPointerExitedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementPointerExitedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapElementPointerExitedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementPointerExitedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementPointerExitedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementPointerExitedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementPointerExitedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElement<Impl: IMapElementPointerExitedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementPointerExitedEventArgs>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, MapElement::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementPointerExitedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementStaticsVtbl {
        unsafe extern "system" fn ZIndexProperty<Impl: IMapElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibleProperty<Impl: IMapElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementStatics>, ::windows::core::GetTrustLevel, ZIndexProperty::<Impl, IMPL_OFFSET>, VisibleProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementStatics2Vtbl {
        unsafe extern "system" fn MapTabIndexProperty<Impl: IMapElementStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapTabIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementStatics2>, ::windows::core::GetTrustLevel, MapTabIndexProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementStatics2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementStatics3Vtbl {
        unsafe extern "system" fn MapStyleSheetEntryProperty<Impl: IMapElementStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapStyleSheetEntryProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapStyleSheetEntryStateProperty<Impl: IMapElementStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapStyleSheetEntryStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TagProperty<Impl: IMapElementStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TagProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementStatics3>, ::windows::core::GetTrustLevel, MapStyleSheetEntryProperty::<Impl, IMPL_OFFSET>, MapStyleSheetEntryStateProperty::<Impl, IMPL_OFFSET>, TagProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementStatics3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementStatics4Vtbl {
        unsafe extern "system" fn IsEnabledProperty<Impl: IMapElementStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementStatics4>, ::windows::core::GetTrustLevel, IsEnabledProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementsLayer {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapElementsLayerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerVtbl {
        unsafe extern "system" fn MapElements<Impl: IMapElementsLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapElements<Impl: IMapElementsLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapElements(&*(&value as *const <super::super::super::super::Foundation::Collections::IVector<MapElement> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVector<MapElement> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementClick<Impl: IMapElementsLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElementClick(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerClickEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerClickEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementClick<Impl: IMapElementsLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerEntered<Impl: IMapElementsLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElementPointerEntered(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerEnteredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerEnteredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementPointerEntered<Impl: IMapElementsLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerExited<Impl: IMapElementsLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElementPointerExited(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerExitedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerExitedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapElementPointerExited<Impl: IMapElementsLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapContextRequested<Impl: IMapElementsLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapContextRequested(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerContextRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerContextRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapContextRequested<Impl: IMapElementsLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapContextRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapElementsLayer>,
            ::windows::core::GetTrustLevel,
            MapElements::<Impl, IMPL_OFFSET>,
            SetMapElements::<Impl, IMPL_OFFSET>,
            MapElementClick::<Impl, IMPL_OFFSET>,
            RemoveMapElementClick::<Impl, IMPL_OFFSET>,
            MapElementPointerEntered::<Impl, IMPL_OFFSET>,
            RemoveMapElementPointerEntered::<Impl, IMPL_OFFSET>,
            MapElementPointerExited::<Impl, IMPL_OFFSET>,
            RemoveMapElementPointerExited::<Impl, IMPL_OFFSET>,
            MapContextRequested::<Impl, IMPL_OFFSET>,
            RemoveMapContextRequested::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapElementsLayerClickEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementsLayerClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerClickEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapElementsLayerClickEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerClickEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerClickEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerClickEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementsLayerClickEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElements<Impl: IMapElementsLayerClickEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementsLayerClickEventArgs>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, MapElements::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayerClickEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapElementsLayerContextRequestedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementsLayerContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerContextRequestedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapElementsLayerContextRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerContextRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerContextRequestedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerContextRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementsLayerContextRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElements<Impl: IMapElementsLayerContextRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementsLayerContextRequestedEventArgs>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, MapElements::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayerContextRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapElementsLayerPointerEnteredEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementsLayerPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerPointerEnteredEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapElementsLayerPointerEnteredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerPointerEnteredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerPointerEnteredEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerPointerEnteredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementsLayerPointerEnteredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElement<Impl: IMapElementsLayerPointerEnteredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementsLayerPointerEnteredEventArgs>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, MapElement::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayerPointerEnteredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapElementsLayerPointerExitedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementsLayerPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerPointerExitedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapElementsLayerPointerExitedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerPointerExitedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerPointerExitedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerPointerExitedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapElementsLayerPointerExitedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapElement<Impl: IMapElementsLayerPointerExitedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementsLayerPointerExitedEventArgs>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>, MapElement::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayerPointerExitedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerStaticsVtbl {
        unsafe extern "system" fn MapElementsProperty<Impl: IMapElementsLayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapElementsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapElementsLayerStatics>, ::windows::core::GetTrustLevel, MapElementsProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapIcon {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapIcon";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMapIconVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapIconImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapIconVtbl {
        unsafe extern "system" fn Location<Impl: IMapIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IMapIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Title<Impl: IMapIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IMapIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalizedAnchorPoint<Impl: IMapIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedAnchorPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedAnchorPoint<Impl: IMapIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalizedAnchorPoint(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IMapIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: IMapIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapIcon>,
            ::windows::core::GetTrustLevel,
            Location::<Impl, IMPL_OFFSET>,
            SetLocation::<Impl, IMPL_OFFSET>,
            Title::<Impl, IMPL_OFFSET>,
            SetTitle::<Impl, IMPL_OFFSET>,
            NormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
            SetNormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
            Image::<Impl, IMPL_OFFSET>,
            SetImage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapIcon as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapIcon2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapIcon2Vtbl {
        unsafe extern "system" fn CollisionBehaviorDesired<Impl: IMapIcon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapElementCollisionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollisionBehaviorDesired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollisionBehaviorDesired<Impl: IMapIcon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapElementCollisionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCollisionBehaviorDesired(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapIcon2>, ::windows::core::GetTrustLevel, CollisionBehaviorDesired::<Impl, IMPL_OFFSET>, SetCollisionBehaviorDesired::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapIcon2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapIconStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapIconStaticsVtbl {
        unsafe extern "system" fn LocationProperty<Impl: IMapIconStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleProperty<Impl: IMapIconStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TitleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedAnchorPointProperty<Impl: IMapIconStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedAnchorPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapIconStatics>, ::windows::core::GetTrustLevel, LocationProperty::<Impl, IMPL_OFFSET>, TitleProperty::<Impl, IMPL_OFFSET>, NormalizedAnchorPointProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapIconStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapIconStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapIconStatics2Vtbl {
        unsafe extern "system" fn CollisionBehaviorDesiredProperty<Impl: IMapIconStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollisionBehaviorDesiredProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapIconStatics2>, ::windows::core::GetTrustLevel, CollisionBehaviorDesiredProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapIconStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapInputEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapInputEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapInputEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapInputEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapInputEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapInputEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapInputEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapInputEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapInputEventArgs>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapInputEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapItemsControlImpl: Sized {
    fn ItemsSource(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetItemsSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>>;
    fn ItemTemplate(&self) -> ::windows::core::Result<super::super::DataTemplate>;
    fn SetItemTemplate(&self, value: &::core::option::Option<super::super::DataTemplate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapItemsControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapItemsControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapItemsControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapItemsControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapItemsControlVtbl {
        unsafe extern "system" fn ItemsSource<Impl: IMapItemsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemsSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemsSource<Impl: IMapItemsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemsSource(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Items<Impl: IMapItemsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemTemplate<Impl: IMapItemsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemTemplate<Impl: IMapItemsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemTemplate(&*(&value as *const <super::super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::super::DataTemplate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapItemsControl>, ::windows::core::GetTrustLevel, ItemsSource::<Impl, IMPL_OFFSET>, SetItemsSource::<Impl, IMPL_OFFSET>, Items::<Impl, IMPL_OFFSET>, ItemTemplate::<Impl, IMPL_OFFSET>, SetItemTemplate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapItemsControl as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapItemsControlStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapItemsControlStaticsVtbl {
        unsafe extern "system" fn ItemsSourceProperty<Impl: IMapItemsControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemsSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemsProperty<Impl: IMapItemsControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemTemplateProperty<Impl: IMapItemsControlStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemTemplateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapItemsControlStatics>, ::windows::core::GetTrustLevel, ItemsSourceProperty::<Impl, IMPL_OFFSET>, ItemsProperty::<Impl, IMPL_OFFSET>, ItemTemplateProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapItemsControlStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapLayerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapLayerVtbl {
        unsafe extern "system" fn MapTabIndex<Impl: IMapLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapTabIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapTabIndex<Impl: IMapLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapTabIndex(value).into()
        }
        unsafe extern "system" fn Visible<Impl: IMapLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: IMapLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        unsafe extern "system" fn ZIndex<Impl: IMapLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Impl: IMapLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapLayer>, ::windows::core::GetTrustLevel, MapTabIndex::<Impl, IMPL_OFFSET>, SetMapTabIndex::<Impl, IMPL_OFFSET>, Visible::<Impl, IMPL_OFFSET>, SetVisible::<Impl, IMPL_OFFSET>, ZIndex::<Impl, IMPL_OFFSET>, SetZIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapLayer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapLayerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapLayerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapLayerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapLayerFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapLayerFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapLayerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapLayerStaticsVtbl {
        unsafe extern "system" fn MapTabIndexProperty<Impl: IMapLayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapTabIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibleProperty<Impl: IMapLayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZIndexProperty<Impl: IMapLayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapLayerStatics>, ::windows::core::GetTrustLevel, MapTabIndexProperty::<Impl, IMPL_OFFSET>, VisibleProperty::<Impl, IMPL_OFFSET>, ZIndexProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapLayerStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapModel3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapModel3DVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapModel3D>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapModel3D as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapModel3DFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapModel3DFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapModel3DFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapModel3DFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapModel3DFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMapModel3DStaticsImpl: Sized {
    fn CreateFrom3MFAsync(&self, source: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>>;
    fn CreateFrom3MFWithShadingOptionAsync(&self, source: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>, shadingoption: MapModel3DShadingOption) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapModel3DStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapModel3DStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMapModel3DStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapModel3DStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapModel3DStaticsVtbl {
        unsafe extern "system" fn CreateFrom3MFAsync<Impl: IMapModel3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFrom3MFAsync(&*(&source as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrom3MFWithShadingOptionAsync<Impl: IMapModel3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, shadingoption: MapModel3DShadingOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFrom3MFWithShadingOptionAsync(&*(&source as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType), shadingoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapModel3DStatics>, ::windows::core::GetTrustLevel, CreateFrom3MFAsync::<Impl, IMPL_OFFSET>, CreateFrom3MFWithShadingOptionAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapModel3DStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapPolygon {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolygon";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapPolygonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapPolygonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapPolygonVtbl {
        unsafe extern "system" fn Path<Impl: IMapPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IMapPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeColor<Impl: IMapPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeColor<Impl: IMapPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeThickness<Impl: IMapPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Impl: IMapPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeThickness(value).into()
        }
        unsafe extern "system" fn StrokeDashed<Impl: IMapPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashed<Impl: IMapPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashed(value).into()
        }
        unsafe extern "system" fn FillColor<Impl: IMapPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillColor<Impl: IMapPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapPolygon>,
            ::windows::core::GetTrustLevel,
            Path::<Impl, IMPL_OFFSET>,
            SetPath::<Impl, IMPL_OFFSET>,
            StrokeColor::<Impl, IMPL_OFFSET>,
            SetStrokeColor::<Impl, IMPL_OFFSET>,
            StrokeThickness::<Impl, IMPL_OFFSET>,
            SetStrokeThickness::<Impl, IMPL_OFFSET>,
            StrokeDashed::<Impl, IMPL_OFFSET>,
            SetStrokeDashed::<Impl, IMPL_OFFSET>,
            FillColor::<Impl, IMPL_OFFSET>,
            SetFillColor::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapPolygon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapPolygon2Impl: Sized {
    fn Paths(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Devices::Geolocation::Geopath>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapPolygon2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolygon2";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapPolygon2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapPolygon2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapPolygon2Vtbl {
        unsafe extern "system" fn Paths<Impl: IMapPolygon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Paths() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapPolygon2>, ::windows::core::GetTrustLevel, Paths::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapPolygon2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapPolygonStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapPolygonStaticsVtbl {
        unsafe extern "system" fn PathProperty<Impl: IMapPolygonStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeThicknessProperty<Impl: IMapPolygonStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeDashedProperty<Impl: IMapPolygonStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapPolygonStatics>, ::windows::core::GetTrustLevel, PathProperty::<Impl, IMPL_OFFSET>, StrokeThicknessProperty::<Impl, IMPL_OFFSET>, StrokeDashedProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapPolygonStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapPolyline {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolyline";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapPolylineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapPolylineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapPolylineVtbl {
        unsafe extern "system" fn Path<Impl: IMapPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IMapPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeColor<Impl: IMapPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeColor<Impl: IMapPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeThickness<Impl: IMapPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Impl: IMapPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeThickness(value).into()
        }
        unsafe extern "system" fn StrokeDashed<Impl: IMapPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashed<Impl: IMapPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashed(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapPolyline>,
            ::windows::core::GetTrustLevel,
            Path::<Impl, IMPL_OFFSET>,
            SetPath::<Impl, IMPL_OFFSET>,
            StrokeColor::<Impl, IMPL_OFFSET>,
            SetStrokeColor::<Impl, IMPL_OFFSET>,
            StrokeThickness::<Impl, IMPL_OFFSET>,
            SetStrokeThickness::<Impl, IMPL_OFFSET>,
            StrokeDashed::<Impl, IMPL_OFFSET>,
            SetStrokeDashed::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapPolyline as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapPolylineStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapPolylineStaticsVtbl {
        unsafe extern "system" fn PathProperty<Impl: IMapPolylineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeDashedProperty<Impl: IMapPolylineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapPolylineStatics>, ::windows::core::GetTrustLevel, PathProperty::<Impl, IMPL_OFFSET>, StrokeDashedProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapPolylineStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapRightTappedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapRightTappedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapRightTappedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapRightTappedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapRightTappedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IMapRightTappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IMapRightTappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapRightTappedEventArgs>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, Location::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapRightTappedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
pub trait IMapRouteViewImpl: Sized {
    fn RouteColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetRouteColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn OutlineColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetOutlineColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn Route(&self) -> ::windows::core::Result<super::super::super::super::Services::Maps::MapRoute>;
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapRouteView {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapRouteView";
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
impl IMapRouteViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapRouteViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapRouteViewVtbl {
        unsafe extern "system" fn RouteColor<Impl: IMapRouteViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RouteColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRouteColor<Impl: IMapRouteViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRouteColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutlineColor<Impl: IMapRouteViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutlineColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutlineColor<Impl: IMapRouteViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutlineColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Route<Impl: IMapRouteViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Route() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapRouteView>, ::windows::core::GetTrustLevel, RouteColor::<Impl, IMPL_OFFSET>, SetRouteColor::<Impl, IMPL_OFFSET>, OutlineColor::<Impl, IMPL_OFFSET>, SetOutlineColor::<Impl, IMPL_OFFSET>, Route::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapRouteView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
pub trait IMapRouteViewFactoryImpl: Sized {
    fn CreateInstanceWithMapRoute(&self, route: &::core::option::Option<super::super::super::super::Services::Maps::MapRoute>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapRouteView>;
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapRouteViewFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapRouteViewFactory";
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
impl IMapRouteViewFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapRouteViewFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapRouteViewFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithMapRoute<Impl: IMapRouteViewFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, route: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithMapRoute(&*(&route as *const <super::super::super::super::Services::Maps::MapRoute as ::windows::core::Abi>::Abi as *const <super::super::super::super::Services::Maps::MapRoute as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapRouteViewFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithMapRoute::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapRouteViewFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapSceneImpl: Sized {
    fn TargetCamera(&self) -> ::windows::core::Result<MapCamera>;
    fn TargetCameraChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapScene, MapTargetCameraChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetCameraChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapScene {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapScene";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapSceneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapSceneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapSceneVtbl {
        unsafe extern "system" fn TargetCamera<Impl: IMapSceneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetCameraChanged<Impl: IMapSceneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetCameraChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<MapScene, MapTargetCameraChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<MapScene, MapTargetCameraChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTargetCameraChanged<Impl: IMapSceneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTargetCameraChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapScene>, ::windows::core::GetTrustLevel, TargetCamera::<Impl, IMPL_OFFSET>, TargetCameraChanged::<Impl, IMPL_OFFSET>, RemoveTargetCameraChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapScene as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapSceneStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapSceneStatics";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapSceneStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapSceneStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapSceneStaticsVtbl {
        unsafe extern "system" fn CreateFromBoundingBox<Impl: IMapSceneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBoundingBox(&*(&bounds as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBoundingBoxWithHeadingAndPitch<Impl: IMapSceneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBoundingBoxWithHeadingAndPitch(&*(&bounds as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromCamera<Impl: IMapSceneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, camera: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromCamera(&*(&camera as *const <MapCamera as ::windows::core::Abi>::Abi as *const <MapCamera as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocation<Impl: IMapSceneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLocation(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocationWithHeadingAndPitch<Impl: IMapSceneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLocationWithHeadingAndPitch(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocationAndRadius<Impl: IMapSceneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLocationAndRadius(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), radiusinmeters) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocationAndRadiusWithHeadingAndPitch<Impl: IMapSceneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLocationAndRadiusWithHeadingAndPitch(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), radiusinmeters, headingindegrees, pitchindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocations<Impl: IMapSceneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLocations(&*(&locations as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLocationsWithHeadingAndPitch<Impl: IMapSceneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locations: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapSceneStatics>,
            ::windows::core::GetTrustLevel,
            CreateFromBoundingBox::<Impl, IMPL_OFFSET>,
            CreateFromBoundingBoxWithHeadingAndPitch::<Impl, IMPL_OFFSET>,
            CreateFromCamera::<Impl, IMPL_OFFSET>,
            CreateFromLocation::<Impl, IMPL_OFFSET>,
            CreateFromLocationWithHeadingAndPitch::<Impl, IMPL_OFFSET>,
            CreateFromLocationAndRadius::<Impl, IMPL_OFFSET>,
            CreateFromLocationAndRadiusWithHeadingAndPitch::<Impl, IMPL_OFFSET>,
            CreateFromLocations::<Impl, IMPL_OFFSET>,
            CreateFromLocationsWithHeadingAndPitch::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapSceneStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapStyleSheetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapStyleSheetVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapStyleSheet>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapStyleSheet as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapStyleSheetEntriesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapStyleSheetEntriesStaticsVtbl {
        unsafe extern "system" fn Area<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Area() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Airport<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Airport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cemetery<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cemetery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Continent<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Continent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Education<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Education() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndigenousPeoplesReserve<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndigenousPeoplesReserve() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Island<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Island() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Medical<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Medical() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Military<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Military() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Nautical<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Nautical() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Neighborhood<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Neighborhood() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Runway<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Runway() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sand<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sand() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShoppingCenter<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShoppingCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stadium<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stadium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vegetation<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Vegetation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forest<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Forest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GolfCourse<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GolfCourse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Park<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Park() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlayingField<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayingField() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserve<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reserve() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Point<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalPoint<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peak<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peak() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolcanicPeak<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolcanicPeak() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaterPoint<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaterPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointOfInterest<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointOfInterest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Business<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Business() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FoodPoint<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FoodPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopulatedPlace<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PopulatedPlace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capital<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capital() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminDistrictCapital<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminDistrictCapital() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryRegionCapital<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryRegionCapital() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadShield<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoadShield() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadExit<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoadExit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transit<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Political<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Political() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryRegion<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminDistrict<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminDistrict() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn District<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).District() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Structure<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Structure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Building<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Building() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EducationBuilding<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EducationBuilding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MedicalBuilding<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MedicalBuilding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitBuilding<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitBuilding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transportation<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transportation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Road<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Road() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlledAccessHighway<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlledAccessHighway() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighSpeedRamp<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighSpeedRamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Highway<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Highway() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorRoad<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorRoad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArterialRoad<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArterialRoad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Street<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Street() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ramp<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ramp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnpavedStreet<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnpavedStreet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TollRoad<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TollRoad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Railway<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Railway() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trail<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaterRoute<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaterRoute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Water<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Water() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn River<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).River() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RouteLine<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RouteLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WalkingRoute<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WalkingRoute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrivingRoute<Impl: IMapStyleSheetEntriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapStyleSheetEntriesStatics>,
            ::windows::core::GetTrustLevel,
            Area::<Impl, IMPL_OFFSET>,
            Airport::<Impl, IMPL_OFFSET>,
            Cemetery::<Impl, IMPL_OFFSET>,
            Continent::<Impl, IMPL_OFFSET>,
            Education::<Impl, IMPL_OFFSET>,
            IndigenousPeoplesReserve::<Impl, IMPL_OFFSET>,
            Island::<Impl, IMPL_OFFSET>,
            Medical::<Impl, IMPL_OFFSET>,
            Military::<Impl, IMPL_OFFSET>,
            Nautical::<Impl, IMPL_OFFSET>,
            Neighborhood::<Impl, IMPL_OFFSET>,
            Runway::<Impl, IMPL_OFFSET>,
            Sand::<Impl, IMPL_OFFSET>,
            ShoppingCenter::<Impl, IMPL_OFFSET>,
            Stadium::<Impl, IMPL_OFFSET>,
            Vegetation::<Impl, IMPL_OFFSET>,
            Forest::<Impl, IMPL_OFFSET>,
            GolfCourse::<Impl, IMPL_OFFSET>,
            Park::<Impl, IMPL_OFFSET>,
            PlayingField::<Impl, IMPL_OFFSET>,
            Reserve::<Impl, IMPL_OFFSET>,
            Point::<Impl, IMPL_OFFSET>,
            NaturalPoint::<Impl, IMPL_OFFSET>,
            Peak::<Impl, IMPL_OFFSET>,
            VolcanicPeak::<Impl, IMPL_OFFSET>,
            WaterPoint::<Impl, IMPL_OFFSET>,
            PointOfInterest::<Impl, IMPL_OFFSET>,
            Business::<Impl, IMPL_OFFSET>,
            FoodPoint::<Impl, IMPL_OFFSET>,
            PopulatedPlace::<Impl, IMPL_OFFSET>,
            Capital::<Impl, IMPL_OFFSET>,
            AdminDistrictCapital::<Impl, IMPL_OFFSET>,
            CountryRegionCapital::<Impl, IMPL_OFFSET>,
            RoadShield::<Impl, IMPL_OFFSET>,
            RoadExit::<Impl, IMPL_OFFSET>,
            Transit::<Impl, IMPL_OFFSET>,
            Political::<Impl, IMPL_OFFSET>,
            CountryRegion::<Impl, IMPL_OFFSET>,
            AdminDistrict::<Impl, IMPL_OFFSET>,
            District::<Impl, IMPL_OFFSET>,
            Structure::<Impl, IMPL_OFFSET>,
            Building::<Impl, IMPL_OFFSET>,
            EducationBuilding::<Impl, IMPL_OFFSET>,
            MedicalBuilding::<Impl, IMPL_OFFSET>,
            TransitBuilding::<Impl, IMPL_OFFSET>,
            Transportation::<Impl, IMPL_OFFSET>,
            Road::<Impl, IMPL_OFFSET>,
            ControlledAccessHighway::<Impl, IMPL_OFFSET>,
            HighSpeedRamp::<Impl, IMPL_OFFSET>,
            Highway::<Impl, IMPL_OFFSET>,
            MajorRoad::<Impl, IMPL_OFFSET>,
            ArterialRoad::<Impl, IMPL_OFFSET>,
            Street::<Impl, IMPL_OFFSET>,
            Ramp::<Impl, IMPL_OFFSET>,
            UnpavedStreet::<Impl, IMPL_OFFSET>,
            TollRoad::<Impl, IMPL_OFFSET>,
            Railway::<Impl, IMPL_OFFSET>,
            Trail::<Impl, IMPL_OFFSET>,
            WaterRoute::<Impl, IMPL_OFFSET>,
            Water::<Impl, IMPL_OFFSET>,
            River::<Impl, IMPL_OFFSET>,
            RouteLine::<Impl, IMPL_OFFSET>,
            WalkingRoute::<Impl, IMPL_OFFSET>,
            DrivingRoute::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapStyleSheetEntriesStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapStyleSheetEntryStatesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapStyleSheetEntryStatesStaticsVtbl {
        unsafe extern "system" fn Disabled<Impl: IMapStyleSheetEntryStatesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hover<Impl: IMapStyleSheetEntryStatesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hover() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Selected<Impl: IMapStyleSheetEntryStatesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapStyleSheetEntryStatesStatics>, ::windows::core::GetTrustLevel, Disabled::<Impl, IMPL_OFFSET>, Hover::<Impl, IMPL_OFFSET>, Selected::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapStyleSheetEntryStatesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapStyleSheetStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapStyleSheetStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapStyleSheetStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapStyleSheetStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapStyleSheetStaticsVtbl {
        unsafe extern "system" fn Aerial<Impl: IMapStyleSheetStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Aerial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AerialWithOverlay<Impl: IMapStyleSheetStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AerialWithOverlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadLight<Impl: IMapStyleSheetStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoadLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadDark<Impl: IMapStyleSheetStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoadDark() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadHighContrastLight<Impl: IMapStyleSheetStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoadHighContrastLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadHighContrastDark<Impl: IMapStyleSheetStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoadHighContrastDark() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Combine<Impl: IMapStyleSheetStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Combine(&*(&stylesheets as *const <super::super::super::super::Foundation::Collections::IIterable<MapStyleSheet> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<MapStyleSheet> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseFromJson<Impl: IMapStyleSheetStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, styleasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParseFromJson(&*(&styleasjson as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParseFromJson<Impl: IMapStyleSheetStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, styleasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, stylesheet: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseFromJson(&*(&styleasjson as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&stylesheet)) {
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
            ::windows::core::GetRuntimeClassName::<IMapStyleSheetStatics>,
            ::windows::core::GetTrustLevel,
            Aerial::<Impl, IMPL_OFFSET>,
            AerialWithOverlay::<Impl, IMPL_OFFSET>,
            RoadLight::<Impl, IMPL_OFFSET>,
            RoadDark::<Impl, IMPL_OFFSET>,
            RoadHighContrastLight::<Impl, IMPL_OFFSET>,
            RoadHighContrastDark::<Impl, IMPL_OFFSET>,
            Combine::<Impl, IMPL_OFFSET>,
            ParseFromJson::<Impl, IMPL_OFFSET>,
            TryParseFromJson::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapStyleSheetStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTargetCameraChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTargetCameraChangedEventArgsVtbl {
        unsafe extern "system" fn Camera<Impl: IMapTargetCameraChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Camera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTargetCameraChangedEventArgs>, ::windows::core::GetTrustLevel, Camera::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTargetCameraChangedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTargetCameraChangedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTargetCameraChangedEventArgs2Vtbl {
        unsafe extern "system" fn ChangeReason<Impl: IMapTargetCameraChangedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTargetCameraChangedEventArgs2>, ::windows::core::GetTrustLevel, ChangeReason::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTargetCameraChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMapTileBitmapRequestImpl: Sized {
    fn PixelData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetPixelData(&self, value: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<MapTileBitmapRequestDeferral>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapTileBitmapRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileBitmapRequest";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMapTileBitmapRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileBitmapRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileBitmapRequestVtbl {
        unsafe extern "system" fn PixelData<Impl: IMapTileBitmapRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelData<Impl: IMapTileBitmapRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPixelData(&*(&value as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMapTileBitmapRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileBitmapRequest>, ::windows::core::GetTrustLevel, PixelData::<Impl, IMPL_OFFSET>, SetPixelData::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileBitmapRequest as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileBitmapRequestDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileBitmapRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IMapTileBitmapRequestDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileBitmapRequestDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileBitmapRequestDeferral as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileBitmapRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileBitmapRequestedEventArgsVtbl {
        unsafe extern "system" fn X<Impl: IMapTileBitmapRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Y<Impl: IMapTileBitmapRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Y() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevel<Impl: IMapTileBitmapRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Impl: IMapTileBitmapRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileBitmapRequestedEventArgs>, ::windows::core::GetTrustLevel, X::<Impl, IMPL_OFFSET>, Y::<Impl, IMPL_OFFSET>, ZoomLevel::<Impl, IMPL_OFFSET>, Request::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileBitmapRequestedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileBitmapRequestedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileBitmapRequestedEventArgs2Vtbl {
        unsafe extern "system" fn FrameIndex<Impl: IMapTileBitmapRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileBitmapRequestedEventArgs2>, ::windows::core::GetTrustLevel, FrameIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileBitmapRequestedEventArgs2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileDataSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileDataSourceVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileDataSource>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileDataSource as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileDataSourceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileDataSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapTileDataSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileDataSourceFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileDataSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapTileSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSource";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapTileSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileSourceVtbl {
        unsafe extern "system" fn DataSource<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSource<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataSource(&*(&value as *const <MapTileDataSource as ::windows::core::Abi>::Abi as *const <MapTileDataSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Layer<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapTileLayer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Layer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLayer<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapTileLayer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLayer(value).into()
        }
        unsafe extern "system" fn ZoomLevelRange<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapZoomLevelRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevelRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoomLevelRange<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapZoomLevelRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoomLevelRange(&*(&value as *const <MapZoomLevelRange as ::windows::core::Abi>::Abi as *const <MapZoomLevelRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bounds<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounds<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBounds(&*(&value as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowOverstretch<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOverstretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOverstretch<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowOverstretch(value).into()
        }
        unsafe extern "system" fn IsFadingEnabled<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFadingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFadingEnabled<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsFadingEnabled(value).into()
        }
        unsafe extern "system" fn IsTransparencyEnabled<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransparencyEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTransparencyEnabled<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTransparencyEnabled(value).into()
        }
        unsafe extern "system" fn IsRetryEnabled<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRetryEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRetryEnabled<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRetryEnabled(value).into()
        }
        unsafe extern "system" fn ZIndex<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        unsafe extern "system" fn TilePixelSize<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TilePixelSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTilePixelSize<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTilePixelSize(value).into()
        }
        unsafe extern "system" fn Visible<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: IMapTileSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapTileSource>,
            ::windows::core::GetTrustLevel,
            DataSource::<Impl, IMPL_OFFSET>,
            SetDataSource::<Impl, IMPL_OFFSET>,
            Layer::<Impl, IMPL_OFFSET>,
            SetLayer::<Impl, IMPL_OFFSET>,
            ZoomLevelRange::<Impl, IMPL_OFFSET>,
            SetZoomLevelRange::<Impl, IMPL_OFFSET>,
            Bounds::<Impl, IMPL_OFFSET>,
            SetBounds::<Impl, IMPL_OFFSET>,
            AllowOverstretch::<Impl, IMPL_OFFSET>,
            SetAllowOverstretch::<Impl, IMPL_OFFSET>,
            IsFadingEnabled::<Impl, IMPL_OFFSET>,
            SetIsFadingEnabled::<Impl, IMPL_OFFSET>,
            IsTransparencyEnabled::<Impl, IMPL_OFFSET>,
            SetIsTransparencyEnabled::<Impl, IMPL_OFFSET>,
            IsRetryEnabled::<Impl, IMPL_OFFSET>,
            SetIsRetryEnabled::<Impl, IMPL_OFFSET>,
            ZIndex::<Impl, IMPL_OFFSET>,
            SetZIndex::<Impl, IMPL_OFFSET>,
            TilePixelSize::<Impl, IMPL_OFFSET>,
            SetTilePixelSize::<Impl, IMPL_OFFSET>,
            Visible::<Impl, IMPL_OFFSET>,
            SetVisible::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapTileSource2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSource2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapTileSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileSource2Vtbl {
        unsafe extern "system" fn AnimationState<Impl: IMapTileSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapTileAnimationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnimationState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoPlay<Impl: IMapTileSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoPlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoPlay<Impl: IMapTileSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoPlay(value).into()
        }
        unsafe extern "system" fn FrameCount<Impl: IMapTileSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameCount<Impl: IMapTileSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameCount(value).into()
        }
        unsafe extern "system" fn FrameDuration<Impl: IMapTileSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameDuration<Impl: IMapTileSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameDuration(&*(&value as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pause<Impl: IMapTileSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Play<Impl: IMapTileSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play().into()
        }
        unsafe extern "system" fn Stop<Impl: IMapTileSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapTileSource2>,
            ::windows::core::GetTrustLevel,
            AnimationState::<Impl, IMPL_OFFSET>,
            AutoPlay::<Impl, IMPL_OFFSET>,
            SetAutoPlay::<Impl, IMPL_OFFSET>,
            FrameCount::<Impl, IMPL_OFFSET>,
            SetFrameCount::<Impl, IMPL_OFFSET>,
            FrameDuration::<Impl, IMPL_OFFSET>,
            SetFrameDuration::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            Play::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IMapTileSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSource(&self, datasource: &::core::option::Option<MapTileDataSource>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceAndZoomRange(&self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceZoomRangeAndBounds(&self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize(&self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, tilesizeinpixels: i32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapTileSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSourceFactory";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapTileSourceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileSourceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapTileSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithDataSource<Impl: IMapTileSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDataSource(&*(&datasource as *const <MapTileDataSource as ::windows::core::Abi>::Abi as *const <MapTileDataSource as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithDataSourceAndZoomRange<Impl: IMapTileSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn CreateInstanceWithDataSourceZoomRangeAndBounds<Impl: IMapTileSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, bounds: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize<Impl: IMapTileSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, bounds: ::windows::core::RawPtr, tilesizeinpixels: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapTileSourceFactory>,
            ::windows::core::GetTrustLevel,
            CreateInstance::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDataSource::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDataSourceAndZoomRange::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDataSourceZoomRangeAndBounds::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileSourceFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileSourceStaticsVtbl {
        unsafe extern "system" fn DataSourceProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LayerProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LayerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevelRangeProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevelRangeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundsProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowOverstretchProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOverstretchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFadingEnabledProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFadingEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransparencyEnabledProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransparencyEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRetryEnabledProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRetryEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZIndexProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TilePixelSizeProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TilePixelSizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibleProperty<Impl: IMapTileSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMapTileSourceStatics>,
            ::windows::core::GetTrustLevel,
            DataSourceProperty::<Impl, IMPL_OFFSET>,
            LayerProperty::<Impl, IMPL_OFFSET>,
            ZoomLevelRangeProperty::<Impl, IMPL_OFFSET>,
            BoundsProperty::<Impl, IMPL_OFFSET>,
            AllowOverstretchProperty::<Impl, IMPL_OFFSET>,
            IsFadingEnabledProperty::<Impl, IMPL_OFFSET>,
            IsTransparencyEnabledProperty::<Impl, IMPL_OFFSET>,
            IsRetryEnabledProperty::<Impl, IMPL_OFFSET>,
            ZIndexProperty::<Impl, IMPL_OFFSET>,
            TilePixelSizeProperty::<Impl, IMPL_OFFSET>,
            VisibleProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileSourceStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileSourceStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileSourceStatics2Vtbl {
        unsafe extern "system" fn AnimationStateProperty<Impl: IMapTileSourceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnimationStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoPlayProperty<Impl: IMapTileSourceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoPlayProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameCountProperty<Impl: IMapTileSourceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameCountProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameDurationProperty<Impl: IMapTileSourceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameDurationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileSourceStatics2>, ::windows::core::GetTrustLevel, AnimationStateProperty::<Impl, IMPL_OFFSET>, AutoPlayProperty::<Impl, IMPL_OFFSET>, FrameCountProperty::<Impl, IMPL_OFFSET>, FrameDurationProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileSourceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapTileUriRequestImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<MapTileUriRequestDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapTileUriRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileUriRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapTileUriRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileUriRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileUriRequestVtbl {
        unsafe extern "system" fn Uri<Impl: IMapTileUriRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: IMapTileUriRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMapTileUriRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileUriRequest>, ::windows::core::GetTrustLevel, Uri::<Impl, IMPL_OFFSET>, SetUri::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileUriRequest as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileUriRequestDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileUriRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IMapTileUriRequestDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileUriRequestDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileUriRequestDeferral as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileUriRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileUriRequestedEventArgsVtbl {
        unsafe extern "system" fn X<Impl: IMapTileUriRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Y<Impl: IMapTileUriRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Y() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevel<Impl: IMapTileUriRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Impl: IMapTileUriRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileUriRequestedEventArgs>, ::windows::core::GetTrustLevel, X::<Impl, IMPL_OFFSET>, Y::<Impl, IMPL_OFFSET>, ZoomLevel::<Impl, IMPL_OFFSET>, Request::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileUriRequestedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileUriRequestedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileUriRequestedEventArgs2Vtbl {
        unsafe extern "system" fn FrameIndex<Impl: IMapTileUriRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapTileUriRequestedEventArgs2>, ::windows::core::GetTrustLevel, FrameIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileUriRequestedEventArgs2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreetsideExperienceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreetsideExperienceVtbl {
        unsafe extern "system" fn AddressTextVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressTextVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddressTextVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddressTextVisible(value).into()
        }
        unsafe extern "system" fn CursorVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CursorVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCursorVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCursorVisible(value).into()
        }
        unsafe extern "system" fn OverviewMapVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverviewMapVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverviewMapVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverviewMapVisible(value).into()
        }
        unsafe extern "system" fn StreetLabelsVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreetLabelsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreetLabelsVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreetLabelsVisible(value).into()
        }
        unsafe extern "system" fn ExitButtonVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitButtonVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitButtonVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitButtonVisible(value).into()
        }
        unsafe extern "system" fn ZoomButtonsVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomButtonsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoomButtonsVisible<Impl: IStreetsideExperienceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoomButtonsVisible(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStreetsideExperience>,
            ::windows::core::GetTrustLevel,
            AddressTextVisible::<Impl, IMPL_OFFSET>,
            SetAddressTextVisible::<Impl, IMPL_OFFSET>,
            CursorVisible::<Impl, IMPL_OFFSET>,
            SetCursorVisible::<Impl, IMPL_OFFSET>,
            OverviewMapVisible::<Impl, IMPL_OFFSET>,
            SetOverviewMapVisible::<Impl, IMPL_OFFSET>,
            StreetLabelsVisible::<Impl, IMPL_OFFSET>,
            SetStreetLabelsVisible::<Impl, IMPL_OFFSET>,
            ExitButtonVisible::<Impl, IMPL_OFFSET>,
            SetExitButtonVisible::<Impl, IMPL_OFFSET>,
            ZoomButtonsVisible::<Impl, IMPL_OFFSET>,
            SetZoomButtonsVisible::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreetsideExperience as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreetsideExperienceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreetsideExperienceFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithPanorama<Impl: IStreetsideExperienceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panorama: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithPanorama(&*(&panorama as *const <StreetsidePanorama as ::windows::core::Abi>::Abi as *const <StreetsidePanorama as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithPanoramaHeadingPitchAndFieldOfView<Impl: IStreetsideExperienceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panorama: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, fieldofviewindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithPanoramaHeadingPitchAndFieldOfView(&*(&panorama as *const <StreetsidePanorama as ::windows::core::Abi>::Abi as *const <StreetsidePanorama as ::windows::core::DefaultType>::DefaultType), headingindegrees, pitchindegrees, fieldofviewindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreetsideExperienceFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithPanorama::<Impl, IMPL_OFFSET>, CreateInstanceWithPanoramaHeadingPitchAndFieldOfView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreetsideExperienceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IStreetsidePanoramaImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreetsidePanorama {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IStreetsidePanorama";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IStreetsidePanoramaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreetsidePanoramaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreetsidePanoramaVtbl {
        unsafe extern "system" fn Location<Impl: IStreetsidePanoramaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreetsidePanorama>, ::windows::core::GetTrustLevel, Location::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreetsidePanorama as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreetsidePanoramaStaticsImpl: Sized {
    fn FindNearbyWithLocationAsync(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>>;
    fn FindNearbyWithLocationAndRadiusAsync(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, radiusinmeters: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreetsidePanoramaStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IStreetsidePanoramaStatics";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IStreetsidePanoramaStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreetsidePanoramaStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreetsidePanoramaStaticsVtbl {
        unsafe extern "system" fn FindNearbyWithLocationAsync<Impl: IStreetsidePanoramaStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNearbyWithLocationAsync(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNearbyWithLocationAndRadiusAsync<Impl: IStreetsidePanoramaStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNearbyWithLocationAndRadiusAsync(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), radiusinmeters) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreetsidePanoramaStatics>, ::windows::core::GetTrustLevel, FindNearbyWithLocationAsync::<Impl, IMPL_OFFSET>, FindNearbyWithLocationAndRadiusAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreetsidePanoramaStatics as ::windows::core::Interface>::IID
    }
}
