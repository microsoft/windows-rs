#[cfg(feature = "implement_exclusive")]
pub trait IVibrationDeviceImpl: Sized {
    fn Vibrate(&self, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVibrationDevice {
    const NAME: &'static str = "Windows.Phone.Devices.Notification.IVibrationDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IVibrationDeviceVtbl {
    pub const fn new<Impl: IVibrationDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVibrationDeviceVtbl {
        unsafe extern "system" fn Vibrate<Impl: IVibrationDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Vibrate(&*(&duration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Cancel<Impl: IVibrationDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVibrationDevice>, base.5, Vibrate::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
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
    pub const fn new<Impl: IVibrationDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVibrationDeviceStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IVibrationDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVibrationDeviceStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
