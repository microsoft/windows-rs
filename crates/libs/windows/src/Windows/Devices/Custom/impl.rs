#[doc = "*Required features: `\"Devices_Custom\"`, `\"implement\"`*"]
pub trait IIOControlCode_Impl: Sized {
    fn AccessMode(&self) -> ::windows::core::Result<IOControlAccessMode>;
    fn BufferingMethod(&self) -> ::windows::core::Result<IOControlBufferingMethod>;
    fn Function(&self) -> ::windows::core::Result<u16>;
    fn DeviceType(&self) -> ::windows::core::Result<u16>;
    fn ControlCode(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IIOControlCode {
    const NAME: &'static str = "Windows.Devices.Custom.IIOControlCode";
}
impl IIOControlCode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: isize>() -> IIOControlCode_Vtbl {
        unsafe extern "system" fn AccessMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IOControlAccessMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccessMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferingMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IOControlBufferingMethod) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BufferingMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Function<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Function() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ControlCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IIOControlCode, OFFSET>(),
            AccessMode: AccessMode::<Identity, Impl, OFFSET>,
            BufferingMethod: BufferingMethod::<Identity, Impl, OFFSET>,
            Function: Function::<Identity, Impl, OFFSET>,
            DeviceType: DeviceType::<Identity, Impl, OFFSET>,
            ControlCode: ControlCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIOControlCode as ::windows::core::ComInterface>::IID
    }
}
