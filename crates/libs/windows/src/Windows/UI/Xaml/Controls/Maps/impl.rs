#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICustomMapTileDataSource_Impl: Sized {
    fn BitmapRequested(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CustomMapTileDataSource, MapTileBitmapRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBitmapRequested(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICustomMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.ICustomMapTileDataSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICustomMapTileDataSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomMapTileDataSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomMapTileDataSource_Vtbl {
        unsafe extern "system" fn BitmapRequested<Impl: ICustomMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBitmapRequested<Impl: ICustomMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBitmapRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomMapTileDataSource, BASE_OFFSET>(),
            BitmapRequested: BitmapRequested::<Impl, IMPL_OFFSET>,
            RemoveBitmapRequested: RemoveBitmapRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomMapTileDataSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomMapTileDataSourceFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CustomMapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomMapTileDataSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.ICustomMapTileDataSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomMapTileDataSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomMapTileDataSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomMapTileDataSourceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICustomMapTileDataSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomMapTileDataSourceFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomMapTileDataSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpMapTileDataSource_Impl: Sized {
    fn UriFormatString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUriFormatString(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AdditionalRequestHeaders(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn AllowCaching(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowCaching(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn UriRequested(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<HttpMapTileDataSource, MapTileUriRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUriRequested(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IHttpMapTileDataSource";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpMapTileDataSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMapTileDataSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMapTileDataSource_Vtbl {
        unsafe extern "system" fn UriFormatString<Impl: IHttpMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUriFormatString<Impl: IHttpMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUriFormatString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdditionalRequestHeaders<Impl: IHttpMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AllowCaching<Impl: IHttpMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowCaching<Impl: IHttpMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowCaching(value).into()
        }
        unsafe extern "system" fn UriRequested<Impl: IHttpMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUriRequested<Impl: IHttpMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUriRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpMapTileDataSource, BASE_OFFSET>(),
            UriFormatString: UriFormatString::<Impl, IMPL_OFFSET>,
            SetUriFormatString: SetUriFormatString::<Impl, IMPL_OFFSET>,
            AdditionalRequestHeaders: AdditionalRequestHeaders::<Impl, IMPL_OFFSET>,
            AllowCaching: AllowCaching::<Impl, IMPL_OFFSET>,
            SetAllowCaching: SetAllowCaching::<Impl, IMPL_OFFSET>,
            UriRequested: UriRequested::<Impl, IMPL_OFFSET>,
            RemoveUriRequested: RemoveUriRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMapTileDataSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMapTileDataSourceFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HttpMapTileDataSource>;
    fn CreateInstanceWithUriFormatString(&mut self, uriformatstring: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HttpMapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMapTileDataSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IHttpMapTileDataSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMapTileDataSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMapTileDataSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMapTileDataSourceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IHttpMapTileDataSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithUriFormatString<Impl: IHttpMapTileDataSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriformatstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpMapTileDataSourceFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateInstanceWithUriFormatString: CreateInstanceWithUriFormatString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMapTileDataSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILocalMapTileDataSource_Impl: Sized {
    fn UriFormatString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUriFormatString(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UriRequested(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<LocalMapTileDataSource, MapTileUriRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUriRequested(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILocalMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.ILocalMapTileDataSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILocalMapTileDataSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalMapTileDataSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalMapTileDataSource_Vtbl {
        unsafe extern "system" fn UriFormatString<Impl: ILocalMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUriFormatString<Impl: ILocalMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUriFormatString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UriRequested<Impl: ILocalMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUriRequested<Impl: ILocalMapTileDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUriRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILocalMapTileDataSource, BASE_OFFSET>(),
            UriFormatString: UriFormatString::<Impl, IMPL_OFFSET>,
            SetUriFormatString: SetUriFormatString::<Impl, IMPL_OFFSET>,
            UriRequested: UriRequested::<Impl, IMPL_OFFSET>,
            RemoveUriRequested: RemoveUriRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalMapTileDataSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalMapTileDataSourceFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<LocalMapTileDataSource>;
    fn CreateInstanceWithUriFormatString(&mut self, uriformatstring: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<LocalMapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalMapTileDataSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.ILocalMapTileDataSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalMapTileDataSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalMapTileDataSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalMapTileDataSourceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ILocalMapTileDataSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithUriFormatString<Impl: ILocalMapTileDataSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriformatstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILocalMapTileDataSourceFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateInstanceWithUriFormatString: CreateInstanceWithUriFormatString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalMapTileDataSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangedEventArgs_Impl: Sized {
    fn Camera(&mut self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapActualCameraChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapActualCameraChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapActualCameraChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapActualCameraChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapActualCameraChangedEventArgs_Vtbl {
        unsafe extern "system" fn Camera<Impl: IMapActualCameraChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapActualCameraChangedEventArgs, BASE_OFFSET>(), Camera: Camera::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapActualCameraChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangedEventArgs2_Impl: Sized {
    fn ChangeReason(&mut self) -> ::windows::core::Result<MapCameraChangeReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapActualCameraChangedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapActualCameraChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapActualCameraChangedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapActualCameraChangedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapActualCameraChangedEventArgs2_Vtbl {
        unsafe extern "system" fn ChangeReason<Impl: IMapActualCameraChangedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapActualCameraChangedEventArgs2, BASE_OFFSET>(),
            ChangeReason: ChangeReason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapActualCameraChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangingEventArgs_Impl: Sized {
    fn Camera(&mut self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapActualCameraChangingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapActualCameraChangingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapActualCameraChangingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapActualCameraChangingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapActualCameraChangingEventArgs_Vtbl {
        unsafe extern "system" fn Camera<Impl: IMapActualCameraChangingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapActualCameraChangingEventArgs, BASE_OFFSET>(), Camera: Camera::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapActualCameraChangingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangingEventArgs2_Impl: Sized {
    fn ChangeReason(&mut self) -> ::windows::core::Result<MapCameraChangeReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapActualCameraChangingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapActualCameraChangingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapActualCameraChangingEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapActualCameraChangingEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapActualCameraChangingEventArgs2_Vtbl {
        unsafe extern "system" fn ChangeReason<Impl: IMapActualCameraChangingEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapActualCameraChangingEventArgs2, BASE_OFFSET>(),
            ChangeReason: ChangeReason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapActualCameraChangingEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMapBillboard_Impl: Sized {
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&mut self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn NormalizedAnchorPoint(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetNormalizedAnchorPoint(&mut self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Image(&mut self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetImage(&mut self, value: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn CollisionBehaviorDesired(&mut self) -> ::windows::core::Result<MapElementCollisionBehavior>;
    fn SetCollisionBehaviorDesired(&mut self, value: MapElementCollisionBehavior) -> ::windows::core::Result<()>;
    fn ReferenceCamera(&mut self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapBillboard {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapBillboard";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMapBillboard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapBillboard_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapBillboard_Vtbl {
        unsafe extern "system" fn Location<Impl: IMapBillboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocation<Impl: IMapBillboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalizedAnchorPoint<Impl: IMapBillboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNormalizedAnchorPoint<Impl: IMapBillboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalizedAnchorPoint(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IMapBillboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetImage<Impl: IMapBillboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CollisionBehaviorDesired<Impl: IMapBillboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapElementCollisionBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCollisionBehaviorDesired<Impl: IMapBillboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapElementCollisionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCollisionBehaviorDesired(value).into()
        }
        unsafe extern "system" fn ReferenceCamera<Impl: IMapBillboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapBillboard, BASE_OFFSET>(),
            Location: Location::<Impl, IMPL_OFFSET>,
            SetLocation: SetLocation::<Impl, IMPL_OFFSET>,
            NormalizedAnchorPoint: NormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
            SetNormalizedAnchorPoint: SetNormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            SetImage: SetImage::<Impl, IMPL_OFFSET>,
            CollisionBehaviorDesired: CollisionBehaviorDesired::<Impl, IMPL_OFFSET>,
            SetCollisionBehaviorDesired: SetCollisionBehaviorDesired::<Impl, IMPL_OFFSET>,
            ReferenceCamera: ReferenceCamera::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapBillboard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapBillboardFactory_Impl: Sized {
    fn CreateInstanceFromCamera(&mut self, camera: &::core::option::Option<MapCamera>) -> ::windows::core::Result<MapBillboard>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapBillboardFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapBillboardFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapBillboardFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapBillboardFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapBillboardFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceFromCamera<Impl: IMapBillboardFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, camera: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapBillboardFactory, BASE_OFFSET>(),
            CreateInstanceFromCamera: CreateInstanceFromCamera::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapBillboardFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapBillboardStatics_Impl: Sized {
    fn LocationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn NormalizedAnchorPointProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CollisionBehaviorDesiredProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapBillboardStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapBillboardStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapBillboardStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapBillboardStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapBillboardStatics_Vtbl {
        unsafe extern "system" fn LocationProperty<Impl: IMapBillboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NormalizedAnchorPointProperty<Impl: IMapBillboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CollisionBehaviorDesiredProperty<Impl: IMapBillboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapBillboardStatics, BASE_OFFSET>(),
            LocationProperty: LocationProperty::<Impl, IMPL_OFFSET>,
            NormalizedAnchorPointProperty: NormalizedAnchorPointProperty::<Impl, IMPL_OFFSET>,
            CollisionBehaviorDesiredProperty: CollisionBehaviorDesiredProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapBillboardStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IMapCamera_Impl: Sized {
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&mut self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Heading(&mut self) -> ::windows::core::Result<f64>;
    fn SetHeading(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Pitch(&mut self) -> ::windows::core::Result<f64>;
    fn SetPitch(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Roll(&mut self) -> ::windows::core::Result<f64>;
    fn SetRoll(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn FieldOfView(&mut self) -> ::windows::core::Result<f64>;
    fn SetFieldOfView(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapCamera {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCamera";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapCamera_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapCamera_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapCamera_Vtbl {
        unsafe extern "system" fn Location<Impl: IMapCamera_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocation<Impl: IMapCamera_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Heading<Impl: IMapCamera_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHeading<Impl: IMapCamera_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeading(value).into()
        }
        unsafe extern "system" fn Pitch<Impl: IMapCamera_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPitch<Impl: IMapCamera_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPitch(value).into()
        }
        unsafe extern "system" fn Roll<Impl: IMapCamera_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRoll<Impl: IMapCamera_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoll(value).into()
        }
        unsafe extern "system" fn FieldOfView<Impl: IMapCamera_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFieldOfView<Impl: IMapCamera_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFieldOfView(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapCamera, BASE_OFFSET>(),
            Location: Location::<Impl, IMPL_OFFSET>,
            SetLocation: SetLocation::<Impl, IMPL_OFFSET>,
            Heading: Heading::<Impl, IMPL_OFFSET>,
            SetHeading: SetHeading::<Impl, IMPL_OFFSET>,
            Pitch: Pitch::<Impl, IMPL_OFFSET>,
            SetPitch: SetPitch::<Impl, IMPL_OFFSET>,
            Roll: Roll::<Impl, IMPL_OFFSET>,
            SetRoll: SetRoll::<Impl, IMPL_OFFSET>,
            FieldOfView: FieldOfView::<Impl, IMPL_OFFSET>,
            SetFieldOfView: SetFieldOfView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapCamera as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IMapCameraFactory_Impl: Sized {
    fn CreateInstanceWithLocation(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationAndHeading(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationHeadingAndPitch(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationHeadingPitchRollAndFieldOfView(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64, rollindegrees: f64, fieldofviewindegrees: f64) -> ::windows::core::Result<MapCamera>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapCameraFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCameraFactory";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapCameraFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapCameraFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapCameraFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithLocation<Impl: IMapCameraFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithLocationAndHeading<Impl: IMapCameraFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithLocationHeadingAndPitch<Impl: IMapCameraFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithLocationHeadingPitchRollAndFieldOfView<Impl: IMapCameraFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, rollindegrees: f64, fieldofviewindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapCameraFactory, BASE_OFFSET>(),
            CreateInstanceWithLocation: CreateInstanceWithLocation::<Impl, IMPL_OFFSET>,
            CreateInstanceWithLocationAndHeading: CreateInstanceWithLocationAndHeading::<Impl, IMPL_OFFSET>,
            CreateInstanceWithLocationHeadingAndPitch: CreateInstanceWithLocationHeadingAndPitch::<Impl, IMPL_OFFSET>,
            CreateInstanceWithLocationHeadingPitchRollAndFieldOfView: CreateInstanceWithLocationHeadingPitchRollAndFieldOfView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapCameraFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapContextRequestedEventArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapContextRequestedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapContextRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapContextRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapContextRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IMapContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapElements<Impl: IMapContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapContextRequestedEventArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            MapElements: MapElements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapContextRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControl_Impl: Sized {
    fn Center(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetCenter(&mut self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Children(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>>;
    fn ColorScheme(&mut self) -> ::windows::core::Result<MapColorScheme>;
    fn SetColorScheme(&mut self, value: MapColorScheme) -> ::windows::core::Result<()>;
    fn DesiredPitch(&mut self) -> ::windows::core::Result<f64>;
    fn SetDesiredPitch(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Heading(&mut self) -> ::windows::core::Result<f64>;
    fn SetHeading(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn LandmarksVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetLandmarksVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn LoadingStatus(&mut self) -> ::windows::core::Result<MapLoadingStatus>;
    fn MapServiceToken(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMapServiceToken(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MaxZoomLevel(&mut self) -> ::windows::core::Result<f64>;
    fn MinZoomLevel(&mut self) -> ::windows::core::Result<f64>;
    fn PedestrianFeaturesVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetPedestrianFeaturesVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Pitch(&mut self) -> ::windows::core::Result<f64>;
    fn Style(&mut self) -> ::windows::core::Result<MapStyle>;
    fn SetStyle(&mut self, value: MapStyle) -> ::windows::core::Result<()>;
    fn TrafficFlowVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetTrafficFlowVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TransformOrigin(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetTransformOrigin(&mut self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn WatermarkMode(&mut self) -> ::windows::core::Result<MapWatermarkMode>;
    fn SetWatermarkMode(&mut self, value: MapWatermarkMode) -> ::windows::core::Result<()>;
    fn ZoomLevel(&mut self) -> ::windows::core::Result<f64>;
    fn SetZoomLevel(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn MapElements(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
    fn Routes(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapRouteView>>;
    fn TileSources(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapTileSource>>;
    fn CenterChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCenterChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HeadingChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadingChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LoadingStatusChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoadingStatusChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapDoubleTapped(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapDoubleTapped(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapHolding(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapHolding(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapTapped(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapTapped(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PitchChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePitchChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransformOriginChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransformOriginChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ZoomLevelChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveZoomLevelChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindMapElementsAtOffset(&mut self, offset: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
    fn GetLocationFromOffset(&mut self, offset: &super::super::super::super::Foundation::Point, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn GetOffsetFromLocation(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, offset: &mut super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsLocationInView(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, isinview: &mut bool) -> ::windows::core::Result<()>;
    fn TrySetViewBoundsAsync(&mut self, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, margin: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::Thickness>>, animation: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterAsync(&mut self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterAndZoomAsync(&mut self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, zoomlevel: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterZoomHeadingAndPitchAsync(&mut self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, zoomlevel: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, heading: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, desiredpitch: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync(&mut self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, zoomlevel: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, heading: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, desiredpitch: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, animation: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl_Vtbl {
        unsafe extern "system" fn Center<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCenter<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Children<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ColorScheme<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapColorScheme) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColorScheme<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapColorScheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorScheme(value).into()
        }
        unsafe extern "system" fn DesiredPitch<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredPitch<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredPitch(value).into()
        }
        unsafe extern "system" fn Heading<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHeading<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeading(value).into()
        }
        unsafe extern "system" fn LandmarksVisible<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLandmarksVisible<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLandmarksVisible(value).into()
        }
        unsafe extern "system" fn LoadingStatus<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapLoadingStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapServiceToken<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMapServiceToken<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapServiceToken(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxZoomLevel<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinZoomLevel<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PedestrianFeaturesVisible<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPedestrianFeaturesVisible<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPedestrianFeaturesVisible(value).into()
        }
        unsafe extern "system" fn Pitch<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Style<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapStyle) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStyle<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyle(value).into()
        }
        unsafe extern "system" fn TrafficFlowVisible<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTrafficFlowVisible<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrafficFlowVisible(value).into()
        }
        unsafe extern "system" fn TransformOrigin<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransformOrigin<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformOrigin(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WatermarkMode<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapWatermarkMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWatermarkMode<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapWatermarkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWatermarkMode(value).into()
        }
        unsafe extern "system" fn ZoomLevel<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetZoomLevel<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoomLevel(value).into()
        }
        unsafe extern "system" fn MapElements<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Routes<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TileSources<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CenterChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCenterChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCenterChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HeadingChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveHeadingChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHeadingChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoadingStatusChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveLoadingStatusChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoadingStatusChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapDoubleTapped<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapDoubleTapped<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapDoubleTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapHolding<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapHolding<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapHolding(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapTapped<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapTapped<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PitchChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePitchChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePitchChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformOriginChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTransformOriginChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransformOriginChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ZoomLevelChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveZoomLevelChanged<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveZoomLevelChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindMapElementsAtOffset<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLocationFromOffset<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, location: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocationFromOffset(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&location)).into()
        }
        unsafe extern "system" fn GetOffsetFromLocation<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, offset: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOffsetFromLocation(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn IsLocationInView<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, isinview: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsLocationInView(&*(&location as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isinview)).into()
        }
        unsafe extern "system" fn TrySetViewBoundsAsync<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, margin: ::windows::core::RawPtr, animation: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetViewWithCenterAsync<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetViewWithCenterAndZoomAsync<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetViewWithCenterZoomHeadingAndPitchAsync<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, heading: ::windows::core::RawPtr, desiredpitch: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync<Impl: IMapControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, heading: ::windows::core::RawPtr, desiredpitch: ::windows::core::RawPtr, animation: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControl, BASE_OFFSET>(),
            Center: Center::<Impl, IMPL_OFFSET>,
            SetCenter: SetCenter::<Impl, IMPL_OFFSET>,
            Children: Children::<Impl, IMPL_OFFSET>,
            ColorScheme: ColorScheme::<Impl, IMPL_OFFSET>,
            SetColorScheme: SetColorScheme::<Impl, IMPL_OFFSET>,
            DesiredPitch: DesiredPitch::<Impl, IMPL_OFFSET>,
            SetDesiredPitch: SetDesiredPitch::<Impl, IMPL_OFFSET>,
            Heading: Heading::<Impl, IMPL_OFFSET>,
            SetHeading: SetHeading::<Impl, IMPL_OFFSET>,
            LandmarksVisible: LandmarksVisible::<Impl, IMPL_OFFSET>,
            SetLandmarksVisible: SetLandmarksVisible::<Impl, IMPL_OFFSET>,
            LoadingStatus: LoadingStatus::<Impl, IMPL_OFFSET>,
            MapServiceToken: MapServiceToken::<Impl, IMPL_OFFSET>,
            SetMapServiceToken: SetMapServiceToken::<Impl, IMPL_OFFSET>,
            MaxZoomLevel: MaxZoomLevel::<Impl, IMPL_OFFSET>,
            MinZoomLevel: MinZoomLevel::<Impl, IMPL_OFFSET>,
            PedestrianFeaturesVisible: PedestrianFeaturesVisible::<Impl, IMPL_OFFSET>,
            SetPedestrianFeaturesVisible: SetPedestrianFeaturesVisible::<Impl, IMPL_OFFSET>,
            Pitch: Pitch::<Impl, IMPL_OFFSET>,
            Style: Style::<Impl, IMPL_OFFSET>,
            SetStyle: SetStyle::<Impl, IMPL_OFFSET>,
            TrafficFlowVisible: TrafficFlowVisible::<Impl, IMPL_OFFSET>,
            SetTrafficFlowVisible: SetTrafficFlowVisible::<Impl, IMPL_OFFSET>,
            TransformOrigin: TransformOrigin::<Impl, IMPL_OFFSET>,
            SetTransformOrigin: SetTransformOrigin::<Impl, IMPL_OFFSET>,
            WatermarkMode: WatermarkMode::<Impl, IMPL_OFFSET>,
            SetWatermarkMode: SetWatermarkMode::<Impl, IMPL_OFFSET>,
            ZoomLevel: ZoomLevel::<Impl, IMPL_OFFSET>,
            SetZoomLevel: SetZoomLevel::<Impl, IMPL_OFFSET>,
            MapElements: MapElements::<Impl, IMPL_OFFSET>,
            Routes: Routes::<Impl, IMPL_OFFSET>,
            TileSources: TileSources::<Impl, IMPL_OFFSET>,
            CenterChanged: CenterChanged::<Impl, IMPL_OFFSET>,
            RemoveCenterChanged: RemoveCenterChanged::<Impl, IMPL_OFFSET>,
            HeadingChanged: HeadingChanged::<Impl, IMPL_OFFSET>,
            RemoveHeadingChanged: RemoveHeadingChanged::<Impl, IMPL_OFFSET>,
            LoadingStatusChanged: LoadingStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveLoadingStatusChanged: RemoveLoadingStatusChanged::<Impl, IMPL_OFFSET>,
            MapDoubleTapped: MapDoubleTapped::<Impl, IMPL_OFFSET>,
            RemoveMapDoubleTapped: RemoveMapDoubleTapped::<Impl, IMPL_OFFSET>,
            MapHolding: MapHolding::<Impl, IMPL_OFFSET>,
            RemoveMapHolding: RemoveMapHolding::<Impl, IMPL_OFFSET>,
            MapTapped: MapTapped::<Impl, IMPL_OFFSET>,
            RemoveMapTapped: RemoveMapTapped::<Impl, IMPL_OFFSET>,
            PitchChanged: PitchChanged::<Impl, IMPL_OFFSET>,
            RemovePitchChanged: RemovePitchChanged::<Impl, IMPL_OFFSET>,
            TransformOriginChanged: TransformOriginChanged::<Impl, IMPL_OFFSET>,
            RemoveTransformOriginChanged: RemoveTransformOriginChanged::<Impl, IMPL_OFFSET>,
            ZoomLevelChanged: ZoomLevelChanged::<Impl, IMPL_OFFSET>,
            RemoveZoomLevelChanged: RemoveZoomLevelChanged::<Impl, IMPL_OFFSET>,
            FindMapElementsAtOffset: FindMapElementsAtOffset::<Impl, IMPL_OFFSET>,
            GetLocationFromOffset: GetLocationFromOffset::<Impl, IMPL_OFFSET>,
            GetOffsetFromLocation: GetOffsetFromLocation::<Impl, IMPL_OFFSET>,
            IsLocationInView: IsLocationInView::<Impl, IMPL_OFFSET>,
            TrySetViewBoundsAsync: TrySetViewBoundsAsync::<Impl, IMPL_OFFSET>,
            TrySetViewWithCenterAsync: TrySetViewWithCenterAsync::<Impl, IMPL_OFFSET>,
            TrySetViewWithCenterAndZoomAsync: TrySetViewWithCenterAndZoomAsync::<Impl, IMPL_OFFSET>,
            TrySetViewWithCenterZoomHeadingAndPitchAsync: TrySetViewWithCenterZoomHeadingAndPitchAsync::<Impl, IMPL_OFFSET>,
            TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync: TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapControl2_Impl: Sized {
    fn BusinessLandmarksVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetBusinessLandmarksVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TransitFeaturesVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetTransitFeaturesVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PanInteractionMode(&mut self) -> ::windows::core::Result<MapPanInteractionMode>;
    fn SetPanInteractionMode(&mut self, value: MapPanInteractionMode) -> ::windows::core::Result<()>;
    fn RotateInteractionMode(&mut self) -> ::windows::core::Result<MapInteractionMode>;
    fn SetRotateInteractionMode(&mut self, value: MapInteractionMode) -> ::windows::core::Result<()>;
    fn TiltInteractionMode(&mut self) -> ::windows::core::Result<MapInteractionMode>;
    fn SetTiltInteractionMode(&mut self, value: MapInteractionMode) -> ::windows::core::Result<()>;
    fn ZoomInteractionMode(&mut self) -> ::windows::core::Result<MapInteractionMode>;
    fn SetZoomInteractionMode(&mut self, value: MapInteractionMode) -> ::windows::core::Result<()>;
    fn Is3DSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStreetsideSupported(&mut self) -> ::windows::core::Result<bool>;
    fn Scene(&mut self) -> ::windows::core::Result<MapScene>;
    fn SetScene(&mut self, value: &::core::option::Option<MapScene>) -> ::windows::core::Result<()>;
    fn ActualCamera(&mut self) -> ::windows::core::Result<MapCamera>;
    fn TargetCamera(&mut self) -> ::windows::core::Result<MapCamera>;
    fn CustomExperience(&mut self) -> ::windows::core::Result<MapCustomExperience>;
    fn SetCustomExperience(&mut self, value: &::core::option::Option<MapCustomExperience>) -> ::windows::core::Result<()>;
    fn MapElementClick(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementClick(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerEntered(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerEntered(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerExited(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerExited(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActualCameraChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualCameraChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActualCameraChanging(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangingEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualCameraChanging(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TargetCameraChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapTargetCameraChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetCameraChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CustomExperienceChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapCustomExperienceChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCustomExperienceChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartContinuousRotate(&mut self, rateindegreespersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousRotate(&mut self) -> ::windows::core::Result<()>;
    fn StartContinuousTilt(&mut self, rateindegreespersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousTilt(&mut self) -> ::windows::core::Result<()>;
    fn StartContinuousZoom(&mut self, rateofchangepersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousZoom(&mut self) -> ::windows::core::Result<()>;
    fn TryRotateAsync(&mut self, degrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryRotateToAsync(&mut self, angleindegrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryTiltAsync(&mut self, degrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryTiltToAsync(&mut self, angleindegrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryZoomInAsync(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryZoomOutAsync(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryZoomToAsync(&mut self, zoomlevel: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetSceneAsync(&mut self, scene: &::core::option::Option<MapScene>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetSceneWithAnimationAsync(&mut self, scene: &::core::option::Option<MapScene>, animationkind: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl2_Vtbl {
        unsafe extern "system" fn BusinessLandmarksVisible<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBusinessLandmarksVisible<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusinessLandmarksVisible(value).into()
        }
        unsafe extern "system" fn TransitFeaturesVisible<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransitFeaturesVisible<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransitFeaturesVisible(value).into()
        }
        unsafe extern "system" fn PanInteractionMode<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapPanInteractionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPanInteractionMode<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapPanInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPanInteractionMode(value).into()
        }
        unsafe extern "system" fn RotateInteractionMode<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotateInteractionMode<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotateInteractionMode(value).into()
        }
        unsafe extern "system" fn TiltInteractionMode<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTiltInteractionMode<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTiltInteractionMode(value).into()
        }
        unsafe extern "system" fn ZoomInteractionMode<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetZoomInteractionMode<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoomInteractionMode(value).into()
        }
        unsafe extern "system" fn Is3DSupported<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsStreetsideSupported<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Scene<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScene<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScene(&*(&value as *const <MapScene as ::windows::core::Abi>::Abi as *const <MapScene as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualCamera<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TargetCamera<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomExperience<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCustomExperience<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomExperience(&*(&value as *const <MapCustomExperience as ::windows::core::Abi>::Abi as *const <MapCustomExperience as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementClick<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapElementClick<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerEntered<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapElementPointerEntered<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerExited<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapElementPointerExited<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualCameraChanged<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveActualCameraChanged<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActualCameraChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualCameraChanging<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveActualCameraChanging<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActualCameraChanging(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetCameraChanged<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTargetCameraChanged<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTargetCameraChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CustomExperienceChanged<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCustomExperienceChanged<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCustomExperienceChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartContinuousRotate<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rateindegreespersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartContinuousRotate(rateindegreespersecond).into()
        }
        unsafe extern "system" fn StopContinuousRotate<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopContinuousRotate().into()
        }
        unsafe extern "system" fn StartContinuousTilt<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rateindegreespersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartContinuousTilt(rateindegreespersecond).into()
        }
        unsafe extern "system" fn StopContinuousTilt<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopContinuousTilt().into()
        }
        unsafe extern "system" fn StartContinuousZoom<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rateofchangepersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartContinuousZoom(rateofchangepersecond).into()
        }
        unsafe extern "system" fn StopContinuousZoom<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopContinuousZoom().into()
        }
        unsafe extern "system" fn TryRotateAsync<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryRotateToAsync<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angleindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryTiltAsync<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryTiltToAsync<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angleindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryZoomInAsync<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryZoomOutAsync<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryZoomToAsync<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomlevel: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetSceneAsync<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scene: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetSceneWithAnimationAsync<Impl: IMapControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scene: ::windows::core::RawPtr, animationkind: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControl2, BASE_OFFSET>(),
            BusinessLandmarksVisible: BusinessLandmarksVisible::<Impl, IMPL_OFFSET>,
            SetBusinessLandmarksVisible: SetBusinessLandmarksVisible::<Impl, IMPL_OFFSET>,
            TransitFeaturesVisible: TransitFeaturesVisible::<Impl, IMPL_OFFSET>,
            SetTransitFeaturesVisible: SetTransitFeaturesVisible::<Impl, IMPL_OFFSET>,
            PanInteractionMode: PanInteractionMode::<Impl, IMPL_OFFSET>,
            SetPanInteractionMode: SetPanInteractionMode::<Impl, IMPL_OFFSET>,
            RotateInteractionMode: RotateInteractionMode::<Impl, IMPL_OFFSET>,
            SetRotateInteractionMode: SetRotateInteractionMode::<Impl, IMPL_OFFSET>,
            TiltInteractionMode: TiltInteractionMode::<Impl, IMPL_OFFSET>,
            SetTiltInteractionMode: SetTiltInteractionMode::<Impl, IMPL_OFFSET>,
            ZoomInteractionMode: ZoomInteractionMode::<Impl, IMPL_OFFSET>,
            SetZoomInteractionMode: SetZoomInteractionMode::<Impl, IMPL_OFFSET>,
            Is3DSupported: Is3DSupported::<Impl, IMPL_OFFSET>,
            IsStreetsideSupported: IsStreetsideSupported::<Impl, IMPL_OFFSET>,
            Scene: Scene::<Impl, IMPL_OFFSET>,
            SetScene: SetScene::<Impl, IMPL_OFFSET>,
            ActualCamera: ActualCamera::<Impl, IMPL_OFFSET>,
            TargetCamera: TargetCamera::<Impl, IMPL_OFFSET>,
            CustomExperience: CustomExperience::<Impl, IMPL_OFFSET>,
            SetCustomExperience: SetCustomExperience::<Impl, IMPL_OFFSET>,
            MapElementClick: MapElementClick::<Impl, IMPL_OFFSET>,
            RemoveMapElementClick: RemoveMapElementClick::<Impl, IMPL_OFFSET>,
            MapElementPointerEntered: MapElementPointerEntered::<Impl, IMPL_OFFSET>,
            RemoveMapElementPointerEntered: RemoveMapElementPointerEntered::<Impl, IMPL_OFFSET>,
            MapElementPointerExited: MapElementPointerExited::<Impl, IMPL_OFFSET>,
            RemoveMapElementPointerExited: RemoveMapElementPointerExited::<Impl, IMPL_OFFSET>,
            ActualCameraChanged: ActualCameraChanged::<Impl, IMPL_OFFSET>,
            RemoveActualCameraChanged: RemoveActualCameraChanged::<Impl, IMPL_OFFSET>,
            ActualCameraChanging: ActualCameraChanging::<Impl, IMPL_OFFSET>,
            RemoveActualCameraChanging: RemoveActualCameraChanging::<Impl, IMPL_OFFSET>,
            TargetCameraChanged: TargetCameraChanged::<Impl, IMPL_OFFSET>,
            RemoveTargetCameraChanged: RemoveTargetCameraChanged::<Impl, IMPL_OFFSET>,
            CustomExperienceChanged: CustomExperienceChanged::<Impl, IMPL_OFFSET>,
            RemoveCustomExperienceChanged: RemoveCustomExperienceChanged::<Impl, IMPL_OFFSET>,
            StartContinuousRotate: StartContinuousRotate::<Impl, IMPL_OFFSET>,
            StopContinuousRotate: StopContinuousRotate::<Impl, IMPL_OFFSET>,
            StartContinuousTilt: StartContinuousTilt::<Impl, IMPL_OFFSET>,
            StopContinuousTilt: StopContinuousTilt::<Impl, IMPL_OFFSET>,
            StartContinuousZoom: StartContinuousZoom::<Impl, IMPL_OFFSET>,
            StopContinuousZoom: StopContinuousZoom::<Impl, IMPL_OFFSET>,
            TryRotateAsync: TryRotateAsync::<Impl, IMPL_OFFSET>,
            TryRotateToAsync: TryRotateToAsync::<Impl, IMPL_OFFSET>,
            TryTiltAsync: TryTiltAsync::<Impl, IMPL_OFFSET>,
            TryTiltToAsync: TryTiltToAsync::<Impl, IMPL_OFFSET>,
            TryZoomInAsync: TryZoomInAsync::<Impl, IMPL_OFFSET>,
            TryZoomOutAsync: TryZoomOutAsync::<Impl, IMPL_OFFSET>,
            TryZoomToAsync: TryZoomToAsync::<Impl, IMPL_OFFSET>,
            TrySetSceneAsync: TrySetSceneAsync::<Impl, IMPL_OFFSET>,
            TrySetSceneWithAnimationAsync: TrySetSceneWithAnimationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapControl3_Impl: Sized {
    fn MapRightTapped(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapRightTappedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapRightTapped(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapControl3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl3_Vtbl {
        unsafe extern "system" fn MapRightTapped<Impl: IMapControl3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapRightTapped<Impl: IMapControl3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapRightTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControl3, BASE_OFFSET>(),
            MapRightTapped: MapRightTapped::<Impl, IMPL_OFFSET>,
            RemoveMapRightTapped: RemoveMapRightTapped::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IMapControl4_Impl: Sized {
    fn BusinessLandmarksEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetBusinessLandmarksEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TransitFeaturesEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetTransitFeaturesEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetVisibleRegion(&mut self, region: MapVisibleRegionKind) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl4";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapControl4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl4_Vtbl {
        unsafe extern "system" fn BusinessLandmarksEnabled<Impl: IMapControl4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBusinessLandmarksEnabled<Impl: IMapControl4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusinessLandmarksEnabled(value).into()
        }
        unsafe extern "system" fn TransitFeaturesEnabled<Impl: IMapControl4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransitFeaturesEnabled<Impl: IMapControl4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransitFeaturesEnabled(value).into()
        }
        unsafe extern "system" fn GetVisibleRegion<Impl: IMapControl4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, region: MapVisibleRegionKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControl4, BASE_OFFSET>(),
            BusinessLandmarksEnabled: BusinessLandmarksEnabled::<Impl, IMPL_OFFSET>,
            SetBusinessLandmarksEnabled: SetBusinessLandmarksEnabled::<Impl, IMPL_OFFSET>,
            TransitFeaturesEnabled: TransitFeaturesEnabled::<Impl, IMPL_OFFSET>,
            SetTransitFeaturesEnabled: SetTransitFeaturesEnabled::<Impl, IMPL_OFFSET>,
            GetVisibleRegion: GetVisibleRegion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControl5_Impl: Sized {
    fn MapProjection(&mut self) -> ::windows::core::Result<MapProjection>;
    fn SetMapProjection(&mut self, value: MapProjection) -> ::windows::core::Result<()>;
    fn StyleSheet(&mut self) -> ::windows::core::Result<MapStyleSheet>;
    fn SetStyleSheet(&mut self, value: &::core::option::Option<MapStyleSheet>) -> ::windows::core::Result<()>;
    fn ViewPadding(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetViewPadding(&mut self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn MapContextRequested(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapContextRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapContextRequested(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindMapElementsAtOffsetWithRadius(&mut self, offset: &super::super::super::super::Foundation::Point, radius: f64) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
    fn GetLocationFromOffsetWithReferenceSystem(&mut self, offset: &super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn StartContinuousPan(&mut self, horizontalpixelspersecond: f64, verticalpixelspersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousPan(&mut self) -> ::windows::core::Result<()>;
    fn TryPanAsync(&mut self, horizontalpixels: f64, verticalpixels: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryPanToAsync(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl5";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControl5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl5_Vtbl {
        unsafe extern "system" fn MapProjection<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapProjection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMapProjection<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapProjection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapProjection(value).into()
        }
        unsafe extern "system" fn StyleSheet<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStyleSheet<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyleSheet(&*(&value as *const <MapStyleSheet as ::windows::core::Abi>::Abi as *const <MapStyleSheet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ViewPadding<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetViewPadding<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewPadding(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapContextRequested<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapContextRequested<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapContextRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindMapElementsAtOffsetWithRadius<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLocationFromOffsetWithReferenceSystem<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocationFromOffsetWithReferenceSystem(&*(&offset as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), desiredreferencesystem, ::core::mem::transmute_copy(&location)).into()
        }
        unsafe extern "system" fn StartContinuousPan<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpixelspersecond: f64, verticalpixelspersecond: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartContinuousPan(horizontalpixelspersecond, verticalpixelspersecond).into()
        }
        unsafe extern "system" fn StopContinuousPan<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopContinuousPan().into()
        }
        unsafe extern "system" fn TryPanAsync<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpixels: f64, verticalpixels: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryPanToAsync<Impl: IMapControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControl5, BASE_OFFSET>(),
            MapProjection: MapProjection::<Impl, IMPL_OFFSET>,
            SetMapProjection: SetMapProjection::<Impl, IMPL_OFFSET>,
            StyleSheet: StyleSheet::<Impl, IMPL_OFFSET>,
            SetStyleSheet: SetStyleSheet::<Impl, IMPL_OFFSET>,
            ViewPadding: ViewPadding::<Impl, IMPL_OFFSET>,
            SetViewPadding: SetViewPadding::<Impl, IMPL_OFFSET>,
            MapContextRequested: MapContextRequested::<Impl, IMPL_OFFSET>,
            RemoveMapContextRequested: RemoveMapContextRequested::<Impl, IMPL_OFFSET>,
            FindMapElementsAtOffsetWithRadius: FindMapElementsAtOffsetWithRadius::<Impl, IMPL_OFFSET>,
            GetLocationFromOffsetWithReferenceSystem: GetLocationFromOffsetWithReferenceSystem::<Impl, IMPL_OFFSET>,
            StartContinuousPan: StartContinuousPan::<Impl, IMPL_OFFSET>,
            StopContinuousPan: StopContinuousPan::<Impl, IMPL_OFFSET>,
            TryPanAsync: TryPanAsync::<Impl, IMPL_OFFSET>,
            TryPanToAsync: TryPanToAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControl6_Impl: Sized {
    fn Layers(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapLayer>>;
    fn SetLayers(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::Collections::IVector<MapLayer>>) -> ::windows::core::Result<()>;
    fn TryGetLocationFromOffset(&mut self, offset: &super::super::super::super::Foundation::Point, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<bool>;
    fn TryGetLocationFromOffsetWithReferenceSystem(&mut self, offset: &super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControl6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl6";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControl6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl6_Vtbl {
        unsafe extern "system" fn Layers<Impl: IMapControl6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLayers<Impl: IMapControl6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLayers(&*(&value as *const <super::super::super::super::Foundation::Collections::IVector<MapLayer> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVector<MapLayer> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryGetLocationFromOffset<Impl: IMapControl6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, location: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetLocationFromOffsetWithReferenceSystem<Impl: IMapControl6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControl6, BASE_OFFSET>(),
            Layers: Layers::<Impl, IMPL_OFFSET>,
            SetLayers: SetLayers::<Impl, IMPL_OFFSET>,
            TryGetLocationFromOffset: TryGetLocationFromOffset::<Impl, IMPL_OFFSET>,
            TryGetLocationFromOffsetWithReferenceSystem: TryGetLocationFromOffsetWithReferenceSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl7_Impl: Sized {
    fn Region(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRegion(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControl7 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl7";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControl7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl7_Vtbl {
        unsafe extern "system" fn Region<Impl: IMapControl7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRegion<Impl: IMapControl7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRegion(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControl7, BASE_OFFSET>(),
            Region: Region::<Impl, IMPL_OFFSET>,
            SetRegion: SetRegion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl8_Impl: Sized {
    fn CanTiltDown(&mut self) -> ::windows::core::Result<bool>;
    fn CanTiltUp(&mut self) -> ::windows::core::Result<bool>;
    fn CanZoomIn(&mut self) -> ::windows::core::Result<bool>;
    fn CanZoomOut(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControl8 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControl8";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControl8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControl8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControl8_Vtbl {
        unsafe extern "system" fn CanTiltDown<Impl: IMapControl8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanTiltUp<Impl: IMapControl8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanZoomIn<Impl: IMapControl8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanZoomOut<Impl: IMapControl8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControl8, BASE_OFFSET>(),
            CanTiltDown: CanTiltDown::<Impl, IMPL_OFFSET>,
            CanTiltUp: CanTiltUp::<Impl, IMPL_OFFSET>,
            CanZoomIn: CanZoomIn::<Impl, IMPL_OFFSET>,
            CanZoomOut: CanZoomOut::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControl8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
pub trait IMapControlBusinessLandmarkClickEventArgs_Impl: Sized {
    fn LocalLocations(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkClickEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl IMapControlBusinessLandmarkClickEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlBusinessLandmarkClickEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlBusinessLandmarkClickEventArgs_Vtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkClickEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlBusinessLandmarkClickEventArgs, BASE_OFFSET>(),
            LocalLocations: LocalLocations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlBusinessLandmarkClickEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
pub trait IMapControlBusinessLandmarkPointerEnteredEventArgs_Impl: Sized {
    fn LocalLocations(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkPointerEnteredEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl IMapControlBusinessLandmarkPointerEnteredEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlBusinessLandmarkPointerEnteredEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlBusinessLandmarkPointerEnteredEventArgs_Vtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkPointerEnteredEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlBusinessLandmarkPointerEnteredEventArgs, BASE_OFFSET>(),
            LocalLocations: LocalLocations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlBusinessLandmarkPointerEnteredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
pub trait IMapControlBusinessLandmarkPointerExitedEventArgs_Impl: Sized {
    fn LocalLocations(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkPointerExitedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl IMapControlBusinessLandmarkPointerExitedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlBusinessLandmarkPointerExitedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlBusinessLandmarkPointerExitedEventArgs_Vtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkPointerExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlBusinessLandmarkPointerExitedEventArgs, BASE_OFFSET>(),
            LocalLocations: LocalLocations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlBusinessLandmarkPointerExitedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
pub trait IMapControlBusinessLandmarkRightTappedEventArgs_Impl: Sized {
    fn LocalLocations(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlBusinessLandmarkRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlBusinessLandmarkRightTappedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch", feature = "implement_exclusive"))]
impl IMapControlBusinessLandmarkRightTappedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlBusinessLandmarkRightTappedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlBusinessLandmarkRightTappedEventArgs_Vtbl {
        unsafe extern "system" fn LocalLocations<Impl: IMapControlBusinessLandmarkRightTappedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlBusinessLandmarkRightTappedEventArgs, BASE_OFFSET>(),
            LocalLocations: LocalLocations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlBusinessLandmarkRightTappedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapControlDataHelper_Impl: Sized {
    fn BusinessLandmarkClick(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkClick(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeatureClick(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeatureClick(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BusinessLandmarkRightTapped(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkRightTappedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkRightTapped(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeatureRightTapped(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureRightTappedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeatureRightTapped(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlDataHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlDataHelper";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapControlDataHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlDataHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlDataHelper_Vtbl {
        unsafe extern "system" fn BusinessLandmarkClick<Impl: IMapControlDataHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBusinessLandmarkClick<Impl: IMapControlDataHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeatureClick<Impl: IMapControlDataHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTransitFeatureClick<Impl: IMapControlDataHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeatureClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BusinessLandmarkRightTapped<Impl: IMapControlDataHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBusinessLandmarkRightTapped<Impl: IMapControlDataHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkRightTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeatureRightTapped<Impl: IMapControlDataHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTransitFeatureRightTapped<Impl: IMapControlDataHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeatureRightTapped(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlDataHelper, BASE_OFFSET>(),
            BusinessLandmarkClick: BusinessLandmarkClick::<Impl, IMPL_OFFSET>,
            RemoveBusinessLandmarkClick: RemoveBusinessLandmarkClick::<Impl, IMPL_OFFSET>,
            TransitFeatureClick: TransitFeatureClick::<Impl, IMPL_OFFSET>,
            RemoveTransitFeatureClick: RemoveTransitFeatureClick::<Impl, IMPL_OFFSET>,
            BusinessLandmarkRightTapped: BusinessLandmarkRightTapped::<Impl, IMPL_OFFSET>,
            RemoveBusinessLandmarkRightTapped: RemoveBusinessLandmarkRightTapped::<Impl, IMPL_OFFSET>,
            TransitFeatureRightTapped: TransitFeatureRightTapped::<Impl, IMPL_OFFSET>,
            RemoveTransitFeatureRightTapped: RemoveTransitFeatureRightTapped::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlDataHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapControlDataHelper2_Impl: Sized {
    fn BusinessLandmarkPointerEntered(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkPointerEntered(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeaturePointerEntered(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeaturePointerEntered(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BusinessLandmarkPointerExited(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkPointerExited(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeaturePointerExited(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeaturePointerExited(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlDataHelper2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlDataHelper2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapControlDataHelper2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlDataHelper2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlDataHelper2_Vtbl {
        unsafe extern "system" fn BusinessLandmarkPointerEntered<Impl: IMapControlDataHelper2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBusinessLandmarkPointerEntered<Impl: IMapControlDataHelper2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkPointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeaturePointerEntered<Impl: IMapControlDataHelper2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTransitFeaturePointerEntered<Impl: IMapControlDataHelper2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeaturePointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BusinessLandmarkPointerExited<Impl: IMapControlDataHelper2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBusinessLandmarkPointerExited<Impl: IMapControlDataHelper2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBusinessLandmarkPointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransitFeaturePointerExited<Impl: IMapControlDataHelper2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTransitFeaturePointerExited<Impl: IMapControlDataHelper2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransitFeaturePointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlDataHelper2, BASE_OFFSET>(),
            BusinessLandmarkPointerEntered: BusinessLandmarkPointerEntered::<Impl, IMPL_OFFSET>,
            RemoveBusinessLandmarkPointerEntered: RemoveBusinessLandmarkPointerEntered::<Impl, IMPL_OFFSET>,
            TransitFeaturePointerEntered: TransitFeaturePointerEntered::<Impl, IMPL_OFFSET>,
            RemoveTransitFeaturePointerEntered: RemoveTransitFeaturePointerEntered::<Impl, IMPL_OFFSET>,
            BusinessLandmarkPointerExited: BusinessLandmarkPointerExited::<Impl, IMPL_OFFSET>,
            RemoveBusinessLandmarkPointerExited: RemoveBusinessLandmarkPointerExited::<Impl, IMPL_OFFSET>,
            TransitFeaturePointerExited: TransitFeaturePointerExited::<Impl, IMPL_OFFSET>,
            RemoveTransitFeaturePointerExited: RemoveTransitFeaturePointerExited::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlDataHelper2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlDataHelperFactory_Impl: Sized {
    fn CreateInstance(&mut self, map: &::core::option::Option<MapControl>) -> ::windows::core::Result<MapControlDataHelper>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlDataHelperFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlDataHelperFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlDataHelperFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlDataHelperFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlDataHelperFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapControlDataHelperFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, map: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlDataHelperFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlDataHelperFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlDataHelperStatics_Impl: Sized {
    fn CreateMapControl(&mut self, rasterrendermode: bool) -> ::windows::core::Result<MapControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlDataHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlDataHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlDataHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlDataHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlDataHelperStatics_Vtbl {
        unsafe extern "system" fn CreateMapControl<Impl: IMapControlDataHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rasterrendermode: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlDataHelperStatics, BASE_OFFSET>(),
            CreateMapControl: CreateMapControl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlDataHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapControlStatics_Impl: Sized {
    fn CenterProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ChildrenProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ColorSchemeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DesiredPitchProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HeadingProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LandmarksVisibleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LoadingStatusProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MapServiceTokenProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PedestrianFeaturesVisibleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PitchProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StyleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TrafficFlowVisibleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TransformOriginProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn WatermarkModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZoomLevelProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MapElementsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RoutesProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TileSourcesProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LocationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetLocation(&mut self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&mut self, element: &::core::option::Option<super::super::DependencyObject>, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn NormalizedAnchorPointProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetNormalizedAnchorPoint(&mut self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetNormalizedAnchorPoint(&mut self, element: &::core::option::Option<super::super::DependencyObject>, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapControlStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics_Vtbl {
        unsafe extern "system" fn CenterProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChildrenProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ColorSchemeProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DesiredPitchProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HeadingProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LandmarksVisibleProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LoadingStatusProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapServiceTokenProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PedestrianFeaturesVisibleProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PitchProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StyleProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrafficFlowVisibleProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransformOriginProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WatermarkModeProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ZoomLevelProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapElementsProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoutesProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TileSourcesProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocationProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLocation<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocation<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalizedAnchorPointProperty<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNormalizedAnchorPoint<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNormalizedAnchorPoint<Impl: IMapControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalizedAnchorPoint(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlStatics, BASE_OFFSET>(),
            CenterProperty: CenterProperty::<Impl, IMPL_OFFSET>,
            ChildrenProperty: ChildrenProperty::<Impl, IMPL_OFFSET>,
            ColorSchemeProperty: ColorSchemeProperty::<Impl, IMPL_OFFSET>,
            DesiredPitchProperty: DesiredPitchProperty::<Impl, IMPL_OFFSET>,
            HeadingProperty: HeadingProperty::<Impl, IMPL_OFFSET>,
            LandmarksVisibleProperty: LandmarksVisibleProperty::<Impl, IMPL_OFFSET>,
            LoadingStatusProperty: LoadingStatusProperty::<Impl, IMPL_OFFSET>,
            MapServiceTokenProperty: MapServiceTokenProperty::<Impl, IMPL_OFFSET>,
            PedestrianFeaturesVisibleProperty: PedestrianFeaturesVisibleProperty::<Impl, IMPL_OFFSET>,
            PitchProperty: PitchProperty::<Impl, IMPL_OFFSET>,
            StyleProperty: StyleProperty::<Impl, IMPL_OFFSET>,
            TrafficFlowVisibleProperty: TrafficFlowVisibleProperty::<Impl, IMPL_OFFSET>,
            TransformOriginProperty: TransformOriginProperty::<Impl, IMPL_OFFSET>,
            WatermarkModeProperty: WatermarkModeProperty::<Impl, IMPL_OFFSET>,
            ZoomLevelProperty: ZoomLevelProperty::<Impl, IMPL_OFFSET>,
            MapElementsProperty: MapElementsProperty::<Impl, IMPL_OFFSET>,
            RoutesProperty: RoutesProperty::<Impl, IMPL_OFFSET>,
            TileSourcesProperty: TileSourcesProperty::<Impl, IMPL_OFFSET>,
            LocationProperty: LocationProperty::<Impl, IMPL_OFFSET>,
            GetLocation: GetLocation::<Impl, IMPL_OFFSET>,
            SetLocation: SetLocation::<Impl, IMPL_OFFSET>,
            NormalizedAnchorPointProperty: NormalizedAnchorPointProperty::<Impl, IMPL_OFFSET>,
            GetNormalizedAnchorPoint: GetNormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
            SetNormalizedAnchorPoint: SetNormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics2_Impl: Sized {
    fn BusinessLandmarksVisibleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TransitFeaturesVisibleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PanInteractionModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RotateInteractionModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TiltInteractionModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZoomInteractionModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn Is3DSupportedProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsStreetsideSupportedProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SceneProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics2_Vtbl {
        unsafe extern "system" fn BusinessLandmarksVisibleProperty<Impl: IMapControlStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransitFeaturesVisibleProperty<Impl: IMapControlStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PanInteractionModeProperty<Impl: IMapControlStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RotateInteractionModeProperty<Impl: IMapControlStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TiltInteractionModeProperty<Impl: IMapControlStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ZoomInteractionModeProperty<Impl: IMapControlStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Is3DSupportedProperty<Impl: IMapControlStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsStreetsideSupportedProperty<Impl: IMapControlStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SceneProperty<Impl: IMapControlStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlStatics2, BASE_OFFSET>(),
            BusinessLandmarksVisibleProperty: BusinessLandmarksVisibleProperty::<Impl, IMPL_OFFSET>,
            TransitFeaturesVisibleProperty: TransitFeaturesVisibleProperty::<Impl, IMPL_OFFSET>,
            PanInteractionModeProperty: PanInteractionModeProperty::<Impl, IMPL_OFFSET>,
            RotateInteractionModeProperty: RotateInteractionModeProperty::<Impl, IMPL_OFFSET>,
            TiltInteractionModeProperty: TiltInteractionModeProperty::<Impl, IMPL_OFFSET>,
            ZoomInteractionModeProperty: ZoomInteractionModeProperty::<Impl, IMPL_OFFSET>,
            Is3DSupportedProperty: Is3DSupportedProperty::<Impl, IMPL_OFFSET>,
            IsStreetsideSupportedProperty: IsStreetsideSupportedProperty::<Impl, IMPL_OFFSET>,
            SceneProperty: SceneProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics4_Impl: Sized {
    fn BusinessLandmarksEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TransitFeaturesEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics4_Vtbl {
        unsafe extern "system" fn BusinessLandmarksEnabledProperty<Impl: IMapControlStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransitFeaturesEnabledProperty<Impl: IMapControlStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlStatics4, BASE_OFFSET>(),
            BusinessLandmarksEnabledProperty: BusinessLandmarksEnabledProperty::<Impl, IMPL_OFFSET>,
            TransitFeaturesEnabledProperty: TransitFeaturesEnabledProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics5_Impl: Sized {
    fn MapProjectionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StyleSheetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ViewPaddingProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics5_Vtbl {
        unsafe extern "system" fn MapProjectionProperty<Impl: IMapControlStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StyleSheetProperty<Impl: IMapControlStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ViewPaddingProperty<Impl: IMapControlStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlStatics5, BASE_OFFSET>(),
            MapProjectionProperty: MapProjectionProperty::<Impl, IMPL_OFFSET>,
            StyleSheetProperty: StyleSheetProperty::<Impl, IMPL_OFFSET>,
            ViewPaddingProperty: ViewPaddingProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics6_Impl: Sized {
    fn LayersProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics6_Vtbl {
        unsafe extern "system" fn LayersProperty<Impl: IMapControlStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlStatics6, BASE_OFFSET>(),
            LayersProperty: LayersProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics7_Impl: Sized {
    fn RegionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics7";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics7_Vtbl {
        unsafe extern "system" fn RegionProperty<Impl: IMapControlStatics7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlStatics7, BASE_OFFSET>(),
            RegionProperty: RegionProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics8_Impl: Sized {
    fn CanTiltDownProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CanTiltUpProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CanZoomInProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CanZoomOutProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlStatics8 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlStatics8";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlStatics8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlStatics8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlStatics8_Vtbl {
        unsafe extern "system" fn CanTiltDownProperty<Impl: IMapControlStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanTiltUpProperty<Impl: IMapControlStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanZoomInProperty<Impl: IMapControlStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanZoomOutProperty<Impl: IMapControlStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlStatics8, BASE_OFFSET>(),
            CanTiltDownProperty: CanTiltDownProperty::<Impl, IMPL_OFFSET>,
            CanTiltUpProperty: CanTiltUpProperty::<Impl, IMPL_OFFSET>,
            CanZoomInProperty: CanZoomInProperty::<Impl, IMPL_OFFSET>,
            CanZoomOutProperty: CanZoomOutProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlStatics8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControlTransitFeatureClickEventArgs_Impl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlTransitFeatureClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeatureClickEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControlTransitFeatureClickEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlTransitFeatureClickEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlTransitFeatureClickEventArgs_Vtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeatureClickEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeatureClickEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeatureClickEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlTransitFeatureClickEventArgs, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            TransitProperties: TransitProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlTransitFeatureClickEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControlTransitFeaturePointerEnteredEventArgs_Impl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlTransitFeaturePointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeaturePointerEnteredEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControlTransitFeaturePointerEnteredEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlTransitFeaturePointerEnteredEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlTransitFeaturePointerEnteredEventArgs_Vtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeaturePointerEnteredEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeaturePointerEnteredEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeaturePointerEnteredEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlTransitFeaturePointerEnteredEventArgs, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            TransitProperties: TransitProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlTransitFeaturePointerEnteredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControlTransitFeaturePointerExitedEventArgs_Impl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlTransitFeaturePointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeaturePointerExitedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControlTransitFeaturePointerExitedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlTransitFeaturePointerExitedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlTransitFeaturePointerExitedEventArgs_Vtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeaturePointerExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeaturePointerExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeaturePointerExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlTransitFeaturePointerExitedEventArgs, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            TransitProperties: TransitProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlTransitFeaturePointerExitedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapControlTransitFeatureRightTappedEventArgs_Impl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapControlTransitFeatureRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapControlTransitFeatureRightTappedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapControlTransitFeatureRightTappedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlTransitFeatureRightTappedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlTransitFeatureRightTappedEventArgs_Vtbl {
        unsafe extern "system" fn DisplayName<Impl: IMapControlTransitFeatureRightTappedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapControlTransitFeatureRightTappedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransitProperties<Impl: IMapControlTransitFeatureRightTappedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlTransitFeatureRightTappedEventArgs, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            TransitProperties: TransitProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlTransitFeatureRightTappedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCustomExperience_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapCustomExperience {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCustomExperience";
}
#[cfg(feature = "implement_exclusive")]
impl IMapCustomExperience_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapCustomExperience_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapCustomExperience_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapCustomExperience, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapCustomExperience as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCustomExperienceChangedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapCustomExperienceChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCustomExperienceChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapCustomExperienceChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapCustomExperienceChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapCustomExperienceChangedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapCustomExperienceChangedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapCustomExperienceChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCustomExperienceFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapCustomExperience>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapCustomExperienceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapCustomExperienceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapCustomExperienceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapCustomExperienceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapCustomExperienceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapCustomExperienceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapCustomExperienceFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapCustomExperienceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement_Impl: Sized {
    fn ZIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Visible(&mut self) -> ::windows::core::Result<bool>;
    fn SetVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement_Vtbl {
        unsafe extern "system" fn ZIndex<Impl: IMapElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetZIndex<Impl: IMapElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        unsafe extern "system" fn Visible<Impl: IMapElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVisible<Impl: IMapElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElement, BASE_OFFSET>(),
            ZIndex: ZIndex::<Impl, IMPL_OFFSET>,
            SetZIndex: SetZIndex::<Impl, IMPL_OFFSET>,
            Visible: Visible::<Impl, IMPL_OFFSET>,
            SetVisible: SetVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement2_Impl: Sized {
    fn MapTabIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetMapTabIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement2_Vtbl {
        unsafe extern "system" fn MapTabIndex<Impl: IMapElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMapTabIndex<Impl: IMapElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapTabIndex(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElement2, BASE_OFFSET>(),
            MapTabIndex: MapTabIndex::<Impl, IMPL_OFFSET>,
            SetMapTabIndex: SetMapTabIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement3_Impl: Sized {
    fn MapStyleSheetEntry(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMapStyleSheetEntry(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MapStyleSheetEntryState(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMapStyleSheetEntryState(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement3";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElement3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement3_Vtbl {
        unsafe extern "system" fn MapStyleSheetEntry<Impl: IMapElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMapStyleSheetEntry<Impl: IMapElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapStyleSheetEntry(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapStyleSheetEntryState<Impl: IMapElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMapStyleSheetEntryState<Impl: IMapElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapStyleSheetEntryState(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IMapElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTag<Impl: IMapElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElement3, BASE_OFFSET>(),
            MapStyleSheetEntry: MapStyleSheetEntry::<Impl, IMPL_OFFSET>,
            SetMapStyleSheetEntry: SetMapStyleSheetEntry::<Impl, IMPL_OFFSET>,
            MapStyleSheetEntryState: MapStyleSheetEntryState::<Impl, IMPL_OFFSET>,
            SetMapStyleSheetEntryState: SetMapStyleSheetEntryState::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IMapElement3D_Impl: Sized {
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&mut self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Model(&mut self) -> ::windows::core::Result<MapModel3D>;
    fn SetModel(&mut self, value: &::core::option::Option<MapModel3D>) -> ::windows::core::Result<()>;
    fn Heading(&mut self) -> ::windows::core::Result<f64>;
    fn SetHeading(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Pitch(&mut self) -> ::windows::core::Result<f64>;
    fn SetPitch(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Roll(&mut self) -> ::windows::core::Result<f64>;
    fn SetRoll(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Scale(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&mut self, value: &super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElement3D {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement3D";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IMapElement3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement3D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement3D_Vtbl {
        unsafe extern "system" fn Location<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocation<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Model<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetModel<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModel(&*(&value as *const <MapModel3D as ::windows::core::Abi>::Abi as *const <MapModel3D as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Heading<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHeading<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeading(value).into()
        }
        unsafe extern "system" fn Pitch<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPitch<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPitch(value).into()
        }
        unsafe extern "system" fn Roll<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRoll<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoll(value).into()
        }
        unsafe extern "system" fn Scale<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScale<Impl: IMapElement3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElement3D, BASE_OFFSET>(),
            Location: Location::<Impl, IMPL_OFFSET>,
            SetLocation: SetLocation::<Impl, IMPL_OFFSET>,
            Model: Model::<Impl, IMPL_OFFSET>,
            SetModel: SetModel::<Impl, IMPL_OFFSET>,
            Heading: Heading::<Impl, IMPL_OFFSET>,
            SetHeading: SetHeading::<Impl, IMPL_OFFSET>,
            Pitch: Pitch::<Impl, IMPL_OFFSET>,
            SetPitch: SetPitch::<Impl, IMPL_OFFSET>,
            Roll: Roll::<Impl, IMPL_OFFSET>,
            SetRoll: SetRoll::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            SetScale: SetScale::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement3D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement3DStatics_Impl: Sized {
    fn LocationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HeadingProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PitchProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RollProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ScaleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement3DStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement3DStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElement3DStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement3DStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement3DStatics_Vtbl {
        unsafe extern "system" fn LocationProperty<Impl: IMapElement3DStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HeadingProperty<Impl: IMapElement3DStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PitchProperty<Impl: IMapElement3DStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RollProperty<Impl: IMapElement3DStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScaleProperty<Impl: IMapElement3DStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElement3DStatics, BASE_OFFSET>(),
            LocationProperty: LocationProperty::<Impl, IMPL_OFFSET>,
            HeadingProperty: HeadingProperty::<Impl, IMPL_OFFSET>,
            PitchProperty: PitchProperty::<Impl, IMPL_OFFSET>,
            RollProperty: RollProperty::<Impl, IMPL_OFFSET>,
            ScaleProperty: ScaleProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement3DStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement4_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElement4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElement4";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElement4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElement4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElement4_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMapElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IMapElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElement4, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElement4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapElementClickEventArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementClickEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapElementClickEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementClickEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementClickEventArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IMapElementClickEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapElementClickEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapElements<Impl: IMapElementClickEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementClickEventArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            MapElements: MapElements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementClickEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapElementFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapElementPointerEnteredEventArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&mut self) -> ::windows::core::Result<MapElement>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementPointerEnteredEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapElementPointerEnteredEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementPointerEnteredEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementPointerEnteredEventArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IMapElementPointerEnteredEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapElementPointerEnteredEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapElement<Impl: IMapElementPointerEnteredEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementPointerEnteredEventArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            MapElement: MapElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementPointerEnteredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapElementPointerExitedEventArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&mut self) -> ::windows::core::Result<MapElement>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementPointerExitedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapElementPointerExitedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementPointerExitedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementPointerExitedEventArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IMapElementPointerExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapElementPointerExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapElement<Impl: IMapElementPointerExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementPointerExitedEventArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            MapElement: MapElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementPointerExitedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStatics_Impl: Sized {
    fn ZIndexProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VisibleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementStatics_Vtbl {
        unsafe extern "system" fn ZIndexProperty<Impl: IMapElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VisibleProperty<Impl: IMapElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementStatics, BASE_OFFSET>(),
            ZIndexProperty: ZIndexProperty::<Impl, IMPL_OFFSET>,
            VisibleProperty: VisibleProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStatics2_Impl: Sized {
    fn MapTabIndexProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementStatics2_Vtbl {
        unsafe extern "system" fn MapTabIndexProperty<Impl: IMapElementStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementStatics2, BASE_OFFSET>(),
            MapTabIndexProperty: MapTabIndexProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStatics3_Impl: Sized {
    fn MapStyleSheetEntryProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MapStyleSheetEntryStateProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TagProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementStatics3_Vtbl {
        unsafe extern "system" fn MapStyleSheetEntryProperty<Impl: IMapElementStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapStyleSheetEntryStateProperty<Impl: IMapElementStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TagProperty<Impl: IMapElementStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementStatics3, BASE_OFFSET>(),
            MapStyleSheetEntryProperty: MapStyleSheetEntryProperty::<Impl, IMPL_OFFSET>,
            MapStyleSheetEntryStateProperty: MapStyleSheetEntryStateProperty::<Impl, IMPL_OFFSET>,
            TagProperty: TagProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStatics4_Impl: Sized {
    fn IsEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementStatics4_Vtbl {
        unsafe extern "system" fn IsEnabledProperty<Impl: IMapElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementStatics4, BASE_OFFSET>(),
            IsEnabledProperty: IsEnabledProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapElementsLayer_Impl: Sized {
    fn MapElements(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
    fn SetMapElements(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::Collections::IVector<MapElement>>) -> ::windows::core::Result<()>;
    fn MapElementClick(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementClick(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerEntered(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerEntered(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerExited(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerExited(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapContextRequested(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerContextRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapContextRequested(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementsLayer {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapElementsLayer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayer_Vtbl {
        unsafe extern "system" fn MapElements<Impl: IMapElementsLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMapElements<Impl: IMapElementsLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapElements(&*(&value as *const <super::super::super::super::Foundation::Collections::IVector<MapElement> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVector<MapElement> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementClick<Impl: IMapElementsLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapElementClick<Impl: IMapElementsLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerEntered<Impl: IMapElementsLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapElementPointerEntered<Impl: IMapElementsLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerEntered(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapElementPointerExited<Impl: IMapElementsLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapElementPointerExited<Impl: IMapElementsLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapElementPointerExited(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MapContextRequested<Impl: IMapElementsLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMapContextRequested<Impl: IMapElementsLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapContextRequested(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementsLayer, BASE_OFFSET>(),
            MapElements: MapElements::<Impl, IMPL_OFFSET>,
            SetMapElements: SetMapElements::<Impl, IMPL_OFFSET>,
            MapElementClick: MapElementClick::<Impl, IMPL_OFFSET>,
            RemoveMapElementClick: RemoveMapElementClick::<Impl, IMPL_OFFSET>,
            MapElementPointerEntered: MapElementPointerEntered::<Impl, IMPL_OFFSET>,
            RemoveMapElementPointerEntered: RemoveMapElementPointerEntered::<Impl, IMPL_OFFSET>,
            MapElementPointerExited: MapElementPointerExited::<Impl, IMPL_OFFSET>,
            RemoveMapElementPointerExited: RemoveMapElementPointerExited::<Impl, IMPL_OFFSET>,
            MapContextRequested: MapContextRequested::<Impl, IMPL_OFFSET>,
            RemoveMapContextRequested: RemoveMapContextRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapElementsLayerClickEventArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementsLayerClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerClickEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapElementsLayerClickEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerClickEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerClickEventArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerClickEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapElementsLayerClickEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapElements<Impl: IMapElementsLayerClickEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementsLayerClickEventArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            MapElements: MapElements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayerClickEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapElementsLayerContextRequestedEventArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementsLayerContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerContextRequestedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapElementsLayerContextRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerContextRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerContextRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapElementsLayerContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapElements<Impl: IMapElementsLayerContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementsLayerContextRequestedEventArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            MapElements: MapElements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayerContextRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapElementsLayerPointerEnteredEventArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&mut self) -> ::windows::core::Result<MapElement>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementsLayerPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerPointerEnteredEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapElementsLayerPointerEnteredEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerPointerEnteredEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerPointerEnteredEventArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerPointerEnteredEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapElementsLayerPointerEnteredEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapElement<Impl: IMapElementsLayerPointerEnteredEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementsLayerPointerEnteredEventArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            MapElement: MapElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayerPointerEnteredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapElementsLayerPointerExitedEventArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&mut self) -> ::windows::core::Result<MapElement>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapElementsLayerPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerPointerExitedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapElementsLayerPointerExitedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerPointerExitedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerPointerExitedEventArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IMapElementsLayerPointerExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapElementsLayerPointerExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MapElement<Impl: IMapElementsLayerPointerExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementsLayerPointerExitedEventArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            MapElement: MapElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayerPointerExitedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerStatics_Impl: Sized {
    fn MapElementsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapElementsLayerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapElementsLayerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapElementsLayerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapElementsLayerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapElementsLayerStatics_Vtbl {
        unsafe extern "system" fn MapElementsProperty<Impl: IMapElementsLayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapElementsLayerStatics, BASE_OFFSET>(),
            MapElementsProperty: MapElementsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapElementsLayerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMapIcon_Impl: Sized {
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&mut self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NormalizedAnchorPoint(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetNormalizedAnchorPoint(&mut self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Image(&mut self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetImage(&mut self, value: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapIcon {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapIcon";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMapIcon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapIcon_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapIcon_Vtbl {
        unsafe extern "system" fn Location<Impl: IMapIcon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocation<Impl: IMapIcon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Title<Impl: IMapIcon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IMapIcon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalizedAnchorPoint<Impl: IMapIcon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNormalizedAnchorPoint<Impl: IMapIcon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalizedAnchorPoint(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IMapIcon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetImage<Impl: IMapIcon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapIcon, BASE_OFFSET>(),
            Location: Location::<Impl, IMPL_OFFSET>,
            SetLocation: SetLocation::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            NormalizedAnchorPoint: NormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
            SetNormalizedAnchorPoint: SetNormalizedAnchorPoint::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            SetImage: SetImage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapIcon as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIcon2_Impl: Sized {
    fn CollisionBehaviorDesired(&mut self) -> ::windows::core::Result<MapElementCollisionBehavior>;
    fn SetCollisionBehaviorDesired(&mut self, value: MapElementCollisionBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapIcon2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapIcon2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapIcon2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapIcon2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapIcon2_Vtbl {
        unsafe extern "system" fn CollisionBehaviorDesired<Impl: IMapIcon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapElementCollisionBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCollisionBehaviorDesired<Impl: IMapIcon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapElementCollisionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCollisionBehaviorDesired(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapIcon2, BASE_OFFSET>(),
            CollisionBehaviorDesired: CollisionBehaviorDesired::<Impl, IMPL_OFFSET>,
            SetCollisionBehaviorDesired: SetCollisionBehaviorDesired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapIcon2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIconStatics_Impl: Sized {
    fn LocationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TitleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn NormalizedAnchorPointProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapIconStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapIconStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapIconStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapIconStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapIconStatics_Vtbl {
        unsafe extern "system" fn LocationProperty<Impl: IMapIconStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TitleProperty<Impl: IMapIconStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NormalizedAnchorPointProperty<Impl: IMapIconStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapIconStatics, BASE_OFFSET>(),
            LocationProperty: LocationProperty::<Impl, IMPL_OFFSET>,
            TitleProperty: TitleProperty::<Impl, IMPL_OFFSET>,
            NormalizedAnchorPointProperty: NormalizedAnchorPointProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapIconStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIconStatics2_Impl: Sized {
    fn CollisionBehaviorDesiredProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapIconStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapIconStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapIconStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapIconStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapIconStatics2_Vtbl {
        unsafe extern "system" fn CollisionBehaviorDesiredProperty<Impl: IMapIconStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapIconStatics2, BASE_OFFSET>(),
            CollisionBehaviorDesiredProperty: CollisionBehaviorDesiredProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapIconStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapInputEventArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapInputEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapInputEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapInputEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapInputEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapInputEventArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IMapInputEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapInputEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapInputEventArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapInputEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapItemsControl_Impl: Sized {
    fn ItemsSource(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetItemsSource(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Items(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>>;
    fn ItemTemplate(&mut self) -> ::windows::core::Result<super::super::DataTemplate>;
    fn SetItemTemplate(&mut self, value: &::core::option::Option<super::super::DataTemplate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapItemsControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapItemsControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapItemsControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapItemsControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapItemsControl_Vtbl {
        unsafe extern "system" fn ItemsSource<Impl: IMapItemsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetItemsSource<Impl: IMapItemsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemsSource(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Items<Impl: IMapItemsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemTemplate<Impl: IMapItemsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetItemTemplate<Impl: IMapItemsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemTemplate(&*(&value as *const <super::super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::super::DataTemplate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapItemsControl, BASE_OFFSET>(),
            ItemsSource: ItemsSource::<Impl, IMPL_OFFSET>,
            SetItemsSource: SetItemsSource::<Impl, IMPL_OFFSET>,
            Items: Items::<Impl, IMPL_OFFSET>,
            ItemTemplate: ItemTemplate::<Impl, IMPL_OFFSET>,
            SetItemTemplate: SetItemTemplate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapItemsControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapItemsControlStatics_Impl: Sized {
    fn ItemsSourceProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemTemplateProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapItemsControlStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapItemsControlStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapItemsControlStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapItemsControlStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapItemsControlStatics_Vtbl {
        unsafe extern "system" fn ItemsSourceProperty<Impl: IMapItemsControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemsProperty<Impl: IMapItemsControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemTemplateProperty<Impl: IMapItemsControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapItemsControlStatics, BASE_OFFSET>(),
            ItemsSourceProperty: ItemsSourceProperty::<Impl, IMPL_OFFSET>,
            ItemsProperty: ItemsProperty::<Impl, IMPL_OFFSET>,
            ItemTemplateProperty: ItemTemplateProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapItemsControlStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLayer_Impl: Sized {
    fn MapTabIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetMapTabIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Visible(&mut self) -> ::windows::core::Result<bool>;
    fn SetVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ZIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapLayer {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapLayer";
}
#[cfg(feature = "implement_exclusive")]
impl IMapLayer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapLayer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapLayer_Vtbl {
        unsafe extern "system" fn MapTabIndex<Impl: IMapLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMapTabIndex<Impl: IMapLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapTabIndex(value).into()
        }
        unsafe extern "system" fn Visible<Impl: IMapLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVisible<Impl: IMapLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        unsafe extern "system" fn ZIndex<Impl: IMapLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetZIndex<Impl: IMapLayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapLayer, BASE_OFFSET>(),
            MapTabIndex: MapTabIndex::<Impl, IMPL_OFFSET>,
            SetMapTabIndex: SetMapTabIndex::<Impl, IMPL_OFFSET>,
            Visible: Visible::<Impl, IMPL_OFFSET>,
            SetVisible: SetVisible::<Impl, IMPL_OFFSET>,
            ZIndex: ZIndex::<Impl, IMPL_OFFSET>,
            SetZIndex: SetZIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapLayer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLayerFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapLayer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapLayerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapLayerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapLayerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapLayerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapLayerFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapLayerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapLayerFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapLayerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLayerStatics_Impl: Sized {
    fn MapTabIndexProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VisibleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZIndexProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapLayerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapLayerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapLayerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapLayerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapLayerStatics_Vtbl {
        unsafe extern "system" fn MapTabIndexProperty<Impl: IMapLayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VisibleProperty<Impl: IMapLayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ZIndexProperty<Impl: IMapLayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapLayerStatics, BASE_OFFSET>(),
            MapTabIndexProperty: MapTabIndexProperty::<Impl, IMPL_OFFSET>,
            VisibleProperty: VisibleProperty::<Impl, IMPL_OFFSET>,
            ZIndexProperty: ZIndexProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapLayerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapModel3D_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapModel3D {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapModel3D";
}
#[cfg(feature = "implement_exclusive")]
impl IMapModel3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapModel3D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapModel3D_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapModel3D, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapModel3D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapModel3DFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapModel3D>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapModel3DFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapModel3DFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapModel3DFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapModel3DFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapModel3DFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapModel3DFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapModel3DFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapModel3DFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMapModel3DStatics_Impl: Sized {
    fn CreateFrom3MFAsync(&mut self, source: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>>;
    fn CreateFrom3MFWithShadingOptionAsync(&mut self, source: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>, shadingoption: MapModel3DShadingOption) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapModel3DStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapModel3DStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMapModel3DStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapModel3DStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapModel3DStatics_Vtbl {
        unsafe extern "system" fn CreateFrom3MFAsync<Impl: IMapModel3DStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFrom3MFWithShadingOptionAsync<Impl: IMapModel3DStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, shadingoption: MapModel3DShadingOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapModel3DStatics, BASE_OFFSET>(),
            CreateFrom3MFAsync: CreateFrom3MFAsync::<Impl, IMPL_OFFSET>,
            CreateFrom3MFWithShadingOptionAsync: CreateFrom3MFWithShadingOptionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapModel3DStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IMapPolygon_Impl: Sized {
    fn Path(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath>;
    fn SetPath(&mut self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopath>) -> ::windows::core::Result<()>;
    fn StrokeColor(&mut self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetStrokeColor(&mut self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn StrokeThickness(&mut self) -> ::windows::core::Result<f64>;
    fn SetStrokeThickness(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeDashed(&mut self) -> ::windows::core::Result<bool>;
    fn SetStrokeDashed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn FillColor(&mut self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetFillColor(&mut self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapPolygon {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolygon";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapPolygon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapPolygon_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapPolygon_Vtbl {
        unsafe extern "system" fn Path<Impl: IMapPolygon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPath<Impl: IMapPolygon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeColor<Impl: IMapPolygon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeColor<Impl: IMapPolygon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeThickness<Impl: IMapPolygon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeThickness<Impl: IMapPolygon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeThickness(value).into()
        }
        unsafe extern "system" fn StrokeDashed<Impl: IMapPolygon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeDashed<Impl: IMapPolygon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashed(value).into()
        }
        unsafe extern "system" fn FillColor<Impl: IMapPolygon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFillColor<Impl: IMapPolygon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapPolygon, BASE_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
            StrokeColor: StrokeColor::<Impl, IMPL_OFFSET>,
            SetStrokeColor: SetStrokeColor::<Impl, IMPL_OFFSET>,
            StrokeThickness: StrokeThickness::<Impl, IMPL_OFFSET>,
            SetStrokeThickness: SetStrokeThickness::<Impl, IMPL_OFFSET>,
            StrokeDashed: StrokeDashed::<Impl, IMPL_OFFSET>,
            SetStrokeDashed: SetStrokeDashed::<Impl, IMPL_OFFSET>,
            FillColor: FillColor::<Impl, IMPL_OFFSET>,
            SetFillColor: SetFillColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapPolygon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapPolygon2_Impl: Sized {
    fn Paths(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Devices::Geolocation::Geopath>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapPolygon2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolygon2";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapPolygon2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapPolygon2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapPolygon2_Vtbl {
        unsafe extern "system" fn Paths<Impl: IMapPolygon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapPolygon2, BASE_OFFSET>(), Paths: Paths::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapPolygon2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolygonStatics_Impl: Sized {
    fn PathProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StrokeThicknessProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StrokeDashedProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapPolygonStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolygonStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapPolygonStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapPolygonStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapPolygonStatics_Vtbl {
        unsafe extern "system" fn PathProperty<Impl: IMapPolygonStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StrokeThicknessProperty<Impl: IMapPolygonStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StrokeDashedProperty<Impl: IMapPolygonStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapPolygonStatics, BASE_OFFSET>(),
            PathProperty: PathProperty::<Impl, IMPL_OFFSET>,
            StrokeThicknessProperty: StrokeThicknessProperty::<Impl, IMPL_OFFSET>,
            StrokeDashedProperty: StrokeDashedProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapPolygonStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IMapPolyline_Impl: Sized {
    fn Path(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath>;
    fn SetPath(&mut self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopath>) -> ::windows::core::Result<()>;
    fn StrokeColor(&mut self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetStrokeColor(&mut self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn StrokeThickness(&mut self) -> ::windows::core::Result<f64>;
    fn SetStrokeThickness(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeDashed(&mut self) -> ::windows::core::Result<bool>;
    fn SetStrokeDashed(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapPolyline {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolyline";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapPolyline_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapPolyline_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapPolyline_Vtbl {
        unsafe extern "system" fn Path<Impl: IMapPolyline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPath<Impl: IMapPolyline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::Geopath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeColor<Impl: IMapPolyline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeColor<Impl: IMapPolyline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeThickness<Impl: IMapPolyline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeThickness<Impl: IMapPolyline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeThickness(value).into()
        }
        unsafe extern "system" fn StrokeDashed<Impl: IMapPolyline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeDashed<Impl: IMapPolyline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashed(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapPolyline, BASE_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
            StrokeColor: StrokeColor::<Impl, IMPL_OFFSET>,
            SetStrokeColor: SetStrokeColor::<Impl, IMPL_OFFSET>,
            StrokeThickness: StrokeThickness::<Impl, IMPL_OFFSET>,
            SetStrokeThickness: SetStrokeThickness::<Impl, IMPL_OFFSET>,
            StrokeDashed: StrokeDashed::<Impl, IMPL_OFFSET>,
            SetStrokeDashed: SetStrokeDashed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapPolyline as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolylineStatics_Impl: Sized {
    fn PathProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StrokeDashedProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapPolylineStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapPolylineStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapPolylineStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapPolylineStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapPolylineStatics_Vtbl {
        unsafe extern "system" fn PathProperty<Impl: IMapPolylineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StrokeDashedProperty<Impl: IMapPolylineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapPolylineStatics, BASE_OFFSET>(),
            PathProperty: PathProperty::<Impl, IMPL_OFFSET>,
            StrokeDashedProperty: StrokeDashedProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapPolylineStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapRightTappedEventArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapRightTappedEventArgs";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IMapRightTappedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapRightTappedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapRightTappedEventArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IMapRightTappedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Location<Impl: IMapRightTappedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapRightTappedEventArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapRightTappedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
pub trait IMapRouteView_Impl: Sized {
    fn RouteColor(&mut self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetRouteColor(&mut self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn OutlineColor(&mut self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetOutlineColor(&mut self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn Route(&mut self) -> ::windows::core::Result<super::super::super::super::Services::Maps::MapRoute>;
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapRouteView {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapRouteView";
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
impl IMapRouteView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapRouteView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapRouteView_Vtbl {
        unsafe extern "system" fn RouteColor<Impl: IMapRouteView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRouteColor<Impl: IMapRouteView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRouteColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutlineColor<Impl: IMapRouteView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutlineColor<Impl: IMapRouteView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutlineColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Route<Impl: IMapRouteView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapRouteView, BASE_OFFSET>(),
            RouteColor: RouteColor::<Impl, IMPL_OFFSET>,
            SetRouteColor: SetRouteColor::<Impl, IMPL_OFFSET>,
            OutlineColor: OutlineColor::<Impl, IMPL_OFFSET>,
            SetOutlineColor: SetOutlineColor::<Impl, IMPL_OFFSET>,
            Route: Route::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapRouteView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
pub trait IMapRouteViewFactory_Impl: Sized {
    fn CreateInstanceWithMapRoute(&mut self, route: &::core::option::Option<super::super::super::super::Services::Maps::MapRoute>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapRouteView>;
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapRouteViewFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapRouteViewFactory";
}
#[cfg(all(feature = "Services_Maps", feature = "implement_exclusive"))]
impl IMapRouteViewFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapRouteViewFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapRouteViewFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithMapRoute<Impl: IMapRouteViewFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, route: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapRouteViewFactory, BASE_OFFSET>(),
            CreateInstanceWithMapRoute: CreateInstanceWithMapRoute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapRouteViewFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapScene_Impl: Sized {
    fn TargetCamera(&mut self) -> ::windows::core::Result<MapCamera>;
    fn TargetCameraChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapScene, MapTargetCameraChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetCameraChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapScene {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapScene";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapScene_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapScene_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapScene_Vtbl {
        unsafe extern "system" fn TargetCamera<Impl: IMapScene_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TargetCameraChanged<Impl: IMapScene_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTargetCameraChanged<Impl: IMapScene_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTargetCameraChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapScene, BASE_OFFSET>(),
            TargetCamera: TargetCamera::<Impl, IMPL_OFFSET>,
            TargetCameraChanged: TargetCameraChanged::<Impl, IMPL_OFFSET>,
            RemoveTargetCameraChanged: RemoveTargetCameraChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapScene as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapSceneStatics_Impl: Sized {
    fn CreateFromBoundingBox(&mut self, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>) -> ::windows::core::Result<MapScene>;
    fn CreateFromBoundingBoxWithHeadingAndPitch(&mut self, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromCamera(&mut self, camera: &::core::option::Option<MapCamera>) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocation(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationWithHeadingAndPitch(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationAndRadius(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, radiusinmeters: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationAndRadiusWithHeadingAndPitch(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, radiusinmeters: f64, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocations(&mut self, locations: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint>>) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationsWithHeadingAndPitch(&mut self, locations: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint>>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapSceneStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapSceneStatics";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapSceneStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapSceneStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapSceneStatics_Vtbl {
        unsafe extern "system" fn CreateFromBoundingBox<Impl: IMapSceneStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromBoundingBoxWithHeadingAndPitch<Impl: IMapSceneStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromCamera<Impl: IMapSceneStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, camera: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromLocation<Impl: IMapSceneStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromLocationWithHeadingAndPitch<Impl: IMapSceneStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromLocationAndRadius<Impl: IMapSceneStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromLocationAndRadiusWithHeadingAndPitch<Impl: IMapSceneStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromLocations<Impl: IMapSceneStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromLocationsWithHeadingAndPitch<Impl: IMapSceneStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locations: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapSceneStatics, BASE_OFFSET>(),
            CreateFromBoundingBox: CreateFromBoundingBox::<Impl, IMPL_OFFSET>,
            CreateFromBoundingBoxWithHeadingAndPitch: CreateFromBoundingBoxWithHeadingAndPitch::<Impl, IMPL_OFFSET>,
            CreateFromCamera: CreateFromCamera::<Impl, IMPL_OFFSET>,
            CreateFromLocation: CreateFromLocation::<Impl, IMPL_OFFSET>,
            CreateFromLocationWithHeadingAndPitch: CreateFromLocationWithHeadingAndPitch::<Impl, IMPL_OFFSET>,
            CreateFromLocationAndRadius: CreateFromLocationAndRadius::<Impl, IMPL_OFFSET>,
            CreateFromLocationAndRadiusWithHeadingAndPitch: CreateFromLocationAndRadiusWithHeadingAndPitch::<Impl, IMPL_OFFSET>,
            CreateFromLocations: CreateFromLocations::<Impl, IMPL_OFFSET>,
            CreateFromLocationsWithHeadingAndPitch: CreateFromLocationsWithHeadingAndPitch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapSceneStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheet_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapStyleSheet {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapStyleSheet";
}
#[cfg(feature = "implement_exclusive")]
impl IMapStyleSheet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapStyleSheet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapStyleSheet_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapStyleSheet, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapStyleSheet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheetEntriesStatics_Impl: Sized {
    fn Area(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Airport(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Cemetery(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Continent(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Education(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IndigenousPeoplesReserve(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Island(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Medical(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Military(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Nautical(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Neighborhood(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Runway(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sand(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShoppingCenter(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Stadium(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Vegetation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Forest(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GolfCourse(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Park(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PlayingField(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Reserve(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Point(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NaturalPoint(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Peak(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VolcanicPeak(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WaterPoint(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PointOfInterest(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Business(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FoodPoint(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PopulatedPlace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capital(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdminDistrictCapital(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CountryRegionCapital(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoadShield(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoadExit(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transit(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Political(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CountryRegion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdminDistrict(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn District(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Structure(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Building(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EducationBuilding(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MedicalBuilding(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransitBuilding(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transportation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Road(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ControlledAccessHighway(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HighSpeedRamp(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Highway(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MajorRoad(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ArterialRoad(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Street(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ramp(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UnpavedStreet(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TollRoad(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Railway(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Trail(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WaterRoute(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Water(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn River(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RouteLine(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WalkingRoute(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DrivingRoute(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapStyleSheetEntriesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapStyleSheetEntriesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapStyleSheetEntriesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapStyleSheetEntriesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapStyleSheetEntriesStatics_Vtbl {
        unsafe extern "system" fn Area<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Airport<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cemetery<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Continent<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Education<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IndigenousPeoplesReserve<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Island<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Medical<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Military<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Nautical<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Neighborhood<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Runway<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Sand<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShoppingCenter<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Stadium<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Vegetation<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Forest<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GolfCourse<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Park<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PlayingField<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Reserve<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Point<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NaturalPoint<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Peak<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VolcanicPeak<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WaterPoint<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PointOfInterest<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Business<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FoodPoint<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PopulatedPlace<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capital<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdminDistrictCapital<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CountryRegionCapital<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoadShield<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoadExit<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Transit<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Political<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CountryRegion<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdminDistrict<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn District<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Structure<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Building<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EducationBuilding<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MedicalBuilding<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransitBuilding<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Transportation<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Road<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ControlledAccessHighway<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HighSpeedRamp<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Highway<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MajorRoad<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ArterialRoad<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Street<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Ramp<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnpavedStreet<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TollRoad<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Railway<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Trail<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WaterRoute<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Water<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn River<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RouteLine<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WalkingRoute<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DrivingRoute<Impl: IMapStyleSheetEntriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapStyleSheetEntriesStatics, BASE_OFFSET>(),
            Area: Area::<Impl, IMPL_OFFSET>,
            Airport: Airport::<Impl, IMPL_OFFSET>,
            Cemetery: Cemetery::<Impl, IMPL_OFFSET>,
            Continent: Continent::<Impl, IMPL_OFFSET>,
            Education: Education::<Impl, IMPL_OFFSET>,
            IndigenousPeoplesReserve: IndigenousPeoplesReserve::<Impl, IMPL_OFFSET>,
            Island: Island::<Impl, IMPL_OFFSET>,
            Medical: Medical::<Impl, IMPL_OFFSET>,
            Military: Military::<Impl, IMPL_OFFSET>,
            Nautical: Nautical::<Impl, IMPL_OFFSET>,
            Neighborhood: Neighborhood::<Impl, IMPL_OFFSET>,
            Runway: Runway::<Impl, IMPL_OFFSET>,
            Sand: Sand::<Impl, IMPL_OFFSET>,
            ShoppingCenter: ShoppingCenter::<Impl, IMPL_OFFSET>,
            Stadium: Stadium::<Impl, IMPL_OFFSET>,
            Vegetation: Vegetation::<Impl, IMPL_OFFSET>,
            Forest: Forest::<Impl, IMPL_OFFSET>,
            GolfCourse: GolfCourse::<Impl, IMPL_OFFSET>,
            Park: Park::<Impl, IMPL_OFFSET>,
            PlayingField: PlayingField::<Impl, IMPL_OFFSET>,
            Reserve: Reserve::<Impl, IMPL_OFFSET>,
            Point: Point::<Impl, IMPL_OFFSET>,
            NaturalPoint: NaturalPoint::<Impl, IMPL_OFFSET>,
            Peak: Peak::<Impl, IMPL_OFFSET>,
            VolcanicPeak: VolcanicPeak::<Impl, IMPL_OFFSET>,
            WaterPoint: WaterPoint::<Impl, IMPL_OFFSET>,
            PointOfInterest: PointOfInterest::<Impl, IMPL_OFFSET>,
            Business: Business::<Impl, IMPL_OFFSET>,
            FoodPoint: FoodPoint::<Impl, IMPL_OFFSET>,
            PopulatedPlace: PopulatedPlace::<Impl, IMPL_OFFSET>,
            Capital: Capital::<Impl, IMPL_OFFSET>,
            AdminDistrictCapital: AdminDistrictCapital::<Impl, IMPL_OFFSET>,
            CountryRegionCapital: CountryRegionCapital::<Impl, IMPL_OFFSET>,
            RoadShield: RoadShield::<Impl, IMPL_OFFSET>,
            RoadExit: RoadExit::<Impl, IMPL_OFFSET>,
            Transit: Transit::<Impl, IMPL_OFFSET>,
            Political: Political::<Impl, IMPL_OFFSET>,
            CountryRegion: CountryRegion::<Impl, IMPL_OFFSET>,
            AdminDistrict: AdminDistrict::<Impl, IMPL_OFFSET>,
            District: District::<Impl, IMPL_OFFSET>,
            Structure: Structure::<Impl, IMPL_OFFSET>,
            Building: Building::<Impl, IMPL_OFFSET>,
            EducationBuilding: EducationBuilding::<Impl, IMPL_OFFSET>,
            MedicalBuilding: MedicalBuilding::<Impl, IMPL_OFFSET>,
            TransitBuilding: TransitBuilding::<Impl, IMPL_OFFSET>,
            Transportation: Transportation::<Impl, IMPL_OFFSET>,
            Road: Road::<Impl, IMPL_OFFSET>,
            ControlledAccessHighway: ControlledAccessHighway::<Impl, IMPL_OFFSET>,
            HighSpeedRamp: HighSpeedRamp::<Impl, IMPL_OFFSET>,
            Highway: Highway::<Impl, IMPL_OFFSET>,
            MajorRoad: MajorRoad::<Impl, IMPL_OFFSET>,
            ArterialRoad: ArterialRoad::<Impl, IMPL_OFFSET>,
            Street: Street::<Impl, IMPL_OFFSET>,
            Ramp: Ramp::<Impl, IMPL_OFFSET>,
            UnpavedStreet: UnpavedStreet::<Impl, IMPL_OFFSET>,
            TollRoad: TollRoad::<Impl, IMPL_OFFSET>,
            Railway: Railway::<Impl, IMPL_OFFSET>,
            Trail: Trail::<Impl, IMPL_OFFSET>,
            WaterRoute: WaterRoute::<Impl, IMPL_OFFSET>,
            Water: Water::<Impl, IMPL_OFFSET>,
            River: River::<Impl, IMPL_OFFSET>,
            RouteLine: RouteLine::<Impl, IMPL_OFFSET>,
            WalkingRoute: WalkingRoute::<Impl, IMPL_OFFSET>,
            DrivingRoute: DrivingRoute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapStyleSheetEntriesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheetEntryStatesStatics_Impl: Sized {
    fn Disabled(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hover(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Selected(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapStyleSheetEntryStatesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapStyleSheetEntryStatesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapStyleSheetEntryStatesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapStyleSheetEntryStatesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapStyleSheetEntryStatesStatics_Vtbl {
        unsafe extern "system" fn Disabled<Impl: IMapStyleSheetEntryStatesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Hover<Impl: IMapStyleSheetEntryStatesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Selected<Impl: IMapStyleSheetEntryStatesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapStyleSheetEntryStatesStatics, BASE_OFFSET>(),
            Disabled: Disabled::<Impl, IMPL_OFFSET>,
            Hover: Hover::<Impl, IMPL_OFFSET>,
            Selected: Selected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapStyleSheetEntryStatesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMapStyleSheetStatics_Impl: Sized {
    fn Aerial(&mut self) -> ::windows::core::Result<MapStyleSheet>;
    fn AerialWithOverlay(&mut self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadLight(&mut self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadDark(&mut self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadHighContrastLight(&mut self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadHighContrastDark(&mut self) -> ::windows::core::Result<MapStyleSheet>;
    fn Combine(&mut self, stylesheets: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<MapStyleSheet>>) -> ::windows::core::Result<MapStyleSheet>;
    fn ParseFromJson(&mut self, styleasjson: &::windows::core::HSTRING) -> ::windows::core::Result<MapStyleSheet>;
    fn TryParseFromJson(&mut self, styleasjson: &::windows::core::HSTRING, stylesheet: &mut ::core::option::Option<MapStyleSheet>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapStyleSheetStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapStyleSheetStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMapStyleSheetStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapStyleSheetStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapStyleSheetStatics_Vtbl {
        unsafe extern "system" fn Aerial<Impl: IMapStyleSheetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AerialWithOverlay<Impl: IMapStyleSheetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoadLight<Impl: IMapStyleSheetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoadDark<Impl: IMapStyleSheetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoadHighContrastLight<Impl: IMapStyleSheetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoadHighContrastDark<Impl: IMapStyleSheetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Combine<Impl: IMapStyleSheetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ParseFromJson<Impl: IMapStyleSheetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, styleasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParseFromJson<Impl: IMapStyleSheetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, styleasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, stylesheet: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapStyleSheetStatics, BASE_OFFSET>(),
            Aerial: Aerial::<Impl, IMPL_OFFSET>,
            AerialWithOverlay: AerialWithOverlay::<Impl, IMPL_OFFSET>,
            RoadLight: RoadLight::<Impl, IMPL_OFFSET>,
            RoadDark: RoadDark::<Impl, IMPL_OFFSET>,
            RoadHighContrastLight: RoadHighContrastLight::<Impl, IMPL_OFFSET>,
            RoadHighContrastDark: RoadHighContrastDark::<Impl, IMPL_OFFSET>,
            Combine: Combine::<Impl, IMPL_OFFSET>,
            ParseFromJson: ParseFromJson::<Impl, IMPL_OFFSET>,
            TryParseFromJson: TryParseFromJson::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapStyleSheetStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTargetCameraChangedEventArgs_Impl: Sized {
    fn Camera(&mut self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTargetCameraChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTargetCameraChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTargetCameraChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTargetCameraChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTargetCameraChangedEventArgs_Vtbl {
        unsafe extern "system" fn Camera<Impl: IMapTargetCameraChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTargetCameraChangedEventArgs, BASE_OFFSET>(), Camera: Camera::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTargetCameraChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTargetCameraChangedEventArgs2_Impl: Sized {
    fn ChangeReason(&mut self) -> ::windows::core::Result<MapCameraChangeReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTargetCameraChangedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTargetCameraChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTargetCameraChangedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTargetCameraChangedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTargetCameraChangedEventArgs2_Vtbl {
        unsafe extern "system" fn ChangeReason<Impl: IMapTargetCameraChangedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTargetCameraChangedEventArgs2, BASE_OFFSET>(),
            ChangeReason: ChangeReason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTargetCameraChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMapTileBitmapRequest_Impl: Sized {
    fn PixelData(&mut self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetPixelData(&mut self, value: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<MapTileBitmapRequestDeferral>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapTileBitmapRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileBitmapRequest";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMapTileBitmapRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileBitmapRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileBitmapRequest_Vtbl {
        unsafe extern "system" fn PixelData<Impl: IMapTileBitmapRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPixelData<Impl: IMapTileBitmapRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPixelData(&*(&value as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMapTileBitmapRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileBitmapRequest, BASE_OFFSET>(),
            PixelData: PixelData::<Impl, IMPL_OFFSET>,
            SetPixelData: SetPixelData::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileBitmapRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestDeferral_Impl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileBitmapRequestDeferral {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileBitmapRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileBitmapRequestDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileBitmapRequestDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileBitmapRequestDeferral_Vtbl {
        unsafe extern "system" fn Complete<Impl: IMapTileBitmapRequestDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileBitmapRequestDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileBitmapRequestDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestedEventArgs_Impl: Sized {
    fn X(&mut self) -> ::windows::core::Result<i32>;
    fn Y(&mut self) -> ::windows::core::Result<i32>;
    fn ZoomLevel(&mut self) -> ::windows::core::Result<i32>;
    fn Request(&mut self) -> ::windows::core::Result<MapTileBitmapRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileBitmapRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileBitmapRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileBitmapRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileBitmapRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileBitmapRequestedEventArgs_Vtbl {
        unsafe extern "system" fn X<Impl: IMapTileBitmapRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Y<Impl: IMapTileBitmapRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ZoomLevel<Impl: IMapTileBitmapRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Request<Impl: IMapTileBitmapRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileBitmapRequestedEventArgs, BASE_OFFSET>(),
            X: X::<Impl, IMPL_OFFSET>,
            Y: Y::<Impl, IMPL_OFFSET>,
            ZoomLevel: ZoomLevel::<Impl, IMPL_OFFSET>,
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileBitmapRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestedEventArgs2_Impl: Sized {
    fn FrameIndex(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileBitmapRequestedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileBitmapRequestedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileBitmapRequestedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileBitmapRequestedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileBitmapRequestedEventArgs2_Vtbl {
        unsafe extern "system" fn FrameIndex<Impl: IMapTileBitmapRequestedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileBitmapRequestedEventArgs2, BASE_OFFSET>(),
            FrameIndex: FrameIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileBitmapRequestedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileDataSource_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileDataSource";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileDataSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileDataSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileDataSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileDataSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileDataSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileDataSourceFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileDataSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileDataSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileDataSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileDataSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileDataSourceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapTileDataSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileDataSourceFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileDataSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IMapTileSource_Impl: Sized {
    fn DataSource(&mut self) -> ::windows::core::Result<MapTileDataSource>;
    fn SetDataSource(&mut self, value: &::core::option::Option<MapTileDataSource>) -> ::windows::core::Result<()>;
    fn Layer(&mut self) -> ::windows::core::Result<MapTileLayer>;
    fn SetLayer(&mut self, value: MapTileLayer) -> ::windows::core::Result<()>;
    fn ZoomLevelRange(&mut self) -> ::windows::core::Result<MapZoomLevelRange>;
    fn SetZoomLevelRange(&mut self, value: &MapZoomLevelRange) -> ::windows::core::Result<()>;
    fn Bounds(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::GeoboundingBox>;
    fn SetBounds(&mut self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>) -> ::windows::core::Result<()>;
    fn AllowOverstretch(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowOverstretch(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsFadingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsFadingEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsTransparencyEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsTransparencyEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsRetryEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsRetryEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ZIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn TilePixelSize(&mut self) -> ::windows::core::Result<i32>;
    fn SetTilePixelSize(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Visible(&mut self) -> ::windows::core::Result<bool>;
    fn SetVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapTileSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSource";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapTileSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileSource_Vtbl {
        unsafe extern "system" fn DataSource<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDataSource<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataSource(&*(&value as *const <MapTileDataSource as ::windows::core::Abi>::Abi as *const <MapTileDataSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Layer<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapTileLayer) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLayer<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapTileLayer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLayer(value).into()
        }
        unsafe extern "system" fn ZoomLevelRange<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapZoomLevelRange) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetZoomLevelRange<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MapZoomLevelRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoomLevelRange(&*(&value as *const <MapZoomLevelRange as ::windows::core::Abi>::Abi as *const <MapZoomLevelRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bounds<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBounds<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBounds(&*(&value as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowOverstretch<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowOverstretch<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowOverstretch(value).into()
        }
        unsafe extern "system" fn IsFadingEnabled<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsFadingEnabled<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsFadingEnabled(value).into()
        }
        unsafe extern "system" fn IsTransparencyEnabled<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsTransparencyEnabled<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTransparencyEnabled(value).into()
        }
        unsafe extern "system" fn IsRetryEnabled<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsRetryEnabled<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRetryEnabled(value).into()
        }
        unsafe extern "system" fn ZIndex<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetZIndex<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        unsafe extern "system" fn TilePixelSize<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTilePixelSize<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTilePixelSize(value).into()
        }
        unsafe extern "system" fn Visible<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVisible<Impl: IMapTileSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileSource, BASE_OFFSET>(),
            DataSource: DataSource::<Impl, IMPL_OFFSET>,
            SetDataSource: SetDataSource::<Impl, IMPL_OFFSET>,
            Layer: Layer::<Impl, IMPL_OFFSET>,
            SetLayer: SetLayer::<Impl, IMPL_OFFSET>,
            ZoomLevelRange: ZoomLevelRange::<Impl, IMPL_OFFSET>,
            SetZoomLevelRange: SetZoomLevelRange::<Impl, IMPL_OFFSET>,
            Bounds: Bounds::<Impl, IMPL_OFFSET>,
            SetBounds: SetBounds::<Impl, IMPL_OFFSET>,
            AllowOverstretch: AllowOverstretch::<Impl, IMPL_OFFSET>,
            SetAllowOverstretch: SetAllowOverstretch::<Impl, IMPL_OFFSET>,
            IsFadingEnabled: IsFadingEnabled::<Impl, IMPL_OFFSET>,
            SetIsFadingEnabled: SetIsFadingEnabled::<Impl, IMPL_OFFSET>,
            IsTransparencyEnabled: IsTransparencyEnabled::<Impl, IMPL_OFFSET>,
            SetIsTransparencyEnabled: SetIsTransparencyEnabled::<Impl, IMPL_OFFSET>,
            IsRetryEnabled: IsRetryEnabled::<Impl, IMPL_OFFSET>,
            SetIsRetryEnabled: SetIsRetryEnabled::<Impl, IMPL_OFFSET>,
            ZIndex: ZIndex::<Impl, IMPL_OFFSET>,
            SetZIndex: SetZIndex::<Impl, IMPL_OFFSET>,
            TilePixelSize: TilePixelSize::<Impl, IMPL_OFFSET>,
            SetTilePixelSize: SetTilePixelSize::<Impl, IMPL_OFFSET>,
            Visible: Visible::<Impl, IMPL_OFFSET>,
            SetVisible: SetVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapTileSource2_Impl: Sized {
    fn AnimationState(&mut self) -> ::windows::core::Result<MapTileAnimationState>;
    fn AutoPlay(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn FrameCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetFrameCount(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn FrameDuration(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn SetFrameDuration(&mut self, value: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Play(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapTileSource2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSource2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapTileSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileSource2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileSource2_Vtbl {
        unsafe extern "system" fn AnimationState<Impl: IMapTileSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MapTileAnimationState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoPlay<Impl: IMapTileSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutoPlay<Impl: IMapTileSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoPlay(value).into()
        }
        unsafe extern "system" fn FrameCount<Impl: IMapTileSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFrameCount<Impl: IMapTileSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameCount(value).into()
        }
        unsafe extern "system" fn FrameDuration<Impl: IMapTileSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFrameDuration<Impl: IMapTileSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameDuration(&*(&value as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pause<Impl: IMapTileSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Play<Impl: IMapTileSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play().into()
        }
        unsafe extern "system" fn Stop<Impl: IMapTileSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileSource2, BASE_OFFSET>(),
            AnimationState: AnimationState::<Impl, IMPL_OFFSET>,
            AutoPlay: AutoPlay::<Impl, IMPL_OFFSET>,
            SetAutoPlay: SetAutoPlay::<Impl, IMPL_OFFSET>,
            FrameCount: FrameCount::<Impl, IMPL_OFFSET>,
            SetFrameCount: SetFrameCount::<Impl, IMPL_OFFSET>,
            FrameDuration: FrameDuration::<Impl, IMPL_OFFSET>,
            SetFrameDuration: SetFrameDuration::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Play: Play::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IMapTileSourceFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSource(&mut self, datasource: &::core::option::Option<MapTileDataSource>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceAndZoomRange(&mut self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceZoomRangeAndBounds(&mut self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize(&mut self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, tilesizeinpixels: i32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapTileSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSourceFactory";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IMapTileSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileSourceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMapTileSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithDataSource<Impl: IMapTileSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithDataSourceAndZoomRange<Impl: IMapTileSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithDataSourceZoomRangeAndBounds<Impl: IMapTileSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, bounds: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize<Impl: IMapTileSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, bounds: ::windows::core::RawPtr, tilesizeinpixels: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileSourceFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDataSource: CreateInstanceWithDataSource::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDataSourceAndZoomRange: CreateInstanceWithDataSourceAndZoomRange::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDataSourceZoomRangeAndBounds: CreateInstanceWithDataSourceZoomRangeAndBounds::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize: CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSourceStatics_Impl: Sized {
    fn DataSourceProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LayerProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZoomLevelRangeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn BoundsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AllowOverstretchProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsFadingEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsTransparencyEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsRetryEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZIndexProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TilePixelSizeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VisibleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileSourceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileSourceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileSourceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileSourceStatics_Vtbl {
        unsafe extern "system" fn DataSourceProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LayerProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ZoomLevelRangeProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BoundsProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AllowOverstretchProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsFadingEnabledProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsTransparencyEnabledProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsRetryEnabledProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ZIndexProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TilePixelSizeProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VisibleProperty<Impl: IMapTileSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileSourceStatics, BASE_OFFSET>(),
            DataSourceProperty: DataSourceProperty::<Impl, IMPL_OFFSET>,
            LayerProperty: LayerProperty::<Impl, IMPL_OFFSET>,
            ZoomLevelRangeProperty: ZoomLevelRangeProperty::<Impl, IMPL_OFFSET>,
            BoundsProperty: BoundsProperty::<Impl, IMPL_OFFSET>,
            AllowOverstretchProperty: AllowOverstretchProperty::<Impl, IMPL_OFFSET>,
            IsFadingEnabledProperty: IsFadingEnabledProperty::<Impl, IMPL_OFFSET>,
            IsTransparencyEnabledProperty: IsTransparencyEnabledProperty::<Impl, IMPL_OFFSET>,
            IsRetryEnabledProperty: IsRetryEnabledProperty::<Impl, IMPL_OFFSET>,
            ZIndexProperty: ZIndexProperty::<Impl, IMPL_OFFSET>,
            TilePixelSizeProperty: TilePixelSizeProperty::<Impl, IMPL_OFFSET>,
            VisibleProperty: VisibleProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSourceStatics2_Impl: Sized {
    fn AnimationStateProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AutoPlayProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FrameCountProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FrameDurationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileSourceStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileSourceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileSourceStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileSourceStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileSourceStatics2_Vtbl {
        unsafe extern "system" fn AnimationStateProperty<Impl: IMapTileSourceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoPlayProperty<Impl: IMapTileSourceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FrameCountProperty<Impl: IMapTileSourceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FrameDurationProperty<Impl: IMapTileSourceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileSourceStatics2, BASE_OFFSET>(),
            AnimationStateProperty: AnimationStateProperty::<Impl, IMPL_OFFSET>,
            AutoPlayProperty: AutoPlayProperty::<Impl, IMPL_OFFSET>,
            FrameCountProperty: FrameCountProperty::<Impl, IMPL_OFFSET>,
            FrameDurationProperty: FrameDurationProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileSourceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMapTileUriRequest_Impl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn SetUri(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<MapTileUriRequestDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapTileUriRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileUriRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMapTileUriRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileUriRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileUriRequest_Vtbl {
        unsafe extern "system" fn Uri<Impl: IMapTileUriRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUri<Impl: IMapTileUriRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMapTileUriRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileUriRequest, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            SetUri: SetUri::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileUriRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestDeferral_Impl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileUriRequestDeferral {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileUriRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileUriRequestDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileUriRequestDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileUriRequestDeferral_Vtbl {
        unsafe extern "system" fn Complete<Impl: IMapTileUriRequestDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileUriRequestDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileUriRequestDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestedEventArgs_Impl: Sized {
    fn X(&mut self) -> ::windows::core::Result<i32>;
    fn Y(&mut self) -> ::windows::core::Result<i32>;
    fn ZoomLevel(&mut self) -> ::windows::core::Result<i32>;
    fn Request(&mut self) -> ::windows::core::Result<MapTileUriRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileUriRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileUriRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileUriRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileUriRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileUriRequestedEventArgs_Vtbl {
        unsafe extern "system" fn X<Impl: IMapTileUriRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Y<Impl: IMapTileUriRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ZoomLevel<Impl: IMapTileUriRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Request<Impl: IMapTileUriRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileUriRequestedEventArgs, BASE_OFFSET>(),
            X: X::<Impl, IMPL_OFFSET>,
            Y: Y::<Impl, IMPL_OFFSET>,
            ZoomLevel: ZoomLevel::<Impl, IMPL_OFFSET>,
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileUriRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestedEventArgs2_Impl: Sized {
    fn FrameIndex(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapTileUriRequestedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IMapTileUriRequestedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapTileUriRequestedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapTileUriRequestedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapTileUriRequestedEventArgs2_Vtbl {
        unsafe extern "system" fn FrameIndex<Impl: IMapTileUriRequestedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapTileUriRequestedEventArgs2, BASE_OFFSET>(),
            FrameIndex: FrameIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapTileUriRequestedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreetsideExperience_Impl: Sized {
    fn AddressTextVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetAddressTextVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CursorVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetCursorVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn OverviewMapVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetOverviewMapVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn StreetLabelsVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetStreetLabelsVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ExitButtonVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetExitButtonVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ZoomButtonsVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetZoomButtonsVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreetsideExperience {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IStreetsideExperience";
}
#[cfg(feature = "implement_exclusive")]
impl IStreetsideExperience_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreetsideExperience_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreetsideExperience_Vtbl {
        unsafe extern "system" fn AddressTextVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAddressTextVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddressTextVisible(value).into()
        }
        unsafe extern "system" fn CursorVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCursorVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCursorVisible(value).into()
        }
        unsafe extern "system" fn OverviewMapVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOverviewMapVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverviewMapVisible(value).into()
        }
        unsafe extern "system" fn StreetLabelsVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStreetLabelsVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreetLabelsVisible(value).into()
        }
        unsafe extern "system" fn ExitButtonVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExitButtonVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitButtonVisible(value).into()
        }
        unsafe extern "system" fn ZoomButtonsVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetZoomButtonsVisible<Impl: IStreetsideExperience_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoomButtonsVisible(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreetsideExperience, BASE_OFFSET>(),
            AddressTextVisible: AddressTextVisible::<Impl, IMPL_OFFSET>,
            SetAddressTextVisible: SetAddressTextVisible::<Impl, IMPL_OFFSET>,
            CursorVisible: CursorVisible::<Impl, IMPL_OFFSET>,
            SetCursorVisible: SetCursorVisible::<Impl, IMPL_OFFSET>,
            OverviewMapVisible: OverviewMapVisible::<Impl, IMPL_OFFSET>,
            SetOverviewMapVisible: SetOverviewMapVisible::<Impl, IMPL_OFFSET>,
            StreetLabelsVisible: StreetLabelsVisible::<Impl, IMPL_OFFSET>,
            SetStreetLabelsVisible: SetStreetLabelsVisible::<Impl, IMPL_OFFSET>,
            ExitButtonVisible: ExitButtonVisible::<Impl, IMPL_OFFSET>,
            SetExitButtonVisible: SetExitButtonVisible::<Impl, IMPL_OFFSET>,
            ZoomButtonsVisible: ZoomButtonsVisible::<Impl, IMPL_OFFSET>,
            SetZoomButtonsVisible: SetZoomButtonsVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreetsideExperience as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreetsideExperienceFactory_Impl: Sized {
    fn CreateInstanceWithPanorama(&mut self, panorama: &::core::option::Option<StreetsidePanorama>) -> ::windows::core::Result<StreetsideExperience>;
    fn CreateInstanceWithPanoramaHeadingPitchAndFieldOfView(&mut self, panorama: &::core::option::Option<StreetsidePanorama>, headingindegrees: f64, pitchindegrees: f64, fieldofviewindegrees: f64) -> ::windows::core::Result<StreetsideExperience>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreetsideExperienceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IStreetsideExperienceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStreetsideExperienceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreetsideExperienceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreetsideExperienceFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithPanorama<Impl: IStreetsideExperienceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panorama: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithPanoramaHeadingPitchAndFieldOfView<Impl: IStreetsideExperienceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panorama: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, fieldofviewindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreetsideExperienceFactory, BASE_OFFSET>(),
            CreateInstanceWithPanorama: CreateInstanceWithPanorama::<Impl, IMPL_OFFSET>,
            CreateInstanceWithPanoramaHeadingPitchAndFieldOfView: CreateInstanceWithPanoramaHeadingPitchAndFieldOfView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreetsideExperienceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IStreetsidePanorama_Impl: Sized {
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreetsidePanorama {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IStreetsidePanorama";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IStreetsidePanorama_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreetsidePanorama_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreetsidePanorama_Vtbl {
        unsafe extern "system" fn Location<Impl: IStreetsidePanorama_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStreetsidePanorama, BASE_OFFSET>(), Location: Location::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreetsidePanorama as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreetsidePanoramaStatics_Impl: Sized {
    fn FindNearbyWithLocationAsync(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>>;
    fn FindNearbyWithLocationAndRadiusAsync(&mut self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, radiusinmeters: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreetsidePanoramaStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.IStreetsidePanoramaStatics";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IStreetsidePanoramaStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreetsidePanoramaStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreetsidePanoramaStatics_Vtbl {
        unsafe extern "system" fn FindNearbyWithLocationAsync<Impl: IStreetsidePanoramaStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindNearbyWithLocationAndRadiusAsync<Impl: IStreetsidePanoramaStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreetsidePanoramaStatics, BASE_OFFSET>(),
            FindNearbyWithLocationAsync: FindNearbyWithLocationAsync::<Impl, IMPL_OFFSET>,
            FindNearbyWithLocationAndRadiusAsync: FindNearbyWithLocationAndRadiusAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreetsidePanoramaStatics as ::windows::core::Interface>::IID
    }
}
