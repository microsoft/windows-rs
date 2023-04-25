#[doc = "*Required features: `\"Devices_Spi\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait ISpiDeviceStatics_Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetBusInfo(&self, busid: &::windows_core::HSTRING) -> ::windows_core::Result<SpiBusInfo>;
    fn FromIdAsync(&self, busid: &::windows_core::HSTRING, settings: ::core::option::Option<&SpiConnectionSettings>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SpiDevice>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for ISpiDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Spi.ISpiDeviceStatics";
}
#[cfg(feature = "Foundation")]
impl ISpiDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceStatics_Impl, const OFFSET: isize>() -> ISpiDeviceStatics_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromFriendlyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceSelectorFromFriendlyName(::core::mem::transmute(&friendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBusInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBusInfo(::core::mem::transmute(&busid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FromIdAsync(::core::mem::transmute(&busid), ::windows_core::from_raw_borrowed(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISpiDeviceStatics, OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Identity, Impl, OFFSET>,
            GetDeviceSelectorFromFriendlyName: GetDeviceSelectorFromFriendlyName::<Identity, Impl, OFFSET>,
            GetBusInfo: GetBusInfo::<Identity, Impl, OFFSET>,
            FromIdAsync: FromIdAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpiDeviceStatics as ::windows_core::ComInterface>::IID
    }
}
