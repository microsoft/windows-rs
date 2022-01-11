#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVibrationDeviceImpl: Sized {
    fn Vibrate(&self, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVibrationDevice {
    const NAME: &'static str = "Windows.Phone.Devices.Notification.IVibrationDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVibrationDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVibrationDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVibrationDeviceVtbl {
        unsafe extern "system" fn Vibrate<Impl: IVibrationDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Vibrate(&*(&duration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Cancel<Impl: IVibrationDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVibrationDevice>, ::windows::core::GetTrustLevel, Vibrate::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVibrationDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVibrationDeviceStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<VibrationDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVibrationDeviceStatics {
    const NAME: &'static str = "Windows.Phone.Devices.Notification.IVibrationDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVibrationDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVibrationDeviceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVibrationDeviceStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IVibrationDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVibrationDeviceStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVibrationDeviceStatics as ::windows::core::Interface>::IID
    }
}
