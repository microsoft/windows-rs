#[cfg(feature = "deprecated")]
pub trait IAllJoynAcceptSessionJoiner_Impl: Sized {
    fn Accept(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IAllJoynAcceptSessionJoiner {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynAcceptSessionJoiner";
}
#[cfg(feature = "deprecated")]
impl IAllJoynAcceptSessionJoiner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAllJoynAcceptSessionJoiner_Impl, const OFFSET: isize>() -> IAllJoynAcceptSessionJoiner_Vtbl {
        unsafe extern "system" fn Accept<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAllJoynAcceptSessionJoiner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Accept().into()
        }
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, IAllJoynAcceptSessionJoiner, OFFSET>(), Accept: Accept::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAllJoynAcceptSessionJoiner as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait IAllJoynProducer_Impl: Sized {
    fn SetBusObject(&self, busobject: &::core::option::Option<AllJoynBusObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IAllJoynProducer {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynProducer";
}
#[cfg(feature = "deprecated")]
impl IAllJoynProducer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAllJoynProducer_Impl, const OFFSET: isize>() -> IAllJoynProducer_Vtbl {
        unsafe extern "system" fn SetBusObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAllJoynProducer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBusObject(::core::mem::transmute(&busobject)).into()
        }
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, IAllJoynProducer, OFFSET>(), SetBusObject: SetBusObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAllJoynProducer as ::windows::core::Interface>::IID
    }
}
