pub trait IAccIdentity_Impl: Sized {
    fn GetIdentityString(&mut self, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::Result<()>;
}
impl IAccIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccIdentity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccIdentity_Vtbl {
        unsafe extern "system" fn GetIdentityString<Impl: IAccIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdentityString(::core::mem::transmute_copy(&dwidchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIdentityString: GetIdentityString::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAccPropServer_Impl: Sized {
    fn GetPropValue(&mut self, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAccPropServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccPropServer_Vtbl {
        unsafe extern "system" fn GetPropValue<Impl: IAccPropServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropValue(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&pfhasprop)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetPropValue: GetPropValue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccPropServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IAccPropServices_Impl: Sized {
    fn SetPropValue(&mut self, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetPropServer(&mut self, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::core::option::Option<IAccPropServer>, annoscope: AnnoScope) -> ::windows::core::Result<()>;
    fn ClearProps(&mut self, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::Result<()>;
    fn SetHwndProp(&mut self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetHwndPropStr(&mut self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetHwndPropServer(&mut self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::core::option::Option<IAccPropServer>, annoscope: AnnoScope) -> ::windows::core::Result<()>;
    fn ClearHwndProps(&mut self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::Result<()>;
    fn ComposeHwndIdentityString(&mut self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::Result<()>;
    fn DecomposeHwndIdentityString(&mut self, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows::core::Result<()>;
    fn SetHmenuProp(&mut self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetHmenuPropStr(&mut self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetHmenuPropServer(&mut self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::core::option::Option<IAccPropServer>, annoscope: AnnoScope) -> ::windows::core::Result<()>;
    fn ClearHmenuProps(&mut self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::Result<()>;
    fn ComposeHmenuIdentityString(&mut self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::Result<()>;
    fn DecomposeHmenuIdentityString(&mut self, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl IAccPropServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccPropServices_Vtbl {
        unsafe extern "system" fn SetPropValue<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropValue(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn SetPropServer<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropServer(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::core::mem::transmute(&pserver), ::core::mem::transmute_copy(&annoscope)).into()
        }
        unsafe extern "system" fn ClearProps<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearProps(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into()
        }
        unsafe extern "system" fn SetHwndProp<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHwndProp(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn SetHwndPropStr<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHwndPropStr(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&str)).into()
        }
        unsafe extern "system" fn SetHwndPropServer<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHwndPropServer(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::core::mem::transmute(&pserver), ::core::mem::transmute_copy(&annoscope)).into()
        }
        unsafe extern "system" fn ClearHwndProps<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearHwndProps(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into()
        }
        unsafe extern "system" fn ComposeHwndIdentityString<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ComposeHwndIdentityString(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into()
        }
        unsafe extern "system" fn DecomposeHwndIdentityString<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecomposeHwndIdentityString(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pidobject), ::core::mem::transmute_copy(&pidchild)).into()
        }
        unsafe extern "system" fn SetHmenuProp<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHmenuProp(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn SetHmenuPropStr<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHmenuPropStr(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&str)).into()
        }
        unsafe extern "system" fn SetHmenuPropServer<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHmenuPropServer(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::core::mem::transmute(&pserver), ::core::mem::transmute_copy(&annoscope)).into()
        }
        unsafe extern "system" fn ClearHmenuProps<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearHmenuProps(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into()
        }
        unsafe extern "system" fn ComposeHmenuIdentityString<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ComposeHmenuIdentityString(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into()
        }
        unsafe extern "system" fn DecomposeHmenuIdentityString<Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecomposeHmenuIdentityString(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&phmenu), ::core::mem::transmute_copy(&pidchild)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPropValue: SetPropValue::<Impl, IMPL_OFFSET>,
            SetPropServer: SetPropServer::<Impl, IMPL_OFFSET>,
            ClearProps: ClearProps::<Impl, IMPL_OFFSET>,
            SetHwndProp: SetHwndProp::<Impl, IMPL_OFFSET>,
            SetHwndPropStr: SetHwndPropStr::<Impl, IMPL_OFFSET>,
            SetHwndPropServer: SetHwndPropServer::<Impl, IMPL_OFFSET>,
            ClearHwndProps: ClearHwndProps::<Impl, IMPL_OFFSET>,
            ComposeHwndIdentityString: ComposeHwndIdentityString::<Impl, IMPL_OFFSET>,
            DecomposeHwndIdentityString: DecomposeHwndIdentityString::<Impl, IMPL_OFFSET>,
            SetHmenuProp: SetHmenuProp::<Impl, IMPL_OFFSET>,
            SetHmenuPropStr: SetHmenuPropStr::<Impl, IMPL_OFFSET>,
            SetHmenuPropServer: SetHmenuPropServer::<Impl, IMPL_OFFSET>,
            ClearHmenuProps: ClearHmenuProps::<Impl, IMPL_OFFSET>,
            ComposeHmenuIdentityString: ComposeHmenuIdentityString::<Impl, IMPL_OFFSET>,
            DecomposeHmenuIdentityString: DecomposeHmenuIdentityString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccPropServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAccessible_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn accParent(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn accChildCount(&mut self) -> ::windows::core::Result<i32>;
    fn accChild(&mut self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn accName(&mut self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accValue(&mut self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accDescription(&mut self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accRole(&mut self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accState(&mut self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accHelp(&mut self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accHelpTopic(&mut self, pszhelpfile: *mut super::super::Foundation::BSTR, varchild: super::super::System::Com::VARIANT, pidtopic: *mut i32) -> ::windows::core::Result<()>;
    fn accKeyboardShortcut(&mut self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accFocus(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accSelection(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accDefaultAction(&mut self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accSelect(&mut self, flagsselect: i32, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn accLocation(&mut self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn accNavigate(&mut self, navdir: i32, varstart: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accHitTest(&mut self, xleft: i32, ytop: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accDoDefaultAction(&mut self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetaccName(&mut self, varchild: super::super::System::Com::VARIANT, szname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetaccValue(&mut self, varchild: super::super::System::Com::VARIANT, szvalue: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAccessible_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessible_Vtbl {
        unsafe extern "system" fn accParent<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdispparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accParent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdispparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accChildCount<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcountchildren: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accChildCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcountchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accChild<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppdispchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accChild(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdispchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accName<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accName(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accValue<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accValue(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accDescription<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accDescription(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accRole<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarrole: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accRole(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accState<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarstate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accState(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accHelp<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accHelp(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszhelp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accHelpTopic<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelpfile: *mut super::super::Foundation::BSTR, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pidtopic: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).accHelpTopic(::core::mem::transmute_copy(&pszhelpfile), ::core::mem::transmute_copy(&varchild), ::core::mem::transmute_copy(&pidtopic)).into()
        }
        unsafe extern "system" fn accKeyboardShortcut<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accKeyboardShortcut(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszkeyboardshortcut = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accFocus<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accSelection<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarchildren: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accDefaultAction<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accDefaultAction(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszdefaultaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accSelect<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).accSelect(::core::mem::transmute_copy(&flagsselect), ::core::mem::transmute_copy(&varchild)).into()
        }
        unsafe extern "system" fn accLocation<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).accLocation(::core::mem::transmute_copy(&pxleft), ::core::mem::transmute_copy(&pytop), ::core::mem::transmute_copy(&pcxwidth), ::core::mem::transmute_copy(&pcyheight), ::core::mem::transmute_copy(&varchild)).into()
        }
        unsafe extern "system" fn accNavigate<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, navdir: i32, varstart: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarendupat: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accNavigate(::core::mem::transmute_copy(&navdir), ::core::mem::transmute_copy(&varstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarendupat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accHitTest<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xleft: i32, ytop: i32, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accHitTest(::core::mem::transmute_copy(&xleft), ::core::mem::transmute_copy(&ytop)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accDoDefaultAction<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).accDoDefaultAction(::core::mem::transmute_copy(&varchild)).into()
        }
        unsafe extern "system" fn SetaccName<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, szname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetaccName(::core::mem::transmute_copy(&varchild), ::core::mem::transmute_copy(&szname)).into()
        }
        unsafe extern "system" fn SetaccValue<Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, szvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetaccValue(::core::mem::transmute_copy(&varchild), ::core::mem::transmute_copy(&szvalue)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            accParent: accParent::<Impl, IMPL_OFFSET>,
            accChildCount: accChildCount::<Impl, IMPL_OFFSET>,
            accChild: accChild::<Impl, IMPL_OFFSET>,
            accName: accName::<Impl, IMPL_OFFSET>,
            accValue: accValue::<Impl, IMPL_OFFSET>,
            accDescription: accDescription::<Impl, IMPL_OFFSET>,
            accRole: accRole::<Impl, IMPL_OFFSET>,
            accState: accState::<Impl, IMPL_OFFSET>,
            accHelp: accHelp::<Impl, IMPL_OFFSET>,
            accHelpTopic: accHelpTopic::<Impl, IMPL_OFFSET>,
            accKeyboardShortcut: accKeyboardShortcut::<Impl, IMPL_OFFSET>,
            accFocus: accFocus::<Impl, IMPL_OFFSET>,
            accSelection: accSelection::<Impl, IMPL_OFFSET>,
            accDefaultAction: accDefaultAction::<Impl, IMPL_OFFSET>,
            accSelect: accSelect::<Impl, IMPL_OFFSET>,
            accLocation: accLocation::<Impl, IMPL_OFFSET>,
            accNavigate: accNavigate::<Impl, IMPL_OFFSET>,
            accHitTest: accHitTest::<Impl, IMPL_OFFSET>,
            accDoDefaultAction: accDoDefaultAction::<Impl, IMPL_OFFSET>,
            SetaccName: SetaccName::<Impl, IMPL_OFFSET>,
            SetaccValue: SetaccValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessible as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleEx_Impl: Sized {
    fn GetObjectForChild(&mut self, idchild: i32) -> ::windows::core::Result<IAccessibleEx>;
    fn GetIAccessiblePair(&mut self, ppacc: *mut ::core::option::Option<IAccessible>, pidchild: *mut i32) -> ::windows::core::Result<()>;
    fn GetRuntimeId(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ConvertReturnedElement(&mut self, pin: ::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<IAccessibleEx>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibleEx_Vtbl {
        unsafe extern "system" fn GetObjectForChild<Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idchild: i32, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectForChild(::core::mem::transmute_copy(&idchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIAccessiblePair<Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacc: *mut ::windows::core::RawPtr, pidchild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIAccessiblePair(::core::mem::transmute_copy(&ppacc), ::core::mem::transmute_copy(&pidchild)).into()
        }
        unsafe extern "system" fn GetRuntimeId<Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertReturnedElement<Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, ppretvalout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertReturnedElement(::core::mem::transmute(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppretvalout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetObjectForChild: GetObjectForChild::<Impl, IMPL_OFFSET>,
            GetIAccessiblePair: GetIAccessiblePair::<Impl, IMPL_OFFSET>,
            GetRuntimeId: GetRuntimeId::<Impl, IMPL_OFFSET>,
            ConvertReturnedElement: ConvertReturnedElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessibleEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleHandler_Impl: Sized {
    fn AccessibleObjectFromID(&mut self, hwnd: i32, lobjectid: i32) -> ::windows::core::Result<IAccessible>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibleHandler_Vtbl {
        unsafe extern "system" fn AccessibleObjectFromID<Impl: IAccessibleHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, lobjectid: i32, piaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessibleObjectFromID(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&lobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *piaccessible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AccessibleObjectFromID: AccessibleObjectFromID::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessibleHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleHostingElementProviders_Impl: Sized {
    fn GetEmbeddedFragmentRoots(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetObjectIdForProvider(&mut self, pprovider: ::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleHostingElementProviders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHostingElementProviders_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibleHostingElementProviders_Vtbl {
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbeddedFragmentRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectIdForProvider<Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, pidobject: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectIdForProvider(::core::mem::transmute(&pprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *pidobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEmbeddedFragmentRoots: GetEmbeddedFragmentRoots::<Impl, IMPL_OFFSET>,
            GetObjectIdForProvider: GetObjectIdForProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessibleHostingElementProviders as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleWindowlessSite_Impl: Sized {
    fn AcquireObjectIdRange(&mut self, rangesize: i32, prangeowner: ::core::option::Option<IAccessibleHandler>) -> ::windows::core::Result<i32>;
    fn ReleaseObjectIdRange(&mut self, rangebase: i32, prangeowner: ::core::option::Option<IAccessibleHandler>) -> ::windows::core::Result<()>;
    fn QueryObjectIdRanges(&mut self, prangesowner: ::core::option::Option<IAccessibleHandler>) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetParentAccessible(&mut self) -> ::windows::core::Result<IAccessible>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleWindowlessSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleWindowlessSite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibleWindowlessSite_Vtbl {
        unsafe extern "system" fn AcquireObjectIdRange<Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangesize: i32, prangeowner: ::windows::core::RawPtr, prangebase: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireObjectIdRange(::core::mem::transmute_copy(&rangesize), ::core::mem::transmute(&prangeowner)) {
                ::core::result::Result::Ok(ok__) => {
                    *prangebase = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseObjectIdRange<Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangebase: i32, prangeowner: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseObjectIdRange(::core::mem::transmute_copy(&rangebase), ::core::mem::transmute(&prangeowner)).into()
        }
        unsafe extern "system" fn QueryObjectIdRanges<Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangesowner: ::windows::core::RawPtr, psaranges: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryObjectIdRanges(::core::mem::transmute(&prangesowner)) {
                ::core::result::Result::Ok(ok__) => {
                    *psaranges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentAccessible<Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentAccessible() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AcquireObjectIdRange: AcquireObjectIdRange::<Impl, IMPL_OFFSET>,
            ReleaseObjectIdRange: ReleaseObjectIdRange::<Impl, IMPL_OFFSET>,
            QueryObjectIdRanges: QueryObjectIdRanges::<Impl, IMPL_OFFSET>,
            GetParentAccessible: GetParentAccessible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessibleWindowlessSite as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAnnotationProvider_Impl: Sized {
    fn AnnotationTypeId(&mut self) -> ::windows::core::Result<i32>;
    fn AnnotationTypeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Author(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DateTime(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Target(&mut self) -> ::windows::core::Result<IRawElementProviderSimple>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAnnotationProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnnotationProvider_Vtbl {
        unsafe extern "system" fn AnnotationTypeId<Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnnotationTypeName<Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateTime<Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Target<Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AnnotationTypeId: AnnotationTypeId::<Impl, IMPL_OFFSET>,
            AnnotationTypeName: AnnotationTypeName::<Impl, IMPL_OFFSET>,
            Author: Author::<Impl, IMPL_OFFSET>,
            DateTime: DateTime::<Impl, IMPL_OFFSET>,
            Target: Target::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnnotationProvider as ::windows::core::Interface>::IID
    }
}
pub trait ICustomNavigationProvider_Impl: Sized {
    fn Navigate(&mut self, direction: NavigateDirection) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ICustomNavigationProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomNavigationProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomNavigationProvider_Vtbl {
        unsafe extern "system" fn Navigate<Impl: ICustomNavigationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigate(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Navigate: Navigate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomNavigationProvider as ::windows::core::Interface>::IID
    }
}
pub trait IDockProvider_Impl: Sized {
    fn SetDockPosition(&mut self, dockposition: DockPosition) -> ::windows::core::Result<()>;
    fn DockPosition(&mut self) -> ::windows::core::Result<DockPosition>;
}
impl IDockProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDockProvider_Vtbl {
        unsafe extern "system" fn SetDockPosition<Impl: IDockProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockposition: DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDockPosition(::core::mem::transmute_copy(&dockposition)).into()
        }
        unsafe extern "system" fn DockPosition<Impl: IDockProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDockPosition: SetDockPosition::<Impl, IMPL_OFFSET>,
            DockPosition: DockPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDockProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDragProvider_Impl: Sized {
    fn IsGrabbed(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DropEffect(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DropEffects(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetGrabbedItems(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDragProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragProvider_Vtbl {
        unsafe extern "system" fn IsGrabbed<Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffect<Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffects<Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGrabbedItems<Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGrabbedItems() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsGrabbed: IsGrabbed::<Impl, IMPL_OFFSET>,
            DropEffect: DropEffect::<Impl, IMPL_OFFSET>,
            DropEffects: DropEffects::<Impl, IMPL_OFFSET>,
            GetGrabbedItems: GetGrabbedItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDropTargetProvider_Impl: Sized {
    fn DropTargetEffect(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DropTargetEffects(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDropTargetProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetProvider_Vtbl {
        unsafe extern "system" fn DropTargetEffect<Impl: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropTargetEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropTargetEffects<Impl: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropTargetEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DropTargetEffect: DropTargetEffect::<Impl, IMPL_OFFSET>,
            DropTargetEffects: DropTargetEffects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTargetProvider as ::windows::core::Interface>::IID
    }
}
pub trait IExpandCollapseProvider_Impl: Sized {
    fn Expand(&mut self) -> ::windows::core::Result<()>;
    fn Collapse(&mut self) -> ::windows::core::Result<()>;
    fn ExpandCollapseState(&mut self) -> ::windows::core::Result<ExpandCollapseState>;
}
impl IExpandCollapseProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExpandCollapseProvider_Vtbl {
        unsafe extern "system" fn Expand<Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Expand().into()
        }
        unsafe extern "system" fn Collapse<Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Collapse().into()
        }
        unsafe extern "system" fn ExpandCollapseState<Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Expand: Expand::<Impl, IMPL_OFFSET>,
            Collapse: Collapse::<Impl, IMPL_OFFSET>,
            ExpandCollapseState: ExpandCollapseState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpandCollapseProvider as ::windows::core::Interface>::IID
    }
}
pub trait IGridItemProvider_Impl: Sized {
    fn Row(&mut self) -> ::windows::core::Result<i32>;
    fn Column(&mut self) -> ::windows::core::Result<i32>;
    fn RowSpan(&mut self) -> ::windows::core::Result<i32>;
    fn ColumnSpan(&mut self) -> ::windows::core::Result<i32>;
    fn ContainingGrid(&mut self) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl IGridItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridItemProvider_Vtbl {
        unsafe extern "system" fn Row<Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Row() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Column<Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Column() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowSpan<Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnSpan<Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainingGrid<Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Row: Row::<Impl, IMPL_OFFSET>,
            Column: Column::<Impl, IMPL_OFFSET>,
            RowSpan: RowSpan::<Impl, IMPL_OFFSET>,
            ColumnSpan: ColumnSpan::<Impl, IMPL_OFFSET>,
            ContainingGrid: ContainingGrid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridItemProvider as ::windows::core::Interface>::IID
    }
}
pub trait IGridProvider_Impl: Sized {
    fn GetItem(&mut self, row: i32, column: i32) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn RowCount(&mut self) -> ::windows::core::Result<i32>;
    fn ColumnCount(&mut self) -> ::windows::core::Result<i32>;
}
impl IGridProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridProvider_Vtbl {
        unsafe extern "system" fn GetItem<Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowCount<Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnCount<Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
            RowCount: RowCount::<Impl, IMPL_OFFSET>,
            ColumnCount: ColumnCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridProvider as ::windows::core::Interface>::IID
    }
}
pub trait IInvokeProvider_Impl: Sized {
    fn Invoke(&mut self) -> ::windows::core::Result<()>;
}
impl IInvokeProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInvokeProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInvokeProvider_Vtbl {
        unsafe extern "system" fn Invoke<Impl: IInvokeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInvokeProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IItemContainerProvider_Impl: Sized {
    fn FindItemByProperty(&mut self, pstartafter: ::core::option::Option<IRawElementProviderSimple>, propertyid: i32, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IItemContainerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemContainerProvider_Vtbl {
        unsafe extern "system" fn FindItemByProperty<Impl: IItemContainerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartafter: ::windows::core::RawPtr, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItemByProperty(::core::mem::transmute(&pstartafter), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfound = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FindItemByProperty: FindItemByProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemContainerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ILegacyIAccessibleProvider_Impl: Sized {
    fn Select(&mut self, flagsselect: i32) -> ::windows::core::Result<()>;
    fn DoDefaultAction(&mut self) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, szvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetIAccessible(&mut self) -> ::windows::core::Result<IAccessible>;
    fn ChildId(&mut self) -> ::windows::core::Result<i32>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Role(&mut self) -> ::windows::core::Result<u32>;
    fn State(&mut self) -> ::windows::core::Result<u32>;
    fn Help(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn KeyboardShortcut(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSelection(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn DefaultAction(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ILegacyIAccessibleProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILegacyIAccessibleProvider_Vtbl {
        unsafe extern "system" fn Select<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Select(::core::mem::transmute_copy(&flagsselect)).into()
        }
        unsafe extern "system" fn DoDefaultAction<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoDefaultAction().into()
        }
        unsafe extern "system" fn SetValue<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&szvalue)).into()
        }
        unsafe extern "system" fn GetIAccessible<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIAccessible() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaccessible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildId<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Role<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Role() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Help<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Help() {
                ::core::result::Result::Ok(ok__) => {
                    *pszhelp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyboardShortcut<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardShortcut() {
                ::core::result::Result::Ok(ok__) => {
                    *pszkeyboardshortcut = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselectedchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultAction<Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdefaultaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Select: Select::<Impl, IMPL_OFFSET>,
            DoDefaultAction: DoDefaultAction::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            GetIAccessible: GetIAccessible::<Impl, IMPL_OFFSET>,
            ChildId: ChildId::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Role: Role::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Help: Help::<Impl, IMPL_OFFSET>,
            KeyboardShortcut: KeyboardShortcut::<Impl, IMPL_OFFSET>,
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            DefaultAction: DefaultAction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILegacyIAccessibleProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMultipleViewProvider_Impl: Sized {
    fn GetViewName(&mut self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCurrentView(&mut self, viewid: i32) -> ::windows::core::Result<()>;
    fn CurrentView(&mut self) -> ::windows::core::Result<i32>;
    fn GetSupportedViews(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMultipleViewProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultipleViewProvider_Vtbl {
        unsafe extern "system" fn GetViewName<Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewName(::core::mem::transmute_copy(&viewid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentView<Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentView(::core::mem::transmute_copy(&viewid)).into()
        }
        unsafe extern "system" fn CurrentView<Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedViews<Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedViews() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetViewName: GetViewName::<Impl, IMPL_OFFSET>,
            SetCurrentView: SetCurrentView::<Impl, IMPL_OFFSET>,
            CurrentView: CurrentView::<Impl, IMPL_OFFSET>,
            GetSupportedViews: GetSupportedViews::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultipleViewProvider as ::windows::core::Interface>::IID
    }
}
pub trait IObjectModelProvider_Impl: Sized {
    fn GetUnderlyingObjectModel(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IObjectModelProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectModelProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectModelProvider_Vtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Impl: IObjectModelProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnderlyingObjectModel() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectModelProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProxyProviderWinEventHandler_Impl: Sized {
    fn RespondToWinEvent(&mut self, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: ::core::option::Option<IProxyProviderWinEventSink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IProxyProviderWinEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProxyProviderWinEventHandler_Vtbl {
        unsafe extern "system" fn RespondToWinEvent<Impl: IProxyProviderWinEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RespondToWinEvent(::core::mem::transmute_copy(&idwinevent), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute(&psink)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), RespondToWinEvent: RespondToWinEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProxyProviderWinEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProxyProviderWinEventSink_Impl: Sized {
    fn AddAutomationPropertyChangedEvent(&mut self, pprovider: ::core::option::Option<IRawElementProviderSimple>, id: i32, newvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddAutomationEvent(&mut self, pprovider: ::core::option::Option<IRawElementProviderSimple>, id: i32) -> ::windows::core::Result<()>;
    fn AddStructureChangedEvent(&mut self, pprovider: ::core::option::Option<IRawElementProviderSimple>, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProxyProviderWinEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProxyProviderWinEventSink_Vtbl {
        unsafe extern "system" fn AddAutomationPropertyChangedEvent<Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, id: i32, newvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAutomationPropertyChangedEvent(::core::mem::transmute(&pprovider), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&newvalue)).into()
        }
        unsafe extern "system" fn AddAutomationEvent<Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAutomationEvent(::core::mem::transmute(&pprovider), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn AddStructureChangedEvent<Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStructureChangedEvent(::core::mem::transmute(&pprovider), ::core::mem::transmute_copy(&structurechangetype), ::core::mem::transmute_copy(&runtimeid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddAutomationPropertyChangedEvent: AddAutomationPropertyChangedEvent::<Impl, IMPL_OFFSET>,
            AddAutomationEvent: AddAutomationEvent::<Impl, IMPL_OFFSET>,
            AddStructureChangedEvent: AddStructureChangedEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProxyProviderWinEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRangeValueProvider_Impl: Sized {
    fn SetValue(&mut self, val: f64) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<f64>;
    fn IsReadOnly(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Maximum(&mut self) -> ::windows::core::Result<f64>;
    fn Minimum(&mut self) -> ::windows::core::Result<f64>;
    fn LargeChange(&mut self) -> ::windows::core::Result<f64>;
    fn SmallChange(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRangeValueProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeValueProvider_Vtbl {
        unsafe extern "system" fn SetValue<Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn Value<Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maximum<Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Maximum() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Minimum<Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Minimum() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeChange<Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmallChange<Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            IsReadOnly: IsReadOnly::<Impl, IMPL_OFFSET>,
            Maximum: Maximum::<Impl, IMPL_OFFSET>,
            Minimum: Minimum::<Impl, IMPL_OFFSET>,
            LargeChange: LargeChange::<Impl, IMPL_OFFSET>,
            SmallChange: SmallChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeValueProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderAdviseEvents_Impl: Sized {
    fn AdviseEventAdded(&mut self, eventid: i32, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn AdviseEventRemoved(&mut self, eventid: i32, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderAdviseEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderAdviseEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderAdviseEvents_Vtbl {
        unsafe extern "system" fn AdviseEventAdded<Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseEventAdded(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyids)).into()
        }
        unsafe extern "system" fn AdviseEventRemoved<Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseEventRemoved(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyids)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseEventAdded: AdviseEventAdded::<Impl, IMPL_OFFSET>,
            AdviseEventRemoved: AdviseEventRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderAdviseEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderFragment_Impl: Sized {
    fn Navigate(&mut self, direction: NavigateDirection) -> ::windows::core::Result<IRawElementProviderFragment>;
    fn GetRuntimeId(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn BoundingRectangle(&mut self) -> ::windows::core::Result<UiaRect>;
    fn GetEmbeddedFragmentRoots(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFocus(&mut self) -> ::windows::core::Result<()>;
    fn FragmentRoot(&mut self) -> ::windows::core::Result<IRawElementProviderFragmentRoot>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderFragment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderFragment_Vtbl {
        unsafe extern "system" fn Navigate<Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigate(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeId<Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingRectangle<Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut UiaRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbeddedFragmentRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocus().into()
        }
        unsafe extern "system" fn FragmentRoot<Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FragmentRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Navigate: Navigate::<Impl, IMPL_OFFSET>,
            GetRuntimeId: GetRuntimeId::<Impl, IMPL_OFFSET>,
            BoundingRectangle: BoundingRectangle::<Impl, IMPL_OFFSET>,
            GetEmbeddedFragmentRoots: GetEmbeddedFragmentRoots::<Impl, IMPL_OFFSET>,
            SetFocus: SetFocus::<Impl, IMPL_OFFSET>,
            FragmentRoot: FragmentRoot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderFragment as ::windows::core::Interface>::IID
    }
}
pub trait IRawElementProviderFragmentRoot_Impl: Sized {
    fn ElementProviderFromPoint(&mut self, x: f64, y: f64) -> ::windows::core::Result<IRawElementProviderFragment>;
    fn GetFocus(&mut self) -> ::windows::core::Result<IRawElementProviderFragment>;
}
impl IRawElementProviderFragmentRoot_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragmentRoot_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderFragmentRoot_Vtbl {
        unsafe extern "system" fn ElementProviderFromPoint<Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementProviderFromPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ElementProviderFromPoint: ElementProviderFromPoint::<Impl, IMPL_OFFSET>,
            GetFocus: GetFocus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderFragmentRoot as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderHostingAccessibles_Impl: Sized {
    fn GetEmbeddedAccessibles(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderHostingAccessibles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderHostingAccessibles_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderHostingAccessibles_Vtbl {
        unsafe extern "system" fn GetEmbeddedAccessibles<Impl: IRawElementProviderHostingAccessibles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbeddedAccessibles() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetEmbeddedAccessibles: GetEmbeddedAccessibles::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderHostingAccessibles as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRawElementProviderHwndOverride_Impl: Sized {
    fn GetOverrideProviderForHwnd(&mut self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<IRawElementProviderSimple>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRawElementProviderHwndOverride_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderHwndOverride_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderHwndOverride_Vtbl {
        unsafe extern "system" fn GetOverrideProviderForHwnd<Impl: IRawElementProviderHwndOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOverrideProviderForHwnd(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetOverrideProviderForHwnd: GetOverrideProviderForHwnd::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderHwndOverride as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRawElementProviderSimple_Impl: Sized {
    fn ProviderOptions(&mut self) -> ::windows::core::Result<ProviderOptions>;
    fn GetPatternProvider(&mut self, patternid: i32) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetPropertyValue(&mut self, propertyid: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn HostRawElementProvider(&mut self) -> ::windows::core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawElementProviderSimple_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderSimple_Vtbl {
        unsafe extern "system" fn ProviderOptions<Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ProviderOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatternProvider<Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatternProvider(::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyValue(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostRawElementProvider<Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostRawElementProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProviderOptions: ProviderOptions::<Impl, IMPL_OFFSET>,
            GetPatternProvider: GetPatternProvider::<Impl, IMPL_OFFSET>,
            GetPropertyValue: GetPropertyValue::<Impl, IMPL_OFFSET>,
            HostRawElementProvider: HostRawElementProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderSimple as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRawElementProviderSimple2_Impl: Sized + IRawElementProviderSimple_Impl {
    fn ShowContextMenu(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawElementProviderSimple2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderSimple2_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: IRawElementProviderSimple2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        Self { base: IRawElementProviderSimple_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ShowContextMenu: ShowContextMenu::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderSimple2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRawElementProviderSimple3_Impl: Sized + IRawElementProviderSimple_Impl + IRawElementProviderSimple2_Impl {
    fn GetMetadataValue(&mut self, targetid: i32, metadataid: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawElementProviderSimple3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderSimple3_Vtbl {
        unsafe extern "system" fn GetMetadataValue<Impl: IRawElementProviderSimple3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: i32, returnval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataValue(::core::mem::transmute_copy(&targetid), ::core::mem::transmute_copy(&metadataid)) {
                ::core::result::Result::Ok(ok__) => {
                    *returnval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IRawElementProviderSimple2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMetadataValue: GetMetadataValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderSimple3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderWindowlessSite_Impl: Sized {
    fn GetAdjacentFragment(&mut self, direction: NavigateDirection) -> ::windows::core::Result<IRawElementProviderFragment>;
    fn GetRuntimeIdPrefix(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderWindowlessSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderWindowlessSite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderWindowlessSite_Vtbl {
        unsafe extern "system" fn GetAdjacentFragment<Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdjacentFragment(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeIdPrefix<Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRuntimeIdPrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAdjacentFragment: GetAdjacentFragment::<Impl, IMPL_OFFSET>,
            GetRuntimeIdPrefix: GetRuntimeIdPrefix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderWindowlessSite as ::windows::core::Interface>::IID
    }
}
pub trait IRichEditUiaInformation_Impl: Sized {
    fn GetBoundaryRectangle(&mut self, puiarect: *mut UiaRect) -> ::windows::core::Result<()>;
    fn IsVisible(&mut self) -> ::windows::core::Result<()>;
}
impl IRichEditUiaInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditUiaInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditUiaInformation_Vtbl {
        unsafe extern "system" fn GetBoundaryRectangle<Impl: IRichEditUiaInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiarect: *mut UiaRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBoundaryRectangle(::core::mem::transmute_copy(&puiarect)).into()
        }
        unsafe extern "system" fn IsVisible<Impl: IRichEditUiaInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsVisible().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBoundaryRectangle: GetBoundaryRectangle::<Impl, IMPL_OFFSET>,
            IsVisible: IsVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditUiaInformation as ::windows::core::Interface>::IID
    }
}
pub trait IRicheditWindowlessAccessibility_Impl: Sized {
    fn CreateProvider(&mut self, psite: ::core::option::Option<IRawElementProviderWindowlessSite>) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl IRicheditWindowlessAccessibility_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRicheditWindowlessAccessibility_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRicheditWindowlessAccessibility_Vtbl {
        unsafe extern "system" fn CreateProvider<Impl: IRicheditWindowlessAccessibility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psite: ::windows::core::RawPtr, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProvider(::core::mem::transmute(&psite)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateProvider: CreateProvider::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRicheditWindowlessAccessibility as ::windows::core::Interface>::IID
    }
}
pub trait IScrollItemProvider_Impl: Sized {
    fn ScrollIntoView(&mut self) -> ::windows::core::Result<()>;
}
impl IScrollItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollItemProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollItemProvider_Vtbl {
        unsafe extern "system" fn ScrollIntoView<Impl: IScrollItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollIntoView().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ScrollIntoView: ScrollIntoView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollItemProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IScrollProvider_Impl: Sized {
    fn Scroll(&mut self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::Result<()>;
    fn SetScrollPercent(&mut self, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::Result<()>;
    fn HorizontalScrollPercent(&mut self) -> ::windows::core::Result<f64>;
    fn VerticalScrollPercent(&mut self) -> ::windows::core::Result<f64>;
    fn HorizontalViewSize(&mut self) -> ::windows::core::Result<f64>;
    fn VerticalViewSize(&mut self) -> ::windows::core::Result<f64>;
    fn HorizontallyScrollable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn VerticallyScrollable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IScrollProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollProvider_Vtbl {
        unsafe extern "system" fn Scroll<Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Scroll(::core::mem::transmute_copy(&horizontalamount), ::core::mem::transmute_copy(&verticalamount)).into()
        }
        unsafe extern "system" fn SetScrollPercent<Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScrollPercent(::core::mem::transmute_copy(&horizontalpercent), ::core::mem::transmute_copy(&verticalpercent)).into()
        }
        unsafe extern "system" fn HorizontalScrollPercent<Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalScrollPercent<Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalViewSize<Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalViewSize<Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontallyScrollable<Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticallyScrollable<Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Scroll: Scroll::<Impl, IMPL_OFFSET>,
            SetScrollPercent: SetScrollPercent::<Impl, IMPL_OFFSET>,
            HorizontalScrollPercent: HorizontalScrollPercent::<Impl, IMPL_OFFSET>,
            VerticalScrollPercent: VerticalScrollPercent::<Impl, IMPL_OFFSET>,
            HorizontalViewSize: HorizontalViewSize::<Impl, IMPL_OFFSET>,
            VerticalViewSize: VerticalViewSize::<Impl, IMPL_OFFSET>,
            HorizontallyScrollable: HorizontallyScrollable::<Impl, IMPL_OFFSET>,
            VerticallyScrollable: VerticallyScrollable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISelectionItemProvider_Impl: Sized {
    fn Select(&mut self) -> ::windows::core::Result<()>;
    fn AddToSelection(&mut self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&mut self) -> ::windows::core::Result<()>;
    fn IsSelected(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SelectionContainer(&mut self) -> ::windows::core::Result<IRawElementProviderSimple>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISelectionItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionItemProvider_Vtbl {
        unsafe extern "system" fn Select<Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn AddToSelection<Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn IsSelected<Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionContainer<Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Select: Select::<Impl, IMPL_OFFSET>,
            AddToSelection: AddToSelection::<Impl, IMPL_OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Impl, IMPL_OFFSET>,
            IsSelected: IsSelected::<Impl, IMPL_OFFSET>,
            SelectionContainer: SelectionContainer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionItemProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISelectionProvider_Impl: Sized {
    fn GetSelection(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CanSelectMultiple(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsSelectionRequired(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISelectionProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionProvider_Vtbl {
        unsafe extern "system" fn GetSelection<Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanSelectMultiple<Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelectionRequired<Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            CanSelectMultiple: CanSelectMultiple::<Impl, IMPL_OFFSET>,
            IsSelectionRequired: IsSelectionRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISelectionProvider2_Impl: Sized + ISelectionProvider_Impl {
    fn FirstSelectedItem(&mut self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn LastSelectedItem(&mut self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn CurrentSelectedItem(&mut self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn ItemCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISelectionProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionProvider2_Vtbl {
        unsafe extern "system" fn FirstSelectedItem<Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastSelectedItem<Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSelectedItem<Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISelectionProvider_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FirstSelectedItem: FirstSelectedItem::<Impl, IMPL_OFFSET>,
            LastSelectedItem: LastSelectedItem::<Impl, IMPL_OFFSET>,
            CurrentSelectedItem: CurrentSelectedItem::<Impl, IMPL_OFFSET>,
            ItemCount: ItemCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpreadsheetItemProvider_Impl: Sized {
    fn Formula(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetAnnotationObjects(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetAnnotationTypes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpreadsheetItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpreadsheetItemProvider_Vtbl {
        unsafe extern "system" fn Formula<Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Formula() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationObjects<Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationTypes<Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Formula: Formula::<Impl, IMPL_OFFSET>,
            GetAnnotationObjects: GetAnnotationObjects::<Impl, IMPL_OFFSET>,
            GetAnnotationTypes: GetAnnotationTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpreadsheetItemProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpreadsheetProvider_Impl: Sized {
    fn GetItemByName(&mut self, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<IRawElementProviderSimple>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpreadsheetProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpreadsheetProvider_Vtbl {
        unsafe extern "system" fn GetItemByName<Impl: ISpreadsheetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetItemByName: GetItemByName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpreadsheetProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStylesProvider_Impl: Sized {
    fn StyleId(&mut self) -> ::windows::core::Result<i32>;
    fn StyleName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FillColor(&mut self) -> ::windows::core::Result<i32>;
    fn FillPatternStyle(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Shape(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FillPatternColor(&mut self) -> ::windows::core::Result<i32>;
    fn ExtendedProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IStylesProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStylesProvider_Vtbl {
        unsafe extern "system" fn StyleId<Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleName<Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillColor<Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternStyle<Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shape() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternColor<Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StyleId: StyleId::<Impl, IMPL_OFFSET>,
            StyleName: StyleName::<Impl, IMPL_OFFSET>,
            FillColor: FillColor::<Impl, IMPL_OFFSET>,
            FillPatternStyle: FillPatternStyle::<Impl, IMPL_OFFSET>,
            Shape: Shape::<Impl, IMPL_OFFSET>,
            FillPatternColor: FillPatternColor::<Impl, IMPL_OFFSET>,
            ExtendedProperties: ExtendedProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylesProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISynchronizedInputProvider_Impl: Sized {
    fn StartListening(&mut self, inputtype: SynchronizedInputType) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
impl ISynchronizedInputProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronizedInputProvider_Vtbl {
        unsafe extern "system" fn StartListening<Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartListening(::core::mem::transmute_copy(&inputtype)).into()
        }
        unsafe extern "system" fn Cancel<Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartListening: StartListening::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizedInputProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITableItemProvider_Impl: Sized {
    fn GetRowHeaderItems(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetColumnHeaderItems(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITableItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITableItemProvider_Vtbl {
        unsafe extern "system" fn GetRowHeaderItems<Impl: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRowHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaderItems<Impl: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRowHeaderItems: GetRowHeaderItems::<Impl, IMPL_OFFSET>,
            GetColumnHeaderItems: GetColumnHeaderItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableItemProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITableProvider_Impl: Sized {
    fn GetRowHeaders(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetColumnHeaders(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn RowOrColumnMajor(&mut self) -> ::windows::core::Result<RowOrColumnMajor>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITableProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITableProvider_Vtbl {
        unsafe extern "system" fn GetRowHeaders<Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRowHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowOrColumnMajor<Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRowHeaders: GetRowHeaders::<Impl, IMPL_OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Impl, IMPL_OFFSET>,
            RowOrColumnMajor: RowOrColumnMajor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableProvider as ::windows::core::Interface>::IID
    }
}
pub trait ITextChildProvider_Impl: Sized {
    fn TextContainer(&mut self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn TextRange(&mut self) -> ::windows::core::Result<ITextRangeProvider>;
}
impl ITextChildProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextChildProvider_Vtbl {
        unsafe extern "system" fn TextContainer<Impl: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRange<Impl: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextRange() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TextContainer: TextContainer::<Impl, IMPL_OFFSET>,
            TextRange: TextRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextChildProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextEditProvider_Impl: Sized + ITextProvider_Impl {
    fn GetActiveComposition(&mut self) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetConversionTarget(&mut self) -> ::windows::core::Result<ITextRangeProvider>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITextEditProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextEditProvider_Vtbl {
        unsafe extern "system" fn GetActiveComposition<Impl: ITextEditProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveComposition() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionTarget<Impl: ITextEditProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITextProvider_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Impl, IMPL_OFFSET>,
            GetConversionTarget: GetConversionTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextEditProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextProvider_Impl: Sized {
    fn GetSelection(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetVisibleRanges(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn RangeFromChild(&mut self, childelement: ::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn RangeFromPoint(&mut self, point: UiaPoint) -> ::windows::core::Result<ITextRangeProvider>;
    fn DocumentRange(&mut self) -> ::windows::core::Result<ITextRangeProvider>;
    fn SupportedTextSelection(&mut self) -> ::windows::core::Result<SupportedTextSelection>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITextProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextProvider_Vtbl {
        unsafe extern "system" fn GetSelection<Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisibleRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromChild<Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childelement: ::windows::core::RawPtr, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromChild(::core::mem::transmute(&childelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint<Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: UiaPoint, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromPoint(::core::mem::transmute_copy(&point)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentRange<Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentRange() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut SupportedTextSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedTextSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Impl, IMPL_OFFSET>,
            RangeFromChild: RangeFromChild::<Impl, IMPL_OFFSET>,
            RangeFromPoint: RangeFromPoint::<Impl, IMPL_OFFSET>,
            DocumentRange: DocumentRange::<Impl, IMPL_OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITextProvider2_Impl: Sized + ITextProvider_Impl {
    fn RangeFromAnnotation(&mut self, annotationelement: ::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetCaretRange(&mut self, isactive: *mut super::super::Foundation::BOOL, pretval: *mut ::core::option::Option<ITextRangeProvider>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITextProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextProvider2_Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Impl: ITextProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationelement: ::windows::core::RawPtr, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromAnnotation(::core::mem::transmute(&annotationelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretRange<Impl: ITextProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaretRange(::core::mem::transmute_copy(&isactive), ::core::mem::transmute_copy(&pretval)).into()
        }
        Self {
            base: ITextProvider_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Impl, IMPL_OFFSET>,
            GetCaretRange: GetCaretRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRangeProvider_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<ITextRangeProvider>;
    fn Compare(&mut self, range: ::core::option::Option<ITextRangeProvider>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CompareEndpoints(&mut self, endpoint: TextPatternRangeEndpoint, targetrange: ::core::option::Option<ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<i32>;
    fn ExpandToEnclosingUnit(&mut self, unit: TextUnit) -> ::windows::core::Result<()>;
    fn FindAttribute(&mut self, attributeid: i32, val: super::super::System::Com::VARIANT, backward: super::super::Foundation::BOOL) -> ::windows::core::Result<ITextRangeProvider>;
    fn FindText(&mut self, text: super::super::Foundation::BSTR, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetAttributeValue(&mut self, attributeid: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetBoundingRectangles(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEnclosingElement(&mut self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn GetText(&mut self, maxlength: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Move(&mut self, unit: TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByUnit(&mut self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByRange(&mut self, endpoint: TextPatternRangeEndpoint, targetrange: ::core::option::Option<ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<()>;
    fn Select(&mut self) -> ::windows::core::Result<()>;
    fn AddToSelection(&mut self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&mut self) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&mut self, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetChildren(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRangeProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRangeProvider_Vtbl {
        unsafe extern "system" fn Clone<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(::core::mem::transmute(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEndpoints<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareEndpoints(::core::mem::transmute_copy(&endpoint), ::core::mem::transmute(&targetrange), ::core::mem::transmute_copy(&targetendpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExpandToEnclosingUnit(::core::mem::transmute_copy(&unit)).into()
        }
        unsafe extern "system" fn FindAttribute<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, val: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, backward: super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAttribute(::core::mem::transmute_copy(&attributeid), ::core::mem::transmute_copy(&val), ::core::mem::transmute_copy(&backward)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindText(::core::mem::transmute_copy(&text), ::core::mem::transmute_copy(&backward), ::core::mem::transmute_copy(&ignorecase)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeValue(::core::mem::transmute_copy(&attributeid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundingRectangles() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnclosingElement<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnclosingElement() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&maxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndpointByUnit(::core::mem::transmute_copy(&endpoint), ::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveEndpointByRange(::core::mem::transmute_copy(&endpoint), ::core::mem::transmute(&targetrange), ::core::mem::transmute_copy(&targetendpoint)).into()
        }
        unsafe extern "system" fn Select<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn AddToSelection<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn ScrollIntoView<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollIntoView(::core::mem::transmute_copy(&aligntotop)).into()
        }
        unsafe extern "system" fn GetChildren<Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Compare: Compare::<Impl, IMPL_OFFSET>,
            CompareEndpoints: CompareEndpoints::<Impl, IMPL_OFFSET>,
            ExpandToEnclosingUnit: ExpandToEnclosingUnit::<Impl, IMPL_OFFSET>,
            FindAttribute: FindAttribute::<Impl, IMPL_OFFSET>,
            FindText: FindText::<Impl, IMPL_OFFSET>,
            GetAttributeValue: GetAttributeValue::<Impl, IMPL_OFFSET>,
            GetBoundingRectangles: GetBoundingRectangles::<Impl, IMPL_OFFSET>,
            GetEnclosingElement: GetEnclosingElement::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            MoveEndpointByUnit: MoveEndpointByUnit::<Impl, IMPL_OFFSET>,
            MoveEndpointByRange: MoveEndpointByRange::<Impl, IMPL_OFFSET>,
            Select: Select::<Impl, IMPL_OFFSET>,
            AddToSelection: AddToSelection::<Impl, IMPL_OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Impl, IMPL_OFFSET>,
            ScrollIntoView: ScrollIntoView::<Impl, IMPL_OFFSET>,
            GetChildren: GetChildren::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRangeProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRangeProvider2_Impl: Sized + ITextRangeProvider_Impl {
    fn ShowContextMenu(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRangeProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRangeProvider2_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: ITextRangeProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        Self { base: ITextRangeProvider_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ShowContextMenu: ShowContextMenu::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRangeProvider2 as ::windows::core::Interface>::IID
    }
}
pub trait IToggleProvider_Impl: Sized {
    fn Toggle(&mut self) -> ::windows::core::Result<()>;
    fn ToggleState(&mut self) -> ::windows::core::Result<ToggleState>;
}
impl IToggleProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleProvider_Vtbl {
        unsafe extern "system" fn Toggle<Impl: IToggleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Toggle().into()
        }
        unsafe extern "system" fn ToggleState<Impl: IToggleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Toggle: Toggle::<Impl, IMPL_OFFSET>,
            ToggleState: ToggleState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransformProvider_Impl: Sized {
    fn Move(&mut self, x: f64, y: f64) -> ::windows::core::Result<()>;
    fn Resize(&mut self, width: f64, height: f64) -> ::windows::core::Result<()>;
    fn Rotate(&mut self, degrees: f64) -> ::windows::core::Result<()>;
    fn CanMove(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CanResize(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CanRotate(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITransformProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformProvider_Vtbl {
        unsafe extern "system" fn Move<Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Resize<Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Rotate<Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Rotate(::core::mem::transmute_copy(&degrees)).into()
        }
        unsafe extern "system" fn CanMove<Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMove() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanResize<Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanResize() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRotate<Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Move: Move::<Impl, IMPL_OFFSET>,
            Resize: Resize::<Impl, IMPL_OFFSET>,
            Rotate: Rotate::<Impl, IMPL_OFFSET>,
            CanMove: CanMove::<Impl, IMPL_OFFSET>,
            CanResize: CanResize::<Impl, IMPL_OFFSET>,
            CanRotate: CanRotate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransformProvider2_Impl: Sized + ITransformProvider_Impl {
    fn Zoom(&mut self, zoom: f64) -> ::windows::core::Result<()>;
    fn CanZoom(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ZoomLevel(&mut self) -> ::windows::core::Result<f64>;
    fn ZoomMinimum(&mut self) -> ::windows::core::Result<f64>;
    fn ZoomMaximum(&mut self) -> ::windows::core::Result<f64>;
    fn ZoomByUnit(&mut self, zoomunit: ZoomUnit) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITransformProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformProvider2_Vtbl {
        unsafe extern "system" fn Zoom<Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoom: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Zoom(::core::mem::transmute_copy(&zoom)).into()
        }
        unsafe extern "system" fn CanZoom<Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevel<Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomMinimum<Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomMaximum<Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomByUnit<Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ZoomByUnit(::core::mem::transmute_copy(&zoomunit)).into()
        }
        Self {
            base: ITransformProvider_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Zoom: Zoom::<Impl, IMPL_OFFSET>,
            CanZoom: CanZoom::<Impl, IMPL_OFFSET>,
            ZoomLevel: ZoomLevel::<Impl, IMPL_OFFSET>,
            ZoomMinimum: ZoomMinimum::<Impl, IMPL_OFFSET>,
            ZoomMaximum: ZoomMaximum::<Impl, IMPL_OFFSET>,
            ZoomByUnit: ZoomByUnit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation_Impl: Sized {
    fn CompareElements(&mut self, el1: ::core::option::Option<IUIAutomationElement>, el2: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CompareRuntimeIds(&mut self, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetRootElement(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn ElementFromHandle(&mut self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<IUIAutomationElement>;
    fn ElementFromPoint(&mut self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetFocusedElement(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetRootElementBuildCache(&mut self, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn ElementFromHandleBuildCache(&mut self, hwnd: super::super::Foundation::HWND, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn ElementFromPointBuildCache(&mut self, pt: super::super::Foundation::POINT, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetFocusedElementBuildCache(&mut self, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn CreateTreeWalker(&mut self, pcondition: ::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationTreeWalker>;
    fn ControlViewWalker(&mut self) -> ::windows::core::Result<IUIAutomationTreeWalker>;
    fn ContentViewWalker(&mut self) -> ::windows::core::Result<IUIAutomationTreeWalker>;
    fn RawViewWalker(&mut self) -> ::windows::core::Result<IUIAutomationTreeWalker>;
    fn RawViewCondition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn ControlViewCondition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn ContentViewCondition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateCacheRequest(&mut self) -> ::windows::core::Result<IUIAutomationCacheRequest>;
    fn CreateTrueCondition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateFalseCondition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreatePropertyCondition(&mut self, propertyid: i32, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreatePropertyConditionEx(&mut self, propertyid: i32, value: super::super::System::Com::VARIANT, flags: PropertyConditionFlags) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateAndCondition(&mut self, condition1: ::core::option::Option<IUIAutomationCondition>, condition2: ::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromArray(&mut self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromNativeArray(&mut self, conditions: *const ::core::option::Option<IUIAutomationCondition>, conditioncount: i32) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateOrCondition(&mut self, condition1: ::core::option::Option<IUIAutomationCondition>, condition2: ::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromArray(&mut self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromNativeArray(&mut self, conditions: *const ::core::option::Option<IUIAutomationCondition>, conditioncount: i32) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateNotCondition(&mut self, condition: ::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationCondition>;
    fn AddAutomationEventHandler(&mut self, eventid: i32, element: ::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveAutomationEventHandler(&mut self, eventid: i32, element: ::core::option::Option<IUIAutomationElement>, handler: ::core::option::Option<IUIAutomationEventHandler>) -> ::windows::core::Result<()>;
    fn AddPropertyChangedEventHandlerNativeArray(&mut self, element: ::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const i32, propertycount: i32) -> ::windows::core::Result<()>;
    fn AddPropertyChangedEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RemovePropertyChangedEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, handler: ::core::option::Option<IUIAutomationPropertyChangedEventHandler>) -> ::windows::core::Result<()>;
    fn AddStructureChangedEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationStructureChangedEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveStructureChangedEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, handler: ::core::option::Option<IUIAutomationStructureChangedEventHandler>) -> ::windows::core::Result<()>;
    fn AddFocusChangedEventHandler(&mut self, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationFocusChangedEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveFocusChangedEventHandler(&mut self, handler: ::core::option::Option<IUIAutomationFocusChangedEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveAllEventHandlers(&mut self) -> ::windows::core::Result<()>;
    fn IntNativeArrayToSafeArray(&mut self, array: *const i32, arraycount: i32) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn IntSafeArrayToNativeArray(&mut self, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::Result<()>;
    fn RectToVariant(&mut self, rc: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn VariantToRect(&mut self, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SafeArrayToRectNativeArray(&mut self, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::Result<()>;
    fn CreateProxyFactoryEntry(&mut self, factory: ::core::option::Option<IUIAutomationProxyFactory>) -> ::windows::core::Result<IUIAutomationProxyFactoryEntry>;
    fn ProxyFactoryMapping(&mut self) -> ::windows::core::Result<IUIAutomationProxyFactoryMapping>;
    fn GetPropertyProgrammaticName(&mut self, property: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPatternProgrammaticName(&mut self, pattern: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PollForPotentialSupportedPatterns(&mut self, pelement: ::core::option::Option<IUIAutomationElement>, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn PollForPotentialSupportedProperties(&mut self, pelement: ::core::option::Option<IUIAutomationElement>, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn CheckNotSupported(&mut self, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ReservedNotSupportedValue(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ReservedMixedAttributeValue(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ElementFromIAccessible(&mut self, accessible: ::core::option::Option<IAccessible>, childid: i32) -> ::windows::core::Result<IUIAutomationElement>;
    fn ElementFromIAccessibleBuildCache(&mut self, accessible: ::core::option::Option<IAccessible>, childid: i32, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation_Vtbl {
        unsafe extern "system" fn CompareElements<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, el1: ::windows::core::RawPtr, el2: ::windows::core::RawPtr, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareElements(::core::mem::transmute(&el1), ::core::mem::transmute(&el2)) {
                ::core::result::Result::Ok(ok__) => {
                    *aresame = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareRuntimeIds<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareRuntimeIds(::core::mem::transmute_copy(&runtimeid1), ::core::mem::transmute_copy(&runtimeid2)) {
                ::core::result::Result::Ok(ok__) => {
                    *aresame = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootElement<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRootElement() {
                ::core::result::Result::Ok(ok__) => {
                    *root = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromHandle<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromHandle(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromPoint<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromPoint(::core::mem::transmute_copy(&pt)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElement<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootElementBuildCache<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRootElementBuildCache(::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *root = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromHandleBuildCache<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromHandleBuildCache(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromPointBuildCache<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromPointBuildCache(::core::mem::transmute_copy(&pt), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElementBuildCache<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocusedElementBuildCache(::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTreeWalker<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcondition: ::windows::core::RawPtr, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTreeWalker(::core::mem::transmute(&pcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *walker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlViewWalker<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlViewWalker() {
                ::core::result::Result::Ok(ok__) => {
                    *walker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentViewWalker<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentViewWalker() {
                ::core::result::Result::Ok(ok__) => {
                    *walker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawViewWalker<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawViewWalker() {
                ::core::result::Result::Ok(ok__) => {
                    *walker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawViewCondition<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawViewCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *condition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlViewCondition<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlViewCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *condition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentViewCondition<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentViewCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *condition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCacheRequest<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCacheRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *cacherequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrueCondition<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTrueCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFalseCondition<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFalseCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyCondition<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyCondition(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyConditionEx<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, flags: PropertyConditionFlags, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyConditionEx(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndCondition<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition1: ::windows::core::RawPtr, condition2: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAndCondition(::core::mem::transmute(&condition1), ::core::mem::transmute(&condition2)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndConditionFromArray<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAndConditionFromArray(::core::mem::transmute_copy(&conditions)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndConditionFromNativeArray<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const ::windows::core::RawPtr, conditioncount: i32, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAndConditionFromNativeArray(::core::mem::transmute_copy(&conditions), ::core::mem::transmute_copy(&conditioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrCondition<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition1: ::windows::core::RawPtr, condition2: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOrCondition(::core::mem::transmute(&condition1), ::core::mem::transmute(&condition2)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrConditionFromArray<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOrConditionFromArray(::core::mem::transmute_copy(&conditions)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrConditionFromNativeArray<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const ::windows::core::RawPtr, conditioncount: i32, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOrConditionFromNativeArray(::core::mem::transmute_copy(&conditions), ::core::mem::transmute_copy(&conditioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNotCondition<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNotCondition(::core::mem::transmute(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAutomationEventHandler<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAutomationEventHandler(::core::mem::transmute_copy(&eventid), ::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveAutomationEventHandler<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAutomationEventHandler(::core::mem::transmute_copy(&eventid), ::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandlerNativeArray<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const i32, propertycount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyChangedEventHandlerNativeArray(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler), ::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler), ::core::mem::transmute_copy(&propertyarray)).into()
        }
        unsafe extern "system" fn RemovePropertyChangedEventHandler<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePropertyChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStructureChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveStructureChangedEventHandler<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStructureChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddFocusChangedEventHandler<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFocusChangedEventHandler(::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveFocusChangedEventHandler<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFocusChangedEventHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveAllEventHandlers<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllEventHandlers().into()
        }
        unsafe extern "system" fn IntNativeArrayToSafeArray<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, array: *const i32, arraycount: i32, safearray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IntNativeArrayToSafeArray(::core::mem::transmute_copy(&array), ::core::mem::transmute_copy(&arraycount)) {
                ::core::result::Result::Ok(ok__) => {
                    *safearray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntSafeArrayToNativeArray<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IntSafeArrayToNativeArray(::core::mem::transmute_copy(&intarray), ::core::mem::transmute_copy(&array), ::core::mem::transmute_copy(&arraycount)).into()
        }
        unsafe extern "system" fn RectToVariant<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, var: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RectToVariant(::core::mem::transmute_copy(&rc)) {
                ::core::result::Result::Ok(ok__) => {
                    *var = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VariantToRect<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, rc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VariantToRect(::core::mem::transmute_copy(&var)) {
                ::core::result::Result::Ok(ok__) => {
                    *rc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SafeArrayToRectNativeArray<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SafeArrayToRectNativeArray(::core::mem::transmute_copy(&rects), ::core::mem::transmute_copy(&rectarray), ::core::mem::transmute_copy(&rectarraycount)).into()
        }
        unsafe extern "system" fn CreateProxyFactoryEntry<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, factoryentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProxyFactoryEntry(::core::mem::transmute(&factory)) {
                ::core::result::Result::Ok(ok__) => {
                    *factoryentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyFactoryMapping<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factorymapping: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyFactoryMapping() {
                ::core::result::Result::Ok(ok__) => {
                    *factorymapping = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyProgrammaticName<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyProgrammaticName(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatternProgrammaticName<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatternProgrammaticName(::core::mem::transmute_copy(&pattern)) {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PollForPotentialSupportedPatterns<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PollForPotentialSupportedPatterns(::core::mem::transmute(&pelement), ::core::mem::transmute_copy(&patternids), ::core::mem::transmute_copy(&patternnames)).into()
        }
        unsafe extern "system" fn PollForPotentialSupportedProperties<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PollForPotentialSupportedProperties(::core::mem::transmute(&pelement), ::core::mem::transmute_copy(&propertyids), ::core::mem::transmute_copy(&propertynames)).into()
        }
        unsafe extern "system" fn CheckNotSupported<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, isnotsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckNotSupported(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *isnotsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReservedNotSupportedValue<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notsupportedvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReservedNotSupportedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *notsupportedvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReservedMixedAttributeValue<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mixedattributevalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReservedMixedAttributeValue() {
                ::core::result::Result::Ok(ok__) => {
                    *mixedattributevalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromIAccessible<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessible: ::windows::core::RawPtr, childid: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromIAccessible(::core::mem::transmute(&accessible), ::core::mem::transmute_copy(&childid)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromIAccessibleBuildCache<Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessible: ::windows::core::RawPtr, childid: i32, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromIAccessibleBuildCache(::core::mem::transmute(&accessible), ::core::mem::transmute_copy(&childid), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CompareElements: CompareElements::<Impl, IMPL_OFFSET>,
            CompareRuntimeIds: CompareRuntimeIds::<Impl, IMPL_OFFSET>,
            GetRootElement: GetRootElement::<Impl, IMPL_OFFSET>,
            ElementFromHandle: ElementFromHandle::<Impl, IMPL_OFFSET>,
            ElementFromPoint: ElementFromPoint::<Impl, IMPL_OFFSET>,
            GetFocusedElement: GetFocusedElement::<Impl, IMPL_OFFSET>,
            GetRootElementBuildCache: GetRootElementBuildCache::<Impl, IMPL_OFFSET>,
            ElementFromHandleBuildCache: ElementFromHandleBuildCache::<Impl, IMPL_OFFSET>,
            ElementFromPointBuildCache: ElementFromPointBuildCache::<Impl, IMPL_OFFSET>,
            GetFocusedElementBuildCache: GetFocusedElementBuildCache::<Impl, IMPL_OFFSET>,
            CreateTreeWalker: CreateTreeWalker::<Impl, IMPL_OFFSET>,
            ControlViewWalker: ControlViewWalker::<Impl, IMPL_OFFSET>,
            ContentViewWalker: ContentViewWalker::<Impl, IMPL_OFFSET>,
            RawViewWalker: RawViewWalker::<Impl, IMPL_OFFSET>,
            RawViewCondition: RawViewCondition::<Impl, IMPL_OFFSET>,
            ControlViewCondition: ControlViewCondition::<Impl, IMPL_OFFSET>,
            ContentViewCondition: ContentViewCondition::<Impl, IMPL_OFFSET>,
            CreateCacheRequest: CreateCacheRequest::<Impl, IMPL_OFFSET>,
            CreateTrueCondition: CreateTrueCondition::<Impl, IMPL_OFFSET>,
            CreateFalseCondition: CreateFalseCondition::<Impl, IMPL_OFFSET>,
            CreatePropertyCondition: CreatePropertyCondition::<Impl, IMPL_OFFSET>,
            CreatePropertyConditionEx: CreatePropertyConditionEx::<Impl, IMPL_OFFSET>,
            CreateAndCondition: CreateAndCondition::<Impl, IMPL_OFFSET>,
            CreateAndConditionFromArray: CreateAndConditionFromArray::<Impl, IMPL_OFFSET>,
            CreateAndConditionFromNativeArray: CreateAndConditionFromNativeArray::<Impl, IMPL_OFFSET>,
            CreateOrCondition: CreateOrCondition::<Impl, IMPL_OFFSET>,
            CreateOrConditionFromArray: CreateOrConditionFromArray::<Impl, IMPL_OFFSET>,
            CreateOrConditionFromNativeArray: CreateOrConditionFromNativeArray::<Impl, IMPL_OFFSET>,
            CreateNotCondition: CreateNotCondition::<Impl, IMPL_OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Impl, IMPL_OFFSET>,
            RemoveAutomationEventHandler: RemoveAutomationEventHandler::<Impl, IMPL_OFFSET>,
            AddPropertyChangedEventHandlerNativeArray: AddPropertyChangedEventHandlerNativeArray::<Impl, IMPL_OFFSET>,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Impl, IMPL_OFFSET>,
            RemovePropertyChangedEventHandler: RemovePropertyChangedEventHandler::<Impl, IMPL_OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Impl, IMPL_OFFSET>,
            RemoveStructureChangedEventHandler: RemoveStructureChangedEventHandler::<Impl, IMPL_OFFSET>,
            AddFocusChangedEventHandler: AddFocusChangedEventHandler::<Impl, IMPL_OFFSET>,
            RemoveFocusChangedEventHandler: RemoveFocusChangedEventHandler::<Impl, IMPL_OFFSET>,
            RemoveAllEventHandlers: RemoveAllEventHandlers::<Impl, IMPL_OFFSET>,
            IntNativeArrayToSafeArray: IntNativeArrayToSafeArray::<Impl, IMPL_OFFSET>,
            IntSafeArrayToNativeArray: IntSafeArrayToNativeArray::<Impl, IMPL_OFFSET>,
            RectToVariant: RectToVariant::<Impl, IMPL_OFFSET>,
            VariantToRect: VariantToRect::<Impl, IMPL_OFFSET>,
            SafeArrayToRectNativeArray: SafeArrayToRectNativeArray::<Impl, IMPL_OFFSET>,
            CreateProxyFactoryEntry: CreateProxyFactoryEntry::<Impl, IMPL_OFFSET>,
            ProxyFactoryMapping: ProxyFactoryMapping::<Impl, IMPL_OFFSET>,
            GetPropertyProgrammaticName: GetPropertyProgrammaticName::<Impl, IMPL_OFFSET>,
            GetPatternProgrammaticName: GetPatternProgrammaticName::<Impl, IMPL_OFFSET>,
            PollForPotentialSupportedPatterns: PollForPotentialSupportedPatterns::<Impl, IMPL_OFFSET>,
            PollForPotentialSupportedProperties: PollForPotentialSupportedProperties::<Impl, IMPL_OFFSET>,
            CheckNotSupported: CheckNotSupported::<Impl, IMPL_OFFSET>,
            ReservedNotSupportedValue: ReservedNotSupportedValue::<Impl, IMPL_OFFSET>,
            ReservedMixedAttributeValue: ReservedMixedAttributeValue::<Impl, IMPL_OFFSET>,
            ElementFromIAccessible: ElementFromIAccessible::<Impl, IMPL_OFFSET>,
            ElementFromIAccessibleBuildCache: ElementFromIAccessibleBuildCache::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation2_Impl: Sized + IUIAutomation_Impl {
    fn AutoSetFocus(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAutoSetFocus(&mut self, autosetfocus: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ConnectionTimeout(&mut self) -> ::windows::core::Result<u32>;
    fn SetConnectionTimeout(&mut self, timeout: u32) -> ::windows::core::Result<()>;
    fn TransactionTimeout(&mut self) -> ::windows::core::Result<u32>;
    fn SetTransactionTimeout(&mut self, timeout: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation2_Vtbl {
        unsafe extern "system" fn AutoSetFocus<Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autosetfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoSetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *autosetfocus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoSetFocus<Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autosetfocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoSetFocus(::core::mem::transmute_copy(&autosetfocus)).into()
        }
        unsafe extern "system" fn ConnectionTimeout<Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *timeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionTimeout<Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectionTimeout(::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn TransactionTimeout<Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *timeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransactionTimeout<Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransactionTimeout(::core::mem::transmute_copy(&timeout)).into()
        }
        Self {
            base: IUIAutomation_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AutoSetFocus: AutoSetFocus::<Impl, IMPL_OFFSET>,
            SetAutoSetFocus: SetAutoSetFocus::<Impl, IMPL_OFFSET>,
            ConnectionTimeout: ConnectionTimeout::<Impl, IMPL_OFFSET>,
            SetConnectionTimeout: SetConnectionTimeout::<Impl, IMPL_OFFSET>,
            TransactionTimeout: TransactionTimeout::<Impl, IMPL_OFFSET>,
            SetTransactionTimeout: SetTransactionTimeout::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation3_Impl: Sized + IUIAutomation_Impl + IUIAutomation2_Impl {
    fn AddTextEditTextChangedEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationTextEditTextChangedEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveTextEditTextChangedEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, handler: ::core::option::Option<IUIAutomationTextEditTextChangedEventHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation3_Vtbl {
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Impl: IUIAutomation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTextEditTextChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&texteditchangetype), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveTextEditTextChangedEventHandler<Impl: IUIAutomation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTextEditTextChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        Self {
            base: IUIAutomation2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Impl, IMPL_OFFSET>,
            RemoveTextEditTextChangedEventHandler: RemoveTextEditTextChangedEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation4_Impl: Sized + IUIAutomation_Impl + IUIAutomation2_Impl + IUIAutomation3_Impl {
    fn AddChangesEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationChangesEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveChangesEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, handler: ::core::option::Option<IUIAutomationChangesEventHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation4_Vtbl {
        unsafe extern "system" fn AddChangesEventHandler<Impl: IUIAutomation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChangesEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&changetypes), ::core::mem::transmute_copy(&changescount), ::core::mem::transmute(&pcacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveChangesEventHandler<Impl: IUIAutomation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChangesEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        Self {
            base: IUIAutomation3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddChangesEventHandler: AddChangesEventHandler::<Impl, IMPL_OFFSET>,
            RemoveChangesEventHandler: RemoveChangesEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation5_Impl: Sized + IUIAutomation_Impl + IUIAutomation2_Impl + IUIAutomation3_Impl + IUIAutomation4_Impl {
    fn AddNotificationEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationNotificationEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveNotificationEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, handler: ::core::option::Option<IUIAutomationNotificationEventHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation5_Vtbl {
        unsafe extern "system" fn AddNotificationEventHandler<Impl: IUIAutomation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddNotificationEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveNotificationEventHandler<Impl: IUIAutomation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNotificationEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        Self {
            base: IUIAutomation4_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddNotificationEventHandler: AddNotificationEventHandler::<Impl, IMPL_OFFSET>,
            RemoveNotificationEventHandler: RemoveNotificationEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation6_Impl: Sized + IUIAutomation_Impl + IUIAutomation2_Impl + IUIAutomation3_Impl + IUIAutomation4_Impl + IUIAutomation5_Impl {
    fn CreateEventHandlerGroup(&mut self) -> ::windows::core::Result<IUIAutomationEventHandlerGroup>;
    fn AddEventHandlerGroup(&mut self, element: ::core::option::Option<IUIAutomationElement>, handlergroup: ::core::option::Option<IUIAutomationEventHandlerGroup>) -> ::windows::core::Result<()>;
    fn RemoveEventHandlerGroup(&mut self, element: ::core::option::Option<IUIAutomationElement>, handlergroup: ::core::option::Option<IUIAutomationEventHandlerGroup>) -> ::windows::core::Result<()>;
    fn ConnectionRecoveryBehavior(&mut self) -> ::windows::core::Result<ConnectionRecoveryBehaviorOptions>;
    fn SetConnectionRecoveryBehavior(&mut self, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows::core::Result<()>;
    fn CoalesceEvents(&mut self) -> ::windows::core::Result<CoalesceEventsOptions>;
    fn SetCoalesceEvents(&mut self, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows::core::Result<()>;
    fn AddActiveTextPositionChangedEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveActiveTextPositionChangedEventHandler(&mut self, element: ::core::option::Option<IUIAutomationElement>, handler: ::core::option::Option<IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation6_Vtbl {
        unsafe extern "system" fn CreateEventHandlerGroup<Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handlergroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEventHandlerGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *handlergroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEventHandlerGroup<Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handlergroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEventHandlerGroup(::core::mem::transmute(&element), ::core::mem::transmute(&handlergroup)).into()
        }
        unsafe extern "system" fn RemoveEventHandlerGroup<Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handlergroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEventHandlerGroup(::core::mem::transmute(&element), ::core::mem::transmute(&handlergroup)).into()
        }
        unsafe extern "system" fn ConnectionRecoveryBehavior<Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: *mut ConnectionRecoveryBehaviorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionRecoveryBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *connectionrecoverybehavioroptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionRecoveryBehavior<Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectionRecoveryBehavior(::core::mem::transmute_copy(&connectionrecoverybehavioroptions)).into()
        }
        unsafe extern "system" fn CoalesceEvents<Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: *mut CoalesceEventsOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoalesceEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *coalesceeventsoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoalesceEvents<Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoalesceEvents(::core::mem::transmute_copy(&coalesceeventsoptions)).into()
        }
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddActiveTextPositionChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveActiveTextPositionChangedEventHandler<Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActiveTextPositionChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        Self {
            base: IUIAutomation5_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateEventHandlerGroup: CreateEventHandlerGroup::<Impl, IMPL_OFFSET>,
            AddEventHandlerGroup: AddEventHandlerGroup::<Impl, IMPL_OFFSET>,
            RemoveEventHandlerGroup: RemoveEventHandlerGroup::<Impl, IMPL_OFFSET>,
            ConnectionRecoveryBehavior: ConnectionRecoveryBehavior::<Impl, IMPL_OFFSET>,
            SetConnectionRecoveryBehavior: SetConnectionRecoveryBehavior::<Impl, IMPL_OFFSET>,
            CoalesceEvents: CoalesceEvents::<Impl, IMPL_OFFSET>,
            SetCoalesceEvents: SetCoalesceEvents::<Impl, IMPL_OFFSET>,
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Impl, IMPL_OFFSET>,
            RemoveActiveTextPositionChangedEventHandler: RemoveActiveTextPositionChangedEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation6 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationActiveTextPositionChangedEventHandler_Impl: Sized {
    fn HandleActiveTextPositionChangedEvent(&mut self, sender: ::core::option::Option<IUIAutomationElement>, range: ::core::option::Option<IUIAutomationTextRange>) -> ::windows::core::Result<()>;
}
impl IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationActiveTextPositionChangedEventHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleActiveTextPositionChangedEvent<Impl: IUIAutomationActiveTextPositionChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, range: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleActiveTextPositionChangedEvent(::core::mem::transmute(&sender), ::core::mem::transmute(&range)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            HandleActiveTextPositionChangedEvent: HandleActiveTextPositionChangedEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationActiveTextPositionChangedEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationAndCondition_Impl: Sized + IUIAutomationCondition_Impl {
    fn ChildCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetChildrenAsNativeArray(&mut self, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows::core::Result<()>;
    fn GetChildren(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationAndCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAndCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationAndCondition_Vtbl {
        unsafe extern "system" fn ChildCount<Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildCount() {
                ::core::result::Result::Ok(ok__) => {
                    *childcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::windows::core::RawPtr, childarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChildrenAsNativeArray(::core::mem::transmute_copy(&childarray), ::core::mem::transmute_copy(&childarraycount)).into()
        }
        unsafe extern "system" fn GetChildren<Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *childarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationCondition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ChildCount: ChildCount::<Impl, IMPL_OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Impl, IMPL_OFFSET>,
            GetChildren: GetChildren::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationAndCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationAnnotationPattern_Impl: Sized {
    fn CurrentAnnotationTypeId(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentAnnotationTypeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentAuthor(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentDateTime(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentTarget(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CachedAnnotationTypeId(&mut self) -> ::windows::core::Result<i32>;
    fn CachedAnnotationTypeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedAuthor(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedDateTime(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedTarget(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationAnnotationPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationAnnotationPattern_Vtbl {
        unsafe extern "system" fn CurrentAnnotationTypeId<Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationTypeName<Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAuthor<Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAuthor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDateTime<Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentTarget<Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypeId<Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypeName<Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAuthor<Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAuthor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDateTime<Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedTarget<Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CurrentAnnotationTypeId: CurrentAnnotationTypeId::<Impl, IMPL_OFFSET>,
            CurrentAnnotationTypeName: CurrentAnnotationTypeName::<Impl, IMPL_OFFSET>,
            CurrentAuthor: CurrentAuthor::<Impl, IMPL_OFFSET>,
            CurrentDateTime: CurrentDateTime::<Impl, IMPL_OFFSET>,
            CurrentTarget: CurrentTarget::<Impl, IMPL_OFFSET>,
            CachedAnnotationTypeId: CachedAnnotationTypeId::<Impl, IMPL_OFFSET>,
            CachedAnnotationTypeName: CachedAnnotationTypeName::<Impl, IMPL_OFFSET>,
            CachedAuthor: CachedAuthor::<Impl, IMPL_OFFSET>,
            CachedDateTime: CachedDateTime::<Impl, IMPL_OFFSET>,
            CachedTarget: CachedTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationAnnotationPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationBoolCondition_Impl: Sized + IUIAutomationCondition_Impl {
    fn BooleanValue(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationBoolCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationBoolCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationBoolCondition_Vtbl {
        unsafe extern "system" fn BooleanValue<Impl: IUIAutomationBoolCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boolval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BooleanValue() {
                ::core::result::Result::Ok(ok__) => {
                    *boolval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUIAutomationCondition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), BooleanValue: BooleanValue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationBoolCondition as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationCacheRequest_Impl: Sized {
    fn AddProperty(&mut self, propertyid: i32) -> ::windows::core::Result<()>;
    fn AddPattern(&mut self, patternid: i32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IUIAutomationCacheRequest>;
    fn TreeScope(&mut self) -> ::windows::core::Result<TreeScope>;
    fn SetTreeScope(&mut self, scope: TreeScope) -> ::windows::core::Result<()>;
    fn TreeFilter(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn SetTreeFilter(&mut self, filter: ::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<()>;
    fn AutomationElementMode(&mut self) -> ::windows::core::Result<AutomationElementMode>;
    fn SetAutomationElementMode(&mut self, mode: AutomationElementMode) -> ::windows::core::Result<()>;
}
impl IUIAutomationCacheRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationCacheRequest_Vtbl {
        unsafe extern "system" fn AddProperty<Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddProperty(::core::mem::transmute_copy(&propertyid)).into()
        }
        unsafe extern "system" fn AddPattern<Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPattern(::core::mem::transmute_copy(&patternid)).into()
        }
        unsafe extern "system" fn Clone<Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clonedrequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *clonedrequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TreeScope<Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut TreeScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TreeScope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTreeScope<Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTreeScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn TreeFilter<Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TreeFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *filter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTreeFilter<Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTreeFilter(::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn AutomationElementMode<Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut AutomationElementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomationElementMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomationElementMode<Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: AutomationElementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomationElementMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddProperty: AddProperty::<Impl, IMPL_OFFSET>,
            AddPattern: AddPattern::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            TreeScope: TreeScope::<Impl, IMPL_OFFSET>,
            SetTreeScope: SetTreeScope::<Impl, IMPL_OFFSET>,
            TreeFilter: TreeFilter::<Impl, IMPL_OFFSET>,
            SetTreeFilter: SetTreeFilter::<Impl, IMPL_OFFSET>,
            AutomationElementMode: AutomationElementMode::<Impl, IMPL_OFFSET>,
            SetAutomationElementMode: SetAutomationElementMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationCacheRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationChangesEventHandler_Impl: Sized {
    fn HandleChangesEvent(&mut self, sender: ::core::option::Option<IUIAutomationElement>, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationChangesEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationChangesEventHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationChangesEventHandler_Vtbl {
        unsafe extern "system" fn HandleChangesEvent<Impl: IUIAutomationChangesEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleChangesEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&uiachanges), ::core::mem::transmute_copy(&changescount)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandleChangesEvent: HandleChangesEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationChangesEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationCondition_Impl: Sized {}
impl IUIAutomationCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationCondition_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationCondition as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationCustomNavigationPattern_Impl: Sized {
    fn Navigate(&mut self, direction: NavigateDirection) -> ::windows::core::Result<IUIAutomationElement>;
}
impl IUIAutomationCustomNavigationPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCustomNavigationPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationCustomNavigationPattern_Vtbl {
        unsafe extern "system" fn Navigate<Impl: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigate(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Navigate: Navigate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationCustomNavigationPattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationDockPattern_Impl: Sized {
    fn SetDockPosition(&mut self, dockpos: DockPosition) -> ::windows::core::Result<()>;
    fn CurrentDockPosition(&mut self) -> ::windows::core::Result<DockPosition>;
    fn CachedDockPosition(&mut self) -> ::windows::core::Result<DockPosition>;
}
impl IUIAutomationDockPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDockPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationDockPattern_Vtbl {
        unsafe extern "system" fn SetDockPosition<Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockpos: DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDockPosition(::core::mem::transmute_copy(&dockpos)).into()
        }
        unsafe extern "system" fn CurrentDockPosition<Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDockPosition<Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDockPosition: SetDockPosition::<Impl, IMPL_OFFSET>,
            CurrentDockPosition: CurrentDockPosition::<Impl, IMPL_OFFSET>,
            CachedDockPosition: CachedDockPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationDockPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUIAutomationDragPattern_Impl: Sized {
    fn CurrentIsGrabbed(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsGrabbed(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentDropEffect(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedDropEffect(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentDropEffects(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedDropEffects(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetCurrentGrabbedItems(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetCachedGrabbedItems(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationDragPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationDragPattern_Vtbl {
        unsafe extern "system" fn CurrentIsGrabbed<Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsGrabbed<Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropEffect<Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropEffect<Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropEffects<Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropEffects<Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentGrabbedItems<Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentGrabbedItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedGrabbedItems<Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedGrabbedItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CurrentIsGrabbed: CurrentIsGrabbed::<Impl, IMPL_OFFSET>,
            CachedIsGrabbed: CachedIsGrabbed::<Impl, IMPL_OFFSET>,
            CurrentDropEffect: CurrentDropEffect::<Impl, IMPL_OFFSET>,
            CachedDropEffect: CachedDropEffect::<Impl, IMPL_OFFSET>,
            CurrentDropEffects: CurrentDropEffects::<Impl, IMPL_OFFSET>,
            CachedDropEffects: CachedDropEffects::<Impl, IMPL_OFFSET>,
            GetCurrentGrabbedItems: GetCurrentGrabbedItems::<Impl, IMPL_OFFSET>,
            GetCachedGrabbedItems: GetCachedGrabbedItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationDragPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUIAutomationDropTargetPattern_Impl: Sized {
    fn CurrentDropTargetEffect(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedDropTargetEffect(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentDropTargetEffects(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedDropTargetEffects(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationDropTargetPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDropTargetPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationDropTargetPattern_Vtbl {
        unsafe extern "system" fn CurrentDropTargetEffect<Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDropTargetEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropTargetEffect<Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDropTargetEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropTargetEffects<Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDropTargetEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropTargetEffects<Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDropTargetEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CurrentDropTargetEffect: CurrentDropTargetEffect::<Impl, IMPL_OFFSET>,
            CachedDropTargetEffect: CachedDropTargetEffect::<Impl, IMPL_OFFSET>,
            CurrentDropTargetEffects: CurrentDropTargetEffects::<Impl, IMPL_OFFSET>,
            CachedDropTargetEffects: CachedDropTargetEffects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationDropTargetPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement_Impl: Sized {
    fn SetFocus(&mut self) -> ::windows::core::Result<()>;
    fn GetRuntimeId(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn FindFirst(&mut self, scope: TreeScope, condition: ::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationElement>;
    fn FindAll(&mut self, scope: TreeScope, condition: ::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn FindFirstBuildCache(&mut self, scope: TreeScope, condition: ::core::option::Option<IUIAutomationCondition>, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn FindAllBuildCache(&mut self, scope: TreeScope, condition: ::core::option::Option<IUIAutomationCondition>, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn BuildUpdatedCache(&mut self, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetCurrentPropertyValue(&mut self, propertyid: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetCurrentPropertyValueEx(&mut self, propertyid: i32, ignoredefaultvalue: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetCachedPropertyValue(&mut self, propertyid: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetCachedPropertyValueEx(&mut self, propertyid: i32, ignoredefaultvalue: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetCurrentPatternAs(&mut self, patternid: i32, riid: *const ::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetCachedPatternAs(&mut self, patternid: i32, riid: *const ::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetCurrentPattern(&mut self, patternid: i32) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetCachedPattern(&mut self, patternid: i32) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetCachedParent(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetCachedChildren(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CurrentProcessId(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentControlType(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentLocalizedControlType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentAcceleratorKey(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentAccessKey(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentHasKeyboardFocus(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsKeyboardFocusable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentAutomationId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentClassName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentHelpText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentCulture(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentIsControlElement(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsContentElement(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsPassword(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentNativeWindowHandle(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn CurrentItemType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentIsOffscreen(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentOrientation(&mut self) -> ::windows::core::Result<OrientationType>;
    fn CurrentFrameworkId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentIsRequiredForForm(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentItemStatus(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentBoundingRectangle(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn CurrentLabeledBy(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CurrentAriaRole(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentAriaProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentIsDataValidForForm(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentControllerFor(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CurrentDescribedBy(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CurrentFlowsTo(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CurrentProviderDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedProcessId(&mut self) -> ::windows::core::Result<i32>;
    fn CachedControlType(&mut self) -> ::windows::core::Result<i32>;
    fn CachedLocalizedControlType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedAcceleratorKey(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedAccessKey(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedHasKeyboardFocus(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsKeyboardFocusable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedAutomationId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedClassName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedHelpText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedCulture(&mut self) -> ::windows::core::Result<i32>;
    fn CachedIsControlElement(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsContentElement(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsPassword(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedNativeWindowHandle(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn CachedItemType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedIsOffscreen(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedOrientation(&mut self) -> ::windows::core::Result<OrientationType>;
    fn CachedFrameworkId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedIsRequiredForForm(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedItemStatus(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedBoundingRectangle(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn CachedLabeledBy(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CachedAriaRole(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedAriaProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedIsDataValidForForm(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedControllerFor(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CachedDescribedBy(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CachedFlowsTo(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CachedProviderDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetClickablePoint(&mut self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement_Vtbl {
        unsafe extern "system" fn SetFocus<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocus().into()
        }
        unsafe extern "system" fn GetRuntimeId<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimeid: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    *runtimeid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirst<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirst(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAll<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAll(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstBuildCache<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstBuildCache(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllBuildCache<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllBuildCache(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildUpdatedCache<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, updatedelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildUpdatedCache(::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *updatedelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValue<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPropertyValue(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValueEx<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPropertyValueEx(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ignoredefaultvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPropertyValue<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedPropertyValue(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPropertyValueEx<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedPropertyValueEx(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ignoredefaultvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPatternAs<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, riid: *const ::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentPatternAs(::core::mem::transmute_copy(&patternid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&patternobject)).into()
        }
        unsafe extern "system" fn GetCachedPatternAs<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, riid: *const ::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCachedPatternAs(::core::mem::transmute_copy(&patternid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&patternobject)).into()
        }
        unsafe extern "system" fn GetCurrentPattern<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPattern(::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    *patternobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPattern<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedPattern(::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    *patternobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedParent<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedParent() {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedChildren<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProcessId<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentControlType<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLocalizedControlType<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLocalizedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentName<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAcceleratorKey<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAcceleratorKey() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAccessKey<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHasKeyboardFocus<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHasKeyboardFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsKeyboardFocusable<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsKeyboardFocusable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsEnabled<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAutomationId<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAutomationId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentClassName<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentClassName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHelpText<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCulture<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCulture() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsControlElement<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsControlElement() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsContentElement<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsContentElement() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsPassword<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsPassword() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentNativeWindowHandle<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentNativeWindowHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemType<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItemType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsOffscreen<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsOffscreen() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentOrientation<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFrameworkId<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFrameworkId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsRequiredForForm<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsRequiredForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemStatus<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItemStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentBoundingRectangle<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentBoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLabeledBy<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLabeledBy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAriaRole<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAriaRole() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAriaProperties<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAriaProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsDataValidForForm<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsDataValidForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentControllerFor<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentControllerFor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDescribedBy<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDescribedBy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFlowsTo<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFlowsTo() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProviderDescription<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProviderDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedProcessId<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedControlType<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLocalizedControlType<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLocalizedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedName<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAcceleratorKey<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAcceleratorKey() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAccessKey<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHasKeyboardFocus<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHasKeyboardFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsKeyboardFocusable<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsKeyboardFocusable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsEnabled<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAutomationId<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAutomationId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedClassName<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedClassName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHelpText<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCulture<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCulture() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsControlElement<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsControlElement() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsContentElement<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsContentElement() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsPassword<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsPassword() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedNativeWindowHandle<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedNativeWindowHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemType<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedItemType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsOffscreen<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsOffscreen() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedOrientation<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFrameworkId<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFrameworkId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsRequiredForForm<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsRequiredForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemStatus<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedItemStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedBoundingRectangle<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedBoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLabeledBy<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLabeledBy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAriaRole<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAriaRole() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAriaProperties<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAriaProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsDataValidForForm<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsDataValidForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedControllerFor<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedControllerFor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDescribedBy<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDescribedBy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFlowsTo<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFlowsTo() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedProviderDescription<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedProviderDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClickablePoint<Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClickablePoint(::core::mem::transmute_copy(&clickable), ::core::mem::transmute_copy(&gotclickable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetFocus: SetFocus::<Impl, IMPL_OFFSET>,
            GetRuntimeId: GetRuntimeId::<Impl, IMPL_OFFSET>,
            FindFirst: FindFirst::<Impl, IMPL_OFFSET>,
            FindAll: FindAll::<Impl, IMPL_OFFSET>,
            FindFirstBuildCache: FindFirstBuildCache::<Impl, IMPL_OFFSET>,
            FindAllBuildCache: FindAllBuildCache::<Impl, IMPL_OFFSET>,
            BuildUpdatedCache: BuildUpdatedCache::<Impl, IMPL_OFFSET>,
            GetCurrentPropertyValue: GetCurrentPropertyValue::<Impl, IMPL_OFFSET>,
            GetCurrentPropertyValueEx: GetCurrentPropertyValueEx::<Impl, IMPL_OFFSET>,
            GetCachedPropertyValue: GetCachedPropertyValue::<Impl, IMPL_OFFSET>,
            GetCachedPropertyValueEx: GetCachedPropertyValueEx::<Impl, IMPL_OFFSET>,
            GetCurrentPatternAs: GetCurrentPatternAs::<Impl, IMPL_OFFSET>,
            GetCachedPatternAs: GetCachedPatternAs::<Impl, IMPL_OFFSET>,
            GetCurrentPattern: GetCurrentPattern::<Impl, IMPL_OFFSET>,
            GetCachedPattern: GetCachedPattern::<Impl, IMPL_OFFSET>,
            GetCachedParent: GetCachedParent::<Impl, IMPL_OFFSET>,
            GetCachedChildren: GetCachedChildren::<Impl, IMPL_OFFSET>,
            CurrentProcessId: CurrentProcessId::<Impl, IMPL_OFFSET>,
            CurrentControlType: CurrentControlType::<Impl, IMPL_OFFSET>,
            CurrentLocalizedControlType: CurrentLocalizedControlType::<Impl, IMPL_OFFSET>,
            CurrentName: CurrentName::<Impl, IMPL_OFFSET>,
            CurrentAcceleratorKey: CurrentAcceleratorKey::<Impl, IMPL_OFFSET>,
            CurrentAccessKey: CurrentAccessKey::<Impl, IMPL_OFFSET>,
            CurrentHasKeyboardFocus: CurrentHasKeyboardFocus::<Impl, IMPL_OFFSET>,
            CurrentIsKeyboardFocusable: CurrentIsKeyboardFocusable::<Impl, IMPL_OFFSET>,
            CurrentIsEnabled: CurrentIsEnabled::<Impl, IMPL_OFFSET>,
            CurrentAutomationId: CurrentAutomationId::<Impl, IMPL_OFFSET>,
            CurrentClassName: CurrentClassName::<Impl, IMPL_OFFSET>,
            CurrentHelpText: CurrentHelpText::<Impl, IMPL_OFFSET>,
            CurrentCulture: CurrentCulture::<Impl, IMPL_OFFSET>,
            CurrentIsControlElement: CurrentIsControlElement::<Impl, IMPL_OFFSET>,
            CurrentIsContentElement: CurrentIsContentElement::<Impl, IMPL_OFFSET>,
            CurrentIsPassword: CurrentIsPassword::<Impl, IMPL_OFFSET>,
            CurrentNativeWindowHandle: CurrentNativeWindowHandle::<Impl, IMPL_OFFSET>,
            CurrentItemType: CurrentItemType::<Impl, IMPL_OFFSET>,
            CurrentIsOffscreen: CurrentIsOffscreen::<Impl, IMPL_OFFSET>,
            CurrentOrientation: CurrentOrientation::<Impl, IMPL_OFFSET>,
            CurrentFrameworkId: CurrentFrameworkId::<Impl, IMPL_OFFSET>,
            CurrentIsRequiredForForm: CurrentIsRequiredForForm::<Impl, IMPL_OFFSET>,
            CurrentItemStatus: CurrentItemStatus::<Impl, IMPL_OFFSET>,
            CurrentBoundingRectangle: CurrentBoundingRectangle::<Impl, IMPL_OFFSET>,
            CurrentLabeledBy: CurrentLabeledBy::<Impl, IMPL_OFFSET>,
            CurrentAriaRole: CurrentAriaRole::<Impl, IMPL_OFFSET>,
            CurrentAriaProperties: CurrentAriaProperties::<Impl, IMPL_OFFSET>,
            CurrentIsDataValidForForm: CurrentIsDataValidForForm::<Impl, IMPL_OFFSET>,
            CurrentControllerFor: CurrentControllerFor::<Impl, IMPL_OFFSET>,
            CurrentDescribedBy: CurrentDescribedBy::<Impl, IMPL_OFFSET>,
            CurrentFlowsTo: CurrentFlowsTo::<Impl, IMPL_OFFSET>,
            CurrentProviderDescription: CurrentProviderDescription::<Impl, IMPL_OFFSET>,
            CachedProcessId: CachedProcessId::<Impl, IMPL_OFFSET>,
            CachedControlType: CachedControlType::<Impl, IMPL_OFFSET>,
            CachedLocalizedControlType: CachedLocalizedControlType::<Impl, IMPL_OFFSET>,
            CachedName: CachedName::<Impl, IMPL_OFFSET>,
            CachedAcceleratorKey: CachedAcceleratorKey::<Impl, IMPL_OFFSET>,
            CachedAccessKey: CachedAccessKey::<Impl, IMPL_OFFSET>,
            CachedHasKeyboardFocus: CachedHasKeyboardFocus::<Impl, IMPL_OFFSET>,
            CachedIsKeyboardFocusable: CachedIsKeyboardFocusable::<Impl, IMPL_OFFSET>,
            CachedIsEnabled: CachedIsEnabled::<Impl, IMPL_OFFSET>,
            CachedAutomationId: CachedAutomationId::<Impl, IMPL_OFFSET>,
            CachedClassName: CachedClassName::<Impl, IMPL_OFFSET>,
            CachedHelpText: CachedHelpText::<Impl, IMPL_OFFSET>,
            CachedCulture: CachedCulture::<Impl, IMPL_OFFSET>,
            CachedIsControlElement: CachedIsControlElement::<Impl, IMPL_OFFSET>,
            CachedIsContentElement: CachedIsContentElement::<Impl, IMPL_OFFSET>,
            CachedIsPassword: CachedIsPassword::<Impl, IMPL_OFFSET>,
            CachedNativeWindowHandle: CachedNativeWindowHandle::<Impl, IMPL_OFFSET>,
            CachedItemType: CachedItemType::<Impl, IMPL_OFFSET>,
            CachedIsOffscreen: CachedIsOffscreen::<Impl, IMPL_OFFSET>,
            CachedOrientation: CachedOrientation::<Impl, IMPL_OFFSET>,
            CachedFrameworkId: CachedFrameworkId::<Impl, IMPL_OFFSET>,
            CachedIsRequiredForForm: CachedIsRequiredForForm::<Impl, IMPL_OFFSET>,
            CachedItemStatus: CachedItemStatus::<Impl, IMPL_OFFSET>,
            CachedBoundingRectangle: CachedBoundingRectangle::<Impl, IMPL_OFFSET>,
            CachedLabeledBy: CachedLabeledBy::<Impl, IMPL_OFFSET>,
            CachedAriaRole: CachedAriaRole::<Impl, IMPL_OFFSET>,
            CachedAriaProperties: CachedAriaProperties::<Impl, IMPL_OFFSET>,
            CachedIsDataValidForForm: CachedIsDataValidForForm::<Impl, IMPL_OFFSET>,
            CachedControllerFor: CachedControllerFor::<Impl, IMPL_OFFSET>,
            CachedDescribedBy: CachedDescribedBy::<Impl, IMPL_OFFSET>,
            CachedFlowsTo: CachedFlowsTo::<Impl, IMPL_OFFSET>,
            CachedProviderDescription: CachedProviderDescription::<Impl, IMPL_OFFSET>,
            GetClickablePoint: GetClickablePoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement2_Impl: Sized + IUIAutomationElement_Impl {
    fn CurrentOptimizeForVisualContent(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedOptimizeForVisualContent(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentLiveSetting(&mut self) -> ::windows::core::Result<LiveSetting>;
    fn CachedLiveSetting(&mut self) -> ::windows::core::Result<LiveSetting>;
    fn CurrentFlowsFrom(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CachedFlowsFrom(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement2_Vtbl {
        unsafe extern "system" fn CurrentOptimizeForVisualContent<Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentOptimizeForVisualContent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedOptimizeForVisualContent<Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedOptimizeForVisualContent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLiveSetting<Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLiveSetting() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLiveSetting<Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLiveSetting() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFlowsFrom<Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFlowsFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFlowsFrom<Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFlowsFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentOptimizeForVisualContent: CurrentOptimizeForVisualContent::<Impl, IMPL_OFFSET>,
            CachedOptimizeForVisualContent: CachedOptimizeForVisualContent::<Impl, IMPL_OFFSET>,
            CurrentLiveSetting: CurrentLiveSetting::<Impl, IMPL_OFFSET>,
            CachedLiveSetting: CachedLiveSetting::<Impl, IMPL_OFFSET>,
            CurrentFlowsFrom: CurrentFlowsFrom::<Impl, IMPL_OFFSET>,
            CachedFlowsFrom: CachedFlowsFrom::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement3_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl {
    fn ShowContextMenu(&mut self) -> ::windows::core::Result<()>;
    fn CurrentIsPeripheral(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsPeripheral(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement3_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        unsafe extern "system" fn CurrentIsPeripheral<Impl: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsPeripheral() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsPeripheral<Impl: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsPeripheral() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ShowContextMenu: ShowContextMenu::<Impl, IMPL_OFFSET>,
            CurrentIsPeripheral: CurrentIsPeripheral::<Impl, IMPL_OFFSET>,
            CachedIsPeripheral: CachedIsPeripheral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement4_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl + IUIAutomationElement3_Impl {
    fn CurrentPositionInSet(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentSizeOfSet(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentLevel(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentAnnotationTypes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentAnnotationObjects(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CachedPositionInSet(&mut self) -> ::windows::core::Result<i32>;
    fn CachedSizeOfSet(&mut self) -> ::windows::core::Result<i32>;
    fn CachedLevel(&mut self) -> ::windows::core::Result<i32>;
    fn CachedAnnotationTypes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedAnnotationObjects(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement4_Vtbl {
        unsafe extern "system" fn CurrentPositionInSet<Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPositionInSet() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSizeOfSet<Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSizeOfSet() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLevel<Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationTypes<Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationObjects<Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedPositionInSet<Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedPositionInSet() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSizeOfSet<Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedSizeOfSet() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLevel<Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypes<Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationObjects<Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentPositionInSet: CurrentPositionInSet::<Impl, IMPL_OFFSET>,
            CurrentSizeOfSet: CurrentSizeOfSet::<Impl, IMPL_OFFSET>,
            CurrentLevel: CurrentLevel::<Impl, IMPL_OFFSET>,
            CurrentAnnotationTypes: CurrentAnnotationTypes::<Impl, IMPL_OFFSET>,
            CurrentAnnotationObjects: CurrentAnnotationObjects::<Impl, IMPL_OFFSET>,
            CachedPositionInSet: CachedPositionInSet::<Impl, IMPL_OFFSET>,
            CachedSizeOfSet: CachedSizeOfSet::<Impl, IMPL_OFFSET>,
            CachedLevel: CachedLevel::<Impl, IMPL_OFFSET>,
            CachedAnnotationTypes: CachedAnnotationTypes::<Impl, IMPL_OFFSET>,
            CachedAnnotationObjects: CachedAnnotationObjects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement5_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl + IUIAutomationElement3_Impl + IUIAutomationElement4_Impl {
    fn CurrentLandmarkType(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentLocalizedLandmarkType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedLandmarkType(&mut self) -> ::windows::core::Result<i32>;
    fn CachedLocalizedLandmarkType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement5_Vtbl {
        unsafe extern "system" fn CurrentLandmarkType<Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLocalizedLandmarkType<Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLocalizedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLandmarkType<Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLocalizedLandmarkType<Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLocalizedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement4_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentLandmarkType: CurrentLandmarkType::<Impl, IMPL_OFFSET>,
            CurrentLocalizedLandmarkType: CurrentLocalizedLandmarkType::<Impl, IMPL_OFFSET>,
            CachedLandmarkType: CachedLandmarkType::<Impl, IMPL_OFFSET>,
            CachedLocalizedLandmarkType: CachedLocalizedLandmarkType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement6_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl + IUIAutomationElement3_Impl + IUIAutomationElement4_Impl + IUIAutomationElement5_Impl {
    fn CurrentFullDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedFullDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement6_Vtbl {
        unsafe extern "system" fn CurrentFullDescription<Impl: IUIAutomationElement6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFullDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFullDescription<Impl: IUIAutomationElement6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFullDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement5_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentFullDescription: CurrentFullDescription::<Impl, IMPL_OFFSET>,
            CachedFullDescription: CachedFullDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement7_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl + IUIAutomationElement3_Impl + IUIAutomationElement4_Impl + IUIAutomationElement5_Impl + IUIAutomationElement6_Impl {
    fn FindFirstWithOptions(&mut self, scope: TreeScope, condition: ::core::option::Option<IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn FindAllWithOptions(&mut self, scope: TreeScope, condition: ::core::option::Option<IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn FindFirstWithOptionsBuildCache(&mut self, scope: TreeScope, condition: ::core::option::Option<IUIAutomationCondition>, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn FindAllWithOptionsBuildCache(&mut self, scope: TreeScope, condition: ::core::option::Option<IUIAutomationCondition>, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetCurrentMetadataValue(&mut self, targetid: i32, metadataid: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement7_Vtbl {
        unsafe extern "system" fn FindFirstWithOptions<Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstWithOptions(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute_copy(&traversaloptions), ::core::mem::transmute(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithOptions<Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllWithOptions(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute_copy(&traversaloptions), ::core::mem::transmute(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstWithOptionsBuildCache<Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstWithOptionsBuildCache(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute(&cacherequest), ::core::mem::transmute_copy(&traversaloptions), ::core::mem::transmute(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithOptionsBuildCache<Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllWithOptionsBuildCache(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute(&cacherequest), ::core::mem::transmute_copy(&traversaloptions), ::core::mem::transmute(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentMetadataValue<Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: i32, returnval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentMetadataValue(::core::mem::transmute_copy(&targetid), ::core::mem::transmute_copy(&metadataid)) {
                ::core::result::Result::Ok(ok__) => {
                    *returnval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement6_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FindFirstWithOptions: FindFirstWithOptions::<Impl, IMPL_OFFSET>,
            FindAllWithOptions: FindAllWithOptions::<Impl, IMPL_OFFSET>,
            FindFirstWithOptionsBuildCache: FindFirstWithOptionsBuildCache::<Impl, IMPL_OFFSET>,
            FindAllWithOptionsBuildCache: FindAllWithOptionsBuildCache::<Impl, IMPL_OFFSET>,
            GetCurrentMetadataValue: GetCurrentMetadataValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement8_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl + IUIAutomationElement3_Impl + IUIAutomationElement4_Impl + IUIAutomationElement5_Impl + IUIAutomationElement6_Impl + IUIAutomationElement7_Impl {
    fn CurrentHeadingLevel(&mut self) -> ::windows::core::Result<i32>;
    fn CachedHeadingLevel(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement8_Vtbl {
        unsafe extern "system" fn CurrentHeadingLevel<Impl: IUIAutomationElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHeadingLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHeadingLevel<Impl: IUIAutomationElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHeadingLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement7_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentHeadingLevel: CurrentHeadingLevel::<Impl, IMPL_OFFSET>,
            CachedHeadingLevel: CachedHeadingLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement9_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl + IUIAutomationElement3_Impl + IUIAutomationElement4_Impl + IUIAutomationElement5_Impl + IUIAutomationElement6_Impl + IUIAutomationElement7_Impl + IUIAutomationElement8_Impl {
    fn CurrentIsDialog(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsDialog(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement9_Vtbl {
        unsafe extern "system" fn CurrentIsDialog<Impl: IUIAutomationElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsDialog() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsDialog<Impl: IUIAutomationElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsDialog() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement8_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentIsDialog: CurrentIsDialog::<Impl, IMPL_OFFSET>,
            CachedIsDialog: CachedIsDialog::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement9 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationElementArray_Impl: Sized {
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn GetElement(&mut self, index: i32) -> ::windows::core::Result<IUIAutomationElement>;
}
impl IUIAutomationElementArray_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElementArray_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElementArray_Vtbl {
        unsafe extern "system" fn Length<Impl: IUIAutomationElementArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Impl: IUIAutomationElementArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElement(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Length: Length::<Impl, IMPL_OFFSET>,
            GetElement: GetElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElementArray as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationEventHandler_Impl: Sized {
    fn HandleAutomationEvent(&mut self, sender: ::core::option::Option<IUIAutomationElement>, eventid: i32) -> ::windows::core::Result<()>;
}
impl IUIAutomationEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationEventHandler_Vtbl {
        unsafe extern "system" fn HandleAutomationEvent<Impl: IUIAutomationEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, eventid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleAutomationEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&eventid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandleAutomationEvent: HandleAutomationEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationEventHandlerGroup_Impl: Sized {
    fn AddActiveTextPositionChangedEventHandler(&mut self, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows::core::Result<()>;
    fn AddAutomationEventHandler(&mut self, eventid: i32, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationEventHandler>) -> ::windows::core::Result<()>;
    fn AddChangesEventHandler(&mut self, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationChangesEventHandler>) -> ::windows::core::Result<()>;
    fn AddNotificationEventHandler(&mut self, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationNotificationEventHandler>) -> ::windows::core::Result<()>;
    fn AddPropertyChangedEventHandler(&mut self, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const i32, propertycount: i32) -> ::windows::core::Result<()>;
    fn AddStructureChangedEventHandler(&mut self, scope: TreeScope, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationStructureChangedEventHandler>) -> ::windows::core::Result<()>;
    fn AddTextEditTextChangedEventHandler(&mut self, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>, handler: ::core::option::Option<IUIAutomationTextEditTextChangedEventHandler>) -> ::windows::core::Result<()>;
}
impl IUIAutomationEventHandlerGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationEventHandlerGroup_Vtbl {
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddActiveTextPositionChangedEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddAutomationEventHandler<Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAutomationEventHandler(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddChangesEventHandler<Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChangesEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&changetypes), ::core::mem::transmute_copy(&changescount), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddNotificationEventHandler<Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddNotificationEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const i32, propertycount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyChangedEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler), ::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStructureChangedEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTextEditTextChangedEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&texteditchangetype), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Impl, IMPL_OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Impl, IMPL_OFFSET>,
            AddChangesEventHandler: AddChangesEventHandler::<Impl, IMPL_OFFSET>,
            AddNotificationEventHandler: AddNotificationEventHandler::<Impl, IMPL_OFFSET>,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Impl, IMPL_OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Impl, IMPL_OFFSET>,
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationEventHandlerGroup as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationExpandCollapsePattern_Impl: Sized {
    fn Expand(&mut self) -> ::windows::core::Result<()>;
    fn Collapse(&mut self) -> ::windows::core::Result<()>;
    fn CurrentExpandCollapseState(&mut self) -> ::windows::core::Result<ExpandCollapseState>;
    fn CachedExpandCollapseState(&mut self) -> ::windows::core::Result<ExpandCollapseState>;
}
impl IUIAutomationExpandCollapsePattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationExpandCollapsePattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationExpandCollapsePattern_Vtbl {
        unsafe extern "system" fn Expand<Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Expand().into()
        }
        unsafe extern "system" fn Collapse<Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Collapse().into()
        }
        unsafe extern "system" fn CurrentExpandCollapseState<Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedExpandCollapseState<Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Expand: Expand::<Impl, IMPL_OFFSET>,
            Collapse: Collapse::<Impl, IMPL_OFFSET>,
            CurrentExpandCollapseState: CurrentExpandCollapseState::<Impl, IMPL_OFFSET>,
            CachedExpandCollapseState: CachedExpandCollapseState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationExpandCollapsePattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationFocusChangedEventHandler_Impl: Sized {
    fn HandleFocusChangedEvent(&mut self, sender: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<()>;
}
impl IUIAutomationFocusChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationFocusChangedEventHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationFocusChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleFocusChangedEvent<Impl: IUIAutomationFocusChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleFocusChangedEvent(::core::mem::transmute(&sender)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandleFocusChangedEvent: HandleFocusChangedEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationFocusChangedEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationGridItemPattern_Impl: Sized {
    fn CurrentContainingGrid(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CurrentRow(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentColumn(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentRowSpan(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentColumnSpan(&mut self) -> ::windows::core::Result<i32>;
    fn CachedContainingGrid(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CachedRow(&mut self) -> ::windows::core::Result<i32>;
    fn CachedColumn(&mut self) -> ::windows::core::Result<i32>;
    fn CachedRowSpan(&mut self) -> ::windows::core::Result<i32>;
    fn CachedColumnSpan(&mut self) -> ::windows::core::Result<i32>;
}
impl IUIAutomationGridItemPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationGridItemPattern_Vtbl {
        unsafe extern "system" fn CurrentContainingGrid<Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRow<Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRow() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumn<Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentColumn() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowSpan<Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumnSpan<Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedContainingGrid<Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRow<Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedRow() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumn<Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedColumn() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowSpan<Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedRowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumnSpan<Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CurrentContainingGrid: CurrentContainingGrid::<Impl, IMPL_OFFSET>,
            CurrentRow: CurrentRow::<Impl, IMPL_OFFSET>,
            CurrentColumn: CurrentColumn::<Impl, IMPL_OFFSET>,
            CurrentRowSpan: CurrentRowSpan::<Impl, IMPL_OFFSET>,
            CurrentColumnSpan: CurrentColumnSpan::<Impl, IMPL_OFFSET>,
            CachedContainingGrid: CachedContainingGrid::<Impl, IMPL_OFFSET>,
            CachedRow: CachedRow::<Impl, IMPL_OFFSET>,
            CachedColumn: CachedColumn::<Impl, IMPL_OFFSET>,
            CachedRowSpan: CachedRowSpan::<Impl, IMPL_OFFSET>,
            CachedColumnSpan: CachedColumnSpan::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationGridItemPattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationGridPattern_Impl: Sized {
    fn GetItem(&mut self, row: i32, column: i32) -> ::windows::core::Result<IUIAutomationElement>;
    fn CurrentRowCount(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentColumnCount(&mut self) -> ::windows::core::Result<i32>;
    fn CachedRowCount(&mut self) -> ::windows::core::Result<i32>;
    fn CachedColumnCount(&mut self) -> ::windows::core::Result<i32>;
}
impl IUIAutomationGridPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationGridPattern_Vtbl {
        unsafe extern "system" fn GetItem<Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowCount<Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRowCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumnCount<Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowCount<Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedRowCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumnCount<Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
            CurrentRowCount: CurrentRowCount::<Impl, IMPL_OFFSET>,
            CurrentColumnCount: CurrentColumnCount::<Impl, IMPL_OFFSET>,
            CachedRowCount: CachedRowCount::<Impl, IMPL_OFFSET>,
            CachedColumnCount: CachedColumnCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationGridPattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationInvokePattern_Impl: Sized {
    fn Invoke(&mut self) -> ::windows::core::Result<()>;
}
impl IUIAutomationInvokePattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationInvokePattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationInvokePattern_Vtbl {
        unsafe extern "system" fn Invoke<Impl: IUIAutomationInvokePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationInvokePattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationItemContainerPattern_Impl: Sized {
    fn FindItemByProperty(&mut self, pstartafter: ::core::option::Option<IUIAutomationElement>, propertyid: i32, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationItemContainerPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationItemContainerPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationItemContainerPattern_Vtbl {
        unsafe extern "system" fn FindItemByProperty<Impl: IUIAutomationItemContainerPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartafter: ::windows::core::RawPtr, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItemByProperty(::core::mem::transmute(&pstartafter), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfound = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FindItemByProperty: FindItemByProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationItemContainerPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUIAutomationLegacyIAccessiblePattern_Impl: Sized {
    fn Select(&mut self, flagsselect: i32) -> ::windows::core::Result<()>;
    fn DoDefaultAction(&mut self) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, szvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CurrentChildId(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentValue(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentRole(&mut self) -> ::windows::core::Result<u32>;
    fn CurrentState(&mut self) -> ::windows::core::Result<u32>;
    fn CurrentHelp(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentKeyboardShortcut(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCurrentSelection(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CurrentDefaultAction(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedChildId(&mut self) -> ::windows::core::Result<i32>;
    fn CachedName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedValue(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedRole(&mut self) -> ::windows::core::Result<u32>;
    fn CachedState(&mut self) -> ::windows::core::Result<u32>;
    fn CachedHelp(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedKeyboardShortcut(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCachedSelection(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CachedDefaultAction(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetIAccessible(&mut self) -> ::windows::core::Result<IAccessible>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationLegacyIAccessiblePattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationLegacyIAccessiblePattern_Vtbl {
        unsafe extern "system" fn Select<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Select(::core::mem::transmute_copy(&flagsselect)).into()
        }
        unsafe extern "system" fn DoDefaultAction<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoDefaultAction().into()
        }
        unsafe extern "system" fn SetValue<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&szvalue)).into()
        }
        unsafe extern "system" fn CurrentChildId<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentChildId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentName<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentName() {
                ::core::result::Result::Ok(ok__) => {
                    *pszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentValue<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentValue() {
                ::core::result::Result::Ok(ok__) => {
                    *pszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDescription<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRole<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRole() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentState<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHelp<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHelp() {
                ::core::result::Result::Ok(ok__) => {
                    *pszhelp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentKeyboardShortcut<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentKeyboardShortcut() {
                ::core::result::Result::Ok(ok__) => {
                    *pszkeyboardshortcut = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSelection<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselectedchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDefaultAction<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdefaultaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedChildId<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedChildId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedName<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedName() {
                ::core::result::Result::Ok(ok__) => {
                    *pszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *pszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDescription<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRole<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedRole() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedState<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHelp<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHelp() {
                ::core::result::Result::Ok(ok__) => {
                    *pszhelp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedKeyboardShortcut<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedKeyboardShortcut() {
                ::core::result::Result::Ok(ok__) => {
                    *pszkeyboardshortcut = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSelection<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselectedchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDefaultAction<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdefaultaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIAccessible<Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIAccessible() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaccessible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Select: Select::<Impl, IMPL_OFFSET>,
            DoDefaultAction: DoDefaultAction::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            CurrentChildId: CurrentChildId::<Impl, IMPL_OFFSET>,
            CurrentName: CurrentName::<Impl, IMPL_OFFSET>,
            CurrentValue: CurrentValue::<Impl, IMPL_OFFSET>,
            CurrentDescription: CurrentDescription::<Impl, IMPL_OFFSET>,
            CurrentRole: CurrentRole::<Impl, IMPL_OFFSET>,
            CurrentState: CurrentState::<Impl, IMPL_OFFSET>,
            CurrentHelp: CurrentHelp::<Impl, IMPL_OFFSET>,
            CurrentKeyboardShortcut: CurrentKeyboardShortcut::<Impl, IMPL_OFFSET>,
            GetCurrentSelection: GetCurrentSelection::<Impl, IMPL_OFFSET>,
            CurrentDefaultAction: CurrentDefaultAction::<Impl, IMPL_OFFSET>,
            CachedChildId: CachedChildId::<Impl, IMPL_OFFSET>,
            CachedName: CachedName::<Impl, IMPL_OFFSET>,
            CachedValue: CachedValue::<Impl, IMPL_OFFSET>,
            CachedDescription: CachedDescription::<Impl, IMPL_OFFSET>,
            CachedRole: CachedRole::<Impl, IMPL_OFFSET>,
            CachedState: CachedState::<Impl, IMPL_OFFSET>,
            CachedHelp: CachedHelp::<Impl, IMPL_OFFSET>,
            CachedKeyboardShortcut: CachedKeyboardShortcut::<Impl, IMPL_OFFSET>,
            GetCachedSelection: GetCachedSelection::<Impl, IMPL_OFFSET>,
            CachedDefaultAction: CachedDefaultAction::<Impl, IMPL_OFFSET>,
            GetIAccessible: GetIAccessible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationLegacyIAccessiblePattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUIAutomationMultipleViewPattern_Impl: Sized {
    fn GetViewName(&mut self, view: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCurrentView(&mut self, view: i32) -> ::windows::core::Result<()>;
    fn CurrentCurrentView(&mut self) -> ::windows::core::Result<i32>;
    fn GetCurrentSupportedViews(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedCurrentView(&mut self) -> ::windows::core::Result<i32>;
    fn GetCachedSupportedViews(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationMultipleViewPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationMultipleViewPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationMultipleViewPattern_Vtbl {
        unsafe extern "system" fn GetViewName<Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewName(::core::mem::transmute_copy(&view)) {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentView<Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentView(::core::mem::transmute_copy(&view)).into()
        }
        unsafe extern "system" fn CurrentCurrentView<Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSupportedViews<Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSupportedViews() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCurrentView<Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSupportedViews<Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedSupportedViews() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetViewName: GetViewName::<Impl, IMPL_OFFSET>,
            SetCurrentView: SetCurrentView::<Impl, IMPL_OFFSET>,
            CurrentCurrentView: CurrentCurrentView::<Impl, IMPL_OFFSET>,
            GetCurrentSupportedViews: GetCurrentSupportedViews::<Impl, IMPL_OFFSET>,
            CachedCurrentView: CachedCurrentView::<Impl, IMPL_OFFSET>,
            GetCachedSupportedViews: GetCachedSupportedViews::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationMultipleViewPattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationNotCondition_Impl: Sized + IUIAutomationCondition_Impl {
    fn GetChild(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
}
impl IUIAutomationNotCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationNotCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationNotCondition_Vtbl {
        unsafe extern "system" fn GetChild<Impl: IUIAutomationNotCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChild() {
                ::core::result::Result::Ok(ok__) => {
                    *condition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUIAutomationCondition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetChild: GetChild::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationNotCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationNotificationEventHandler_Impl: Sized {
    fn HandleNotificationEvent(&mut self, sender: ::core::option::Option<IUIAutomationElement>, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: super::super::Foundation::BSTR, activityid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationNotificationEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationNotificationEventHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationNotificationEventHandler_Vtbl {
        unsafe extern "system" fn HandleNotificationEvent<Impl: IUIAutomationNotificationEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, activityid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleNotificationEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&notificationkind), ::core::mem::transmute_copy(&notificationprocessing), ::core::mem::transmute_copy(&displaystring), ::core::mem::transmute_copy(&activityid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandleNotificationEvent: HandleNotificationEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationNotificationEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationObjectModelPattern_Impl: Sized {
    fn GetUnderlyingObjectModel(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IUIAutomationObjectModelPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationObjectModelPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationObjectModelPattern_Vtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Impl: IUIAutomationObjectModelPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnderlyingObjectModel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationObjectModelPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationOrCondition_Impl: Sized + IUIAutomationCondition_Impl {
    fn ChildCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetChildrenAsNativeArray(&mut self, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows::core::Result<()>;
    fn GetChildren(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationOrCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationOrCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationOrCondition_Vtbl {
        unsafe extern "system" fn ChildCount<Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildCount() {
                ::core::result::Result::Ok(ok__) => {
                    *childcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::windows::core::RawPtr, childarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChildrenAsNativeArray(::core::mem::transmute_copy(&childarray), ::core::mem::transmute_copy(&childarraycount)).into()
        }
        unsafe extern "system" fn GetChildren<Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *childarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationCondition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ChildCount: ChildCount::<Impl, IMPL_OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Impl, IMPL_OFFSET>,
            GetChildren: GetChildren::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationOrCondition as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationPatternHandler_Impl: Sized {
    fn CreateClientWrapper(&mut self, ppatterninstance: ::core::option::Option<IUIAutomationPatternInstance>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Dispatch(&mut self, ptarget: ::core::option::Option<::windows::core::IUnknown>, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::Result<()>;
}
impl IUIAutomationPatternHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationPatternHandler_Vtbl {
        unsafe extern "system" fn CreateClientWrapper<Impl: IUIAutomationPatternHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppatterninstance: ::windows::core::RawPtr, pclientwrapper: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateClientWrapper(::core::mem::transmute(&ppatterninstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *pclientwrapper = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatch<Impl: IUIAutomationPatternHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Dispatch(::core::mem::transmute(&ptarget), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&cparams)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateClientWrapper: CreateClientWrapper::<Impl, IMPL_OFFSET>,
            Dispatch: Dispatch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationPatternHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationPatternInstance_Impl: Sized {
    fn GetProperty(&mut self, index: u32, cached: super::super::Foundation::BOOL, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CallMethod(&mut self, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationPatternInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternInstance_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationPatternInstance_Vtbl {
        unsafe extern "system" fn GetProperty<Impl: IUIAutomationPatternInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, cached: super::super::Foundation::BOOL, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&cached), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pptr)).into()
        }
        unsafe extern "system" fn CallMethod<Impl: IUIAutomationPatternInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CallMethod(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&cparams)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            CallMethod: CallMethod::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationPatternInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationPropertyChangedEventHandler_Impl: Sized {
    fn HandlePropertyChangedEvent(&mut self, sender: ::core::option::Option<IUIAutomationElement>, propertyid: i32, newvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationPropertyChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyChangedEventHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationPropertyChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandlePropertyChangedEvent<Impl: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, propertyid: i32, newvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandlePropertyChangedEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&newvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandlePropertyChangedEvent: HandlePropertyChangedEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationPropertyChangedEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationPropertyCondition_Impl: Sized + IUIAutomationCondition_Impl {
    fn PropertyId(&mut self) -> ::windows::core::Result<i32>;
    fn PropertyValue(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PropertyConditionFlags(&mut self) -> ::windows::core::Result<PropertyConditionFlags>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationPropertyCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationPropertyCondition_Vtbl {
        unsafe extern "system" fn PropertyId<Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyId() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyValue() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyConditionFlags<Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut PropertyConditionFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyConditionFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationCondition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PropertyId: PropertyId::<Impl, IMPL_OFFSET>,
            PropertyValue: PropertyValue::<Impl, IMPL_OFFSET>,
            PropertyConditionFlags: PropertyConditionFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationPropertyCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationProxyFactory_Impl: Sized {
    fn CreateProvider(&mut self, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn ProxyFactoryId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationProxyFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationProxyFactory_Vtbl {
        unsafe extern "system" fn CreateProvider<Impl: IUIAutomationProxyFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, provider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProvider(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *provider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyFactoryId<Impl: IUIAutomationProxyFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoryid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyFactoryId() {
                ::core::result::Result::Ok(ok__) => {
                    *factoryid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateProvider: CreateProvider::<Impl, IMPL_OFFSET>,
            ProxyFactoryId: ProxyFactoryId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUIAutomationProxyFactoryEntry_Impl: Sized {
    fn ProxyFactory(&mut self) -> ::windows::core::Result<IUIAutomationProxyFactory>;
    fn ClassName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ImageName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AllowSubstringMatch(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CanCheckBaseClass(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn NeedsAdviseEvents(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetClassName(&mut self, classname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetImageName(&mut self, imagename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetAllowSubstringMatch(&mut self, allowsubstringmatch: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetCanCheckBaseClass(&mut self, cancheckbaseclass: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetNeedsAdviseEvents(&mut self, adviseevents: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetWinEventsForAutomationEvent(&mut self, eventid: i32, propertyid: i32, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn GetWinEventsForAutomationEvent(&mut self, eventid: i32, propertyid: i32) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationProxyFactoryEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationProxyFactoryEntry_Vtbl {
        unsafe extern "system" fn ProxyFactory<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyFactory() {
                ::core::result::Result::Ok(ok__) => {
                    *factory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassName<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassName() {
                ::core::result::Result::Ok(ok__) => {
                    *classname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageName() {
                ::core::result::Result::Ok(ok__) => {
                    *imagename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSubstringMatch<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowSubstringMatch() {
                ::core::result::Result::Ok(ok__) => {
                    *allowsubstringmatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCheckBaseClass<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCheckBaseClass() {
                ::core::result::Result::Ok(ok__) => {
                    *cancheckbaseclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeedsAdviseEvents<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adviseevents: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeedsAdviseEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *adviseevents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassName<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClassName(::core::mem::transmute_copy(&classname)).into()
        }
        unsafe extern "system" fn SetImageName<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImageName(::core::mem::transmute_copy(&imagename)).into()
        }
        unsafe extern "system" fn SetAllowSubstringMatch<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowSubstringMatch(::core::mem::transmute_copy(&allowsubstringmatch)).into()
        }
        unsafe extern "system" fn SetCanCheckBaseClass<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanCheckBaseClass(::core::mem::transmute_copy(&cancheckbaseclass)).into()
        }
        unsafe extern "system" fn SetNeedsAdviseEvents<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adviseevents: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNeedsAdviseEvents(::core::mem::transmute_copy(&adviseevents)).into()
        }
        unsafe extern "system" fn SetWinEventsForAutomationEvent<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyid: i32, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWinEventsForAutomationEvent(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&winevents)).into()
        }
        unsafe extern "system" fn GetWinEventsForAutomationEvent<Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyid: i32, winevents: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWinEventsForAutomationEvent(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *winevents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProxyFactory: ProxyFactory::<Impl, IMPL_OFFSET>,
            ClassName: ClassName::<Impl, IMPL_OFFSET>,
            ImageName: ImageName::<Impl, IMPL_OFFSET>,
            AllowSubstringMatch: AllowSubstringMatch::<Impl, IMPL_OFFSET>,
            CanCheckBaseClass: CanCheckBaseClass::<Impl, IMPL_OFFSET>,
            NeedsAdviseEvents: NeedsAdviseEvents::<Impl, IMPL_OFFSET>,
            SetClassName: SetClassName::<Impl, IMPL_OFFSET>,
            SetImageName: SetImageName::<Impl, IMPL_OFFSET>,
            SetAllowSubstringMatch: SetAllowSubstringMatch::<Impl, IMPL_OFFSET>,
            SetCanCheckBaseClass: SetCanCheckBaseClass::<Impl, IMPL_OFFSET>,
            SetNeedsAdviseEvents: SetNeedsAdviseEvents::<Impl, IMPL_OFFSET>,
            SetWinEventsForAutomationEvent: SetWinEventsForAutomationEvent::<Impl, IMPL_OFFSET>,
            GetWinEventsForAutomationEvent: GetWinEventsForAutomationEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactoryEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationProxyFactoryMapping_Impl: Sized {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn GetTable(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEntry(&mut self, index: u32) -> ::windows::core::Result<IUIAutomationProxyFactoryEntry>;
    fn SetTable(&mut self, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn InsertEntries(&mut self, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn InsertEntry(&mut self, before: u32, factory: ::core::option::Option<IUIAutomationProxyFactoryEntry>) -> ::windows::core::Result<()>;
    fn RemoveEntry(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn ClearTable(&mut self) -> ::windows::core::Result<()>;
    fn RestoreDefaultTable(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationProxyFactoryMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationProxyFactoryMapping_Vtbl {
        unsafe extern "system" fn Count<Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTable<Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, table: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTable() {
                ::core::result::Result::Ok(ok__) => {
                    *table = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntry<Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEntry(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *entry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTable<Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTable(::core::mem::transmute_copy(&factorylist)).into()
        }
        unsafe extern "system" fn InsertEntries<Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEntries(::core::mem::transmute_copy(&before), ::core::mem::transmute_copy(&factorylist)).into()
        }
        unsafe extern "system" fn InsertEntry<Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: u32, factory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEntry(::core::mem::transmute_copy(&before), ::core::mem::transmute(&factory)).into()
        }
        unsafe extern "system" fn RemoveEntry<Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEntry(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn ClearTable<Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearTable().into()
        }
        unsafe extern "system" fn RestoreDefaultTable<Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreDefaultTable().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            GetTable: GetTable::<Impl, IMPL_OFFSET>,
            GetEntry: GetEntry::<Impl, IMPL_OFFSET>,
            SetTable: SetTable::<Impl, IMPL_OFFSET>,
            InsertEntries: InsertEntries::<Impl, IMPL_OFFSET>,
            InsertEntry: InsertEntry::<Impl, IMPL_OFFSET>,
            RemoveEntry: RemoveEntry::<Impl, IMPL_OFFSET>,
            ClearTable: ClearTable::<Impl, IMPL_OFFSET>,
            RestoreDefaultTable: RestoreDefaultTable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactoryMapping as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationRangeValuePattern_Impl: Sized {
    fn SetValue(&mut self, val: f64) -> ::windows::core::Result<()>;
    fn CurrentValue(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentIsReadOnly(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentMaximum(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentMinimum(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentLargeChange(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentSmallChange(&mut self) -> ::windows::core::Result<f64>;
    fn CachedValue(&mut self) -> ::windows::core::Result<f64>;
    fn CachedIsReadOnly(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedMaximum(&mut self) -> ::windows::core::Result<f64>;
    fn CachedMinimum(&mut self) -> ::windows::core::Result<f64>;
    fn CachedLargeChange(&mut self) -> ::windows::core::Result<f64>;
    fn CachedSmallChange(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationRangeValuePattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationRangeValuePattern_Vtbl {
        unsafe extern "system" fn SetValue<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn CurrentValue<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsReadOnly<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMaximum<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMinimum<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLargeChange<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSmallChange<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsReadOnly<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedMaximum<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedMinimum<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLargeChange<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSmallChange<Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedSmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            CurrentValue: CurrentValue::<Impl, IMPL_OFFSET>,
            CurrentIsReadOnly: CurrentIsReadOnly::<Impl, IMPL_OFFSET>,
            CurrentMaximum: CurrentMaximum::<Impl, IMPL_OFFSET>,
            CurrentMinimum: CurrentMinimum::<Impl, IMPL_OFFSET>,
            CurrentLargeChange: CurrentLargeChange::<Impl, IMPL_OFFSET>,
            CurrentSmallChange: CurrentSmallChange::<Impl, IMPL_OFFSET>,
            CachedValue: CachedValue::<Impl, IMPL_OFFSET>,
            CachedIsReadOnly: CachedIsReadOnly::<Impl, IMPL_OFFSET>,
            CachedMaximum: CachedMaximum::<Impl, IMPL_OFFSET>,
            CachedMinimum: CachedMinimum::<Impl, IMPL_OFFSET>,
            CachedLargeChange: CachedLargeChange::<Impl, IMPL_OFFSET>,
            CachedSmallChange: CachedSmallChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationRangeValuePattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationRegistrar_Impl: Sized {
    fn RegisterProperty(&mut self, property: *const UIAutomationPropertyInfo) -> ::windows::core::Result<i32>;
    fn RegisterEvent(&mut self, event: *const UIAutomationEventInfo) -> ::windows::core::Result<i32>;
    fn RegisterPattern(&mut self, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, propertyidcount: u32, ppropertyids: *mut i32, eventidcount: u32, peventids: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationRegistrar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRegistrar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationRegistrar_Vtbl {
        unsafe extern "system" fn RegisterProperty<Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const UIAutomationPropertyInfo, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterProperty(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEvent<Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const UIAutomationEventInfo, eventid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterEvent(::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    *eventid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPattern<Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, propertyidcount: u32, ppropertyids: *mut i32, eventidcount: u32, peventids: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterPattern(::core::mem::transmute_copy(&pattern), ::core::mem::transmute_copy(&ppatternid), ::core::mem::transmute_copy(&ppatternavailablepropertyid), ::core::mem::transmute_copy(&propertyidcount), ::core::mem::transmute_copy(&ppropertyids), ::core::mem::transmute_copy(&eventidcount), ::core::mem::transmute_copy(&peventids)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterProperty: RegisterProperty::<Impl, IMPL_OFFSET>,
            RegisterEvent: RegisterEvent::<Impl, IMPL_OFFSET>,
            RegisterPattern: RegisterPattern::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationRegistrar as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationScrollItemPattern_Impl: Sized {
    fn ScrollIntoView(&mut self) -> ::windows::core::Result<()>;
}
impl IUIAutomationScrollItemPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollItemPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationScrollItemPattern_Vtbl {
        unsafe extern "system" fn ScrollIntoView<Impl: IUIAutomationScrollItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollIntoView().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ScrollIntoView: ScrollIntoView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationScrollItemPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationScrollPattern_Impl: Sized {
    fn Scroll(&mut self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::Result<()>;
    fn SetScrollPercent(&mut self, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::Result<()>;
    fn CurrentHorizontalScrollPercent(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentVerticalScrollPercent(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentHorizontalViewSize(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentVerticalViewSize(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentHorizontallyScrollable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentVerticallyScrollable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedHorizontalScrollPercent(&mut self) -> ::windows::core::Result<f64>;
    fn CachedVerticalScrollPercent(&mut self) -> ::windows::core::Result<f64>;
    fn CachedHorizontalViewSize(&mut self) -> ::windows::core::Result<f64>;
    fn CachedVerticalViewSize(&mut self) -> ::windows::core::Result<f64>;
    fn CachedHorizontallyScrollable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedVerticallyScrollable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationScrollPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationScrollPattern_Vtbl {
        unsafe extern "system" fn Scroll<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Scroll(::core::mem::transmute_copy(&horizontalamount), ::core::mem::transmute_copy(&verticalamount)).into()
        }
        unsafe extern "system" fn SetScrollPercent<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScrollPercent(::core::mem::transmute_copy(&horizontalpercent), ::core::mem::transmute_copy(&verticalpercent)).into()
        }
        unsafe extern "system" fn CurrentHorizontalScrollPercent<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticalScrollPercent<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentVerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHorizontalViewSize<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticalViewSize<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentVerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHorizontallyScrollable<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticallyScrollable<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentVerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontalScrollPercent<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticalScrollPercent<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedVerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontalViewSize<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticalViewSize<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedVerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontallyScrollable<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticallyScrollable<Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedVerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Scroll: Scroll::<Impl, IMPL_OFFSET>,
            SetScrollPercent: SetScrollPercent::<Impl, IMPL_OFFSET>,
            CurrentHorizontalScrollPercent: CurrentHorizontalScrollPercent::<Impl, IMPL_OFFSET>,
            CurrentVerticalScrollPercent: CurrentVerticalScrollPercent::<Impl, IMPL_OFFSET>,
            CurrentHorizontalViewSize: CurrentHorizontalViewSize::<Impl, IMPL_OFFSET>,
            CurrentVerticalViewSize: CurrentVerticalViewSize::<Impl, IMPL_OFFSET>,
            CurrentHorizontallyScrollable: CurrentHorizontallyScrollable::<Impl, IMPL_OFFSET>,
            CurrentVerticallyScrollable: CurrentVerticallyScrollable::<Impl, IMPL_OFFSET>,
            CachedHorizontalScrollPercent: CachedHorizontalScrollPercent::<Impl, IMPL_OFFSET>,
            CachedVerticalScrollPercent: CachedVerticalScrollPercent::<Impl, IMPL_OFFSET>,
            CachedHorizontalViewSize: CachedHorizontalViewSize::<Impl, IMPL_OFFSET>,
            CachedVerticalViewSize: CachedVerticalViewSize::<Impl, IMPL_OFFSET>,
            CachedHorizontallyScrollable: CachedHorizontallyScrollable::<Impl, IMPL_OFFSET>,
            CachedVerticallyScrollable: CachedVerticallyScrollable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationScrollPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationSelectionItemPattern_Impl: Sized {
    fn Select(&mut self) -> ::windows::core::Result<()>;
    fn AddToSelection(&mut self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&mut self) -> ::windows::core::Result<()>;
    fn CurrentIsSelected(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentSelectionContainer(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CachedIsSelected(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedSelectionContainer(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationSelectionItemPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSelectionItemPattern_Vtbl {
        unsafe extern "system" fn Select<Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn AddToSelection<Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn CurrentIsSelected<Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSelectionContainer<Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsSelected<Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSelectionContainer<Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedSelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Select: Select::<Impl, IMPL_OFFSET>,
            AddToSelection: AddToSelection::<Impl, IMPL_OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Impl, IMPL_OFFSET>,
            CurrentIsSelected: CurrentIsSelected::<Impl, IMPL_OFFSET>,
            CurrentSelectionContainer: CurrentSelectionContainer::<Impl, IMPL_OFFSET>,
            CachedIsSelected: CachedIsSelected::<Impl, IMPL_OFFSET>,
            CachedSelectionContainer: CachedSelectionContainer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationSelectionItemPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationSelectionPattern_Impl: Sized {
    fn GetCurrentSelection(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CurrentCanSelectMultiple(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsSelectionRequired(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetCachedSelection(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CachedCanSelectMultiple(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsSelectionRequired(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationSelectionPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSelectionPattern_Vtbl {
        unsafe extern "system" fn GetCurrentSelection<Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanSelectMultiple<Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsSelectionRequired<Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSelection<Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanSelectMultiple<Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsSelectionRequired<Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrentSelection: GetCurrentSelection::<Impl, IMPL_OFFSET>,
            CurrentCanSelectMultiple: CurrentCanSelectMultiple::<Impl, IMPL_OFFSET>,
            CurrentIsSelectionRequired: CurrentIsSelectionRequired::<Impl, IMPL_OFFSET>,
            GetCachedSelection: GetCachedSelection::<Impl, IMPL_OFFSET>,
            CachedCanSelectMultiple: CachedCanSelectMultiple::<Impl, IMPL_OFFSET>,
            CachedIsSelectionRequired: CachedIsSelectionRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationSelectionPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationSelectionPattern2_Impl: Sized + IUIAutomationSelectionPattern_Impl {
    fn CurrentFirstSelectedItem(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CurrentLastSelectedItem(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CurrentCurrentSelectedItem(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CurrentItemCount(&mut self) -> ::windows::core::Result<i32>;
    fn CachedFirstSelectedItem(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CachedLastSelectedItem(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CachedCurrentSelectedItem(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn CachedItemCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationSelectionPattern2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSelectionPattern2_Vtbl {
        unsafe extern "system" fn CurrentFirstSelectedItem<Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFirstSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLastSelectedItem<Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLastSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCurrentSelectedItem<Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCurrentSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemCount<Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFirstSelectedItem<Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFirstSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLastSelectedItem<Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLastSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCurrentSelectedItem<Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCurrentSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemCount<Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationSelectionPattern_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentFirstSelectedItem: CurrentFirstSelectedItem::<Impl, IMPL_OFFSET>,
            CurrentLastSelectedItem: CurrentLastSelectedItem::<Impl, IMPL_OFFSET>,
            CurrentCurrentSelectedItem: CurrentCurrentSelectedItem::<Impl, IMPL_OFFSET>,
            CurrentItemCount: CurrentItemCount::<Impl, IMPL_OFFSET>,
            CachedFirstSelectedItem: CachedFirstSelectedItem::<Impl, IMPL_OFFSET>,
            CachedLastSelectedItem: CachedLastSelectedItem::<Impl, IMPL_OFFSET>,
            CachedCurrentSelectedItem: CachedCurrentSelectedItem::<Impl, IMPL_OFFSET>,
            CachedItemCount: CachedItemCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationSelectionPattern2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUIAutomationSpreadsheetItemPattern_Impl: Sized {
    fn CurrentFormula(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCurrentAnnotationObjects(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetCurrentAnnotationTypes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedFormula(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCachedAnnotationObjects(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetCachedAnnotationTypes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationSpreadsheetItemPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSpreadsheetItemPattern_Vtbl {
        unsafe extern "system" fn CurrentFormula<Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFormula() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentAnnotationObjects<Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentAnnotationTypes<Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFormula<Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFormula() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedAnnotationObjects<Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedAnnotationTypes<Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CurrentFormula: CurrentFormula::<Impl, IMPL_OFFSET>,
            GetCurrentAnnotationObjects: GetCurrentAnnotationObjects::<Impl, IMPL_OFFSET>,
            GetCurrentAnnotationTypes: GetCurrentAnnotationTypes::<Impl, IMPL_OFFSET>,
            CachedFormula: CachedFormula::<Impl, IMPL_OFFSET>,
            GetCachedAnnotationObjects: GetCachedAnnotationObjects::<Impl, IMPL_OFFSET>,
            GetCachedAnnotationTypes: GetCachedAnnotationTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationSpreadsheetItemPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationSpreadsheetPattern_Impl: Sized {
    fn GetItemByName(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<IUIAutomationElement>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationSpreadsheetPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSpreadsheetPattern_Vtbl {
        unsafe extern "system" fn GetItemByName<Impl: IUIAutomationSpreadsheetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetItemByName: GetItemByName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationSpreadsheetPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationStructureChangedEventHandler_Impl: Sized {
    fn HandleStructureChangedEvent(&mut self, sender: ::core::option::Option<IUIAutomationElement>, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationStructureChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStructureChangedEventHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationStructureChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleStructureChangedEvent<Impl: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleStructureChangedEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&changetype), ::core::mem::transmute_copy(&runtimeid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            HandleStructureChangedEvent: HandleStructureChangedEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationStructureChangedEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationStylesPattern_Impl: Sized {
    fn CurrentStyleId(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentStyleName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentFillColor(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentFillPatternStyle(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentShape(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentFillPatternColor(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentExtendedProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCurrentExtendedPropertiesAsArray(&mut self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::Result<()>;
    fn CachedStyleId(&mut self) -> ::windows::core::Result<i32>;
    fn CachedStyleName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedFillColor(&mut self) -> ::windows::core::Result<i32>;
    fn CachedFillPatternStyle(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedShape(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedFillPatternColor(&mut self) -> ::windows::core::Result<i32>;
    fn CachedExtendedProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCachedExtendedPropertiesAsArray(&mut self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationStylesPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationStylesPattern_Vtbl {
        unsafe extern "system" fn CurrentStyleId<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStyleId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStyleName<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStyleName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillColor<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFillColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillPatternStyle<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentShape<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentShape() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillPatternColor<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentExtendedProperties<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentExtendedPropertiesAsArray<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentExtendedPropertiesAsArray(::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn CachedStyleId<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedStyleId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedStyleName<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedStyleName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillColor<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFillColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillPatternStyle<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedShape<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedShape() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillPatternColor<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedExtendedProperties<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedExtendedPropertiesAsArray<Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCachedExtendedPropertiesAsArray(::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CurrentStyleId: CurrentStyleId::<Impl, IMPL_OFFSET>,
            CurrentStyleName: CurrentStyleName::<Impl, IMPL_OFFSET>,
            CurrentFillColor: CurrentFillColor::<Impl, IMPL_OFFSET>,
            CurrentFillPatternStyle: CurrentFillPatternStyle::<Impl, IMPL_OFFSET>,
            CurrentShape: CurrentShape::<Impl, IMPL_OFFSET>,
            CurrentFillPatternColor: CurrentFillPatternColor::<Impl, IMPL_OFFSET>,
            CurrentExtendedProperties: CurrentExtendedProperties::<Impl, IMPL_OFFSET>,
            GetCurrentExtendedPropertiesAsArray: GetCurrentExtendedPropertiesAsArray::<Impl, IMPL_OFFSET>,
            CachedStyleId: CachedStyleId::<Impl, IMPL_OFFSET>,
            CachedStyleName: CachedStyleName::<Impl, IMPL_OFFSET>,
            CachedFillColor: CachedFillColor::<Impl, IMPL_OFFSET>,
            CachedFillPatternStyle: CachedFillPatternStyle::<Impl, IMPL_OFFSET>,
            CachedShape: CachedShape::<Impl, IMPL_OFFSET>,
            CachedFillPatternColor: CachedFillPatternColor::<Impl, IMPL_OFFSET>,
            CachedExtendedProperties: CachedExtendedProperties::<Impl, IMPL_OFFSET>,
            GetCachedExtendedPropertiesAsArray: GetCachedExtendedPropertiesAsArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationStylesPattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationSynchronizedInputPattern_Impl: Sized {
    fn StartListening(&mut self, inputtype: SynchronizedInputType) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
impl IUIAutomationSynchronizedInputPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSynchronizedInputPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSynchronizedInputPattern_Vtbl {
        unsafe extern "system" fn StartListening<Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartListening(::core::mem::transmute_copy(&inputtype)).into()
        }
        unsafe extern "system" fn Cancel<Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartListening: StartListening::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationSynchronizedInputPattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationTableItemPattern_Impl: Sized {
    fn GetCurrentRowHeaderItems(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetCurrentColumnHeaderItems(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetCachedRowHeaderItems(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetCachedColumnHeaderItems(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
}
impl IUIAutomationTableItemPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTableItemPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTableItemPattern_Vtbl {
        unsafe extern "system" fn GetCurrentRowHeaderItems<Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentRowHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentColumnHeaderItems<Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentColumnHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedRowHeaderItems<Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedRowHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedColumnHeaderItems<Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedColumnHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrentRowHeaderItems: GetCurrentRowHeaderItems::<Impl, IMPL_OFFSET>,
            GetCurrentColumnHeaderItems: GetCurrentColumnHeaderItems::<Impl, IMPL_OFFSET>,
            GetCachedRowHeaderItems: GetCachedRowHeaderItems::<Impl, IMPL_OFFSET>,
            GetCachedColumnHeaderItems: GetCachedColumnHeaderItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTableItemPattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationTablePattern_Impl: Sized {
    fn GetCurrentRowHeaders(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetCurrentColumnHeaders(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CurrentRowOrColumnMajor(&mut self) -> ::windows::core::Result<RowOrColumnMajor>;
    fn GetCachedRowHeaders(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetCachedColumnHeaders(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn CachedRowOrColumnMajor(&mut self) -> ::windows::core::Result<RowOrColumnMajor>;
}
impl IUIAutomationTablePattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTablePattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTablePattern_Vtbl {
        unsafe extern "system" fn GetCurrentRowHeaders<Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentRowHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentColumnHeaders<Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowOrColumnMajor<Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedRowHeaders<Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedRowHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedColumnHeaders<Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowOrColumnMajor<Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedRowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrentRowHeaders: GetCurrentRowHeaders::<Impl, IMPL_OFFSET>,
            GetCurrentColumnHeaders: GetCurrentColumnHeaders::<Impl, IMPL_OFFSET>,
            CurrentRowOrColumnMajor: CurrentRowOrColumnMajor::<Impl, IMPL_OFFSET>,
            GetCachedRowHeaders: GetCachedRowHeaders::<Impl, IMPL_OFFSET>,
            GetCachedColumnHeaders: GetCachedColumnHeaders::<Impl, IMPL_OFFSET>,
            CachedRowOrColumnMajor: CachedRowOrColumnMajor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTablePattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationTextChildPattern_Impl: Sized {
    fn TextContainer(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn TextRange(&mut self) -> ::windows::core::Result<IUIAutomationTextRange>;
}
impl IUIAutomationTextChildPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextChildPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextChildPattern_Vtbl {
        unsafe extern "system" fn TextContainer<Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *container = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRange<Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextRange() {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TextContainer: TextContainer::<Impl, IMPL_OFFSET>,
            TextRange: TextRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextChildPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTextEditPattern_Impl: Sized + IUIAutomationTextPattern_Impl {
    fn GetActiveComposition(&mut self) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn GetConversionTarget(&mut self) -> ::windows::core::Result<IUIAutomationTextRange>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTextEditPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextEditPattern_Vtbl {
        unsafe extern "system" fn GetActiveComposition<Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveComposition() {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionTarget<Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationTextPattern_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Impl, IMPL_OFFSET>,
            GetConversionTarget: GetConversionTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextEditPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationTextEditTextChangedEventHandler_Impl: Sized {
    fn HandleTextEditTextChangedEvent(&mut self, sender: ::core::option::Option<IUIAutomationElement>, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationTextEditTextChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditTextChangedEventHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextEditTextChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleTextEditTextChangedEvent<Impl: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleTextEditTextChangedEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&texteditchangetype), ::core::mem::transmute_copy(&eventstrings)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            HandleTextEditTextChangedEvent: HandleTextEditTextChangedEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextEditTextChangedEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTextPattern_Impl: Sized {
    fn RangeFromPoint(&mut self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn RangeFromChild(&mut self, child: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn GetSelection(&mut self) -> ::windows::core::Result<IUIAutomationTextRangeArray>;
    fn GetVisibleRanges(&mut self) -> ::windows::core::Result<IUIAutomationTextRangeArray>;
    fn DocumentRange(&mut self) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn SupportedTextSelection(&mut self) -> ::windows::core::Result<SupportedTextSelection>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTextPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextPattern_Vtbl {
        unsafe extern "system" fn RangeFromPoint<Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromPoint(::core::mem::transmute_copy(&pt)) {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromChild<Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, child: ::windows::core::RawPtr, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromChild(::core::mem::transmute(&child)) {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *ranges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisibleRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *ranges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentRange<Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentRange() {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedtextselection: *mut SupportedTextSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedTextSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedtextselection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RangeFromPoint: RangeFromPoint::<Impl, IMPL_OFFSET>,
            RangeFromChild: RangeFromChild::<Impl, IMPL_OFFSET>,
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Impl, IMPL_OFFSET>,
            DocumentRange: DocumentRange::<Impl, IMPL_OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTextPattern2_Impl: Sized + IUIAutomationTextPattern_Impl {
    fn RangeFromAnnotation(&mut self, annotation: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn GetCaretRange(&mut self, isactive: *mut super::super::Foundation::BOOL, range: *mut ::core::option::Option<IUIAutomationTextRange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTextPattern2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextPattern2_Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Impl: IUIAutomationTextPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotation: ::windows::core::RawPtr, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromAnnotation(::core::mem::transmute(&annotation)) {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretRange<Impl: IUIAutomationTextPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaretRange(::core::mem::transmute_copy(&isactive), ::core::mem::transmute_copy(&range)).into()
        }
        Self {
            base: IUIAutomationTextPattern_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Impl, IMPL_OFFSET>,
            GetCaretRange: GetCaretRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextPattern2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationTextRange_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn Compare(&mut self, range: ::core::option::Option<IUIAutomationTextRange>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CompareEndpoints(&mut self, srcendpoint: TextPatternRangeEndpoint, range: ::core::option::Option<IUIAutomationTextRange>, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<i32>;
    fn ExpandToEnclosingUnit(&mut self, textunit: TextUnit) -> ::windows::core::Result<()>;
    fn FindAttribute(&mut self, attr: i32, val: super::super::System::Com::VARIANT, backward: super::super::Foundation::BOOL) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn FindText(&mut self, text: super::super::Foundation::BSTR, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn GetAttributeValue(&mut self, attr: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetBoundingRectangles(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEnclosingElement(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetText(&mut self, maxlength: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Move(&mut self, unit: TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByUnit(&mut self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByRange(&mut self, srcendpoint: TextPatternRangeEndpoint, range: ::core::option::Option<IUIAutomationTextRange>, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<()>;
    fn Select(&mut self) -> ::windows::core::Result<()>;
    fn AddToSelection(&mut self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&mut self) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&mut self, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetChildren(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationTextRange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextRange_Vtbl {
        unsafe extern "system" fn Clone<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clonedrange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *clonedrange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(::core::mem::transmute(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *aresame = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEndpoints<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint, compvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareEndpoints(::core::mem::transmute_copy(&srcendpoint), ::core::mem::transmute(&range), ::core::mem::transmute_copy(&targetendpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *compvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textunit: TextUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExpandToEnclosingUnit(::core::mem::transmute_copy(&textunit)).into()
        }
        unsafe extern "system" fn FindAttribute<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attr: i32, val: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, backward: super::super::Foundation::BOOL, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAttribute(::core::mem::transmute_copy(&attr), ::core::mem::transmute_copy(&val), ::core::mem::transmute_copy(&backward)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindText(::core::mem::transmute_copy(&text), ::core::mem::transmute_copy(&backward), ::core::mem::transmute_copy(&ignorecase)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attr: i32, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeValue(::core::mem::transmute_copy(&attr)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingrects: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundingRectangles() {
                ::core::result::Result::Ok(ok__) => {
                    *boundingrects = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnclosingElement<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosingelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnclosingElement() {
                ::core::result::Result::Ok(ok__) => {
                    *enclosingelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&maxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *moved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndpointByUnit(::core::mem::transmute_copy(&endpoint), ::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *moved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveEndpointByRange(::core::mem::transmute_copy(&srcendpoint), ::core::mem::transmute(&range), ::core::mem::transmute_copy(&targetendpoint)).into()
        }
        unsafe extern "system" fn Select<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn AddToSelection<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn ScrollIntoView<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollIntoView(::core::mem::transmute_copy(&aligntotop)).into()
        }
        unsafe extern "system" fn GetChildren<Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Compare: Compare::<Impl, IMPL_OFFSET>,
            CompareEndpoints: CompareEndpoints::<Impl, IMPL_OFFSET>,
            ExpandToEnclosingUnit: ExpandToEnclosingUnit::<Impl, IMPL_OFFSET>,
            FindAttribute: FindAttribute::<Impl, IMPL_OFFSET>,
            FindText: FindText::<Impl, IMPL_OFFSET>,
            GetAttributeValue: GetAttributeValue::<Impl, IMPL_OFFSET>,
            GetBoundingRectangles: GetBoundingRectangles::<Impl, IMPL_OFFSET>,
            GetEnclosingElement: GetEnclosingElement::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            MoveEndpointByUnit: MoveEndpointByUnit::<Impl, IMPL_OFFSET>,
            MoveEndpointByRange: MoveEndpointByRange::<Impl, IMPL_OFFSET>,
            Select: Select::<Impl, IMPL_OFFSET>,
            AddToSelection: AddToSelection::<Impl, IMPL_OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Impl, IMPL_OFFSET>,
            ScrollIntoView: ScrollIntoView::<Impl, IMPL_OFFSET>,
            GetChildren: GetChildren::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationTextRange2_Impl: Sized + IUIAutomationTextRange_Impl {
    fn ShowContextMenu(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationTextRange2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextRange2_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: IUIAutomationTextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        Self { base: IUIAutomationTextRange_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ShowContextMenu: ShowContextMenu::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextRange2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationTextRange3_Impl: Sized + IUIAutomationTextRange_Impl + IUIAutomationTextRange2_Impl {
    fn GetEnclosingElementBuildCache(&mut self, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetChildrenBuildCache(&mut self, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetAttributeValues(&mut self, attributeids: *const i32, attributeidcount: i32) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationTextRange3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextRange3_Vtbl {
        unsafe extern "system" fn GetEnclosingElementBuildCache<Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, enclosingelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnclosingElementBuildCache(::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *enclosingelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenBuildCache<Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildrenBuildCache(::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValues<Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeids: *const i32, attributeidcount: i32, attributevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeValues(::core::mem::transmute_copy(&attributeids), ::core::mem::transmute_copy(&attributeidcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributevalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationTextRange2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetEnclosingElementBuildCache: GetEnclosingElementBuildCache::<Impl, IMPL_OFFSET>,
            GetChildrenBuildCache: GetChildrenBuildCache::<Impl, IMPL_OFFSET>,
            GetAttributeValues: GetAttributeValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextRange3 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationTextRangeArray_Impl: Sized {
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn GetElement(&mut self, index: i32) -> ::windows::core::Result<IUIAutomationTextRange>;
}
impl IUIAutomationTextRangeArray_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRangeArray_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextRangeArray_Vtbl {
        unsafe extern "system" fn Length<Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElement(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Length: Length::<Impl, IMPL_OFFSET>,
            GetElement: GetElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextRangeArray as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationTogglePattern_Impl: Sized {
    fn Toggle(&mut self) -> ::windows::core::Result<()>;
    fn CurrentToggleState(&mut self) -> ::windows::core::Result<ToggleState>;
    fn CachedToggleState(&mut self) -> ::windows::core::Result<ToggleState>;
}
impl IUIAutomationTogglePattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTogglePattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTogglePattern_Vtbl {
        unsafe extern "system" fn Toggle<Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Toggle().into()
        }
        unsafe extern "system" fn CurrentToggleState<Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedToggleState<Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Toggle: Toggle::<Impl, IMPL_OFFSET>,
            CurrentToggleState: CurrentToggleState::<Impl, IMPL_OFFSET>,
            CachedToggleState: CachedToggleState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTogglePattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTransformPattern_Impl: Sized {
    fn Move(&mut self, x: f64, y: f64) -> ::windows::core::Result<()>;
    fn Resize(&mut self, width: f64, height: f64) -> ::windows::core::Result<()>;
    fn Rotate(&mut self, degrees: f64) -> ::windows::core::Result<()>;
    fn CurrentCanMove(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentCanResize(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentCanRotate(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedCanMove(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedCanResize(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedCanRotate(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTransformPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTransformPattern_Vtbl {
        unsafe extern "system" fn Move<Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Resize<Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Rotate<Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Rotate(::core::mem::transmute_copy(&degrees)).into()
        }
        unsafe extern "system" fn CurrentCanMove<Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanMove() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanResize<Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanResize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanRotate<Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMove<Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanMove() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanResize<Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanResize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanRotate<Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Move: Move::<Impl, IMPL_OFFSET>,
            Resize: Resize::<Impl, IMPL_OFFSET>,
            Rotate: Rotate::<Impl, IMPL_OFFSET>,
            CurrentCanMove: CurrentCanMove::<Impl, IMPL_OFFSET>,
            CurrentCanResize: CurrentCanResize::<Impl, IMPL_OFFSET>,
            CurrentCanRotate: CurrentCanRotate::<Impl, IMPL_OFFSET>,
            CachedCanMove: CachedCanMove::<Impl, IMPL_OFFSET>,
            CachedCanResize: CachedCanResize::<Impl, IMPL_OFFSET>,
            CachedCanRotate: CachedCanRotate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTransformPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTransformPattern2_Impl: Sized + IUIAutomationTransformPattern_Impl {
    fn Zoom(&mut self, zoomvalue: f64) -> ::windows::core::Result<()>;
    fn ZoomByUnit(&mut self, zoomunit: ZoomUnit) -> ::windows::core::Result<()>;
    fn CurrentCanZoom(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedCanZoom(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentZoomLevel(&mut self) -> ::windows::core::Result<f64>;
    fn CachedZoomLevel(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentZoomMinimum(&mut self) -> ::windows::core::Result<f64>;
    fn CachedZoomMinimum(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentZoomMaximum(&mut self) -> ::windows::core::Result<f64>;
    fn CachedZoomMaximum(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTransformPattern2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTransformPattern2_Vtbl {
        unsafe extern "system" fn Zoom<Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Zoom(::core::mem::transmute_copy(&zoomvalue)).into()
        }
        unsafe extern "system" fn ZoomByUnit<Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ZoomByUnit(::core::mem::transmute_copy(&zoomunit)).into()
        }
        unsafe extern "system" fn CurrentCanZoom<Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanZoom<Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomLevel<Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomLevel<Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomMinimum<Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentZoomMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomMinimum<Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedZoomMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomMaximum<Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentZoomMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomMaximum<Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedZoomMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationTransformPattern_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Zoom: Zoom::<Impl, IMPL_OFFSET>,
            ZoomByUnit: ZoomByUnit::<Impl, IMPL_OFFSET>,
            CurrentCanZoom: CurrentCanZoom::<Impl, IMPL_OFFSET>,
            CachedCanZoom: CachedCanZoom::<Impl, IMPL_OFFSET>,
            CurrentZoomLevel: CurrentZoomLevel::<Impl, IMPL_OFFSET>,
            CachedZoomLevel: CachedZoomLevel::<Impl, IMPL_OFFSET>,
            CurrentZoomMinimum: CurrentZoomMinimum::<Impl, IMPL_OFFSET>,
            CachedZoomMinimum: CachedZoomMinimum::<Impl, IMPL_OFFSET>,
            CurrentZoomMaximum: CurrentZoomMaximum::<Impl, IMPL_OFFSET>,
            CachedZoomMaximum: CachedZoomMaximum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTransformPattern2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationTreeWalker_Impl: Sized {
    fn GetParentElement(&mut self, element: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetFirstChildElement(&mut self, element: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetLastChildElement(&mut self, element: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetNextSiblingElement(&mut self, element: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElement(&mut self, element: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn NormalizeElement(&mut self, element: ::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetParentElementBuildCache(&mut self, element: ::core::option::Option<IUIAutomationElement>, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetFirstChildElementBuildCache(&mut self, element: ::core::option::Option<IUIAutomationElement>, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetLastChildElementBuildCache(&mut self, element: ::core::option::Option<IUIAutomationElement>, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetNextSiblingElementBuildCache(&mut self, element: ::core::option::Option<IUIAutomationElement>, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElementBuildCache(&mut self, element: ::core::option::Option<IUIAutomationElement>, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn NormalizeElementBuildCache(&mut self, element: ::core::option::Option<IUIAutomationElement>, cacherequest: ::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn Condition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
}
impl IUIAutomationTreeWalker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTreeWalker_Vtbl {
        unsafe extern "system" fn GetParentElement<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildElement<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, first: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstChildElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *first = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastChildElement<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, last: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastChildElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *last = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingElement<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextSiblingElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *next = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElement<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, previous: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviousSiblingElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *previous = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizeElement<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, normalized: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizeElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *normalized = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentElementBuildCache<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildElementBuildCache<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, first: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstChildElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *first = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastChildElementBuildCache<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, last: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastChildElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *last = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingElementBuildCache<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextSiblingElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *next = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElementBuildCache<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, previous: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviousSiblingElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *previous = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizeElementBuildCache<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, normalized: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizeElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *normalized = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Condition<Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *condition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetParentElement: GetParentElement::<Impl, IMPL_OFFSET>,
            GetFirstChildElement: GetFirstChildElement::<Impl, IMPL_OFFSET>,
            GetLastChildElement: GetLastChildElement::<Impl, IMPL_OFFSET>,
            GetNextSiblingElement: GetNextSiblingElement::<Impl, IMPL_OFFSET>,
            GetPreviousSiblingElement: GetPreviousSiblingElement::<Impl, IMPL_OFFSET>,
            NormalizeElement: NormalizeElement::<Impl, IMPL_OFFSET>,
            GetParentElementBuildCache: GetParentElementBuildCache::<Impl, IMPL_OFFSET>,
            GetFirstChildElementBuildCache: GetFirstChildElementBuildCache::<Impl, IMPL_OFFSET>,
            GetLastChildElementBuildCache: GetLastChildElementBuildCache::<Impl, IMPL_OFFSET>,
            GetNextSiblingElementBuildCache: GetNextSiblingElementBuildCache::<Impl, IMPL_OFFSET>,
            GetPreviousSiblingElementBuildCache: GetPreviousSiblingElementBuildCache::<Impl, IMPL_OFFSET>,
            NormalizeElementBuildCache: NormalizeElementBuildCache::<Impl, IMPL_OFFSET>,
            Condition: Condition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTreeWalker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationValuePattern_Impl: Sized {
    fn SetValue(&mut self, val: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CurrentValue(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentIsReadOnly(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedValue(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedIsReadOnly(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationValuePattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationValuePattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationValuePattern_Vtbl {
        unsafe extern "system" fn SetValue<Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn CurrentValue<Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsReadOnly<Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsReadOnly<Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            CurrentValue: CurrentValue::<Impl, IMPL_OFFSET>,
            CurrentIsReadOnly: CurrentIsReadOnly::<Impl, IMPL_OFFSET>,
            CachedValue: CachedValue::<Impl, IMPL_OFFSET>,
            CachedIsReadOnly: CachedIsReadOnly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationValuePattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationVirtualizedItemPattern_Impl: Sized {
    fn Realize(&mut self) -> ::windows::core::Result<()>;
}
impl IUIAutomationVirtualizedItemPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationVirtualizedItemPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationVirtualizedItemPattern_Vtbl {
        unsafe extern "system" fn Realize<Impl: IUIAutomationVirtualizedItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Realize().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Realize: Realize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationVirtualizedItemPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationWindowPattern_Impl: Sized {
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn WaitForInputIdle(&mut self, milliseconds: i32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetWindowVisualState(&mut self, state: WindowVisualState) -> ::windows::core::Result<()>;
    fn CurrentCanMaximize(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentCanMinimize(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsModal(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsTopmost(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CurrentWindowVisualState(&mut self) -> ::windows::core::Result<WindowVisualState>;
    fn CurrentWindowInteractionState(&mut self) -> ::windows::core::Result<WindowInteractionState>;
    fn CachedCanMaximize(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedCanMinimize(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsModal(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsTopmost(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedWindowVisualState(&mut self) -> ::windows::core::Result<WindowVisualState>;
    fn CachedWindowInteractionState(&mut self) -> ::windows::core::Result<WindowInteractionState>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationWindowPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationWindowPattern_Vtbl {
        unsafe extern "system" fn Close<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn WaitForInputIdle<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, success: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForInputIdle(::core::mem::transmute_copy(&milliseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *success = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowVisualState<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWindowVisualState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CurrentCanMaximize<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanMaximize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanMinimize<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanMinimize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsModal<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsModal() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsTopmost<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWindowVisualState<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWindowVisualState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWindowInteractionState<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWindowInteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMaximize<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanMaximize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMinimize<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanMinimize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsModal<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsModal() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsTopmost<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedWindowVisualState<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedWindowVisualState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedWindowInteractionState<Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedWindowInteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Close: Close::<Impl, IMPL_OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Impl, IMPL_OFFSET>,
            SetWindowVisualState: SetWindowVisualState::<Impl, IMPL_OFFSET>,
            CurrentCanMaximize: CurrentCanMaximize::<Impl, IMPL_OFFSET>,
            CurrentCanMinimize: CurrentCanMinimize::<Impl, IMPL_OFFSET>,
            CurrentIsModal: CurrentIsModal::<Impl, IMPL_OFFSET>,
            CurrentIsTopmost: CurrentIsTopmost::<Impl, IMPL_OFFSET>,
            CurrentWindowVisualState: CurrentWindowVisualState::<Impl, IMPL_OFFSET>,
            CurrentWindowInteractionState: CurrentWindowInteractionState::<Impl, IMPL_OFFSET>,
            CachedCanMaximize: CachedCanMaximize::<Impl, IMPL_OFFSET>,
            CachedCanMinimize: CachedCanMinimize::<Impl, IMPL_OFFSET>,
            CachedIsModal: CachedIsModal::<Impl, IMPL_OFFSET>,
            CachedIsTopmost: CachedIsTopmost::<Impl, IMPL_OFFSET>,
            CachedWindowVisualState: CachedWindowVisualState::<Impl, IMPL_OFFSET>,
            CachedWindowInteractionState: CachedWindowInteractionState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationWindowPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IValueProvider_Impl: Sized {
    fn SetValue(&mut self, val: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsReadOnly(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IValueProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValueProvider_Vtbl {
        unsafe extern "system" fn SetValue<Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn Value<Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            IsReadOnly: IsReadOnly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValueProvider as ::windows::core::Interface>::IID
    }
}
pub trait IVirtualizedItemProvider_Impl: Sized {
    fn Realize(&mut self) -> ::windows::core::Result<()>;
}
impl IVirtualizedItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizedItemProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualizedItemProvider_Vtbl {
        unsafe extern "system" fn Realize<Impl: IVirtualizedItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Realize().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Realize: Realize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualizedItemProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowProvider_Impl: Sized {
    fn SetVisualState(&mut self, state: WindowVisualState) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn WaitForInputIdle(&mut self, milliseconds: i32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CanMaximize(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CanMinimize(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsModal(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn WindowVisualState(&mut self) -> ::windows::core::Result<WindowVisualState>;
    fn WindowInteractionState(&mut self) -> ::windows::core::Result<WindowInteractionState>;
    fn IsTopmost(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWindowProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowProvider_Vtbl {
        unsafe extern "system" fn SetVisualState<Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisualState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn Close<Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn WaitForInputIdle<Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForInputIdle(::core::mem::transmute_copy(&milliseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMaximize<Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMaximize() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMinimize<Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMinimize() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsModal<Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsModal() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowVisualState<Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowVisualState() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowInteractionState<Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowInteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTopmost<Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetVisualState: SetVisualState::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Impl, IMPL_OFFSET>,
            CanMaximize: CanMaximize::<Impl, IMPL_OFFSET>,
            CanMinimize: CanMinimize::<Impl, IMPL_OFFSET>,
            IsModal: IsModal::<Impl, IMPL_OFFSET>,
            WindowVisualState: WindowVisualState::<Impl, IMPL_OFFSET>,
            WindowInteractionState: WindowInteractionState::<Impl, IMPL_OFFSET>,
            IsTopmost: IsTopmost::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowProvider as ::windows::core::Interface>::IID
    }
}
