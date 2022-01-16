#[cfg(feature = "deprecated")]
pub trait IAllJoynAcceptSessionJoiner_Impl: Sized {
    fn Accept(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IAllJoynAcceptSessionJoiner {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynAcceptSessionJoiner";
}
#[cfg(feature = "deprecated")]
impl IAllJoynAcceptSessionJoiner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynAcceptSessionJoiner_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAllJoynAcceptSessionJoiner_Vtbl {
        unsafe extern "system" fn Accept<Impl: IAllJoynAcceptSessionJoiner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAllJoynAcceptSessionJoiner, BASE_OFFSET>(), Accept: Accept::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAllJoynAcceptSessionJoiner as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait IAllJoynProducer_Impl: Sized {
    fn SetBusObject(&mut self, busobject: &::core::option::Option<AllJoynBusObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IAllJoynProducer {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynProducer";
}
#[cfg(feature = "deprecated")]
impl IAllJoynProducer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynProducer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAllJoynProducer_Vtbl {
        unsafe extern "system" fn SetBusObject<Impl: IAllJoynProducer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusObject(&*(&busobject as *const <AllJoynBusObject as ::windows::core::Abi>::Abi as *const <AllJoynBusObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAllJoynProducer, BASE_OFFSET>(), SetBusObject: SetBusObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAllJoynProducer as ::windows::core::Interface>::IID
    }
}
