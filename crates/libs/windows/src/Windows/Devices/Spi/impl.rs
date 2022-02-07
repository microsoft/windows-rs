#[cfg(feature = "Foundation")]
pub trait ISpiDeviceStatics_Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetBusInfo(&self, busid: &::windows::core::HSTRING) -> ::windows::core::Result<SpiBusInfo>;
    fn FromIdAsync(&self, busid: &::windows::core::HSTRING, settings: &::core::option::Option<SpiConnectionSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpiDevice>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISpiDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Spi.ISpiDeviceStatics";
}
#[cfg(feature = "Foundation")]
impl ISpiDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceStatics_Impl, const OFFSET: isize>() -> ISpiDeviceStatics_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromFriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceSelectorFromFriendlyName(::core::mem::transmute(&friendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBusInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBusInfo(::core::mem::transmute(&busid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FromIdAsync(::core::mem::transmute(&busid), ::core::mem::transmute(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiDeviceStatics, OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Identity, Impl, OFFSET>,
            GetDeviceSelectorFromFriendlyName: GetDeviceSelectorFromFriendlyName::<Identity, Impl, OFFSET>,
            GetBusInfo: GetBusInfo::<Identity, Impl, OFFSET>,
            FromIdAsync: FromIdAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiDeviceStatics as ::windows::core::Interface>::IID
    }
}
