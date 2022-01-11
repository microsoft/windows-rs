#[cfg(feature = "Win32_Foundation")]
pub trait ICloneViewHelperImpl: Sized {
    fn GetConnectedIDs();
    fn GetActiveTopology();
    fn SetActiveTopology();
    fn Commit();
}
#[cfg(feature = "Win32_Foundation")]
impl ICloneViewHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICloneViewHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICloneViewHelperVtbl {
        unsafe extern "system" fn GetConnectedIDs<Impl: ICloneViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActiveTopology<Impl: ICloneViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActiveTopology<Impl: ICloneViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: ICloneViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffinalcall: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetConnectedIDs: GetConnectedIDs::<Impl, IMPL_OFFSET>,
            GetActiveTopology: GetActiveTopology::<Impl, IMPL_OFFSET>,
            SetActiveTopology: SetActiveTopology::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICloneViewHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IViewHelperImpl: Sized {
    fn GetConnectedIDs();
    fn GetActiveTopology();
    fn SetActiveTopology();
    fn Commit();
    fn SetConfiguration();
    fn GetProceedOnNewConfiguration();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IViewHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewHelperVtbl {
        unsafe extern "system" fn GetConnectedIDs<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActiveTopology<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActiveTopology<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConfiguration<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProceedOnNewConfiguration<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetConnectedIDs: GetConnectedIDs::<Impl, IMPL_OFFSET>,
            GetActiveTopology: GetActiveTopology::<Impl, IMPL_OFFSET>,
            SetActiveTopology: SetActiveTopology::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            SetConfiguration: SetConfiguration::<Impl, IMPL_OFFSET>,
            GetProceedOnNewConfiguration: GetProceedOnNewConfiguration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewHelper as ::windows::core::Interface>::IID
    }
}
