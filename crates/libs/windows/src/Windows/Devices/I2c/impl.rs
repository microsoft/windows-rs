#[cfg(feature = "Foundation")]
pub trait II2cDeviceStatics_Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING, settings: &::core::option::Option<I2cConnectionSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for II2cDeviceStatics {
    const NAME: &'static str = "Windows.Devices.I2c.II2cDeviceStatics";
}
#[cfg(feature = "Foundation")]
impl II2cDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceStatics_Impl, const OFFSET: isize>() -> II2cDeviceStatics_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromFriendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceSelectorFromFriendlyName(::core::mem::transmute(&friendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FromIdAsync(::core::mem::transmute(&deviceid), ::core::mem::transmute(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, II2cDeviceStatics, OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Identity, Impl, OFFSET>,
            GetDeviceSelectorFromFriendlyName: GetDeviceSelectorFromFriendlyName::<Identity, Impl, OFFSET>,
            FromIdAsync: FromIdAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<II2cDeviceStatics as ::windows::core::Interface>::IID
    }
}
